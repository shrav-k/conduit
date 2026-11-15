# L03-operations: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The base of this slice is sound and follows existing practice: three outcomes, four reasons in one order, an idempotency key with a stated scope, records modelled on audit events. The faults are in what was built on it. The repeat rules form a complete decision table, but it is written out about a dozen times in observables of 250 to 900 words, and the one thing it lacks, where a repeat stands against checks on state that its own invocation changed, makes the lost-result path fail for controls, retirements and reconciliations (F1). Unknown is over-produced: reads, cancelled calls and, under IMP-007 outside this slice, the engine's own atomic verbs at a restart all become items that only an actor can clear, while the fence that guards recording failed can rarely be established over HTTP (F2, F5), and one of its evidence rules does not prove what its definition asks (F3). The weakest point is reconciliation as an operator's task: the engine holds the late answer, the fence state and the invocation's particulars, and shows the actor an ID (F4). On records, REC-002 fixes who the caller is in ten cases and not which object a record is about, content kept for a target is readable through its caller (F8, F9), and nothing bounds what is kept or returned (F10, F11; round 6's D19 on invocation state kept forever still stands and is not repeated). I would cut OPS-006, which is IFC-009; move OPS-007's wire cases to the interface layer and OPS-029 and OPS-036 to access; split OPS-014 into a short promise and a table of fencing events; and state each repeat rule once. The layer is right in kind, since every part must share these behaviors, but 38 promises with some 7,100 words of observable are not a few behaviors stated as invariants.

**What the reader read.** Commit 2eb8f258 at the repository root, read only; nothing built or run.

Read in full, line by line: spec/conventions/operations.yaml (886 lines, 32 promises), spec/conventions/records.yaml (177 lines, 6 promises), spec/definitions.md (467 lines), spec/implementation-defined.yaml (257 lines), docs/guides/full-review.md, and AGENTS.md "What holds".

Read in part: spec/README.md lines 1-98 (status, what this is, the three layers), 272-337 (form of a promise, conformance) and 331-618 (all of "Limits of this version"); not its reading paths, contents table (grep only) or lines 619-678. IN_FORCE.md topic 7 in full and a grep of the rest for operations, records, Open and Spec pending lines; not the whole file. docs/AUDIT.md "Known to be open" in full. docs/guides/cold-readers.md "What is known to be open" and the protocol sections in full.

Cited text opened and read whole: IFC-009, IFC-010, IFC-016, IFC-023, IFC-025, IFC-026, IFC-028, IFC-032, IFC-046, IFC-050; ACC-014, ACC-019, ACC-033, ACC-034, ACC-036, ACC-037, ACC-055, and ACC-012's observable; REV-002, REV-010, REV-014, REV-015; LIF-004, LIF-013, LIF-028, LIF-029, LIF-037; CON-004; TPL-012; IMP-003, IMP-007, IMP-008; OBJ-002; PRV-005, PRV-009, PRV-010; CTR-002. Core package: the contracts inspect.v1, set-up.v1, retire.v1, reconcile.v1, purge.v1, open.v1, close.v1, setup-hook.v1, retire-hook.v1, inspect-run.v1, list.v1 (inputs and the start of its outputs), the schemas condition.v1 and record.v1, and "What this package does not settle" in spec/core/README.md.

Checked for novelty, by grep or part reads: docs/proposals/first-build/log-increment-4.md (4-04, 4-12, 4-15, 4-20), to-review-with-the-owner.md, docs/LANGUAGE_DEFECTS.md, experiments/012-model-checking/G5.md (MC2 rows), experiments/015-s4-partial-setup/README.md lines 196-257, docs/proposals/ruling-75/README.md (A1 and its later answer), docs/proposals/ruling-62/README.md (C6 to C12), and docs/proposals/reaudit/round-6/design-critique.md (headings, D10, D19, the four open points, what to cut). Three findings considered and dropped as already recorded: reconciling a partial effect (experiment 015 G1 and Q5, answered under ruling 75 D), invocation state kept without end (round 6 D19), and how a gap in records is shown (spec/core/README.md).

Not read: the other promises the slice cites by ID, known only through the rows of definitions.md, among them IFC-006, 007, 008, 012, 017, 019, 020, 033, 036, 041, 045, 047; ACC-001, 005, 008, 011, 015 to 018, 039, 044, 053, 056; LIF-005, 009, 017, 021, 024, 026, 031 to 036, 040, 041; POOL-013; PRV-007, 019, 021, 025 to 027, 030; the SEC, PND and SCH promises; TPL-009, 013, 017, 018; REV-006, 007, 011 to 013; PLC-017, 023, 024. Conformance vectors beyond the headers of three cases (read-missing, cancel-confirmed, cancellation); no engine or package code; the Model Context Protocol itself was not opened, so statements about it rest on the specification's own text and the cold readers' dated check. F3 rests on reasoning about gates that no promise or experiment here tests, and is marked low confidence for that reason.


## FR-020

**A repeat that is answered without a dispatch has no place in the order of checks, so the state its own invocation changed turns it away**

L03/F1 · contradiction · verifier: confirmed, high · decides: coordinator · new

**Promises.** OPS-009, OPS-023, OPS-029, OPS-030, OPS-034, REV-002, REV-014, ACC-012, ACC-033, LIF-037

**Where.** spec/conventions/operations.yaml:193

**Quote.** "The request is a repeat, not a new request. Nothing is dispatched, and the result is that invocation's: the outcome succeeded, its outputs and any plain content."

**Situation.** Three reachable cases. (1) An actor sends a control with key K that names revision r1. It succeeds and the revision advances; the result is lost; the actor repeats with the same contents, so still r1 (other contents are conflict under OPS-010). OPS-009's `when` holds, and so does REV-002's (spec/conventions/revisions.yaml:27, a request that names a revision that is not current reports conflict). Neither excepts the other, and OPS-023 orders only the four reasons among themselves. One builder answers succeeded, another conflict. (2) A holder of a Conduit to a Pool retires a Resource with key K. It succeeds and the Resource is removed at once (LIF-035). The repeat's access decision is made anew (ACC-036); for an object that no longer exists ACC-033 names `management` alone (spec/conventions/access.yaml:665), so the repeat reports refused, and a holder of `management` meets OPS-029's invalid against OPS-009's succeeded. (3) A repeat of a `reconcile.v1` that succeeded meets OPS-030: the outcome is no longer unknown, so conflict. Only REV-014 (spec/conventions/revisions.yaml:251) and, for `close.v1` alone, ACC-012 (spec/conventions/access.yaml:219) say that the repeat's own promise answers. The definitions' rule that a caller that lost a result repeats its request therefore fails for exactly the verbs that change what they name.

