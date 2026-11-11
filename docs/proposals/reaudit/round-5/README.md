# Re-audit, round 5

Status: done, 2026-11-07 (plan-audit action A59): read, decided, fixed and verified. The last point
that the verification left open, V6, was settled by the coordinator. Ruling 41 ordered this round, on the
specification as rulings 40 and 41 rewrote it, before pull request 7 is merged. Six cold
readers read the repository at commit `d0c425f` on `docs/pre-codex`, as rulings 40 to 43 left
it. Readers 1 to 4 read `spec/` alone. Reader 5 read the whole repository. Reader 6 checked what
`spec/` says of the Model Context Protocol against the published protocol, at revision
2026-07-28, fetched the same day. The next read is the owner's cold read by Codex
([for the next reader](../../../AUDIT.md#for-the-next-reader)). The round's fixes and this
record came after `d0c425f` and are not in it: they are in `main` from the merge of pull
request 7 on, which is what the next reader reads. (Note of 2026-11-07: pull request 7 was
merged as commit `c4d7d33`, and the next read was [round 6](../round-6/README.md), which read
that commit.)

| Step | Record |
| --- | --- |
| The decisions on the findings, in seven areas, F1 to F7 | [DECISIONS.md](DECISIONS.md) |
| The fix pass, one pass for each area, each in its own files | [F1](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F1.md), [F2](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F2.md), [F3](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F3.md), [F4](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F4.md), [F5](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F5.md), [F6](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F6.md), [F7](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/log-fix-F7.md) |
| The integration, which made the files agree, with its one choice, on PLC-023 and LIF-036 | [log-fixes.md](log-fixes.md) |
| The verification of the fixes by two verifiers, and the point it left for the owner, V6, which the coordinator then settled | [log-fixes.md, "Verification"](log-fixes.md#verification) |
| The check of the handoff to the next reader, and what was done about it | [handoff-check.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/handoff-check.md), [below](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/README.md#handoff-check) |

| Reader | Sees | Looks for | Report |
| --- | --- | --- | --- |
| 1 | `spec/` only | Promises that contradict each other, the definitions, the schemas or the core package | [reader-1.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-1.md) |
| 2 | `spec/` only | Terms and document shapes: every term defined once, and the schemas, examples and core package agreeing with the promises | [reader-2.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-2.md) |
| 3 | `spec/` only | Behavior on one engine that two conforming engines could disagree on, by walking ten scenarios end to end | [reader-3.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-3.md) |
| 4 | `spec/` only | Behavior across machines that two conforming engines could disagree on, by walking twelve situations end to end | [reader-4.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-4.md) |
| 5 | The whole repository | What an outside reviewer with no context would be misled by or unable to find | [reader-5.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-5.md) |
| 6 | What `spec/` says of the Model Context Protocol, and the published protocol | What the interface says of the protocol, checked against the published revision | [reader-6.md](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-5/reader-6.md) |

## Known after the round

Open when the round ended, for whoever reads next:

- V6, a minor point, was settled by the coordinator after the round, as its option 1: an Agent
  or a Resource that runs nothing, which PLC-002 places on `local`, is not checked against its
  type's and its items' `nodeSelector` under PLC-007 ([the fix log](log-fixes.md#for-the-owner)).
- The owner may want to read some decisions first: the eight that
  [DECISIONS.md](DECISIONS.md#for-the-owner) lists, the integration's
  [choice](log-fixes.md#the-choice-of-this-pass) on PLC-023 and LIF-036, and five decisions of
  the verification that each settle a behavior: V2-01, V2-02, V2-04, V2-15 and V7. (Note of
  2026-11-07: ruling 44 reversed V2-15, on round 6's finding
  [6-1-01](../round-6/DECISIONS.md#the-six-findings). The protocol's -32021 for a client
  capability that the call did not declare is now read as any other error, except where
  Conduit's own contract binds the server to have done nothing.)
- The text that this round changed, 101 promises changed and 3 added, has been read by its
  editors and by the two verifiers, and not by a cold reader.
- Reader 6 checked the interface as it stood at `d0c425f`. The round changed 22 of its 41
  promises and added 2. The records do not say that these were checked against the published
  protocol again.
- `tools/check_schemas.py` does not reject an item selected for `get-secret.v1` at an `http`
  URL whose host is not a loopback address. Its rule `one-run` does not reject an item that is
  run for an Interface or a Pool. The rows of the schemas README say what the tool checks, and
  no decision asked for either rule.
- DECISIONS.md gives 203 promises of the layer `language` after the round. The recount gives
  202, since M05 moved SEC-024 to `convention`. The record keeps its figure.

Note of 2026-11-07: [round 6](../round-6/README.md), the owner's cold read by Codex, read the
whole repository at `c4d7d33`, with this round's changes in it, though selectively, as its
report says. Its finding 6-1-01 checked one of the interface's readings against the published
protocol, the -32021 of V2-15, and ruling 44 reversed that reading. The two limits of
`tools/check_schemas.py` above still stand.
