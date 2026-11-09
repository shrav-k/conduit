# Work order 005: the results of experiment 008 (A71)

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `005-real-client-results.schema.json` beside this
file. You have no network access, and need none.

## What and why

The admin session has run the 68 runs of experiment 008, what a real client shows its model
(A71), with the harness that order 002 built. Its
[pre-registration](../../experiments/008-real-client/README.md) fixes what the results must
show and, in its method's step 4, the categories by which each run's next action is classified.
This order reads every run's records, classifies them, and writes the dated results. The admin
session gives the directory of the records below this order, outside the repository; read it,
and never copy its absolute path into a file.

## The decisions to apply

1. **Wire, client and model apart**, as E17 asks: for each run, what the stub's log shows
   reached the client (`stub.jsonl`), what the client's events show it gave its model
   (`events.jsonl`, `client-exposed.jsonl`), and what the model did next (`final.txt` and the
   events). Never infer what the model saw from the wire alone.
2. **The next action** of each run, by the pre-registration's categories and no others: the
   model retried the same call; told the user the call was refused, failed or unknown, in those
   words or the same meaning; called another tool; reported success that did not happen; or
   something else, quoted. Each classification cites the event or line that shows it.
3. **The questions the pre-registration asks**, answered per client and case, each with the
   runs that show it: whether the client's tool list followed the server's without a restart,
   when a Conduit closed (C1) and when one opened (C5), under each cache and `ttlMs` variant;
   what the model was told on a refusal, a failure and an unknown outcome, and whether any of
   Conduit's `_meta` keys reached it; whether each client read its token from the file by its
   helper, and what it did after a 401. Record the protocol revision each client negotiated
   and how each client named the tools to its model.
4. **Runs that did not run as planned** (a client error, a timeout, a harness fault) are listed
   with their cause and are not classified as model behaviour.
5. **Before the matrix**, the admin ran three smoke runs to check the harness, and added
   Codex's approval of the stub's server after order 002; say so under the results' deviations.
6. **Findings of the first build**: where the results bear on a promise (IFC-012, IFC-013,
   IFC-025, IFC-038, IFC-042, PRV-022 or the third thesis), say how, as a candidate for the
   admin, without changing `spec/`.

## What to write

- `experiments/008-real-client/results-2026-11-09.md`: the clients and their versions, a
  summary table per case and client, the answers of decision 3, the classification of every
  run (one row per run), the runs that did not run as planned, the deviations, the limits as
  the pre-registration states them with any found in the run, and the candidate findings.
- `experiments/008-real-client/evidence-2026-11-09/`: for each run, a trimmed copy of what the
  results cite (the stub's log, the client's events and the final text), with every absolute
  path replaced by `<runs>` or `<lab>`, and no token, key or credential of any kind. Keep the
  copies small: what is cited, not every byte.
- Update the pre-registration's status line and its "Results" section to link the results, the
  [register](../../experiments/README.md)'s row for 008, and the roadmap's A71 row, whose state
  becomes "done on 2026-11-09" with a link.

## Scope

Only the files named above. Do not change `spec/` or the harness's code.

## Done when

Every run is classified or listed as not run as planned; every claim in the results cites runs;
no absolute path, user name or credential is in any file you write; and `python3
tools/check_docs.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 11 minutes. Every run classified or listed as not run as planned; dated results
  and trimmed evidence; four candidate findings for the admin.
- **Verifier**: 3 minutes. No fault found.
- **Admin**: checked the classification of C5 against the raw records of two runs, and
  recorded that review in the results, which the worker had left as pending. No second round.
