# Round 2: decisions for the fix pass

The three readers of `spec/` reported 125 findings: [reader 1](reader-1.md), 36;
[reader 2](reader-2.md), 49; [reader 3](reader-3.md), 40, of which that reader marked 7 as
unsafe. The coordinator classed one finding of reader 1, R1-1.1, as unsafe as well, which
makes the 8 that the tracker gives. The owner ruled
that findings are resolved without a ruling on each (plan-audit ruling 37). This file settles
every finding a reader marked as needing a decision, and the unsafe ones. Every other finding
is fixed as its reader proposes. Each row is the coordinator's choice and can be struck by the
owner.

Accepted by the owner in [ruling 39](../../ruling-39/README.md), with every other
logged decision of the coordinator and choice of an editor.

The rules for editing promises and the vocabulary are in
[the consolidation instructions](../CONSOLIDATION.md).

## Unsafe findings (reader 3)

| Finding | Decision |
| --- | --- |
| R3-1 Filters skip graph steps | A step without `conduit` carries the restrictions of the invocation its graph is serving, and the implementation it invokes applies them as IFC-019 states. |
| R3-2 A retried step has no identity | The engine gives every step an operation identity derived from the invocation's ID and the step's name, and sends it. A new attempt of the step carries the same one. |
| R3-3 Retained results scoped to the caller alone | An operation identity, and the retrieval of a result, are scoped to the caller, the target and the instance that sent the request. |
| R3-4 No test for "access at that moment" | A retained result is bound to the Conduit its invocation was dispatched through. Its content is delivered only while that Conduit is open, and never under another one. |
| R3-5 Nested setup steps use the caller's rights | The steps of a setup are authorized exactly as the request to set up was: by the Conduits of the instance that sent it. For an onward request that is the sending instance, not the caller it serves. |
| R3-6 Closing stops one level only | The rule holds for every invocation that serves, directly or through others, one that was dispatched through the closed Conduit. |
| R3-7 Reconciling a management operation | Reconciling is authorized against the target that authorized the request being reconciled. `access-control.v1` therefore includes `reconcile.v1`. Recording that an unknown setup or open occurred makes the instance ready or the Conduit open; that a retirement or close occurred confirms the shutdown; that it did not occur gives the outcome failed. |
| R1-1.1 An unbounded right passes the "within" test | A Conduit to `access` that applies no bound is unlimited, and only an actor whose own Conduit to `access` applies no bound may open or close one. |

## Decisions on the other findings

