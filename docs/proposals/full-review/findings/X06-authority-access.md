# X06-authority-access: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Reviewed at 2eb8f258 (2026-11-12). A security reviewer could not yet audit the access model from spec/ alone. The outline is auditable: one rule (ACC-001: no open Conduit, no dispatch), one table for the management verbs (ACC-033), bounds that keep IDs (ACC-029, ACC-030), and root-only conditions (ACC-024, ACC-054, ACC-057, SEC-008). The corners are not, and they fall in three places.

- **After a removal.** Lift, close and reconcile decisions need facts about objects that are gone, and no promise keeps them (F1). The engine keeps them in state of its own, and five vectors pin the result.
- **Reconciliation.** A close is routed two ways (F2). A removed sender's matching is unsaid, and the first build got it wrong for three rounds (F3). A non-root actor can complete an opening that only root may request (F9).
- **Disclosure.** ACC-056's claim that tier 1 is independent of the named object does not hold for open and close through a bound (F4). The refusing rule is observable but unordered (F5). Who receives tier 2 is undecided (F6). No vector cites ACC-056 and the engine builds no refusal.v1, so this part was judged from the text only.

Answers to the six scenarios:

1. **Delegation chain.** Opening within a bound, and passing a bound on, are decided (ACC-006, ACC-029, ACC-030, ACC-054). Once the bound's target is removed, they are not (F1). No promise requires keep_bound_targets; ACC-029 keeps only the ID. A tombstone lifted before its instance is removed is also undecided (F10).
2. **Tombstoned Conduit and register-replacement.** The case was decided by spec/: LIF-009 (spec/conventions/lifecycle.yaml:182-186) and LIF-041 (line 873) both say the replacement lifts nothing and reports conflict until an explicit lift. The vector cites exactly these. The failure after 042's rebase was the build's: the merge commit's own message says a Codex round fixed the one vector the merge broke.
3. **Management over one Fleet.** Through its Fleet's Conduit a holder can do nothing to another Fleet's Agents, Resources or Conduits (rows 4A, 6A, 14A and 16, and ACC-055). It sees the other ends, provenance and one hop of own-tools reach in its own Agents' inventories, which LIF-013 states. Retiring its Agents closes others' Conduits to them (CON-008). An invocation sent across Fleets can be reconciled only by root. All of this is decided.
4. **Disclosure.** Yes, a refused caller can learn that an object exists: through open and close under a bound (F4), through the choice of refusing rule (F5), and through set-up or retire naming a Conduit or template under one reading (F8). A Pool-only or Fleet-only holder inspecting or setting up a missing or foreign instance learns nothing, as OPS-029 intends.
5. **ACC-033's table.** It is decidable except for a close (F2) and for which Conduit carries row 1 (F11, known, with a new situation). Its retired-owner fallbacks are unreachable (F7). It lacks a row for set-up or retire of a Conduit or template (F8).
6. **Removed sender.** spec/ does not say how a removed sender is matched (F3).

Of the Fleet papers' three unverified gaps, ACC-055 now writes the first. The second, another Resource of the management or access Type, is still unforbidden but fails closed, because ACC-033 names the first-start objects. The third is F11.

**What the reader read.** Recounted at the pinned commit: 519 promises under spec/, and 494 case files under conformance/. access.yaml has 35 promises, all with `vectors: []`. No case cites ACC-056 or ACC-039.

Read completely:
- spec/conventions/access.yaml, spec/promises/fleets.yaml, spec/conventions/lifecycle.yaml and spec/promises/first-start.yaml.
- docs/guides/full-review.md, and the known-open lists: spec/README.md "Limits of this version", docs/AUDIT.md "Known to be open", docs/guides/cold-readers.md, and IN_FORCE.md topics 6 to 9.
- docs/proposals/spec-batch-1/areas/fleet.md and the unverified authority gaps in docs/proposals/projects/brief.md (lines 2247-2257).
- The Records of orders 042 and 041b.

Read in part:
- Promises: OPS-008, 014, 018, 019, 023, 029, 030 and 036; CON-004, 008, 024, 027, 028 and 031; PND-002, 007, 009 and 010; SEC-008; IMP-007; PLC-006 and PLC-017; OBJ-002, 004, 009 and 021; IFC-012 and IFC-050.
- spec/core: capabilities.yaml; tombstone.v1, provenance.v1 and object.v1 in results.yaml; register, inspect, list and lift in operations.yaml; bound.v1; the management and access Types; the template names.
- Engine, read only and not run: engine/crates/core/src/access.rs (lines 1-330 and 480-1000), inventory.rs (1-230), lifecycle.rs (90-340), and parts of requests.rs and registration.rs.
- Vectors, summarized with a read-only script: 21 cases under conformance/conventions/inventory/e4, four under fleets/e3b, access/bounds, and e3a/register-agent-non-root.

Not checked:
- Secrets beyond SEC-008, pools.yaml beyond a grep, the interface promises beyond the two named, the engine's shell, G5's models and batch-2 text.
- F7's claim that the state is unreachable rests on my reading of LIF-020, LIF-021, LIF-035, POOL-004, FLEET-003 and PLC-006. I did not model-check it.
- F9 and F2 need an engine-implemented verb to end unknown. I took IMP-007's restart case as the way and did not look for others.
- That a Resource's owner never changes is cited from spec/definitions.md:135, not from POOL-001's own text.


## FR-206

**Lift, close and reconcile decisions after a removal rest on facts no promise keeps**

X06/F1 · gap · verifier: confirmed, high · decides: coordinator · new

Same fault, or overlapping: FR-215, FR-241.

**Promises.** LIF-040, ACC-053, ACC-030, ACC-029, ACC-055, OPS-008

**Where.** spec/conventions/lifecycle.yaml:835

