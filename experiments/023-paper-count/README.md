Not binding. The specification in [spec/](../../spec/README.md) alone binds.

# 023 — PB: a paper count of S1 in flow style

Work order 073 authorizes this follow-up to [011](../011-short-form-spike/README.md),
serving A79 in the [register](../../ROADMAP.md#experiments-and-probes). Prerequisites
checked on 2026-11-11: K3's CLI reference is present, this directory held its placeholder,
the index holds 023, and no other experiment directory begins `023-`.

## What this must show

Before writing any state: count S1's T1–T7 in the documented flow layout and block layout,
using the present short form, package templates, owners, account declarations and root's
turn Conduits. Compare with baseline A70, 33/16/0/8/3/2/2. Apply ruling 71's per-task rule
only as a forecast on paper. This informs the flow style and CLI design, never the bar.
Ruling 71 fixed that rule on 2026-11-11 before this count; order 069's reference is present.
X2's dropped author-effort experiment supplies no authority: this is the design input that
work order 073 authorizes under the accepted plan, not an author run under ruling 45.

## Counting fixed before execution

The following method is fixed by work order 073 before execution. Write all seven states
in each style and all ledgers before counting. T3 edits nothing. After counting, change a
state only to correct an error found by the prescribed checks, recording every correction.

For each file, run `git diff --no-index --minimal -U0 OLD NEW`. Status 0 means equal;
1 means different; anything else is an error. Each hunk costs max(removed, added).
At T1 OLD is an empty file. Later transitions are t1→t2, t2→t4, t4→t5, t5→t6 and
t6→t7-revoked→t7. T7 counts the restored grant once, not both transitions.

Split each manifest into two scratch files before diffing: authored (`apiVersion`, with
`packages:` and all pin lines removed), and pins (`packages:` and all pin lines only).
Pin copies, including the first `packages:` line, are tool-written and listed apart.
Keep every split copy outside the repository, in `<scratch>`.

Primary = authored file hunk costs + commands carrying chosen paths, names, packages,
digests or named flags. Account lines and root's Conduits count; list root's lines apart.
Panel actions are separate. T3 primary is zero: list files/commands as inspection effort.
Strict = primary + every otherwise excluded command (T3 included) + every panel action.
Block uses primary rules with block files. Digests copied into install commands count once
with those commands and are listed apart. Checks, including turns/tools used to demonstrate
a task, are absent from the ledgers. No ledger command is executed.

Use K3's first/default alternatives A–E in ledgers, counting every alternative beside them:
A omit narrowing replace/prune approvals; B kind/name operands or operandless flags;
C selected context, explicit store (and optional URL), or fixed local engine;
D stop and resume apply after each sign-in; E explicit plan before each apply.
Also count K3 O6's dependency preinstalled before T1: move T2's first install to a
separately reported setup action, retaining it at T2 in primary.
K3 proposes no engine value for plan/apply, but leaves initial context selection open
(point C, for the owner). Until that is settled, conservatively count today's required
store value for every command, even though ledgers omit it as this order directs.
Also give C's no-value default for plan/apply, and the reading omitting store throughout. Compare root lines excluded,
init's generated skeleton excluded, packages key authored, install digest computed, flags
with no object operand, and T3 value-bearing commands counted. Primary takes the higher
reading unless this order fixes otherwise (T3 and the packages key). No choice settles a
rule; order 053 does that. Count the request-only route wherever expressible; edits/apply
remain primary. Compare one versus two Agent packages and the missing turn default as
assumptions, separately from rule readings.

Review correction on 2026-11-11: the request-route inventory now includes T1, T2,
T4, T5 and T6 beside T7. These additional ledgers were written before their counts;
no state or primary ledger changed. They use framed registration and, where necessary,
template replacement and instance retirement, rather than assuming a dedicated control
is necessary. The fixed counting rules above also apply to these routes.

## States and limits

Use fixture pins, paths and addresses. History's filesystem 0.1.0 dependency is installed
first by its own command; it is never a second project pin at that address. The project's
filesystem pin is 0.1.1 then 0.2.1. The fixture Agent Types lack a turn implementation and
sign-in; states assume order 050's .v2 Types supply both, with the implementation default
of shipped .v1 Types. Omit no state for that known limit. Omit unnecessary manages/console.
Use each documented configuration default S1 needs and list it in the results.

This is paper by an author who knows the design, a floor rather than an author run.
No engine, client, provider, build or network runs. Schema checking checks syntax only;
translation awaits 043a/043b. Decisions affecting S1's count stay with the owner until the
rerun (ruling 71.4). Ruling 74's newer ownTools requirement is reported separately from
this order's fixed fixture-based states.

## Reproduction

Run from the repository root. `<scratch>` always means a scratch directory outside this
repository, never a literal machine path in these records.

1. Check the five retained archives with `shasum -a 256 ARCHIVE`; compare with
   `conformance/fixtures/install-archives/digests.json` and `conformance/fixtures/s1/README.md`.
2. Copy `spec/` and `tools/` into `<scratch>`. Copy each `states/STYLE/STAGE/` into
   `<scratch>/spec/schemas/examples/valid/pb-STYLE-STAGE/`. Run
   `python3 tools/check_schemas.py` there; require `Errors: 0` and 14 more sectioned projects.
3. Compare manifest and s1.yaml in each style with the PyYAML one-liner in the results,
   after the strict schema check passes. Check t7 equals t6 with `cmp` for each file/style.
4. Produce the split manifest copies described above in `<scratch>`, preserving lines and
   ordering; copy s1.yaml unchanged. For every transition/file/style run
   `git diff --no-index --minimal -U0 OLD NEW`. Sum max(deletions, additions) per hunk,
   excluding diff headers; retain the separate pin totals. T7 uses the revocation cost once
   after checking its restoration is identical. Read ledgers with `grep -v '^#' FILE`;
   count non-comment command/action lines with `grep -vc '^#' FILE` (status 1 means none).
5. Add command and panel costs using the rules above. Root's Conduits are the `operator`
   lines in flow and the entire operator subtrees in block. Report every alternate count
   and its forecast, without changing states to improve a result.
6. Compare each flow file with K3's corresponding case's `files` values; report textual
   and parsed differences. Run the repository checks and cleanliness searches in the order.

Clarification after the file-only diffs, before adding command totals: the initial wording
read K3 C as settling selection for plan/apply. C explicitly leaves selection open. The
paragraph above now follows the order's higher-count rule for unresolved store selection.
No state or ledger changed, and none of the fixed counting rules changed.

The dated [results](results-2026-11-11.md) record the counts, checks, choices and limits.
