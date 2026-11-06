# Consolidation pass 1 — log of group B

Group B's part of [consolidation pass 1](CONSOLIDATION.md), done on 2026-11-05. Files edited:
`spec/conventions/operations.yaml`, `revisions.yaml`, `records.yaml`, `observation.yaml`,
`implicit.yaml`, `spec/interface/implementation.yaml` and `spec/implementation-defined.yaml`.
No promise was removed and no ID was reused. Finding IDs are written R1 (reader 1), R2 (reader
2) and R3 (reader 3).

## Counts

| File | Changed | Removed | Added |
| --- | --- | --- | --- |
| `conventions/operations.yaml` | 19 | 0 | 4 (OPS-021 to OPS-024) |
| `conventions/revisions.yaml` | 4 | 0 | 3 (REV-010 to REV-012) |
| `conventions/records.yaml` | 6 | 0 | 0 |
| `conventions/observation.yaml` | 5 | 0 | 0 |
| `conventions/implicit.yaml` | 4 | 0 | 1 (IMP-008) |
| `interface/implementation.yaml` | 12 | 0 | 8 (IFC-018 to IFC-025) |
| `implementation-defined.yaml` | 4 entries | 0 | 1 (DL13) |

The validation command of CONSOLIDATION.md passes, `implementation-defined.yaml` parses as 13
entries with `id`, `entry` and `document`, and `python3 tools/check_docs.py` reports no error.

## New IDs, for cross-references from other files

| ID | Holds |
| --- | --- |
| OPS-021 | Decision U5: a step with an unknown outcome when setup or retire runs again |
| OPS-022 | Decision Q3: the outcome of an operation that a graph implements (GRA-003 cites this) |
| OPS-023 | Decision Q5: the order of the four reasons |
| OPS-024 | Decision Q1: a repeat after a failed outcome is a new attempt |
| REV-010 | Decision Q4: a control reconciled as having taken effect |
| REV-011 | R3-37: a change request that names no revision is invalid |
| REV-012 | R3-37: the revision is checked at the moment of storing |
| IMP-008 | Decision U3: model work asked for while serving an unmarked invocation |
| IFC-018 | Decision U1: what `conduit.dev/restrictions` holds |
| IFC-019 | Decision U1: an implementation applies every restriction it is sent |
| IFC-020 | Decision U2: `conduit.dev/serving` |
| IFC-021 | Decision U2: an instance that is not an actor must name the invocation it serves |
| IFC-022 | Decision U10: which instance a connection belongs to |
| IFC-023 | Decision Q7: a consumer supplies `conduit.dev/operationIdentity` |
| IFC-024 | Decision Q7: the tool for retrieving the caller's own results |
| IFC-025 | Decision Q7: `conduit.dev/outcome` or `conduit.dev/reason` in a consumer's result |
| DL13 | Decision U10: identifying consumers |

## What changed

