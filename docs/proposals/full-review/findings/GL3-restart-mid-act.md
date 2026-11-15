# GL3-restart-mid-act: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The restart rule is one sentence in an observable, and it treats the engine's own bookkeeping like a call to a stranger. For the eleven verbs that make no call outside the engine once dispatched, and for unregister.v1 by its own text, a promise already fixes what the store holds (LIF-004, LIF-005 with LIF-024, PLC-017, LIF-036), so unknown there buys only a reconciliation by hand and is the route to L02/F2 and X06/F9. The other half is missing: no promise says that such a verb's stored effect is whole or absent after a restart, and OPS-004 with IMP-007 lets a builder treat a half-made effect as a lawful unknown; for declare-lost.v1 that reading leaves access standing under a result that says succeeded (F2, unsafe on that reading). The first build avoids all of this by a choice recorded only in its conformance document, one committed transition per step, so its vectors have to write an unknown open.v1 or lift.v1 into the state by a fixture. Lifting that choice into one promise, a database transaction that after recovery is committed or is not, settles F1, F2, F4 and F7 together and lets the owner strike `deduplicates` from the twelve core contracts and open and close from LIF-028. Unknown stays right for exactly what left the engine and had not answered: a call to an implementation, a hook, run.v1, stop.v1 inside a retirement, accept-machine.v1, enroll.v1 and remove-machine.v1. The fetch and place.v1 come before dispatch and state no effect, so they never justify unknown. For the verbs made of parts the text gives two answers, IMP-007 against OPS-022 and LIF-021 (F3); that one is the owner's, because the answer I recommend changes what the first build reports. First start needs no finding of its own: FST-001 to FST-003 and FST-007 fix its end state, DL6 leaves an interrupted one to each engine, and F2's rule would close that clause.

**What the reader read.** Commit 2eb8f258, read-only; nothing was built, run or changed.

Read in full: docs/guides/full-review.md; AGENTS.md; spec/conventions/implicit.yaml, operations.yaml, lifecycle.yaml and records.yaml; spec/promises/first-start.yaml, fleets.yaml, submissions.yaml, enrollment.yaml, pending.yaml and pools.yaml; spec/implementation-defined.yaml.

Read in part, for the promises cited: spec/conventions/access.yaml (ACC-005, 006, 011, 012, 033, 044, 053 to 055, 057) and revisions.yaml (REV-010, 012, 014); spec/promises/templates.yaml (TPL-008, 009, 012, 017, 018), providers.yaml (PRV-005, 013, 017, 020, 021, 026, 029, 030), packages.yaml (PKG-023 to 027, 032), placement.yaml (PLC-017, 024 to 027), conduits.yaml (CON-004's end, CON-006 to 008), objects.yaml (OBJ-002, 023, 025), instantiation.yaml (INS-016), contracts.yaml (CTR-002); spec/interface/implementation.yaml (IFC-014, 025, 028, 050); spec/core/operations.yaml (the head comment and the contracts of register, replace, unregister, set-up, retire, reconcile, register-replacement, install, declare-lost, purge, retire-with-resources, open, close and lift); spec/core/README.md (the verbs table and the `deduplicates` note); spec/core/conduit.yaml (the engine's `implements` list); spec/definitions.md and spec/README.md (rows and sections cited); docs/AUDIT.md (who decides, known to be open); docs/implementation/HANDOVER.md (the dogfooding section; the rest by search).

First pass: language/INDEX.txt in full by search; critic.json's sampled entries; L02/F2 and F8 with the verifier's additions, L03/F1, L05/F6, X06/F1, F2 and F9, X07/F6 and F8, X09/F1, X10/F11.

As evidence only, not as what holds: engine/CONFORMANCE.md (rows DL2, DL3, DL6 and the lines cited), engine/crates/core/src/requests.rs (recover, the fence predicate and its test), engine/crates/conformance/src/lib.rs:455-520, and the steps of the vectors with a restore-state or lifecycle-fixture step under conformance/.

The walk of each act is in the findings: verbs with no call out and unregister.v1 in F1, stored state in F2, set-up, retire and the cascades in F3, keyed repeats in F4, reconciliation after recovery in F5, removal in F6, admission in F7, the fetch and place.v1 in F8. 'The unregister of a retired owner' has no cascade in spec/: LIF-020 and LIF-021 let a Pool or Fleet retire only once every member is removed, so it is an ordinary unregister.v1.

Not checked: sign-in.v1, enroll-machine.v1 and refuse-machine.v1 beyond naming their calls out; cancellation of an engine verb under OPS-027, which has the same root as F1; an engine of several parts under DL17 beyond its text; spec/schemas, spec/core/results.yaml and the secrets, naming, manifest and sections files; the ruling records beyond a search for the restart sentence, which entered before round 6 and for which I found no ruling; the model-checking results. I ran no vector and did not test the first build's behavior; statements about it rest on its conformance document and the code lines cited. That etcd bounds the operations of one transaction is from memory and is not used in any finding.


## FR-556

**A restart gives every engine-only verb the outcome unknown, although a promise already fixes what the store holds for each**

GL3/F1 · needless-complexity · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** IMP-007, OPS-004, LIF-004, LIF-005, LIF-024, LIF-028, LIF-036, PLC-017, LIF-013, OPS-014

**Where.** spec/conventions/implicit.yaml:75

**Quote.** "An invocation that was in progress across a restart has the outcome unknown."

