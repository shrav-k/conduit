# Ruling 90: the README and the documents refreshed at the first dogfood milestone, in two orders, and a merged order's archived file as its record

Status: in force; directive (A86, A87: the entry documents at milestone 1, then the archive move) (ruled by the owner on 2026-11-19, as the coordinator's notes date the answers). Holds now: [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed) (points 1 to 9), and the rows of A80, A86 and A87 in [the tracker's action items](../../AUDIT.md#p2--prune-and-re-baseline-under-the-scope-guard) (points 1 to 9). This record says why.

On 2026-11-19, after [ruling 89](../ruling-89/README.md), the owner said that the README and the
structure of the documents were out of date, and asked whether a Fable review should plan a new
README and documentation structure, to be published at the dogfood checkpoint. The coordinator
put questions to the owner through the session's question tool, in two sittings, each with
options and a recommendation. The owner chose the recommended option in each. The owner's words,
the questions as the coordinator's notes (private) hold them and the options chosen follow,
verbatim. The notes date them 2026-11-19.

## The questions and the answers

### The README and the structure of the documents

The owner: "The readme and docs structure is very out of date, should we do a fable review to fix
this and publish a new readme and docs structure when we reach the dogfood checkpoint?"

1. As put: "How should the README/docs refresh run?"

   The owner chose "Review now, apply at checkpoint (Recommended)".

2. As put: "Who should the new README speak to first?"

   The owner chose "Visitors, then contributors (Recommended)".

3. As the notes hold it: "... Should the review plan to archive the merged/old ones out of the
   main paths?"

   The owner chose "Plan it, keep links working (Recommended)".

### The docs plan's decisions

The review then ran (below), and its plan put eight decisions. The coordinator put four of them
to the owner:

4. As the notes hold it: "... Ship them as one order or two?"

   The owner chose "Two orders (Recommended)".

5. As the notes hold it: "Ruling 58 point 4 ... should the archived file itself be the record (the
   record page links each file's Record section), retiring the per-order rows?"

   The owner chose "Archive file is the record (Recommended)".

6. As the notes hold it: "docs/OVERVIEW.md ... What should the docs order do with it?"

   The owner chose "Fix now, full re-read later (Recommended)".

7. As put: "How long should the new README be?"

   The owner chose "~110 lines with quick start (Recommended)".

The notes hold questions 3 to 6 in part, the elisions marked as they mark them, and every option
by its label alone; they hold no option the owner did not choose. The other four decisions of the
plan the coordinator took, open to the owner's strike, as the notes record: the link targets in
ruling records repointed by script, a minimal edit of the non-normative lines of
`spec/README.md`, A77's records in one place, and DA12's wording (points 6 to 9). Each option the
owner chose was the plan's recommendation; the details beyond each label, in points 2, 3 and 5
(the README's outline, what each step holds and when step B runs, and the Overview's named
lines), are the plan's recommendation as the coordinator applies it, by the coordinator's
reading, open to the owner's strike, since the notes hold the labels alone.

## The review, in brief

The review that answer 1 started ran on Claude Fable, read only, over four dimensions: a
staleness inventory of the entry and live documents, their audience and structure, an archive
plan for `docs/work-orders/` and `docs/proposals/`, and a drafted README and documentation map;
then a synthesis, the plan, and an adversarial check of the plan, whose thirteen corrections the
plan took whole or narrowed. Its reports, plan and check are the coordinator's notes (private).
It read `main` at `45026c21` and re-verified each correction at `9c313097`. Its findings, as the
coordinator summarizes them:

- Every document a visitor meets was last brought up to date between 2026-11-10 and 2026-11-12,
  while `main` has since taken rulings 68 to 89, spec batch 1, the sign-in chain and the dogfood.
- README.md says the language has five families, where
  [the definitions](../../../spec/definitions.md) have six, with the Fleet, and it never names
  IN_FORCE.md, the second of [ruling 67](../ruling-67/README.md)'s four documents.
- The documentation map lists ruling records up to ruling 67.
- The same state of the build is written in six places, at six dates.
- `docs/work-orders/` holds 182 order files, of which 160 carry a filled Record and have merged;
  its README lacks a row for fifteen of them, and the archive's index has no row for the 45
  orders that order 067 removed, which [ruling 58](../ruling-58/README.md), point 9, owes.

The plan: one order at the milestone rewrites the entry documents, and a second moves the merged
orders whole into `docs/archive/work-orders/`, every link to them rewritten in the same change
and proven by `tools/check_docs.py`.

## The decisions

1. **The review now, its changes at the checkpoint** (answer 1). The review ran on 2026-11-19 and
   changed nothing in the tree; its changes land at the dogfood checkpoint, which is
   [ruling 89](../ruling-89/README.md)'s milestone 1: the Agents of the owner's primary Claude and
   Codex accounts signed in through Conduit on the Mac, after orders 050n and 051b-2. By the
   coordinator's reading:
   - the review is one of the owner's reviews of the repository's structure, started by hand,
     which [ruling 79](../ruling-79/README.md), point 1, keeps on Fable, since point 1 names the
     repository's structure and this review is of the structure of its documents; unlike the
     targeted reviews of rulings 81 and 89, of one code surface and of the readiness for the
     dogfood, each of which amended point 1 for that review alone, it amends nothing there;
   - nothing of the refresh lands on `main` before the checkpoint, this record and the lines that
     carry it aside; the plan's proposal to add the owed archive row or a proposals index early is
     dropped;
   - the orders' texts are written from the plan, their facts refreshed on the day (the vector
     count, the packages' version, which orders have merged and which Records are filled), and
     less prescriptive, as ruling 89, point 2, has it for the chain: exact text is prescribed for
     the decisions and the README's and the map's drafts, not for every line.
2. **The README speaks to visitors first, then contributors, in about 110 lines with a quick
   start** (answers 2 and 7): what Conduit is and why, what works on the stated date, a quick start
   on a Mac, one example of the language, where the work is going, how the repository is
   organized, contributing, and the license. By the coordinator's reading:
   - the quick start and the "what works" lines state only what has run on the day, each fact
     taken from its source file, as the archive's index has it for a quick start that cannot run;
   - the documentation map routes a reader by what they came for and lists no single order and no
     single ruling;
   - the contributor's material leaves the README for AGENTS.md and CONTRIBUTING.md, and AGENTS.md
     stays near 150 lines, as ruling 67 has it, keeping its list of checks and the two worker
     rules that live orders invoke by name (the coordinator's plan, open to the owner's strike).
3. **Two orders** (answers 3 and 4). The archive keeps every link working.
   - **Step A, the entry documents, at the checkpoint**: README.md, the documentation map,
     AGENTS.md, CONTRIBUTING.md, the tracker's and the roadmap's live tops, the Overview's named
     lines (point 5), the engine's README and live S1's steps, `spec/README.md`'s non-normative
     lines (point 7), the guides and the method's pages brought to the checkpoint, the record
     page's new form among them (point 4); the indexes repaired: the fifteen missing rows, the
     directory's README split into live and merged tables, the archive row ruling 58 owes, the
     archive's index split, a new index of `docs/proposals/`, and
     `docs/archive/work-orders/README.md` created empty; and a check that each order file has one
     row in its directory's index. No order file and no declaration moves. It is action item A86.
   - **Step B, the archive move, in the next quiet merge window after step A**: the merged orders
     move whole into `docs/archive/work-orders/` under the same file names, each with its
     declaration, every link to them rewritten by script in the same change. It is action item
     A87.

   By the coordinator's reading, open to the owner's strike:
   - step A lands once 050n and 051b-2 have merged, and 048b-join, 051b-2's base, with them, and
     between other merges, since every Record-filling merge edits the directory's README;
   - a quiet window is one with the merge queue empty and no order between the end of its run and
     its merge, since a merge that fills a Record touches a file the move renames; if a merge comes
     during it, the script runs again on a fresh branch and nothing is resolved by hand;
   - a merged order moves when its Record is filled and no file staying in `docs/work-orders/`
     other than its README links it or names it, by path or by file name, in text or in code, a
     merged order held back counting as staying and the selection repeated until nothing changes;
     step B drops the README's row of each moved order, and the archive's index lists it; an
     order without a filled Record does not move, and neither does order 041a, whose declaration
     a tool names, until an order edits that tool with it;
   - the directory's README, the verifier's order and its schema, the shared report schema, the
     staged directories and every open order stay at the top of `docs/work-orders/`, where the
     coordinator's runners read them from `main` (the coordinator's notes, private), so the runners
     change in nothing;
   - nothing under `docs/proposals/` moves: the ledger, every ruling record, `spec/` and IN_FORCE.md
     stay where they are, and working papers condense at their close, under ruling 58, point 10;
   - an order held back moves at a later condensation by the same rule, and so does each order
     that merges after step B (point 4).