### conventions/operations.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| OPS-001 | "The record of the invocation's end" in place of "the record of the invocation" | W2, R1-2.1, R2-T10 |
| OPS-002 | "what the effect changes" in place of "the target of the effect" | R2-T11 |
| OPS-003 | Same | R2-T11 |
| OPS-004 | "The record of the invocation's end" | W2 |
| OPS-005 | A repeat is excepted; the response to the caller and the record carry the same reason (from ACC-021); cites OPS-023 | Q1, Q5, R1-1.17, R1-2.4 |
| OPS-006 | Scoped to implementations that carry Conduit's keys; cites IFC-009 and IFC-010 for plain | R1-1.21, R1-2.3 |
| OPS-007 | "No outcome stated" scoped to implementations that carry Conduit's keys; an output that fails validation is unknown for a plain one too; succeeded or failed may come from IFC-010 | R1-2.3, R3-39 |
| OPS-008 | Identity scoped to one caller and one target; "one invocation, whose state is read from its records" in place of "one record" | Q2, W2, R3-23 |
| OPS-009 | Defines a repeat (same caller, target, identity and contents) and "same contents" (same operation, arguments equal under SCH-007); covers only in progress and succeeded | Q1, Q2, W2, R1-2.4, R3-22 |
| OPS-010 | Scoped by caller and target; "the invocation's state is unchanged" | Q2, W2 |
| OPS-011 | "Two invocations", each with its own ID under IFC-006, in place of "two records" | W2, U2 |
| OPS-012 | A repeat after unknown is dispatched as a new attempt only if the contract declares `deduplicates: true` and the implementation is not plain; otherwise the state unknown is returned | Q1, U4, R3-3, R3-22 |
| OPS-013 | "No new attempt is dispatched under OPS-012"; "the invocation's state" | Q1, W2 |
| OPS-014 | Reconciling is authorized against the target ACC-033 gives for setting up and retiring the instance; the earlier record is kept and a new one added | A2, W2, R3-30 |
| OPS-015 | "Not dispatched, and reports refused"; "whatever open Conduits it is the consumer of" | Rule 6, R2-T6 |
| OPS-016 | Asked by operation identity; needs no open Conduit; cites the tool of IFC-024 | Q7, R1-2.24, R3-13 |
| OPS-018 | Scoped by caller and target; "what the effect changes"; "the engine or the implementation" in place of "the service" | Q2, R2-T11 |
| OPS-019 | "Instantiate" dropped from the verbs and "replace" named; a request MAY carry an operation identity, and without one is a new invocation each time | W3, R3-44, R1-2.21 |
| OPS-020 | A control is an operation defined by an Operation contract that also states the settings it changes, listed by a Conduit like any other and authorized by an open Conduit to the instance itself | G11, R3-24, R2-T28 |
| OPS-021 | New | U5, R3-4 |
| OPS-022 | New | Q3, R3-21 |
| OPS-023 | New | Q5, R1-2.6 |
| OPS-024 | New | Q1 |

### conventions/revisions.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| REV-001 | "exists" in place of "is registered" | W1, R2-T9 |
| REV-006 | "A control is requested and one of its inputs is not supplied" in place of "is invoked and its input omits a field" | G11, R2-S2, R2-T28 |
| REV-007 | Observable points at REV-008 for confirmed settings that cannot be stored | Q4, R1-2.10 |
| REV-008 | The outcome is unknown and nothing is stored | Q4, R1-2.10 |
| REV-010 | New | Q4, R3-25 |
| REV-011 | New | R3-37 |
| REV-012 | New | R3-37 |

REV-009 was only re-wrapped to keep its lines under 97 characters.

### conventions/records.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| REC-001 | "A request ends without being dispatched" joins the list; every request is found with how it ended; what the record of an access decision states (from ACC-020) | Q6, R1-2.2, R1-1.17 |
| REC-002 | A record also holds the engine's ID for the invocation; the operation identity only if the caller supplied one | U2 |
| REC-003 | "Every Interface has detached" and "an export of it has failed" removed; a reconciliation adds a record and alters none | R1-3.7, W2 |
| REC-004 | A purge keeps, for every invocation, its operation identity and its outcome; a repeat after a purge is still recognized | U9, R3-10 |
| REC-005 | "An actor that reads records" in place of "a reader" | R2-T7 |
| REC-006 | "The result of reading them says" in place of "the reader is told" | R2-T7 |

### conventions/observation.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| OBS-001 | "an operation that an actor requested" in place of "a registered change" | W1 |
| OBS-003 | Takes the unreachable-location case from PLC-010 | R1-1.3 |
| OBS-004 | Excepts an implementation restoring its own runtime | R1-2.25 |
| OBS-005 | Restoring means the implementation keeps alive what is behind the handle the instance holds; the handle stays valid; the engine never invokes `start` again. The model-work clause moved to IMP-002 | R1-3.1, R1-1.16 |
| OBS-006 | Model-work clause dropped (IMP-002 has it); "the actor that inspects it" in place of "the observer" | R1-1.16, R2-T7 |

### conventions/implicit.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| IMP-002 | "A message or a notice is delivered to an Agent" restated as an unmarked operation invoked on an Agent; "an implementation restores a runtime" added, from OBS-005 | R1-3.7, R1-1.16 |
| IMP-003 | Names the key `startsModelWork: true`. Stays here, layer `language` | R1-4.8 |
| IMP-005 | Reduced to "the setup operation is never marked", so that IMP-004 does the rejecting. Stays here, layer `language` | R1-1.16, R1-4.8 |
| IMP-007 | "At most once per attempt"; a new attempt only for a repeat that OPS-012 or OPS-024 dispatches; "in progress" in place of "in flight" | Q1, R1-2.4 |
| IMP-008 | New | U3, R3-9 |

