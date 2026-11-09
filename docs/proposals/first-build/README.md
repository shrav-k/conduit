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
reading. Increment 1 and its review have eighteen findings, all settled. Ruling 48, point 3,
settled the restriction on pattern tokens; the other settlements are the coordinator's under
ruling 46, point 4, and the owner can strike them. None adds or removes a feature or changes
what Conduit is for.

## Increment 2's plan

[The study of increment 2](increment-2/plan.md), by work order 003, proposes S1 as documents,
the promises and safety cases it touches, a format for the vectors and a sequence of orders.
[The admin's decisions](increment-2/decisions.md) settle what it left open.

## The logs

| Increment | Log | Findings |
| --- | --- | --- |
| 1, the strict loader (A43) | [log-increment-1.md](log-increment-1.md) | Eighteen, all settled: three defects of a tool, two fixed with vectors and one stated in the tool; fifteen against the promises, including five after the review of 2026-11-09. Ruling 48, point 3, settles the pattern tokens; the other settlements are the coordinator's and can be struck |
| 2, the core on S1 (A38, A39) | [log-increment-2.md](log-increment-2.md) | I2-01: required first-start cases decided; double-BOM YAML boundary 2-01 settled by the admin with a vector; the owner can strike it. Verification corrections cover registration authority, diagnostic precedence and lifecycle record targets |

## Noted for later increments

What one increment found that a later one must meet. None is a finding against the promises
yet.

- **The token that Codex sends, for increment 3.** Codex 0.167.0 has
  `mcp_servers.<id>.http_headers_helper`, a local command that prints a JSON object of headers,
  as Claude Code has `headersHelper`. Both can read `token` from `CONDUIT_IDENTITY_DIR` with a
  one-line helper. PRV-022 and DL13 give the run that directory and the engine's address in
  `CONDUIT_URL`, under the names in [spec/names.yaml](../../../spec/names.yaml). No adapter is
  counted in advance. A4 or A71 must confirm whether Codex runs the helper again after a 401,
  and whether its "locally connected" servers include an engine at a loopback address.
- **A missing tool of a plain server, for increment 3.** When `tools/list` lacks a tool that
  the item's `tools` map names, whether the request reports `failed` or `unavailable` is to be
  settled there, with a vector.