**Situation.** The sentence stands in an observable and has no exception. Eleven engine verbs make no call outside the engine once dispatched, and a twelfth makes one that its own text keeps out of the effect. For each, a promise already fixes the stored state, so the engine can tell.
- open.v1: LIF-004 (spec/conventions/lifecycle.yaml:65) leaves the state unchanged after an unknown setup, and LIF-006 (:92) forbids ready back to registered. The Conduit is registered; the true outcome is failed. So 'occurred' is never a true record of an open cut by a restart, and LIF-028 (:597) then performs the open (L02/F2, X06/F9).
- close.v1, and retire.v1 of an instance with no hook and no run, such as a Fleet (spec/promises/templates.yaml:159; spec/promises/fleets.yaml:18): LIF-005 (:77) retires at dispatch, LIF-024 (:518) confirms a close 'with whatever outcome', LIF-035 removes. The true outcome is succeeded. A keyed repeat is a new attempt that reports invalid under OPS-029; 'did not occur' records failed for a Conduit that is gone.
- declare-lost.v1: 'takes effect at once and dispatches nothing' (spec/promises/placement.yaml:236).
- register.v1 and register-replacement.v1 (place.v1 runs before dispatch, spec/promises/providers.yaml:146), install.v1 (the fetch runs before dispatch, PKG-032), replace.v1, lift.v1, answer.v1, purge.v1, reconcile.v1: the store holds the objects, the package, the revision, the tombstone, the answer, the purge's record or the reconciled outcome, or it does not.
- unregister.v1 calls stop.v1, but 'the outcome of the request is that of the removal' (lifecycle.yaml:766) and the stop 'is not part of the effect' (spec/core/operations.yaml:44).
Cost today: each such invocation stays in `unreconciled` of `management`, `access` or the owning Pool or Fleet (lifecycle.yaml:242) until an actor asserts what the store could answer; reconcile.v1 takes that word against the store (succeeded for a registration that stored nothing); and reconcile.v1 is itself one of the twelve, so one cut by a restart needs a reconciliation of its own. Without a key the way on is a new request, which meets the first attempt's state (F4). The first build cannot reach the state: it commits each such verb as one transition (engine/CONFORMANCE.md:17), and the four vectors that reconcile an unknown open.v1 or lift.v1 write the invocation into the state by a fixture (conformance/conventions/inventory/e4/reconcile-open-retired-consumer, reconcile-open-replaced-subject, bound-removal-reconcile, pool-bound-removal-reconcile; engine/crates/conformance/src/lib.rs:479). The dogfood harness restarts the engine for every fresh conversation (docs/implementation/HANDOVER.md:124, :148).

**Counterargument.** One rule for every invocation is short and safe: unknown never claims an effect. An engine whose store is not transactional cannot always tell. OPS-007's 'the implementation stops before returning one' can be read to cover the engine as its own implementation. A deduplicating keyed repeat already recovers open, install, declare-lost and purge.

**Least change.** IMP-007, observable, last sentence: 'An invocation that was in progress across a restart and had a call outside the engine in flight (a call to an implementation, a hook, a provider call, a read) has the outcome unknown. One of an operation that the engine implements and that had none ends with the outcome its stored state gives: succeeded where the effect is stored, failed where it is not.' It needs F2's rule that the effect, the invocation's state and the outputs kept under OPS-009 are one change, and L03/F1's order of checks for the OPS-009 repeat. unregister.v1 follows by LIF-036. Then, as one question for the owner: strike 'open' and 'close' from LIF-028 and `deduplicates: true` from the twelve core contracts, which nothing reaches any longer.

**Precedent.** A database transaction: after recovery it is committed or it is not. etcd's transaction, which DL17 already cites; the Kubernetes API server answers a write only once etcd has committed it.

**Who decides.** coordinator. A defect the coordinator may fix (rulings 37; 46 point 4): LIF-004, LIF-005, LIF-024, PLC-017 and LIF-036 already fix the state, so the sentence contradicts what unknown means (spec/definitions.md:377), and the first build never reports the outcome the sentence gives, so no build or vector changes. The text of a reported outcome changes: log it for the owner to strike and add it to the review list. Striking the marks and LIF-028's open and close removes a feature and is the owner's (docs/AUDIT.md:124-125).

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/implicit.yaml:75, in IMP-007's observable with no exception. It reads against definitions: unknown is 'The one word for "cannot tell"' (spec/definitions.md:377). For the engine-only verbs the stored state is already fixed. open.v1's effect is 'Sets the Conduit's state to ready' (spec/core/operations.yaml:785). LIF-004 keeps an unknown setup's state unchanged, and a Conduit's setup is its open (spec/conventions/lifecycle.yaml:54-55, :62-67). LIF-005 retires at dispatch (:76-78). LIF-024 confirms a close 'with whatever outcome' (:517-518). A Fleet's retirement 'dispatches nothing and succeeds at once' (spec/promises/templates.yaml:158-159; spec/promises/fleets.yaml:18). Twelve core contracts declare `deduplicates: true` (`grep -n 'deduplicates: true' spec/core/operations.yaml`: lines 182, 240, 269, 549, 602, 668, 690, 714, 787, 823, 1318, 1362), and reconcile.v1 is one of them. In the first build the case cannot arise: each step is committed atomically (engine/CONFORMANCE.md:17), and recover() marks every in-progress invocation unknown (engine/crates/core/src/requests.rs:1785-1800). The reconcile-open vectors write the unknown open.v1 into the state with a lifecycle fixture (conformance/conventions/inventory/e4/reconcile-open-retired-consumer/requests.json:7,14; case.yaml:640). HANDOVER.md:123 and :148 confirm a restart for each fresh conversation. The point itself is the critic's second sampled entry (critic.json sampled[1]). It is not in AUDIT.md's known-open list, cold-readers.md, spec/README.md's limits, IN_FORCE.md's Open lines, the review list or the increment-4 log (grep for restart and IMP-007).

