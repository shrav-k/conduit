# Work order 064: the language-shape, simplification and Project papers leave the tree

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Action item A80 carries the rest of [the condensation plan](../proposals/condensation/plan.md)
(ruling 58; read its sections `target_structure`, `removal_list`, `extractions` and `sequence`, and
its [critic](../proposals/condensation/critic.md)). This order is the plan's steps 7 and 8: removal
list D's papers that go now, with extractions E21, E22 and E23, and the Project investigations'
papers, which ruling 58 (point 8) lets go once the Project rulings are recorded: rulings 59 to 62
and 66 are.

The plan predates later work. Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger,
the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to
extract into them is present, and extract only what is missing. The plan's line numbers refer to
**C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line
in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

Since the plan: the brief E23 describes was written as `docs/proposals/projects/brief.md` (not under
`https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project`); ruling 63 settled the rerun's counting rules (order 035 put them in the
measurement plan), so E11 adds only what ruling 63 did not settle; order 063 did E13's planned work
in `conformance/README.md`, so E36's to-do from `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research/map-constraints.md` section 1 goes there only if it
is missing. Keep `language-shape/short-form/synthesis.md` and `critic.md`: spec order 037 still
works from them (the plan's "later, after the spec batches").

## What to do

1. **Extract first**: E21 into `docs/proposals/simplification/README.md` ("Rejected, and why",
   copied verbatim from the synthesis, so that you do not select; the load-bearing guarantees; the
   v1-candidate reductions with their measured costs); E22's closing section "Carried from the
   papers" in `short-form/synthesis.md`; E23: check that `docs/proposals/projects/brief.md` carries
   its provenance quotes (verbatim, with their sources), every synthesis decision 1 to 20 with the
   options and why each rejected one lost, and the other points E23 lists, and add, verbatim and
   with the source, only what is missing; E9's and E11's parts not yet carried; E36's to-do if
   missing.
2. **Then remove**: `https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/simplification/research` (all 13 files); `language-shape/short-form/`'s
   design-N1 to N4 and its four inventories; and `https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/language-shape/project` (all 10 files). Remove
   nothing else. Before deleting each, check that since C0 it changed only in link targets or path
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

Change: `docs/proposals/simplification/`, `docs/proposals/language-shape/`, `docs/proposals/projects/brief.md`
(only what E23 still lacks), `docs/proposals/first-build/increment-4/measurement-plan.md` (E11),
`conformance/README.md` (E36's to-do, if missing), `docs/archive/README.md`, and links and path
mentions elsewhere that point into what this order removes (the spec batch 1 papers included: their
targets may change, their words may not). Do not change `spec/`, `IN_FORCE.md`'s decisions, a
ruling's decisions or an owner's words. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py` (it checks the C0 permalinks against Git
history), `python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass. Do
not stage anything.

## Record

Run on 2026-11-11: the worker in 14 minutes, then a fresh verifier in 5, then a second round. The
worker removed 31 papers, the simplification's research, the short form's designs and inventories,
and the Project investigations' papers, a net 270,808 fewer words, after extracting E21 into the
simplification's record, E22 into the short form's synthesis and what the Projects brief lacked of
E23. The verifier found four passages copied verbatim that had lost a space, and row IDs, such as
PRJ-010 and AUT-018, still pointing at definitions only the removed papers held; the second round
restored the spaces, compared all 31 extracted passages with C0 by script, and linked 47 row
references to their C0 definitions. The short form's synthesis and critic stay until spec order 037
lands.