**Quote.** "A Conduit's tombstone also retains both ends by kind, name and ID, its local name, capability and operations."

**Situation.** Root gives operator O a Conduit to `access` bound to pools [P]. O opens for Agent A a Conduit to `access` bound to targets [R1], where P owns R1. R1 is retired and removed. (a) O asks to close A's Conduit: ACC-030 asks whether R1 is "covered by the old one, as ACC-029 states", that is, a Resource P owns; R1 is gone and ACC-029 keeps only its ID (spec/conventions/access.yaml:506). (b) A's Conduit is closed and O lifts its tombstone: ACC-053 says "meets ACC-030", which needs the closed Conduit's bound; LIF-040 and tombstone.v1 (spec/core/results.yaml:251-361) keep none. (c) A Conduit to R1 that CON-008 closed: a holder bound to pools [P] lifts it only if R1 is covered through P, but the tombstone keeps the ends without their owners. LIF-040's own visibility rule ("whoever may inspect either end as it stood") needs the same owner. (d) ACC-055's check of a reconciled lift needs what the lifted tombstone held, which OPS-008's list (spec/conventions/operations.yaml:155-160) does not keep. One builder refuses all four to a bounded holder. The first build permits them from state it added: `bound_targets` (engine/crates/core/src/access.rs:143-164), `bound` and each end's `owner` on the tombstone (engine/crates/core/src/inventory.rs:10-19 and 85-87), `lifted_tombstone` (engine/crates/core/src/lifecycle.rs:221). Five vectors pin the permit: conformance/conventions/inventory/e4/bound-removal-lift, pool-bound-removal-lift, bound-removal-reconcile, pool-bound-removal-reconcile and bound-removal-lift-register-replacement. No promise requires keep_bound_targets.

**Counterargument.** Judging a kept ID by the owner it had is the natural reading, and an owner never changes, so the engine's added state is harmless. Root is unbounded and can always close or lift, so nothing is stuck under the stricter reading.

**Least change.** LIF-040: "A Conduit's tombstone also retains, for each end that had one, that end's owning Pool or Fleet by kind, name and ID, and, for a Conduit to `access`, its bound with the IDs its items kept." ACC-029: "An item that names an individual target keeps, with its ID, the target's kind and its owning Pool; a target that has been removed is judged by them under ACC-030, ACC-053 and ACC-055." OPS-008: add "and, for `lift.v1`, what the lifted tombstone held". Add the members to tombstone.v1.

**Precedent.** Active Directory keeps lastKnownParent on a tombstone so that authority to restore can be judged after deletion. Kubernetes ownerReferences hold the owner's UID, not its name.

**Who decides.** coordinator. Ruling 46 point 4: a build finding fixed in the specification with a vector. The engine and five vectors already behave this way, so the text adds no behavior. Log it for the owner to strike, because it settles who may lift.

**Verifier.** confirmed, high. Quote is verbatim at spec/conventions/lifecycle.yaml:835-837. LIF-040's list and tombstone.v1 (spec/core/results.yaml:263-361) hold no bound and no owner for either end. Three texts need those facts. spec/conventions/access.yaml:833-836: "For a Conduit, it covers the target and every listed operation as for closing (ACC-012), or meets ACC-030 for a Conduit to `access`." spec/conventions/lifecycle.yaml:845-846: "shown in full to whoever may inspect either end as it stood". spec/conventions/access.yaml:876-878 (ACC-055): "or the tombstone lifted under ACC-053". ACC-029 keeps only "the ID it resolved to" (access.yaml:506-507). OPS-008's list (spec/conventions/operations.yaml:155-160) holds no input and no lifted tombstone. Refutation tried: nothing in spec/ keeps a removed target's kind or owner; operations.yaml:702-703, "A bound is judged against what exists", leans to the strict reading. The engine adds the state: engine/crates/core/src/access.rs:143-164 ("keep bound target kind and ownership independently of objects, tombstones and records"), inventory.rs:15-16 and 85-86, lifecycle.rs:221. All five vectors exist; conformance/conventions/inventory/e4/pool-bound-removal-lift is titled "Pool target coverage survives removal, tombstone lift and serialized restart for lift" and cites ACC-029, ACC-030, ACC-053, ACC-055 and LIF-040, none of which states it. Not in docs/AUDIT.md:174-245, cold-readers.md, the README limits, IN_FORCE.md, the increment 4 log, the review list or G5 (ACC-053 and ACC-055 are "static" at experiments/012-model-checking/G5.md:250-252).

**Verifier's corrections.** Severity is high, not only for corner cases: the ordinary stable-name replacement is affected. A pools-bounded operator who lifts a retired Resource's tombstone gets the Conduit tombstones it caused lifted only if the removed target still counts as covered through its Pool; a strict builder leaves them, and LIF-041 then blocks the reopen until root lifts. Part (b) is a defect, not a gap: ACC-053's "meets ACC-030" cannot be followed from LIF-040's list. The fix must also say whether the kept bound and end owners are shown: "shown in full" would give an end's inspector more than the live inventory does (lifecycle.yaml:252-254 lists no bound and no owner of the other end). Unsafe stays off: the strict reading fails closed and root can always lift. Coordinator, logged for the owner.


## FR-207

**Reconciling a close: row 12 sends it to `management`, four other texts send it to `access`**

X06/F2 · contradiction · verifier: confirmed, medium · decides: coordinator · new

**Promises.** ACC-033, ACC-055, LIF-028, LIF-005, LIF-024, LIF-035, IMP-007

**Where.** spec/conventions/access.yaml:620

**Quote.** "(12) Reconciling an invocation whose dispatched-to instance, or the instance a management verb named, no longer exists: `management`; it records the outcome and stores nothing."