**Verifier's corrections.** The core holds, but the downstream claims do not. (1) Other routes still produce an unknown engine verb, so striking open and close from LIF-028 and the twelve `deduplicates` marks does not follow. OPS-027 cancels 'a part whose implementation is `engine`, such as an `open.v1` under CON-004' and ends it 'with the outcome that this promise gives it', which is unknown unless confirmed (spec/conventions/operations.yaml:608-609, :624-630, :636-638). L02/F2's verifier already named this way to an unknown open. DL17 also ends an invocation on a stopped part unknown 'as under IMP-007' (spec/implementation-defined.yaml:208-210), and the fix has to reach DL17 too. Changing IMP-007 alone therefore does not close the L02/F2 and X06/F9 routes. (2) unregister.v1 is misclassified. Its stop is a call out of the engine, and the instance is removed only 'once the stop has ended' (spec/conventions/lifecycle.yaml:746, :762-766). Under F1's own first sentence, a restart during that stop ends it unknown, so its mark stays reachable. Yet the stop 'is not part of the effect' (spec/core/operations.yaml:43-45), so the test should be whether the effect waits on an outside call, not whether one is in flight. (3) Class: closer to a defect (unknown where the engine can tell, against definitions.md:377) than needless complexity, though either is defensible. (4) For open.v1 there is a further point. In an engine that stored the open apart from its end, LIF-004 (state unchanged) and LIF-006 (no return from ready to registered, lifecycle.yaml:91-93) cannot both hold. That supports F2 and not this finding. Who decides stands: the coordinator for the IMP-007 sentence, logged to strike; the owner for any removal.


## FR-557

**No promise says that an engine verb's stored effect is whole or absent after a restart**

GL3/F2 · ambiguity · unsafe · verifier: weakened, medium · decides: coordinator · new

**Promises.** OPS-004, IMP-007, PLC-017, SUB-006, FLEET-005, ACC-053, REC-004, CON-008, LIF-035, FST-001, DL2, DL6, DL17

**Where.** spec/conventions/operations.yaml:54

**Quote.** "An implementation cannot confirm that the effect occurred, cannot confirm that it did not, or knows that it occurred only in part."

**Situation.** Several engine verbs store many things: declare-lost.v1 (a node, every instance on it, their Conduits and tombstones), register.v1 (the objects, the nested instances and FLEET-005's default Conduits, made and opened 'within that registration', spec/promises/fleets.yaml:79), lift.v1 (the named tombstone and each one it caused), purge.v1 (the records and the record of the purge), a removal (the instance and its consumer's Conduits), a reconciled retirement (the outcome and the removal).
Reading A: dispatch stores the whole effect. LIF-005 says 'from that moment', PLC-017 'at once', SUB-003 and POOL-002 give all or nothing on a rejection, and the first build commits one transition (engine/CONFORMANCE.md:17).
Reading B: the steps are stored one after another. OPS-004 names an effect that 'occurred only in part' as a lawful unknown, IMP-007 forbids carrying the invocation on without a repeat, DL2 leaves the dispatch boundary to the engine, DL17 asks for a compare-and-set on the invocation's state and not for one change, and the word atomic occurs nowhere in spec/ (grep -rni atomic spec: no match).
Harm under B: (1) root sends declare-lost.v1 for a stolen machine's node N, which holds Agents A1 to A3; (2) the engine stores N's retirement, A1's retirement and the closing of A1's Conduits, then restarts; (3) the invocation is unknown; (4) root repeats, with or without its key, and PLC-017 answers that a request for 'a node that was already declared lost changes nothing and has the outcome succeeded' (spec/promises/placement.yaml:227); (5) A2 and A3 stay ready with their Conduits open and their credentials accepted, under a result that says succeeded.
The other partial states fail closed and have no way on. A Pool stored without root's default Conduit: a repeat reports conflict under OBJ-025 and root has no Conduit to its own Pool. A named tombstone lifted with its caused ones left: the repeat reports invalid, or refused to a bounded holder (spec/conventions/access.yaml:844). Records removed with no record 'stating which records were purged' (REC-004). First start is the same question with no request: DL6 leaves 'what an interrupted or repeated one does' to each engine (spec/implementation-defined.yaml:53), and the first build answers 'one atomic empty-store transition' (engine/CONFORMANCE.md:23).

**Counterargument.** Read strictly, each verb's `then` forbids the partial state: PLC-017 has the node and every instance retired, so an engine left as in step 2 has not kept it and must finish at recovery. A careful builder would commit once or finish, and root sees A2 and A3 ready on inspection. The unsafe mark rests on reading B being sound.

**Least change.** One convention promise beside OPS-008: 'What the engine stores for one step of an invocation of an operation that it implements (the objects, Conduits, tombstones, provenance, pending actions and records that the effect changes, the invocation's state and the outputs kept under OPS-009) it stores as one change: after a restart the change is stored in whole or not at all. Where such an invocation makes a call outside the engine, the result of the call is stored with what the engine does on it.' Observable: after any restart no object, Conduit, tombstone or record exists that an invocation made in part. DL2 keeps how; DL6 loses 'what an interrupted ... one does': an interrupted first start is not stored and runs again.

**Precedent.** A database transaction (atomicity). Stripe's idempotency keys, where the key's record and the local effect commit in one transaction and a foreign call sits between two such phases.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity settled toward the reading under which each verb's own `then` holds, which is the first build's stated choice under DL2 and DL6. It constrains the store and adds no feature. Logged for the owner to strike.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/operations.yaml:55-56; the promise begins at :51, and the citation :54 is the `when` key. `grep -rni atomic spec` has no match. DL2 says nothing on storing many things at once (spec/implementation-defined.yaml:18-24). DL17 asks for a compare-and-set and for a commit 'as an etcd transaction' only for decisions that admit one of several competing changes (:201-208). DL6 leaves 'what an interrupted or repeated one does' to each engine (:53). The first build's choices are not binding (engine/CONFORMANCE.md:17 and :23). Each verb's own `then` already binds the whole effect, whatever the outcome. PLC-017's `when` is 'The root Interface declares a node lost' and its `then` is 'The node and every instance placed on it are retired' (spec/promises/placement.yaml:216-219), and the declaration 'takes effect at once' (:236). SUB-006: 'A submission is accepted. ... The engine also makes and opens root's default Conduits' (spec/promises/submissions.yaml:97-101). For records the text already settles a crash. REC-003: 'The engine's own crash and restart are its acts, as are every write it makes to the storage of its records, including one interrupted by its crash' (spec/conventions/records.yaml:103-104). REC-004 requires a record 'stating which records were purged' (:123-124), so a purge cut between its removals and its own record is decided: it breaks REC-004. New: no first-pass finding, known-open list or log has it. X09/F1 is about a store that cannot be written, and X10/F12 about a store restored from an older copy.

