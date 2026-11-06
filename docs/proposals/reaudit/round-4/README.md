# Re-audit, round 4

Status: done, 2026-11-06 (plan-audit action A59). No fifth round is started: whether to go on
reading or to move to the reference model and the vectors is
[for the owner](../../../AUDIT.md#waiting-for-the-owner). Four cold
readers read the repository as it stood after [round 3](../round-3/README.md), its fix pass and
the follow-up, at commit `f583469`.

| | Round 2 | Round 3 | Round 4 |
| --- | --- | --- | --- |
| Reader 1, contradictions | 36 | 16 | 12 |
| Reader 2, terms and shapes | 49 | 13 | 11 |
| Reader 3, undetermined behavior | 40 | 15 | 15 |
| Findings in `spec/`, three readers | 125 | 44 | 38 |
| Of them classed as unsafe | 8 | 4 | 3 |
| Findings of the reader of the whole repository | 47 | 23 | 26 |

Of the three unsafe findings of round 4, the readers marked two, and the coordinator classed a
third, 4A-5, which its reader marked as unsafe only if applied as written.

The round was not clean, and the fall has slowed. The done-when of action A59 asks for two
consecutive rounds with no finding in the language, the conventions or the interface, and none
of the four rounds has been clean. What reader 3 finds has stayed level: its findings are where
two features that were specified apart meet, and each round's fixes add text for the next round
to meet. Rounds 2 to 4 are counted alike, as the sum of the findings in the three reports on
`spec/`. Round 1 is counted otherwise, as about 170 problems by class after duplicates among
the reports were merged; its three reports hold about 200 items. Whether the readiness
condition G7 is judged on these rounds is
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

A fifth reader, with access to the published Model Context Protocol, checks what the interface
promises say about the protocol itself, as they stand after the changes of rounds 2 and 3:
[mcp-recheck.md](mcp-recheck.md).

Findings are resolved under ruling 37, without a ruling on each, and every choice is logged in
this directory as the coordinator's and not the owner's
([who decides what](../../../AUDIT.md#who-decides-what-since-ruling-37)).

## Known after the round

Open when the round ended, for whoever reads next:

- The two points at the end of [DECISIONS.md](DECISIONS.md), on opening a composite Conduit.
- The text that the fix pass of this round changed, 92 promises and 5 new ones, has been read by
  its editor alone.
- The interface promises that the fix pass changed have not been checked against the Model
  Context Protocol again. The [second check](mcp-recheck.md) read them as round 3 left them.

## Known before the round

These points are open and are not to be reported again unless a reader has something new:

- How a value is marked as secret, so that a promise can say secrets never appear in
  configuration or records. Left for the owner.
- The shape of a diagnostic beyond its `message`, and the full shape of an inspection result.
  Both are settled with the conformance vectors, which do not exist yet.
- No promise has a conformance vector: every `vectors` list is empty.
- What the interface says about the Model Context Protocol itself cannot be checked from
  `spec/` alone; the fifth reader checks it.
