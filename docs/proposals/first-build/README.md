# The first build: what it finds

Not binding. The specification in [spec/](../../../spec/README.md) alone binds. Where a log
here and the specification differ, the specification governs.

This directory holds what the first build finds in the language: one log per increment, as
[ruling 46](../ruling-46/README.md), point 4, has it. The increments and what each must show are
pre-registered in [the first build](../../implementation/FIRST_BUILD.md#increments), and the
code is in [reference-model/](../../../reference-model/README.md).

## What a finding is

Each row of a log is one finding, against the IDs of the promises it concerns, of one of four
classes:

- a **gap**: a case that no promise decides;
- an **ambiguity**: a promise that two readings would build differently;
- a **contradiction**: two promises that cannot both hold;
- **needless complexity**: a part of the language that the task did not need, or that cost more
  than it gave, among it what the profile asks that the tools of the ecosystem make needlessly
  hard.

A defect in a tool that checks the specification, such as `tools/check_schemas.py`, is logged
too, and marked as such: it is not a finding against the promises, but a reader who trusts the
tool is misled by it. Each row says what the build did, and the fix.

## Who settles a finding

Under ruling 46, point 4, the coordinator fixes a finding in the same pull request as the
increment that found it, in the promises and with a vector, and logs the fix as its own, as it
settles a cold reader's finding under rulings 37 and 38. The owner can strike any such fix. A
finding whose fix adds or removes a feature, or changes what Conduit is for, goes to the owner,
and the log records what the build assumed in the meantime.

A log marks each finding as fixed, with its vector, or as left for the owner, with the build's
reading. Ruling 46 lets the coordinator settle more than the log of increment 1 settles: only a
fix that adds or removes a feature, or changes what Conduit is for, must go to the owner. For
increment 1 the coordinator chose, in its brief, to fix in the increment's pull request only
what was clear, changed no behavior that a ruling settled and could be shown by a vector that
the checks run, and to put every other finding to the owner with a recommendation, though most
of them add or remove no feature. Each of those fixes chooses between readings that the text
leaves open, or narrows what an engine accepts, and this is the first time the promises are read
in order to build them. The owner may hand any of them back to the coordinator to settle.

## The logs

| Increment | Log | Findings |
| --- | --- | --- |
| 1, the strict loader (A43) | [log-increment-1.md](log-increment-1.md) | Thirteen: three defects of a tool, two fixed with vectors and one stated in the tool, and ten for the owner |
