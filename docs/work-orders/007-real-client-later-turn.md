# Work order 007: experiment 008's follow-up, a later turn

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `002-real-client-harness.schema.json`, the schema
of the harness's first order. This order has network access, to fetch crates; use it for
nothing else. You run no agent.

## What and why

Experiment 008's [pre-registration](../../experiments/008-real-client/README.md) now has a
follow-up, "a later turn", pre-registered before it runs: does a model see a Conduit that opened,
or stop offering one that closed, in a second user turn of the same session? Read it, and the
[results of 2026-11-09](../../experiments/008-real-client/results-2026-11-09.md) that led to it.
This order extends the harness of [order 002](002-real-client-harness.md) to run it.

## The decisions to apply

1. **Two new cases**, C1-later and C5-later, as the follow-up states, with their prompts fixed in
   code: one prompt for the first turn and one for the second. The stub's states are those of C1
   and C5.
2. **A second turn in the same session**: for Claude Code, on its streamed input
   (`--input-format stream-json`) in the same process, the second message sent only after the
   first turn's result; for Codex, by resuming the same session (`codex exec resume`), which
   needs the first run without `--ephemeral`, and with the same isolation flags otherwise.
   Every other flag and the removal of the API key variables stay as order 002 left them,
   including Codex's approval of the stub's server.
3. **The follow-up's matrix**, 12 runs, selected by a new option such as `--follow-up`, so the
   original 68 stay as they are; `--dry-run` with it prints exactly the 12.
4. **What is recorded** per turn: the stub's log, the client's events, and the model's text,
   marked by turn.
5. **Tests that need no agent**: the runner's planning of both turns for each client, and the
   dry run's 12.

## Scope

Only files under `experiments/008-real-client/` other than `README.md` and the results.

## Done when

As order 002's done-when, with the follow-up's dry run printing its 12 runs.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 8 minutes. Two cases with two turns each, the follow-up's 12 runs behind
  `--follow-up`, the original 68 unchanged; 20 tests that need no agent.
- **Verifier**: 2 minutes. No fault found.
- **Admin**: ran the tests, read the resume command, and ran the 12 runs. No second round.
