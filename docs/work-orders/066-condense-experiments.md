# Work order 066: the experiments keep their method and results, and shed raw records

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Action item A80 carries the rest of [the condensation plan](../proposals/condensation/plan.md)
(ruling 58, point 5: dated results, the method as run, the prompts and the counted artifacts stay as
v0's evidence; raw run records leave the tree with links to their last version). This order is the
plan's step 10: its section `experiments`, with extractions E28 and E29. Read that section first: it
says, per experiment, what stays and what goes.

The plan predates later work. Orders 034 and 035 have already done E1, E2, E3 and E19 (the ledger,
the tracker, the cold readers' guide and `CONTRIBUTING.md`): check that what removal list A asks to
extract into them is present, and extract only what is missing. The plan's line numbers refer to
**C0**, `f1cb473493fbb7b8667686d6fdf639ae96579ec3`; find each passage by its text at C0, not by line
in today's tree. The plan's order numbers are older than today's ("WO-033" there is this order).

## What to do

1. **Extract first**, as E28 and E29 and the section `experiments` say: each experiment keeps its
   README, dated results, HARNESS.md where a harness ran, its evidence READMEs and its counted
   artifacts; links into removed raw files become C0 permalinks; the stale lines the plan names are
   fixed; `experiments/README.md` gains "Evidence for v0" (A75) and the raw-records rule, and its
   001 row moves to the archive's index. **Do not touch `experiments/011-short-form-spike/`** (work
   order 039 is changing it) or experiment 010's dated results. A file a live proposal cites as
   evidence (for example `docs/proposals/metrics/README.md` citing experiment 008's later-turn runs)
   may go only with its citation repointed to C0.
2. **Then remove** the raw records and superseded code the section `experiments` names, and only those (each
   experiment directory that loses its code also leaves continuous integration's matrix, which finds
   `Cargo.toml` files by itself; say which). Before deleting each, check that since C0 it changed only in link targets or path
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

Change: `experiments/` (except 011), `docs/archive/README.md` (rows), and links and path mentions elsewhere that point into what this order removes. Do not change `spec/`, `engine/`, `IN_FORCE.md`'s decisions, a ruling's decisions or an owner's words. Report anything else.

## Done when

From the repository root: `python3 tools/check_docs.py` (it checks the C0 permalinks against Git
history), `python3 tools/check_rulings.py`, `python3 tools/set_names.py --check`,
`python3 tools/check_schemas.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass. Do
not stage anything.

## Record

Run on 2026-11-11: the worker in 12 minutes, then a fresh verifier in 2, which found nothing to fix.
The worker removed 942 files, 148,612 words: the raw run records of experiments 005, 007, 008 and
009, and the harness crates of 008 and 009, which the engine's shell and the Agent packages
superseded, so they leave continuous integration's matrix. Each experiment keeps its README, dated
results, HARNESS.md and evidence READMEs, 38 files and 46,100 words, with links into what left
pointing at C0; `experiments/README.md` gained "Evidence for v0" and the raw-records rule. Experiment
011 was not touched. One package test fixture's `source` note now names its C0 path; the admin ran
that package's tests: 6 pass.