**Situation.** A `close.v1` is in progress when the engine restarts, so its outcome is unknown (IMP-007). From its dispatch the Conduit is retired (LIF-005), its shutdown is confirmed and it is removed (LIF-024, LIF-035). The instance the verb named therefore never exists when the close is reconciled, and row 12, the first fitting row, gives `management`, where ACC-055 adds no condition. Four texts say otherwise. ACC-033's observable: "Opening, closing and lifting are reconciled through `access`" (spec/conventions/access.yaml:648). LIF-028: each of the four verbs is reconciled against the target that authorized it. ACC-055 limits "the Conduit opened or closed" through `access`. spec/core/capabilities.yaml:11-14 says a close is reconciled against `access`. A builder who follows the table uses `management`, as engine/crates/core/src/access.rs:572-592 does; a builder who follows the sentences uses `access` under the bound. I found no vector that reconciles a close.

**Counterargument.** Either route only records an outcome. The Conduit is already closed and removed, so nothing is granted or lost whichever target authorizes it.

**Least change.** Row 12: "... no longer exists, other than a Conduit that the reconciled `close.v1` itself closed: `management`; ...". The close then falls to row 13 and `access`, and ACC-055 checks the target and operations that the tombstone kept. The reverse fix strikes the close from the four other places.

**Precedent.** Row 13's own rule, "the target that authorized it". `access` is a first-start object and always exists, so the reason for row 12 (no owner left to ask) does not apply to a close.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix adds no behavior.

**Verifier.** confirmed, medium. Quote is verbatim at spec/conventions/access.yaml:620-621. A close's Conduit is always gone when the close is reconciled: spec/conventions/lifecycle.yaml:516-519, "one whose `close.v1` ended with whatever outcome, is confirmed when it is closed, and the Conduit is then removed". So row 12, the first fitting row, takes every close, and OPS-014 agrees (spec/conventions/operations.yaml:331-333). Against it: access.yaml:648, "Opening, closing and lifting are reconciled through `access`"; lifecycle.yaml:614-616; access.yaml:876-877; spec/core/capabilities.yaml:11-14. The engine routes to `management` once the object is gone (engine/crates/core/src/access.rs:571-576). No vector title reconciles a close. No record of the point in docs/, IN_FORCE.md or G5.

**Verifier's corrections.** Two more texts carry the `access` side: access-control.v1's description (spec/core/capabilities.yaml:97-98) and reconcile.v1's contract (spec/core/operations.yaml:499-500, "`access` for an open or a close"), which states the `management` rule four lines later (503-504). The reviewer's first fix depends on F1: under `access`, ACC-055's bounded check needs the closed Conduit's target, operations and bound from a tombstone that may already be lifted. The reverse fix, striking the close from the six places, matches the engine and needs nothing kept. Coordinator.


## FR-208

**ACC-055 does not say how a kept sender is matched, or what "owns" means once the sender is removed**

X06/F3 · ambiguity · unsafe · verifier: weakened, low · decides: coordinator · new

**Promises.** ACC-055, OPS-008, FLEET-002

**Where.** spec/conventions/access.yaml:879

**Quote.** "Through a Fleet, it must have been sent by that consumer, that Fleet or an Agent the Fleet owns."

**Situation.** Agent x of Fleet G sends a request to Agent t of Fleet F, and its outcome is unknown. x is retired and removed, its tombstone is lifted, and a new Agent named x is registered in F. F's operator H then asks `reconcile.v1` through F. ACC-055 says only that the sender "is kept under OPS-008 and compared". It does not say by what, or whether "owns" means when the request was sent or when it is reconciled. Reading (a), by ID and owned now: H is refused; and for a sender of F's own that has been removed, only root can ever reconcile (engine/crates/core/src/access.rs:914-924). Reading (b), owned when sent: needs the sender's Fleet kept, which OPS-008 does not list. Reading (c), by name: the new x counts, and H reconciles an invocation that another Fleet's Agent sent. The first build did (c) through three rounds of passing vectors until a fresh verifier caught it (docs/work-orders/042-build-tombstones-and-inventory.md:59-61). The engine was changed and spec/ was not. The same question holds for a hook's sender and for a Pool's "that consumer" after a replacement under its name.

**Counterargument.** Every other kept identity in the access convention is an ID (ACC-029, LIF-040), so a careful builder takes (a). Root's default Conduits list reconcile.v1 and cannot be closed, so no invocation is left without a reconciler.

**Least change.** ACC-055, observable: "The sender is compared by the ID kept under OPS-008, never by a name. An Agent the Fleet owns is one that exists, and that the Fleet owns, when the reconciliation is decided (ACC-036). Once the sender is removed, only root reconciles its invocations through the Pool or Fleet."

**Precedent.** Kubernetes matches ownerReferences by UID and never by name. ACC-036 decides from the state as it stands.

**Who decides.** coordinator. Ruling 37: an ambiguity whose fix adds no behavior. The admin's fix under ruling 71 already chose reading (a) in the build.

**Verifier.** weakened, low. Quote is verbatim at spec/conventions/access.yaml:879-880. Reading (c) is not a sound reading. spec/definitions.md:110: "The stable identity of an object ... a name comes to belong to another object only once the first no longer exists, and an instance's reference never follows it". spec/promises/fleets.yaml:37-39: "a new Agent with its own ID". spec/conventions/operations.yaml:176: "The kept state explicitly includes the instance that sent the request." The new x did not send the invocation, so an engine that counts it breaks ACC-055. Between (a) and (b) the text leans to (a): "owns" is present tense, ACC-036 decides from the state as it stands (access.yaml:710-711), and OPS-008 keeps the sender but not its owner, so (b) has nothing to read. Already recorded as a build decision: docs/work-orders/042-build-tombstones-and-inventory.md:59-61 and docs/implementation/HANDOVER.md:265, "Fleet reconciliation matches a sender by ID only, not through a reused name". Engine: engine/crates/core/src/access.rs:918-923.

