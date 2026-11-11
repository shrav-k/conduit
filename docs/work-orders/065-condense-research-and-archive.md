# Work order 065: the research notes and the archive condense

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Action item A80 carries the rest of [the condensation plan](../proposals/condensation/plan.md)
(ruling 58; read its sections `target_structure`, `removal_list` (list E), `extractions` and
`sequence`, and its [critic](../proposals/condensation/critic.md)). This order is the plan's step 9:
`docs/research/` and `docs/archive/`, with extractions E25, E26 and E27.

The plan predates later work. Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger,
the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to
extract into them is present, and extract only what is missing. The plan's line numbers refer to
**C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line
in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

## What to do

1. **Extract first**, as E25, E26 and E27 say: the archive's index rewritten with a row for each
   removed group, what it was and what it is source material for, FORMATS' paragraph, the
   collaborator proposals' "Not adopted, and why" verbatim, "Prior art still applied" with C0 links,
   and the attribution (E25); `docs/archive/plan-audit-history.md` condensed to what E26 keeps, its
   multi-system options linked to the ledger's appendix, which order 034 already holds; and
   `docs/LANGUAGE_DEFECTS.md` as E27 says, its 30 open type-layer rows and 16 runtime rows kept in
   full and every other row a crosswalk line, with "None has a vector" checked against the manifest.
   `docs/research/SIMPLIFICATION.md` stays as written. Where an extraction's destination was already
   written by an earlier order, check it and add only what is missing.
2. **Then remove** the files removal list E names under `docs/research/` and `docs/archive/` (including
   `tools/check_doc_migration.py` with the migrations, its CONTRIBUTING block and its row in
   `docs/README.md`, and the reviews only because the ledger already holds "Rulings before the plan
   audit"), and only those. Before deleting each, check that since C0 it changed only in link targets or path
   mentions, which earlier orders (033 to 063) repointed (`git diff --word-diff f1cb473 HEAD -- <path>` shows only those); if anything else
   changed, stop and report that file, and go on with the others.
3. **Repoint every link and mention** of a removed file or section, found by path, by bare file
   name, by section name and by row ID, in the plan's three classes: to the new home of text this order moved; to a live destination for a sentence that says
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

Change: `docs/research/`, `docs/archive/`, `docs/LANGUAGE_DEFECTS.md`, `tools/check_doc_migration.py` and its test if any, `CONTRIBUTING.md` and `docs/README.md` (only the migration lines), `ROADMAP.md` (a link target), and links and path mentions elsewhere that point into what this order removes. Do not change `spec/`, `IN_FORCE.md`'s decisions, a ruling's decisions or an owner's words. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py` (it checks the C0 permalinks against Git
history), `python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass. Do
not stage anything.
