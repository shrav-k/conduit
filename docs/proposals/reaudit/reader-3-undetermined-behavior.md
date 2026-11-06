# Re-audit, reader 3: undetermined behavior

Cold reader's report of 2026-11-05, kept verbatim. The reader saw only `spec/` and walked ten
scenarios end to end as an engine builder would. Findings are not rulings; what was done about
each is in the [re-audit record](README.md).

```text
COLD-READ FINDINGS: spec/ only, all 25 files read in full (312 promises, 12 list entries). Nothing edited.
Each finding gives: scenario; question; promises consulted; plausible behaviours; layer and list coverage; one-line fix.

=== UNSAFE ===

1. S3, filtered call. How does a filter or policy reach whoever must apply it, and who is that?
   - Promises: ACC-013 "the target's implementation does not support a filter"; PKG-019 `supports`, "the requirements it can apply"; IFC-006 lists five `_meta` keys (interfaceVersion, instance, operation, operationIdentity, caller), none naming the Conduit, its filters or policies; IFC-007 sends a plain server no keys at all; CTR-005 gives a Requirement no evaluable semantics.
   - Behaviours: (a) the engine evaluates and the implementation never sees the filter; (b) the engine invents a `_meta` key; (c) the engine pushes filters at open by an unspecified call. An implementation written for (b) and run under (a) applies nothing, yet the Conduit opens because the manifest says `supports`.
   - Layer: interface. No list entry.
   - Fix: add `_meta` keys for the Conduit and applied restrictions, and state that a plain implementation supports no requirement.

2. S3, onward request. How does the engine tie an onward request to the inbound call it serves?
   - Promises: IFC-015 "as a client of the server the engine presents for that instance"; ACC-017 "carries the caller's identity, the filters on the Conduit the caller came in through"; ACC-018.
   - Behaviours: with two callers in flight on one Resource the engine (a) guesses, (b) attributes the request to the instance itself and drops the caller's filters, or (c) invents a parent-identity key. (a) and (b) are a confused deputy.
   - Layer: interface. No list entry.
   - Fix: require the onward request's `_meta` to carry the inbound operation identity.

3. S7, unknown outcome on a deduplicating operation served by a plain server.
   - Promises: OPS-012 allows a new attempt when the contract declares `deduplicates`; CTR-002; IFC-007 says a plain server never receives the operation identity; IFC-010 "no answer, is unknown"; TPL-005 and PKG-020 do not forbid the pairing.
   - Behaviour: after a timeout the engine re-dispatches, the server cannot recognise the repeat, and the effect is applied twice. A fully conforming engine does this.
   - Layer: convention/interface.
   - Fix: reject selecting a plain implementation for a `deduplicates: true` operation, or treat such an operation as non-deduplicating when plain.

4. S2/S4/S7, setup or retire run again. Are steps that already succeeded, or ended unknown, dispatched again, and under what identity?
   - Promises: CON-004 and POOL-013 skip only Conduits and entries; LIF-014 "can be retired again"; OPS-012; IMP-007 "without a new request from its caller"; TPL-007 holds one handle per runtime name.
   - Behaviours: (a) a new setup request gives every written step a fresh identity, so a non-deduplicating step whose earlier outcome was unknown runs again, bypassing OPS-012, and a runtime start runs twice, orphaning the first process; (b) steps keep stable identities and are skipped or blocked until reconciled.
   - Layer: convention. DL1 and DL2 do not cover it.
   - Fix: define a step's identity as derived from instance and step, and say what a re-run does with succeeded and unknown steps.

5. S4, close during an in-flight call. Is the response content delivered, and may the remaining steps and onward requests of that invocation continue?
   - Promises: ACC-011 "all the access it carried ends"; CON-014 speaks only of the "next request"; OPS-016 (outcome is told); OPS-017 (content withheld, but only for a retained result); ACC-019 "work's requests report refused when it runs".
   - Behaviours: (a) deliver full content and let the graph finish; (b) withhold content by analogy with OPS-017 and refuse remaining onward steps, leaving a partial effect; (c) cancel.
   - Layer: convention. DL1 covers cancellation of graphs only.
   - Fix: one promise for an invocation dispatched before a close.

6. S5, policies. Is applying or removing a policy limited by the holder's bound?
   - Promises: ACC-031 "permitted like any other by an open Conduit to `access`"; the definition of right to open says its filters bound "the Conduits its consumer may open and close", with no mention of policies; ACC-024 and LIF-022 protect root only from close and retire.
   - Behaviours: (a) literal: a delegated Interface whose bound names one Pool may remove a policy root applied elsewhere, restoring access (ACC-026), or apply a deny policy to `root`; (b) the bound is applied by analogy.
   - Layer: convention.
   - Fix: state that the bound covers the actors and targets of policies, and who may remove a policy.

7. S5, passing a right on. Four open questions:
   - Must the delegator's bound cover `access` itself as a target? ACC-006 says the bound must cover "the new Conduit's target"; ACC-029 says a Pool bound covers "exactly the Resources that Pool owns", and `access` belongs to `system`; ACC-030 presumes passing on is possible.
   - Is the new Conduit's operation list limited to the operations the delegator's own `access` Conduit lists?
   - What does "within" mean between two bounds (a Pool against a list of its current Resources; operations per target or as a product)?
   - Can the delegator close what it delegated? ACC-012 needs a bound covering the Conduit's target, which is `access`.
   - Behaviours: engine A refuses all delegation by a Pool-bounded holder; engine B special-cases it and lets a holder with only `open` hand out `applyPolicy`.
   - Layer: convention.
   - Fix: define the bound contract, its subset relation, and the rule for Conduits whose target is `access`.

8. S8, digest. What happens when the installed archive's digest differs from the manifest's for the same address and version?
   - Promises: NAM-006 (the manifest gives a digest); PKG-003 rejects only a version mismatch; PKG-005 "the instance holds that package's version and digest"; NAM-012 "provided the digest matches" appears only in an observable about sources.
   - Behaviours: (a) reject; (b) use the installed archive and record its digest. (b) runs unpinned code.
   - Layer: language. DL7 covers storage and fetch only.
   - Fix: a promise that a digest mismatch is unresolved, exactly as PKG-003.

9. S9, model work through code. May a code implementation of an unmarked operation, or of a setup step, start model work by an onward request?
   - Promises: IMP-004 and IMP-005 check graphs only; IMP-001 is satisfied because the onward invocation is itself of a marked operation through the instance's own open Conduit; IMP-002 excepts "a separate invocation of an operation that declares it".
   - Behaviours: (a) permitted, so setting an instance up can start model work and "who may cause whom to spend" is not readable from documents; (b) the engine refuses onward marked operations while serving an unmarked one, which needs the correlation missing in finding 2.
   - Layer: convention.
   - Fix: state that an onward request for a marked operation is refused unless the operation being served is marked.

10. S7, purge. Does purging remove the identity records that deduplication relies on?
   - Promises: OPS-008 "the target keeps one record for that identity"; OPS-018 at most once; REC-004 purge; the definition of record, "kept until purged".
   - Behaviours: (a) after a purge the same identity dispatches again and the effect repeats; (b) identity tombstones survive.
   - Layer: convention.
   - Fix: state that a purge keeps the identity and outcome of every record.

11. S1/S5, who is the consumer. Nothing says how a connection is bound to `root`, another Interface or an Agent.
   - Promises: IFC-012 "the engine presents to that instance" a server; ACC-022; DL12 covers transport and processes, not identity.
   - Behaviours: any local connection is `root`, or tokens, or something else.
   - Probably meant as engine-defined, but the implementation-defined list has no entry and no invariant such as "a consumer cannot present as another".
   - Fix: add the list entry and that invariant.

=== DIVERGENT ===

12. S1/S2/all. The core package's contents are unspecified, apart from the single name `conduit.dev/core#location`.
   - Missing: export names, inputs, outputs, `effect` and `deduplicates` of the `management`, `access` and Pool operations (register, replace, rename, set up, retire, open, close, apply policy, reconcile, declare lost, purge, inspect, new instance for a declaration); the bound and shared-host Requirements; the built-in types, templates and schemas; the names and operation lists of `root`'s two Conduits.
   - Promises: OPS-019, FST-002, FST-005, NAM-020, CTR-003; the definition of core package.
   - Effect: IFC-012 makes these names the tool names, so two engines present different tools and no document is portable.
   - Layer: language and interface. The README's "still to come" list does not mention it.
   - Fix: publish the core package as normative documents.

