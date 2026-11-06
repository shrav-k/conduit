# Applying the owner's ruling on the core package, the schemas and the manifest

Owner ruling of 2026-11-05 (plan-audit ruling 37): the batch in the [record](README.md) is
accepted as drafted, with the changes below. This file tells whoever edits `spec/` what that
means for each item. The rules for editing promises, and the vocabulary, are in
[the consolidation instructions](../reaudit/CONSOLIDATION.md).

## What is accepted as written

- Every choice C1 to C56 and every name N1 to N42 in section (b) of the record.
- Every recommendation P1 to P14 in section (c).
- For every item D1 to D32 in section (d), what the batch does, except where a row below says
  otherwise.

## Changes the owner accepted on top of the draft

| ID | Change | Promise side | Core and schema side |
| --- | --- | --- | --- |
| X1 | A location's facts change only by a control, and the draft has none (D11). | PLC-008 names the control. | Add the control `set-facts.v1`, an Operation contract with `changes: [/facts]`, in a new capability `location-settings.v1` that the location type offers. It takes `revision` and `facts`, and returns `settings`. |
| X2 | Requests name an object by reference only (D10). | LIF-007 no longer says "by its ID": a retired instance's name still resolves until it is unregistered. | — |
| X3 | A bound in the wrong place (D29). | New promise in conduits.yaml: a Conduit that applies `bound.v1` and whose target is not `access`, or that applies it more than once, is not instantiated. | An invalid example for each case. |
| X4 | The reserved mark (D30). | CTR-003 or a new promise in contracts.yaml: a package other than the core package that holds an Operation contract marked `reservedToRoot` is rejected when it is installed, and a project that holds one is rejected when it is submitted. | `operation.v1` documents the mark. |
| X5 | `#name` carries no kind (D19). | New promise in naming.yaml: within one project or package no two documents share a name, whatever their kinds. | An invalid example. |

## How each open point and disagreement is written into the promises

Items not listed need no change to a promise.