**Counterargument.** REV-014 and ACC-012 show the intent, OPS-005 already excepts an OPS-009 repeat from having no outcome, and a builder may generalize. But REV-002, OPS-029, OPS-030 and LIF-037 carry no exception, OPS-034 itself ranks a repeat's conflict inside OPS-023's order (so after invalid), and case 2 is decided the other way by explicit text in ACC-033.

**Least change.** Add to OPS-023's `then`: "A repeat that OPS-009, OPS-012, OPS-032 or OPS-034 answers without a dispatch is matched once the access decision is made and before any other check: no promise that gives invalid, conflict or unavailable from the state as it now stands applies to it." In ACC-033 add: "A repeat whose invocation removed the object it names is authorized against the target that authorized that invocation, which OPS-008 keeps."

**Precedent.** The IETF draft of the Idempotency-Key HTTP header and Stripe's idempotent requests: a retry of a completed request is answered with the stored result of the first, without evaluating the request against current state again.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction whose fix follows what REV-014 and ACC-012 already state and adds no behavior. The ACC-033 sentence changes who is answered and is logged for the owner to strike.

**Verifier.** confirmed, high. Quote verbatim at spec/conventions/operations.yaml:193-194. REV-002 has no exception: spec/conventions/revisions.yaml:27-29 "A request to change an object names a revision that is not the object's current revision ... The change is not dispatched, and reports conflict." Only REV-014 excepts repeats (revisions.yaml:232-234 "the request is not a repeat that is answered without a dispatch, under OPS-009, OPS-012, OPS-032 or OPS-034"; 251-253), and ACC-012 does so for close.v1 alone (spec/conventions/access.yaml:218-221). OPS-030 (operations.yaml:723-729, 739-741 "A second reconciliation of one invocation reports conflict in this way, whichever way the first recorded the effect") and OPS-029 (operations.yaml:680-685) carry none. OPS-023 (operations.yaml:487-494) orders only the four reasons. The word "repeat" does not occur in spec/conventions/lifecycle.yaml. Not recorded: G5 lists REV-002 as holds, OPS-029 as static and OPS-030 as not modelled (experiments/012-model-checking/G5.md:279-286); nothing in AUDIT.md "Known to be open", cold-readers.md, IN_FORCE.md, the increment logs or the review list. Not binding, but it shows the divergence: the first build takes "the repeat wins" unstated (engine/crates/core/src/requests.rs:908-931 skips both preflights when an earlier invocation exists and nothing is redispatched; :949 "A kept repeat performs no new reconciliation"), so a builder who applies REV-002, OPS-029 and OPS-030 literally fails those cases.

**Verifier's corrections.** Case 2 is half right. OPS-008 scopes a key to its target (operations.yaml:152-155), and a request through `management` that names a Resource of a live Pool is refused (operations.yaml:701-703), so OPS-009 against OPS-029 arises only where the first request went to `management` (ACC-033 rows 2 and 5: unregister.v1, retiring an Interface, Pool or Fleet), not for "a holder of management" in the Pool example. For the Pool holder the result is decided: refused (access.yaml:663-667; operations.yaml:699-700). That rule is review-list item 1 (finding 2-08), confirmed by the owner (docs/proposals/first-build/to-review-with-the-owner.md:14, 32-33). Who decides: the OPS-023 sentence is the coordinator's; the ACC-033 sentence changes an access decision the owner confirmed and is the owner's question, not a logged fix. Class: cases 1 and 3 are contradictions on the literal text; case 2 for a Pool is a decided result that defeats the lost-result path.


## FR-021

**An operation that states no effect still ends unknown when no result arrives, and then waits for a reconciliation that has nothing to check**

L03/F2 · needless-complexity · verifier: confirmed, medium · decides: owner · new

Same fault, or overlapping: FR-043.

**Promises.** OPS-007, OPS-012, OPS-014, OPS-024, OPS-027, OPS-034, IFC-010, IFC-046, LIF-013

**Where.** spec/conventions/operations.yaml:281

**Quote.** "The repeat is dispatched as a new attempt only if the operation's contract declares `deduplicates: true` and the implementation is not marked `plain`. Otherwise nothing is dispatched, and the result gives the state unknown."

**Situation.** An Agent reads a file through a plain server reached by `remote`, with key K. The engine stops waiting (DL10), or the Agent's connection drops. OPS-007 and IFC-010 (spec/interface/implementation.yaml:230, "no result at all, is unknown") give unknown although the contract states no `effect`. The invocation is listed in the Resource's `unreconciled` (LIF-013) until an actor with a Conduit to its Pool that lists `reconcile.v1` records something, and there is no effect to check. It cannot be recorded failed: a remote server that never answered is never fenced (spec/conventions/operations.yaml:374). Recording that it occurred makes it succeeded with no outputs, so a later repeat reports conflict under OPS-034. A repeat with K is never dispatched, since a plain server never counts as deduplicating. Yet the same class (no effect, not a control, starts no model work) is treated as safe to send again in three places: IFC-010 gives failed for its errors, IFC-046 sends it once more, and OPS-024 dispatches a filtered repeat of a success. A further inconsistency: an unfiltered repeat of a succeeded read whose outputs are no longer kept reports conflict for good (OPS-034), while the same repeat with a filter is dispatched again.

**Counterargument.** Unknown is honest: the engine did not see the read end. An author can mark a read `deduplicates: true`, and a caller without a key simply asks again. But a plain server can never be so marked, plain servers are the common case, and every deployment then carries reads in `unreconciled`.

**Least change.** In OPS-007's `then`, and in IFC-010's last clause: "For an operation whose contract states no `effect`, that is not a control and whose contract is not marked `startsModelWork: true`, the outcome in these cases is failed." OPS-024 then dispatches a repeat. In OPS-034, let the first case (outputs no longer kept, no filter) dispatch such an operation as OPS-024 does.

**Precedent.** RFC 9110, sections 9.2.1 and 9.2.2: a safe or idempotent method may be sent again automatically after a failure to read its response. Also the class that IFC-010, IFC-046 and OPS-024 already use.

**Who decides.** owner. It changes which outcome a request has, which is behavior.

**Verifier.** confirmed, medium. Quote verbatim at operations.yaml:281-283. spec/interface/implementation.yaml:225-230: an error on an operation with no `effect` "is failed", while "no result at all, is unknown". operations.yaml:373-376: such an attempt "is never fenced: its invocation can be recorded as having occurred, or stays unknown under OPS-013, and the Agent's way on is a new request". operations.yaml:309-310: a reconciliation records "that the effect ... occurred or that it did not", which means nothing for a contract with no `effect`. spec/conventions/lifecycle.yaml:242-246: `unreconciled` lists every unknown invocation dispatched to the instance. The asymmetry holds: OPS-034's `when` (operations.yaml:820-825) against OPS-024's (operations.yaml:515-518). IFC-046 (implementation.yaml:1281-1283) and IMP-007 (spec/conventions/implicit.yaml:67-71) use the same class of operation as safe to send again.

