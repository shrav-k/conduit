# Consolidation pass 1 — instructions and log

Owner ruling of 2026-11-05 (plan-audit ruling 32): every decision in the
[re-audit record](README.md) is accepted as recommended. This file tells whoever edits `spec/`
how to apply those decisions together with the findings that need no ruling. The log of what
was done is appended at the end.

## Rules for every edit

1. A promise keeps the exact form `id`, `layer`, `level`, `when`, `then`, `observable`,
   `vectors: []`, with folded `>-` text indented four spaces and lines under 97 characters.
   `layer` is `language`, `convention` or `interface`. `level` is `must`, `must not`, `should`
   or `may`.
2. IDs are never reused or renumbered. A removed promise is deleted and named in its file's
   header comment with where it went, as in "CON-015 was folded into LIF-003 and LIF-006". A new
   promise takes the next number never used in that file's prefix; check the header comment for
   numbers already retired.
3. No promise moves between files in this pass. Where a reader says a promise is in the wrong
   layer, change its `layer` in place.
4. Apply only these splits from reader 1 section 5: 5.1 (OBJ-011, SUB-007), 5.2 (LIF-014) and
   5.3 (PLC-011). Leave the others.
5. Add no behavior beyond the accepted decisions and the reader fixes that follow from rulings
   already made. Where a finding would need a decision that is not in the record, leave the text
   and report it.
6. Every promise about a request that is not dispatched names exactly one of refused, invalid,
   conflict and unavailable. Where several apply, the order is decision Q5.
7. A reference to another promise by ID must still exist after the pass. The planned removals
   are listed below.
8. Run the validation command at the end of this file before reporting.

## Words (decisions W1 to W4 and reader 2's term findings)

| Use | Not |
| --- | --- |
| "exists", "is not unregistered" | "registered" for anything but the lifecycle state |
| "record" for a log entry only; "the invocation's state" for what is kept per operation identity | "record" for stored facts or for a mutable entry |
| "register" for the one operation that takes documents; "instantiate" for what registering an instance document does | "create" as the name of an operation |
| "the document is rejected", which means its registration is not dispatched and reports invalid, or conflict for a clash with an existing name | "rejected" with no reason where a request is described |
| "the engine" for the running thing | "Conduit" or "the system" for the running thing; "implementation" for an engine |
| "target" for a Conduit's end only | "target" for what a reference points at or what an effect changes |
| "entry" for a Pool's entry only; "item" elsewhere | "entry" for a manifest line or a map member |
| "is the consumer of an open Conduit to" | "holds a right", "holds a Conduit" |
| actor, caller | requester, holder, submitter, observer, reader, author |
| "not permitted" as the meaning of refused, whoever asks | — |
| "nested instantiation" | "declaration" for a declared Conduit or an entry |
| "document" | "declaration" for a submitted document |
| "the Model Context Protocol" spelled out | MCP |
| "is permitted" | "is covered" for a request |

## Planned removals (so that cross-references can be fixed)

| Removed | Folded into |
| --- | --- |
| ACC-004 | ACC-003 |
| ACC-007 | ACC-006 |
| ACC-013 | new promise in conduits.yaml (decision G9) |
| ACC-020, ACC-021 | REC-001 and OPS-005 |
| ACC-025 | ACC-033 (decision A1) |
| CON-010 | LIF-008 |
| CON-015 | LIF-001, LIF-003, LIF-005, LIF-006 |
| POOL-010 | LIF-009 |
| PLC-010 | OBS-003 and LIF-010 |
| PLC-014 | CTR-004 |
| FST-004 | OBJ-002 |
| PKG-002 | NAM-006 |
| PKG-008 | NAM-011 |
| PKG-009, PKG-010, PKG-011 | TPL-004 and CTR-004 |
| PKG-015 | PLC-007 |
| INS-015 | OBJ-013 |
| GRA-013 | GRA-019 |
| GRA-016 | GRA-020 |

Other duplicates in reader 1 section 1 are trimmed as that report says, keeping both IDs.

## Where each decision lands

Group A owns `conventions/access.yaml`, `conventions/lifecycle.yaml`, `promises/conduits.yaml`,
`promises/pools.yaml`, `promises/placement.yaml`, `promises/first-start.yaml`.
Group B owns `conventions/operations.yaml`, `conventions/revisions.yaml`,
`conventions/records.yaml`, `conventions/observation.yaml`, `conventions/implicit.yaml`,
`interface/implementation.yaml`, `implementation-defined.yaml`.
Group C owns `promises/objects.yaml`, `promises/instantiation.yaml`,
`promises/submissions.yaml`, `promises/templates.yaml`, `promises/graphs.yaml`,
`promises/contracts.yaml`, `promises/schemas.yaml`, `promises/naming.yaml`,
`promises/packages.yaml`, `schema-profile.md`.
`definitions.md` and `README.md` are edited last, by the coordinator.