**Verifier's corrections.** Not unsafe. The harm path needs an engine that, after recovery, leaves A2 and A3 ready. That engine breaks PLC-017's `then`, which binds on the declaration and not on its success, so it is not an engine that keeps every promise. This is the same correction X09/F1's verifier made for builder C. No promise forbids finishing the stored effect at recovery: IMP-007 forbids dispatching again, and the engine's own store write is not a dispatch to an implementation. A builder who also reads IMP-007 as forbidding that finish has no lawful state left except atomic storage. Class: a gap or omission rather than two sound readings. The whole-effect rule follows from each verb's `then` but is stated nowhere for state, while REC-003 states it for records, and OPS-004's 'occurred only in part' invites reading B. The purge example in the situation is already decided (REC-003, REC-004). The fail-closed partial states (lift, Pool without its default) are real undecided ways on, low. The least change stands, and REC-003's sentence is its in-spec precedent: extend 'the engine's own crash ... are its acts' from records to the engine's stored state. Narrowing DL6 constrains a deliberate implementation-defined freedom. It is still the coordinator's under ruling 37, logged to strike, because the FST promises already fix the end state.


## FR-558

**For the verbs the engine carries out in parts, IMP-007 says unknown where OPS-022 and LIF-021 compute the outcome from the parts**

GL3/F3 · contradiction · verifier: confirmed, medium · decides: owner · new

**Promises.** OPS-022, LIF-021, IMP-007, LIF-024, OPS-012, OPS-024, OPS-018, DL2

**Where.** spec/conventions/operations.yaml:465

**Quote.** "Its outcome is succeeded only if every part not marked optional succeeded, failed only if no part had an effect, and otherwise unknown."

**Situation.** set-up.v1, retire.v1 and the two cascades call out of the engine (reads of secrets, run.v1 and stop.v1, a `remote` connection, the hooks) and are in progress for as long as those calls take. Three restarts:
(1) A retirement is dispatched and the engine restarts before the hook or the stop is sent. OPS-022's observable (:471-474) gives failed, since no part was dispatched, and LIF-024 then reports the shutdown pending (spec/conventions/lifecycle.yaml:505). IMP-007 gives unknown and LIF-024 reports unknown (:502). The first build takes IMP-007 (engine/CONFORMANCE.md:268; engine/crates/core/src/requests.rs:1793-1800).
(2) A setup restarts before any part is begun, or while it connects to a `remote` item, a part 'of an operation whose contract states no effect' (:473-476): OPS-022 gives failed, IMP-007 unknown.
(3) A cascade restarts while members are being retired. LIF-021 gives failed with diagnostics that name the remaining members, or the outcome of the container's retirement (lifecycle.yaml:434-438); IMP-007 gives unknown. Neither cascade declares `deduplicates`, so a repeat with the same key reports unknown and dispatches nothing (OPS-012), against 'a request made again finishes what this one could not' (spec/core/operations.yaml:733-734). With LIF-021's failed the same repeat is a new attempt (OPS-024).
Unknown is right for the part that left the engine and had not answered: a hook, run.v1, stop.v1 inside a retirement, an account or sign-in hook, accept-machine.v1, enroll.v1, remove-machine.v1. It is not right for the whole verb where every part's state is stored. Two builders report different outcomes, different shutdowns (pending or unknown) and different setup actions under PND-006.

**Counterargument.** IMP-007 is the rule written for a restart and the first build follows it. The outcome can be computed only if the engine stored each dispatch before sending the call, which DL2 leaves open. Unknown is the cautious answer, and a further setup or retirement goes on either way.

**Least change.** Option A, recommended. OPS-022 and LIF-021, observable: 'A setup, a retirement or a cascade that was in progress when the engine restarted has ended, and its outcome is this promise's, from its parts as stored: a part that had a call outside the engine in flight ended unknown, and a part not begun was not dispatched.' OPS-008: 'The engine stores that it is about to send a call before it sends it', which OPS-018 already needs for a keyed request. Option B: OPS-022 and LIF-021 add 'or unknown where the engine restarted while it was in progress'.

**Precedent.** A saga whose steps each keep a durable outcome. Terraform: an apply cut part-way is an error, its state keeps what each step made, and the next apply continues.

**Who decides.** owner. Option A changes what the first build reports (failed in place of unknown, a shutdown pending in place of unknown), which is behavior (docs/AUDIT.md:122-125). Option B adds no behavior and the coordinator could write it under ruling 37. The choice is one lettered question, with F1's removals.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:465-466. OPS-022's `when` is 'An invocation of `set-up.v1` or of `retire.v1` has ended' (:463), and an invocation ended by recovery has ended. Its observable gives failed where 'every part that was dispatched either failed ... or is of an operation whose contract states no `effect`' (:471-474). That holds vacuously when no part was dispatched, and a `remote` connection counts as a no-effect part (:475-477). IMP-007 gives unknown (spec/conventions/implicit.yaml:75). LIF-024 gives a shutdown of unknown or pending from that outcome (spec/conventions/lifecycle.yaml:502-505). LIF-021 gives a cascade failed while members remain unconfirmed (:434-436). The quote 'a request made again finishes what this one could not' is verbatim at spec/core/operations.yaml:733-734. Neither cascade declares `deduplicates` (grep shows none at :720-746 or :1323-1342). The first build takes IMP-007 (engine/crates/core/src/requests.rs:1793-1800; engine/CONFORMANCE.md:268). Further support the reviewer missed: for a cancellation of the same verbs the text already takes the outcome from the parts. OPS-027 reads 'The outcome of a setup or a retirement is then OPS-022's, and that of `retire-with-resources.v1` LIF-021's' (spec/conventions/operations.yaml:635-636). So a restart and a cancellation of one composite verb are given different rules. Not found in INDEX.txt (L03/F7 and L02/F8 are other situations of OPS-022) or in any known-open record.