**Verifier's corrections.** Mark known-new-situation, not new. The limit itself is recorded: ruling 75's later answer (docs/proposals/ruling-75/README.md:121-124 "A call to a remote server that never answers stays unknown, and the Agent's way on is a new request") and IN_FORCE.md:150. The new situation is the operation with no `effect`, which those records do not separate. Class and who decides as stated; the fix also touches IN_FORCE.md:22 ("An unknown outcome stays unknown"), which is one more reason it is the owner's.


## FR-022

**A report of `stopped` fences an attempt although a copy of its dispatch can still be waiting before the run**

L03/F3 · contradiction · unsafe · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-180.

**Promises.** OPS-014, OPS-018, OPS-024, PRV-009, PRV-010, PRV-025

**Where.** spec/conventions/operations.yaml:361

**Quote.** "An attempt sent to a run is fenced once the engine's own runner has seen its process end, or the runtime provider reports it `stopped` under PRV-009 in a report made after dispatch."

**Situation.** Steps to the harm. (1) The engine dispatches a keyed write to a run behind its provider's gate: a proxy, the provider's end of a tunnel, or a listening socket the provider holds. The call is accepted there and the server has not read it. (2) The run's process ends, and the engine stops waiting: unknown. (3) `inspect-run.v1` answers `stopped`, truthfully: no run is running or being started. OPS-014 counts the attempt as fenced. (4) An actor records that the effect did not occur: failed. (5) The provider restarts the run under PRV-010, or a further setup asks for it, and the held call reaches the new process, as a connection queued on a socket-activated service does. (6) The Agent repeats, and OPS-024 dispatches a new attempt. Both land: one keyed effect twice, against OPS-018. The same sentence's definition ("no copy of its dispatch remains in flight") is not established by the evidence it then accepts, and no promise on the gate (PRV-025) forbids holding a call across a restart or passing it to a later run.

**Counterargument.** Most gates fail a call when the server behind them goes away, and that error is an answer and fences rightly. HTTP proxies do not retry such requests. A careful builder may treat the definition as governing and not count a `stopped` report where it knows a copy is held. But the engine cannot see into a gate, and the evidence sentence is categorical.

**Least change.** In OPS-014: "or the runtime provider reports it `stopped` under PRV-009 in a report made after dispatch, once the engine has closed the connection on which it sent the attempt." In PRV-025 add: "A gate passes a call to the run that was running when it received the call, or fails it; it passes none to a run started later."

**Precedent.** RFC 9110, section 9.2.2: a proxy must not automatically retry a request with a non-idempotent method. Kubernetes shuts a node down before its Pods are made elsewhere, which DL17 already cites.

**Who decides.** coordinator. Rulings 37 and 46 point 4: OPS-014's evidence contradicts its own definition, which ruling 75 A1 fixed. The fix narrows when failed may be recorded and adds no feature; logged for the owner to strike.

**Verifier.** confirmed, medium. Quote verbatim at operations.yaml:361-363; the definition it must establish is at 315-317 ("fenced once no copy of its dispatch remains in flight"). Each step is permitted. `stopped` speaks only of the run: spec/core/operations.yaml:1029-1031 "only where it knows that no run that it started for the ID is running or still being started under an earlier `run.v1`". Restart after it: spec/promises/providers.yaml:332-333 (PRV-010) and spec/definitions.md:314 "A `run.v1` may answer with a run already running, including one restarted after a report of stopped". PRV-025 (providers.yaml:806-819) puts no duty on a gate about a call accepted for an earlier run, and the engine's token names the instance's ID, the same for every run (spec/implementation-defined.yaml:110-113), so a later run's gate admits the old call. New: "The run's end fences" is a coordinator's reading in order 077 (docs/work-orders/077-spec-reconcile-fenced.md:190-191, 204-207); ruling 75's later answer names two limits and this is neither (ruling-75/README.md:121-124).

**Verifier's corrections.** The unsafe mark stands, in a narrow form: it needs a gate or listener that outlives the run's process, a negative reconciliation between the stop and the restart, and a repeat. Hence medium, not high. Class: nearer a gap in the fence's evidence than a contradiction between promises. The least change does not close it: closing the engine's connection does not recall a call already accepted (a connection queued on a held listening socket is still accepted after the client closes), and the PRV-025 sentence does not cover a call that reaches the gate after the restart on a connection opened before it. What would settle it: let `stopped` fence only where the provider's contract states that its gate fails every call accepted before the run ended, or tie an attempt to one run. Who decides: narrowing order 077's reading is the coordinator's; a new duty on providers in PRV-025, or accepting this as a third limit of ruling 75 A1, is the owner's. See the extra for a more reachable route to the same fault.


## FR-023

**The actor who must reconcile is not shown what the engine knows: a late answer, whether the attempt is fenced, and after a purge even the operation**

L03/F4 · gap · unsafe · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** OPS-013, OPS-014, OPS-030, OPS-008, OPS-024, REC-001, REC-004, LIF-013, DL2

**Where.** spec/conventions/operations.yaml:352

**Quote.** "An answer that ends an attempt's call at the implementation fences it: a result, an error or a JSON-RPC error, including one that arrives after the outcome became unknown, which changes no outcome under OPS-013."

**Situation.** A keyed write runs past the time allowed for a call (DL10): unknown. A second later its result arrives and states succeeded. It changes no outcome, and it fences the attempt. What else is done with it is left to each engine (spec/implementation-defined.yaml:22, "what is done with a result that arrives late"), and REC-001 requires no record of it (spec/conventions/records.yaml:31, "Nothing else is required to be recorded"). The reconciling actor sees an ID in `unreconciled`, which holds IDs only (spec/core/operations.yaml:381). After a purge no record says which operation, caller or time the invocation had, though OPS-008 keeps them. Because the attempt is fenced, a reconciliation that records that the effect did not occur is accepted; the Agent's repeat is dispatched under OPS-024 and the write lands twice. The answer that said the effect occurred is the event that allowed recording that it did not. The actor also cannot learn whether an invocation is fenced except by asking and reading OPS-030's conflict, and an invocation's state can be asked for only by a keyed repeat or by inspecting its instance.