**Verifier's corrections.** Unsafe mark removed: (c) was a build fault, not a reading that a promise-keeping engine may take. Class: a missing explicit statement, not an ambiguity. What is new is small: spec/ was never changed to say it, though ruling 46 point 4 puts a build finding's fix in the specification with a vector, and the consequence that only root reconciles a removed sender's invocations through the Fleet is unstated. The Record says four rounds, with the fault found by the fresh verifier of the round after the rebase, not three. Coordinator: one sentence in ACC-055.


## FR-209

**Tier 1 cannot be independent of the named object: the reason and the refusing rule disclose it to a bounded holder**

X06/F4 · contradiction · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-002.

**Promises.** ACC-056, OPS-029, ACC-012, ACC-024, ACC-029, ACC-030, ACC-053

**Where.** spec/conventions/access.yaml:919

**Quote.** "Nothing in tier 1 depends on whether an object exists, who owns it or its state."

**Situation.** Agent D holds a Conduit to `access` that lists close.v1 and is bound to pools [P]. It may inspect no Conduit: row 7 gives that to `management`. D calls `access.close.v1 {conduit: n}` with guessed names. Where no Conduit n exists the request reports invalid, "whatever the bound" (spec/conventions/operations.yaml:703-705). Where n is root's it reports refused with ruleId ACC-024. Where n's target is outside P it reports refused with ACC-029 and the bound in `restriction`. Where n is a Conduit to `access` with a wider bound it reports refused with ACC-030. D thus learns that n exists, whether root is its consumer and whether its target is `access`. ACC-056 says tier 1 depends on none of this and that the diagnostic is "identical whether a name exists or not". ACC-053 makes the opposite choice for a lift: a bounded holder gets refused for a missing ID and "learns nothing". The engine follows OPS-029 (engine/crates/core/src/access.rs:799-881 and 977-979).

**Counterargument.** A holder of a right to open must be told Conduit names to use it, and names are not secrets. The specific causes of ACC-024 and ACC-029 are what ruling 52 asks for. The sentence may have been meant only for ACC-001 and ACC-033.

**Least change.** OPS-029: "Through a Conduit to `access` that carries a bound, a request to open or close that names no Conduit reports refused under ACC-029, as a lift does under ACC-053." ACC-056: "To a caller without tier 2, a request to open, close or lift that a rule decided from the named object refuses carries ACC-029's row." The smaller alternative narrows the quoted sentence to ACC-001 and ACC-033 and states what the other rows disclose.

**Precedent.** ACC-053's rule for a lift. Kubernetes authorizes before it looks an object up, so a forbidden caller gets 403 whether or not the object exists.

**Who decides.** coordinator. Ruling 37: a contradiction. Ruling 52 point 1 already rules the same answer for a missing target where the caller has no authority over it. Add it to the review list, because a reported reason changes.

**Verifier.** confirmed, medium. Quote is verbatim at spec/conventions/access.yaml:919-921. Tier 1 carries "the cause in the refusing rule's row below" (access.yaml:918), and the rows for ACC-024, ACC-029 and ACC-030 (access.yaml:952, 954, 955) differ; which one applies depends on the named Conduit's consumer, its target, that target's owner and its bound. OPS-029 states the existence split on purpose (spec/conventions/operations.yaml:702-705): "a request for `open.v1` that names no Conduit reports invalid whatever the bound". ACC-053 does the opposite (access.yaml:844-845): "refused to a bounded holder, which learns nothing". IN_FORCE.md:143 records ruling 52 point 1: "a target that does not exist gets the same answer where the caller has no authority over it". The engine names ACC-024, ACC-054, SEC-008, ACC-032, ACC-030 and ACC-029 apart (engine/crates/core/src/access.rs:797-881). Nothing in docs/proposals/spec-batch-1/areas/refusals-and-identity.md or the known-open lists covers open or close under a bound.

**Verifier's corrections.** The finding has two parts of different strength. The invalid-or-refused split is not a contradiction in letter: ACC-056 governs refusals only, and OPS-029 states the split deliberately. It conflicts with ruling 52 point 1 as IN_FORCE.md:143 records it. The contradiction in letter is the choice of rule and cause, which tier 1 carries. The reviewer's first fix changes a reason that OPS-029 states on purpose (invalid to refused): the coordinator may settle it under ruling 52, but it belongs on the review list. The narrower fix is plainly the coordinator's. F5's proposed order would entrench this disclosure; see F5.


## FR-210

**No promise orders the access rules when more than one refuses, yet the ruleId is observable**

X06/F5 · gap · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-004, FR-126, FR-158.

**Promises.** ACC-056, OPS-023, IFC-050, ACC-012, ACC-024, ACC-029, ACC-033, ACC-057

**Where.** spec/conventions/access.yaml:915

**Quote.** "Its result carries one error diagnostic, whose ruleId is the refusing promise and whose properties.refusal has the form refusal.v1."

**Situation.** (i) An Interface with a Conduit to `management` listing register.v1, and none to Fleet F, submits an Agent owned by F. ACC-033 row 1A refuses and ACC-057 refuses. (ii) A delegate asks to close root's default Conduit to a Pool outside its bound. ACC-024 refuses, ACC-029 refuses, and ACC-024's own observable says the request "reports refused under ACC-012". OPS-023 orders the four reasons, not the rules within refused, and IFC-050 makes the ruleId observable. The engine's order is whatever its code checks first (engine/crates/core/src/access.rs:721-881), and vectors pin it: conformance/conventions/fleets/e3a/register-agent-non-root requires ACC-057, and step 17 of conformance/conventions/access/bounds requires ACC-024. A second engine names another rule for the same request, and the choice also decides what tier 1 discloses (F4). The table also gives rows to about twenty promises that only restate a refusal (OPS-023, OPS-029, IFC-025, LIF-037, ACC-036 and others) and that, by the same promise, are never the ruleId.

