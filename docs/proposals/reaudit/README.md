# Re-audit of the whole specification — round 1, decisions accepted

Status: findings recorded on 2026-11-05 (plan-audit action A59). The owner accepted every
decision below as recommended (plan-audit ruling 32). They are applied by
[consolidation pass 1](CONSOLIDATION.md).

This page is the record of round 1. Round 2, with four readers, is done and has
[its own record](round-2/README.md), as does [round 3](round-3/README.md), and round 4 is [in progress](round-4/README.md). Counts of promises on this page are as they stood when
each pass ended; the current counts are in the
[contents table of the specification](../../../spec/README.md#contents).

This is the record of a ruling. Where it and the specification differ, the specification
governs. Rounds 2 to 4 changed some of what ruling 32 accepted, by decisions of the coordinator
that the owner has not ruled on; the tracker lists them under
[later amendments](../../AUDIT.md#later-amendments).

After the last batch of rulings (ruling 31) the owner asked for the promises to be consolidated
and put back through the audit. Three cold readers each read all of `spec/`, and nothing else,
as an engine builder would. Their reports are kept verbatim:

| Reader | Looked for | Findings |
| --- | --- | --- |
| [1](reader-1-duplicates-and-contradictions.md) | Duplicate, contradictory, stale and misplaced promises | About 80: 24 groups of duplicates, 25 contradictions, 12 stale wordings, 10 misplaced, 10 to split |
| [2](reader-2-terms-and-shapes.md) | Undefined or double-meaning terms, and the keys of every document | 35 on terms, 26 on shapes, 8 on standards, and a reconstructed key table |
| [3](reader-3-undetermined-behavior.md) | Anything two conforming engines could disagree on, by walking ten scenarios | 50: 11 unsafe, 26 divergent, 13 minor |

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

## What came next

These four steps were written when the findings were recorded. Each now has its state.

1. The owner rules on the decisions above. Done: plan-audit ruling 32.
2. One consolidation pass applies those rulings together with the fixes that need none. Done:
   [consolidation pass 1](#consolidation-pass-1), then the [follow-up pass](#follow-up-pass).
3. Decision G1's deliverables are written and reviewed. Done: the core package, the schemas
   and the manifest's format, accepted with five changes by plan-audit ruling 37
   ([record](../core-and-schemas/README.md)).
4. A second round of cold readers. Done: [round 2](round-2/README.md), which was not clean.
   [Round 3](round-3/README.md) is done and was not clean either, and
   [round 4](round-4/README.md) is in progress. The done-when of action A59 asks for two
   rounds in a row with no finding. Whether the readiness condition G7 is judged on these
   rounds is [for the owner](../../AUDIT.md#waiting-for-the-owner).

## Consolidation pass 1

Done on 2026-11-05 by three editors working on disjoint groups of files, each from the
[instructions](CONSOLIDATION.md). Their logs list every promise changed, removed or added:
[group A](log-group-a.md), [group B](log-group-b.md), [group C](log-group-c.md).

| | Before | Changed | Removed | Added | After |
| --- | --- | --- | --- | --- | --- |
| Promises | 312 | 212 | 21 | 48 | 339 |
| Implementation-defined entries | 12 | 4 | 0 | 1 | 13 |

After the pass every promise file parses, no ID is used twice, and no promise cites an ID that
no longer exists. `definitions.md` has not yet been brought into line; that waits for the
follow-up decisions below, so that it is rewritten once.

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

Done on 2026-11-05 by two editors: [access, lifecycle and the wire](log-followups-ab.md), and
[documents and packages](log-followups-c.md). They changed 85 promises and added 14. The
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