13. S3/S6/S7, consumer side of the wire.
   - Missing: the `_meta` key by which a consumer supplies an operation identity; where a request "names a revision" (IFC-001 says arguments are "exactly the operation's inputs"); how refused, invalid, conflict, unavailable and the three outcomes are returned to a consumer; how the retained result is requested once the Conduit is closed and IFC-013 has removed its tools (OPS-016); how an Agent implementation finds the presented server.
   - Promises: IFC-006 and IFC-009 cover only the engine-to-implementation direction; IFC-012 and IFC-013 cover only tool lists.
   - Layer: interface.
   - Fix: mirror IFC-006 and IFC-009 for the consumer side.

14. S7. The value of `conduit.dev/interfaceVersion` for this version is never stated, yet IFC-008 obliges an implementation to refuse an unknown one. (OBJ-017 fixes `apiVersion` only.)
   - Layer: interface.
   - Fix: state the value.

15. S1. What is the registered name of a declared Conduit, an entry's Resource and a composite's component, and how does a document outside the Pool refer to an entry's Resource?
   - Promises: OBJ-014 (a nested instantiation has no `metadata`); OBJ-003 (template name plus generated suffix); OBJ-002 (unique per kind); IFC-012 "for a declared Conduit the name is the one its consumer's template gives it"; IFC-014; POOL-005 (`{entry:}` works only inside the Pool).
   - Behaviours: (a) the declaration key is the name, so two Agents from one template collide under OBJ-002; (b) a generated name with a separate tool prefix; (c) `consumer-key`.
   - Under (b) an Agent in the same submission cannot name the Pool's Resource at all. After POOL-010 replaces an entry's Resource, the old name still resolves to the retired one (LIF-007).
   - Layer: language.
   - Fix: define the names of nested instances and a reference form for "Pool P, entry E".