**Counterargument.** The owner ruled that unknown stays unknown and that nothing reconciles on its own; the actor answers for what it checked; an engine may record late answers under DL2. Round 6's D10 already asked that recovery carry enough portable facts, and finding 4-12 recorded lost late replies and was closed on longer deadlines. But here the engine holds the implementation's own statement and no promise has it kept or shown, which ruling 52 point 2 asks for.

**Least change.** Add to REC-001's `when`: "an answer arrives for an attempt after its outcome became unknown", the record holding the outcome it stated. In LIF-013 and `inspect.v1`, let each entry of `unreconciled` hold beside the ID the operation, the caller, the time, and for each attempt whether it is fenced and what a late answer stated, read from the state OPS-008 keeps. Whether a reconciliation that contradicts a late answer reports conflict is the owner's.

**Precedent.** Kubernetes status conditions and Events: the system reports what it observed and the operator decides. Stripe stores the result of the first execution of a keyed request even when the client stopped waiting.

**Who decides.** coordinator. Ruling 71 point 4 with ruling 52 point 2: a small addition to what is recorded and shown. The conflict rule adds behavior and goes to the owner.

**Verifier.** weakened, medium. Quote verbatim at operations.yaml:352-355. implementation-defined.yaml:21-22 leaves "what is done with a result that arrives late" to each engine; spec/conventions/records.yaml:31-32 "Nothing else is required to be recorded"; spec/core/operations.yaml:381-384 `unreconciled` items are `id.v1`; lifecycle.yaml:245-247. The first build drops the late answer's content (engine/CONFORMANCE.md:19 "A stale attempt or ended invocation answers `ignored`"). Already recorded in part: round 6 D10, accepted for S4 and S5 (docs/proposals/reaudit/round-6/DECISIONS.md:114 "whether the portable facts suffice is a question for the vectors"; design-critique.md:331-332 "A mistaken assertion can make a later retry inappropriate"); AUDIT.md:185-186 (shape of an inspection result); finding 4-12 closed with "v0 keeps `reconcile.v1` as it is" (docs/proposals/first-build/log-increment-4.md:23).

**Verifier's corrections.** Unsafe mark not sustained as stated. The double effect needs the reconciling actor to record that an effect did not occur when it did; OPS-014 has the actor answer for "what it checked" (operations.yaml:309), and that hazard is D10's, already recorded. What is new is narrower and real: since order 077 a late answer is what allows the negative record, yet no promise has it kept or shown, and after a purge the actor holds only an ID. Class stays gap, under ruling 52 point 2. Least change: turning `unreconciled` items from `id.v1` into objects breaks `inspect.v1`'s output for existing readers; an added member beside it is smaller. Requiring a record of the late answer turns DL2's choice into a promise. Both are at the edge of ruling 71 point 4: log them for the owner. Whether a negative reconciliation against a stated late answer reports conflict is the owner's, as the reviewer says.


## FR-024

**Passing a cancellation on over Streamable HTTP closes the only channel that could fence the attempt**

L03/F5 · gap · verifier: confirmed, medium · decides: owner · known-new-situation

**Promises.** OPS-027, OPS-014, OPS-021, OPS-022, OPS-030

**Where.** spec/conventions/operations.yaml:608

**Quote.** "The engine passes the cancellation on to the implementation by the protocol's means. The invocation's outcome is unknown, unless its implementation confirms that nothing was done."

**Situation.** On Streamable HTTP the protocol's means is closing the response stream, after which no answer can arrive, and OPS-014 says a cancellation with no confirmation fences nothing (spec/conventions/operations.yaml:358). So for every server the engine reaches over HTTP, which is every run of a runtime provider and every `remote` item, a cancelled attempt is fenced only when its run stops. For an instance that is set up that takes a retirement; for a `remote` item it never happens. A consumer's dropped connection counts as a cancellation (spec/conventions/operations.yaml:616). Concretely: root's client loses its connection during `set-up.v1` of a Resource whose setup hook runs under a runtime provider. The engine cancels the hook by closing its stream; the hook is unknown; every further setup ends unknown (OPS-021, OPS-022). If the hook did nothing, recording that reports conflict under OPS-030 until the run stops, so the instance can only be retired and replaced, or the actor records that the effect occurred in order to get on. The limit as written names only "a plain server reached by `remote` that never answered".

**Counterargument.** This is the conservative reading that ruling 75 A1 chose: without proof that the attempt is dead, failed is not recorded, and the stated way on is a new request. But a hook has no new request, and here the engine's own act removes the proof.

**Least change.** State the limit in full in OPS-014: "An attempt sent over Streamable HTTP whose stream the engine closed is fenced only by its run stopping, and one sent to a `remote` item by nothing." Then the owner chooses: (a) the engine does not pass on a cancellation that a dropped connection caused where the request carries an idempotency key, and lets the call end; (b) keep it, and add the case to "Limits of this version".

**Precedent.** Option (a) follows the Idempotency-Key draft and Stripe: the server finishes the first request and stores its result, and the client retries with the key. The Model Context Protocol lets a receiver ignore a cancellation.

**Who decides.** owner. Behavior; ruling 75 A1 and its later answer are the owner's.

**Verifier.** confirmed, medium. Quote verbatim at operations.yaml:608-609. `when` at 603-605 names "the close of the response stream on Streamable HTTP"; 358-361: "a dropped connection or cancellation with no confirmation under OPS-027 ... fence nothing"; 616-618; 636-638. A setup hook left unknown blocks every further setup: operations.yaml:432-441 (OPS-021) and 476-479 (OPS-022). A stop is asked for only by a retirement or an unregistration (providers.yaml:279-284; lifecycle.yaml:743-749). So a hook cancelled over HTTP can be recorded failed only after its instance is retired. Recorded: the two limits (ruling-75/README.md:121-124; IN_FORCE.md:150) and that cancellation fences nothing (order 077, lines 199-203). Not recorded: the hook, and that this covers every implementation reached over HTTP. docs/guides/full-review.md:169-170 lists the related dogfood friction as not yet a finding.

**Verifier's corrections.** Class: the outcome is decided by the text (unknown, and conflict on a negative record), so it is not a gap in the strict sense. The undecided part is the way on for a hook: "the Agent's way on is a new request" (operations.yaml:376) does not exist for one. The spec says "such as", so its text is not wrong, only narrower in its example than its reach. Known-new-situation and owner decides, as stated. Not checked here: the protocol's own text on cancellation over Streamable HTTP at the baseline revision, on which option (a) rests.


## FR-025

**OPS-027 says both that a cancelled invocation ends when the cancellation is sent and that a later answer can make it failed**

L03/F6 · contradiction · verifier: confirmed, medium · decides: coordinator · new