| Finding | Decision |
| --- | --- |
| R1-2.1, R2-T1, R1-1.4 The setup and retire operations | An instance is set up by one invocation of `set-up.v1` and retired by one of `retire.v1`, requested of the target ACC-033 names, with the instance as an input. The engine implements both. A template's `setup` and `retire` graphs are the steps the engine runs for that instance; they are not implementations selected in the `implementation` map and have no inputs of their own. "The setup operation" and "the retire operation" mean these invocations. |
| R1-1.2 A rename and the revision | A rename advances the revision, like any stored change. |
| R1-1.3 A Conduit closed without a request | A Conduit's shutdown is confirmed when it is closed, however it was closed. It holds nothing to shut down. |
| R1-1.5 Retrieving under an unknown identity | An identity under which the sender has no invocation reports refused. OPS-015 becomes that rule. |
| R1-1.7 "Names a location" | PLC-005 covers placement only: a template that gives a placement, in an entry or as the default of an input whose destination is a placement, is rejected. A template may refer to a location in any other way. |
| R1-2.2, R2-C2 Replacing a component | A component of a composite that was instantiated by its own document cannot be replaced; a new Conduit is instantiated by a new document. A replacement asked for while the instance is not retired reports conflict, for a declared Conduit and a component as for an entry. |
| R1-2.3 A bound applied twice | A template that applies `bound.v1` twice is rejected when it is registered. A Conduit whose target is not `access` and that applies it is not instantiated. |
| R1-2.5, R2-C1 Every Pool offers its capability | A Type of the family Pool that does not offer `pool-management.v1` is rejected when its package is installed. |
| R1-2.6, R2-C4 Policies and bounds of unpublished contracts | A policy and a bound name published contracts only. |
| R1-3.1 Work that runs later | Any operation whose contract says so may arrange later work. A start of model work is arranged only by an operation marked as starting it. Both are bound to the Conduit through which they were arranged; once it is closed no start is dispatched, and the requests of work already started are refused. IMP-006 keeps the model-work half and ACC-019 the access half. |
| R1-5.2 A reason in a language promise | OBJ-002 keeps the rejection. A new convention promise says that a registration rejected for a clash of names reports conflict. |
| R1-1.11 An input with a default | Any input that states a default may be left out, whether or not it is marked optional. This holds for a template's, an Operation's and a Requirement's inputs, and OBJ-015 says so. |
| R2-T2 What a project is | The documents and the one manifest handed over together in one registration. A directory is one way to hand them over. |
| R2-S1 The value of `from` | A bare name or `{kind, name}` for a template that exists, a qualified reference for one a package exports, or `#name` for one of the same project or package. A promise states it. `from` is not a selection; PKG-013 names it beside selections. |
| R2-S3 Runtimes of a type | An instance has the runtimes its type declares as well as its template's. A template that declares a runtime under a name its type declares is rejected. A step may name either. |
| R2-S4 A left-out `arguments` map | Present and empty, as an unwritten `config` is. |
| R2-S6 The recorded packages | The key is `packages`: a list of items with `address`, `version` and `digest`. The core package's item has no `digest`; an unpublished package's has `digest` alone. OBJ-004 names the key. |
| R2-S9 `.` and `..` in an address | A path segment starts with a letter or a digit. |
| R2-G1 The form of a resolved reference | Where a value is a reference, the instance stores `{kind, id}` in its place. Every reference schema accepts that form, which only the engine writes. An implementation receives it, a graph reads it, and inspecting shows it. |
| R2-G2 Which input sets which setting | A control's `changes` is a map from the name of an input to the pointer it sets. "The settings a control addressed" are those of the inputs it was sent. |
| R2-G3 A composed requirement | An implementation supports a composition when its `supports` lists the composition or every one of its constituents. On the wire a composition is sent as one item for each constituent, with its arguments bound, and `restriction` names a constituent. A composition narrows if any constituent does, and may be applied only where every constituent may. |

## The points known before the round

These are settled here too, so that the next round starts with none.

| Point | Decision |
| --- | --- |
| One `package` key cannot name two unpublished packages | Each `#name` on the wire is written with its package's digest: `<digest>#<name>`. The separate `package` key is removed. |
| The reason for retrieving a result once the target is unregistered | Refused, as for any identity under which the sender can reach no invocation. |
| An object's default against its properties' defaults | The object's default is checked after its own properties' defaults are filled in. |
| A pointer that selects nothing in a graph's `outputs` | The invocation's outcome is unknown if a step had an effect, and failed otherwise, as OPS-022 decides any graph. |
| What `obliges` requires | It names the ends whose implementations must state the requirement in `supports`. With none, the target's must. Inspecting a Conduit shows, under each requirement, the qualified reference of the implementation that applies it. |
| The `settings` input of a reconciled control | The same shape as the control's `settings` output. |
| A repeat's "same contents" and defaults | Judged after defaults are filled in. |
| An implementation that restores a runtime its type does not allow | The engine treats the handle as no longer held; the runtime is stopped under its `stop` operation and reported as not held. |
| The record of a request from no known instance | It holds no caller, and says that none could be established. |
| How a value is marked as secret | Left for the owner. |

## Who edits what

Editor P owns every promise file under `spec/promises/` except `manifest.yaml`,
`spec/conventions/`, `spec/interface/`, `spec/implementation-defined.yaml`,
`spec/schema-profile.md`, `spec/definitions.md` and `spec/README.md`.

