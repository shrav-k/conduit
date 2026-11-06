# Re-audit, round 3

Status: in progress, 2026-11-06 (plan-audit action A59). Four cold readers read the repository
as it stands after [round 2](../round-2/README.md) and its passes. The readiness condition asks
for two consecutive rounds with no finding in the language, the conventions or the interface.
Neither of the first two rounds was clean.

| Reader | Sees | Looks for | Report |
| --- | --- | --- | --- |
| 1 | `spec/` only | Promises that duplicate or contradict each other, the definitions, the schemas or the core package | `reader-1.md` |
| 2 | `spec/` only | Terms and document shapes: every term defined once, and the schemas, examples and core package agreeing with the promises | `reader-2.md` |
| 3 | `spec/` only | Anything two conforming engines could disagree on, by walking scenarios end to end | `reader-3.md` |
| 4 | The whole repository, from `README.md` | What an outside reviewer with no context would be misled by or unable to find | `reader-4.md` |

Findings are resolved under ruling 37: without a ruling on each, with every choice logged in
this directory, and only what changes what Conduit is for goes back to the owner.

## Known before the round

These points are open and are not to be reported again unless a reader has something new:

- How a value is marked as secret, so that a promise can say secrets never appear in
  configuration or records. Left for the owner.
- The shape of a diagnostic beyond its `message`, and the full shape of an inspection result.
  Both are settled with the conformance vectors, which do not exist yet.
- No promise has a conformance vector: every `vectors` list is empty.
- What the interface says about the Model Context Protocol itself was checked against the
  published protocol by a separate reader
  ([report](../round-2/mcp-citations.md)); a reader of `spec/` alone cannot check it.