16. S1. How is a reference written and recognised inside `arguments` and `config`?
   - Promises: OBJ-006 (kind and name, syntax not given); OBJ-009 "every reference it holds ... is resolved to an ID"; OBJ-019 (keys inside `config` and `arguments` are unchecked); POOL-005 "in that place"; PLC-005; NAM-003 (a local reference has no `/`).
   - Behaviours: a built-in reference schema, a `{kind, name}` map, or any map with an `entry` key. A literal `{entry: x}` in config may or may not be rewritten to an ID.
   - Layer: language.
   - Fix: one reference schema in core and the rule that only values typed by it are references.

17. S2/S3. On which instance is a step's operation dispatched?
   - Promises: GRA-017 (a step has only `operation` or `runtime`, `arguments`, `dependsOn`, `optional`); OBJ-019 rejects any other key; TPL-004 implies the instance's own implementation; ACC-018 treats a step as "an onward request" to another target; GRA-022.
   - Behaviours: (a) steps only call the instance's own implementations and ACC-018 is vacuous; (b) the engine infers a target from a declared Conduit that lists the operation.
   - Layer: language.
   - Fix: add a `conduit` key to a step, or state that there is none.

18. S1/S10. How is a requirement evaluated?
   - Promises: PLC-007 and PKG-015 "facts ... do not satisfy"; CTR-005 has no predicate; the definition of location fact gives only examples.
   - Questions: who evaluates a third-party placement requirement; what the set of facts is; what happens if the location is unreachable or the fact is unknown at creation (reject or create).
   - Layer: language (it claims to be answerable from documents). DL5 does not cover it.
   - Fix: a closed set of core requirements with defined semantics, a fact schema, and "unknown fact means not created".

19. S1/S2. Authorization of creating and setting up Resources.
   - Promises: ACC-025 (create or set up needs a Conduit to "the Pool that owns, or will own"); ACC-033 (Pool setup needs `management`); POOL-002 and POOL-013 (entries are created and set up with the Pool); ACC-028 "a Pool is never a caller"; ACC-005 exists for declared Conduits, with no analogue for entries.
   - Questions: can a submission creating a Pool with entries, or a Pool and a standalone Resource together, ever be authorized when no Conduit to that Pool can exist yet? Does Pool setup through `management` set up entries without a Pool Conduit?
   - Behaviours: `management` alone suffices (contradicting ACC-025 "whatever other access it holds"), or it is refused.
   - Layer: convention.
   - Fix: an explicit rule for entries at Pool creation and Pool setup.

