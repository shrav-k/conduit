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

| | Round 2 | Round 3 | Round 4 | Round 5 |
| --- | --- | --- | --- | --- |
| Reader 1, contradictions | 36 | 16 | 12 | 24 |
| Reader 2, terms and shapes | 49 | 13 | 11 | 21 |
| Reader 3, undetermined behavior (in round 5, on one engine) | 40 | 15 | 15 | 16 |
| Reader 4 of round 5, behavior across machines | no reader | no reader | no reader | 23 |
| Reader 6 of round 5, the protocol | no reader | no reader | no reader | 21 |
| Findings in `spec/`, readers 1 to 3 | 125 | 44 | 38 | 61 |
| Findings in `spec/`, every reader of `spec/` | 125 | 44 | 38 | 105 |
| Of them classed as unsafe | 8 | 4 | 3 | 10 |
| Findings of the reader of the whole repository | 47 | 23 | 26 | 14 |

Rounds 2 to 4 had three readers of `spec/`, and round 5 had five, so its total is not
comparable with theirs. In rounds 2 and 4 the check against the protocol was a separate report
of claims, and it is not counted here. In rounds 2 to 4 the reader of the whole repository was
reader 4, and in round 5 it was reader 5. The ten unsafe findings of round 5 are the readers'
own: one of reader 1, one of reader 3, seven of reader 4 and one of reader 6. The coordinator
kept each as unsafe.

The round was not clean, and no round has been. Ruling 38 restated the readiness condition G7:
it is judged on no unsafe finding in the latest round, and on the reference model and the
vectors. Round 5 reported ten unsafe findings. All ten are fixed, and no unsafe finding is open.

