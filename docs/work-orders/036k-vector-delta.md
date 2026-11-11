# Work order 036k: a report of what changed in the vectors

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count.

## What and why

Spec batch 1's build orders change several hundred vector files, many by a scripted transform
([the batch 1 plan](../proposals/spec-batch-1/README.md), its coordinator task K2 and orders 040
and 041a). Their reviewers need to see, per case and per step, what an order changed in the
expectations, and whether each change is one the order declared. No expectation may be regenerated
from engine output: a changed outcome must be meant. This order writes the tool that shows it.

## What to do

1. **`tools/vector_delta.py`**, Python 3 with the standard library (and PyYAML only if the case
   files need it; they are JSON written as YAML, and some hold characters PyYAML refuses, so prefer
   `json`). It compares the vector cases under `conformance/` between two Git revisions (default:
   `origin/main` and the working tree) and reports, per case file and per step:
   - a case added, removed or renamed (by its `id`);
   - every **flip**: a step whose expected reply changes its `reason`, its `outcome`, whether it is
     accepted, its `tool`, or `stateUnchanged`, printed as before and after;
   - every other change to an expectation, summarized by its JSON path;
   - changes to inputs and files that a case reads (fixtures), summarized by path.
   Read `conformance/README.md` ("Inputs and files", "Assertions") for what a case holds.
2. **Declared shapes.** An order may pass a file of declared shapes (`--declared FILE`, YAML or
   JSON): a list of entries, each naming a kind of change the order means (for example "a step's
   reason goes from refused to conflict in these cases", or "these JSON paths gain these keys in
   every S1 registration case"), with case globs. The report marks each change as declared or
   undeclared, and the tool exits with status 1 when any flip or undeclared change remains, so a
   verifier can run it as a check. Design the declaration format simply and document it in the
   tool's help and in `conformance/README.md`.
3. **Tests** in `tools/tests/`, on small cases written in the test: an added case, a flip of each
   kind, a declared and an undeclared change, a renamed case, and an unreadable file reported
   rather than crashing.
4. **Today's tree**: run it between `origin/main` and the working tree (no change: empty report,
   status 0) and between the commit before order 033 and after it (paths only: report what it
   finds), and put both last lines in your report.

## Scope

Create: `tools/vector_delta.py`, its tests. Change: `conformance/README.md` (one short section on
the tool). Nothing else; report anything else that needs changing.

## Done when

From the repository root: `python3 tools/check_docs.py`, `python3 tools/check_rulings.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py`, `python3 -m unittest discover -s tools/tests` (with the
interpreter that has PyYAML) and `git diff --check` pass. Do not stage anything.