IMP-004 is unchanged and keeps the layer `language`. The header comment spells out the Model
Context Protocol.

### interface/implementation.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| IFC-001 | Names `tools/call`, `name`, `arguments` and `structuredContent`; a plain implementation's tool name comes from its `tools` map | R2-N2, R1-2.21 |
| IFC-003 | Names `io.modelcontextprotocol/protocolVersion` in `_meta` and the JSON-RPC error for an unsupported version | R2-N2 |
| IFC-004 | Names the four things relied on by the protocol's field names | R2-N2 |
| IFC-006 | Keys are now `interfaceVersion`, `instance`, `operation`, `invocation`, `caller`, `restrictions`, with `operationIdentity` only when supplied and `conduit` only when the request came through a Conduit; the interface version is `v0` | U1, U2, R3-1, R3-2, R3-14, R2-S24, R1-2.21 |
| IFC-007 | Example of what the protocol itself requires | R2-N2 |
| IFC-009 | "Has the outcome unknown" in place of "is recorded as unknown" | W2 |
| IFC-010 | Names `resultType`, `isError` and JSON-RPC errors; a result whose declared output fails validation is unknown under OPS-007 | R2-N2, R3-39, R1-2.3 |
| IFC-012 | The tool list also holds the tool of IFC-024; states the protocol's limits on a tool name | Q7, R2-N2 |
| IFC-013 | Keeps only the tool list; what a call reports is ACC-011 | R1-1.1 |
| IFC-014 | Layer changed in place to `language`; "the document is rejected" in place of "the Conduit is not created" | R1-4.1, W4 |
| IFC-015 | "Through the engine" in place of "through Conduit"; cites IFC-020 | R2-T20, U2 |
| IFC-016 | Names the protocol's tasks extension, `io.modelcontextprotocol/tasks` | R2-N2 |
| IFC-018 to IFC-025 | New, as listed above | U1, U2, U10, Q7 |

### implementation-defined.yaml

| ID | What changed | Answers |
| --- | --- | --- |
| DL1 | Gains "what a re-run of setup or retire does with steps that already ran" | U5 |
| DL3 | Entry is "A request that waits for an actor's answer"; "during an outage" restated | R1-3.7 |
| DL4 | "Whether a request is not dispatched at once, reporting unavailable, or waits, and for how long" | R1-3.7 |
| DL9 | The order left to the engine is only among failed checks that give the same reason | Q5 |
| DL13 | New: identifying consumers | U10, R3-11 |

## Checked against the protocol

The field names were checked on 2026-11-05 against the pages of revision 2026-07-28 at
modelcontextprotocol.io (base protocol, versioning, tools). Confirmed there: `tools/call` with
`name` and `arguments`; `structuredContent`; `isError`; `resultType` with the values `complete`
and `input_required`, an absent one read as `complete`; `io.modelcontextprotocol/protocolVersion`
required in each request's `_meta`; an unsupported version answered with the JSON-RPC error
`UnsupportedProtocolVersionError`, code -32022; an unknown tool and a malformed request answered
with JSON-RPC errors; tool names of 1 to 128 characters from letters, digits, `_`, `-` and `.`;
and tasks as the extension `io.modelcontextprotocol/tasks`, not part of the core protocol.

## Choices made while writing, to confirm

- `conduit.dev/restrictions` is always present and is an empty list when no restriction applies.
  Each item has the keys `requirement` and `arguments`, as an applied requirement has in CON-018.
- A new attempt is an attempt of the same invocation, so it carries the same operation identity
  (OPS-012) and by implication the same `conduit.dev/invocation`. No promise says the second in
  so many words.
- "No step had an effect" (OPS-022) is read as: every step that was dispatched either failed or
  is of an operation whose contract states no `effect`.
- IFC-021 refuses an instance that is not an actor when it names no invocation "it serves". An
  ID the engine never dispatched to that instance is read as naming none.
- A record holds the engine's ID for the invocation (REC-002), since the state of an invocation
  with no operation identity can be read from its records in no other way.
- The retrieval of the caller's own results is by operation identity only (OPS-016, IFC-024), as
  decision Q7 says.

## Left undone, and why

