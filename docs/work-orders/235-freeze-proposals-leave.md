# Work order 235: the decision framework's change 3, the freeze: `docs/proposals/` leaves the tree, its links pinned, and `tools/check_rulings.py` retired

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Report in the shared schema, `004-i2-01-vectors-and-first-start.schema.json`, with
`vectors` empty. This order needs no network and builds nothing: run no cargo command and not
`tools/gate.py`, which the coordinator runs after decision 8's commit. Never print a whole
large file, diff or list: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read
or print a key, a token or any credential. Run no `conduit`, `codex` or `claude`, and sign in
nowhere. Every file you write is public: no absolute local path, user or machine name, or
private repository name.

## What and why

**What.** On 2026-11-20 the owner chose "Git is the archive (Rec.)" and, for the archive clean,
"B: at the freeze (Rec.)"
([DEC-031](../decisions/log.md#dec-031-2026-11-20-records-the-archive-and-the-documents));
archived rulings keep their numbers ([the log](../decisions/README.md#the-log)). Group B is all
of `docs/proposals/`: the ledger `ruling-log.md`, the records of rulings 38 to 92, the full review,
the first build's register and runs, and 30 other topic folders. Two live inputs stay, moved out
(decision 2). Group C, `IN_FORCE.md` and `docs/AUDIT.md`, leaves at change 6.

**Why.** At `2c56efac` the directory holds 254 files and 6,685,148 bytes, 61% of the 10,929,567
under `docs/` (the tree: 41,057,098). 1,127 relative links in 121 staying Markdown files point
into it, at 132 paths, and 42 more in experiment 028's two files.

**Preservation tests** ([the tests](../AUDIT.md#preservation-tests); ruling 92, point 1): none
served and none at risk. No promise, engine code, package or case changes.

**Should this exist** (ruling 92, point 3). Yes: removing a file and linking its last version is
the archive rule the owner chose
([HOW-WE-WORK section 8](../../HOW-WE-WORK.md#8-records-where-each-fact-lives)), as order 222
applied it to groups A and D. It adds no mechanism and retires one, `tools/check_rulings.py`,
whose inputs leave.

**When it runs.** Through Conduit's dogfood: one Codex worker turn (a 4-hour limit; aim for an
hour), a Codex verifier and a Claude reader, who run only what settles a doubt, then the gate
(DEC-033). Decisions 2, 3, 5, 7 and 8 are the coordinator's, open to the owner's strike.

## The decisions it applies

1. **Links pinned.** BASE is the full SHA the start check prints; BASE8 its first 8 characters.
   In every staying Markdown file but those under `experiments/028-a78-dogfood/`, each relative
   link, inline or a reference definition, whose target resolves into `docs/proposals/` becomes
   `https://github.com/shrav-k/conduit/blob/BASE/<path>` (`tree/` for a directory), keeping its
   anchor and text. That includes links whose label spans two lines, which `check_docs.py`
   skips: find links by their target. A link to one of decision 2's files takes its new relative
   path instead. In the two moved files, resolve each link from its old place: a removed target
   is pinned, a moved one gets its new path, a live one is written relative to the new place. A
   path in plain text or code stays as written, but for the three in `tools/check_schemas.py`
   (lines 92, 217, 230), which `check_docs.py` checks: each becomes the same permalink.
   `engine/README.md` line 12's link targets the open log instead. Rewrite with a one-off
   command; add no script to the tree.

2. **The stay list** (fixed). Two files stay, moved by `git mv` into a new
   `docs/implementation/first-build/`, beside `FIRST_BUILD.md`, whose measures they apply:
   - `docs/proposals/first-build/increment-4/measurement-plan.md`, as `measurement-plan.md`: the
     S1 rerun's counting, fixed before it runs. The rerun has not run and is part of v0's gate;
     experiments 013, 014, 018, 019 and 021 pre-register their counts by it.
   - `docs/proposals/first-build/log-increment-4.md`, as `log-increment-4.md`: the open
     increment's log, kept until the increment closes (ruling 58, point 3). Findings 4-01 and
     4-11 stay open in it until S1's bar is judged; AGENTS.md, `conformance/README.md`,
     `engine/README.md`, `FIRST_BUILD.md` and 028's pass condition 6 send findings there.

   Nothing else there is read by an open order's checks (but 212's, decision 9), the Fleet, a
   harness, `tools/` (but `check_rulings.py`) or `engine/`; a pinned link reads the same.

3. **The rest leaves**: `git rm -r -q docs/proposals` after the moves, 252 files. These leave,
   though a reader might keep them:
   - increment 4's runs and reports (76 files). Ruling 58, point 5, kept them "until the S1
     rerun"; the owner's later "B" over all of `docs/proposals/` is read as ending that: a pinned
     link keeps each readable, and the rerun is judged against A70 (ruling 71, point 3);
   - the register, `first-build/README.md`: its settlements of increments 1 to 3 are history,
     and FIRST_BUILD.md's "Findings" says what a finding is and who settles it;
   - the review list, `first-build/to-review-with-the-owner.md`: its items 1 to 29 stand (the
     owner reviewed 15 to 29 on 2026-11-12), and a later coordinator's decision is a Record line
     marked as the coordinator's (HOW-WE-WORK rule 6). Its items 30 to 52 are not yet reviewed;
     they wait at the pinned list (decision 5).

4. **How the ledger and the records are cited until change 6.** AGENTS.md's "What holds" and
   IN_FORCE.md still bind by them, so they cite the archived copies. In "What holds", items 3
   and 4 become, with a paragraph after them:

   ```markdown
   3. **[The ledger](https://github.com/shrav-k/conduit/blob/BASE/docs/proposals/ruling-log.md)**
      says what the owner ruled, when, in the owner's words, and each ruling's status.
   4. **A ruling's record**, [beside the ledger](https://github.com/shrav-k/conduit/tree/BASE/docs/proposals),
      says why. Its first line says whether it still holds.

   The ledger and the records are archived at `BASE8`: work order 235 took them out of the tree,
   git keeps them, and each link to them names that commit. They hold as items 3 and 4 say until
   IN_FORCE.md is retired, and nothing is added to them.
   ```

   In IN_FORCE.md's opening paragraph, "The ledger, `docs/proposals/ruling-log.md`, says" becomes
   "The [ledger](https://github.com/shrav-k/conduit/blob/BASE/docs/proposals/ruling-log.md),
   archived, says", on the paragraph's one line, "archived" linking to AGENTS.md's "What holds".

5. **Live instructions retargeted.** CONTRIBUTING.md's rule for a removal asks for "live
   instructions retargeted to where the work now goes". Change each sentence below, and that
   sentence alone, to send its work where the last column says; report each new one.

   | Where | Sends today | Sends now |
   | --- | --- | --- |
   | AGENTS.md "Where things go", first bullet; CONTRIBUTING.md "Keeping decisions and documents current", first paragraph | a ruling's record and ledger row; an answer into the record | an entry in [the decision log](../decisions/log.md), in its README's entry form, with the IN_FORCE.md lines it changes and any action item, and no ruling, record or ledger row; an answer that confirms or corrects a decision is a new entry citing it |
   | CONTRIBUTING.md "Where work belongs", first two bullets | a ruling and its record; a proposal as a record | an owner decision, an entry of the log; a proposal goes to the owner with options and a recommendation, in the order or pull request that needs it |
   | `docs/guides/full-review.md` "What to produce" (item 1, the paragraph after); `docs/guides/cold-readers.md` "How to report a finding"; ROADMAP.md row "Read"; `docs/AUDIT.md` "For the next reader" and "How to use this document", third bullet; CONTRIBUTING.md "Where work belongs", the defect bullet's re-audit sentence | a record or reports under `full-review/` or `reaudit/`; one ruling | a dated report handed to the coordinator, not committed; its findings land as `spec/` changes with `spec/CHANGES.md` lines, log entries, ROADMAP rows or orders |
   | AGENTS.md "Where things go" and CONTRIBUTING.md "Where work belongs", the first build's bullets; FIRST_BUILD.md "Findings" | the logs under `first-build/` | the open log at its new path |
   | `docs/README.md` "Maintaining the design", last sentence | IN_FORCE.md and the ledger | IN_FORCE.md and the decision log |
   | IN_FORCE.md line 353, second sentence | "stay in the tree" | "stay as v0's evidence, in the tree or at the commit a removal pins (DEC-031)" |
   | `experiments/README.md` lines 80 to 82 | a state | "The results and two reports left the tree under work order 235; the plan moved beside FIRST_BUILD.md." |
   | CONTRIBUTING.md, "increment 4's runs stay until the S1 rerun (ruling 58, point 5)" | a state | "increment 4's raw runs left the tree under work order 235; its open log and the S1 rerun's plan are in" a link to `docs/implementation/first-build/` |

   Add to `docs/AUDIT.md`'s "Waiting for the owner" table:

   ```text
   | The joint review of the coordinator's decisions, items 30 to 52 of the review list | [The list at BASE8](https://github.com/shrav-k/conduit/blob/BASE/docs/proposals/first-build/to-review-with-the-owner.md) |
   ```

   Other mentions of the directory stay as written.

6. **`tools/check_rulings.py` retired.** It reads `docs/proposals/ruling-*`, the ledger, and
   IN_FORCE.md for dates. Delete it and `tools/tests/test_rulings.py`; remove its row from
   `PYTHON_GATES` in `tools/gate.py`, its pair from `test_python_gates_follow_the_docs_job` in
   `tools/tests/test_gate.py` (its slice one row shorter), its step from the `docs` job of
   `.github/workflows/checks.yml`, and its line from AGENTS.md's "Checks". Unchecked from now:
   the ledger's form, archived, and no date in IN_FORCE.md, whose bullets `check_decisions.py`
   keeps from growing. Experiments 018, 019 and 021 name it in their pre-registered checks
   (`plan.md`): keep that text and add one line after its paragraph: "Note of <the date you
   run>: `check_rulings.py` left the tree under work order 235 with the records it read; this
   check is not run."

   `tools/check_decisions.py` names `docs/proposals` and `docs/proposals/` to refuse their return,
   which `check_docs.py`'s path-mention check would report missing. Add those two exact names to
   `PATH_EXCEPTIONS` in `tools/check_docs.py`, as a retired path `check_decisions.py` refuses, and
   let its notice name both kinds; `docs/proposals/<anything>` is still reported. Test it in
   `tools/tests/test_condensation_checks.py`, in a test whose name holds `retired`. Change nothing
   in `check_decisions.py`.

7. **The indexes and the map.** `docs/archive/README.md`: append to its first table (one row per
   group, as order 222 did):

   ```text
   | `docs/proposals/ruling-log.md` | The ledger of rulings 1 to 92; it holds until IN_FORCE.md is retired. | [IN_FORCE.md](../../IN_FORCE.md), then [the decision log](../decisions/log.md). | [BASE8](https://github.com/shrav-k/conduit/blob/BASE/docs/proposals/ruling-log.md) |
   | `docs/proposals/ruling-{38..92}/` | The records of rulings 38 to 92. | The same. | [BASE8](https://github.com/shrav-k/conduit/tree/BASE/docs/proposals) |
   | `docs/proposals/full-review/` | The full review of 2026-11-14. | Rulings 78 to 92 and their orders. | [BASE8](https://github.com/shrav-k/conduit/tree/BASE/docs/proposals/full-review) |
   | `docs/proposals/first-build/`, but its open log and the rerun's plan | The register of increments 1 to 3, increment 4's runs, the review list. | `spec/` and [the open log](../implementation/first-build/log-increment-4.md). | [BASE8](https://github.com/shrav-k/conduit/tree/BASE/docs/proposals/first-build) |
   | `docs/proposals/`, its other 30 folders | Proposals, audits and investigations. | `spec/` and the rulings. | [BASE8](https://github.com/shrav-k/conduit/tree/BASE/docs/proposals) |
   ```

   `docs/README.md`: under "### Records of rulings" (keep the heading: CONTRIBUTING.md links it),
   the paragraph and the table become:

   ```markdown
   The ledger names each ruling and its status; IN_FORCE.md collects what still holds; a
   record says why. They are archived as [AGENTS.md](../AGENTS.md#what-holds) says;
   [this section at `BASE8`](https://github.com/shrav-k/conduit/blob/BASE/docs/README.md#records-of-rulings) lists each.
   ```

   In "Process for the builds", after the `FIRST_BUILD.md` row, add:

   ```text
   | [implementation/first-build/](implementation/first-build/log-increment-4.md) | The first build's open log, increment 4's, and the S1 rerun's [measurement plan](implementation/first-build/measurement-plan.md). |
   ```

8. **Experiment 028.** Its stage-2 rule sends no order through Conduit that changes its
   directory, and its README and results of 2026-11-12 hold 42 links into `docs/proposals/`.
   Change nothing there; until this decision's commit `check_docs.py` reports those links, and
   only those. After the worker's turn and outside every turn, the coordinator commits on the
   branch, in the trusted copy where the reader and the gate run: those links pinned to BASE, the
   four to the open log given its new path, `inputs/plan.md`'s plain paths as written, and at the
   end of its README:

   ```markdown
   ## Amendment of <date>: links after work order 235

   The coordinator, outside every turn, pointed this directory's links into `docs/proposals/` at
   `BASE8`, and the open log's at its new place, since a stage-2 order may not change this
   directory. No other text changed.
   ```

   The verifier reads the worker's diff without it, so 028's errors are expected there.

9. **Branches in flight and open orders.** A branch rebased onto
   `main` after this merges pins its new links into `docs/proposals/` to BASE (`check_docs.py`
   reports each) and adds no file there (`check_decisions.py` refuses one). Open orders keep
   their text, links pinned. Order 212's start check, sections 11 and 12 and fenced draft assume
   the directory, and orders 049a1, 051b3, 080 and 212 run `check_rulings.py`: the coordinator
   rewrites each at its refresh.

## Stop rules

- Under `spec/`, `engine/`, `conformance/` and `packages/` change Markdown link targets alone.
  Change nothing under `experiments/028-a78-dogfood/` or `docs/decisions/`, nor HOW-WE-WORK.md.
  If the removal needs more, stop, change nothing further, and report it.
- If the start check disagrees with a fixed value, stop and report. Do not move a file on or off
  decision 2's list.

## Before you change anything

From the repository root; stop, change nothing and report if a line prints otherwise:

```sh
git merge-base --is-ancestor 2c56efac HEAD; echo "main $?"
git merge-base --is-ancestor HEAD origin/main; echo "pushed $?"
git rev-parse HEAD # BASE
git status --short | wc -l | tr -d ' '
git ls-files docs/proposals | wc -l | tr -d ' '
ls -d docs/proposals/ruling-[0-9]* | wc -l | tr -d ' '
ls docs/proposals/first-build/increment-4/measurement-plan.md docs/proposals/first-build/log-increment-4.md | wc -l | tr -d ' '
test -e docs/implementation/first-build; echo "new $?"
ls tools/check_rulings.py tools/tests/test_rulings.py | wc -l | tr -d ' '
p='(\]\(|\]: *)<?(\./)?(\.\./)*(docs/)?proposals(/|\)|#|$)'
git grep -nE "$p" -- '*.md' ':!docs/proposals' ':!experiments/028-a78-dogfood' ':!docs/work-orders/212-*' | wc -l | tr -d ' '
git grep -nE "$p" -- experiments/028-a78-dogfood | wc -l | tr -d ' '
python3 tools/check_docs.py 2>&1 | tail -1
for d in docs .; do git ls-files -z -- $d | xargs -0 cat | wc -c | tr -d ' '; done
```

They print `main 0`, `pushed 0`, BASE, `0`, `254`, `55`, `2`, `new 1`, `2`, two link lines
(978 and 42 if no order merged since this text; report them), `Errors: 0`, and two sizes
(10,929,567 and 41,057,098 at `2c56efac`; report them).

## Scope

Move decision 2's files and remove the rest of `docs/proposals/`. Delete `tools/check_rulings.py`
and `tools/tests/test_rulings.py`. Change `tools/gate.py`, `tools/tests/test_gate.py`,
`.github/workflows/checks.yml`, `tools/check_docs.py`, `tools/tests/test_condensation_checks.py`,
`tools/check_schemas.py` (three mentions), AGENTS.md, IN_FORCE.md (its opening paragraph, line
353's second sentence, and link targets), `docs/README.md`, `docs/archive/README.md`, decision
5's places, the three experiments' `plan.md`, and link targets alone in any other staying
Markdown file. Create nothing else. Anything else goes in your report.

## Done when

Each line in the first group prints otherwise on BASE ("today") and passes after:

```sh
BASE=<the start check's SHA>; B8=$(echo "$BASE" | cut -c1-8)
p='(\]\(|\]: *)<?(\./)?(\.\./)*(docs/)?proposals(/|\)|#|$)'
test ! -e docs/proposals && echo gone # gone
git diff --cached -M --name-status HEAD | grep -c '^R.*docs/implementation/first-build/' # 2
git grep -nE "$p" -- '*.md' ':!experiments/028-a78-dogfood' ':!docs/work-orders/212-*' | wc -l | tr -d ' ' # 0; 978 today
git grep -nE "$p" -- 'docs/work-orders/212-*' | wc -l | tr -d ' ' # 6, its fenced draft; 16 today
test ! -e tools/check_rulings.py && test ! -e tools/tests/test_rulings.py && echo retired # retired
git grep -l check_rulings -- tools .github AGENTS.md | wc -l | tr -d ' ' # 0; 5 today
grep -c 'def test.*retired' tools/tests/test_condensation_checks.py # 1 or more; 0 today
grep -c "archived at \`$B8\`" AGENTS.md # 1; 0 today
grep -c 'AGENTS.md#what-holds' IN_FORCE.md docs/README.md # 1 each; 0 today
{ grep -F 're-audit](' CONTRIBUTING.md docs/AUDIT.md; grep 'IN_FORCE.md and the ledger' docs/README.md; grep 'stay in the tree as' IN_FORCE.md; grep 'stay beside' experiments/README.md; grep 'stay until the S1 rerun' CONTRIBUTING.md; } | wc -l | tr -d ' ' # 0; 7 today
grep -c 'items 30 to 52' docs/AUDIT.md # 1; 0 today
{ git grep -h '`docs/proposals' -- AGENTS.md CONTRIBUTING.md ROADMAP.md docs/guides; sed -n 5p IN_FORCE.md; grep '^`docs/proposals/full-review' docs/AUDIT.md; } | grep -c '`docs/proposals' # 0; 11 today
grep -c "/$BASE/docs/proposals" docs/archive/README.md # 6 or more; 0 today
sed -n '/^### Records of rulings/,/^### Process/p' docs/README.md | grep -c '^| \[' # 0; 62 today
grep -l 'work order 235' experiments/018-*/plan.md experiments/019-*/plan.md experiments/021-*/plan.md | wc -l | tr -d ' ' # 3; 0 today
python3 tools/check_docs.py 2>&1 >/dev/null | grep -c '^experiments/028-a78-dogfood/' # 1 or more, decision 8's; 0 today
```

Guards, which pass today and must still pass:

```sh
python3 tools/check_docs.py 2>&1 >/dev/null | grep -vc '^experiments/028-a78-dogfood/' # 0: no other error
python3 tools/check_roadmap.py | tail -1 # Errors: 0
python3 tools/check_schemas.py | tail -1 # Errors: 0
python3 tools/check_private_paths.py | tail -1 # Errors: 0
python3 tools/check_decisions.py HEAD WORKTREE | tail -1 # Errors: 0
python3 -m unittest discover -s tools/tests 2>&1 | tail -1 # OK
git diff --check # prints nothing
git diff --name-only HEAD -- spec engine conformance packages | grep -vc '\.md$' # 0
git diff --name-only HEAD -- experiments/028-a78-dogfood docs/decisions HOW-WE-WORK.md | wc -l | tr -d ' ' # 0
u='github\.com/shrav-k/conduit/(blob|tree)/[0-9a-f]+/docs/proposals[^)#> ]*'
comm -23 <(git diff HEAD | grep '^+' | grep -oE "$u" | sort -u) <(git diff HEAD | grep '^-' | grep -oE "$u" | sort -u) | grep -vc "/$BASE/" # 0: each new pin names BASE
for d in docs .; do git ls-files -z -- $d | xargs -0 cat | wc -c | tr -d ' '; done # report both
```

`git mv` and `git rm` stage the moves and deletions; stage nothing else. Report the files moved
and removed, the links pinned and repointed per top directory, both sizes before and after (about
4.3 and 34.5 million bytes after), decision 5's new sentences, `own_choices`, and every check's
last line.

## What follows

Not in this order: change 6 removes IN_FORCE.md and `docs/AUDIT.md` the same way and drops
decision 4's paragraph; change 5 rewords CONTRIBUTING.md on proposal records and HOW-WE-WORK rule
5's ratio, whose `docs/proposals/` lines now count none.

## Record

Not yet run.
