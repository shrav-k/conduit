# Round 4: decisions for the fix pass

The owner ruled that findings are resolved without a ruling on each (plan-audit ruling 37).
Every row below is the coordinator's choice, not the owner's, and the owner can strike any of
them. "The reader's fix" means the least-change fix that the report proposes for that finding.
One editor applies the rows and logs what it did in [log-fixes.md](log-fixes.md). The counts of
the round are in the [record](README.md).

## The unsafe findings

| Finding | Decision |
| --- | --- |
| 4A-1. A retirement reconciled as having occurred confirms the shutdown of an instance that holds a runtime | One rule, in LIF-024: the shutdown of an instance that holds a runtime is never confirmed. A reconciliation that would record the retirement of such an instance as having occurred is not dispatched and reports conflict; the runtime is stopped by a further retirement, or its stop is reconciled, first. LIF-028 and TPL-015 follow. |
| 4C-1. Later work goes on for a caller that is retired | A start is dispatched only where each Conduit of the chain is open and would permit the request under ACC-039, so it is refused once the consumer of any of them is retired. ACC-040 says so. |
| 4A-5. A graph that a manifest lists, or that a type or an operation states as a default, escapes the rules on model work and later work | IMP-004, IMP-010 and IMP-012 cover every selected graph, however it was selected, as GRA-005 does. |

## Reader 1

| Finding | Decision |
| --- | --- |
| 4A-2. A handle for a start whose instance is unregistered; `revision` or `settings` for an invocation that is not of a control | The reconciliation of a start whose instance no longer exists stores nothing and names no handle, and one that gives a handle reports invalid, as OPS-031 has it for a control. `revision` or `settings` given for an invocation that is not of a control reports invalid. |
| 4A-3. The description of `object-management.v1` | The reader's fix: it follows ACC-033. |
| 4A-4. IFC-018 and the definition of restriction after ACC-041 | Both name what a start of later work carries. |
| 4A-6. A revision for a reconciliation that records no effect | `revision` is named only by a reconciliation that stores settings. One that records that the effect did not occur names none, and one that gives it reports invalid under OPS-031. REV-010 states the rule in its `then` and cites nothing that does not cover it. |
| 4A-7. A policy on an actor makes its requests to `management`, `access` or a Pool report unavailable | The behavior stands: a restriction that the target's implementation cannot apply is never dropped, and `engine` supports `bound.v1` alone. ACC-015 and ACC-026 state the consequence in so many words, including that the actor can no longer remove the policy itself and that the root Interface, which no policy applies to, can. The overview says it too. Also listed for the owner. |
| 4A-8. A retrieval that names an unregistered target by `{kind, id}` | Still refused. OPS-015 covers a target that no longer exists, however the request names it. |
| 4A-9. IFC-026 is `may` where IFC-019 requires the reason | IFC-026 becomes `must`. |
| 4A-10. Five more reasons that stand only in an observable | Each moves into a `then`, as in round 3. |
| 4A-11. What the digest of an unpublished package hashes | The manifest as it was handed over, before any relative `fetch` or program path is resolved. |
| 4A-12. `install.v1` with a relative `fetch` for a package that is installed | Invalid. PKG-027 says that the request is decided like any other, so a request that is invalid reports it. |

## Reader 2

| Finding | Decision |
| --- | --- |
| 4B-1. The description of `object-management.v1` | As 4A-3. |
| 4B-2. A handed-over manifest that states the core package's address as its own | Rejected, when it is registered or installed. NAM-021 says so, and the schema's `core` form is for the core package's own manifest alone. |
| 4B-3. "Management verb" has two tests | One test, in the definitions: an operation of the core package other than its two controls and the two tools that the engine always presents, `retrieve-result.v1` and `start-later-work.v1`. CON-024 and ACC-033 use the term so. |
| 4B-4. "Submitted project", "unpublished project", and a registration whose manifest states `address` and `version` | One term, "submitted project", for what is handed over in a registration; "unpublished package" stays the name of what the engine keeps of it under SUB-008. A project that is handed over for registration is never published, whatever its manifest states: `address` and `version` there change nothing, and `#name` means what it means in any submitted project. |
| 4B-5. The manifest handed to `replace.v1` | The schema stands. SUB-009 is cited in the table of what a schema cannot express. |
| 4B-6. "The templates of the first-start objects" | "The templates that first start instantiates", since two of the seven make Conduits that are not first-start objects. |
| 4B-7. "Chain" | The definitions gain the row: the chain of an invocation is the invocations that it serves, directly or through others, and "the Conduits of the chain" are those they came through. The promises use it so. |
| 4B-8. Two rows narrower than their promises | The reader's fix for `hold` and for `supports`. |
| 4B-9. The output `revision` | SUB-009 names it for `replace.v1`, and `rename.v1` gains the same output, since a rename advances the revision. |
| 4B-10. `serving` in the core README | The reader's fix. |
| 4B-11. Status wording at the head of the promise files | As 4D-26: every head says which changes the owner accepted and which are the coordinator's, and the three files with no note gain one. |

