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
reading. The coordinator settled all thirteen findings of increment 1 under ruling 46, point 4,
in the increment's pull request: none adds or removes a feature or changes what Conduit is for.
Each is logged as the coordinator's own decision, and the owner can strike it.

## The logs

| Increment | Log | Findings |
| --- | --- | --- |
| 1, the strict loader (A43) | [log-increment-1.md](log-increment-1.md) | Thirteen, all settled by the coordinator: three defects of a tool, two fixed with vectors and one stated in the tool; and ten against the promises, nine fixed in them, each with a vector, and one kept as written |

## Noted for later increments

What one increment found that a later one must meet. None is a finding against the promises
yet.

- **The token that Codex sends, for increment 3.** Codex 0.167.0, one of S1's Agents, takes the
  bearer token of an MCP server that it reaches over HTTP from a variable of its environment:
  `codex mcp add NAME --url URL --bearer-token-env-var VAR`, or the same setting in its
  configuration. The engine gives a run its token as a file: under PRV-022 and DL13, `identity`
  holds the engine's address in the variable `CONDUIT_URL`, and the token in the file `token` of
  the directory that the variable `CONDUIT_IDENTITY_DIR` names, as the names under `identity` in
  [spec/names.yaml](../../../spec/names.yaml) fix them. So increment 3 needs a step that starts
  Codex with the content of that file in a variable. The author writes it so that one tool fits
  another, so it counts as an adapter in the author measurements of
  [the first build](../../implementation/FIRST_BUILD.md#measures).