| Step | Record |
| --- | --- |
| The decisions on the findings, in seven areas, F1 to F7 | [DECISIONS.md](DECISIONS.md) |
| The fix pass, one pass for each area, each in its own files | [F1](log-fix-F1.md), [F2](log-fix-F2.md), [F3](log-fix-F3.md), [F4](log-fix-F4.md), [F5](log-fix-F5.md), [F6](log-fix-F6.md), [F7](log-fix-F7.md) |
| The integration, which made the files agree, with its one choice, on PLC-023 and LIF-036 | [log-fixes.md](log-fixes.md) |
| The verification of the fixes by two verifiers, and the point it left for the owner, V6, which the coordinator then settled | [log-fixes.md, "Verification"](log-fixes.md#verification) |
| The check of the handoff to the next reader, and what was done about it | [handoff-check.md](handoff-check.md), [below](#handoff-check) |

| Reader | Sees | Looks for | Report |
| --- | --- | --- | --- |
| 1 | `spec/` only | Promises that contradict each other, the definitions, the schemas or the core package | [reader-1.md](reader-1.md) |
| 2 | `spec/` only | Terms and document shapes: every term defined once, and the schemas, examples and core package agreeing with the promises | [reader-2.md](reader-2.md) |
| 3 | `spec/` only | Behavior on one engine that two conforming engines could disagree on, by walking ten scenarios end to end | [reader-3.md](reader-3.md) |
| 4 | `spec/` only | Behavior across machines that two conforming engines could disagree on, by walking twelve situations end to end | [reader-4.md](reader-4.md) |
| 5 | The whole repository | What an outside reviewer with no context would be misled by or unable to find | [reader-5.md](reader-5.md) |
| 6 | What `spec/` says of the Model Context Protocol, and the published protocol | What the interface says of the protocol, checked against the published revision | [reader-6.md](reader-6.md) |

## What the round decided and changed

- The coordinator merged the 119 findings into 111 items, since eight items each join two
  findings. 110 items are fixed and 1, M78, is rejected. None went to the owner. By severity
  the items are 10 unsafe, 8 contradiction, 32 undetermined, 19 wrong, 16 misleading and 26
  minor.
- Seven passes applied the decisions, one for each area. An integration then made the files
  agree. It made 41 changes that the decisions had routed to an area that does not hold their
  file, fixed text that the decisions left behind, and made one choice of its own, on PLC-023
  and LIF-036.
- Two verifiers checked the fixes and the integration, and reported 29 findings, none of them
  unsafe. 28 are fixed. The coordinator then settled the last, V6.

Counted with a script, from `d0c425f` to the end of the round:

| | Before | After |
| --- | --- | --- |
| Promises | 449 | 452 |
| Of the layer `language` | 202 | 202 |
| Of the layer `convention` | 206 | 207 |
| Of the layer `interface` | 41 | 43 |
| Items of the implementation-defined list | 16 | 15 |
| Files under `examples/invalid-by-rule` | 28 | 30 |
| Rules that `tools/check_schemas.py` applies in its step 6 | 24, from 23 promises | 25, from 24 promises |

Three promises were added: PKG-030 (M25), and IFC-045 and IFC-046 (V2, V9). None was removed,
and none moved between files. SEC-024 moved from the layer `language` to `convention` (M05).
101 promises changed and keep their IDs: 91 in the fix pass and 30 in the verification, 20 of
them in both. DL11 was removed from the implementation-defined list (M69), and its ID is not
reused. DL4, DL10, DL12, DL13, DL16 and DL17 changed.

Findings are resolved under ruling 37, as the owner confirmed it in
[ruling 38](../../ruling-38/README.md), without a ruling on each. Every decision and choice is
logged in this directory as the coordinator's or an editor's, and not the owner's. No ruling
has accepted them, and the owner can strike any of them
([who decides what](../../../AUDIT.md#who-decides-what)).

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

## Handoff check

Date: 2026-11-07. A reader checked the entry points cold, as the next reader would arrive,
and reported 13 findings, H-01 to H-13, none unsafe: 4 wrong, 4 misleading and 5 minor
([report](handoff-check.md)). The coordinator checked each against the files and settled it
under rulings 37 and 38; the owner can strike any of these decisions. Every finding holds. 12
are fixed, and 1, H-05, is fixed in part: the commit it asks for is not made here. No promise
changed, and no file under `spec/` but `spec/README.md`.

| Finding | Severity | Decision | What changed |
| --- | --- | --- | --- |
| H-01 | wrong | fix | The contents row for `providers.yaml` in `spec/README.md` and A64's status in the tracker say "other than a `plain` server reached by `remote`", as PRV-025's `when` and the prose of `spec/README.md` have it. A64 says that a `plain` server is outside the gate, takes none of Conduit's keys, and is reached by whatever can reach its URL, so a request to it that crosses machines carries no proof of its sender, by design (IFC-007, IFC-041). |
| H-02 | wrong | fix | In the defect register L155 is ruled by ACC-003 and ACC-036, as M69 has it, and L156 by CON-004 and ACC-006, which have decided it since the last pass of round 2. The register's triage and the tracker's row on language defects say 16 ruled and 29 answered by an item of the list; the register's status values name round 5's removal of DL11 beside ruling 41's narrowing, and its history has a row for it. |
| H-03 | wrong | fix | In the tracker's later amendments, the rows for rulings 7 to 9, 29 and 30, and 32 and 33 add what M22 and M41 changed. A new table under them, "Changes that no ruling has accepted", lists M22 (decision 8c), M41 with the integration's choice on PLC-023 and LIF-036 (decision 17), and M69 (DL11). The overview's sentence on a left-out `placement` says that the machine provider places an instance that has a program to run, and that one that has none is placed on `local` (PLC-002). |
| H-04 | wrong | fix | The tracker's paragraph "Left for a reader by the records" marks each point with what settled it: the check against the protocol and IFC-038 by reader 6; the control in progress by M35; MAN-017 by M30; PRV-023's `()`, PRV-024 and SEC-013's end of a connection by ruling 42's verification; PRV-021's held stop and SEC-023's failing read by the items that waited on ruling 43. It lists as still open the four the report names: TPL-004 and CTR-004 on a hook's default, a repeated named argument such as `-e`, PLC-024's form of a cause, and the late onward request (OPS-028, A65). |
| H-05 | misleading | fix in part | Step 4 of "What happens next" reads "Next: pull request 7 is merged", once round 5's fixes and record are committed to it. "For the next reader" says which text to read: not `d0c425f`, but the head of `docs/pre-codex`, or of `main` once pull request 7 is merged, in a tree that holds this directory. This README says the same. Not done: the commit and push, which this pass was told not to make. The commit cannot be named until it exists; the coordinator names it here and in the tracker when it commits. Note of 2026-11-07: pull request 7 was merged by squashing as commit `c4d7d33`, which round 6 read; step 4 of "What happens next" in the tracker reads "Done" and names it. |
| H-06 | misleading | fix | The tracker's definition of unsafe reads: "any by which an engine that keeps every promise could grant access it should not, leak a secret, lose or repeat an effect, or report a wrong result", as round 5 classed M01 and 5-4-02. |
| H-07 | misleading | fix | "Waiting for the owner" holds V6, A20 and A21, says that the gate's G8 and G11 also need the owner and that the proposals of A18, A65, A66 and A67 come to the owner once written, and moves what ruling 43 settled to a sentence under the table. Note of 2026-11-07: V6's row was removed when the coordinator settled V6, and the table now holds A20, A21 and what round 6 left for the owner. |
| H-08 | misleading | fix | The tracker's row on the readiness gate gives the state as assessed on 2026-11-06, before rulings 40 to 43, and not since, and says that G1's only stated shortfall, secrets, is now written and that the rows for G7, G8 and G9 cite what has since changed. The gate is not assessed again here: that assessment was the owner's to confirm under ruling 39, point 8. |
| H-09 | minor | fix | A61's status adds round 5's reader 5: 14 findings, 7 of them misleading, all fixed as M100 to M111, M30 and M87, with links to its report and to F7 of the decisions. |
| H-10 | minor | fix | "For the next reader" links this README for the counts of rounds 2 to 5, and the re-audit record for round 1's. |
| H-11 | minor | fix | The tracker's table of identifiers has a row for round 5's M01 to M111, V1 to V13 and V2-01 to V2-16, and the row for V and the note above the table say that a V written with round 5 is the verification's, never a vocabulary decision. |
| H-12 | minor | fix | A11's status adds ruling 43 (Q1): two forms of proof, the token form always accepted at the engine's address and mutual TLS beside it (IFC-042, DL13), and says that what waits for A4 is how a native Agent obtains and presents the token. |
| H-13 | minor | fix | The tracker's row on the specification says "as it stood at ruling 39". The head comments of the files under `spec/` that say "as it stands at ruling 39" are history lines of their time, each followed by later dated lines, and keep their words. |

Not changed by this pass: `AGENTS.md`, whose sentence on pull request 7 the coordinator brought
up to date when it merged; and the rows for G1, G7, G8 and G9 in
[INDEPENDENT_BUILDS.md](../../../implementation/INDEPENDENT_BUILDS.md), which keep the
assessment as the owner confirmed it, beside its paragraph of later changes.

Checks, run from the repository root on the working tree after these fixes:

```text
$ /usr/bin/python3 - <<'PY'   # the validation of CONSOLIDATION.md
452 promises ok
$ python3 tools/check_docs.py
Files: listed by Git (tracked, and untracked but not ignored).
Checked 176 Markdown files and 2436 local links (offline).
Errors: 0
$ python3 tools/set_names.py --check
Names: corePackage x437, apiVersion x257, metaPrefix x104; of `identity`, CONDUIT_URL x19,
CONDUIT_IDENTITY_DIR x10, ca.crt x11, jwks.json x12, engine-id x14, token x18, tls.crt x16,
tls.key x14. Errors: 0
$ python3 tools/check_schemas.py
5. ... Checked 1 controls, the 5 operations that take no `revision`, the 1 that returns one,
the tables of the two READMEs, and 1015 citations of promises in 112 files. ...
6. Applied 25 rules of this tool, which come from 24 promises, to the core package and 3
example directories. 30 sets of documents in 30 files under examples/invalid-by-rule/ were
each rejected by the rule they are named for; each of their 53 documents is valid against
the schemas.
Errors: 0
$ git diff --check
(no output)
```