Editor S owns `spec/core/`, `spec/schemas/`, `spec/promises/manifest.yaml` and
`tools/check_schemas.py`.

## The fix pass, and what it left

Done on 2026-11-06 by two editors: [promises](log-promises.md) and
[core package, schemas and checker](log-documents.md). 395 promises became 406, and the
implementation-defined list 15 items. The editors left the points below, which the coordinator
settles here in the same way. A closing pass applies them.

| Point | Decision |
| --- | --- |
| Which Conduit permits reconciling an operation that is not a management operation | An invocation of a management operation is reconciled through the target that authorized it. Any other invocation, of an ordinary operation or a control requested of an instance, is reconciled through the target that manages that instance under ACC-033: its Pool for a Resource, `management` otherwise. No type has to offer `reconcile.v1`; it is removed from `location-settings.v1` and from the example capability. |
| A filter that obliges only the consumer, against sending every filter to the target | The target's implementation must always state a requirement that narrows requests in `supports`, and is always sent it. `obliges` naming `consumer` adds that the consumer's selected implementation must state it too. |
| Naming the implementation that applies a requirement when it has no address | By qualified reference, or by `<digest>#<name>` for one of an unpublished package. |
| How an operation says that it arranges later work | An Operation contract may hold `arrangesLaterWork: true`. Without it the operation arranges none. |
| Renaming one of `root`'s two Conduits | Refused, as renaming a first-start object is. |
| The example that writes out the first-start objects, which a submission would be refused | It stays, valid against the schemas, with a header that says it shows what first start makes and that submitting it is rejected. |
| A consumer that lost a result cannot learn the engine's ID to reconcile | `retrieve-result.v1` also returns the engine's ID for the invocation. |
| What `optional` means on a runtime and on a graph's output | A runtime is no longer marked optional; a step that starts it can be. An optional output is one the graph may leave without a value. |
| What `actors` and `targets` of a policy do | A policy applies to a request whose caller is one of `actors`, when the list is given, and whose target is one of `targets`, when it is given. At least one of the two is given. |
| Retiring a Pool with its Resources | The Conduit need list only `retire-with-resources.v1`. |
| A re-exported operation in a bound | A bound that names an operation by either the re-export or its origin covers both. |
| A relative `fetch` or `command` when documents are handed over without a directory | Rejected. A relative one is resolved against the directory the manifest was read from. |
| "Plain" for the built-in Interface and Pool types | They are called basic. "Plain" keeps its one meaning, an implementation that knows nothing of Conduit. |
| Every other finding an editor did not apply for want of a decision | The closing pass takes the reader's own suggested fix, or where there is none the choice that changes least and adds no behavior, and logs it. |

## The closing pass, and what it left

Done on 2026-11-06 by one editor ([log](log-closing.md)). 406 promises became 410. The pass
listed fifteen points that needed a decision. The coordinator settles them here, under ruling
37, and a last pass applies them ([log](log-tail.md)). Each can be struck by the owner. The
choices the closing pass made itself are in its log and stand unless the owner strikes them.

