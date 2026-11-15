# L02-lifecycle: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The lifecycle itself is clear, minimal and rightly layered: three forward-only states, retirement at once, removal when shutdown is confirmed, each following a named Kubernetes precedent. The core of revisions.yaml (REV-001 to REV-007, REV-011, REV-015) is a sound use of the HTTP entity-tag paradigm. The weakest point is reconciliation used as an act that changes state. REV-010, REV-013 and LIF-028 let reconcile.v1 store configuration, open a Conduit, make an instance ready or remove it on an actor's word, under an authority separate from the operation it completes; the slice's two unsafe paths (F1, F2) and most of its machinery come from there. I would cut it to "record the outcome, then send the request again", because a control, an open, a setup and a retirement are each already safe to send again. The second weakness is form: `observable` holds 6,870 words against 1,492 in `then`, and carries obligations, restatements from other files and commentary; LIF-013 is the output contract of inspect.v1 and belongs in the core package, not among the conventions. Third, four devices guard one concern, a name or a relationship coming back by accident: names as the only address, `expectedId`, tombstones with lift, and register-replacement. For a declared Conduit a replacement always needs a lift first, which the stated limits accept, yet the guards still leave replace.v1 and declare-lost.v1 open to a stale request (F3). Last, the results that matter most to an agent caller are terse: the conflict of REV-014 names nothing, and an unregister succeeds without saying what came of its last stop.