**Promises.** OPS-027, OPS-013, OPS-014

**Where.** spec/conventions/operations.yaml:636

**Quote.** "A cancelled part's invocation ends when the cancellation is sent, with the outcome that this promise gives it, so an onward request that names it is refused from then on, under ACC-019."

**Situation.** A setup hook on the engine's own runner, over stdio, is cancelled with its setup. Shortly after, its implementation answers with a result that states failed. Reading A: the part ended when the cancellation was sent, and no confirmation existed then, so its outcome is unknown; the answer arrives after the outcome became unknown and changes nothing (OPS-013, OPS-014), it only fences; the setup is unknown and the hook must be reconciled. Reading B: by the `then` ("unknown, unless its implementation confirms that nothing was done") and by spec/conventions/operations.yaml:611 ("or failed where the implementation's result states that outcome") the engine waits for the answer and records failed, and the next setup dispatches the hook again. For an invocation that is not a part, nothing says when it ends or how long the engine waits for a confirmation. The build's vector `requests-cancel-confirmed` takes B for a request.

**Counterargument.** "The outcome that this promise gives it" can be read to include a confirmation that arrives later, and OPS-014 lists the confirming result as a fence, which fits either reading.

**Least change.** Replace the sentence with: "From the moment the cancellation is sent, an onward request that names the invocation is refused, under ACC-019. The invocation ends when its implementation answers, with failed where the result states it, or when the engine stops waiting under DL10, with unknown."

**Precedent.** The Model Context Protocol's cancellation: a receiver may still answer, and the sender decides what a late response means.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a contradiction inside one promise; the fix chooses the reading the `then` states and adds no behavior.

**Verifier.** confirmed, medium. Quote verbatim at operations.yaml:636-638. Against it: the `then` at 608-609 ("unknown, unless its implementation confirms that nothing was done") and 611-612 ("or failed where the implementation's result states that outcome"). An ended invocation has exactly one outcome (operations.yaml:20-22), and an answer after unknown changes none (290-294; 352-355). So for a part, either the "unless" clause is dead (reading A) or an ended part changes outcome without a reconciliation (reading B). The vector exists (conformance/conventions/requests/cancel-confirmed/case.yaml:34) and gives failed where the confirmation arrives in the same step as the cancellation; it exercises no part.

**Verifier's corrections.** One sentence of the situation is refuted: for an invocation that is not a part the wait is stated, as implementation-defined (operations.yaml:638-639 "How long the engine waits for a result is DL3 and DL10 of the implementation-defined list"). The fault is confined to parts. Class: ambiguity inside one promise rather than contradiction. The fix picks reading B, which changes what a cancelled hook ends with (failed and dispatched again, against unknown and reconciled); still the coordinator's as a choice between two readings, logged for the owner to strike.


## FR-026

**OPS-022 gives a further setup that fails two outcomes: failed by the parts it dispatched, unknown by a part counted from before**

L03/F7 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

**Promises.** OPS-022, CON-004, POOL-013, LIF-004, LIF-013

**Where.** spec/conventions/operations.yaml:471

**Quote.** "the outcome is failed if every part that was dispatched either failed, was answered with a reason in place of an outcome under IFC-026, or is of an operation whose contract states no `effect`, and is unknown if any part succeeded with an effect or ended unknown on an operation that has one."

**Situation.** An Agent declares one Conduit and has an item that is run. Setup 1 opens the Conduit (`open.v1` states an effect) and `run.v1` fails: unknown. The actor sets up again. The Conduit's part is not dispatched and counts as succeeded (CON-004); `run.v1` is dispatched and fails. Reading 1, from the first clause: every part that was dispatched failed, so failed. Reading 2, from the second clause and the `then` (spec/conventions/operations.yaml:465, "failed only if no part had an effect", and a part not dispatched "counts with the outcome it has from before"): a part succeeded with an effect, so unknown. Under reading 2 no setup of an instance that has an open declared Conduit can ever be failed, and each try adds an entry to `unreconciled` and a setup action.

**Counterargument.** The sentence on a hook left unknown ("has the outcome unknown, even where it dispatched nothing") shows that parts from before count, which favours reading 2; "that was dispatched" may only exclude parts never reached.

**Least change.** Rewrite the second clause: "and is unknown if any part dispatched in this setup or retirement succeeded with an effect, or any part, dispatched now or counted from before, has the outcome unknown on an operation that has one."

**Precedent.** Kubernetes init containers and Terraform apply: a further run reports on what that run did; what earlier runs did is state.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity; the fix states one reading and adds no behavior.

**Verifier.** confirmed, medium. Quote verbatim at operations.yaml:469-473. `then` at 463-467: "failed only if no part had an effect ... a part that is not dispatched counts with the outcome it has from before". 479-483: a part that CON-004 counts as succeeded "counts as succeeded". spec/promises/conduits.yaml:55-56: "nothing is dispatched and the part counts as succeeded". `open.v1` states an `effect` (spec/core/operations.yaml:785). Both clauses of the observable hold in the scenario and give different outcomes. Not recorded: G5's MC6-001 sets OPS-022 against the retire-hook contract, not parts counted from before (G5.md:273).

**Verifier's corrections.** None of substance. Under either reading the first setup stays unknown and listed in `unreconciled`; the rewritten clause should say so, so that a failed second setup is not read as settling the first.


## FR-027

**Inputs and outputs that a target's type keeps in records are returned to whoever may inspect the caller**

L03/F8 · gap · unsafe · verifier: weakened, medium · decides: owner · known-new-situation

**Promises.** REC-001, REC-002, LIF-013, CTR-004, ACC-033

**Where.** spec/conventions/records.yaml:50

**Quote.** "The inputs and outputs of the operation need not be; what content is kept is stated by each type's contract, under `records` as CTR-004 gives it: the inputs where `inputs` is true, and the outputs where `outputs` is true."

**Situation.** A mailbox Resource R in Pool P has a type with `records: {outputs: true}`. Agent A of Fleet F reads R through a Conduit that applies a filter. Each record of a read holds A as caller and consumer, R as target, and the outputs. An Interface whose only Conduit lists `inspect.v1` on F inspects A (ACC-033 row 6A). LIF-013 returns "those that name the instance" (spec/conventions/lifecycle.yaml:248), so that Interface reads the mail A read. It is the consumer of no Conduit to R or to P, and no filter judged what it was given. Whoever may inspect the Conduit through `management` reads the same.

**Counterargument.** The inspector's Conduit to F does list `inspect.v1`, so a Conduit permits the read; it sees no more than A saw; the type's author chose to keep content; and the owner left filtering records by Fleet open until after S1 (ruling 62, C10). But the party that decides to keep content, R's type, is not the party whose Conduits then expose it, and nothing tells P.