4. **A merged order's archived file is its record** (answer 5). A merged work order's file, moved
   whole with its Record into the archive, is its record; the record page,
   [the record](../../implementation/WORK_ORDERS.md#the-record) in work orders, links each archived
   file's Record and becomes an index of links; no row per order is written for it. This amends
   ruling 58, point 4. By the coordinator's reading, open to the owner's strike:
   - the rows of orders 001 to 036k and 060 to 066 stay as written, since those files left the tree
     and the rows, with the Records below them, are their record;
   - no row is owed for order 037 or any later order, so what remains of A80 for merged orders is
     step B and the moves after it;
   - until an order moves, its file in `docs/work-orders/` holds its Record, and the record page
     links it there: step A rewrites the record page's form and links each merged order's Record
     in `docs/work-orders/` (or the work-orders README's "Merged" table that it links, until step
     B drops a moved order's row there and the archive's index lists it), and step B's script
     repoints those links;
   - a move is listed in the archive's index, in a table of files moved within the tree beside the
     table of removed files that ruling 58, point 9, keeps;
   - where A77's row reads "each order's row" in the record, an order after 066 has its Record in
     its own file, which the page links; and
   - every live passage that states ruling 58 point 4's old rule (WORK_ORDERS.md's record page, the
     directory's README, HANDOVER.md's entry on ruling 58, the condensation plan's step for merged
     orders, ROADMAP.md's A77 row, WORK_ORDERS.md's step 8) reads as
     amended here until step A rewrites them; they are left to step A because the chain's merges
     edit the directory's README, and no row is written meanwhile.