**Verifier's corrections.** Stands as stated. Add OPS-027:635-636 as the text's own precedent for Option A: it makes A a consistency fix, and it means A and IMP-007 differ only where no effectful part was in flight. The point on the keyed repeat is narrower than stated. 'A request made again' can be a new request without the key, and that one does finish. Only a client that keeps its key is stuck. Not unsafe: pending and unknown both block removal. Who decides as stated: Option B is the coordinator's under ruling 37, and Option A changes first-build outcomes and goes to the owner, with OPS-027 noted in its favor.


## FR-559

**A keyed repeat of an unknown engine verb is turned away by what its own first attempt stored, for eight of the twelve verbs that declare `deduplicates`**

GL3/F4 · gap · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** OPS-012, CTR-002, OPS-008, OPS-029, OBJ-025, FLEET-005, IFC-014, REV-002, PND-003, OPS-030, LIF-029, ACC-053

**Where.** spec/conventions/operations.yaml:281

**Quote.** "The repeat is dispatched as a new attempt only if the operation's contract declares `deduplicates: true` and the implementation is not marked `plain`."

**Situation.** The core package declares the mark 'where the whole effect is a change to what the engine itself stores, which the engine can recognize by the idempotency key' (spec/core/operations.yaml:39-40). No promise says how. OPS-008's list (:155-160) keeps nothing of what an invocation made, and the checks before dispatch read the state the first attempt left. Where that attempt stored its effect before the restart:
- register.v1: conflict under OBJ-025, naming its own objects. For a submission that makes a Pool or Fleet it is invalid, which OPS-023 puts first, because root 'already has a Conduit that is not closed with that local name' (spec/promises/fleets.yaml:91-93): its own default.
- replace.v1: conflict under REV-002, since the repeat names the revision it advanced.
- unregister.v1 and close.v1: invalid under OPS-029, since the object is gone; ACC-012's observable covers only the OPS-009 and OPS-032 repeats (spec/conventions/access.yaml:218-221).
- lift.v1: invalid, or refused to a bounded holder, 'which learns nothing' (access.yaml:844-845).
- answer.v1: conflict under PND-003. reconcile.v1: conflict under OPS-030. register-replacement.v1: conflict under LIF-029 or POOL-003.
In each the invocation stays unknown and the caller is told that someone else changed the state. Only open.v1 (ACC-006), install.v1 (PKG-027), declare-lost.v1 (PLC-017) and purge.v1 (an absolute time) are safe to send again, by their own promises and not by the mark. So the mark works only where the first attempt stored nothing. Even where the engine could recognize its key, it has no outputs to give: register.v1 requires `objects`, lift.v1 `remaining`, replace.v1 `revision`.

**Counterargument.** X07/F6's verifier holds that OPS-002 and OPS-007 already oblige a deduplicating implementation to answer succeeded with its outputs. The caller can inspect and reconcile. In the first build the case cannot arise.

**Least change.** With F1 the case is gone: an engine-only verb is never unknown, a repeat of one that succeeded is OPS-009's and of one that failed OPS-024's, and the twelve marks can be struck. Without F1, add to OPS-012: 'A new attempt of an operation that the engine implements, whose earlier attempt stored its effect, applies nothing and has the outcome succeeded with the outputs of that effect; no check reads the state that the earlier attempt stored.' And add to OPS-008's list what the engine recognizes its own effect by.

**Precedent.** The IETF Idempotency-Key draft and Stripe: a retry of a completed request is answered with the stored result of the first, without judging it against the state it changed.

**Who decides.** coordinator. Rulings 37 and 46 point 4: it writes what CTR-002 already says a deduplicating implementation does, for the one implementation the specification itself supplies, and adds no behavior. Striking the marks goes to the owner with F1.

**Verifier.** weakened, low. Quote verbatim at spec/conventions/operations.yaml:281-283. The core package declares the mark 'where the whole effect is a change to what the engine itself stores, which the engine can recognize by the idempotency key' (spec/core/operations.yaml:39-40). The pre-dispatch checks each turn a repeat away. OBJ-025: 'The registration is not dispatched, and reports conflict' (spec/promises/objects.yaml:458-459). FLEET-005's own default trips IFC-014 as invalid (spec/promises/fleets.yaml:91-93). A closed Conduit reports invalid under OPS-029, and ACC-012 answers only OPS-009 and OPS-032 repeats (spec/conventions/access.yaml:218-221). A lift reports invalid, or refused to a bounded holder (:844-845). answer.v1 conflicts on an answered action (spec/core/operations.yaml:1344-1350). The count of eight matches the twelve marks less open, install, declare-lost and purge. OPS-008's list keeps no made objects or outputs for an unknown invocation (operations.yaml:155-160). The finding is a new situation beside X07/F6, which assumes the call reaches the implementation.

**Verifier's corrections.** Conditional on F2's gap. The case needs an engine that stored an engine verb's effect apart from the invocation's end. An engine that commits both together, as the first build does (engine/CONFORMANCE.md:17), meets a state without the effect on a keyed repeat after an unknown restart, and the new attempt applies it normally. So F2's rule alone removes the case, not F1 as the least change says. Without F2 the gap is real for a promise-keeping non-atomic engine, and its harm is a stuck unknown with a misleading conflict or invalid diagnostic (ruling 52), not wrong access. Fold it into F2 as a consequence. The fallback text for OPS-012 is a restatement of CTR-002 for the engine's own verbs, which is the coordinator's, as X07/F6's verifier said of the derivable cases.


## FR-560

**Whether an engine-implemented invocation that a restart ended is fenced is said both ways in one observable**

GL3/F5 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** OPS-014, OPS-030, IMP-007, LIF-028, LIF-024

**Where.** spec/conventions/operations.yaml:370

**Quote.** "An invocation or part whose implementation is `engine` is fenced once it has ended; a setup, retirement, open or close is fenced only once every part it dispatched is fenced, under LIF-028 and OPS-022."

