# Round 3: decisions for the fix pass

The three readers of `spec/` reported 44 findings: [reader 1](reader-1.md), 16, of which 1
unsafe; [reader 2](reader-2.md), 13, of which 1 unsafe; [reader 3](reader-3.md), 15, of which
2 unsafe. Round 2 had 125, of which 8 were classed as unsafe. The
[fourth reader](reader-4.md) read the whole repository and reported 23.

The owner ruled that findings are resolved without a ruling on each (plan-audit ruling 37).
Every row below is the coordinator's choice, not the owner's, and the owner can strike any of
them. "The reader's fix" means the least-change fix that the report proposes for that finding.
One editor applies the rows and logs what it did in [log-fixes.md](log-fixes.md).

## The unsafe findings

| Finding | Decision |
| --- | --- |
| 3C-1. Work arranged by a start of later work escapes the chain | The reader's fix. Work arranged by a start of later work, or by a request that serves one, is tied to every Conduit that the work of that start is tied to, and a start of it carries what its arranging start was sent. |
| 3C-2. A runtime whose start is reconciled as having occurred is never held | The reader's fix. `reconcile.v1` gains the optional input `handle`. The start of a runtime is reconciled as having occurred only with the handle, and the instance then holds the runtime; without one the request reports invalid. A stop that is reconciled as having occurred ends the holding. One reconciled as not having occurred leaves the instance as it was. |
| 3A-1. A request that cannot be decided may report conflict or invalid, with diagnostics, to an actor never found to be permitted | Where the decision on access cannot be made (ACC-037), the request reports unavailable and nothing else about it is examined or reported. OPS-023 puts that case where refused stands in its order. Unavailable for any other cause stays last. |
| 3B-1. `bound.v1` as a constituent of a composition | `bound.v1` is never a constituent. A composition that reaches it, directly or through other compositions, is rejected when its package is installed or its project is submitted. |

## Reader 1

| Finding | Decision |
| --- | --- |
| 3A-2. `open.v1` on an open Conduit whose consumer is retired | It succeeds and changes nothing, as for any open Conduit. CON-031 is about a Conduit that is not open. |
| 3A-3. A Resource document owned by a retired Pool | The document is rejected, as PLC-018 and CON-025 reject a retired location and a retired end: it is a rule of the language, and its registration reports invalid under OBJ-023. POOL-004 says so. POOL-017 keeps the request under LIF-009 alone, with the reason of 3A-7. |
| 3A-4. `#name` written as `from` | The decision of round 2 stands, and the `then` of INS-025 and of NAM-022 now say what their observables say: in a submitted project, `#name` as `from` resolves as the bare name does, among the templates that exist, of which those the same submission registers are part. In a package it reaches a template of the package. |
| 3A-5. A result with no outputs, delivered after its Conduit closed | ACC-034 excepts it, as IFC-025 does: there is nothing to withhold, the outcome is delivered, and the result is not an error. |
| 3A-6. A carried restriction and an onward target whose implementation does not support it | The reader's fix: an onward request that carries a restriction which the implementation selected by its target does not state in `supports` is not dispatched, with the reason ACC-014 gives, and the promise says so beside the cases of CON-022, ACC-015 and ACC-018. |
| 3A-7. `register-replacement.v1` that a rule on instantiation does not make | It reports invalid, as the registration of the document would under OBJ-023. |
| 3A-8. "Every request" against a retired Pool moves to `management` | ACC-033 names the two requests that move: inspecting a Resource of the Pool, and reconciling an invocation. Any other request to a retired Pool is not dispatched under LIF-015. |
| 3A-9. A rename to the same name, or a replacement by an equal document | It succeeds, changes nothing and does not advance the revision, as REV-004 states. OBJ-026 and SUB-009 except it. |
| 3A-10. A graph written in place cannot state `supports` | A graph states none. It supports a requirement where the implementation of each of its steps that has no `conduit` does; a step with `conduit` is an onward request and is checked at its own target. CON-022, ACC-015 and ACC-018 say so. |
| 3A-11. A step's identity outlives its run | The reader's first fix. A step keeps the identity of an earlier run only while that run's invocation of the step is in progress or its outcome is unknown. Any other step dispatched in a later run is a new invocation with a new identity, derived from the ID of the invocation of that run. |
| 3A-12. The caller of a retrieval | See 3C-4. |
| 3A-13. The `stop` that the engine sends under OBS-007 has no caller | It carries no `caller` key, and its record says that the engine dispatched it under OBS-007 for no actor. IFC-006 excepts it. |
| 3A-14. Eleven rules that stand only in an observable | Each moves into a `then`: of the same promise where its `when` covers the case, and otherwise of a new promise in the same file. Citations follow. No rule changes. |
| 3A-15. Three `language` promises shown only through operations of the conventions | The editor rewords each observable to what resolving shows, where the rule is about what documents mean. Where the rule is about a request, the promise becomes `convention`. The log says which. |
| 3A-16. Citations in the core README and one example | The reader's fix. |

## Reader 2