**Counterargument.** Any refusing rule is a true cause, and the vectors already fix the order in the cases that matter.

**Least change.** ACC-056: "Where more than one rule refuses, the diagnostic names the first in this order: no instance (IFC-032, IFC-022); the sender and its chain (IFC-021, IFC-028, ACC-019, ACC-034, ACC-039); ACC-001; ACC-033; ACC-032; ACC-057; ACC-054; LIF-034; ACC-024; SEC-008; ACC-029 or ACC-030; ACC-053; ACC-055." Remove the rows of promises that only restate.

**Precedent.** OPS-023's order of the four reasons. HTTP answers 401 before 403.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the fix writes down the order the build already uses and adds no behavior.

**Verifier.** confirmed, medium. Quote is verbatim at spec/conventions/access.yaml:915-916. spec/ orders only the four reasons (spec/conventions/operations.yaml:493-496); a search for "deciding rule" and "refusing rule" finds no order among access rules. The ruleId is observable (spec/interface/implementation.yaml:1349): "the ruleId, reason and refusal.v1 fields are observable". One request has three candidates: access.yaml:428 says a refused close of root's Conduit "reports refused under ACC-012", while ACC-024 and ACC-029 have rows of their own (access.yaml:952, 954). Vectors pin a choice through required_promises: conformance/conventions/fleets/e3a/register-agent-non-root/case.yaml requires ["ACC-057"]; the last step of conformance/conventions/access/bounds/case.yaml requires ["OPS-023", "IFC-025", "ACC-024"]. The engine's order is its code order (engine/crates/core/src/access.rs:721-881). Not recorded in the logs, the review list or the known-open lists.

**Verifier's corrections.** The bounds step is step 18 of 18 counted from one (fixture index 16), not 17. The proposed order copies the build and puts ACC-054, ACC-024 and SEC-008 before ACC-029 and ACC-030. That tells a bounded caller that a Conduit outside its bound is root's or lists get-secret.v1, the disclosure F4 reports. Put bound coverage first for a bounded caller, or say that F4's rule overrides. Removing the rows of promises that only restate is a separate, low point. Coordinator.


## FR-211

**Who receives tier 2 is undecided: "may inspect the authorizing target under ACC-033" has two readings and no meaning for an ordinary operation**

X06/F6 · ambiguity · verifier: weakened, low · decides: coordinator · new

**Promises.** ACC-056, ACC-033, ACC-001, IFC-050

**Where.** spec/conventions/access.yaml:921

**Quote.** "Another caller gets it only where it may inspect the authorizing target under ACC-033 and every object the request names, through open Conduits with no filter."

**Situation.** (i) An Interface holds a Conduit to Pool P that lists only inspect.v1, and calls set-up.v1 on a Resource of P. ACC-033's authorizing target is P. If "may inspect P" means inspecting the Pool object, row 7 gives that to `management` and the caller gets tier 1. If it means holding inspection through P, the caller gets tier 2. (ii) An Agent calls `store.write.v1` through a Conduit that lists only read. The refusing rule is ACC-001, and ACC-033 names no authorizing target for an ordinary operation, so whether anyone but root gets tier 2 is undecided. Engines will differ in which callers receive `target`, `conduits` and `exists` in refusal.v1, fields that IFC-050 makes observable. No vector cites ACC-056, and the engine builds no refusal.v1.

**Counterargument.** Either reading shows a caller only its own Conduits to something it may already inspect. Neither discloses more than inspection would.

**Least change.** "Another caller gets it only where it holds, with no filter, an open Conduit listing `inspect.v1` to the target that rows 6, 6A and 7 name for inspecting the request's target, and likewise for every object the request names."

**Precedent.** ACC-033 row 17 and PND-009 already word answering this way. Kubernetes SelfSubjectAccessReview asks whether the caller may get the object.

**Who decides.** coordinator. Ruling 37: an ambiguity whose fix adds no behavior.

**Verifier.** weakened, low. Quote is verbatim at spec/conventions/access.yaml:921-923. Situation (i) is decided on the plain reading: the object of "inspect" is the authorizing target, here Pool P, and access.yaml:612-613 says "(7) Inspecting any other object, an Interface, Pool, Fleet, Conduit or template: `management`". A Pool-only holder gets tier 1. The second reading, holding inspection through P, has no words behind it; the batch-1 paper says the same (docs/proposals/spec-batch-1/areas/refusals-and-identity.md:25 and :235, "a caller that may inspect a Pool but not its Resource gets tier 1 only"). For (ii), ACC-001 names the target of an ordinary operation (access.yaml:29-30), "under ACC-033" can qualify "may inspect", and IN_FORCE.md:143 says "A caller that may inspect the target". No file under conformance/ cites ACC-056 and no engine source builds refusal.v1, so nothing pins either case.

**Verifier's corrections.** Not two sound readings for (i). What survives is a wording slip in (ii): "the authorizing target under ACC-033" has no referent for an operation ACC-033 does not route. One clause settles it: for such an operation, the request's target. The reviewer's rewrite also works. A consequence worth stating in the text: on the plain reading, only root and holders of inspect.v1 on `management` receive tier 2 for a request that a Pool or Fleet authorizes. Coordinator.


## FR-212

**The retired-owner fallbacks of ACC-033 decide a state no request reaches**

X06/F7 · needless-complexity · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-008, FR-259.