20. S2. May a Conduit be opened to a target that is only registered, and is a request through it dispatched to an implementation never set up?
   - Promises: LIF-015 blocks only retired targets; ACC-001; CON-013 speaks only of the consumer; DL11 covers only the consumer side.
   - Effect: setting up the Agent before the Pool succeeds on one engine and fails on another.
   - Layer: convention.
   - Fix: state the required target state for opening and for dispatch.

21. S2. What is the outcome of a graph-implemented operation when some steps succeeded and a required one did not?
   - Promises: ACC-005, PLC-013 and POOL-013 "setup fails"; OPS-003 failed means no effect "in whole or in part"; OPS-004 partial means unknown; GRA-003 says only "not succeeded"; LIF-014 (pending against unknown shutdown).
   - Behaviours: failed (retry freely) or unknown (OPS-012 and LIF-018 then demand reconciliation).
   - Layer: convention. DL1 covers the remaining steps, not the outcome.
   - Fix: define a graph's outcome from its steps.

22. S7. How is "a new attempt" requested?
   - Promises: OPS-009 (same identity and contents is "not dispatched again"); OPS-012 (a new attempt on the same identity is dispatched if deduplicating); IMP-007 "dispatched once per request for it".
   - Behaviours: a resend after unknown or failed (a) returns the stored state or (b) re-dispatches.
   - Layer: convention.
   - Fix: say that a resend after unknown on a deduplicating operation is the new attempt, and that OPS-009 otherwise holds.

23. S7. Scope of an operation identity, and the meaning of "same contents".
   - Promises: OPS-008 "the target keeps one record for that identity"; OPS-009; OPS-010; OPS-015.
   - Behaviours: scoped per target, so another caller reusing the value sees the first caller's state, or blocks it with conflict; or scoped per caller and target. "Contents" may or may not include the Conduit and caller.
   - Layer: convention.
   - Fix: scope by caller, target and identity; define contents as operation plus arguments under SCH-007.

24. S6. Controls.
   - Promises: OPS-020 "needs an open Conduit that lists it"; CON-017 (operations are qualified references in a capability); CTR-004 (controls are plain names in a Type); ACC-033 (silent on controls); TPL-003 (no per-control implementation); PLC-016.
   - Questions: how a Conduit lists a control; whether the Conduit's target is the instance, its Pool or `management`; the tool name; which implementation runs.
   - Layer: language, convention and interface.
   - Fix: give a control a qualified reference and name its authorizing target.

25. S6. A control with unknown outcome is later reconciled as succeeded. Is the configuration then stored and the revision advanced, and with which values?
   - Promises: REV-007 (stored are the settings "its result states"); REV-009; OPS-014.
   - Behaviours: store the requested input; store nothing; require a fresh control.
   - Layer: convention.
   - Fix: one promise.

26. S6. A template is replaced or unregistered while instances exist.
   - Promises: SUB-007; INS-013; POOL-010 "instantiated from the entry"; LIF-009; LIF-019; INS-002 "no instance exists without a template it came from"; the definition of definition omits declared Conduits and entries.
   - Questions: are later replacement Conduits and Resources made from the declaration as it was at instantiation or as it is now? Is unregistering a template with instances a conflict?
   - Layer: language.
   - Fix: state both.

27. S8. Templates pinned to different versions.
   - Promises: NAM-011; NAM-017 "package versions are not compared"; the definition of contract, "changes only compatibly" (undefined and unchecked).
   - Questions: which version's Operation contract validates a request and its outputs; what a request reports for an operation in the Conduit's version of the capability that the target's version lacks (invalid, unavailable, or failed).
   - Layer: language and convention.
   - Fix: say the target's pinned contract governs and name the reason.