**Least change.** Add to REC-002: "The `inputs` and `outputs` of a record are returned only to an actor that may inspect the record's target under ACC-033; any other reader is given the record without them."

**Precedent.** Kubernetes audit levels, where Metadata and RequestResponse are set per resource, and CloudTrail, where data events are kept apart from management events.

**Who decides.** owner. The scope of access is the owner's, and ruling 62 C10 is the owner's open point.

**Verifier.** weakened, medium. Quote verbatim at records.yaml:50-53. lifecycle.yaml:247-248 "The records are those that name the instance"; spec/promises/contracts.yaml:102-104; access.yaml:611 (row 6A). The facts hold. But it is recorded as the owner's open point: IN_FORCE.md:178 "*Open, after the S1 rerun*: 62 C6 to C12 ... records filtered by Fleet". Its source weighed this very question: docs/proposals/projects/brief.md:1528-1529 "Inputs a Type records are visible only to the caller's project and root", deferred at brief.md:340 "v0 does not filter records by Project (ruling C10)".

**Verifier's corrections.** Mostly known: open point C10, with C12 (reach beyond one hop). Unsafe mark not sustained: the reader holds an open Conduit that lists `inspect.v1`, LIF-013 defines inspection to include the records that name the instance, and it sees no more than the caller was given. Whether it should is what C10 leaves to the owner. The new part is small: outputs kept on the target type's choice, and that spec/README.md "Limits of this version" (372-398) states the inventory's visibility and not this. Least change within the coordinator's reach: state it there as a limit. The REC-002 sentence is the owner's, and it runs against the brief's own recommended option for inputs.


## FR-028

**A record of a management verb or of a change has no stated member for the object it acted on, and the record of a reconciliation none for what OPS-014 says it states**

L03/F9 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-242.

**Promises.** REC-001, REC-002, OPS-014, LIF-013, LIF-005

**Where.** spec/conventions/records.yaml:68

**Quote.** "A record of `run.v1`, `stop.v1` or `inspect-run.v1` names in the CloudEvent's `subject` the instance whose run it is, so inspecting that instance finds it, under LIF-013."

**Situation.** Root retires Resource R through Pool P. The members of `record.v1` (caller, consumer, conduit, target, operation, invocation, outcome) hold root, the Conduit, P and `retire.v1`. R is only in the input `instance`, and management keeps no inputs (spec/conventions/records.yaml:89). Does inspecting R return the record of its own retirement? The quoted rule implies that an input alone does not make a record name an object, so one builder says no; another, following `unreconciled` ("names it in an input or as its target"), says yes. The same question stands for set-up, register, unregister, open, close and declare-lost, and for the records of a lifecycle or revision change, for which no member is named at all. OPS-014 (spec/conventions/operations.yaml:320) has the record of a reconciliation state "the new outcome, the actor that reconciled it and what the actor checked": the reconciled invocation, its new outcome and `checked` are inputs of `reconcile.v1`, and no member of `record.v1` holds them.

**Counterargument.** `record.v1` lets an engine add members, and the core package's README leaves the CloudEvent's `source` and `type` to each engine, as the tracker's known-open list says of the shape of inspection and of the cause of a machine's loss. But here the difference is which records an inspection returns and where a reader finds who retired an instance, not a key's name.

**Least change.** Generalize the quoted sentence: "A record names in the CloudEvent's `subject` the object that the occurrence is about where that is not its target: the instance set up, retired, registered, unregistered or removed, the Conduit opened or closed, the object whose state or revision changed, and for a reconciliation the instance of the invocation reconciled. The record of a reconciliation holds in `inputs` the invocation, `occurred` and `checked`."

**Precedent.** CloudEvents 1.0 `subject`; the `objectRef` of a Kubernetes audit event; the `resources` of a CloudTrail event.

**Who decides.** coordinator. Ruling 71 point 4 and ruling 46 point 4: it extends an existing rule to the cases it leaves out.

