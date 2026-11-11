# Work order 061: the logs of rulings 38, 41, 42 and 43 leave the tree

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Action item A80 carries the rest of [the condensation plan](../proposals/condensation/plan.md)
(ruling 58; read its sections `target_structure`, `removal_list`, `extractions` and `sequence`, and
its [critic](../proposals/condensation/critic.md)). This order is the first half of the plan's step 5,
"ruling and batch records" (removal list C): the editors' logs of rulings 38, 41, 42 and 43, with
extractions E16, E17 and E35 for those logs. Order 062 does the other half.

The plan predates later work. Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger,
the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to
extract into them is present, and extract only what is missing. The plan's line numbers refer to
**C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line
in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

## What to do

1. **Extract first**: E16 (ruling 41's planner's choices P1 to P20 into `ruling-41/README.md`,
   anchor `#planners-choices`); E17 (`ruling-41/open-decisions.md` condensed in place, "For the
   owner" kept verbatim with its anchors; "Left for round 5" dropped only for items you find
   settled, the rest reported for the tracker's "Known to be open"); E35's "Choices that set
   behavior" sections in the READMEs of rulings 41, 42 and 43 from their logs, one line per choice
   that names a rejected alternative, a candidate for later, or a reading a promise does not state,
   copied verbatim where it fits, with a count per log; MX-21 of ruling 43 into the ledger's
   appendix. Check that E19's parts from `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-41/PLAN.md` are already in `CONTRIBUTING.md`
   (order 035) and add only what is missing.
2. **Then remove** the files the list names, and only those. Before deleting each, check that
   since C0 it changed only in link targets or path mentions, which orders 033, 034, 035 and 060
   repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else
   changed, stop and report that file, and go on with the others.
3. **Repoint every link and path mention** into a removed file or section, in the plan's three
   classes: to the new home of text this order moved; to a live destination for a sentence that says
   where future work goes (list each for the admin); otherwise to its C0 permalink,
   `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/<path>#<anchor>`
   (`/tree/<C0>/<directory>` for a directory, with no trailing slash). Links in the ledger
   (`docs/proposals/ruling-log.md`) and in records are repointed too: their targets may change,
   their words may not.
4. **The archive's index**, `docs/archive/README.md`: one row per removed directory or group.
5. **Report** the words removed and kept, the extraction counts, every link repointed by class, and
   `git grep -n -F '<removed path>'` for each removed path, which must show only C0 permalinks and
   archive rows.

## Scope

Change: `docs/proposals/ruling-38/`, `ruling-41/`, `ruling-42/`, `ruling-43/`, `docs/archive/README.md`, `CONTRIBUTING.md` (only what E19 still lacks), the ledger's appendix (MX-21 and link targets), and links and path mentions elsewhere that point into what this order removes. Repoint `spec/core/README.md`'s links to `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/ruling-38/check.md` and its neighbours to `ruling-38/README.md`. Do not change `spec/` (except a link target in a comment), `IN_FORCE.md`'s decisions, a ruling's decisions or an owner's words. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py` (it checks the C0 permalinks against Git
history), `python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass. Do
not stage anything.

## Record

Run on 2026-11-11. The first dispatch stopped at the order's guard, as written: 27 of the 30 files
to remove had changed since C0, only in links that orders 033 and 060 repointed; the admin let
files whose only changes are repointed links go, and dispatched it again. The worker then took 21
minutes and a fresh verifier 5. It removed 30 files, the logs of rulings 38, 41, 42 and 43, for a net
100,954 fewer words, with ruling 41's planner's choices, its open decisions condensed in place and
each ruling's "Choices that set behavior" kept. The verifier found one choice missed (F3's rejected
option (b) and its reason, in ruling 42's integration log) and two links that pointed at an
extraction where the history meant the removed text itself; the admin added the choice and linked
both to C0. The shell's loopback tests could not run in the worker's sandbox; no code changed.
