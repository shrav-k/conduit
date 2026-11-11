# Re-audit of the whole specification — round 1, decisions accepted

Status: ruling 32: in force; amended by 38 (point 12); amended by 41 (items 1, 1c, 3a, 5, 6a, 8, 8b, 18, P20); ruling 33: in force; amended by 41 (items 1, 5, 8, 10, 18); amended by 42 (names) (ruled by the owner on 2026-11-05). Holds now: [IN_FORCE.md: 2. The specification: form, layers and conformance](../../../IN_FORCE.md#2-the-specification-form-layers-and-conformance), [IN_FORCE.md: 6. Conduits and access](../../../IN_FORCE.md#6-conduits-and-access), [IN_FORCE.md: 7. Operations, outcomes, revisions, records and observation](../../../IN_FORCE.md#7-operations-outcomes-revisions-records-and-observation), [IN_FORCE.md: 3. Vocabulary and the project's names](../../../IN_FORCE.md#3-vocabulary-and-the-projects-names), [IN_FORCE.md: 4. Documents, templates, instantiation and submission](../../../IN_FORCE.md#4-documents-templates-instantiation-and-submission), [IN_FORCE.md: 11. The implementation interface, providers, hooks and items](../../../IN_FORCE.md#11-the-implementation-interface-providers-hooks-and-items); [OPS-008, OPS-023](../../../spec/conventions/operations.yaml), [OBJ-024](../../../spec/promises/objects.yaml), [IFC-029](../../../spec/interface/implementation.yaml). This record says why.


Accepted by the owner in [ruling 39](../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

## Rounds

Counts are those of each round at its close, before its fixes; none was clean. The current
promise counts are in the [specification](../../../spec/README.md#contents).

| Round | Read and result | Record |
| --- | --- | --- |
| 1 | Three readers of `spec/`, about 170 problems by class after merging duplicates, about 200 items in the reports; 11 unsafe. | The decisions and follow-up below. |
| 2 | Four readers at `bdacd83`: 125 findings in `spec/`, 8 unsafe; 47 from the repository reader. Three readers started by editors are outside the 125. | [C0 record](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-2/README.md) |
| 3 | Four readers at `b1d3fb4`: 44 findings in `spec/`, 4 unsafe; 23 from the repository reader. All findings resolved under ruling 37. | [C0 record](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-3/README.md) |
| 4 | Four readers at `f583469`: 38 findings in `spec/`, 3 unsafe; 26 from the repository reader. Ruling 38 withdrew the two-clean-round gate; readiness is no unsafe finding in the latest round, with the model and vectors agreeing. | [C0 record](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/round-4/README.md) |
| 5 | Six readers: 119 findings merged into 111 items, 110 fixed and 1 rejected, 10 unsafe. Two verifiers reported 29 findings, all resolved, the last (V6) by the coordinator. | [Record](round-5/README.md), [decisions](round-5/DECISIONS.md), [integration and verification](round-5/log-fixes.md) |
| 6 | The owner’s cold read by Codex at `c4d7d33`: six findings, two wrong and safety-critical; all fixed. A 22-point design critique; recommendations accepted by ruling 44. | [Record](round-6/README.md), [decisions](round-6/DECISIONS.md) |

## What the readers concluded

- The rules that reject a bad document, the submission rules, revisions, the closing of a
  Conduit and bounded delegation trace end to end without a guess.
- The language layer cannot yet be built so that a second team would match it object for
  object. The concrete surface is missing: schemas, the way a reference is written, and the
  contents of the core package.
- Most contradictions come from earlier batches that were not rewritten after rulings 28 to 31
  changed what they rest on.
- Eleven findings are unsafe: an engine could keep every promise and still fail to apply a
  filter, attribute an onward request to the wrong caller, repeat an effect or let a delegate
  act outside its bound.

## How the findings are handled

| Class | Count, about | Handling |
| --- | --- | --- |
| Follows from a ruling already made | 115 | Fixed in the consolidation pass with no new ruling: duplicates merged, stale words removed, promises moved to the right layer, missing key names stated. Each is listed in the consolidation log when it is done. |
| Needs a decision | 37 decisions covering about 55 findings | Below, each with a recommendation. |
| A missing deliverable | 3 | The core package as normative documents, the JSON Schemas, and the manifest's file format. Decision G1. |

## Decisions

### Unsafe findings

| ID | Decision | Recommendation | Findings |
| --- | --- | --- | --- |
| U1 | Who applies a filter or a policy, and how it learns of it. Nothing on the wire names the Conduit or its restrictions. | The implementation applies it, as the accepted text says. Each request's `_meta` names the Conduit and carries every restriction that applies, as a requirement reference and its arguments. An implementation marked `plain` supports no requirement, so a Conduit that carries one cannot be opened to it. | R3-1 |
| U2 | How an onward request is tied to the call it serves. | The onward request's `_meta` names the invocation being served. A Resource or other instance that is not an actor must name one, or the request is refused. The restrictions of the served call are carried from it. | R3-2 |
| U3 | Model work started by code on behalf of an unmarked operation. | An onward request for an operation marked as starting model work is refused unless the invocation being served is itself marked, or the requester is an actor acting for itself. | R3-9 |
| U4 | A plain server chosen for an operation that deduplicates. A plain server never receives the operation identity. | A plain implementation never counts as deduplicating. After an unknown outcome nothing is sent again, and the outcome stays unknown until reconciled. | R3-3 |
| U5 | Running setup or retire again. | Two invariants: a step whose earlier outcome is unknown is not dispatched again unless the operations convention allows a new attempt; and a runtime the instance already holds is not started again. Everything else about a re-run joins entry DL1 of the implementation-defined list. | R3-4 |
| U6 | A Conduit is closed while a call through it is in flight. | The invocation runs to its end and its outcome is recorded. Its content is delivered only if the caller's access at that moment permits, as for a retained result. Onward requests it makes after the close are refused. | R3-5 |
| U7 | Whether a delegate's bound limits the policies it may apply or remove. | Yes. A policy may be applied or removed only where the actors and targets it applies to are within the actor's bound. No policy can be applied to the root Interface. | R3-6 |
| U8 | Passing a right on. | A holder of a right to open may open and close Conduits to `access` for others whose bound is within its own: every target or Pool named, and every operation, is one its own bound names. Closing the delegator's right does not close what it passed on. | R3-7, R3-32, R1-2.19 |
| U9 | Whether purging removes what deduplication relies on. | A purge keeps the identity and outcome of every invocation. | R3-10 |
| U10 | How a connection is known to be `root`, another Interface or an Agent. | A new entry on the implementation-defined list, with one invariant: the engine establishes which instance a connection belongs to before any request, and no consumer can present as another. | R3-11 |
| U11 | A package whose archive does not match the digest its manifest gives. | Not treated as a decision: it is unresolved, exactly as a version mismatch is. | R3-8 |

### Requests and outcomes

| ID | Decision | Recommendation | Findings |
| --- | --- | --- | --- |
| Q1 | What "a new attempt" is. One promise says a repeat is never dispatched again, another that it is. | A repeat of the same identity and contents is the new attempt. It is dispatched again after a failed outcome, and after an unknown one only if the operation deduplicates. Otherwise the stored state is returned. | R1-2.4, R3-22 |
| Q2 | The scope of an operation identity. | One caller, one target and one identity. "Same contents" means the same operation and arguments. | R3-23 |
| Q3 | The outcome of an operation implemented by a graph when some steps succeeded and a required one did not. | Unknown, since part of the effect occurred. Succeeded only if every required step succeeded; failed only if no step had an effect. | R3-21 |
| Q4 | A control whose confirmed settings fail the schema, and a control with an unknown outcome that is later reconciled. | The first has the outcome unknown and stores nothing. For the second, the reconciler states the settings that took effect, and those are stored. | R1-2.10, R3-25 |
| Q5 | When several reasons apply to one request that is not dispatched. | One order: a request that cannot be read is invalid; otherwise refused, then invalid, then conflict, then unavailable. Nothing about the system is told to an actor that is not permitted. | R1-2.6 |
| Q6 | What is always recorded. The closed list omits requests that end invalid or conflict. | Every request and how it ended, dispatched or not, joins the list. | R1-2.2 |
| Q7 | How a consumer supplies an operation identity, learns how its request ended, and fetches its own result after its Conduit is closed. | The consumer's side mirrors the implementation's: the same `_meta` keys for the identity and the ending. A tool for retrieving one's own results is always presented and needs no Conduit. | R3-13, R1-2.24 |

### Authority

| ID | Decision | Recommendation | Findings |
| --- | --- | --- | --- |
| A1 | Which Conduit permits creating a Resource. A Pool's own entries are created with it, when no Conduit to it can exist. | Registering and unregistering always need `management`. Registering a Resource into a Pool that already exists also needs a Conduit to that Pool. A Pool created in the same submission needs nothing more. Setting up a Pool's entries during the Pool's setup needs the actor's Conduit to the Pool. | R1-2.5, R3-19 |
| A2 | Reconciling, and instances retired because their location was declared lost. | Reconciling is authorized against the same target as managing the instance. An instance on a lost location can be unregistered without reconciliation; its status says its shutdown could not be confirmed. | R3-30 |
| A3 | A replacement for a closed declared Conduit: one request or two. | Two, like any Conduit: it is registered through `management`, then opened through `access`. A later setup may open it, because an explicit request created it. | R3-29 |

### The language

| ID | Decision | Recommendation | Findings |
| --- | --- | --- | --- |
| G1 | The missing concrete surface. | Write three things as normative parts of `spec/`: the core package as documents (the operations of `management`, `access` and a Pool; the bound and shared-host requirements; the reference schemas; the first-start templates and types); the JSON Schemas of every document; and the manifest's file name and keys. They are brought to the owner for review as one batch. | R3-12, R3-50, R2-S11 to S25 |
| G2 | How a reference is written and recognized. | `{kind, name}`, or the bare name where the schema fixes the kind. A value is a reference only where its schema is one of the core package's reference schemas. Anything else is data and is never rewritten. | R3-16, R2-S14 |
| G3 | Names of nested instances, and referring to a Pool's entry from outside the Pool. | Nested instances get generated names like any unnamed instance. A document outside the Pool refers to an entry as `{kind: Pool, name: <pool>, entry: <entry>}`, which resolves to the entry's Resource that is not retired. | R3-15, R2-S10, R2-S23 |
| G4 | On which instance a step's operation runs. | Without a further key, the step invokes the implementation selected for its own instance. With `conduit`, naming one of the template's declared Conduits, it is a request through that Conduit to its target. | R3-17, R2-S16 |
| G5 | How a requirement on placement is evaluated, and whether a location's facts are declared or observed. | A location's facts are stored configuration of the location, changed only by a control. A requirement on placement is a schema those facts must satisfy, and an absent fact fails it. Placement stays answerable from documents. | R3-18, R1-4.6 |
| G6 | Whether a Conduit may be opened to a target that is not yet ready. | Yes, to any target that is not retired. A request to a target that is not ready is not dispatched and reports unavailable. Setup then does not depend on the order in which objects are set up. | R3-20 |
| G7 | A template replaced or unregistered while instances exist. | Instances keep what they resolved. A replacement nested instance is made from the declaration as its parent instance holds it. A template with instances may be unregistered. | R3-26 |
| G8 | Which version's contract governs when the two ends of a Conduit were pinned to different versions. | The target's. Within one contract version a later release may add optional things and may not remove or change any; a request for an operation the target's version lacks reports unavailable. | R3-27 |
| G9 | A Conduit that carries a filter its target's implementation does not support. | Rejected when the Conduit is instantiated, like a capability the target does not offer, since the manifest answers it. The open-time rule stays for a policy that arrives later. | R1-2.17 |
| G10 | Contracts in a project that is not published, and referring to something in the same package. | A project may hold contracts, and submitting it makes them available to its own documents. `#name` refers to something in the same project or package. | R1-2.13, R2-T35, R3-46 |
| G11 | Controls. They are bare names in a type, so a Conduit cannot list one. | A control is an Operation contract that also states the settings it changes. A type lists its controls by reference, a Conduit lists one like any operation, and a template narrows one by its inputs. | R3-24, R2-S2, R2-S21 |
| G12 | Retirement details. | An instance that was never set up does not run its `retire` graph. Its own outbound Conduits close after its `retire` graph and stops finish; inbound ones close at once. A Pool may be retired only when the shutdown of every Resource it owns is confirmed. | R1-2.9, R1-2.11, R3-42 |
| G13 | When the shared-host requirement is checked. | When the Conduit is opened and on each request, against the stored facts. A request made while it does not hold reports unavailable. | R3-34 |

### Words

| ID | Decision | Recommendation | Findings |
| --- | --- | --- | --- |
| W1 | "Registered" means both a lifecycle state and "exists". | Keep it for the state. Say "exists" for the other. | R2-T9, R1-3.8 |
| W2 | "Record" means both a log entry and the state kept for an operation identity. | A record is a log entry, as ruled. The operations convention speaks of "the invocation's state", which is read from its records. | R2-T10, R1-2.1 |
| W3 | "Create", "instantiate" and "register". | One operation, register. Registering an instance document instantiates its template. "Create" is not used for an operation. | R2-T25 |
| W4 | "Rejected". | A rejected document makes its registration end not dispatched, with the reason invalid; a clash with an existing name reports conflict. | R2-T26 |

## Follow-up decisions — accepted

The editors left these untouched because the 37 accepted decisions do not settle them. The owner
accepted every one as recommended except F3 (plan-audit ruling 33), and confirmed the editors'
readings listed at the end.

**F3 as ruled.** The owner does not hold a domain, so the names come from this repository,
`github.com/shrav-k/conduit`, until there is one. This also settles decision X1 of the
[last batch](../schemas-and-interface/README.md):

| Name | Value | Existing practice |
| --- | --- | --- |
| The core package's address | `github.com/shrav-k/conduit/core` | A Go module path |
| A document's `apiVersion` | `github.com/shrav-k/conduit/v0` | — |
| The prefix of Conduit's `_meta` keys | `io.github.shrav-k.conduit/` | The Model Context Protocol registry's `io.github.<user>` namespace for publishers verified through GitHub |

Moving to a domain later changes all three. The core package would take a new address, which
every document that refers to it would have to follow, so it is best done before a first
release. At the owner's request (plan-audit ruling 34) the change is one command: the names are
defined once in [spec/names.yaml](../../../spec/names.yaml), and `python3 tools/set_names.py`
rewrites every occurrence and checks that none is left behind.

### The wire

| ID | Decision | Recommendation |
| --- | --- | --- |
| F1 | An implementation that applies a restriction has no way to say "a restriction forbids this" or "I cannot apply it". The caller would see failed. | An implementation that carries Conduit's keys may answer with a reason, refused or unavailable, in place of an outcome, and must have performed nothing. The caller is told that reason. |
| F2 | Which instance a consumer is. The baseline revision says identity must not rest on a connection, only on what each request presents. | The engine establishes the instance for each request from what that request presents. How is entry DL13. |
| F3 | The prefix of Conduit's `_meta` keys. The protocol's own keys use reverse-DNS, `io.modelcontextprotocol/`. | Not accepted as recommended; see the ruling above. |
| F4 | After a purge a repeat cannot be told from a request with different contents. | A purge also keeps a digest of each invocation's contents. |
| F5 | Tool names can exceed the protocol's 128 characters or collide: components of a declared composite Conduit, and a rename onto a declared Conduit's name. | A tool is named by the Conduit's local name, then each component name, then the operation's export name, joined by dots. A Conduit whose tool names would exceed 128 characters or collide is not instantiated, and such a rename reports conflict. An export name is at most 63 characters. |
| F6 | A plain server that answers `input_required`. | The outcome is unknown. |
| F7 | A request that names, in `serving`, an invocation the sender is not serving. | Refused. Only the instance an invocation was dispatched to may name it, and only while it is in progress. |
| F8 | Whether the `retire` graph may start model work. | Never, as for setup. |
| F9 | Work arranged for later, after the arranging Conduit is closed and the actor gets a new one. | It is bound to the Conduit that arranged it and never revives. |

### Changes and controls

| ID | Decision | Recommendation |
| --- | --- | --- |
| F10 | A control whose revision was current when it was dispatched and stale when its result is stored. | It cannot happen: changes to one instance are applied one at a time. Entry DL9 no longer lets several controls run at once on one instance. |
| F11 | Reconciling a control. | The reconciliation names the instance's current revision, and the settings it states must satisfy the schema, or it reports invalid. |

### Authority

| ID | Decision | Recommendation |
| --- | --- | --- |
| F12 | Whether a right passed on may list an operation of `access` that the delegator's own Conduit does not list. | No. |
| F13 | What "within a bound" covers. | A bound names Pools and individual targets. Naming a Pool covers the Pool itself and every Resource it owns. A new bound is within an old one when everything it names is covered by the old one. |
| F14 | Targets that authorize three operations nothing names. | Retiring a Pool with its Resources: the Pool. Operations reserved to `root`, and installing a package: `management`. |

### Placement

| ID | Decision | Recommendation |
| --- | --- | --- |
| F15 | An entry whose placement no Pool input supplies. | The default-placement rules apply, as for any instance. |
| F16 | An ordinary request to retire a location that has instances, and placing an instance on a location that is registered but not ready. | The first reports conflict. The second is allowed; the instance cannot be set up until the location is ready. |

### Documents

| ID | Decision | Recommendation |
| --- | --- | --- |
| F17 | The names of an Operation's inputs and outputs. A plain server's argument names use camelCase or underscores, which the object-name rule rejects. | Any non-empty string. Every other author-chosen name follows the object-name rule. |
| F18 | Which places in a Conduit template an input may fill. | Its `operations`, each still required to be in the capability; a requirement's and a component's `arguments`. Never `capability`, and never the `from` or `optional` of a nested instantiation. |
| F19 | `conduit` on a step. | Rejected on a step that names a runtime. A step with `conduit` depends on that declared Conduit being open. |
| F20 | Which kinds a Conduit's `consumer` and `target` accept. | Consumer: Interface, Agent or Resource. Target: Interface, Agent, Resource or Pool. |
| F21 | Whether a schema can be exported, as the core package's reference schemas must be. | A fifth contract kind, `Schema`, whose `spec` is the schema. |
| F22 | Words: "resolving" against "registration", and "source" with three meanings. | Resolving is the function from documents to objects or diagnostics; registration is the operation that runs it and stores the result. `source` stays for a value reference; a manifest says `fetch` for where a package comes from; where a record or observation was made is its origin. |
| F23 | A `default` on a property of an object inside `items`. | Allowed, as Kubernetes allows it. |
| F24 | Contracts submitted with an unpublished project: how long they stay available. | The engine keeps them as an unpublished package identified by a digest of its documents, and a registered template records it like any package. |
| F25 | The name of a composite Conduit's instance, since no instance represents the composition. | Each component Conduit is named by the given or generated name, a hyphen and the component's name. |
| F26 | File formats left open: several documents in JSON, the host part of an address, and the archive. | A JSON file holds one document or an array of documents. The host is a lowercase DNS name with no port. The archive is a gzip-compressed tar file. These go into decision G1's deliverable. |

### Readings the editors made, for confirmation

- The rule that rejects an unsupported requirement on a Conduit applies to filters only. The
  engine checks the shared-host requirement itself, so no implementation has to claim it.
- A request to open a Conduit while the shared-host requirement does not hold reports
  unavailable.
- An instance that was never set up, on a location declared lost, has its shutdown confirmed.
- The list of restrictions sent with a request is always present, and empty when none applies.
- A record also holds the engine's ID for the invocation.

## Follow-up pass

Done on 2026-11-05 by two editors: [access, lifecycle and the wire](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-ab.md), and
[documents and packages](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/reaudit/log-followups-c.md). They changed 85 promises and added 14. The
coordinator then made three further changes:

| Change | Why | Promise |
| --- | --- | --- |
| The rule that a right to open is passed on only as it is or narrower now also covers a Conduit to `access` that an actor opens for itself. | An editor found that an actor could otherwise open itself a wider right. This follows from decision U8 and from the preservation test that delegation only narrows. | ACC-030, ACC-006 |
| A Pool template declares no Conduits. | A Pool is never a consumer (decision F20, and ACC-028). | TPL-001 |
| The names belong to a version of the specification. | Plan-audit ruling 35. | OBJ-024, IFC-029 |

After this pass the specification had 355 promises. It has more now; the
[contents table](../../../spec/README.md#contents) gives the count. Points the editors left
open, because no ruling settled them, were carried into the batch that wrote the core package,
the schemas and the manifest format. Its [record](../core-and-schemas/README.md) lists each
with a recommendation and with what the owner ruled:

- how the digest of an unpublished project is computed, and how a purge's digest of contents is;
- whether `#name` in a `$ref` reaches a Schema contract in the same project;
- the places an input's destination may reach that are neither allowed nor forbidden yet;
- which reason a rejected installation reports;
- an address's host part: trailing dots, numeric hosts and internationalized names;
- a component Conduit's name that would exceed 63 characters, and a rename that would push a
  tool name past 128;
- a result that carries both an outcome and a reason, and a repeat of a request that ended with
  a reason;
- how an implementation's answer names the restriction that forbade a request;
- "in progress" and "withheld" outside the tool that retrieves results;
- whether setting up and retiring count as changes that are applied one at a time.

### Disagreements found while the definitions were rewritten

The editor who brought `definitions.md` and `README.md` into line reported five pairs of
promises that disagreed. Each is settled by a ruling already made, so each was corrected without
a new one.

| Pair | Problem | Correction |
| --- | --- | --- |
| CON-024 with POOL-013 and POOL-015 | A request to a target that is not ready reports unavailable, so a Pool that is still being set up could never set up its own entries. | CON-024 excepts the operations that manage an object, which a Pool takes while its state is registered. |
| ACC-012 with CON-008 | "Never closed" for a Conduit whose consumer is `root`, against closing a Conduit whose target is retired. | ACC-012 says never closed at an actor's request. |
| TPL-004 with GRA-024 | A step that is a request through a Conduit was still required to have an implementation. | TPL-004 excepts a step that carries `conduit`. |
| LIF-005 with LIF-021 | Retiring a Pool with its Resources left the Pool's state "as it was" although a dispatched retire request takes effect at once. | LIF-021 says the Pool's own retire request is dispatched only once every Resource's shutdown is confirmed. |
| INS-002 with NAM-013 and FST-005 | "A template that exists" excluded a template that a package exports. | INS-002 names both. |

## Choices that set behavior

Copied from the C0 logs, one line per choice, with the source log and item. Adjacent
editorial reasons are kept where they explain the reading. These describe the choices then;
later decisions and promises govern.

- **log-group-a, choice 1.** CON-022 says "filter", which is the word of decision G9 and of the removed ACC-013, where the brief for this pass said "requirement". With "requirement", a Conduit that carries the shared-host requirement would need its target's implementation to list that requirement in `supports`, although the engine checks it under PLC-013.
- **log-group-a, choice 2.** PLC-013 reports unavailable for the request to open as well as for a request through the Conduit. Decision G13 names the reason only for "a request made while it does not hold".
- **log-group-a, choice 3.** ACC-006 and ACC-012 treat ACC-030 as the whole condition on the bound for a Conduit to `access` whose consumer is another instance, so the actor's bound need not also cover `access` as a target (R3-7, first question). A Conduit to `access` that an actor opens for itself stays under ACC-006.
- **log-group-a, choice 4.** An instance that was never set up and is on a location declared lost reports its shutdown as confirmed (LIF-025), not unknown (PLC-017). The two earlier sentences disagreed for this case and no finding raised it.
- **log-group-a, choice 5.** LIF-012 now says a template has no lifecycle state, where it said "no other state" than registered. Its layer became `convention` because its observable is how a request ends.
- **log-group-a, choice 6.** Under decision A2 the lost location itself is unregistered like the instances on it, once none is placed on it, because a location is its own placement.
- **log-group-b, choice 1.** `conduit.dev/restrictions` is always present and is an empty list when no restriction applies. Each item has the keys `requirement` and `arguments`, as an applied requirement has in CON-018.
- **log-group-b, choice 2.** A new attempt is an attempt of the same invocation, so it carries the same operation identity (OPS-012) and by implication the same `conduit.dev/invocation`. No promise says the second in so many words.
- **log-group-b, choice 3.** "No step had an effect" (OPS-022) is read as: every step that was dispatched either failed or is of an operation whose contract states no `effect`.
- **log-group-b, choice 4.** IFC-021 refuses an instance that is not an actor when it names no invocation "it serves". An ID the engine never dispatched to that instance is read as naming none.
- **log-group-b, choice 5.** A record holds the engine's ID for the invocation (REC-002), since the state of an invocation with no operation identity can be read from its records in no other way.
- **log-group-b, choice 6.** The retrieval of the caller's own results is by operation identity only (OPS-016, IFC-024), as decision Q7 says.
- **log-group-c, choice 1.** **OBJ-011 and SUB-007 kept their IDs as the rename and replace promises.** R1-1.10 says to reduce each to its operation, and the rejection each also stated is OBJ-002. No new ID was needed for split 5.1.
- **log-group-c, choice 2.** **Layers changed in place under rule 3 beyond the two the brief named:** NAM-016 and NAM-017 to `convention` (R1-4.5), and PKG-016 to `interface` (R1-4.9). Group B's log reads R1-4.9 as a move and left it; if that reading is preferred, set PKG-016 back to `convention`.
- **log-group-c, choice 3.** **Trims not on the brief's list but in reader 1 section 1:** R1-1.2 on INS-016 and SUB-006, and R1-2.16 on INS-016 (settled by CON-003).
- **log-group-c, choice 4.** **A template's `controls` map is keyed by qualified reference** (decision G11), so it is not among the maps of author-chosen names in OBJ-022, though the brief's list had it there.
- **log-group-c, choice 5.** **The U5 rule is inside TPL-007**, not a new promise, because the decision table and OPS-021 both point at TPL-007.
- **log-group-c, choice 6.** **OBJ-014 holds all three shapes in one promise.** It overlaps POOL-012 and CON-020, which group A owns.
- **log-followups-ab, choice 1.** **F1 is written as a `may` and a `must`.** IFC-026 (`may`) is the permission to give a reason in place of an outcome, with its two cases and two values. IFC-019 (`must`) says an implementation that is sent a restriction which forbids the request, or which it cannot apply, performs nothing and answers so, because ACC-008 and ACC-014 promise the caller refused and unavailable. IFC-027 is the engine's side.
- **log-followups-ab, choice 2.** **The local name of a Conduit instantiated on its own is its own name** (IFC-012), as the brief says. For a component Conduit of a composite that was instantiated on its own, the own name is the one F25 gives, so the tool name repeats the component: `review-read.read.directory.read`. The other reading, in which the local name is the name of the instance document (`review.read.directory.read`), would need the engine to keep that name after a rename. IFC-012's observable states the first reading in an example.
- **log-followups-ab, choice 3.** **"Another tool presented to the same consumer"** (IFC-014, CON-026) counts the tools of the consumer's other Conduits that are not closed, whether or not they are open yet, and the tool of IFC-024. A closed Conduit is never presented again, and a replacement for a closed declared Conduit must be able to take its local name.
- **log-followups-ab, choice 4.** **IFC-014 is narrower than before in one case.** It used to reject a Conduit instantiated on its own under the name of one of its consumer's declared Conduits, whatever they listed. It now rejects it only where the two would have a tool name in common, which is what F5 states.
- **log-followups-ab, choice 5.** **F25 applied again for a composite inside a composite** (CON-003): the name formed for the inner composite's component takes the place of the given or generated name.
- **log-followups-ab, choice 6.** **F12 is applied to opening only.** Closing a Conduit to `access` of another consumer needs only that its bound is within the actor's own, as before.
- **log-followups-ab, choice 7.** **F11's "the schema"** is read as in REV-008: the stored configuration with the stated settings applied must satisfy its schema. The narrowing a template adds to a control's input (TPL-011) is not applied to a reconciliation.
- **log-followups-ab, choice 8.** **F11's revision applies to every reconciliation of a control,** also one that records that the effect did not occur.
- **log-followups-ab, choice 9.** **F10's "change"** is a request that REV-002 checks against a revision. REV-012 keeps, for any object, that at most one of several requests naming the same revision is stored.
- **log-followups-ab, choice 10.** **PLC-011 says "other instances".** A location is its own placement, so without the word no location could ever be retired by an ordinary request.
- **log-followups-ab, choice 11.** **PLC-019 excepts the location itself,** which is its own placement and would otherwise never become ready.
- **log-followups-ab, choice 12.** **POOL-016 covers an optional input that was not supplied,** as well as a template with no input for the entry's placement.
- **log-followups-ab, choice 13.** **IFC-008's result states the outcome failed under IFC-009,** since IFC-009 gives unknown to a result with no outcome key and IFC-008 promises failed.
- **log-followups-c, choice 1.** **OBJ-023 has the layer `convention`.** It states how a request ends, as TPL-011, LIF-012 and REV-011 do. OBJ-002, which states conflict for the same operation, has the layer `language` and was left as it is.
- **log-followups-c, choice 2.** **OBJ-013 states a consequence of F20.** Since `consumer` and `target` each accept more than one kind, OBJ-006 already requires each to be written as `{kind, name}`; a bare name such as `consumer: root` is rejected. The observable now says so. OBJ-013 states the kinds that are accepted and leaves the rejection of any other kind to `conduits.yaml`.
- **log-followups-c, choice 3.** **INS-012 says "is, or is inside"** where it said "inside". `capability` is a single value, so "inside `capability`" would have forbidden nothing, and the same was already true of `type`.
- **log-followups-c, choice 4.** **F18's check is a new promise, INS-023,** and not part of INS-012. CON-017 makes the check on a list the template writes; INS-023 makes it on the list an argument or a default gives.
- **log-followups-c, choice 5.** **F19's dependency is in GRA-024,** not a new promise. Its observable speaks of the setup graph only, because GRA-022 gives a declared Conduit a step only there. See item 4 below.
- **log-followups-c, choice 6.** **SCH-015 also rejects a qualified reference to an export that is not a Schema contract,** in the way it rejects any other form of `$ref`. The ruling says such a reference "resolves to an exported `Schema`"; the rejection is my reading of what follows when it does not.
- **log-followups-c, choice 7.** **PKG-022 is written in the form of PKG-001:** the package is rejected when it is installed. The ruling states only what the archive is.
- **log-followups-c, choice 8.** **PKG-005 and OBJ-021 were touched lightly** so that F24 and F21 read the same in every place that lists what an instance holds or names the reference schemas.
- **log-followups-c, choice 9.** **F22 was applied as a change of subject only.** Every "Registration returns a diagnostic" became "Resolving returns a diagnostic". "Installation returns a diagnostic" was left, since the ruling does not speak of installing. Sentences about what exists afterwards, such as OBJ-008's and PKG-006's "the template exists after registration", already used "registration" for the stored effect and are unchanged.
- **round-5/log-fix-F3, coordinator item 1.** M41 opens one case that no promise settles. Before it, only an instance whose node was declared lost could be unregistered, and such a node represents no machine, so PLC-023 never held back LIF-036's stop. Now a retired instance on a node that is not lost, whose machine is reported unreachable or omitted, can be unregistered; PLC-023 then does not ask for the stop and reports unavailable. LIF-036 removes the instance "once the stop has ended, or has not been dispatched because the runtime provider is not ready or" lacks Conduit's extension, "whatever came of it", and does not name this case. One engine removes the instance at once, as for a runtime provider that is not ready; another reports unavailable and keeps it until the machine is reachable or reported gone. Not changed here: either answer is a decision.
- **round-5/log-fix-F3, coordinator item 2.** LIF-018 is left as it is, as M41's reason leaves statements that only describe the lost-node case. Its `then` names a reconciliation and a further retirement as what ends an unknown shutdown, and its observable names a request to unregister only for an instance whose node was declared lost; a request to unregister now removes any retired instance, under LIF-017.
- **round-5/log-fix-F4, M75.** IFC-012 and IFC-025 observables as decided. IFC-013 observable also: "and is answered as IFC-012 states", dropping ", with the protocol's error for an unknown tool", which at a revision whose answer to an unknown tool is a tool result would contradict IFC-012 as M75 words it.
- **round-5/log-fix-F4, M77.** IFC-036 `when` and `then` as decided. IFC-032 `when` also excepts a request that the protocol has a server reject "with an error or an HTTP status of its own", so that a request whose `Origin` the engine does not accept, and that presents no credential, is answered 403 under IFC-036 and not 401 under IFC-032. DL12: subject and the decided sentence on `Origin`.
- **round-5/log-fix-F5, M20.** `list-machines.v1` description: "trimmed and lowercased, of at most 63 characters, and at most once". `outputSchema` of `list-machines.v1` and of `place.v1`: `hostname` has `maxLength: 63` beside `minLength: 1`. `run.v1`, `stop.v1` and `inspect-run.v1` take a hostname that the engine had reported to it, and were not bounded, as the decision names only the two.
- **round-5/log-fix-F5, M80.** `register.v1` description as decided: "a machine reported unreachable or omitted", and "for want of a machine it is rejected where no node matches, and where the provider names none, unless a node other than `local` that is not retired and matches represents a machine reported unreachable or omitted". `register-replacement.v1` description also gains "or omitted" in its clause on unavailable, which left it out in the same way; that is not in the work order.
- **round-5/log-fix-F6, M90.** Rows grant, declaration, policy, facts and machines provider as decided, with one change to grant: "a grant by which a client obtains a token from an authorization server, as a consumer does from the one that DL13 states", so that the row still holds if IFC-041 comes to name a grant of its own. Row origin: area F4 has already removed IFC-037's sentence "for its origin on HTTP" (M68), so the row names instead the sense that IFC-036 and DL12 now use, the `Origin` header.

## What became of the logs’ open items

- The first group A’s seven decision requests became F9 and F12 to F16 above; its six readings are carried in “Choices that set behavior”. The excluded moves and splits were the pass’s limits, not new behavior.
- The first group B’s requests became F1 to F11 above; the core package and schemas settled the error fields, result retrieval and rejected-document form. The later core-and-schemas record settles the follow-up requests on both result keys, repeats answered with a reason, restriction identifiers and setup/retirement serialization. Its remaining naming and split notes were editorial limits.
- The first group C’s requests became F17 to F26 above or the core package, manifest and schemas. NAM-024/NAM-025 retain the contract author’s compatibility obligation; no automatic proof of compatibility was promised. The kept [core-and-schemas decisions](../core-and-schemas/README.md) hold the detailed shapes, digests, host grammar and destination rules.
- The follow-up AB log’s self-widening and Pool-consumer defects were corrected in the follow-up pass above. Its remaining requests, and follow-up C’s digest, `$ref`, destination, installation, naming and archive requests, were carried to the [core-and-schemas record](../core-and-schemas/README.md); the five disagreements corrected above close the definition pass. Graphs and composite Conduits were later removed by ruling 41.
- Round 5’s F1 to F7 logs route unmade changes to other areas; the integration applied them. F3’s PLC-023/LIF-036 case is settled with three options in the [kept integration](round-5/log-fixes.md#the-choice-of-this-pass). The verification and V6 settlement remain there too.
- Round 6’s logs carry their residuals to the [integration](round-6/log-integration.md#left-for-a-decision), which keeps each item’s current status. The pointer-comparison reading (specdocs choice 7) is item 3. D06 and D12 were applied; IFC-047 and the severity reading are in the [decisions](round-6/DECISIONS.md#for-the-owner), accepted by rulings 44 and 58. The round’s final counts are in the specification’s contents table.