**Verifier.** confirmed, medium. Quote verbatim at records.yaml:68-70. `record.v1` has no member for the object acted on (spec/core/results.yaml:186-205) and gives `subject` a meaning only for run.v1, stop.v1 and inspect-run.v1 (165-166). records.yaml:70-73 names the invocation and caller of a change record, not the object changed. operations.yaml:320: "A record states the new outcome, the actor that reconciled it and what the actor checked", for which no member exists. spec/core/README.md: "No contract of the core package states `records`". lifecycle.yaml:243-245 says of `unreconciled` "names it in an input or as its target", while 247-248 says of records only "that name the instance". Adjacent and known: spec/core/README.md ("what retired an instance"; `source` and `type`), and AUDIT.md:231 (PLC-024's portable field, A39). The reviewer's known-new-situation mark is right.

**Verifier's corrections.** Minor. "Management keeps no inputs" (records.yaml:89) speaks of `management`; for a retirement through a Pool the Pool's Type decides, and a Pool Type outside the core that states `records: {inputs: true}` would hold `instance`. Class, mark and who decides as stated.


## FR-029

**Every readable request that establishes no instance must leave a record that no portable read returns, and REC-001 has no conforming behavior once a record cannot be written**

L03/F10 · gap · verifier: weakened, medium · decides: owner · new

**Promises.** REC-001, REC-002, REC-004, IFC-032, DL8, DL10

**Where.** spec/conventions/records.yaml:73

**Quote.** "The record of a request from which the engine could establish no instance, under IFC-032, holds no caller, and says that none could be established."

**Situation.** The engine's address is an `https` URL that other machines reach. A scanner, or a client whose token has lapsed, sends `tools/call` requests. Each is answered 401 and "is recorded as refused all the same" (IFC-032, spec/interface/implementation.yaml:855). Such a record names no object, since no caller, Conduit or target was established, so `inspect.v1` never returns it and reading it is DL8. Only root's purge by time removes it, with everything else before that time. Nothing bounds their number. When the store fills, DL10 leaves the behavior to the engine, but REC-001 requires a record of every request that ends without being dispatched, the one turned away for lack of space included, and of the purge that would free space. No behavior keeps the promise.

**Counterargument.** Recording failed authentication is ordinary audit practice, and the tracker lists what an engine owes when it cannot store state as order 082's question for the coordinator. But that list names invocation state, tombstones, provenance and packages, not records, and here a party outside can force the case.

**Least change.** Add to REC-001: "An engine may count, in place of recording each, the requests from which no instance could be established; DL8 says how the count is read." And: "Where a record cannot be written, a request that would be dispatched is not dispatched and reports unavailable; requests so turned away are counted and not recorded; `purge.v1` is carried out all the same."

**Precedent.** The Kubernetes audit policy, whose level None drops chosen requests, and its batch backend, which counts dropped events; rate-limited logging of failed logins in sshd.

**Who decides.** owner. Dropping a required record removes a feature, which is the owner's. The second sentence is order 082's question, which the tracker gives the coordinator to settle and log.

**Verifier.** weakened, medium. Quote verbatim at records.yaml:73-75. implementation.yaml:872-876: "It is recorded as refused all the same". records.yaml:30-32 excepts only the request that cannot be read (implementation.yaml:993-994). Such a record names no object, so its reading is DL8 (records.yaml:112-113; implementation-defined.yaml:68-70). implementation-defined.yaml:88 leaves "behavior when storage is exhausted" to each engine, and line 3 says "A choice must keep every promise".

**Verifier's corrections.** First half stands and is new: a sender of well-formed `tools/call` requests with no valid credential adds records that no `inspect.v1` returns and only a purge by time removes. Second half is weaker. (a) "No behavior keeps the promise" is too strong: an engine that stops accepting requests when it cannot write a record keeps REC-001, the fail-closed choice that Linux auditd offers as `disk_full_action`. (b) What an engine owes when it cannot store is recorded (AUDIT.md:238-241; full-review.md:138); records are a new situation of it, so mark that half known-new-situation. Class: the first half is an unbounded obligation rather than a gap. Who decides as stated.


## FR-030

**The only portable read of records returns every record that names the object, each time, with no selection**

L03/F11 · needless-complexity · verifier: weakened, medium · decides: coordinator · new

Same fault, or overlapping: FR-115.

**Promises.** REC-001, REC-003, REC-006, LIF-013, OPS-036

**Where.** spec/conventions/records.yaml:110

**Quote.** "Inspecting an object lists the records that name it, under LIF-013, and an object that has been unregistered or removed is no longer inspected."

**Situation.** `inspect.v1` requires the output `records` (spec/core/operations.yaml:406) and takes no input but `object` and `expectedId`. REC-006 forbids presenting a result that omits records as complete (spec/conventions/records.yaml:171). In v0 root is the caller of nearly every management request and `management` the target of most: inspecting either returns every record since the last purge, and each inspection adds records that the next one returns. Plan and apply inspect objects to compute a plan, and an Agent that inspects the Agents of its own Fleet receives their whole histories in one tool result. `list.v1` likewise returns every item at once, a cursor being ruled out (spec/conventions/operations.yaml:875).

**Counterargument.** DL8 leaves feeds, positions and snapshots to each engine, so an engine can offer a better read beside this one. Histories in v0 are short, and a purge shortens them.

**Least change.** Give `inspect.v1` two optional inputs, `recordsSince` (a time) and `recordsLimit`, and have a result that stops short say where it stopped, which REC-006 does not count as a gap. For `list.v1`, allow an opaque cursor over the items the actor may inspect: it discloses no count of hidden ones.

**Precedent.** CloudTrail's LookupEvents (StartTime, MaxResults, NextToken); `limit` and `continue` on a Kubernetes list; Kubernetes keeps Events and the audit log apart from reading an object.

**Who decides.** coordinator. Ruling 71 point 4: a small finding that adds a feature. It touches no count of S1.

**Verifier.** weakened, medium. Quote verbatim at records.yaml:110-112. `inspect.v1` takes only `object` and `expectedId` (spec/core/operations.yaml:316-323) and requires `records` (401-406). lifecycle.yaml:236-238 and 247-248 give every record that names the instance; records.yaml:171 "It is never presented as complete"; operations.yaml:875-876 rules out a cursor for `list.v1`. implementation-defined.yaml:79-88 bounds no result size. Each inspection is a request and so adds records (records.yaml:17-18).

**Verifier's corrections.** Class: gap, not needless complexity; the fix adds inputs. Partly a recorded layer choice: AUDIT.md:384 puts "cursors" in the runtime profile and DL8 leaves feeds and positions to each engine. New is that the one portable read cannot return less than everything. The `list.v1` cursor reverses a sentence of OPS-036 written for rulings 53 point 3 and 59 A4 ("no counts"; IN_FORCE.md:145). An opaque cursor over visible items discloses no count, but it changes ruled text: log it for the owner apart from the `inspect.v1` inputs. REC-006 needs a sentence so that a selected read is not a gap.


## FR-031

**A result that gives a reason outside IFC-026's form is read neither as that reason nor as unknown by any promise**

L03/F12 · ambiguity · verifier: weakened, low · decides: coordinator · new

**Promises.** OPS-005, OPS-007, OPS-025, IFC-009, IFC-026

**Where.** spec/conventions/operations.yaml:117

**Quote.** "a result from an implementation that carries Conduit's keys states neither an outcome nor a reason under IFC-026, or states both; such a result states an outcome with which its `isError` disagrees"

**Situation.** A package's implementation returns `reason: refused` when its own backend denies a call, after a first write went through. No filter was sent (a hook carries none in this version), so `restriction` names none, or `isError` is not true. IFC-026 allows a reason "only in those two cases" (spec/interface/implementation.yaml:759). Reading A: it is a reason under IFC-026 all the same. OPS-005 reports refused, the engine relies on "it performed nothing", OPS-025 frees the key, and the caller's repeat is dispatched as a new request: the write lands twice. Reading B: it is not a reason under IFC-026, so the result states neither, and OPS-007 gives unknown, as IFC-009 gives for a result that contradicts itself.

**Counterargument.** The implementation broke IFC-026, and an engine cannot check everything an implementation does. IFC-009 already covers a wrong value of either key. Reading B follows from the words "a reason under IFC-026" if they are read strictly.

**Least change.** Add a case to OPS-007's `when`: "a result gives a reason and its `isError` is not true, or its `restriction` is absent or is not an item of the filters that the call was sent".

**Precedent.** IFC-009: a result that contradicts itself is trusted in neither part.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity; the fix follows IFC-009's own rule and adds no behavior.

**Verifier.** weakened, low. Quote verbatim at operations.yaml:117-119. A promise the reviewer did not cite reads such a result: spec/interface/implementation.yaml:765-772 (IFC-027) "The result of an implementation holds `io.github.shrav-k.conduit/reason`, with the value refused or unavailable, and no `io.github.shrav-k.conduit/outcome`, as IFC-026 states. ... The request ends with that reason and has no outcome." IFC-009 (implementation.yaml:203-205) gives unknown only for both keys, neither key, or a wrong value.

**Verifier's corrections.** Largely decided: IFC-027 gives the reviewer's reading A, so "read neither as that reason nor as unknown by any promise" does not hold. What remains is whether "as IFC-026 states" requires `restriction` and `isError` true before IFC-027 applies. One sentence in IFC-027 or IFC-009 would settle it. The harm in the situation follows from the implementation breaking IFC-026 (implementation.yaml:759-760 "never after it has performed any part of what the call asks"), not from the engine's reading. The proposed OPS-007 case is sound hardening on IFC-009's principle, but it changes what IFC-027 now gives; coordinator, logged.


## FR-032

**The obligations of this slice sit in `observable` fields hundreds of words long, restated across promises and layers, and the restatements already differ**

L03/F13 · needless-complexity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-009.

**Promises.** OPS-005, OPS-006, OPS-007, OPS-009, OPS-014, OPS-024, OPS-034, REC-002, IFC-009, IFC-010

**Where.** spec/conventions/operations.yaml:99

**Quote.** "The result states its outcome, or a reason in place of one where IFC-026 has it give one."

**Situation.** The form of a promise gives `then` "the single obligation" and `observable` "what can be checked" (spec/README.md:291). In these 38 promises the observables hold about 7,100 words: 895 in OPS-014, 619 in REC-002, 465 in OPS-009. The events that fence an attempt, the test for equal contents, the keeping of outputs and the meaning of `subject` exist only there. A builder who reads `observable` as a test note has no rule for them. One who reads it as binding must reconcile the restatements: the rule for a filtered repeat is written out in OPS-009 twice, OPS-024, OPS-034, OPS-005, IFC-016, IFC-025, ACC-008 and three rows of definitions.md. They already differ. OPS-005 (spec/conventions/operations.yaml:81) leaves "or is implemented by a plain server" out of OPS-034's case. IFC-009 gives unknown for "a value other than those named", which OPS-007's `when` does not list. OPS-006, quoted here, has the same `when` and obligation as IFC-009 (spec/interface/implementation.yaml:194) in another layer. OPS-007's `when` is the wire (-32021, `isError`, `structuredContent`) in a convention. Every `vectors` field is empty while spec/README.md:245 counts coverage from the manifest.

**Counterargument.** The cross-statements were added so that each promise can be read alone, the checkers keep the IDs consistent, and none of the differences found here changes behavior by itself.

**Least change.** State each decision in one promise and cite its ID elsewhere. Give the fence promises of its own: one for the events that fence an attempt and one for those that do not. Move the test for equal contents and the keeping of outputs into a `then`. Remove OPS-006 in favour of IFC-009. Move OPS-007's wire cases to IFC-009 and IFC-010, leaving OPS-007 as: where the interface gives no outcome, the outcome is unknown.

**Precedent.** The practice of the IETF's RFCs: one normative statement, and references to it elsewhere.

**Who decides.** coordinator. Rulings 37 and 46 point 4: it changes no behavior. The README's rule 1, one obligation per promise, already asks for it.

**Verifier.** confirmed, medium. Recounted with PyYAML at this commit: 38 promises, 7,155 words of `observable`; OPS-014 895, REC-002 619, OPS-009 465; every `vectors` is empty. Quote verbatim at operations.yaml:99. OPS-006's `when` (operations.yaml:96-97) is IFC-009's (implementation.yaml:197-198). spec/README.md:291-292 gives `then` "The single obligation" and `observable` "What can be checked"; README.md:62 calls the layer "The few behaviors every part must agree on, stated as invariants". The drifts are as cited: OPS-005 (operations.yaml:80-83) omits "or is implemented by a plain server" of OPS-034 (820-822); IFC-009 (implementation.yaml:203-205) names a case that OPS-007's `when` (117-121) does not. The fence events, the test for equal contents and `subject` exist only in observables (operations.yaml:352-376, 215-225; records.yaml:68-70). full-review.md:116 asks this question; no record answers it.

**Verifier's corrections.** None of substance. Removing OPS-006 needs an entry in the file's register of removed IDs, where every earlier removal cites a ruling (operations.yaml:7-12): log it for the owner. Moving OPS-007's wire cases changes the per-layer counts in README's Contents. The empty `vectors` fields beside manifest coverage are by design (README.md:293 "See the dated coverage status above"), though rule 5 then cannot be read from a promise.


## Further points the verifier noticed

Not verified by a second reader.

- **A run's end fences an attempt though processes that the run started may still act** (spec/conventions/operations.yaml:361). Noticed while checking F3; a candidate unsafe finding. OPS-014 counts an attempt fenced once "the engine's own runner has seen its process end, or the runtime provider reports it `stopped`". No promise says that a run includes the processes its item starts, or that a stop or a `stopped` report covers them: spec/core/operations.yaml:1029-1031 speaks only of "no run that it started for the ID", spec/definitions.md:314 defines a run as "The running of an instance's item", and a search of spec/ finds no "child", "descendant" or "process group". A child that outlives the process can apply the attempt's effect after failed is recorded, and the repeat that OPS-024 dispatches applies it again, against OPS-018. docs/guides/full-review.md:171-172 records the observation ("Codex's terminal children outlived the run's process-group stop in the smoke test") as not yet a finding. Checked by reading the text only; nothing was run. Least change of the same family as F3: a run's end fences only where the provider's contract, or DL16 for the engine's own runner, states that it ends every process the run started. Narrowing order 077's reading is the coordinator's.

## The verifier's notes

Verified at 2eb8f258 (2026-11-12), read-only. All 13 quotes are verbatim at the cited lines, line wraps aside, and the cited line numbers are right. Result: 7 confirmed (F1, F2, F3, F5, F6, F7, F9, F13 is the eighth: 8 confirmed), 5 weakened (F4, F8, F10, F11, F12), none refuted outright, none wholly known. Unsafe marks: F3's stands in a narrow form; F4's and F8's are not sustained. Read in full: spec/conventions/operations.yaml and spec/conventions/records.yaml. Read in the cited parts only: revisions.yaml, access.yaml, lifecycle.yaml, implicit.yaml, providers.yaml, conduits.yaml, contracts.yaml, templates.yaml, interface/implementation.yaml, core/operations.yaml, core/results.yaml, core/README.md, definitions.md, implementation-defined.yaml, spec/README.md, IN_FORCE.md, AUDIT.md, cold-readers.md, full-review.md, G5.md, the increment 4 log, the review list, the ledger rows and records of rulings 62, 71 and 75, order 077 and order 082. Not checked: the Model Context Protocol's own text (F5, F6 rest on the spec's account of it); the reaudit rounds other than round 6's D10 and D19; the engine beyond two places in requests.rs and CONFORMANCE.md, used only as non-binding evidence that a reading was taken. Nothing was built or run except a word count with /usr/bin/python3.
