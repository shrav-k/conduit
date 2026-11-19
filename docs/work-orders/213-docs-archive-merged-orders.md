# Work order 213: merged work orders move whole into the archive, every link kept

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order adds one tool and its test, moves files
with `git mv` and rewrites link targets across the repository. It has no network access and needs
none. It changes no Rust code and needs no cargo command; do not run `python3 tools/gate.py`,
whose run at the merge is the coordinator's. Never print a whole large file: read the orders and
`docs/implementation/WORK_ORDERS.md` with `grep -n` and `sed -n`. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never read or print a key,
a token or any credential.

## What and why

**What it does.** Of the order files in `docs/work-orders/`, most have merged: their Records are
written at their foot, and the merged orders, about 160, outnumber the live ones in the
directory and its index. The owner chose to plan the archive, keeping links working ("Plan it,
keep links working"), and chose "Two orders". That this order is the second, the move, run in the
next quiet window of the merge queue after order 212, is the plan's recommendation as the
coordinator applies it (the coordinator's reading, open to the owner's strike), as ruling 90
records. Under ruling 90, which amends ruling 58, point 4, a merged order's archived file is its
record: the file moves whole, its Record and decisions with it, and the record page links each
archived file's `## Record` instead of copying it into a row. This order writes a tool,
`tools/archive_work_orders.py`, that moves every merged order that no file staying in
`docs/work-orders/` names, with its declaration, into `docs/archive/work-orders/` under the same
file name, and rewrites every link target so that each link leads where it led before; it then
brings the few sentences that say where merged orders are to the tree as it is.

**What it does not do.** No file leaves the tree. No word of a Record, of a ruling record, of the
ledger, of `IN_FORCE.md` or of `spec/` changes; a link target may (CONTRIBUTING.md, "Keeping
decisions and documents current": "leave surviving text as written except link targets"). No
live order's text changes, because the coordinator's runners read each order, the shared schema,
`verify.md` and `verify.schema.json` from the top of `docs/work-orders/` on `main` and give the
order's text to its worker as it stands. Nothing under `docs/proposals/` moves. The rows of
orders 001 to 036k and 060 to 066 in the record stay as they are.

**Why a tool, kept in `tools/`.** About 1,300 link targets change. The move is reproducible only
if a script makes it, and the orders it holds back move in later sweeps by the same rule. The
coordinator reads AGENTS.md's exception for "the Python checkers in `tools/`" as covering it, on
`tools/migrate_041a.py`'s precedent, a migration kept there with its test (the coordinator's,
open to the owner's strike).

## What to read

- The plan, step B, is in the coordinator's notes (private); what it decided is under "The
  decisions it applies" below, and what its dry run found is under "The coordinator's dry run".
- [Ruling 90's record](../proposals/ruling-90/README.md), and
  [ruling 58](../proposals/ruling-58/README.md), points 4 and 9.
- Order 212's file and its Record: the generator it added for the archive's index, and how the
  work-orders README is split into live and merged orders.
- [tools/check_docs.py](../../tools/check_docs.py): what it proves (each link target and anchor
  in prose, reference definitions included, and the literal paths in `tools/`, `engine/` and
  `conformance/` sources) and what it does not (a path written as text in Markdown, a link whose
  text runs over a line break).
- [The record](../implementation/WORK_ORDERS.md#the-record) and item 8 of "An order" in
  WORK_ORDERS.md; the opening and "Shared and staged files" of [the README](README.md).

## Before dispatch (the coordinator)

The coordinator dispatches this order only when order 212 has merged with its Record filled,
ruling 90 is in the ledger, and the merge queue is quiet: no order's run has ended without its
merge, and no branch not yet merged changes, links or names a file this order would move. Its
dry run of the rules below, on the day, gives the list to compare against. If any of these fails,
this order waits. If an order merges while this one runs, the coordinator reruns the tool on a
fresh branch from the new `main`; nothing is resolved by hand.

## Before you change anything

From the repository root, check that you find what this order was written against, and stop,
change nothing and report if any line prints otherwise:

```sh
git merge-base --is-ancestor 85227914 HEAD; echo "main $?"
ls docs/work-orders/212-*.md | wc -l
tail -n 5 docs/work-orders/212-*.md | grep -c '^Not yet run\.$'
grep -c '^| 90 |' docs/proposals/ruling-log.md
test -f docs/archive/work-orders/README.md; echo "index $?"
test -e tools/archive_work_orders.py; echo "tool $?"
git status --short | wc -l
```

They print `main 0`, `1`, `0`, `1`, `index 0`, `tool 1` and `0`.

## The decisions it applies

1. **What moves.** A top-level `docs/work-orders/*.md`, other than `README.md` and `verify.md`,
   moves when both hold:
   - its `## Record` section, read outside code fences, is filled: present, not empty, and not
     "Not yet run.";
   - no file that stays in `docs/work-orders/` names it, `README.md` excepted. A file names an
     order when its file name, or that name without `.md`, appears anywhere in it as a whole
     token, in prose or in code: a link, a path, a bare file name in a shell loop or a
     `git show origin/main:` command; a file that names the order's declaration names it too
     (the coordinator's reading of ruling 90, point 3, open to the owner's strike). The files
     that stay are the orders without a filled Record, `verify.md`, the schemas, the
     `*-staged/` directories and the declarations of staying orders. A merged order held back
     counts as staying: it holds back each order it names, and the selection repeats until
     nothing changes.

   Also held back: an order whose file or declaration a file outside `docs/work-orders/` that is
   not Markdown names by path. Today that is order 041a, whose declaration
   `tools/migrate_041a.py` reads by path; it moves only with an order that edits that tool.
2. **What goes with it.** Each moving order takes its own `NNN-declared.yaml`, the one whose name
   before `-declared.yaml` begins the order's file name, followed by `-`. If a declaration
   matches no order or more than one, the tool stops. Nothing else moves: `README.md`,
   `verify.md`, `verify.schema.json`, the shared schema, the three `*-staged/` directories, every
   order without a filled Record and every held-back order stay, so the runners find each live,
   deferred and written-ahead order, the schemas and the verifier where they are. A77's records
   are kept in one place, not beside their orders (the coordinator's, open to the owner's
   strike), so the rule is an order and its declaration; any other top-level file but the
   `*-staged/` directories whose name begins with a moving order's number and `-` (an
   `NNN-a77.yaml`, say) stops the tool.
3. **What is rewritten: link targets only.** In each moved Markdown file, every relative link
   target is rewritten so that it leads where it led: one level deeper for a target that stays,
   unchanged for a target that moves with it. In every other Markdown file, every relative link
   target that leads into a moved file is rewritten to its new place, with its anchor. This
   covers inline links and reference definitions (`[label]: path`, as engine/CONFORMANCE.md
   has), and an inline link whose text runs over a line break, which `check_docs.py` does not
   read as a link, so it would otherwise break unseen. Text inside code fences, permalinks to a
   commit and `ref:path` commands are left as they are. A path written as text in Markdown
   prose is left as written: such text sits in dated records, the full review's papers and the
   merged orders' own commands, and names the file where it was when it was written (the
   coordinator's, open to the owner's strike); the tool lists each one. A root-relative path to
   a moved file in a source that `check_docs.py` scans for path mentions (`tools/*.py`, the
   conformance manifest and indexes, `engine/` sources) is rewritten; none is expected while
   041a stays.
4. **Ruling records.** A link target in a ruling record under `docs/proposals/ruling-NN/` is
   rewritten by the tool like any other, as CONTRIBUTING.md allows, and nothing else in it
   changes (the coordinator's, open to the owner's strike). The report lists each such line,
   before and after, so that the coordinator reads every one before the merge.
5. **The record page is an index (ruling 90).** The rows of
   [docs/work-orders/README.md](README.md) whose order moved leave it. Each dropped row
   summarizes its order, whose file now holds the information (ruling 57). List in your report
   each dropped row, and the fact it states, that its order's file does not hold. The
   coordinator carries each into this order's Record before the merge (ruling 57). The archive's
   index,
   `docs/archive/work-orders/README.md`, lists every archived order; write it with
   `python3 tools/check_docs.py --write-archive-index`, order 212's option, whose form (number,
   title, Record link) is the index's form, so that 212's check of one row per file holds in both
   directories. A pull-request column waits for a change to the generator. If 212 added no
   generator, the tool writes the index in those three columns and you say so. That the archive's
   generated index is the record page's list of links to each archived `## Record` is the
   coordinator's reading of ruling 90, point 4, open to the owner's strike. Then make each
   sentence that says where a merged order's file or Record is true of the tree after the move,
   in as few words as do it, keeping order 212's wording where it is already true:
   - WORK_ORDERS.md, item 8 of "An order" and the opening of "The record": the rows of 001 to
     036k and 060 to 066 stay, those files having left the tree; from 037 on, a merged order's
     archived file is its record (ruling 90), and the Records of the merged orders that moved
     are linked from the archive's index (a link to `../archive/work-orders/README.md`); a
     merged order that a file staying in `docs/work-orders/` names, a held-back merged order
     counting as staying, stays there, its Record linked from the README's Merged orders table,
     until a later sweep moves it by the same rule; no row is written per order, and no order's
     file is removed;
   - docs/work-orders/README.md, its opening and "Shared and staged files": where 212's text
     says the merged orders are still in docs/work-orders/, make it true; where it is already
     true, leave it; bring 212's sentence that "a merged order that no live order names ...
     moves" to the rule of the bullet above, in its words;
   - docs/README.md: the `work-orders/` row says the live queue, with the merged orders that
     files staying there still name, and a new `archive/work-orders/` row says the merged
     orders, each with its Record;
   - docs/archive/README.md: under 212's `## Moved within the tree`, add a table with the header
     `| Path or group | What moved | Where it is now | Moved |` and one row: `docs/work-orders/`;
     the merged orders that order 213 moved (ruling 90); a link to `work-orders/README.md`,
     labeled `work-orders/`, whose rows list each file; and the date, with a permalink in the
     form of the rows of `## Removed from the tree` to the tree `docs/work-orders` at the order's
     base (`$B` under "Done when"), labeled with its short hash.
6. **The tool.** Create `tools/archive_work_orders.py`, Python 3's standard library and Git only:
   by default a dry run that prints what would move, what is held back and by which file, and
   changes nothing; with `--write`, it moves each selected file with `git mv`, rewrites as
   decision 3 says, drops the README's rows as decision 5 says, and prints the counts the
   report asks for. Run again after the move, its dry run selects nothing. Create
   `tools/tests/test_archive_work_orders.py`, on a temporary repository as
   `tools/tests/test_migrate_041a.py` does, showing at least: a filled Record moves and an
   absent or "Not yet run." one stays; an order named by a live order in a link, in prose and in
   a shell command is held; a merged order named only by another merged order that a live order
   names is held (two rounds of the selection); an order whose declaration's file name a live
   order names is held; an order whose path a non-Markdown file outside `docs/work-orders/`
   names is held; a declaration moves with its order; a link out of a moved file and a
   link into it, inline and as a reference definition, both lead where they led; a link whose
   text runs over a line break is rewritten; fenced code and a permalink are untouched; the
   README's row of a moved order is dropped. If the admin session appends its dry-run script
   under "From the admin session", start from it; its choices are then the coordinator's, and
   yours are those you change.

## The coordinator's dry run

The coordinator wrote a script to decisions 1 to 4 and ran it in a scratch working tree of
`main` at `85227914` (order 211's merge), before order 212; it is kept in the coordinator's notes
(private), and nothing of it was committed. Without 212, it wrote a minimal index of its own and
made none of decision 5's sentence edits. It found:

- 184 order files: 162 with a filled Record, 1 with "Not yet run." (051b2) and 21 with none
  (048b-join, 049a1, 050a, 050c to 050f, 050h to 050n, 051b3, 080, 144, 147, 203, 205b, 211).
  205b merged without a Record on `main`, and 050n had not yet run; each stays until its Record
  is filled.
- 65 merged orders held back, a held-back merged order counting as staying and the selection
  repeated until nothing changed: 038b, 041a, 042, 044, 047, 047b, 047c, 048a1 to 048a4, 048a5,
  048a5b, 048a5c, 048a5d, 048a5d2, 048a5ef, 048a5gh, 048b-relay1, 048b-relay2, 050g, 051b, 068,
  071, 077, 079, 081, 100, 105, 108, 117, 136, 142, 143, 145, 146, 148 to 150, 153, 156, 157,
  159, 160, 162, 163, 167, 171a, 171b, 172a, 172b, 174, 181, 185, 190, 195, 197, 199, 201, 202,
  205, 207 to 210. Each is named by a live order (048b-join, 049a1, the 0.2.0 chain's texts,
  051b2, 051b3, 080, 144, 147, 203, 205b or 211) or by an order already held (207 by 048a5gh,
  149 and 150 by 146, 048a5c by 209, and so on); 042 and 081 by the path of their declarations
  in held-back orders' commands; and 041a by `tools/migrate_041a.py`.
- 112 files moved, 97 orders and 15 declarations, and no stub: orders 037, 038a, 038c, 039, 040,
  041b, 045, 046a, 046b, 046c, 047d, 048a6, 050b, 067, 069, 069b, 070, 072 to 076, 078, 082,
  101 to 104, 107, 109 to 116, 118 to 135, 137 to 141, 151, 152, 154, 155, 158, 161, 164 to
  166, 168 to 170, 173, 175 to 180, 182 to 184, 186 to 189, 191, 192, 193a, 193b, 194, 196,
  198, 200, 204, 204b and 206, with the declarations of 038c, 039, 040, 041b, 045, 046a, 046b,
  046c, 047d, 048a6, 070, 075, 082, 177 and 206. Git saw each as a rename, the least similar at
  84 percent.
- 919 link targets rewritten inside the moved files and 47 in 20 other files, none of them a
  held-back merged order: CONTRIBUTING.md, docs/AUDIT.md, docs/guides/full-review.md, two pages
  of docs/implementation/, two of first-build's papers, two ruling records,
  engine/CONFORMANCE.md's two reference definitions, conformance/README.md and nine files under
  experiments/. In the ruling records: 5 in ruling 80's and 1 in ruling 87's, each a link target
  alone. 3 of the 47 were links in docs/implementation/METHOD_AS_RUN.md whose text runs over a
  line break: `check_docs.py` passed without them, and only the grep below found them.
- 89 rows dropped from the work-orders README; no path in a scanned source rewritten.
- 70 `#record` links before and after, outside the generated index.
- 132 paths to moved files left as written as text, in 28 of the full review's papers under
  `docs/proposals/full-review/`, which cite files and lines at the commit the review read; none
  in a file that stays in `docs/work-orders/`, and no file staying there names a moved order or
  declaration by its bare file name.
- `check_docs.py` (506 Markdown files, 9,098 local links), `check_rulings.py`,
  `check_roadmap.py`, `check_private_paths.py`, `check_schemas.py`, `check_model_account.py`,
  `promise_standing.py --check` and `set_names.py --check` all at Errors: 0; the tool tests, 130,
  OK; `git diff --check` clean; `vector_delta.py --declared-changed`, 0 changes, since a
  declaration under `docs/archive/` is never read again. `gate.py --plan` would run 16 gates,
  among them the engine's format, lint and tests and experiments 011 and 012's tests, because
  links under `engine/`, `conformance/` and experiment 012 change: cost, not risk.
- The top of `docs/work-orders/` kept 89 Markdown files, every live order, `verify.md`,
  `verify.schema.json` and the shared schema among them, and the directory's archive, as the
  runners extract it, went from about 7.3 MB to 4.9 MB.

Your counts will differ by what has merged or been written since `85227914`, order 212 above
all. Say in your report what differs from the lists above, and why.

## Scope

Create `tools/archive_work_orders.py` and `tools/tests/test_archive_work_orders.py`. Move the
files the tool selects, and change the link targets it rewrites, in whatever Markdown file holds
them, ruling records included, under decisions 3 and 4. Change by hand only the sentences of
decision 5, in `docs/implementation/WORK_ORDERS.md`, `docs/work-orders/README.md`,
`docs/README.md` and `docs/archive/README.md`, and write `docs/archive/work-orders/README.md` with
order 212's generator. Do not change the text of any order without a filled Record, nor any text
other than link targets in a merged order, a ruling record, the ledger, `IN_FORCE.md`, `spec/` or
under `experiments/`. Do not change `AGENTS.md`, `CONTRIBUTING.md` beyond link targets,
`tools/check_docs.py`, `tools/migrate_041a.py`, `.github/` or any Rust source. Anything else
you find that needs changing goes in your report.

## Done when

From the repository root, each passes:

```sh
python3 tools/archive_work_orders.py        # selects nothing: the move is done
python3 tools/check_docs.py                 # Errors: 0
python3 tools/check_rulings.py              # Errors: 0
python3 tools/check_roadmap.py              # Errors: 0
python3 tools/check_private_paths.py        # Errors: 0
python3 -m unittest discover -s tools/tests # OK
git diff --check                            # prints nothing
python3 tools/vector_delta.py HEAD WORKTREE --declared-changed   # 0 changes, 0 errors
```

Then each of these holds. The first prints the paths to moved files still written outside the
archive: each line must be in a file under `docs/proposals/full-review/` or inside a code fence
that quotes it as written. The second, over the files that stay in `docs/work-orders/`, prints
nothing: none names a moved order or declaration by its bare file name (A87).

```sh
M=$(mktemp); ls docs/archive/work-orders | grep -v '^README\.md$' | sed 's|^|work-orders/|' > "$M"
git grep -n -F -f "$M" -- ':!docs/archive/work-orders' | grep -v 'archive/work-orders/'
N=$(mktemp); ls docs/archive/work-orders | grep -v '^README\.md$' | sed 's/\.md$//' > "$N"
git grep -n -w -F -f "$N" -- docs/work-orders | grep -v 'archive/work-orders/'   # prints nothing
for f in README.md verify.md verify.schema.json 004-i2-01-vectors-and-first-start.schema.json; do
  test -f "docs/work-orders/$f" || echo "missing $f"; done                 # prints nothing
B=$(git merge-base HEAD origin/main)       # the order's base, before any checkpoint commit
git diff --name-status -M $B | grep -c '^R' # the number of files the tool moved
git diff --name-status -M $B | grep -c '^D' # 0
grep -c 'archive/work-orders/' docs/README.md                           # at least 1
grep -c 'counting as staying' docs/work-orders/README.md                 # at least 1
```

The `#record` links outside the generated index are as many as before:

```sh
R='\]\([^) ]*#record\)'; X=':!docs/archive/work-orders/README.md'
git grep -h -o -E "$R" $B -- '*.md' "$X" | wc -l       # before
git grep -h -o -E "$R" -- '*.md' "$X" | wc -l          # after: the same
```

And no order without a filled Record changed: for each top-level order file that
`git diff --name-only $B -- docs/work-orders` lists, other than `README.md`, its Record is
filled. Say in your report how you checked it.

Your report lists: the files moved, by number; each held-back order and the file that holds it;
each counted figure that the dry run above gives, beside yours; every changed line in a ruling
record, before and after; the paths left as written, by file; and every check's last line. The
`git mv` stages the renames; stage nothing else. The coordinator then has a fresh, read-only Opus
reader read the diff (ruling 79), runs the gate, and after the squash checks `git log --follow`
on three moved files.

## Record

Not yet run.