| Point | Decision | Why |
| --- | --- | --- |
| 1. A Resource of a retired Pool cannot be inspected or reconciled | Where the Pool that owns a Resource is retired, the requests that ACC-033 authorizes against that Pool are authorized against `management`. | LIF-020 lets a Pool retire only when every Resource's shutdown is confirmed, so what is left is inspecting and reconciling. `management` already authorizes them for the Pool itself. One authority at any moment still holds. |
| 2. A further retirement of a consumer finds its Conduits closed | A Conduit whose consumer is retired is closed once the consumer's shutdown is confirmed, or at once where the consumer is declared lost. From the moment the consumer is retired until then, the Conduit permits only requests that are steps of, or serve, an invocation of `retire.v1` for the consumer and the stops of its runtimes; every other request through it reports refused. | LIF-026 makes a further retirement the way to a confirmed shutdown, and its steps need the Conduits. The access of a retired consumer still ends for everything else, and an actor can close the Conduit by a request. |
| 3. `retire.v1` of an instance whose shutdown is confirmed | It succeeds, dispatches nothing and changes nothing. | `open.v1` on an open Conduit and `close.v1` on a closed one do the same. |
| 4. A permitted request that names what does not exist | A request made through a Conduit that lists the operation, and that names an object, a policy or an invocation that does not exist, reports invalid. A bound is judged against what exists, so it does not turn that into refused. OPS-015 is unchanged: it is about what an instance has no invocation under, and says nothing of what exists. `set-up.v1` or `retire.v1` that names a Conduit reports invalid, as LIF-012 states for a template. | Refused means that no open Conduit permits the request. Whether the Conduit's target is the right authority for the thing named can be judged only for a thing that exists. |
| 5. `#name` as `from` in a submitted project | In a submitted project, `#name` written as `from` resolves as the bare name does: among the objects that exist, of which the templates the same submission registers are part. NAM-022 excepts it from "never looked up among the objects that exist". | A project's templates are registered objects, unlike its contracts. The same documents then read alike as a project and as a package, and a later instantiation or a replacement resolves like any bare name. |
| 6. An operation that a Conduit may list and for which no implementation is selected | TPL-004 covers every operation of every capability the template's type offers and every control the type lists, as well as what it covered. With no selection and no default, the template is rejected. | A type contract promises what its instances offer. A template that cannot keep the promise is wrong when it is registered, not when a Conduit is opened. |
| 7. A requirement on a Conduit that does not narrow and is not `shared-host.v1` | Applying to a Conduit, or as a constituent of a requirement applied to a Conduit, a requirement that does not narrow requests and is not of the core package is rejected. `obliges` on a requirement that does not narrow is rejected. A policy's requirement narrows (ACC-031). | An engine checks only the requirements it knows, which are the core package's. Anything else is applied by implementations, and they are sent only filters. |
| 8. What `obliges: [consumer]` has the consumer's implementation do | The tool that the engine presents to a consumer for an operation of a Conduit carries, in the `_meta` of its definition under the key `restrictions`, the filters of that Conduit whose contract obliges the consumer, each item in the form IFC-018 gives. What keeping to a filter means for a consumer is the requirement's contract. | The consumer's implementation must state support under CON-022 and had no way to learn what applies. |
| 9. Later work: which operation a start invokes, and who asks for it | A start is asked for by the implementation that holds the work, through a tool the engine always presents to an instance, `start-later-work.v1` of the core package. It names the engine's ID of the invocation that arranged the work, an operation of its own instance, and arguments. The engine dispatches an invocation of that operation to that instance, for the caller and through the Conduit of the arranging invocation, with the restrictions as they then stand, where the arranging invocation was dispatched to the instance that asks and is of an operation marked `arrangesLaterWork`, the Conduit is open and lists the operation, and, where the operation starts model work, the arranging operation is marked `startsModelWork` too. Otherwise the request reports refused. Either way a record exists. IMP-008 gains the case. A graph that implements an operation without the mark and holds a step whose operation carries it is rejected, and a request for a marked operation that serves an invocation of an unmarked one reports refused, as IMP-004 and IMP-008 do for model work. A plain implementation cannot ask for a start, so a template that selects one for an operation marked `arrangesLaterWork` is rejected. | ACC-019 and IMP-006 already say the engine dispatches each start and records it against the caller. The implementation holds the timer, so it has to ask, and `retrieve-result.v1` is the precedent for a tool the engine always presents. This is the one addition of this pass that is a new mechanism. |
| 10. Whether a registration installs the packages its manifest lists | It does not. A package is installed only by `install.v1`. `fetch` in a manifest's item is the place an installation of that item fetches from. A relative `fetch` in a manifest handed over without a directory is rejected when the manifest is read, as MAN-006 states. | PKG-006 and PKG-028 already register and install with a package missing. Nothing in the language happens without a request that asks for it. |
| 11. The outcome of `retire-with-resources.v1` when Resources remain | Failed, with the result listing what remains. | The effect of the operation is the Pool's retirement, which was not dispatched. Each Resource's retirement is its own invocation with its own outcome, and one that is unknown is reconciled there. |
| 12. Three implementation-defined items | DL7 loses "whether a package can be used from files that are still being edited". DL11 loses "how an optional Conduit that failed to open is opened later". DL3 stays and is reworded in the terms of an invocation that does not end until an actor replies. | PKG-022, PKG-004 and NAM-006 decide the first, CON-004 and ACC-006 the second. DL3 records defect L74, and nothing limits how long an invocation runs. |
| 13. Whether every engine takes files | Yes. Every engine has an entry point that takes a directory and registers what it reads as one submission, with the same result as `register.v1` given the values read. What the entry point is, and whether it also installs the manifest's packages by requests for `install.v1` first, is implementation-defined (DL7). | The rules on files are language promises, and conformance vectors are files. |
| 14. The shape of an inspection result and of a diagnostic | Deferred to the design of the conformance vectors (A39), and listed as known for round 3. Settled now: what a request that is not dispatched says beyond its reason, it says in diagnostics, so LIF-020 names the Resources in the way in a diagnostic, and `retire.v1` and `unregister.v1` need no output. | What a vector compares fixes the shape, and inventing it before the vectors would fix it twice. |
| 15. The sections of the Model Context Protocol that IFC-003, IFC-010, IFC-012 and IFC-016 rely on | Checked against the published revision by a reader with access to it. The report is [mcp-citations.md](mcp-citations.md), and the coordinator corrects what it found. | They could not be checked from the repository. |