| Item | What the promises say after this pass |
| --- | --- |
| P1 | SUB-008 and NAM-009: the digest of an unpublished project is the SHA-256 of the RFC 8785 form of one object with `documents`, its contract documents ordered by kind and then name, and `manifest`, left out when there is none. Add RFC 8785 to the standards table in schema-profile.md. |
| P2 | REC-004 stays. Entry DL2 gains "how the digest of an invocation's contents is computed". |
| P3 | SCH-015: a `$ref` is a same-document JSON Pointer, a qualified reference to an exported Schema contract, or `#name` for a Schema contract of the same project or package. |
| P4 | INS-012 becomes a closed list: a destination is accepted only if it is a place the promise lists, the whole of `config`, or the whole of one of the `arguments` maps it lists. An item of `operations`, a whole nested instantiation, a requirement's `requirement`, and anything under `inputs` are rejected. |
| P5 | New promise in packages.yaml: a rejected installation is not dispatched and reports invalid; unavailable when the archive cannot be fetched now; conflict when that address and version is already installed with another digest. |
| P6 | NAM-001: no trailing dot; a host whose last label is all digits is not a host name; an internationalized name is written with `xn--` labels; a label is at most 63 characters. |
| P7 | CON-003 and OBJ-003: a component Conduit's name never exceeds 63 characters; a generated part is shortened so that the whole fits, and a document whose given name would make it longer is rejected. |
| P8 | CON-026: a rename that would make a tool name longer than 128 characters reports conflict. |
| P9 | IFC-009 or IFC-027: a result that carries both an outcome and a reason is recorded as unknown. |
| P10 | New promise in operations.yaml: a repeat of a request that ended with a reason is a new request, decided afresh. |
| P11 | IFC-026: the answer's `_meta` holds `io.github.shrav-k.conduit/restriction`, the item of the list it was sent that forbade the request or could not be applied, and its `isError` is true. |
| P12 | OPS-009 and IFC-025: the answer to a repeat of an invocation that has not ended carries neither an outcome nor a reason, and its `_meta` holds `io.github.shrav-k.conduit/state` with the value `inProgress`. |
| P13 | ACC-034 and IFC-025: where content is withheld the result carries its outcome and no `structuredContent`, and its `_meta` holds `io.github.shrav-k.conduit/withheld` with the value true. |
| P14 | REV-012: while a setup, a retirement or a change of one instance has been dispatched and has not ended, no other of them is dispatched for that instance. Setting up and retiring name no revision and advance none. |
| D2 | NAM-009 and PKG-005: for the core package a template and an instance hold its address and version alone. |
| D3 | IFC-012's observable: a tool name ends in the operation's version segment, as in `checkout.directory.read.v1`. |
| D4, D5 | OBJ-013, OBJ-014, POOL-012 and CON-018: `arguments` may be left out and then means none; an instance document may leave out `metadata`. |
| D6 | OBJ-019: inside a schema the names of properties are the author's, and the keywords are closed by SCH-002. |
| D8 | ACC-033: registering a Resource into a Pool that exists needs a Conduit to `management` and a Conduit to that Pool, each listing `register.v1`; asking for a new instance for a nested instantiation needs the same two for `register-replacement.v1`. |
| D9, D20 | TPL-003, TPL-010, PKG-020, CTR-002, NAM-018 and OBJ-019: wherever a selection or such a key is "a qualified reference", `#name` is accepted for something in the same project or package. |
| D12 | ACC-031: a policy is not configuration. Applying one returns its ID, removing one names that ID, neither names a revision, and inspecting `access` lists the policies in its status. |
| D13 | REV-006 and REV-007: a control's result states the settings it confirmed in the output `settings`; `revision` is the one input that must always be supplied. |
| D14 | OBJ-021: a value is a reference where the schema that governs it is a reference schema, directly, as the `items` of a list, or as a branch of `anyOf` or `oneOf`. |
| D15 | POOL-005: `{entry: name}` anywhere other than in an entry of a Pool template is rejected. |
| D16 | OBJ-006 or INS-006: where an input's schema fixes the kind, a bare name is accepted for it, and the destination receives the reference with its kind. |
| D17 | INS-014: a template with no `config` is checked against its type's schema as an empty map. |
| D18 | NAM-022: a document refers to something in its own project or package by `#name`; its own full address is not in its manifest and is rejected under NAM-007. The core package's own documents are the exception. |
| D21 | IFC-025: a result that ends with a reason may carry `diagnostics` in its `structuredContent`, as a rejected registration does. |
| D22 | OPS-016 and IFC-024: a caller asks for its result by operation identity and target. |
| D23 | REC-004: a purge removes the records made before a time the request names. |
| D24 | PKG-019: `supports` and `placement` may be left out, and then the implementation supports no requirement and has no requirement on placement. |
| D25 | CON-016: a Conduit template that is not composed requires `capability`; `operations` is required unless an input's destination fills it; `requirements` may be left out. |
| D26 | SCH-008: whether a `default` is allowed is judged where it is written, in the schema that holds it. |
| D27 | CTR-001: a contract's name has at least one segment before its version segment. |
| D28 | SUB-001 or SUB-005, with MAN-011: submitting a directory reads the `.yaml` and `.json` files in that directory itself and nothing below it. |
| D31 | PLC-008: a location's document that states a placement is rejected. |

## Definitions the owner accepted

| Term | Meaning |
| --- | --- |
| Interface | Where a person, or a system outside, acts on the system. |
| Agent | A participant whose model decides its own actions. |
| Resource | A shared thing that instances use. A Pool owns it. |
| Pool | What owns and groups Resources. |
| model work | An Agent's model being run: a turn started or continued. |
| mediated | Said of a restriction that applies to requests that pass through the engine. |
| enforced | Said of a restriction that an isolation plugin also prevents a consumer from going around. |
| project | One directory with at most one manifest. |
| published | Made available as an archive under an address. |

## Who edits what

Editor P owns every existing promise file under `spec/promises/`, `spec/conventions/` and
`spec/interface/`, `spec/implementation-defined.yaml`, `spec/schema-profile.md`,
`spec/definitions.md` and `spec/README.md`.

Editor S owns `spec/core/`, `spec/schemas/`, `spec/promises/manifest.yaml`,
`tools/check_schemas.py` and the record, `docs/proposals/core-and-schemas/README.md`.