## Reader 3

| Finding | Decision |
| --- | --- |
| 4C-2. A step whose `conduit` names a composite in which two components list the operation | A step may name a component by the dotted path that IFC-012 gives a tool's name. A step that names a composite Conduit in which more than one component lists the operation, and gives no path, is rejected when the template is registered. |
| 4C-3. The setup step of a composite declared Conduit when only some components open | The step succeeds only where every component opens. Where one does not, those that opened are closed again and the step is failed, so a composite Conduit is open as a whole or not at all. |
| 4C-4. Which Conduit an invocation with several attempts was dispatched through | That of its latest attempt. OPS-017, ACC-034 and ACC-019 read it so. |
| 4C-5. `register-replacement.v1` after the template named by `from` changed | The new instance is made from that template as it now stands. Where the template no longer exists, the request reports invalid. |
| 4C-6. `retire-with-resources.v1` and a Resource that is retired with its shutdown not confirmed | A further retirement of it is asked for, decided as LIF-026 decides one. Nothing is dispatched for a Resource whose shutdown is confirmed. |
| 4C-7. The outcome of a run in which a step left unknown is not dispatched | Unknown. |
| 4C-8. What a purge keeps | REC-004 adds the Conduit that the invocation was dispatched through, its caller, and for a step that names a runtime the runtime's name. |
| 4C-9. A policy that names its requirement through a re-export | `version` and "the requirement's package" mean the package that the reference names. The policy keeps that package's version and digest, and PKG-013 leads to the origin. |
| 4C-10. Property defaults in a schema that is not a type's | One rule for every schema that governs a supplied value: a property that is absent from a map that is present, and that has a `default`, is filled before the value is validated and before it is compared, and the filled value is what is stored or sent. SCH-009, INS-006 and OPS-026 say the same. |
| 4C-11. A selection or a `from` that names nothing where the package is at hand | The document is rejected. Only a package that is not installed defers the check. |
| 4C-12. A step that invokes a runtime's stop operation directly | A step may not name, as its `operation`, an operation that a runtime of its template names as its start or its stop. Such a template is rejected. |
| 4C-13. Policies for a step that has no `conduit` | Judged when each step is decided, as ACC-017 now has it for an onward request. |
| 4C-14. A tool for an operation whose contract the target's version of the package does not hold | No tool is presented for it, and a call reports as for any tool that is not presented. |
| 4C-15. Reading records of an unregistered object | REC-003 and LIF-019 point to DL8, which is where it is settled. |

## The second check against the Model Context Protocol

The [report](mcp-recheck.md) read IFC-001 to IFC-035 as round 3 left them: 44 of 48 claims
hold, 1 is wrong as worded and 3 cannot be confirmed from the protocol.

| Row | Decision |
| --- | --- |
| 26. IFC-025 says that only two kinds of request have no tool result | The report's wording. A request that the protocol has a server reject with an error of its own is answered as the protocol states. Where the engine could establish the instance that sent it, the request is recorded as invalid, and the error's `data` carries the reason. An error for an unsupported revision is part of the negotiation of IFC-003 and is not a request that ended. |
| 22. A succeeded result of an operation that declares no outputs | Its `structuredContent` is present and is an empty map, so one rule holds for every tool. |
| 36. A tool list that changes because of the consumer's own request | The report's sentence is added to IFC-012: the list depends only on the instance and on which of its Conduits are open, never on the connection, and the specification takes the opening or closing of a Conduit to be a change in what the instance is permitted. |
| 21. Whether an error result must conform to `outputSchema` | No change. The text already gives it as the specification's own reading. To be looked at again when the protocol settles it. |
| 18, 23, 29, 30, 47. Loose wording in rows that hold | The report's wording for each: what `isError` means on the engine's results; `content` may hold a serialized copy and DL12 says what the engine puts there; a `resultType` that the engine does not recognize is unknown; what the table of IFC-010 applies to for a task that failed, was cancelled or was discarded; and the HTTP status the protocol prescribes. |

## Reader 4, inside the specification and the overview

The findings in the records were fixed by the coordinator when the report came in (commit
`44ab30c`):

