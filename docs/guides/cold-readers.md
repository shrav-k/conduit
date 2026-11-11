# Instructions for cold readers

For the owner's collaborators reading Conduit as the first build leaves it. Not binding: the
[specification](../../spec/README.md) alone binds. These instructions were written on
2026-11-10 for the **Read** row of the [roadmap](../../ROADMAP.md#the-order-of-the-work),
after step 3. As of that date, increments 1 to 3 are done and increment 4 is in progress:
both author runs completed T1 to T7 with real clients but failed the owner's bar for
authoring work. Simplification is being settled before it is built and S1 rerun. A read before step 3
is finished can help, but the build may still change the text. Record which text you read.

## What to read

Begin with [for the next reader](../AUDIT.md#for-the-next-reader). It sets the scope, the
reading order, the safety test, the known open points and where findings go. Follow its order:

1. The top [README](../../README.md), for what Conduit is and what exists.
2. [spec/README.md](../../spec/README.md), then
   [spec/definitions.md](../../spec/definitions.md), for the layers and the terms in force.
3. The promises, in the specification's [contents order](../../spec/README.md#contents),
   checking the definitions, core contracts, schemas and examples they depend on.

Read the head of `main` as the build leaves it, and record its commit ID and your date. Do not
use `c4d7d33`, which round 6 read. If the owner assigns a narrower reading scope, state it in
the report. For a whole-repository read, use the [documentation map](../README.md) to follow
the entry points and supporting documents too. The [overview](../OVERVIEW.md) can help with
the idea, but it cannot supply a rule missing from the promises. Consult earlier decisions
when checking whether a finding is already known; they do not override `spec/`.

Walk concrete situations where promises meet: access changing while work is in flight, a
repeat after a lost result, retirement, and a secret crossing a provider boundary. Apply the
next-reader section's safety test, including a procedure that conflicts with a safety promise.
A passing vector tests its stated slice; it does not prove that the whole promise holds.

## What is known to be open

Use the next-reader section's **Known to be open**, **Left for a reader by the records** and
**To look at after round 6** as the index, rather than reporting an acknowledged limit again
without a new situation. Its status statements have not all caught up with the build. As of
2026-11-10:

- A reference model, vectors and an engine shell exist. Use the
  [roadmap's step 3](../../ROADMAP.md#the-order-of-the-work), the
  [coverage manifest](../../conformance/manifest.yaml) and the build's
  [conformance document](../../reference-model/CONFORMANCE.md) for current coverage and
  implementation limits. A missing implementation is distinct from an undecided language rule.
- The [build logs](../proposals/first-build/README.md) record decisions since round 6. For
  example, increment 2 settled the reserved root `$defs` copy-key question and the two plain
  protocol-error cases. Check the settlement before raising the old question again.
- The [real-client closeout](../proposals/first-build/increment-3/closeout-results-2026-11-10.md)
  records T1, T2 and T7 and carries unshown checks forward. Access is mediated; Codex's home
  instructions still leak, and separate homes and the sign-in flow of
  [ruling 49](../proposals/ruling-49/README.md) are pending work. A ruling is not evidence that
  its specification or implementation changes have been made.
- Rulings 49 to 54 are accepted, with changes not yet written into the specification; its
  [status](../../spec/README.md) links each. The [language-shape investigation](../proposals/language-shape/README.md)
  and the owner's [Project direction](../proposals/owner-direction/2026-11-10-projects.md)
  precede that writing. Do not read a proposed short form or Project model as an existing rule.
- The [increment 4 log](../proposals/first-build/log-increment-4.md) distinguishes the fixes
  from orders 025 to 028 from open findings. In particular, 4-06 is settled as parallel model
  work within the parent Agent's access, 4-03 retains SUB-010's submission rule, and 4-08's
  explicit refusals are ruled but await promises. Turn deadlines (4-12) and the form of ended
  conditions (4-17) remain open; both measured author runs failed the bar for authoring work.

The deliberate limits of v0, including the two delegation limits, remain in the
[specification's limits](../../spec/README.md#limits-of-this-version). No round so far was
clean; the next-reader section explains how the readiness gate is judged. This guide does
not reassess that gate.

## How to report a finding

Prepare one report per reader for `docs/proposals/reaudit/round-7/`, the next directory after
[round 6](../proposals/reaudit/round-6/README.md). Hand the local report to the coordinator;
do not publish it. Use [round 6's reader report](../proposals/reaudit/round-6/reader-1.md) as
the example of the form. At the top, give your date, commit ID, scope and what you did and did
not check. For each finding give:

- A unique ID, severity and a short title. Distinguish wrong, undetermined and misleading;
  mark unsafe cases and safety-critical contradictions explicitly, as the tracker asks.
- The file and line, promise IDs where applicable, and short exact quotes of the text at issue.
- A concrete situation showing the problem and the strongest counterargument you found.
- The least change that fixes it, and any cost or choice it leaves for the owner.

Report unclear entry text as well as defects in promises. Separate observed results from
reasoning and proposed tests. Keep recommendations pending: the owner rules, or the
coordinator decides under ruling 37, and the decision is logged beside the report. Round 6's
[decisions](../proposals/reaudit/round-6/DECISIONS.md) show that separate step.

## What not to do

Do not change `spec/` directly, read the sibling reference clones, or publish anything: no
commit, push, pull request or release. The task is to report what the text means to a new
reader, not to implement an engine or silently repair a rule. Follow [AGENTS.md](../../AGENTS.md)
for repository rules; do not read credentials or change the owner's configuration.

## How long it takes

The next-reader instructions and round 6's reports give no elapsed reading time or fixed
duration. There is no recorded estimate to promise here. State your coverage and any part
left unread when handing over the report.
