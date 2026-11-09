# Work order 008: the results of experiment 008's follow-up, a later turn

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `005-real-client-results.schema.json`, the schema
of the experiment's first results. You have no network access, and need none.

## What and why

The admin session has run the 12 runs of the follow-up of experiment 008, "a later turn", which
the [pre-registration](../../experiments/008-real-client/README.md#follow-up-a-later-turn-pre-registered-on-2026-11-09)
states, with the harness that [order 007](007-real-client-later-turn.md) extended. This order
classifies them and writes their results, as [order 005](005-real-client-results.md) did for
the first 68. The admin session gives the directory of the records below this order, outside
the repository; never copy its absolute path into a file.

## The decisions to apply

1. Order 005's decisions 1, 2, 4 and 6 apply unchanged, for each turn apart: what reached the
   client, what the client gave its model, and what the model did next.
2. **The question of the follow-up**, answered per client and case, each answer citing runs: in
   the second turn, did the model see and use the tool that opened during the first (C5-later),
   and did it stop offering, or still call, the tool that closed (C1-later)? Say what the stub's
   log shows each client fetched between the turns, and what each client's events show its model
   was given in the second turn.
3. Write `experiments/008-real-client/results-2026-11-09-later-turn.md`, with its trimmed
   evidence under `experiments/008-real-client/evidence-2026-11-09-later-turn/`, every absolute
   path replaced and no credential of any kind; link it from the pre-registration's "Results"
   and from the first results' summary; and add to the roadmap's A71 row and the register's 008
   row that the follow-up ran, with a link.

## Scope

Only the files named above.

## Done when

Every run is classified per turn or listed as not run as planned; every claim cites runs; no
absolute path, user name or credential is in any file you write; and `python3
tools/check_docs.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 7 minutes. 12 runs, 24 turns classified, none excluded; results and 37 trimmed
  evidence files.
- **Verifier**: 2 minutes. No fault found.
- **Admin**: read the second turn of all 12 runs against the classification, and recorded its
  review in the results. No second round.