**What the reader read.** Reviewed at commit 2eb8f258 (2026-11-12), read-only; nothing was built or run, and no vector or engine behavior was checked (two greps of engine/ only, to see how revisions are allocated). Read in full, line by line: spec/conventions/lifecycle.yaml (876 lines, 38 promises: 36 convention, 2 language) and spec/conventions/revisions.yaml (281 lines, 14 promises, all convention); the `vectors` field of all 52 is empty. Read in full: spec/definitions.md (467 lines) and docs/guides/full-review.md; the "What holds" section of AGENTS.md. Read in part: spec/README.md lines 1-105 (what this is, the three layers) and 272-618 (form of a promise, conformance, limits of this version), not the reading paths, contents table or closing sections; docs/AUDIT.md "Known to be open" (lines 174-245); docs/guides/cold-readers.md lines 39-173; IN_FORCE.md topics 4 (pending lines), 6, 7 and 8, plus a grep of every Open and Spec pending line (long lines truncated). Cited promises opened in full: OPS-002, 003, 004, 008, 013, 014, 021, 022, 023, 027, 028, 029, 030, 031, 032; ACC-006, 011, 012, 033, 034, 039, 053, 055; CON-004, 008, 031; TPL-008, 009, 012, 018; PLC-017, 024, 026, 027; PRV-008, 009, 020, 021, 029, 030; POOL-003, 004, 017; OBJ-002, 003; SUB-009; CTR-006; IFC-006 (head), IFC-050, part of IFC-025; IMP-007; ENR-007; PND-003, 006, 010, 011. Core package opened: spec/core/operations.yaml lines 183-312 and 405-752 (replace.v1 through retire-with-resources.v1), tombstone.v1 in spec/core/results.yaml, revision.v1 in spec/schemas/values.yaml, and "What this package does not settle" in spec/core/README.md; DL1, DL3, DL10 and DL15 in spec/implementation-defined.yaml. Cited by the slice and not opened: PLC-011, 018, 023; PRV-005, 006, 007, 015, 019, 026, 027; CON-011, 025; ACC-005, 024, 027, 044, 056; IFC-012; OBS-003; REC-001, 002, 004; INS-002, 020; PKG-005, 007; CTR-014; PND-002, 004, 007; OPS-009, 012, 020, 024, 026, 034, 036; TPL-017; POOL-013. Checked for prior record and found none: F1 to F5, F7, F8, F10 (G5.md, the increment 4 log, the review list, plan-and-apply's open points). F6 and F9 extend stated limits with a situation they lack. Considered and not reported: MC6-001 and MC6-002 (settled by order 081), the keys of the inspection result (known open), and the cascade's outcome failed (consistent with the stated effect of retire-with-resources.v1).


## FR-010

**Reconciling an unknown control as having taken effect stores its old values now, on the reconciler's word, without the control's Conduit and over later changes**

L02/F1 · defect · unsafe · verifier: confirmed, high · decides: owner · new

**Promises.** REV-010, REV-013, REV-009, ACC-055, ACC-033, OPS-031

**Where.** spec/conventions/revisions.yaml:141

**Quote.** "After a reconciliation that stores values, the stored configuration equals the current configuration with the values the control was sent applied at those pointers, whatever other change was stored after the control was dispatched."

**Situation.** Steps to the harm. (1) X holds a Conduit to Resource R that lists a control, and a Conduit to R's Pool that lists reconcile.v1 and inspect.v1. (2) X sends the control with limit 20 and drops the stream: the control ends unknown (OPS-027, spec/conventions/operations.yaml:609) and nothing is stored (REV-009). (3) An administrator closes X's Conduit to R and sets limit 10 with its own control. (4) X sends reconcile.v1 through the Pool, with occurred true and the current revision. ACC-033 row 14 names the Pool; ACC-055 passes it because X sent the control (spec/conventions/access.yaml:878); REV-010 says the request 'is not authorized by a Conduit to the instance itself, as the control was' (revisions.yaml:156). (5) The engine stores 20 over 10 and sends 20 as R's configuration with every later call (IFC-006). X has changed R after its access to the control was closed, and no implementation was called. The same rule misreports without bad faith: control A (20) ends unknown, B stores 10, A is truthfully recorded as having occurred, and the stored value returns to 20 although the implementation last applied 10. Where B makes A's values fail the schema, REV-013 leaves A unknown for good, because the only other answer, that it did not occur, is false.

**Counterargument.** The reconciler must name the current revision, so it acts knowing the present state. ACC-055 limits reconciliation through a Pool or Fleet to the sender or root, and the sender was permitted the control when it sent it. The record names the reconciler and what it checked. The stored configuration ought to follow what the implementation applied.

**Least change.** REV-010, then: 'A reconciliation of a control records the outcome and stores nothing, whichever way it records the effect, and names no revision. The settings the control addressed are reported as unknown until a later control sets them: the way on is the control sent again, with the current revision.' REV-013, the `revision` input of reconcile.v1 (OPS-031) and the control's values kept under OPS-008 then go. A smaller patch keeps the storing only where the reconciler is also permitted the control now and no change was stored since the control was dispatched.

**Precedent.** HTTP conditional PUT (RFC 9110, If-Match): after an indeterminate answer the client reads again and sends the PUT again. Kubernetes does the same on an update conflict. Neither stores a value on a client's statement that an earlier write landed.

**Who decides.** owner. The fix removes or narrows ruled behavior (the rulings behind REV-010 are listed at IN_FORCE.md:137); a fix that removes a feature is the owner's (rulings 37 and 46 point 4 limit the coordinator to fixes that add no behavior).

**Verifier.** confirmed, high. Quote verbatim at spec/conventions/revisions.yaml:141-143. Each step holds in the text. spec/conventions/operations.yaml:616-618: a dropped stream "cancels the invocation, whose outcome is then unknown", so X can make the unknown outcome itself. revisions.yaml:186-188 (REV-012) holds back another change only while one "has been dispatched and has not ended", so the administrator's control B is dispatched and stored. spec/conventions/access.yaml:625-627 (row 14) names the Pool; access.yaml:878 "Through a Pool, the invocation must have been sent by that consumer"; revisions.yaml:156-157 "It is not authorized by a Conduit to the instance itself, as the control was." revisions.yaml:132-135 then stores the kept values. Nothing else in spec/ requires a current Conduit to the instance or refuses the store when a later change was stored; REV-013 (revisions.yaml:219-220) only leaves the control unknown. The honest case is worse than stated: REV-009 (revisions.yaml:116-117) keeps the setting reported unknown after B succeeds, and the only exits are to store A over B or to record falsely that A did not occur. Not in docs/AUDIT.md 174-246, cold-readers.md, the README limits, IN_FORCE.md, the increment logs or the review list. experiments/012-model-checking/G5.md:292 gives "REV-010 ... holds" for the text as written, and docs/implementation/SAFETY_CATALOGUE.md:515 considers only substituted values.

**Verifier's corrections.** known: known-new-situation, not new. That a reconciled control stores is ruled (docs/proposals/reaudit/README.md:69 Q4 and :150 F11; docs/proposals/ruling-log.md:472, ruling 41 item 15; the Pool route with its sender limit is ruling 66 point 4, docs/proposals/ruling-66/README.md:33-35). The two harm paths are in no record. Unsafe stands, and rests best on the second path: a truthful record puts an older value over a later one with no call to the implementation. The first path needs X to keep a Pool Conduit listing reconcile.v1, which Pool inspection shows as an administrator (lifecycle.yaml:260-262). The reach is wider than stated: through `management` there is no sender limit (access.yaml:888-889), so for a control of an Interface, Pool or Fleet any holder of reconcile.v1 stores another actor's values. Owner decides, as stated.


## FR-011

**Recording an unknown open as having occurred opens the Conduit with none of the checks an open makes at that time, a matching tombstone included**

L02/F2 · defect · unsafe · verifier: confirmed, high · decides: coordinator · new

**Promises.** LIF-028, LIF-041, ACC-006, ACC-055, IMP-007

**Where.** spec/conventions/lifecycle.yaml:597

**Quote.** "Recording that a setup or an open occurred makes the instance ready or the Conduit open, unless the instance, or the Conduit's consumer, has been retired."

**Situation.** Conduits X and Y both run from Agent A to Resource R for one capability; Y is open and X is registered. open.v1 for X is in progress when the engine restarts, so it ends unknown (IMP-007, spec/conventions/implicit.yaml:75) with X still registered. An administrator closes Y to end A's access. Y's tombstone matches X ('the same target and capability', lifecycle.yaml:863), so open.v1 for X now reports conflict. A holder of a Conduit to `access` that lists reconcile.v1, whose bound covers R (ACC-055) and which does not list lift.v1, records the open as having occurred. LIF-028 makes X open. LIF-041's `when` names only 'open.v1 or a setup part' (lifecycle.yaml:858), so the tombstone is never consulted, and A has its access back with no lift. The engine implements open.v1, so whether it occurred is the engine's own state: the record does not report an open, it performs one, under reconcile.v1 alone.

**Counterargument.** An unknown open is rare, because the engine implements open.v1 and usually knows what it did. The reconciler needs authority through `access` whose bound covers the target and every operation (ACC-055), which is close to the authority to open. The tombstone belongs to a sibling Conduit, not to X.

**Least change.** LIF-041, when: add 'or a reconciliation would make a Conduit open under LIF-028'. Simpler still, in LIF-028: 'Recording that an open occurred records the outcome and changes nothing else; a Conduit that is still registered is opened by open.v1 again, which succeeds on an open Conduit under ACC-006.'

**Precedent.** Idempotent retry of the same request, as open.v1 already allows (ACC-006); in Kubernetes the store's committed state, not a client's statement, says whether a write happened.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect whose fix adds a check and no feature; logged for the owner to strike. Dropping the state change from LIF-028 altogether would be the owner's.

**Verifier.** confirmed, high. Quote verbatim at spec/conventions/lifecycle.yaml:597-598. LIF-041's `when` (lifecycle.yaml:857-859) is "open a Conduit by `open.v1` or a setup part under ACC-005", and the match rule is at :862-863. A search of spec/ for tombstone beside reconcile finds only spec/core/capabilities.yaml:101 (the bound) and lifecycle.yaml:616-618 (provenance kept, tombstone preserved on a close); nothing applies LIF-041 to a reconciliation. ACC-055 (access.yaml:876-878) checks bound coverage only. ACC-053 makes lift separately withholdable (access.yaml:843-844 "so a bound can withhold it"). The scenario is reachable: two Conduits of one consumer to one target are allowed (operations.yaml:542 "another Conduit of the same consumer to the same target"; lifecycle.yaml:870-871). spec/conventions/implicit.yaml:75 gives unknown after a restart, and operations.yaml:626-630 and :637 give a cancelled engine part the outcome unknown, so an actor can produce an unknown open by cancelling a setup. Not recorded: docs/AUDIT.md:240 lists only how a bound judges reconcile.v1 through `access`, since written as ACC-055 (docs/proposals/projects/brief.md:2251).

**Verifier's corrections.** Class and unsafe mark stand. The path is narrow: it needs a sibling Conduit with the same consumer, target and capability, and an unknown open. If LIF-041's words are read to cover a reconciled open.v1, LIF-028 (must open) and LIF-041 (must report conflict) contradict; either way two builders diverge. With the first fix, the reconciliation should report conflict with LIF-041's diagnostic, as REV-013 reports invalid for a control. Who decides is as stated: coordinator for the added check, owner for dropping the state change.


## FR-012

**A revision is unique only within one object, and replace.v1 and declare-lost.v1 take no expectedId, so a stale request can pass against a later object of the same name**

L02/F3 · gap · unsafe · verifier: weakened, medium · decides: coordinator · new

**Promises.** REV-001, REV-002, REV-005, LIF-037, LIF-040, SUB-009, PLC-017, PLC-026

**Where.** spec/conventions/revisions.yaml:64

**Quote.** "It takes a value the object has had before."

**Situation.** REV-005 keeps a revision from repeating within one object. Nothing says a later object of the same kind and name cannot have a value the earlier one had, and an engine that counts revisions per object conforms. A template gets no tombstone (LIF-040's `when` names Conduits and other instances, lifecycle.yaml:830), so its name is free as soon as it is unregistered. Case 1: operator A plans a replace of template `reviewer` at revision '2'. Operator B unregisters it, registers a new `reviewer` and replaces it once, which gives revision '2'. A's replace.v1 passes REV-002 and overwrites B's template; instances made afterwards get the stale document's implementation items. replace.v1 has no `expectedId` (spec/core/operations.yaml:219-228; LIF-037's list, lifecycle.yaml:797). Case 2: a node is named from its hostname and gets no tombstone (PLC-026; spec/promises/placement.yaml:359). A declare-lost.v1 that names `buildbox`, sent after the old node was unregistered and the machine admitted again, retires the new node and every instance on it. It takes no `expectedId` either (spec/core/operations.yaml:682-686).

**Counterargument.** An engine may make revisions unique everywhere, for example with UUIDs. Instances mostly cannot reuse a name without a lift (LIF-041). An actor with replace.v1 may replace any template anyway. The plan-and-apply reference calls the absence of expectedId on replace deliberate (docs/implementation/plan-and-apply/README.md:380), relying on the revision.

**Least change.** REV-005, then: 'It takes a value that the object, or an earlier object of its kind and name, has had.' LIF-037, when: add `replace.v1` and `declare-lost.v1`, or say that every core operation that names an object by a reference accepts `expectedId`.

**Precedent.** Kubernetes: resourceVersion comes from one store-wide counter (the etcd revision), so a recreated object never matches a stale one, and preconditions.uid is accepted on every update and delete.

**Who decides.** coordinator. Ruling 46 point 4 for the wording of REV-005 (it constrains engines and adds no behavior); ruling 71 point 4 for one optional input on two operations.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/revisions.yaml:64; REV-005 speaks of one object (:62-66), and REV-001 (:18-20) makes a revision opaque. No text in spec/ makes revisions unique across objects (spec/definitions.md:386; spec/implementation-defined.yaml has no item on it). replace.v1's inputs are template, revision, contracts and manifest (spec/core/operations.yaml:219-228), and SUB-009 (spec/promises/submissions.yaml:182-192) checks kind, name and revision only. A template gets no tombstone (lifecycle.yaml:830). declare-lost.v1 takes `node` alone (core/operations.yaml:682-686). spec/promises/placement.yaml:359-360: "A node made by the engine gets no tombstone (LIF-040), so the new node may take its hostname again." LIF-037's list is at lifecycle.yaml:797.

**Verifier's corrections.** known: known-new-situation. Round 6's D12 recorded the reuse of a name by a later object and called it "an identity-selection usability problem, not a demonstrated violation" (docs/proposals/reaudit/round-6/design-critique.md:381-384); the owner accepted an expected ID for the lifecycle actions (round-6/DECISIONS.md:116; docs/AUDIT.md:506, A69). New here: a revision is unique per object only, so replace.v1's revision does not stand in for the ID, and declare-lost.v1 has neither check. Unsafe mark: holds narrowly for case 1 only (a second operator's replace is overwritten without conflict when per-object counters coincide, which defeats REV-002). Case 2 is not unsafe by the tracker's test: the operation is reserved to root, the stale request must cross an unregister and a new admission, and a repeat carrying its idempotency key is answered without a dispatch. Add register-replacement.v1, whose `instance` has neither a revision nor an expectedId. Coordinator decides both fixes, but the expectedId extends a list the owner accepted in D12, so log it to strike; docs/implementation/plan-and-apply/README.md:379-380 calls the absence deliberate and needs the same change.


## FR-013

**LIF-019 forbids erasing backing files when an instance is removed; LIF-035 and PRV-029 erase its state directory; LIF-019's two terms are defined nowhere**

L02/F4 · contradiction · verifier: confirmed, medium · decides: coordinator · new

**Promises.** LIF-019, LIF-035, PRV-029

**Where.** spec/conventions/lifecycle.yaml:391

**Quote.** "Records, backing files or service data are erased, or any object that refers to it is changed, retired or unregistered, apart from the Conduits of which the removed instance was an end, which CON-008 closes."

**Situation.** An Agent on the engine's own runner is retired and the retirement succeeds. LIF-035 removes it at once and says (lifecycle.yaml:733) that removal 'removes its state directory and contents under PRV-029' (spec/promises/providers.yaml:975). That directory is the Agent's home, with its client state and login (spec/definitions.md:279). LIF-019, a must not with the same `when`, forbids erasing 'backing files or service data'. Neither term is in definitions.md, and neither appears anywhere else under spec/. One builder reads the state directory as the instance's backing files and keeps it, leaving a removed Agent's login on disk; another deletes it. The `effect` of retire.v1 (spec/core/operations.yaml:485-488) does not mention the deletion, so a caller is not told that a successful retirement erases the home.

**Counterargument.** The state directory is the runner's private scratch for one instance, not the data a Resource serves, such as the files of a directory Resource. PRV-029 is the more specific rule, and LIF-035 cites it.

**Least change.** LIF-019, then: '... are erased, apart from the instance's state directory, which PRV-029 removes, ...'. Define the two terms in definitions.md as what the instance's configuration names outside the engine, or replace them with those words. Add the removal of the state directory to the `effect` of retire.v1 and unregister.v1.

**Precedent.** Kubernetes names the two classes: an emptyDir volume is deleted with its Pod, and a PersistentVolume is kept or deleted by its stated reclaim policy.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix adds no behavior.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/lifecycle.yaml:391-393, `must not`, with the `when` "An object is unregistered or removed" (:389). lifecycle.yaml:733-734: "Removing or unregistering an instance removes its state directory and contents under PRV-029." spec/promises/providers.yaml:974 (PRV-029 `then`): "Removal or unregistration of the instance removes it with its contents (LIF-035)." providers.yaml:981-983: "Agent packages use it for the Agent's own home; their client-specific subdirectories and credential storage belong to their contracts." "backing file" and "service data" occur nowhere in spec/ but lifecycle.yaml:391. The `effect` of retire.v1 (spec/core/operations.yaml:484-487) and the description of unregister.v1 (:248-256) do not mention the deletion. `git log -S` shows the sentence at :733 came with order 038a (d08fb8bb), which left LIF-019 alone; order 081 narrowed LIF-019 for Conduits only (MC6-002). Not in any known-open record.

**Verifier's corrections.** Citation: spec/definitions.md:279 says only "removed with the instance"; the home and the login are at providers.yaml:981-983 and spec/README.md:410-412. The divergence is overstated: PRV-029's `then` and definitions.md:279 decide that the directory is deleted, so a careful builder deletes it. What stays open is what else "backing files or service data" covers, and that a caller of retire.v1 is not told the home is erased. Class stands on the literal text, as MC6-002 was classed. Coordinator decides, as stated.


## FR-014

**An unregister succeeds without saying that its last stop failed, ended unknown or was never sent**

L02/F5 · defect · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-198, FR-232.

**Promises.** LIF-036, LIF-017, IFC-050, IFC-025

**Where.** spec/conventions/lifecycle.yaml:765

**Quote.** "the request to unregister asserts that the instance's shutdown is settled in another way, and the outcome of the request is that of the removal."

**Situation.** An instance is retired with its shutdown unknown because its machine was cut off. An actor unregisters it. The runtime provider's server is not ready, so the last stop is not dispatched (lifecycle.yaml:763-769). The instance is removed and the request succeeds. unregister.v1 declares no outputs (spec/core/operations.yaml:258-269), so its result has no structuredContent (spec/definitions.md:222), and IFC-050 asks for a diagnostic only on a reason or on a failed or unknown outcome. The caller is told that the request succeeded and nothing more, although the run may still hold its secrets and the engine 'asks for no further stop once the instance is removed' (lifecycle.yaml:766-767). Only the record of the provider call shows what came of the stop.

**Counterargument.** The request is itself the statement that the shutdown is settled in another way. The record of the stop is kept and names the instance, and whoever may inspect through `management` can read it.

**Least change.** LIF-036 and unregister.v1: an output `stop` with one of succeeded, failed, unknown, not-dispatched or none, and, where it is not succeeded, a warning diagnostic that names LIF-036 and says the run may still be running and no further stop will be asked.

**Precedent.** kubectl delete --force --grace-period=0 warns that the resource may continue to run on the cluster indefinitely.

**Who decides.** coordinator. Ruling 52 point 2 (every result says what happened and why) and ruling 71 point 4 (a small finding that adds an output).

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/lifecycle.yaml:765-766; :769 "A stop that ended unknown or failed, or was not dispatched, does not prevent removal." unregister.v1 has no outputSchema (spec/core/operations.yaml:258-269). spec/interface/implementation.yaml:669-670: "Where it declares no outputs, the result carries no `structuredContent`, except for error diagnostics"; :1343-1345 asks for a diagnostic on a reason and on "An engine-decided failed or unknown outcome". So a successful unregister has no place for a word about the stop; only the record of the provider call holds it (spec/promises/providers.yaml:145-146). The stated limit (spec/README.md:592-593, "So a stop that the provider never received ... is lost") states the risk and not what the caller is told. Not in the logs, G5, the review list or IN_FORCE.md.

**Verifier's corrections.** None on class, unsafe mark or who decides. IFC-050's `then` (implementation.yaml:1339-1340, "says what happened and why in full") could be read to ask for it, but IFC-025 leaves no place for a diagnostic that is not an error on an operation with no outputs, so the fix must declare an output. The nearest existing pattern is register.v1's declared `diagnostics` with note-level entries (NAM-008).


## FR-015

**Two requests assert that a shutdown is settled; only unregistering asks the runtime provider for a last stop**

L02/F6 · gap · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** LIF-018, LIF-028, LIF-035, LIF-036, LIF-017

**Where.** spec/conventions/lifecycle.yaml:376

**Quote.** "After a reconciliation that records the retirement as having occurred, the shutdown is confirmed and the instance is removed, under LIF-035, as LIF-028 states. Whether the run was stopped, the actor can tell from the condition `Running`, under PRV-009."

**Situation.** The engine restarts while an instance's retire hook is in progress. The retirement ends unknown (IMP-007) and its stop was never asked. An actor who believes the work is done records the retirement as having occurred. LIF-028 confirms the shutdown, and LIF-035 removes the instance and its state directory. No stop is asked: LIF-036's `when` is a request to unregister (lifecycle.yaml:742-744). Had the actor made the same assertion by unregistering, the engine would have asked once more. After removal the engine keeps no state of the run. The stated limit (spec/README.md:582-605) names a declared loss, a machine reported gone and an unregister, and not this path.

**Counterargument.** The actor states, with what it checked, that the retirement occurred, the stop included. The careful way on is a further retirement under LIF-026, and the condition Running shows the run's state.

**Least change.** LIF-036, when: add 'or a reconciliation that records a retirement as having occurred is about to remove the instance, under LIF-028'. The alternative is to drop confirmation by reconciliation for a retirement, leaving a further retirement and unregistering as the two ways on.

**Precedent.** Kubernetes has one forced path, removing the finalizer or force deletion, and LIF-017 already follows it.

**Who decides.** coordinator. Ruling 71 point 4: a small finding that adds one provider call; logged for the owner to strike. Dropping confirmation by reconciliation would be the owner's.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/lifecycle.yaml:376-378. LIF-028 confirms the shutdown (:599) and LIF-035 removes the instance and its state directory (:723, :733). LIF-036's `when` is "A request to unregister an instance is dispatched" (:742-744). The stop follows the hook (spec/promises/templates.yaml:147-151), so a restart during the hook leaves it unasked (spec/conventions/implicit.yaml:75). operations.yaml:341-342 allows the reconciliation "although the hook's outcome is still unknown", and :345-346 says what the actor checks "may be out of date". The stated limit (spec/README.md:582-605) names a declared loss, a machine reported gone and an unregister, and not this path; neither G5.md nor IN_FORCE.md:245 has it.

**Verifier's corrections.** Class: not a gap. LIF-028, LIF-035 and LIF-036's `when` decide the situation: the instance is removed and no stop is asked. It is a defect by omission: two requests assert a settled shutdown and only one carries the safeguard. The spec puts the check on the actor (lifecycle.yaml:377-378; operations.yaml:339-341), which the finding should say. known-new-situation is right. Unsafe false agrees with how the limit treats the unregister path, though here the stop may never have been asked at all on a machine that can be reached. Coordinator under ruling 71 point 4 is sound for one more provider call.


## FR-016

**REV-014's conflict need not name the invocation in the way, nothing shows a setup, retirement or change in progress, and only its requester can end it**

L02/F7 · defect · verifier: confirmed, medium · decides: coordinator · new

**Promises.** REV-014, REV-012, LIF-013, IFC-050, OPS-027

**Where.** spec/conventions/revisions.yaml:238

**Quote.** "The engine neither queues the request nor waits for the other to end: the result is returned at once, the implementation receives no call, and the instance and its revision are as they were. The caller asks again once the other has ended."

**Situation.** Actor X's setup of an Agent hangs in its setup hook. Root asks to retire the Agent and gets conflict with the ruleId REV-014. Nothing obliges the diagnostic to name the invocation in the way, its operation or its caller. The status lists only invocations whose outcome is unknown (LIF-013), so one in progress appears nowhere but in the records. Only X can cancel it, by the protocol's cancellation of the request that dispatched it (OPS-027). Root can only ask again until the engine's own time limit ends the hook (DL10, spec/implementation-defined.yaml:83-84). The head of the file says retirement 'takes effect at once', as a Kubernetes deletion does (lifecycle.yaml:4-6); here it waits behind any setup or control.

**Counterargument.** IFC-050 already asks for the cause in words. The record of the dispatch names the invocation for whoever may inspect. Closing Conduits and declaring a node lost are never held back, so access can still be ended at once.

**Least change.** REV-014, then: '... reports conflict at once; its error diagnostic names the engine's ID, the operation and the caller of the invocation in the way.' LIF-013: the status also lists a setup, retirement or change in progress.

**Precedent.** Terraform's state-lock error prints the lock's ID, the operation, who holds it and since when. OPS-032 already returns the invocation's ID on its own conflict.

**Who decides.** coordinator. Ruling 52 point 2 and ruling 71 point 4. Whether a retirement should wait behind a setup at all is behavior, and the owner's.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/revisions.yaml:238-240. IFC-050 (spec/interface/implementation.yaml:1339-1349) asks for "the operation, what the request named, the reason or outcome, the deciding promise's ID and the cause in words" and adds "Wording is the engine's choice", so nothing obliges naming the invocation in the way. Other conflicts do name what is in the way: lifecycle.yaml:800-801 (LIF-037, both IDs), :412 (LIF-020, each member), :867-868 (LIF-041, the tombstone's ID), operations.yaml:783-784 (OPS-032, the invocation's ID). `unreconciled` lists only unknown outcomes (lifecycle.yaml:242-245). Cancellation is by the request's own protocol means (operations.yaml:604-606). spec/implementation-defined.yaml:28-29: "No promise limits how long an invocation is in progress"; the time limit is at :83-84.

**Verifier's corrections.** known: known-new-situation. Round 6's D10 asked that the ruleId, the invocation's identity and the status be enough to tell the kinds of conflict apart (docs/proposals/reaudit/round-6/design-critique.md:330-337), accepted for S4 and S5 (round-6/DECISIONS.md:114); it does not have REV-014's diagnostic or the lack of a way to end another actor's invocation. Smaller on one point: an inspector does see an invocation in progress, as a record of its dispatch with no end, in the `records` output (spec/conventions/records.yaml:17-18), unless a purge removed it. That a retirement waits behind a setup is ruled (IN_FORCE.md:169), so that part is the owner's, as stated.


## FR-017

**A setup whose hook fails after its Conduits opened ends unknown, stays in `unreconciled` after a later setup succeeds, and has no true answer to reconcile**

L02/F8 · gap · verifier: weakened, medium · decides: owner · new

**Promises.** LIF-013, LIF-028, LIF-004, OPS-022, OPS-013, PND-006

**Where.** spec/conventions/lifecycle.yaml:242

**Quote.** "It also lists, in `unreconciled`, the engine's ID for each invocation whose outcome is unknown, that was dispatched to the instance, its setup hook and its retire hook among them, or that names it in an input or as its target, its setups and retirements among them."

**Situation.** An Agent with one declared Conduit is set up. The Conduit opens, the run starts and the setup hook fails. OPS-022 gives 'failed only if no part had an effect, and otherwise unknown' (spec/conventions/operations.yaml:465-466), so the setup is unknown although the outcome of every part is known. The cause is fixed and a second setup succeeds: the Agent is ready. The first setup stays unknown (OPS-013) and stays in `unreconciled` until someone sends reconcile.v1. LIF-028 offers two answers and both are false: that it occurred (the hook did not succeed; given before the second setup, this answer would have made the Agent ready without it), and that it did not (the Conduit did open). So each ordinary hook failure costs a reconciliation by hand, and the list meant for what nobody can tell (spec/definitions.md:377) fills with what everybody can.

**Counterargument.** OPS-004 deliberately puts an effect that occurred only in part under unknown, and order 081 kept that reading for a retirement whose hook failed (MC6-001). PND-006 shows only the latest setup, so the leftover is confined to `unreconciled`.

**Least change.** OPS-022, then: '... unknown only where a part ended unknown; where the outcome of every part is known and a part not marked optional did not succeed, failed, and the diagnostics name the parts that succeeded, which stay as they are under LIF-027.' A narrower patch, in LIF-013: such an invocation leaves `unreconciled` once a later setup of the instance succeeds.

**Precedent.** Terraform: an apply that fails part-way is an error, its state keeps what was made, and the next apply continues from there.

**Who decides.** owner. It changes what the outcome failed means for set-up.v1 and retire.v1 (OPS-003, OPS-022): behavior, which the owner rules.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/lifecycle.yaml:242-245. The situation follows from the text: operations.yaml:465-466 and :471-474 give unknown where "any part succeeded with an effect"; :299 keeps it unknown until reconciled; :326 lists it "until it is reconciled". set-up.v1 does not declare `deduplicates` (spec/core/operations.yaml:411-446), so under OPS-012 (operations.yaml:281-283) a repeat with the same key dispatches nothing: the second setup is always a new invocation and the first always stays behind. The rule itself is recorded: G5.md:357 (MC6-001); docs/work-orders/081-spec-one-run-per-instance.md:378-398, where the coordinator kept unknown for a failed hook beside a part that had an effect; docs/proposals/first-build/log-increment-4.md:23 (4-12, "v0 keeps `reconcile.v1` as it is", closed with the owner's agreement); round 6 D10 (design-critique.md:343-346, "OPS-022 may report unknown for partial effects"; DECISIONS.md:114).

**Verifier's corrections.** Class: not a gap. OPS-022 decides the outcome and LIF-013 decides the list; it is the cost of a ruled rule. known: known-new-situation, not new. The new situation is that the first setup stays in `unreconciled` after a later setup succeeded, and that a keyed client cannot retry at all. Also worth stating: spec/definitions.md:377 calls unknown "The one word for 'cannot tell'", while OPS-004 (operations.yaml:54-56) and OPS-022 use it for a partial effect that is known. Owner decides, as stated.


## FR-018

**The stated way to narrow access, close and then open a narrower Conduit, reports conflict under LIF-041 until a lift**

L02/F9 · contradiction · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** LIF-001, ACC-011, LIF-040, LIF-041, ACC-053

**Where.** spec/conventions/lifecycle.yaml:29

**Quote.** "No instance, a Conduit included, has a suspended state: to narrow access, its Conduit is closed and a narrower one opened, under ACC-011."

**Situation.** An actor holds a Conduit to `access` that lists open.v1 and close.v1 and not lift.v1. To narrow Agent A's read-write Conduit to R it closes it, as LIF-001 and ACC-011 describe. The close leaves a tombstone (LIF-040). The narrower read Conduit has the same consumer, target and capability, so registering it reports conflict (lifecycle.yaml:862-864) until someone whose Conduit lists lift.v1 lifts the tombstone. A has no access meanwhile, and this actor can only remove access, not narrow it. Registering and opening the narrower Conduit first avoids the lift, but it must take another local name, unique among the consumer's Conduits (spec/definitions.md:300), which changes A's tool names. No promise describes either route. The stated limits (spec/README.md:392-396) speak of recreating an instance and of replacing a declared Conduit, not of narrowing.

**Counterargument.** The limits already say that a matching tombstone needs a lift. The conflict's diagnostic names LIF-041 and says lift.v1 removes the tombstone. The plan-and-apply tooling lifts the tombstones its own apply caused.

**Least change.** LIF-001 and ACC-011: 'To narrow access, a narrower Conduit is registered and opened and the wider one is then closed; a Conduit made after the close needs the closed one's tombstone lifted first, under LIF-041 and ACC-053.'

**Precedent.** Make before break: in Kubernetes RBAC and AWS IAM the narrower binding is created before the wider one is deleted.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the procedure as written cannot be followed, and the fix is wording that adds no behavior.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/lifecycle.yaml:29-31; the same sentence is at spec/conventions/access.yaml:196-198 and IN_FORCE.md:20. LIF-041 (lifecycle.yaml:861-864) reports conflict on registering or opening a Conduit with the same consumer, target and capability, and :870-871 says another name does not escape it. A narrower Conduit has, by definition, the same consumer, target and capability, so every narrowing after a close meets the tombstone. The limits state the mechanism for recreation and for a declared Conduit (spec/README.md:392-394) and accept "a longer window without access" for replacement (:364-367). The plan-and-apply reference already has the tool close and recreate a narrowed Conduit and lift tombstones (docs/implementation/plan-and-apply/README.md:102, :113).

**Verifier's corrections.** Class: defect (a procedure stated incompletely in two observables), not a contradiction. Close, lift, register and open all hold together, and LIF-041 decides the outcome, so no two promises clash and no builder diverges. known-new-situation is right. Add to the fix IN_FORCE.md:20, which carries the same sentence, and the plain statement that every narrowing after a close needs a lift, since the match is certain. Coordinator decides the wording, as stated. Whether narrowing should cost a lift at all is the owner's, with finding 4-01.


## FR-019

**LIF-013 states the output contract of inspect.v1 in its observable; LIF-009, LIF-036, REV-010 and REV-014 are built the same way, and the core package states the same rules again**

L02/F10 · needless-complexity · verifier: weakened, medium · decides: coordinator · new

**Promises.** LIF-013, LIF-009, LIF-036, REV-006, REV-010, REV-014

**Where.** spec/conventions/lifecycle.yaml:237

**Quote.** "The result holds what was declared, the instance's status and its records, and, for an instance other than a Conduit, its inventory."

**Situation.** LIF-013's `then` names four parts of an inspection result. Its `observable` (lifecycle.yaml:240-290, 705 words) then states at least nine further obligations: `unreconciled`, what the inventory holds, its tombstones, that it is shown in full with no counts, an Interface's scope, an Agent's scope, a Pool's administrators, `status.pending`, and `ownTools` and `targetOwnTools`, the last with a disclosure rule (an inspector of the holder learns one value of the target's configuration). Across the two files `then` holds 1,492 words and `observable` 6,870. A rule that sets OBJ-015 aside sits in REV-006's observable (revisions.yaml:79-83); that a failed stop does not prevent removal sits in LIF-036's. The form asks for one obligation in `then` and for `observable` to say what can be checked (spec/README.md:292, 297). A builder cannot tell which sentences bind, and a single vector would count LIF-013 as covered. The same shape rules are written again in inspect.v1 (spec/core/operations.yaml:297-312), in inventory.v1 (spec/core/results.yaml:366) and in three rows of definitions.md, so four texts must be kept equal.

**Counterargument.** Definitions.md says the promises govern, so the other copies are summaries. Splitting would raise the count above 519. The observables help a reader by citing the neighbouring rules, and the IDs stay stable.

**Least change.** Keep LIF-013's `then`. Move the shape of the result to the contracts of inspect.v1 and inventory.v1, which a schema can check, and cite them. Give the two disclosure rules (the inventory in full, and `targetOwnTools`) promises of their own beside the access promises. Elsewhere, a sentence in `observable` that adds an obligation becomes a promise or moves to a contract.

**Precedent.** The Model Context Protocol and OpenAPI put the shape of a result in the operation's output schema; Kubernetes puts it in the type's schema.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a change of form that adds no behavior.

**Verifier.** weakened, medium. Recounted with /usr/bin/python3 and PyYAML: `then` holds 1,199 + 293 = 1,492 words and `observable` 5,332 + 1,538 = 6,870 across the two files; LIF-013 has 22 and 705. Quote verbatim at spec/conventions/lifecycle.yaml:237-238. The form is at spec/README.md:291-292 and :297 ("One obligation per promise"). REV-006's observable sets OBJ-015 aside (revisions.yaml:79-83); LIF-036's holds "does not prevent removal" (lifecycle.yaml:769). The same shape rules are in spec/core/operations.yaml:297-312 and spec/core/results.yaml:366. Recorded before: round 6, design-critique.md:593-596, "Keep one normative source for each rule; avoid three paraphrases ... Treat `observable` as part of the obligation review: several important decisions live there rather than in `then`"; D21 was accepted under ruling 44 and applied to the reading paths only (round-6/DECISIONS.md:125). docs/AUDIT.md:185-186 and spec/core/README.md:694-705 hold the shape of an inspection result as known to be open.

**Verifier's corrections.** known: known-new-situation, not new. The general point is round 6's D21. New here are the measure for these two files, LIF-013's separate obligations, and the two disclosure rules that sit in an observable. Class and unsafe mark stand. The coordinator may make the change (rulings 37 and 46 point 4), but a re-forming of this size across spec/ is a matter of the order of the work, which the owner rules; propose it with the count of promises it would add.


## Further points the verifier noticed

Not verified by a second reader.

- **Recording a setup as having occurred makes the instance ready although the engine's own state shows parts never begun, the account check among them** (spec/conventions/lifecycle.yaml:597). Same root as F2, for set-up.v1. LIF-028 has no condition on the parts. TPL-012 (spec/promises/templates.yaml:217-219) begins a part only once the earlier ones succeeded, so a setup cancelled or cut by a restart leaves the run, the account check and the setup hook not begun. Recording it as occurred then gives a ready Agent with no account check and no sign-in action raised. PND-011 (spec/promises/pending.yaml:216-218) keeps reconciliation from clearing a failed undo, but nothing covers a check that never ran. Not worked through to a full harm path; fix it with F2, for example by refusing the record where the engine's own state shows a part that is not optional and did not succeed.
- **The provenance of a Conduit opened by reconciliation names the first opener and not the reconciler, and a purge removes the only trace of the reconciler** (spec/conventions/access.yaml:816). ACC-044: "Reconciling an open as having occurred gives that invocation's provenance"; lifecycle.yaml:616-617 says the same. provenance.v1 (spec/core/results.yaml:220-237) has only actor, sentBy, invocation and openedThrough, with additionalProperties false. The actor whose word made the Conduit open is only in the record of the reconciliation, which purge.v1 removes (REC-004), while provenance is kept for good. After a purge, inventory attributes the open to an actor whose request never completed, through a Conduit that may since be closed. Small, and the coordinator's with F2.

## The verifier's notes

Commit 2eb8f258 (2026-11-12), read-only; nothing was built, run or changed.

Result: five findings confirmed (F1, F2, F4, F5, F7), five weakened (F3, F6, F8, F9, F10), none refuted and none wholly known. Every quote is verbatim at its cited line.

What changed most often was the label, not the substance:
- Novelty: F1, F3, F7, F8 and F10 were marked new and are known-new-situation. The records are rulings 32 (Q4), 33 (F11), 41 (item 15) and 66 (point 4) for F1, and round 6's D12, D10 and D21, order 081 and finding 4-12 for the others.
- Class: F6 and F8 are not gaps, because the text decides both situations. F9 is not a contradiction.
- Unsafe: F1 and F2 hold. F3's mark holds only narrowly, for the replace case.

F1, F2, F6 and the first extra share one root: a reconciliation that changes state on an actor's word for an operation the engine itself implements. One decision by the owner on that would settle them together.

Read in full: spec/conventions/lifecycle.yaml, spec/conventions/revisions.yaml and docs/guides/full-review.md. Read in part, for the cited promises: spec/conventions/operations.yaml (OPS-001 to OPS-008, OPS-012 to OPS-034), spec/conventions/access.yaml, implicit.yaml, records.yaml, spec/interface/implementation.yaml, the promise files for conduits, providers, placement, templates, pending and submissions, spec/core/operations.yaml and results.yaml, spec/README.md (form, limits), spec/definitions.md (cited rows), spec/implementation-defined.yaml, and the records named in each verdict.

Not checked: the engine's code, the conformance vectors, and the ruling records beyond the lines cited.