**Promises.** ACC-033, LIF-020, LIF-021, LIF-035, POOL-004, FLEET-003

**Where.** spec/conventions/access.yaml:610

**Quote.** "(6) Inspecting a Resource: its owning Pool, or `management` where that Pool is retired."

**Situation.** A Pool or Fleet is retired only by a request that LIF-020 refuses while any member's shutdown is unconfirmed, or by a cascade that dispatches the container's retirement "only once every member's shutdown is confirmed" (spec/conventions/lifecycle.yaml:423). A confirmed shutdown removes the member (LIF-035), and no member is registered into a retired owner (POOL-004, FLEET-003). So no Resource or Agent exists under a retired owner. The clauses for that state decide nothing reachable: rows 4, 4A, 6, 6A, 14, 14A and 17; the sentence on "remaining members" (spec/conventions/access.yaml:653-655); the paragraph at spec/core/capabilities.yaml:49-58; and the engine's branches (engine/crates/core/src/access.rs:547-564 and 584-587). The one vector that exercises them, conformance/conventions/fleets/e3b/routing-retired, reaches the state by patching the Fleet to retired in its requests.json. The Fleet area paper saw this of one sentence and left it out (docs/proposals/spec-batch-1/areas/fleet.md:176).

**Counterargument.** The clauses keep the table total over any state an engine might read from a damaged or restored store. They would be needed if a later version let an owner retire with members.

**Least change.** Strike the retired-owner clauses from rows 4, 4A, 6, 6A, 14, 14A and 17 and the "remaining members" sentence. Keep row 12, row 13's fallback for the cascade and the container's own retirement, and LIF-040's rule for tombstones. Add: "A retired Pool or Fleet has no members (LIF-020, LIF-021)."

**Precedent.** Kubernetes removes a namespace only after its contents, and states no rule for objects of a deleted namespace.

**Who decides.** coordinator. Ruling 37: the fix removes text and no behavior.