28. S8. A template is registered while its package is not installed (PKG-006). When do the contract-dependent checks run?
   - Promises: TPL-002, TPL-004, TPL-005, GRA-006 to GRA-008, CON-017, IMP-004, IMP-005 all say "the template is rejected"; PKG-007 covers only "not installed" at instantiation.
   - Behaviours: re-validate at install or at instantiation, or never. "Never" lets a model-work step into a setup graph.
   - Layer: language.
   - Fix: state that instantiation re-runs every template check.

29. S4. The replacement Conduit.
   - Promises: CON-010 "an actor that holds the right to open it ... asks for a new Conduit ..., which is created ... and then opened"; LIF-009 "authorized as creating that instance would be"; CON-004 (a registered declared Conduit is opened at setup).
   - Questions: one request or two; authorized by `access`, `management` or both; may a later setup open the registered replacement.
   - Layer: convention.
   - Fix: say which.

30. S4/S10. Reconcile.
   - Promises: OPS-014 "an actor whose open Conduit lists reconciling"; OPS-019; ACC-033 omits reconcile; LIF-018 "reconciles the retire operation"; PLC-011.
   - Questions: a Conduit to which target (the invocation's target, the owning Pool, or `management`)? Which invocation is reconciled for an instance retired by declare-lost, which had no retire invocation of its own?
   - Layer: convention.
   - Fix: add reconcile to ACC-033 and define the lost case.

31. S9. Scheduled start after a close, then a new Conduit.
   - Promises: IMP-006 "that actor's access permits it at that moment"; ACC-019 "as it stands when the work runs"; both observables speak of "the actor's Conduit".
   - Behaviours: the schedule is tied to the Conduit that arranged it and is dead for good; or it revives when the actor again holds any Conduit listing the operation.
   - Layer: convention.
   - Fix: bind arranged work to the arranging Conduit's ID.

32. S5. `root` closes the delegator's right.
   - Promises: ACC-030's observable, "no actor holds a right ... outside the bound of the actor that opened it for them", read as a standing invariant, requires the downstream right to end; LIF-023 allows no implicit close.
   - Behaviours: no cascade; or the downstream right silently stops working.
   - Layer: convention.
   - Fix: reword ACC-030's observable as true at the time of opening.

33. S10. Locations.
   - Does "exactly one location is registered" (PLC-002 to PLC-004) count retired or lost locations, which stay registered (LIF-007)?
   - May an instance be placed on a location that is retired or not yet ready? PLC-001 only requires that it be a location.
   - Is declaring `local` lost refused? LIF-022 says "retire or unregister"; PLC-011 and ACC-032 are silent.
   - Layer: language and convention.
   - Fix: three short promises.

34. S10. Shared host.
   - Promises: PLC-013 is worded only for "setup reaches the step"; CON-006; the definition of requirement, "must hold for a relationship or its use".
   - Questions: which reason an explicit open of a standalone Conduit reports; open or not when a host fact is unknown; is the requirement checked again per request if a location later reports another host.
   - Layer: convention.
   - Fix: state all three.

35. S3. May a graph that implements an operation use `source: runtime`?
   - Promises: GRA-010 "may refer only to the operation's declared inputs, to its own steps' outputs and to handles it was given"; GRA-020 restricts only `input` and `config` by graph kind.
   - Layer: language.
   - Fix: reconcile the two.

36. S3. ACC-015 speaks of "the capability the policy governs", but a Requirement contract (CTR-005) names no capability.
   - Behaviours: engines block all, some or none of the target's capabilities.
   - Layer: convention.
   - Fix: add the field or drop the phrase.

37. S6. Revision checking.
   - REV-002 does not say the check is atomic with the store. DL9 lets an engine run controls concurrently, so two callers at the same revision can both apply.
   - Nothing says a change request with no revision is invalid; LIF-017 says "like any change" in passing.
   - Layer: convention. Partly covered by DL9.
   - Fix: "checked at the moment of storing", and "a missing revision is invalid".

=== MINOR ===

38. Empty placement requirements: PLC-014 accepts "an empty list"; CTR-004 says "writes an empty map". (Language.)
39. A plain server's output fails validation: IFC-010 "that table and nothing else" gives succeeded; OPS-007 gives unknown. (Interface.)
40. Tool-name collisions that IFC-014 does not prevent. (Interface.)
   - Two operations with the same export name from different packages in one capability (IFC-001, IFC-012).
   - Components of a declared composite Conduit (CON-003).
   - A rename onto a declared Conduit's name.
41. Generated names (OBJ-003): an exported template's name may contain dots (NAM-004) or leave no room within 63 characters, and no truncation rule exists. (Language.)
42. Retire order: LIF-005 with CON-008 closes the instance's own outbound Conduits at dispatch, so a retire graph (TPL-009) cannot make onward requests. Timing is not stated. (Convention.)
43. POOL-007: setting up entry B directly while entry A, which it refers to, is not ready. Conflict or proceed is not stated. (Convention.)
44. OPS-019 says a management request "carries an operation identity"; OPS-011 allows none. Whether its absence is invalid is not stated. (Convention.)
45. Breaking versions: "a breaking release takes a new address" is not checked. PKG-001 accepts 2.0.0 at the same address, and compatibility within `.v1` is undefined. (Language.)
46. PKG-018 lets a package document refer to a non-exported sibling, but NAM-003 makes local names resolve only among registered objects and NAM-013 registers nothing. The in-package reference form is undefined. (Language.)
47. Instantiating a Conduit whose target or consumer is already retired: CON-008's trigger has passed, so creation and a later open are undetermined. (Language and convention.)
48. Who may install a package is unstated. DL7 covers storage and fetch, not authorization, and install is absent from OPS-019's verbs.
49. A plain `url` server shared by several instances receives no `conduit.dev/instance` (IFC-007) and cannot keep PKG-016. DL12 lets engines share it.
50. The syntax of documents, the manifest (file name and keys) and the diagnostic and resolved-object formats is absent. The README acknowledges this as "still to come", but the language level cannot be tested until it exists.

=== Traced with no guess ===

- Document-level validation: names, reserved names, system-owned fields, YAML restrictions, the schema profile, input and argument assignment (OBJ-001 to OBJ-004, OBJ-012 to OBJ-019, SCH-*, INS-003 to INS-010, INS-019).
- Submission as an unordered, all-or-nothing, create-only set (SUB-001 to SUB-007).
- Default placement while every location is live (PLC-002 to PLC-004, PLC-016).
- Nothing is set up or opened by registration (INS-016, SUB-006).
- A stale revision reports conflict; a partial control input leaves omitted settings; the immediate state after an unknown control (REV-002, REV-006, REV-009).
- Same identity sent twice or with different contents by one caller; no resend after unknown on a non-deduplicating operation; restart mid-invocation gives unknown (OPS-009 to OPS-013, IMP-007).
- After a close the next request is refused, the tools disappear and setup does not reopen (ACC-011, CON-010, CON-014, IFC-013, LIF-008).
- A bounded holder opening inside and outside its Pool, and a wider pass-on refused (ACC-006, ACC-007, ACC-029, ACC-030).
- Unregister refused while shutdown is unknown; referrers report the object missing (LIF-017 to LIF-019, OBJ-010).
- Status reporting for an unreachable location (PLC-010, LIF-010, CON-007).
- Model work called directly, through a marked graph, or rejected in a setup graph (IMP-001, IMP-004, IMP-005).
- Core references without a manifest, replacing core rejected, exact-version matching (NAM-020, NAM-021, PKG-003).

=== Judgement ===

Not yet. I could build a validator for the individual rejection rules today, but not a resolver that a second team would match object for object. The three biggest obstacles:

1. The concrete surface is missing. The schemas of documents, contracts and manifests, the reference syntax, and the whole core package are absent, so no real project can be written the same way twice (findings 12, 16, 50).
2. Nested instances have no defined identity. The names of declared Conduits, entry Resources and components, how to refer to an entry from outside its Pool, and whether replacements follow the old or the current template are all open (findings 15, 26).
3. Several "readable from documents" answers depend on undefined semantics: how any requirement is evaluated, placement against location facts included; where a step's operation is dispatched; and which package version's contract governs when a package is absent or when two versions meet (findings 17, 18, 27, 28).

For layers 2 and 3 the sharpest problem is that the wire carries neither restrictions nor the link from an onward request to its caller (findings 1, 2, 13). The access convention's strongest promises cannot be implemented interoperably until it does.
```
