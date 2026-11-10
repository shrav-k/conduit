# Work order 026: the entry documents, read cold and brought up to date

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; give the reading's findings in its `findings`, and leave
`vectors` empty. This order has no network access and needs none.

## What and why

The owner finds the documents unclear, and the entry documents are out of date: the top
[README](../../README.md) says the repository "holds no engine" and that the reference model "has
no core yet", and [AGENTS.md](../../AGENTS.md) begins "No Conduit engine exists yet". Since then
the first build has a strict loader, a core on S1 and an engine shell that ran the real Claude
Code and Codex as Agents ([the roadmap](../../ROADMAP.md#the-order-of-the-work), step 3). The
owner's collaborators on the repository are to be its next cold readers, and need written
instructions.

## What to do

1. **Read cold first.** Before changing anything, read the entry documents as a newcomer would:
   `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`, `docs/OVERVIEW.md`,
   `ROADMAP.md` down to its register, `reference-model/README.md`, `docs/guides/README.md`, and
   `spec/README.md` down to its contents. For each place where a newcomer would be misled, lost
   or stopped, note it as a finding: the file and line, what is wrong (stale, unclear, missing,
   contradicting another document) and the fix. Keep them in your report whether or not you fix
   them.
2. **Bring them up to date.** Fix every stale statement in your scope from the records it
   summarizes, never from memory: the roadmap's step 3, the logs under
   `docs/proposals/first-build/`, `reference-model/README.md` and `CONFORMANCE.md`, the ruling
   log in `docs/AUDIT.md` (rulings 48 and 49 above all), `docs/work-orders/README.md`, and
   `experiments/README.md`. Each state you write carries its date. Keep each document's role: the
   top README says what Conduit is and what exists, with links; `AGENTS.md` instructs agents and
   workers, and its rules and checks do not change in substance; `CONTRIBUTING.md` instructs
   people.
3. **Make the way in clearer.** Add to the top README a short "Start here" for a person new to
   the project: three or four links, in order, with one line on each, for a reader who wants to
   understand the language, one who wants to run what exists, and the owner's
   [setup guides](../guides/README.md). Do not restructure the documents beyond that, and keep
   the house style: plain sentences, the terms of `spec/definitions.md`, no marketing.
4. **Write the cold readers' instructions**, as `docs/guides/cold-readers.md`, for the owner's
   collaborators, who are to read the text as the first build leaves it (the "Read" row of the
   roadmap's order of the work). Build them on
   [for the next reader](../AUDIT.md#for-the-next-reader) and link to it rather than repeat it:
   what to read and in what order, what is known to be open, how to report a finding (where it
   goes and its form, as round 6's records show), what not to do (change `spec/` directly, read
   the reference clones, publish anything), and how long a read takes, if the records say. Add
   it to the table of `docs/guides/README.md`.

## Scope

Change: `README.md`, `AGENTS.md`, `CONTRIBUTING.md`, `docs/README.md`, `docs/OVERVIEW.md`,
`reference-model/README.md`, `docs/guides/`. Create `docs/guides/cold-readers.md`. Do not change
`spec/`, `ROADMAP.md`, `docs/AUDIT.md` or any record under `docs/proposals/`: report what needs
changing there as findings.

## Done when

The entry documents state what exists as the records do, each with its date; the top README has
its "Start here"; the cold readers' instructions exist and are linked; the reading's findings are
in the report. From the repository root, `python3 tools/check_docs.py`,
`python3 tools/set_names.py --check`, `python3 tools/check_schemas.py`,
`python3 tools/check_roadmap.py` and `git diff --check` pass. Check the files you create for
trailing white space yourself; do not stage anything.