**Verifier.** confirmed, low. Quote is verbatim at spec/conventions/access.yaml:610. The state is unreachable. spec/conventions/lifecycle.yaml:407-410 (LIF-020 reports conflict while any member's shutdown is unconfirmed); 423-424, "the container's retirement is dispatched only once every member's shutdown is confirmed"; 442, "A member whose shutdown is confirmed has been removed (LIF-035)". spec/promises/pools.yaml:60-61 and spec/promises/fleets.yaml:48 reject a member for a retired owner. spec/promises/placement.yaml:79-81 gives a Pool or Fleet no placement, so declaring a node lost retires none, and LIF-023 lists no other retirement. docs/proposals/spec-batch-1/areas/fleet.md:176 saw it of one sentence: "That state cannot arise, because a Fleet retires only once every Agent's shutdown is confirmed, so I do not write it." The vector reaches the state by a patch: conformance/conventions/fleets/e3b/routing-retired/requests.json:127-133 sets "state": "retired". Not in the known-open lists.

**Verifier's corrections.** Add to the text to strike: OPS-036's "its remaining members are listed through `management`" (spec/conventions/operations.yaml:883-884) and the matching phrases of object-management.v1's description (spec/core/capabilities.yaml:67, 69, 74). Keep row 2's retired-owner clause: a registration that names a retired owner which still exists is reachable and must get to invalid. Coordinator.


## FR-213

**ACC-033 has no row for set-up or retire that names a Conduit or a template**

X06/F8 · gap · verifier: confirmed, low · decides: coordinator · new

**Promises.** LIF-032, ACC-033, OPS-029, OPS-023

**Where.** spec/conventions/lifecycle.yaml:664

**Quote.** "An actor asks for `set-up.v1` or for `retire.v1`, and the request names a template or a Conduit."

**Situation.** A Fleet operator calls set-up.v1 through its Conduit to the Fleet with `instance: {kind: Conduit, name: c}`. LIF-032 says the request reports invalid, and "Where the actor is not permitted, OPS-023 gives refused first". Rows 4, 4A and 5 name only Resources, Agents, Interfaces, Pools and Fleets, so no row says which target permits this request. Reading (a): the Conduit it came through permits it. It then reports invalid where c exists and refused where it does not (OPS-029: a missing thing is `management`'s alone), which tells a Pool-only or Fleet-only holder which Conduit and template names exist. Reading (b): `management` alone permits it, so it reports refused either way. The engine does (b) (engine/crates/core/src/access.rs:547-554). ACC-033's `when` also omits signing in and answering, which rows 16 and 17 decide.

**Counterargument.** The description of object-management.v1 ("setting up and retiring what is not a Resource") points to `management`, and the request is useless under either reading.

**Least change.** Row 5: "Setting up or retiring an Interface, Pool or Fleet, or a request for either that names a Conduit or a template (LIF-032): `management`." Add signing in and answering to `when`.

**Precedent.** OPS-029's rule that a thing with no owner is `management`'s alone.

**Who decides.** coordinator. Ruling 37: a gap whose fix adds no behavior; the build already does this.

**Verifier.** confirmed, low. Quote is verbatim at spec/conventions/lifecycle.yaml:664-665; lines 672-673 add "Where the actor is not permitted, OPS-023 gives refused first." Rows 4, 4A and 5 (spec/conventions/access.yaml:605-609) name no Conduit or template, though row 7 names both for inspection (612-613). spec/core/capabilities.yaml:67-68, "setting up and retiring what is not a Resource", leans to `management`. The engine sends anything but a Resource or Agent instance to `management` (engine/crates/core/src/access.rs:547-554). ACC-033's `when` (access.yaml:587-590) omits signing in and answering, which rows 16 and 17 decide. Not recorded.

**Verifier's corrections.** There are three readings, not two. On the strict one no row fits, so no Conduit authorizes the request, and LIF-032's invalid is never reached, even by root. Otherwise as stated. Coordinator.


## FR-214

**A reconciliation that makes a Conduit open is not held to the root-only conditions on opening it**

X06/F9 · ambiguity · unsafe · verifier: weakened, medium · decides: coordinator · new

**Promises.** ACC-055, LIF-028, ACC-054, SEC-008, IMP-007

**Where.** spec/conventions/access.yaml:876

**Quote.** "Through `access`, the bound must cover what the reconciled request needed it to cover: the target and every operation of the Conduit opened or closed, ACC-030's condition for a Conduit to `access`, or the tombstone lifted under ACC-053."

**Situation.** 1. Root, on its own behalf, sends open.v1 for a Conduit that lists get-secret.v1 (SEC-008) or whose consumer is root (ACC-054). 2. The engine restarts while the open is in progress, so its outcome is unknown (IMP-007) and the Conduit stays registered. 3. A second Interface with an unbounded Conduit to `access` listing reconcile.v1, for which ACC-055 has "no additional condition", or a delegate whose bound covers the target and names the operations, sends reconcile.v1 recording that the open occurred. Row 13 gives `access`, and ACC-055 passes. 4. Under LIF-028 the Conduit is now open, and the engine reads the secret for its consumer. That actor could not have sent the open: SEC-008 and ACC-054 refuse it whatever its bound. Whether those conditions bind a reconciliation is not said. SEC-008 speaks of a request "to open", and ACC-054 of one that "would open". The engine applies neither: engine/crates/core/src/access.rs:797-823 tests only open.v1, and engine/crates/core/src/requests.rs:1327-1351 opens the Conduit.

**Counterargument.** Root itself asked for the opening, and the reconciler only records what happened. Root can reconcile first. An unknown open.v1 needs a restart at the wrong moment.

**Least change.** ACC-055: "A reconciliation that records an open as having occurred is dispatched only where every condition on sending that open holds for the reconciling request: ACC-054 and SEC-008 apply to it as to the open."

**Precedent.** Kubernetes RBAC escalation prevention: a user creates or updates a binding only for permissions the user already holds.

**Who decides.** coordinator. Ruling 37: an ambiguity, resolved toward the owner's words in ruling 62 C1 ("Root alone") and toward SEC-008. Log it for the owner to strike, because it adds a refusal.

**Verifier.** weakened, medium. Quote is verbatim at spec/conventions/access.yaml:876-878. Each step holds. spec/conventions/implicit.yaml:75: "An invocation that was in progress across a restart has the outcome unknown." Row 13 gives `access`. access.yaml:888-889: "An unbounded Conduit to `access` and reconciliation through `management` have no additional condition under this promise." spec/conventions/lifecycle.yaml:597-598: "Recording that a setup or an open occurred makes the instance ready or the Conduit open". SEC-008's `when` is "A request is made to open a Conduit that lists `get-secret.v1`" (spec/promises/secrets.yaml:196). The engine opens the Conduit on reconciliation with neither check (engine/crates/core/src/requests.rs:1327-1351; engine/crates/core/src/access.rs:797-823 runs only for open.v1). No record of the point.

**Verifier's corrections.** Two corrections. First, for a Conduit whose consumer is root the text already reaches the case: access.yaml:855 says "A request would open a Conduit whose consumer is root", and a reconciliation that records an open does open it. That half is the build's divergence from a literal reading. The ambiguity is SEC-008. Second, the unsafe mark is overstated: no access arises that root did not itself request on its own behalf. The other actor only completes root's dispatched open, and root can prevent that by reconciling first or closing the registered Conduit. What is lost is that the act making a secret Conduit open is no longer root's. A ruling that this alone counts as granted access would restore the mark and make it high. The fix adds a refusal: the coordinator may settle it toward ruling 62 C1, logged and on the review list. It should name ACC-032 too.


## FR-215

**A tombstone lifted before its instance is removed: what later closures name as cause is the build's choice**

X06/F10 · gap · verifier: confirmed, low · decides: coordinator · new

Same fault, or overlapping: FR-206, FR-241.

**Promises.** LIF-040, ACC-053, LIF-035, CON-008

**Where.** spec/conventions/lifecycle.yaml:839

**Quote.** "For a Conduit closed because an end was retired it names that end's tombstone; for loss it keeps the cause under PLC-024, and a tombstone is made only once for an object's retirement."

**Situation.** An Agent is retired and its stop ends unknown. It still exists, retired, and its tombstone was made at retirement. A holder lifts that tombstone now; ACC-053 does not forbid it. Later the retirement is reconciled as having occurred. The Agent is removed with no tombstone, since one is made "only once", so its name is free with no further lift. CON-008 closes the Conduits it consumed, and their tombstones must name "that end's tombstone", which no longer exists. The earlier lift did not remove them, though a lift after removal would have removed "each tombstone it caused". The engine writes the lifted tombstone's ID as their cause, and conformance/conventions/inventory/e4/retirement-lift-reconcile-cause pins that. Another builder names the confirming invocation, or refuses to lift a tombstone whose object still exists.

**Counterargument.** The end state is one a permitted lift could reach anyway. Only the cause field and the number of lifts differ.

**Least change.** LIF-040, observable: "Where that end's tombstone was lifted before the Conduit closed, the cause still holds its ID; that lift does not reach the Conduit's tombstone, which is lifted on its own."

**Precedent.** Kubernetes leaves an ownerReference holding a deleted owner's UID in place.

**Who decides.** coordinator. Ruling 46 point 4: a build finding; the text states what the vector already shows and adds no behavior.

**Verifier.** confirmed, low. Quote is verbatim at spec/conventions/lifecycle.yaml:839-841. The tombstone is made at retirement (lifecycle.yaml:830, "or another instance is retired"). ACC-053 sets no condition on the object still existing (spec/conventions/access.yaml:831-833). A lift reaches only tombstones "it caused" at that moment (access.yaml:837-838). conformance/conventions/inventory/e4/retirement-lift-reconcile-cause is titled "A lifted retirement keeps its cause ID for later Conduit closures during reconciliation" and cites LIF-040, ACC-053 and LIF-028, none of which states it. Not recorded.

**Verifier's corrections.** None of substance. The fix should also state the other consequence: at removal the name frees with no lift, because no tombstone holds it (lifecycle.yaml:728-729).


## FR-216

**Row 1 needs two Conduits and a request travels through one: the owner's register tools can only be refused**

X06/F11 · ambiguity · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** ACC-033, FLEET-005, IFC-012, FLEET-003

**Where.** spec/conventions/access.yaml:595

**Quote.** "(1) Registering a Resource whose owner is a live existing Pool, with `register.v1`: `management` and that Pool, each listing `register.v1`."

**Situation.** Known: the tracker lists "which Conduit carries ACC-033 row 1" (docs/AUDIT.md:236). New: FLEET-005 puts register.v1 on every default Conduit, and register-replacement.v1 on a Pool's, so IFC-012 presents root a register tool for each Pool and Fleet, and "A call to a Conduit's tool is a request through that Conduit". Root holds both Conduits that row 1 names and calls the Pool's register tool with a Resource for that Pool. By the row the request is authorized. By register.v1's contract, "Authorized against `management`" (spec/core/operations.yaml:119), it is not. The engine refuses it under ACC-033 (engine/crates/core/src/access.rs:538-546 and 721-724), and every vector registers through `management`. Each Pool and Fleet thus adds one or two tools to root's list that can only be refused. ACC-056's cause for ACC-033, "needs the target described by its first fitting row", explains nothing to a caller that holds exactly those Conduits.

**Counterargument.** The contract's description settles the point for a reader of spec/core. The owner's Conduit must list the operation for the row to be checkable, and a tool that is always refused does no harm.

**Least change.** Rows 1, 1A and 3: "The request is made through the Conduit to `management`; the Conduit to the owner is a further condition. Made through the owner's Conduit it reports refused, and the cause says to send it through `management`."

**Precedent.** Kubernetes sends a create to one endpoint and checks further conditions on that same request at admission.

**Who decides.** coordinator. Ruling 37: an ambiguity whose fix adds no behavior; the build already decides it this way.

**Verifier.** weakened, low. Quote is verbatim at spec/conventions/access.yaml:595-596. The core is known: docs/AUDIT.md:236 and docs/proposals/projects/brief.md:2253, "The spec text does not say which carries the request." Its tool-count side is open with the owner: IN_FORCE.md:178 lists "registration through a Fleet" and "root's tool count" (ruling 62 C6 to C12). The core package does name the carrier (spec/core/operations.yaml:119-120): "Authorized against `management`; registering a Resource into a Pool that already exists also needs an open Conduit to that Pool". Against it, spec/core/capabilities.yaml:115: "What every Pool offers: registering a Resource into it". The defaults list both operations (spec/promises/fleets.yaml:80-83). Engine: engine/crates/core/src/access.rs:538-546 and 721-724.

**Verifier's corrections.** Only two things are new. Each default presents a register tool, and a register-replacement tool for a Pool, that is refused when called, which bears on the open point C11. ACC-056's cause for ACC-033 (access.yaml:957) tells a caller that holds both Conduits nothing. The sentence fix is the coordinator's. Taking those operations out of what the defaults present is the owner's: FLEET-005 says "lists exactly", and C11 is open.


## Further points the verifier noticed

Not verified by a second reader.

- **No promise says what recording that a lift occurred changes** (spec/conventions/lifecycle.yaml:594-595; spec/conventions/access.yaml:846-847). ACC-053 says a lift is "reconciled through `access`", but LIF-028's `when` names only set-up.v1, open.v1, retire.v1 and close.v1, and OPS-014 (spec/conventions/operations.yaml:337-339) points only to LIF-028 and REV-010 for what a reconciliation changes. If the tombstone is still there and an actor records that the lift occurred, the outcome becomes succeeded and the tombstone stays: the engine handles only open and retire (engine/crates/core/src/requests.rs:1327-1368). LIF-041 keeps blocking, and a repeat under the same key would be answered from the recorded success. Gap, low, not unsafe. Coordinator: one sentence in ACC-053 or LIF-028, either that it removes the tombstone as a lift does or that it changes nothing and a new lift is needed.

## The verifier's notes

Checked at 2eb8f258 (2026-11-12), read-only; nothing was built or run. All eleven quotes are verbatim at the cited lines.

Outcome: seven confirmed (F1, F2, F4, F5, F7, F8, F10) and four weakened (F3, F6, F9, F11); none refuted, none wholly known.
- Both unsafe marks come off. F3's rested on a reading the definitions exclude. F9's is overstated, because the access is one root itself requested.
- F1 goes up to high: it reaches the ordinary replacement workflow of a pools-bounded operator, and five vectors pin behavior that no promise states.
- F4 and F5 should be fixed together. The rule order F5 proposes copies the build's, and that order is what causes the disclosure F4 reports.
- F2's `access` fix depends on F1's kept facts; the reverse fix does not.

Not checked: the fixtures behind register-agent-non-root (whether its Interface lacks the Fleet Conduit); I read only its required_promises. For F2 I searched vector titles, not every step, for a reconciled close. The engine's lines 977-979 cited in F4 were not opened; OPS-029's text carries that point.

The reviewer's answer to scenario 2 stands on the text: spec/conventions/lifecycle.yaml:182-186 and 873 both say a register-replacement lifts nothing and reports conflict.