Still for the owner, as before: how a value is marked as secret.

## What the check against the Model Context Protocol found

The [check](mcp-citations.md) of 2026-11-06 read the interface promises against the published
protocol. Revision 2026-07-28 exists and is the current release. Of 31 claims, 26 are
confirmed, 3 are wrong and 2 cannot be confirmed. The coordinator settles them here, under
ruling 37 and the owner's standing direction to adopt an existing standard as it is. Where
Conduit's text departed from the protocol, Conduit's text changes. Each can be struck by the
owner.

| Finding | Decision | Why |
| --- | --- | --- |
| Row 17. The protocol recommends, and does not require, that a tool name be at most 128 characters | IFC-012 and IFC-014 say that the protocol recommends at most 128 characters and that this specification makes it a limit. | The figure was right and its force was overstated. |
| Row 19. The engine answered a call to a tool it does not present with a tool result; the protocol makes an unknown tool the invalid-params error, code -32602 | The engine answers with the protocol's error. The error's `data` holds Conduit's `reason` key with the value refused, and the request is recorded as refused. | It is what the protocol prescribes and what IFC-010 already expects of a plain server. The reason stays readable. |
| Rows 21 and 22. A tool that declares `outputSchema` must return `structuredContent` that conforms; a withheld result and a repeat of an invocation still in progress carry none | Both are returned with `isError` true, with the keys that say why: `withheld`, or `state`. An outcome is still carried where there is one. The specification takes it that a result whose `isError` is true need not conform to `outputSchema`, which the revision does not say either way, and says so. | A client that validates results against `outputSchema` does not validate an error result. `isError` then means that the result carries no outputs. |
| Row 28. Where an engine uses the protocol's own authorization over HTTP, an invalid token must receive HTTP 401, so no tool result can say refused | IFC-032 says that such a request is answered as the protocol requires, with no tool result, and is still recorded as refused. | The protocol's requirement stands. |
| Rows 11, 15, 16, 23 and 24. Wording that is true and loose | IFC-007: a plain implementation's request holds only keys the protocol itself defines. IFC-010: -32602 is the code for invalid parameters, of which an unknown tool is one case. IFC-012: the characters the protocol recommends are ASCII letters, digits, `_`, `-` and `.`. IFC-025: `content` is always present in a tool result, and may be empty. IFC-016: the tasks extension is an official extension outside the core revision. | Each is the report's suggested wording. |
| What the revision requires and the interface does not mention: `clientCapabilities` on every request, `server/discover`, `ttlMs` and `cacheScope` on a tool list, the `Mcp-Name` header, `resultType: task` | IFC-001 says that the engine and an implementation each keep every requirement of the protocol at the revision in use, and that this specification repeats none of them. IFC-010 says that where the tasks extension is used its table applies to the task's final result. A consumer may hold a tool list for as long as the protocol lets it, and a call to a tool that is no longer presented is answered as row 19 states. | The interface is the protocol with Conduit's keys added, and restating the protocol would fork it. |