| Decision | Group A | Group B | Group C |
| --- | --- | --- | --- |
| U1 restrictions on the wire | ACC-016, ACC-017 cite the wire keys | IFC-006 gains `conduit.dev/conduit`, the Conduit's ID, and `conduit.dev/restrictions`, a list of applied requirements, each a requirement reference and its arguments, covering the Conduit's own, any policy that applies, and those carried from the served call | PKG-020: a plain implementation states no `supports` |
| U2 onward requests | ACC-016 to ACC-018 | The engine gives every invocation an ID and sends it as `conduit.dev/invocation`; `conduit.dev/operationIdentity` is sent only when the caller supplied one. An onward request names the invocation it serves in `conduit.dev/serving`. An instance that is not an actor must name one, or the request is refused | — |
| U3 model work by code | — | New promise in implicit.yaml | — |
| U4 plain and deduplication | — | OPS-012 | PKG-020 |
| U5 re-running setup or retire | — | New promise in operations.yaml for a step with an unknown outcome; DL1 gains "what a re-run of setup or retire does with steps that already ran" | TPL-007: a runtime the instance holds is not started again |
| U6 close during a call | New promise in access.yaml | — | — |
| U7 policies and the bound | ACC-031 and a new promise | — | — |
| U8 passing a right on | ACC-030 rewritten, ACC-012 | — | — |
| U9 purge | — | REC-004 | — |
| U10 who a connection is | — | New entry DL13 and a new interface promise | — |
| U11 digest mismatch | — | — | New promise in packages.yaml beside PKG-003 |
| Q1 new attempt | — | OPS-009, OPS-012, IMP-007 | — |
| Q2 identity scope | — | OPS-008, OPS-010 | — |
| Q3 graph outcome | — | New promise in operations.yaml | GRA-003 becomes `convention` and cites it |
| Q4 controls | — | REV-007 to REV-009 | — |
| Q5 order of reasons | LIF-022 keeps refused | New promise in operations.yaml | — |
| Q6 what is recorded | — | REC-001 | — |
| Q7 the consumer's side | ACC-001 excepts retrieving one's own result | New promises in implementation.yaml; OPS-016 | — |
| A1 creating a Resource | ACC-033, pools.yaml | — | SUB-002 if it speaks of authority |
| A2 reconcile; lost locations | ACC-033, LIF-017, LIF-018, PLC-011 | OPS-014 | — |
| A3 replacement Conduit | LIF-009, CON-004 | — | — |
| G2 references | — | — | New promises in objects.yaml; OBJ-006 |
| G3 nested names; entry from outside | New promise in pools.yaml for `{kind: Pool, name, entry}` | IFC-012 and IFC-014 unchanged | OBJ-003 covers nested instances; OBJ-014 |
| G4 where a step runs | ACC-018 | — | GRA-017 gains `conduit` |
| G5 location facts | PLC-007, PLC-008 | — | CTR-005: a requirement that applies to placement states `schema`, a schema the location's facts must satisfy, and declares no inputs |
| G6 target not ready | New promise in conduits.yaml | — | — |
| G7 template replaced | LIF-009 | — | SUB-007, INS-002, INS-013 |
| G8 contract versions | — | — | New promises in naming.yaml |
| G9 unsupported filter | New promise in conduits.yaml; ACC-013 removed | — | — |
| G10 contracts in a project | — | — | SUB-005, NAM-003, new promise for `#name` |
| G11 controls | ACC-033: a control is authorized by a Conduit to the instance itself | OPS-020 | CTR-002, CTR-004, CTR-006, TPL-010, TPL-011 |
| G12 retirement | LIF-014, LIF-020, LIF-021, CON-008 | — | TPL-009 |
| G13 shared host | PLC-013 generalized to any opening and to each request | — | — |

## Choices made by the coordinator while planning

| Choice | Why |
| --- | --- |
| The engine gives every invocation its own ID, separate from the caller's operation identity. | A caller need not supply an identity (OPS-011), and an onward request has to name what it serves. |
| A requirement that applies to placement is a fixed schema with no inputs. | A schema cannot take arguments without a templating rule, which decision G5 does not add. |
| Promises are not moved between files in this pass. | An ID's prefix names its file. Layers are corrected in place. |
| Only three of reader 1's proposed splits are applied. | The others join halves that are tested together. |

## Validation

```sh
/usr/bin/python3 - <<'PY'
import yaml, glob, collections
ids = collections.Counter()
for f in sorted(glob.glob('spec/promises/*.yaml') + glob.glob('spec/conventions/*.yaml') + glob.glob('spec/interface/*.yaml')):
    for p in yaml.safe_load(open(f)):
        assert set(p) == {'id','layer','level','when','then','observable','vectors'}, (f, p.get('id'))
        assert p['layer'] in ('language','convention','interface'), p['id']
        assert p['level'] in ('must','must not','should','may'), p['id']
        ids[p['id']] += 1
assert not [k for k, v in ids.items() if v > 1]
print(len(ids), 'promises ok')
PY
```

## Log

Appended by each group when it finishes.
