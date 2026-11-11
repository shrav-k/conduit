# Work order 069b (K3b): the plan-and-apply reference under ruling 76

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share; leave `vectors` empty. This order has no network access, and needs
none. Never print a whole large file: use `head`, `tail`, `grep` or a count. Never run `env`,
`printenv`, `set` or `export -p`, and never print an environment variable's value.

## What and why

Order 069 wrote the reference for `conduit plan` and `conduit apply`, with nine golden cases for
S1's tasks, in `docs/implementation/plan-and-apply/`. It does not bind. It left five readings to the
owner, and [ruling 76](../proposals/ruling-76/README.md) settled them:

1. **Narrowing never needs a flag.** A Conduit the file no longer names is closed on apply, and so
   is a narrowing of a Conduit's operations; the plan prints and records each closing. Widening,
   replacing, reopening, recreating, adopting and cascades keep their named flags. `--prune` is no
   longer needed to close a Conduit the file dropped. This amends ruling 53 point 4 for removal
   only.
2. **Every flag names its object by its generated name,** such as `--replace agent-a--project`.
3. **Commands find the engine through a current engine context** that `conduit serve` records;
   `--store` remains as an override.
4. **When a set-up raises a sign-in, apply goes on.** It prints each pending sign-in as the person's
   action at the panel, and answers none.
5. **`conduit plan` is available and never required.**

Experiment 023 (order 073) counted S1 on paper under each reading; with points 1, 3 and 4 it forecasts
21, 5, 0, 3, 2, 2 and 2 lines for T1 to T7. Read ruling 76, order 069's file and its Record, the
reference and its cases, and experiment 023's results first.

## What to do

1. **The reference.** In `docs/implementation/plan-and-apply/README.md`, replace each "for the
   owner" reading with the ruled one, citing ruling 76, and remove what no longer holds:
   - `--prune` for a dropped Conduit;
   - `--store` on every command;
   - the stop-and-apply-again alternative for sign-ins;
   - any wording that requires `conduit plan`.

   Say how the current engine context is chosen: `conduit serve` records the engine it serves as
   the person's current context, and `--store` overrides it. Do not invent a command beyond that;
   if the reference needs one, name it as a gap for order 052a.
2. **The golden cases.** Update the nine cases in `cases/` to match:
   - commands without `--store`;
   - T7's revocation as a bare `conduit apply`, with the plan printing and recording the closing;
   - each case's printed and recorded form, with its counts.

   Keep their schema and the chain in which each case's `applied` is the previous case's `kept`.
   Re-derive any digest a case states only if it changes, and say so.
3. **Counts.** Report T1 to T7's command lines, primary and strict, under the ruled readings. They
   must agree with experiment 023's corresponding row. If they don't, report the difference and its
   cause; do not change experiment 023.

## Scope

Change:

- `docs/implementation/plan-and-apply/`.

Do not change:

- `spec/`, `engine/` or `conformance/`;
- experiment 023;
- any ruling record, IN_FORCE.md or the ledger.

Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/check_roadmap.py`
- `git diff --check`

Also, the case checks that order 069's "Done when" lists, run again on the updated cases.

Report each check's last line. Do not stage anything.

## Record

Run on 2026-11-11, the first work order run on the Windows computer over SSH (ruling 76, point 6):
the worker in 8 minutes, then a fresh verifier in 3, which found nothing to fix. The runner's
checkpoint commit failed there for want of a git identity; the verifier reviewed the staged changes
against the base all the same, and the identity is now set in the work-order clone. The reference
and its nine cases follow ruling 76: no flag to close a dropped Conduit, generated names, the current
engine context, apply going on past a sign-in, and `conduit plan` optional. S1's tasks count 21, 5,
0, 3, 2, 2 and 2 lines, primary, matching experiment 023; strict 25, 6, 0, 3, 2, 2 and 3. Finding
WO069B-O8 is for order 052a: how the current context is kept, found and reported when missing.
