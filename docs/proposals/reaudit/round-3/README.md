# Re-audit, round 3

Status: done, 2026-11-06 (plan-audit action A59), and [round 4](../round-4/README.md) is next. Four cold
readers read the repository as it stood after [round 2](../round-2/README.md) and its passes, at
commit `b1d3fb4`.

| | Round 2 | Round 3 |
| --- | --- | --- |
| Findings in `spec/`, three readers | 125 | 44 |
| Of them classed as unsafe | 8 | 4 |
| Findings of the reader of the whole repository | 57 | 23 |

The round was not clean. The done-when of action A59 asks for two consecutive rounds with no
finding in the language, the conventions or the interface, and none of the three rounds has
been clean. Whether the readiness condition G7 is judged on these rounds is
[for the owner](../../../AUDIT.md#waiting-for-the-owner).

| Step | Record |
| --- | --- |
| The decisions on the findings | [DECISIONS.md](DECISIONS.md) |
| The fix pass and its follow-up, with the choices the editor made | [log-fixes.md](log-fixes.md) |

| Reader | Sees | Looks for | Report |
| --- | --- | --- | --- |
| 1 | `spec/` only | Promises that duplicate or contradict each other, the definitions, the schemas or the core package | [reader-1.md](reader-1.md) |
| 2 | `spec/` only | Terms and document shapes: every term defined once, and the schemas, examples and core package agreeing with the promises | [reader-2.md](reader-2.md) |
| 3 | `spec/` only | Anything two conforming engines could disagree on, by walking scenarios end to end | [reader-3.md](reader-3.md) |
| 4 | The whole repository, from `README.md` | What an outside reviewer with no context would be misled by or unable to find | [reader-4.md](reader-4.md) |

Findings are resolved under ruling 37, without a ruling on each, and every choice is logged in
this directory as the coordinator's. The coordinator's own reading of the ruling, which the
owner has not confirmed, is that a finding goes back to the owner only where settling it would
change what Conduit is for
([who decides what](../../../AUDIT.md#who-decides-what-since-ruling-37)).

## Known before the round

These points are open and are not to be reported again unless a reader has something new:

- How a value is marked as secret, so that a promise can say secrets never appear in
  configuration or records. Left for the owner.
- The shape of a diagnostic beyond its `message`, and the full shape of an inspection result.
  Both are settled with the conformance vectors, which do not exist yet.
- No promise has a conformance vector: every `vectors` list is empty.
- What the interface says about the Model Context Protocol itself cannot be checked from
  `spec/` alone. A separate reader checked IFC-001 to IFC-033 against the published protocol
  ([report](../round-2/mcp-citations.md)). The promises were then changed to follow the
  report, and the changed text has not been checked again.
