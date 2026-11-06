# Re-audit, round 4

Status: in progress, 2026-11-06 (plan-audit action A59). Four cold readers read the repository
as it stands after [round 3](../round-3/README.md) and its fix pass. The done-when of action A59
asks for two consecutive rounds with no finding in the language, the conventions or the
interface. None of the first three rounds was clean: the readers of `spec/` reported about 170
findings, then 125, then 44. Whether the readiness condition G7 is judged on these rounds is
[for the owner](../../../AUDIT.md#waiting-for-the-owner).

| Reader | Sees | Looks for | Report |
| --- | --- | --- | --- |
| 1 | `spec/` only | Promises that duplicate or contradict each other, the definitions, the schemas or the core package | `reader-1.md` |
| 2 | `spec/` only | Terms and document shapes: every term defined once, and the schemas, examples and core package agreeing with the promises | `reader-2.md` |
| 3 | `spec/` only | Anything two conforming engines could disagree on, by walking scenarios end to end | `reader-3.md` |
| 4 | The whole repository, from `README.md` | What an outside reviewer with no context would be misled by or unable to find | `reader-4.md` |

A fifth reader, with access to the published Model Context Protocol, checks what the interface
promises say about the protocol itself, as they stand after the changes of rounds 2 and 3:
`mcp-recheck.md`.

Findings are resolved under ruling 37, without a ruling on each, and every choice is logged in
this directory as the coordinator's and not the owner's
([who decides what](../../../AUDIT.md#who-decides-what-since-ruling-37)).

## Known before the round

These points are open and are not to be reported again unless a reader has something new:

- How a value is marked as secret, so that a promise can say secrets never appear in
  configuration or records. Left for the owner.
- The shape of a diagnostic beyond its `message`, and the full shape of an inspection result.
  Both are settled with the conformance vectors, which do not exist yet.
- No promise has a conformance vector: every `vectors` list is empty.
- What the interface says about the Model Context Protocol itself cannot be checked from
  `spec/` alone; the fifth reader checks it.