| Finding | Decision |
| --- | --- |
| 3B-2. The resolved form `{kind, id}` in a request | A request that an actor makes on its own behalf names an object by reference only, as ruling 37 has it, and one that holds the resolved form is not dispatched and reports invalid. A step of a graph, and an onward request, may hold the resolved form, because the engine handed it to them: a value read from `config` and an argument sent to an implementation are in that form. The owner may want to look at this one, since it bears on a change the owner made in ruling 37. |
| 3B-3. The example's `directory.sweep.v1` | The example package gains the operation that a start invokes, listed in the capability, and the description says which. |
| 3B-4. A `graph` item with `plain: true` or `tools` | The schema stands. MAN-009 and PKG-020 say that an implementation that is a graph is never plain and lists no tools, and the README names the choice. |
| 3B-5. Request and caller have a third case | The definitions' rows gain it: a request for `start-later-work.v1`, which an instance makes neither on its own behalf nor as an onward request, and whose caller is the caller of the arranging invocation. |
| 3B-6. `deduplicates` has no stated meaning | CTR-002 states it in its `then`, and the definitions gain a row. |
| 3B-7 to 3B-10. Second words and loose wording | The reader's fixes: "origin" is kept for a record; "sender", "composed" of a template and "the report" are replaced; a rejected package reports invalid only; TPL-003 says "a member of a template's `controls` map". |
| 3B-11. Missing citations under `spec/schemas/` | The reader's fix. |
| 3B-12. An instance document in a package | A package holds templates and contracts only. One that holds an instance document is rejected when it is installed, and MAN-007 and MAN-011 say so. |
| 3B-13. The keys of an implementation item's `placement` map | They follow the rule that OBJ-022 gives for a Type's. |

## Reader 3

| Finding | Decision |
| --- | --- |
| 3C-3. Which policies an onward request or a start carries | The reader's fix. The policies carried are those that apply, when the onward request or the start is decided, to a request of that caller to the target of each invocation in the chain. "A start never carries less than the request that arranged the work" is restricted to filters. |
| 3C-4. For which caller a retrieval is made | The reader's fix. A retrieval names in `serving` the invocation that the original request served, which need not be in progress, and the caller is that invocation's. One that names none is made for the instance as its own caller. IFC-021 and IFC-028 except the tool. |
| 3C-5. Installing what is installed when the archive cannot be fetched | The reader's fix. PKG-025 and PKG-027 are decided from what is installed, and nothing is fetched for an address and version that is already installed. |
| 3C-6. The order in which `retire-with-resources.v1` retires | The reverse of the order of setup: a Resource is retired before any Resource that it refers to or to which it is the consumer of a Conduit, among those the Pool owns. |
| 3C-7. Which version of a requirement's contract a policy names | `apply-policy.v1` gains the optional input `version`. Where it is left out, exactly one installed version of the package must export the requirement, and otherwise the request reports invalid. The policy keeps the version and the digest. |
| 3C-8. Reconciling an invocation whose instance is unregistered | It is authorized against `management`, records the outcome and stores nothing. For a control it names no revision and no settings. |
| 3C-9. A value reference to a step that was not dispatched in this run | Two rules. A value reference to an output of a step that names a runtime is rejected when the template is registered: the handle is read with the source `runtime`. A step that is not dispatched in a run, because it is left as succeeded or was reconciled, returned no outputs in that run, and a reference to one yields no value under GRA-026. |
| 3C-10. A constituent's binding that yields no value | The constituent's input counts as not supplied: its default applies, and where it has none and is not optional, applying the composition is rejected. |
| 3C-11. The manifest handed over with `replace.v1` | The reader's fix. `#name` in the replacing document reaches the contracts and the implementations of the unpublished package that the template holds. `implementations` and `exports` in a manifest handed to `replace.v1` are rejected. A replacing document that refers to a package, with no manifest, is rejected under NAM-007. |
| 3C-12. `declare-lost.v1` for a retired location; "placed on it" | PLC-011 counts every instance that exists, retired or not. `declare-lost.v1` for a location that was retired by a request is dispatched, and LIF-015 lists it. For one already declared lost it succeeds and changes nothing. |
| 3C-13. A project's own contract that needs a package that is not installed | The reader's fix: accepted with a diagnostic, in the words of PKG-028. |
| 3C-14. What a purge keeps | The reader's fix: REC-004 also keeps the operation, the instance the invocation was dispatched to and the target that authorized it, and they are read as records of that instance. |
| 3C-15. A graph that invokes the operation it implements | Rejected when the template is registered. GRA-005 extends the cycle to the graphs selected in one template's `implementation` map and in its type's defaults. |

## Reader 4, inside the specification and the overview

The findings outside `spec/` were fixed when the report came in (commit `d3ae339`).

| Finding | Decision |
| --- | --- |
| 3D-2. "Accepted by the owner" above text that later passes changed | `spec/README.md` says what the word covers: the owner accepted each part as it stood at the ruling named, and what rounds 2 and 3 of the re-audit changed since rests on the coordinator's decisions, which are logged and which the owner has not ruled on. The status lines of `spec/core/README.md`, `spec/schemas/README.md` and the head comments of the core documents say the same and no longer call a later choice one the owner accepted. |
| 3D-11 to 3D-17. The overview lags the specification | `docs/OVERVIEW.md` is read against the specification once more after this pass and corrected: the two tools that need no Conduit, a retired consumer's Conduits, installing, later work, the count of ways a request ends, and that a conformance claim names a version. |
| 3D-16. "Not one of the seven" in the definitions | The definitions' section on how a request ends agrees with its own opening sentence. |
| 3D-5. The interface as changed has not been checked against the protocol | It is checked again after this pass, by a reader with access to the published protocol. |

## Still open

- How a value is marked as secret. For the owner.
- The shape of a diagnostic beyond `message`, and the full shape of an inspection result.
  Settled with the conformance vectors.