5. **The Overview fixed now, read again in full later** (answer 6). Step A fixes the Overview's
   named lines: its preamble cut to one line, the six families with the Fleet, its dated paragraphs
   pointed at the tracker, and a heading that no file links dropped. A full re-read against spec
   batch 1 follows as an order of its own after the checkpoint, tracked under A86; it discharges
   the condensation plan's later trigger "Rewrite OVERVIEW". Its path stays.
6. **The link targets inside ruling records are repointed by script** (*Coordinator's*, open to
   the owner's strike). Step B's script rewrites the targets of links into moved orders in ruling
   records, nine at the plan's count, and step A's split of the archive index keeps the heading
   that ruling 45's roadmap log links (`#additional-roadmap-candidates`) as a pointer, or
   repoints that link and lists it. A link target is not the owner's words or a decision as
   ruled, which ruling 67 keeps unrewritten, and CONTRIBUTING.md lets a condensation change link
   targets; each order's report lists every such edit, and the coordinator reads the diff.
7. **A minimal edit of the non-normative lines of `spec/README.md` in step A** (*Coordinator's*,
   open to the owner's strike): its status lines, the passage that speaks of the protocol
   documents as present, its dated coverage line, its line on what AGENTS.md's checks say, and its
   list of where the reasons are, brought up to the checkpoint. No promise and no normative text
   changes; the local gate runs the engine's checks for a change under `spec/`.
8. **A77's records in one place** (*Coordinator's*, open to the owner's strike). Each order's A77
   record, `NNN-a77.yaml`, is kept in one place and not beside its order, so that the archive's
   rule moves an order and its declaration and nothing else. Step A names the place by a dated
   amendment of [A77's registration](../../implementation/WORK_ORDERS.md#the-record-each-order-keeps)
   and moves there any record already written. By the coordinator's reading, the amendment changes
   where a record is kept, not what it holds, which orders it covers or how they are judged.
9. **DA12's wording** (*Coordinator's*, open to the owner's strike). DA12 of
   [the documents audit](../docs-audit/README.md), "The same list of four checks in `AGENTS.md`
   and `CONTRIBUTING.md`", becomes: the commands in AGENTS.md, the requirements of each in
   CONTRIBUTING.md. It takes effect with step A, which moves the checks' requirements to
   CONTRIBUTING.md and adds a note beside DA12.

## What it amends

- **Ruling 58, point 4**: a merged work order's file no longer leaves the tree once its row is in
  the work-order record; moved whole into the archive with its Record, it is its record, and the
  record page links it (point 4).
- **DA12** of the documents audit, accepted under ruling 58, point 1, as point 9 words it, with
  step A. *Coordinator's*, open to the owner's strike.

## What it does not change

- **Ruling 57**: a document that does not bind is edited, condensed or removed only once every
  relevant piece of it is preserved. - **Ruling 58's other points**: closed increments' logs
  become the settlements register (point 3), evidence stays (point 5), each removal has a row in
  the archive's index (point 9), and working papers condense at close (point 10). - **Ruling 67**:
  the four documents and their order, AGENTS.md near 150 lines, and the owner's words and the
  decisions as ruled never rewritten. - **Ruling 79**, point 1 (point 1 above). - **Ruling 89**:
  the milestones, and the levers for the chain. - **The paths of `spec/`, IN_FORCE.md, the ledger
  and every ruling record.**

## What is left for the owner

Nothing new. The coordinator's readings and points 6 to 9 are open to the owner's strike.

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md), topic 15: the line on the work-order record (point 4), and
  a line on the refresh and the archive (points 1 to 3 and 5 to 9). -
  [The ledger](../ruling-log.md) gains row 90, and ruling 58's status, there and in
  [its record](../ruling-58/README.md), notes the amendment. -
  [The tracker](../../AUDIT.md#p2--prune-and-re-baseline-under-the-scope-guard): A80's status
  (point 4) and the rows of A86 (step A, with the Overview's re-read) and A87 (step B). -
  [The roadmap](../../../ROADMAP.md#the-order-of-the-work) cites A86 and A87 in its order of the
  work. - Later, and not in this change: the two orders' texts, written from the plan and
  refreshed on the day; step A, with the amendment of A77's registration and the note beside DA12;
  step B; the Overview's full re-read.
