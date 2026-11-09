# Work order 011: the results of experiment 009 (A4), native launch and identity

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `005-real-client-results.schema.json`, reading its
`classification_counts` as one row per client and scenario with each turn's outcome counted in
`told_user` (completed as asked) or `other` (anything else, quoted). You have no network access.

## What and why

The admin session has run experiment 009's nine scenarios with the harness of
[order 010](010-native-launch-harness.md). Its
[pre-registration](../../experiments/009-native-launch/README.md) fixes the seven things the
probe must show. This order reads the records and writes the dated results. The admin session
gives the directories of the records below this order, outside the repository; never copy an
absolute path into a file.

## The decisions to apply

1. **One answer per point of "What it must show"**, per client, each citing runs, from the
   records and never from the flags alone: the wrapper (its files and lines), the launch with
   only the variables of PRV-022 and PRV-023, the owner's configuration kept out, the login's
   reach off, two Agents apart (the stub's attribution of every request in the pair runs), the
   change of access between turns, and the login each client reports. Where a client reports
   nothing that shows a point, as Codex does not report its tools in its events, say so, and say
   what would show it.
2. **The two first attempts are results too.** The admin ran the matrix three times: the first
   two failed for Codex, and the admin changed the harness between them. Report each attempt,
   what failed, what the records show of the cause, and what changed. Codex runs its header
   helper with a filtered environment, rejects `env_vars` for an HTTP server, and so its wrapper
   writes the identity directory into the helper's command line; this is a finding for increment
   3's Codex Agent type, and goes under the candidate findings with its runs.
3. **Write** `experiments/009-native-launch/results-2026-11-09.md` and trimmed evidence under
   `experiments/009-native-launch/evidence-2026-11-09/`, every absolute path, user name and
   home directory replaced, and no token, key or credential of any kind (the identity
   directories hold tokens: never copy them). Link the results from the pre-registration's
   status and "Results", the [register](../../experiments/README.md)'s row for 009 and the
   roadmap's A4 row, whose state becomes "done on 2026-11-09" with a link.
4. **Limits**: the pre-registration's, and any found in the run, such as what the records cannot
   show.

## Scope

Only the files named above.

## Done when

Every point of the pre-registration is answered or said to be unshown, with runs cited; every
attempt is reported; no absolute path, user name or credential is in any file you write; and
`python3 tools/check_docs.py`, `python3 tools/check_roadmap.py` and `git diff --check` pass.