**Situation.** Ten lines earlier the same observable says: 'recovery of an in-progress invocation under IMP-007 fence nothing; another event described here must fence the attempt' (:360-361). An engine-implemented invocation is in progress at a restart and recovery ends it unknown.
Reading 1: it has ended, so it is fenced, and an actor may record that it did not occur.
Reading 2: recovery fences nothing, and for work inside the engine no other listed event exists (no answer, no run's end, no confirming result), so it is never fenced, and recording that it did not occur reports conflict under OPS-030 for good.
The first build takes reading 2 (engine/crates/core/src/requests.rs:1408-1411; the test at :2011-2047; the check at :969). Under it the only record the engine accepts is 'occurred', and LIF-028 turns that into a change. For a retirement cut before its stop was sent it confirms the shutdown 'whatever a later one ended with' (spec/conventions/lifecycle.yaml:511) and removes the instance with no stop (L02/F6's harm). For an open it opens the Conduit (L02/F2). The true record, that nothing occurred, is the one refused, and the entry leaves `unreconciled` only by the false one. Yet after a restart nothing of the engine's own work is in flight, which is the fence's own test: 'no copy of its dispatch remains in flight' (:315-316).

**Counterargument.** Reading 2 is the cautious one for a setup or retirement whose parts went out. On an engine of several parts a paused part may still act (DL17). The way on needs no record: a further setup or retirement. With F1 and F3 an engine-only invocation never ends unknown and the question shrinks to the verbs in parts.

**Least change.** OPS-014, observable: 'Recovery fences no attempt that was sent outside the engine. The engine's own part of an invocation is fenced by the restart: nothing of it remains in flight once the engine, or under DL17 the part that held it, has stopped and the store admits no write from it. A setup, a retirement or a cascade that recovery ended is fenced once every part it dispatched is fenced, and at once where it dispatched none.' Vector: restore-state on a retirement with no part dispatched, then a reconciliation that records it did not occur.

**Precedent.** A write-ahead log: after recovery a transaction with no commit record did not happen, and a process that has exited holds nothing in flight.

**Who decides.** coordinator. Ruling 37: an ambiguity inside one observable; the fix lets the true record be made and adds no feature. It changes what the first build answers, so it is a build finding fixed in the specification with a vector (ruling 46 point 4), logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:370-372. The rival sentence is verbatim at :358-361: 'recovery of an in-progress invocation under IMP-007 fence nothing; another event described here must fence the attempt'. Further support for reading 1 is OPS-014's own `then`: 'An attempt is fenced once no copy of its dispatch remains in flight' (:315-316). That test is met for engine work in a single-process engine that has stopped, so the observable's blanket sentence conflicts with the `then` for engine work. The first build takes reading 2. Its comment says 'Only actual execution observes the end of engine work: recover, missing completions and unconfirmed cancellation establish no fence' (engine/crates/core/src/requests.rs:1408-1411). Its test recovery_missing_and_unconfirmed_cancel_never_fence_engine_work is at :2011-2047, and the OPS-030 check at :969. LIF-024's 'whatever a later one ended with' is at spec/conventions/lifecycle.yaml:510-511. Not in INDEX.txt: X07/F8 is about whether fence marks survive a restart, and X07/F9 about remote servers and hooks.

**Verifier's corrections.** None on class, mark or who decides. One precision. The first build never fences a composite engine invocation (set-up, retire, open, close), restart or not (requests.rs:1405-1420), and order 077 recorded that as a build limit (docs/work-orders/077-spec-reconcile-fenced.md:373-376). The ambiguity in the text is therefore what matters for a second builder. Under DL17's several parts, the least change's 'the store admits no write from it' is the needed condition, since a paused part may still act (spec/implementation-defined.yaml:212-214).


## FR-561

**Removing an instance's state directory is outside the store's change, and no promise orders it or finishes it after a restart**

GL3/F6 · gap · verifier: weakened, low · decides: coordinator · new

**Promises.** PRV-029, LIF-035, LIF-036, LIF-017, OPS-029

**Where.** spec/promises/providers.yaml:975

**Quote.** "Removal or unregistration of the instance removes it with its contents (LIF-035)."

**Situation.** A removal, under LIF-035 after a retirement succeeded or under LIF-017 by unregister.v1, has the store's steps (the instance gone, its consumer's Conduits closed with their tombstones, the records) and one step on disk. The first can be one change (F2); the directory cannot be part of it.
Restart after the store's change and before the deletion: the instance no longer exists, so no request can name it (OPS-029), the engine 'keeps no state of the run' (spec/conventions/lifecycle.yaml:767), and the path 'is never reused for another instance' (providers.yaml:974). The directory stays, holding what an Agent package keeps there, its home and its login (providers.yaml:982-984), and no promise has the engine look for it.
Deletion first, restart before the store's change: a retired instance whose directory is gone, and a further retirement that asks for a run to serve the retire hook (TPL-018) starts that run in an empty one.
Under ruling 70 the Agents run unconfined as the engine's user, and 'a process under the same operating-system user may still read it' (providers.yaml:978), so a left directory keeps a removed Agent's login readable after its owner was told it is gone.

**Counterargument.** PRV-029 is a must, so an engine that leaves the directory has broken it, and a careful builder sweeps at start. The exposure is the stated limit that the engine's user reaches every state directory; a left directory only extends it past the removal. The window is short.

**Least change.** PRV-029, then: 'The directory is deleted once the instance's removal is stored. At each start the engine deletes every state directory whose instance no longer exists.' LIF-035 cites it.

**Precedent.** The kubelet's housekeeping removes the directories of Pods that no longer exist (orphaned pod directories).

**Who decides.** coordinator. Ruling 71 point 4: a small finding that adds one duty, a sweep at start, and cannot move S1's counts. Logged for the owner to strike.

**Verifier.** weakened, low. Quote verbatim at spec/promises/providers.yaml:975. 'keeps no state of the run' is at spec/conventions/lifecycle.yaml:767, and 'a process under the same operating-system user may still read it' at providers.yaml:977-978. The text decides the end state and forces the order. PRV-029's `then` has removal remove the directory, so an engine that leaves it after a restart has not kept the promise. Deleting first breaks 'Every later run, including after engine restart, gets the same directory' (:972-973) for a retired instance whose retire hook still needs a run (TPL-018). The first build already deletes after the durable removal and retries 'every later core commit and store reopen' (engine/CONFORMANCE.md:39). Not in INDEX.txt: L11/F3, X01/F14 and X08/F8 are about other runtime providers, and L02/F4 about LIF-019.

**Verifier's corrections.** Class: not a gap in what happens. PRV-029 decides that the directory is removed, and the forced order is store first, then disk. What is missing is only an explicit duty to finish the deletion after a restart, which a careful builder already derives and the first build does. Low. The least change is fine as a clarifying sentence (kubelet orphaned-pod cleanup is apt) under ruling 71 point 4 or ruling 37, and it cannot move S1's counts.


## FR-562

**An admission cut after accept-machine.v1 answered leaves a machine accepted, no finished node and an open enrollment, though the engine holds the provider's answer**

GL3/F7 · gap · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** ENR-007, ENR-006, IMP-007, OPS-012, REC-001, ENR-011

**Where.** spec/promises/enrollment.yaml:171

**Quote.** "Once it succeeds, the engine registers a node from core node, owned by system, names it under PLC-026 and sets it up."

**Situation.** admit-machine.v1 has one step outside the engine, accept-machine.v1, and then four in the store: the node registered, the node set up, the action answered admitted, the enrollment closed. L05/F6 has the case where the accept call's result is lost; there unknown is right, since the call left the engine.
New: the call answers succeeded, its end is recorded (REC-001), and the engine restarts before the node is stored, or after the node and before the answer and the close. IMP-007 gives unknown. The operation declares no `deduplicates`, so a keyed repeat reports unknown and dispatches nothing (OPS-012). A new admission reports conflict: check 1 wants the machine 'pending' (enrollment.yaml:132) and it is now accepted, and in the second case check 3 also fails, because a node that is not retired has the hostname. In the second case a node stands in the state registered beside an open enrollment whose action nobody can answer admitted.
The engine's own state decides every part of this: the accept succeeded and the node is or is not stored. Nothing is granted, since a node gives no Conduit and no credential (ENR-011).

**Counterargument.** refuse-machine.v1 and a new enrollment recover the first case, as L05/F6's verifier found, and root can set the registered node up through `system`. The window is a few store writes wide.

**Least change.** F2's rule covers it with no text in ENR-007: the answer of accept-machine.v1 is stored together with the node, its setup, the action's answer and the close, as the first build stores a completion and what follows it in one transition (engine/CONFORMANCE.md:17). The state 'accepted and recorded, no node' then cannot be stored, and only L05/F6's lost result remains.

**Precedent.** Stripe's recovery points: the result of a foreign call and the local change that follows it commit together.

**Who decides.** coordinator. Rulings 37 and 46 point 4, with F2: it adds no behavior of its own. L05/F6's reconciliation sentence, which this does not replace, adds behavior and is logged to strike, as its verifier said.

**Verifier.** weakened, low. Quote verbatim at spec/promises/enrollment.yaml:171-172. Check 1 wants the machine 'pending' (:132) and check 3 a hostname that is not a non-retired node's (:135). admit-machine.v1 has no `deduplicates` (spec/core/operations.yaml:1495-1522, absent from the grep). ENR-007's `then` already binds the rest once the accept succeeded: 'Once it succeeds, the engine registers a node ... sets it up ... The action is answered admitted and the enrollment closes' (enrollment.yaml:171-174). The call's end is recorded under REC-001 (spec/conventions/records.yaml:18), so the engine holds the answer, and registering the node is no dispatch that IMP-007 forbids. Nothing is granted: 'No Conduit or credential is made' (:180-181). L05/F6, with its verifier's split, has the lost-result case.

**Verifier's corrections.** A sub-case of F2, not a separate finding. ENR-007's `then`, like PLC-017's, requires the whole end state once the accept succeeded, so a promise-keeping engine finishes or stores atomically. The undecided part is F2's general recovery question. Fold it into F2 as one of its situations, low, not unsafe. It adds no text of its own, as the reviewer says.


## FR-563

**A request that a restart cuts before dispatch, during an install's fetch or a registration's place.v1, has no end: no reason and no record**

GL3/F8 · gap · verifier: weakened, low · decides: coordinator · new

**Promises.** PRV-005, PRV-017, PKG-032, OPS-005, OPS-008, REC-001, IMP-007

**Where.** spec/promises/providers.yaml:146

**Quote.** "`place.v1` and `fetch-package.v1` are made while the engine decides a request to register or to install, before it is dispatched, and so carry its `caller` and no `invocation`."

**Situation.** The one call that install.v1 makes outside the engine, and the one that register.v1 and register-replacement.v1 make, come before dispatch. While they run no invocation exists and IMP-007 says nothing. A restart during a fetch, which nothing bounds (L09/F7, X10/F14), leaves three things. Nothing is installed, which PKG-024's text implies. A record that a provider call was made for the caller, and none that it ended. A request with neither an outcome nor one of OPS-005's four reasons, against 'Every request can be found with how it ended' (spec/conventions/records.yaml:25-26).
A keyed repeat finds no invocation and is a new request. That is the right result and is written nowhere: OPS-008's `when` includes 'a caller makes a request of a target that carries an idempotency key' (spec/conventions/operations.yaml:150), so an engine that keeps the key from the request's arrival would treat the cut request as an invocation in progress at the restart and answer unknown. Unknown would be wrong here: these calls state no effect and nothing is stored.

**Counterargument.** X09/F1's verifier held that stopping without an answer keeps every promise. The caller sees its connection drop and asks again; with nothing stored, nothing can go wrong. Low.

**Least change.** IMP-007, observable, one sentence: 'A request that the engine was still deciding when it restarted was not dispatched and began no invocation: nothing it asked for is stored, a repeat of it is a new request, and the record of a provider call made for it stands without an end.'

**Precedent.** Kubernetes audit: a request cut by an API server's crash leaves a RequestReceived event and no ResponseComplete.

**Who decides.** coordinator. Ruling 37: it writes what the text implies and adds no behavior.

**Verifier.** weakened, low. Quote verbatim at spec/promises/providers.yaml:146-148. Definitions decide the main reading: an invocation is 'What a dispatched request or hook begins' (spec/definitions.md:39, :210), and OPS-009 matches only 'the invocation that request began' (spec/conventions/operations.yaml:185). A request cut before dispatch therefore began no invocation, and IMP-007 cannot apply to it. A repeat is a new request, and an engine that answered unknown would be breaking definitions, not taking a second sound reading. What remains is the records. REC-001 asks that 'Every request can be found with how it ended' and that a provider call be recorded 'with how it ended' (spec/conventions/records.yaml:25-29), and neither can be met for a request a crash cut. The reviewer missed one sharper point. PKG-032 has the engine begin to decide an install 'only once every request to install that address and version that it began to decide before has ended' (spec/promises/packages.yaml:450-452), and a request cut by a restart never ends. L09/F7's counterargument assumes 'a restart of the engine ends the request', which no text says.

**Verifier's corrections.** The unknown-after-restart reading is refuted by definitions.md:39/:210. Keep the finding as a low gap in records and explicit ends, ruling 52: after a restart the records show a permitted access decision and a provider call made with no end, and nothing says what ended them. Extend the least change to say such a request 'has ended' for PKG-032, so that no engine keeps a later install waiting on it. Coordinator under ruling 37.


## Further points the verifier noticed

Not verified by a second reader.

- **A restart and a cancellation of the same composite engine verb get different outcome rules** (spec/conventions/operations.yaml:635-636 against spec/conventions/implicit.yaml:75). OPS-027: 'The outcome of a setup or a retirement is then OPS-022's, and that of `retire-with-resources.v1` LIF-021's.' A restart of the same invocation is unknown under IMP-007 whatever its parts did. This is F3's contradiction seen from the text's own precedent, recorded here so the owner's question can cite it. It is evidence for F3, not a separate finding.
- **The spec already makes a crash-interrupted write the engine's own act, but only for records** (spec/conventions/records.yaml:103-106). REC-003: 'The engine's own crash and restart are its acts, as are every write it makes to the storage of its records, including one interrupted by its crash'. No such sentence covers the engine's stored state (objects, Conduits, tombstones, invocation state). This is the in-spec template for F2's least change, and it already decides F2's purge example.

## The verifier's notes

I checked the eight findings at commit 2eb8f258, read-only. Two are confirmed (F3, F5) and six weakened (F1, F2, F4, F6, F7, F8). None is refuted outright, but the alternative reading in F8 is refuted by spec/definitions.md:39 and :210. Every quote is verbatim; only F2's line is off by one (the `when` key at :54, the text at :55-56).

Main corrections:
- F2 is not unsafe. Its harm path needs an engine that breaks PLC-017's `then`, which binds on the declaration whatever the outcome. The whole-effect rule follows from each verb's `then` but is stated nowhere for state, though REC-003 states it for records.
- F1's core point holds, but its downstream removals do not follow. Striking open and close from LIF-028 and the twelve `deduplicates` marks fails because OPS-027 still ends a cancelled engine part, such as open.v1 under CON-004, unknown, and DL17 ends a stopped part's invocation unknown. unregister.v1's in-flight stop keeps it unknown under F1's own test, although the stop is no part of its effect.
- F4 and F7 are consequences of F2's gap and should fold into it. F4 arises only for an engine that stores an engine verb's effect apart from its end.
- F6 is decided in what happens (PRV-029 removes the directory; the order is forced); only the duty to finish after a restart is unwritten, and the first build already does it.
- F8's unknown reading is refuted by the definition of invocation. A sharper point remains: PKG-032's 'has ended' for a request that a restart cut.
- F3 gains support from OPS-027:635-636. F5 gains support from OPS-014's own `then` ('no copy of its dispatch remains in flight').

Coverage. Read in full: spec/conventions/implicit.yaml, spec/promises/fleets.yaml, docs/guides/full-review.md, and the first pass's L02 findings F2 and F5 to F8 with that file's verifier notes, plus L05/F6, X06/F9, X07/F6, F8 and F9, X09/F1, X10/F11 and critic.json's sampled entries. Read in part:
- spec/conventions/operations.yaml lines 1-520 and 596-660.
- spec/conventions/lifecycle.yaml lines 1-130, 225-250, 360-445, 480-640 and 715-775.
- spec/core/operations.yaml lines 1-80, 185-245, 718-828 and 1289-1366, with a grep of every `deduplicates` and contract name.
- spec/promises/placement.yaml 205-262, enrollment.yaml 118-200 and providers.yaml 120-160 and 955-1005.
- spec/conventions/records.yaml 15-125, spec/implementation-defined.yaml 1-56 and 194-243, and access.yaml 205-224 and 838-850.
- SUB-003, SUB-006, OBJ-025, PKG-032, TPL-009, and the definitions rows for unknown, invocation, dispatch, attempt and `deduplicates`.
- engine/CONFORMANCE.md rows DL2, DL6 and DL16 and line 268; engine/crates/core/src/requests.rs lines 30-60, 955-985, 1395-1420, 1785-1800 and 2005-2050.
- One reconcile-open vector fixture, and HANDOVER.md 120-150.

Known-open checks were by grep for restart, IMP-007, atomic, interrupted and related words in INDEX.txt and the first-pass files, docs/AUDIT.md, cold-readers.md, spec/README.md's limits, IN_FORCE.md, the review list, the increment-4 log, G5.md, the ledger, ruling records 41 and 42, and order 077.

Not read: the other spec files in full, the other vectors, the shell's code, and ruling records beyond those greps.