## The last pass, and what it left

Done on 2026-11-06 by one editor ([log](log-tail.md)). 410 promises became 421. The pass listed
fourteen points, of which three were already known or were its own choices, and eleven were
new, most of them about later work, which decision 9 above made concrete for
the first time. The coordinator settles them here, under ruling 37, and the same editor applies
them (the log gains a section). Each can be struck by the owner. The choices that pass made
itself are in its log and stand unless the owner strikes them.

| Point | Decision | Why |
| --- | --- | --- |
| 1. What remains after `retire-with-resources.v1` fails | The editor's choice stands: the Resources that remain are named in diagnostics, and the operation declares no output for them. | A failed result holds no outputs. |
| 2. A Resource registered into a retired Pool | The request is not dispatched and reports conflict, and so does a request under LIF-009 for a new Resource for an entry of a retired Pool. | A retired instance is not changed (LIF-015), and a retired location or end is already rejected. |
| 3. Later work arranged by an onward request | The work is tied to every Conduit of the chain: the one the arranging request came through and each Conduit that the invocations it served came through. A start is dispatched only while all of them are open, and carries the restrictions that the arranging request carried under ACC-017, as they stand at the start, with those of its own Conduit. | Closing a Conduit already covers the whole onward chain, and an onward request never carries less than the request it serves. |
| 4. Later work arranged by a step that has no `conduit` | The work is tied to the Conduit, and the chain, of the invocation that the step's graph serves, as ACC-018 ties the step's restrictions to it. | The step acts within that invocation. |
| 5. A `setup` or `retire` graph with a step whose operation is marked `arrangesLaterWork` | The template is rejected, as IMP-005 and IMP-009 reject the like for model work. | `set-up.v1` and `retire.v1` do not carry the mark. |
| 6. A start that names an arranging invocation that did not succeed | A start is dispatched only where the arranging invocation has ended with the outcome succeeded, as first reported or as reconciled. Otherwise the request reports refused. | A failed invocation arranged nothing, and one whose outcome is unknown is reconciled first. |
| 7. How a consumer learns the result of later work | The conventions give no way: a start is an invocation that the consumer did not request, so `retrieve-result.v1` does not return it. The record holds it. An operation that arranges later work says in its contract how its results are read, usually by another operation of the same capability. IFC-016 is about a single invocation that outlasts its call, and says so. | Delivering results of later work is the business of the type that offers it. |
| 8. Opening a Conduit whose consumer is retired | `open.v1` for a Conduit either of whose ends is retired is not dispatched and reports conflict, whatever the shutdown. | CON-025 already refuses to instantiate one, and ACC-006 to open one to a retired target. |
| 9. A composition made only of core requirements that do not narrow | It is accepted. CON-030 rejects a requirement that does not narrow and is not of the core package, applied to a Conduit itself or as a constituent, and nothing else. | The engine checks every such constituent. The earlier decision's wording went further than its reason. |
| 10. `obliges` of a constituent | A composition obliges the consumer where its own contract or the contract of any constituent that narrows names `consumer`, as it narrows where any constituent does. | The two marks are read the same way. |
| 11. An implementation's result whose outcome key and `isError` disagree | The outcome is unknown, as IFC-009 gives a result that holds both an outcome and a reason. | A result that contradicts itself cannot be trusted either way. |