1. **How a restriction's refusal reaches the caller.** Decision U1 has the implementation apply
   each restriction and refuse when it cannot (IFC-019). ACC-008 and ACC-014 say such a request
   "reports refused" or "unavailable" and is not dispatched, yet the request has reached the
   implementation. Nothing on the wire lets an implementation say "a restriction forbids this" or
   "I cannot apply this" so that the engine can report that reason and not the outcome failed.
   Needs a decision.
2. **A control that was dispatched and then loses the revision check at storing** (REV-012).
   The promise says it stores nothing. Its outcome is not decided; unknown, as in decision Q4,
   is the nearest ruled case. DL9 still lets an engine run several controls at once on one
   instance.
3. **"In progress" on the consumer's side.** IFC-025 gives `conduit.dev/outcome` three values.
   How the answer to a repeat (OPS-009) or to a retrieval (OPS-016) says "in progress", and how
   "withheld" (OPS-017) is said, are not decided. They may belong to the core package's tool.
4. **An earlier request that was not dispatched.** Whether a later request with the same
   caller, target and identity is then judged as a new request is implied (no invocation
   exists) and not stated.
5. **A purge and "different contents".** Decision U9 keeps the identity and the outcome. If the
   contents are purged, OPS-010 can no longer tell a repeat from a request with different
   contents.
6. **A reconciliation of a control whose stated settings fail the schema** (REV-010), and
   whether that reconciliation is itself checked against a revision. Not in the record.
7. **Tool names.** No promise bounds the length of an export name, so a name of the form Conduit
   name, dot, export name can pass the protocol's 128 characters. The collisions of R3-40 are
   also open: two operations of one capability with the same export name from different
   packages (group C's, reported only), components of a declared composite Conduit, and a
   rename onto a declared Conduit's name.
8. **`_meta` key prefix.** The protocol says a prefix SHOULD be in reverse DNS form, such as
   `dev.conduit/`. The decisions fix `conduit.dev/`. Valid, and against a SHOULD.
9. **U10 against the baseline revision.** Decision U10 speaks of "which instance a connection
   belongs to". Revision 2026-07-28 says a server must not rely on earlier requests on a
   connection for the client's identity, and that a tool list must not vary by connection,
   though it may vary by the authorization presented on each request. IFC-022 is written as
   decided; it may need to say "request" where it says "connection".
10. **A plain server's `input_required` result.** IFC-010's table covers a complete result, an
    error and no answer. A result whose `resultType` is `input_required` is none of these.
11. **Error codes.** IFC-010 names JSON-RPC errors and gives no codes for "tool unknown" and
    "request malformed". The protocol uses -32602 for the first; the second was not pinned down.
    IFC-008's "an error that says so" is likewise not tied to a field.
12. **A request that cannot be read** (OPS-023) is invalid. On the wire such a request gets a
    JSON-RPC error with no result, so IFC-025's `conduit.dev/reason` has nowhere to go.
13. **A scheduled start after a close and a new Conduit** (R3-31, IMP-006). Not in the record.
14. **An actor that names in `conduit.dev/serving` an invocation it does not serve.** IFC-021
    covers only instances that are not actors.
15. **Whether the retire operation may be marked as starting model work.** IMP-005 covers setup
    only, as before.
16. **W4 as a promise.** The vocabulary table defines "the document is rejected" as a
    registration that is not dispatched and reports invalid, or conflict for a clash of names.
    No promise in group B's files states it, because one promise may name only one reason (rule
    6). It is left to `definitions.md`.
17. **Splits not applied**, as instructed: R1-5.7 (IFC-011), 5.8 (IFC-012), and the parts of
    5.10 that concern OBS-006, IMP-006 and REC-004.
18. **Not moved**, under rule 3: R1-4.9 asks for PKG-016 to move beside IFC-017. Reported only.
    R3-49 (a plain `url` server shared by several instances receives no `conduit.dev/instance`
    and so cannot keep PKG-016) is group C's and is open.
19. **The header of `implementation-defined.yaml` and its key `entry`** keep the word "entry",
    which the vocabulary table reserves for a Pool's entry. The key is part of the file's form.

## Terms the new text uses that `definitions.md` does not yet define

For the coordinator: repeat; attempt and new attempt; in progress; the invocation's state;
access decision; withheld; restore (a runtime); onward request and "serves" an invocation;
connection; the engine's ID for an invocation. The rows for `record`, `control`,
`implementation interface`, `plain` and "How a request ends" (refused as "not permitted") need
to follow decisions W2, G11, U1, U2, U4 and Q5.