| Finding | What was done |
| --- | --- |
| 4D-1 "Until ruling 37 every change was a ruling" | The tracker says that every decision put to the owner was ruled on, and names the choices that the applying passes made, with where each is logged. They are in the list for the owner. |
| 4D-2 A third decider, and readers with no kept report | The tracker's table says that an editor's own choices stand unless struck, and names the three readers that editors started in round 2. The round-2 record says the same. |
| 4D-3 Committing at each milestone given as the owner's instruction | `AGENTS.md` gives it as the coordinator's reading, which the owner has not confirmed. |
| 4D-4 "Accepted by the owner" on the front page | Reworded to what `spec/README.md` says. |
| 4D-5 "What the owner ruled" of every record | The documents index and `AGENTS.md` say whose decisions the last records hold. |
| 4D-6 Two records that do not say whose decisions they hold | The documents audit and round 2 say that they are the coordinator's. |
| 4D-7 Decisions that changed what the owner accepted | Six rows under "Later amendments" in the tracker, and a line in the five records concerned. |
| 4D-8 Omissions from the list for the owner | Added: the earlier choices, the conversions of the documents audit, the choices made while ruling 37 was applied, the four protocol corrections, and two points of the documents audit. The first protocol report carries a note. |
| 4D-9 Three pointers | Corrected. |
| 4D-10 "A result with no outputs" | Reworded. |
| 4D-11 The cold read by Codex | A step in "What happens next", and a clause in the status of A61. It is the owner's to start. |
| 4D-12 No record of the round-3 fixes outside `spec/` | A row for each in the round-3 decisions. |
| 4D-13 The status of A61, and "Round 3 is read" | Corrected. |
| 4D-14 57 findings | 47. |
| 4D-15 The counts of round 1 | "About 80" for reader 1, and the records say what "about 170" counts. |
| 4D-16 Which passes changed the interface after the check | Corrected, and the second check is named. |
| 4D-22 A second repository that answers "not found" | Left for the owner, in the list. |
| 4D-23 The "Claude-Session" line on commits | Left for the owner, in the list. |
| 4D-24 "Every packet predates the specification" | "Every packet but the naming research". |
| 4D-25 The round-4 record named no commit | It names `f583469`. |

The rest are in `spec/` and the overview, and belong to the fix pass:

| Finding | Decision |
| --- | --- |
| 4D-17 to 4D-21. The overview | Corrected against the specification after this pass: that the target's implementation applies filters and policies and a plain one supports none; the exceptions for the root Interface and the first-start objects; the stop that the engine dispatches for no actor; a consumer declared lost; where reconciliation goes when a Pool is retired or an instance unregistered. |
| 4D-26. File heads under `spec/` | The heads say of the documents audit and of round 2, as they say of round 3, that the changes are the coordinator's. |

## Still open

- How a value is marked as secret. For the owner.
- The shape of a diagnostic beyond `message`, and the full shape of an inspection result.
  Settled with the conformance vectors.

## The fix pass, and what it left

Done on 2026-11-06 by one editor ([log](log-fixes.md)). 437 promises became 442. The pass left
five points. The coordinator settles them here, as above, and the same editor applies them (the
log gains a section). The choices that the pass made itself are in its log under "Choices";
they are the editor's, and stand unless struck. One of them is struck here, in row 1.

| Point | Decision |
| --- | --- |
| 1. A composite Conduit of which one component cannot be opened | The decision on 4C-3 is replaced, because applied to the letter it leaves the components that opened closed for good. The opening of a Conduit made from a composite template is decided for every component before any is opened. Where the opening of any component would not be dispatched, none is opened, nothing is closed, and the request reports that component's reason; a setup step is then failed, or left as it is where it is optional. Otherwise all are opened. Where the opening ends unknown, it is unknown for the whole, and its reconciliation records every component as open or every component as not open. The exceptions that the pass gave LIF-027 and LIF-023 are taken out again. |
| 2. Onward requests that serve an invocation whose chain has a retired consumer | The rule of 4C-1 holds for them too. An onward request is dispatched only where each Conduit of its chain is open and would permit the request under ACC-039, so it reports refused once the consumer of any of them is retired, unless the invocation it serves belongs to that consumer's own retirement. |
| 3. How a re-exported requirement is named on the wire and matched against `supports` | By its origin. `restrictions` names a requirement by the address and export name of the package that first exports it, and an implementation supports it where its `supports` names that origin or any re-export of it, as ACC-029 has it for a bound. |
| 4. A re-export in `exports` that names nothing in an installed package | Rejected. NAM-028 covers every reference into an installed package that names nothing there, a re-export included. |
| 5. A request that the protocol rejects, from a client that is not an instance | IFC-036 governs: a request that cannot be read is answered as the protocol states, whoever sent it, and IFC-032 is about a request that can be read. It is not recorded where no instance can be established. |

The follow-up is applied, with no new promise: the specification stays at 442. The editor left
two points for a later reader, and they are not decided:

- `open.v1` names one Conduit in its input, while CON-004 now speaks of one invocation that
  opens every component of a composite declared Conduit.
- The rule that a composite is opened whole is stated for the setup step. A request that an
  actor makes for `open.v1` or `close.v1` still names one component, so a composite can still
  be left partly open outside setup.

Nothing in the text that this round changed has been read by a cold reader, and the interface
text that the fix pass changed has not been checked against the Model Context Protocol again.
