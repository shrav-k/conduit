# X07-retries-fence: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Commit 2eb8f258 (2026-11-12).

MC2-001's trace (experiments/012-model-checking/README.md:421-430) against the text now in spec/: actions 1 to 3 run as before. Action 4, Reconcile(0), reports conflict under OPS-030, because stopping waiting fences nothing (spec/conventions/operations.yaml:358-361). Action 5's repeat is answered unknown with no dispatch (OPS-012, plain). Action 6 applies one effect, and there is no second dispatch. So 077's text stops that trace at action 4.

It does not do so without new machinery. The engine must keep a fence mark for each attempt, track late answers, and order run reports against dispatches; OPS-008 and DL2 hold none of this (F8). The double effect also returns by five neighbouring routes that the text allows: a check made before the fence and recorded after it (F1, eight actions), an answer that a gate gave (F2), a process end with children left (F3), failed stated by a later attempt (F4), and an onward request that the engine itself dispatched (F7). MC2's Fenced control saw none of them: it makes the actor's read and record one atomic step, applies an effect atomically at delivery, and ignores late replies. Its rerun on 077's text should drop those three assumptions.

Fenced, by kind of implementation. The rules exist, listed by event inside one observable. Derived from them: `engine`, once the invocation has ended; the engine's own runner, an answer or the process's end; a run under a runtime provider, an answer or a `stopped` report after dispatch, and never while the machine is cut off; a `remote` server, plain or aware, an answer only, so never after a dropped stream or a cancellation (F9). The ruling's second branch, the implementation's confirmation, has no form of its own. Parts that are provider calls are undecided (F8).

The two limits are stated in OPS-014's observable and pointed to from OPS-018 and OPS-030, which an engine builder reads. They are not in `reconcile.v1`'s description, not required in OPS-030's message, and not in "Limits of this version" (F10).

After unknown, an Agent may repeat (dispatched only for a deduplicating, aware implementation), reconcile where it holds the Conduit, or make a new request. A new key is dispatched at once, beside whatever is still in flight, so the double effect returns outside OPS-018's letter. That is the owner's accepted limit and the tracker's OPS-011 point; the new case is a hook, which has no new request (F9).

OPS-008's scope and OPS-009 are consistent on the engine's side, which alone resolves repeats. They are not consistent with what CTR-002 tells an implementation to do with the key it is sent (F5).

Paradigms. The engine's keyed repeats follow payment APIs' idempotency keys: same key and contents, same result; other contents, conflict; in progress, conflict. There are three departures. The sender is in the scope, which onward requests need. Expired outputs give conflict, not a fresh run, which is safer than Stripe's 24 hours. A new attempt runs under the same key after failed, where Stripe and the IETF draft replay the stored failure; this is what makes a reconciled failed dangerous and so what the fence pays for, and hooks depend on it. Toward a plain server the engine is at-most-once RPC. The fence is not a fencing token: nothing travels with the dispatch and the resource rejects nothing. It is quiescence, as STONITH without the power to stop the other side. A real fencing token, the attempt number, is the missing piece for deduplicating implementations (F4).

Could an implementer of a Resource build correct retry behavior from these promises alone? No. A plain server has nothing to build. An aware one finds what its outcomes must mean (OPS-002 to OPS-004) and what a call carries (IFC-006), but not what to deduplicate by (F5), what to answer to a call it has seen (F6), how to tell an old attempt from a new one (F4), or how to key its own onward requests (F7). Order 143's worker had to invent the scope.

**What the reader read.** Read in full: spec/conventions/operations.yaml (all 32 promises, recounted). In spec/interface/implementation.yaml: IFC-006 to IFC-010, IFC-016, IFC-018 to IFC-021, IFC-023, IFC-025, IFC-026, IFC-028, IFC-046, IFC-047, IFC-050 and the head of IFC-033. In spec/core/operations.yaml: `reconcile.v1`, `inspect-run.v1` and the head comment on `deduplicates` (12 core operations declare it, recounted). Also CTR-002, IMP-007, LIF-028, TPL-017, ACC-019, REC-002, PRV-003, PRV-005, PRV-007, PRV-009, PRV-010, PRV-029, PKG-020; DL2, DL3, DL10, DL16, DL17; the rows for repeat, attempt, fenced, `deduplicates` and reconcile in spec/definitions.md.

Outside spec/: ruling 75's record (A1 and its later answers) and ruling 71 point 4; order 077 with its Record; order 143's Record; experiment 012's MC2 pre-registration, trace and MC2-001 (README lines 165-261 and 405-560), and G5's rows by search; the vector requests-reconcile-fenced; the READMEs of packages/runtime-provider-kit and packages/scenario-s4-effect where cited.

Checked for what is already known: spec/README.md "Limits of this version", docs/AUDIT.md "Known to be open", docs/guides/cold-readers.md, the brief's queued list, and IN_FORCE.md topic 7 (line 150 holds the fence and its two limits as in spec/).

Not done:
- Nothing was built or run, and no model was rerun.
- The engine's source was searched once, not read.
- Not reviewed: ACC-033 and ACC-055 (who may reconcile), the filtered-repeat logic of OPS-009, OPS-024 and OPS-034, REV-010, LIF-013 beyond the `unreconciled` schema, and the safety catalogue.
- Order 105 was read only through experiment 012; PRV-025 was read in part.
- The protocol's revision 2026-07-28 was not checked for what a gateway or a cancelled receiver may answer, so F2 and F9 rest on the specification's own statements of it.


## FR-217

**The fence orders the record after the fence, not the check; a late answer fences whatever it says**

X07/F1 · contradiction · unsafe · verifier: confirmed, high · decides: coordinator · new

**Promises.** OPS-014, OPS-013, OPS-018, OPS-024, OPS-030

**Where.** spec/conventions/operations.yaml:352

**Quote.** "An answer that ends an attempt's call at the implementation fences it: a result, an error or a JSON-RPC error, including one that arrives after the outcome became unknown, which changes no outcome under OPS-013."

**Situation.** Eight actions, one key, a plain effectful operation, no fabricated evidence. (1) A's request is dispatched and delayed. (2) The engine stops waiting: unknown, unfenced. (3) The actor reads what the effect changes: absent. (4) Its reconcile with occurred false reports conflict (OPS-030). (5) The dispatch lands, the effect is applied, and the answer, a complete result, reaches the engine late: the attempt is fenced and the outcome stays unknown. (6) The actor sends the same reconciliation again, as line 736 invites ('made again later is decided afresh'): failed is recorded. (7) A's repeat is dispatched under OPS-024. (8) It lands: two effects for one key, against OPS-018. The engine held an answer saying the call completed and still recorded failed. Nothing in spec/ says the check must follow the fence. MC2's Fenced control could not see this: it makes the actor's read and record one atomic step and ignores late replies (experiments/012-model-checking/README.md:193-194). The vector requests-reconcile-fenced is steps 4 to 7 with an error as the late answer; its title is 'late answer permits actor retry'.

**Counterargument.** A reconciliation is the actor's statement, and a stale one was always the actor's fault. Ruling 75 A1 asked only that no dispatch be in flight, which holds at action 6, and an actor that checks again after the fence sees the effect.

**Least change.** In OPS-030's observable and in reconcile.v1's description: 'What the actor checked must have been checked after the invocation became fenced, and the conflict's message says so.' In OPS-014: 'Where the answer that fenced an attempt stated succeeded, or was a plain server's complete result that is not an error, a request that records that the effect did not occur reports conflict under OPS-030, and its diagnostic says that the implementation answered that it occurred.' OPS-013 is kept: no outcome changes on its own.

**Precedent.** Fence, then inspect, then act, as Pacemaker fences a node (STONITH) before it promotes another. Stripe and the IETF Idempotency-Key draft treat the server's stored answer for a key as the authority.

**Who decides.** coordinator. The message and the description add no behavior (rulings 37; 46 point 4). The added conflict is a small refusal that closes an unsafe contradiction (ruling 71 point 4), logged for the owner to strike.

**Verifier.** confirmed, high. Quote verbatim at spec/conventions/operations.yaml:352-355. Each step is allowed: :354-355 a late answer 'changes no outcome under OPS-013', whatever it states; :313-315 'Recording that it did not occur makes the outcome failed only once the invocation is fenced'; :736-737 'A request made again later is decided afresh under OPS-025'; :520-521 'The repeat is dispatched as a new attempt' (a must). Nothing in spec/ orders the check after the fence or looks at what the fencing answer said: `checked` is a free string (spec/core/operations.yaml:533-535), and the only other fence text is lifecycle.yaml:600, revisions.yaml:136 and :150, definitions.md:215-216 and :223, core/README.md:205 and :212, and reconcile.v1 (core/operations.yaml:506, :513, :544). The actor cannot order it either: :377-378 'Becoming fenced records no outcome, wakes nothing and reconciles nothing'; `unreconciled` holds IDs only (spec/core/operations.yaml:381-384); a late answer leaves no record (DL2, spec/implementation-defined.yaml:21-22; the vector expects "records": [] at conformance/conventions/requests/reconcile-fenced/case.yaml:1708). Vector title at case.yaml:36; its late answer is an isError result at :1691-1696. MC2's atomic read-and-record and ignored late replies at experiments/012-model-checking/README.md:193-194. Not recorded in docs/AUDIT.md:174-245, docs/guides/cold-readers.md:39-80, spec/README.md:331 onward (no word on the fence), IN_FORCE.md:150, docs/guides/full-review.md:126-142, G5.md:262-278 and :353, the increment logs (4-12 at log-increment-4.md:23 is turn deadlines, closed), the review list, ruling 75's record, or 077's Record (docs/work-orders/077-spec-reconcile-fenced.md:585-599).

**Verifier's corrections.** Class stands: a procedure against OPS-018, the form MC2-001 had. It rests on an actor's stale statement, so 'gap' would also be fair. Unsafe stands and is wider than stated: no result tells an actor that or when an invocation became fenced, so even an actor that checks just before each request can be overtaken (check; the answer arrives and fences; record). Least change (a) therefore cannot be followed or checked as written. It needs the fence made visible, for example the time it came in OPS-030's diagnostic or beside the ID in `unreconciled`: a small feature, ruling 71 point 4. Part (b) is the operative fix where the answer stated succeeded. It adds a condition to what ruling 75 A1 lets an actor record, so log it to strike and put it on the review list. Coordinator decides, as stated.


## FR-218

**An answer fences even where the implementation did not give it**

X07/F2 · gap · unsafe · verifier: confirmed, medium · decides: coordinator · new

**Promises.** OPS-014, PRV-025, PRV-012, IFC-010

**Where.** spec/conventions/operations.yaml:315

**Quote.** "An attempt is fenced once no copy of its dispatch remains in flight, or the implementation confirms that the attempt can no longer act."

**Situation.** PRV-025 puts 'something between the network and the server' before every run of a runtime provider and every aware remote server (spec/promises/providers.yaml:810): a proxy, a mesh, a stdio bridge (line 861). Line 352 fences on 'a result, an error or a JSON-RPC error', and no promise says who wrote it. A bridge or gateway times out upstream and answers with a JSON-RPC error while the server still holds the call. The error gives unknown (IFC-010, OPS-007), the attempt is fenced, the actor reads no effect and records failed, the repeat is dispatched under OPS-024, and both land. Whether a bare HTTP 502 or 504 is 'an error' here is also undecided: IFC-010 names only tool errors and JSON-RPC errors. The shipped kit is safe by its own choice, which it calls no addition to the language: 'A refusal carries no JSON-RPC body, since the engine bridge would forward one' (packages/runtime-provider-kit/README.md:167).

**Counterargument.** The text says an answer 'at the implementation', so a gate's answer is not one. The kit's gate sends none, and an operator's proxy before a plain remote server is outside any contract.

**Least change.** OPS-014: 'A response that carries no JSON-RPC message, such as an HTTP status alone, is no answer and fences nothing.' PRV-025: 'A gate answers no call in the server's place with a JSON-RPC message; where it refuses or fails a request, it answers with an HTTP status and no JSON-RPC body.' Name an operator's proxy before a plain `remote` server among the fence's stated limits.

**Precedent.** The runtime-provider kit's bridge. RFC 9110 reserves 502 and 504 for gateways, and RFC 9209's Proxy-Status marks a response that a proxy generated.

**Who decides.** coordinator. It closes an unsafe gap by narrowing what fences. The one new obligation on a provider is small and the kit already meets it (rulings 37; 46 point 4; 71 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:315-317. 'at the implementation' (:352-353) is a condition the engine cannot test: it sees a JSON-RPC message, not its author. PRV-025: 'The gate is the server itself, something between the network and the server or, for a run, the runtime provider's own end of a channel' (spec/promises/providers.yaml:810-811); the gates it names as conforming include 'a bridge on the run's machine that checks the proof and writes to the server's standard input' (:838-839). No sentence of PRV-025 (:798-868) or PRV-012 (:369-387) forbids a gate from answering a call itself, and a search of spec/ finds none. Kit quote verbatim at packages/runtime-provider-kit/README.md:167. Partly decided by text: 'is an error when its `isError` is true. A protocol error is a JSON-RPC error' (spec/interface/implementation.yaml:232-233), and the list of fencing events is closed by 'An attempt that none of these events fences ... is never fenced' (operations.yaml:373-375), so an HTTP status with no JSON-RPC message fences nothing on a careful reading. Not found in any known-open record.

**Verifier's corrections.** Narrow it in three ways. (1) The bare 502 or 504 point is decided conservatively by the closed list; the proposed OPS-014 sentence is a clarification, worth writing. (2) 'before every run' overstates: the gate may be the server itself. (3) For a plain `remote` server, an answering proxy is the implementation as Conduit sees it, and its upstream call is the stated limit (operations.yaml:348-352). The new case is a runtime provider's gate or bridge, and an aware remote's gate, which PRV-025 binds (spec/README.md:322). For that case class gap and unsafe stand: gate times out and answers a JSON-RPC error, attempt fenced, actor truthfully reads no effect and records failed, OPS-024 dispatches, both land. The operative fix is the PRV-025 sentence, a new obligation on providers: coordinator under ruling 71 point 4, logged to strike. Medium because no shipped gate answers in the server's place.


## FR-219

**The run's end as a fence: processes that outlive the run's process, and a report 'made after dispatch'**

X07/F3 · ambiguity · unsafe · verifier: confirmed, medium · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-231.

**Promises.** OPS-014, PRV-009, PRV-010, PRV-003

**Where.** spec/conventions/operations.yaml:361

**Quote.** "An attempt sent to a run is fenced once the engine's own runner has seen its process end, or the runtime provider reports it `stopped` under PRV-009 in a report made after dispatch."

**Situation.** Two cases. (a) The runner 'has seen its process end' while processes that it started still run. The brief records it of Codex: 'Codex's terminal children outlived the run's process-group stop' (docs/guides/full-review.md:171). A turn ends unknown, the process ends, the attempt is fenced, the actor records failed, and the repeat starts a second turn while the first's children still write in the same directory. The stated limit names only 'its own request on its way to another service'. (b) 'A report made after dispatch' reads two ways: asked after, or received after. A provider asked before the dispatch, while it restarts the run under PRV-010, answers `stopped` after the dispatch reached the restarted run. Under 'received after', the attempt is fenced while the run works on it.

**Counterargument.** inspect-run.v1 reports stopped only where no run it started is running, a careful runner ends the whole group, and the window in (b) is narrow. Children are the implementation's own business, as its onward calls are.

**Least change.** Line 361: 'once the engine's own runner has seen every process of the run end, or the runtime provider, asked after the attempt was dispatched, reports it `stopped` under PRV-009'. DL16 gains one item: how the runner tells that no process of a run remains.

**Precedent.** systemd's KillMode=control-group and cgroup.kill, and Windows job objects: a unit has ended when its control group is empty.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (rulings 37; 46 point 4); the DL16 item is a small addition (ruling 71 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:361-363. (a) spec/ says nothing of processes that a run starts: no hit for child, descendant, process group or cgroup in spec/promises/providers.yaml, spec/conventions, spec/interface, spec/implementation-defined.yaml or spec/definitions.md. DL16 (implementation-defined.yaml:169-190) covers the working directory, the user, the restart policy and what happens when the engine stops, not how the runner tells that a run has no process left. The stated limit names only 'its own request on its way to another service' (operations.yaml:349-350). The friction is recorded, not as a finding and not against the fence, at docs/guides/full-review.md:171-172 and docs/implementation/HANDOVER.md:149. (b) 'in a report made after dispatch' (:363): PRV-009 gives only 'the time of that report in `lastHeartbeatTime`' (providers.yaml:312) and definitions.md:372 'the time of the last successful observation'; neither says asked or received. PRV-010 lets the provider restart the run meanwhile (providers.yaml:332-333).

**Verifier's corrections.** Class ambiguity and known-new-situation stand. Unsafe stands, narrowly: (a) needs an implementation whose effect is applied by a process that outlives the one the runner watches; (b) needs a `stopped` answer asked before a dispatch and read after it. For the fix, 'every process of the run' should follow the operating system's own unit (process group, cgroup, job object), with DL16 saying which, so it stays a documented choice and adds no engine machinery. Coordinator decides, as stated.


## FR-220

**Failed stated by a later attempt is recorded while an earlier attempt is still in flight**

X07/F4 · contradiction · unsafe · verifier: confirmed, medium · decides: owner · new

Same fault, or overlapping: FR-181.

**Promises.** OPS-012, OPS-024, OPS-003, OPS-014, OPS-018

**Where.** spec/conventions/operations.yaml:524

**Quote.** "A failed outcome recorded by a reconciliation was recorded only once every attempt was fenced, under OPS-014, so the new attempt cannot meet an earlier dispatch still on its way. The implementation receives a further call. The invocation's state is then in progress, and afterwards the outcome of the new attempt."

**Situation.** A deduplicating, aware operation. (1) Attempt 1 is dispatched and delayed. (2) The engine stops waiting: unknown. (3) The caller's repeat is dispatched as attempt 2 under OPS-012 (line 281). (4) Attempt 2 arrives first and truthfully answers failed, its store being down: the invocation's outcome is failed, and no fence is asked, since no reconciliation recorded it. (5) Attempt 1 lands and applies the effect once, as CTR-002 allows; its late answer changes nothing. The engine now reports failed for an invocation whose effect occurred, against OPS-003's observable, and a caller that trusts failed and asks again under a new key applies it twice. The implementation cannot prevent it: IFC-006 sends no attempt number, so it cannot tell late attempt 1 from the new attempt that OPS-024 sends after failed, which it must carry out (OPS-028 relies on that for hooks).

**Counterargument.** Per key the effect is applied once, so OPS-018 holds as written. A repeat under the same key is answered succeeded by the implementation's deduplication, and the wrong failed heals.

**Least change.** In OPS-012 and OPS-024: 'Where a new attempt's result states failed while an earlier attempt of the invocation is not fenced under OPS-014, the invocation's outcome stays unknown.' It uses the fence marks the engine already keeps and adds no key.

**Precedent.** OPS-014's own 'every attempt' rule. The alternative is a fencing token (Kleppmann; Chubby's sequencer): send the attempt number under IFC-006 and have a deduplicating implementation reject an older one.

**Who decides.** owner. It changes which outcome is reported and chooses between two designs. Ruling 75 A1 named only an actor's record.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:524-527; it speaks only of a failed 'recorded by a reconciliation'. OPS-012 dispatches a new attempt for an unknown invocation and asks no fence (:281-282). 'This applies also to a deduplicating implementation' (:317-318) is about a reconciliation. IFC-006's keys hold no attempt number (spec/interface/implementation.yaml:128-136). OPS-003 at :45-48. Order 077 named only the reconciliation case: 'A deduplicating new attempt under OPS-012 can be dispatched while an earlier attempt is still on its way' (docs/work-orders/077-spec-reconcile-fenced.md:166-168). MC2's deduplicating variant models no failed answer (experiments/012-model-checking/README.md:225-231, :247-252). One premise is read in, not stated: that an OPS-012 attempt's answer becomes the invocation's outcome is said for OPS-024's attempts (:526-528) and reaches OPS-012 only through 'The same holds for a new attempt that OPS-012 dispatches' (:546-547), which follows a sentence about the Conduit; OPS-013's `when` (:297) and OPS-030 (:742-743) support it. No known-open record has the case.

**Verifier's corrections.** The example is wrong. With its store down, a deduplicating implementation cannot tell whether an earlier attempt applied the effect, so OPS-004 (operations.yaml:55-58) has it answer unknown, not failed. Use a truthful failed for a cause that passes: a precondition not met when attempt 2 arrives and met when attempt 1 lands. With that the path holds. Unsafe stands as a wrong reported outcome (failed for an invocation whose effect occurred), not as a second effect under one key; the assessment's 'double effect returns by five routes' should not count this one. Contradiction against OPS-003's observable is defensible. Owner decides, as stated. Medium: it needs a deduplicating, aware implementation (today only packages/scenario-s4-effect/contracts.yaml:9) and two attempts that cross.


## FR-221

**CTR-002 has an implementation deduplicate by the key alone; OPS-008's scope cannot be rebuilt from IFC-006**

X07/F5 · defect · unsafe · verifier: confirmed, high · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-253.

**Promises.** CTR-002, OPS-008, IFC-006, OPS-012, OPS-028

**Where.** spec/promises/contracts.yaml:43

**Quote.** "`deduplicates`, the mark of an operation whose implementation recognizes an idempotency key that it has already been sent and applies the effect at most once for it"

**Situation.** OPS-008 makes one key from another caller, another sender or to another target 'another invocation', and gives the example of an Agent's own request and a Resource's onward request for that Agent with one key and one target (spec/conventions/operations.yaml:164-169). Both reach the target's implementation with the same `idempotencyKey` and the same `caller`. IFC-006 sends no sender (spec/interface/implementation.yaml:128-136), and the Conduit is no part of the scope (OPS-024). An implementation that follows CTR-002's words, or keys on caller and key, treats the second invocation as done: its effect is lost and it is answered succeeded with the first's outputs. With two callers and one key string, one caller receives the other's outputs. Order 143's worker met this and used the invocation's ID as a stand-in (packages/scenario-s4-effect/README.md:46-48); the text does not say so. The definitions row repeats CTR-002's words (spec/definitions.md:217).

**Counterargument.** The engine alone resolves repeats, and the invocation's ID, the same for every attempt, is already sent. A careful builder finds it, as order 143's did.

**Least change.** CTR-002 and the `deduplicates` row of definitions.md: 'whose implementation recognizes an invocation that it has already been sent, by `io.github.shrav-k.conduit/invocation`, which every attempt of it carries, and applies the effect at most once for it'. In IFC-006's observable: 'An idempotency key is unique only within OPS-008's scope; an implementation tells one invocation from another by the invocation's ID.'

**Precedent.** Stripe scopes a key to the account, and the IETF Idempotency-Key draft has the server define the key's scope. Here the engine has already resolved the scope into one ID.

**Who decides.** coordinator. A defect whose fix adds no behavior: the engine already sends the ID (rulings 37; 46 point 4).

**Verifier.** confirmed, high. Quote verbatim at spec/promises/contracts.yaml:43-44, repeated at spec/definitions.md:217. OPS-008: 'a request an Agent sends itself and an onward request that a Resource sends for that Agent, with one idempotency key and one target, are two invocations' (spec/conventions/operations.yaml:167-169). IFC-006's keys hold no sender (spec/interface/implementation.yaml:128-136); the invocation is sent 'by the ID the engine gives every invocation' (:140-141). No text in spec/ says what an implementation deduplicates by; OPS-028 says again that it is the key: 'An implementation therefore recognizes the new attempt of a hook that failed' (operations.yaml:664-665). The missing sender is recorded only in order 143 (docs/work-orders/143-s4-srv1-effect.md:155-159, :311-312, :330-331) and packages/scenario-s4-effect/README.md:46-48, as a package's own choice; neither names CTR-002's wording. Not in docs/AUDIT.md known-open, cold-readers.md, IN_FORCE.md or the review list.

**Verifier's corrections.** None of substance. Defect, unsafe (a second invocation's effect lost and answered succeeded; with a key shared by two callers, one receives the other's outputs), coordinator and known-new-situation all stand. The fix should also change OPS-028's observable (operations.yaml:664-666), and can note that order 143's package already deduplicates by the invocation's ID.


## FR-222

**No promise says what a deduplicating implementation answers to a call it has already been sent**

X07/F6 · gap · verifier: confirmed, medium · decides: owner · new

**Promises.** CTR-002, OPS-012, OPS-018, OPS-002, OPS-003, OPS-007

**Where.** spec/conventions/operations.yaml:286

**Quote.** "A new attempt carries the same idempotency key as the first and, like any dispatch, needs the request to be permitted under ACC-001."

**Situation.** An implementer of a Resource whose operation declares `deduplicates: true` receives attempt 2 of an invocation under OPS-012. Three cases are reachable and undecided. The earlier call is still running: wait, answer unknown, or answer failed? Its effect was applied: succeeded needs the declared outputs again, or OPS-007 makes the result unknown, so outputs must be stored, and for how long, when unknown has no end under OPS-013? It failed: carry the call out again, as OPS-024 and OPS-028 expect, or replay the failure, as Stripe does? The whole contract is CTR-002's clause and OPS-018's 'must not'. The Implementation level's list of promises that bind an implementation (spec/README.md:322) names none for this. Only the 12 core operations that the engine implements declare the mark today, so the first package author to declare it writes the rule.

**Counterargument.** Deduplication is the implementation's own design, and OPS-002 to OPS-004 already bind what each answer must mean.

**Least change.** One interface promise: 'An implementation of an operation whose contract declares `deduplicates: true` receives a call for an invocation it has already been sent. Where that invocation's effect was applied, it applies nothing and answers succeeded with the outputs of that effect. Where an earlier call for it has not ended, it answers once that call ends, or answers unknown. Where every earlier call failed, it carries the call out.' Name it at the Implementation level.

**Precedent.** The IETF Idempotency-Key draft and Stripe: replay the stored response of a completed key, and answer 409 for one in progress. Conduit departs only in carrying out again after failed, which OPS-024 needs.

**Who decides.** owner. It sets a contract that every package author must meet, and chooses between replaying a failure and carrying the call out again. The coordinator may draft it.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:286-288. The whole contract is CTR-002's clause (spec/promises/contracts.yaml:43-44) and OPS-018 (:384-397); spec/README.md:322 lists examples and none for this. Two cases follow from text already there: once the effect was applied, OPS-002 (:28-36) with OPS-007's 'a declared output fails validation in a result that would otherwise have the outcome succeeded' (:119-120) means succeeded with valid outputs; after failed, OPS-024 dispatches the attempt to be carried out (:520-521; OPS-021 :453-454). Undecided: what to answer while an earlier call still runs, and how long the implementation keeps what a repeat needs, while unknown has no end (OPS-013, :297-302). No known-open record has it.

**Verifier's corrections.** One fact is wrong. The mark is declared outside the core package: at packages/scenario-s4-effect/contracts.yaml:9, whose author wrote its own rule (packages/scenario-s4-effect/README.md:46-51), and in the specification's own example package (spec/schemas/examples/valid/files-package/contracts.yaml:53 and :177). The 12 core declarations are right (spec/core/operations.yaml). So a package author has already had to write the rule, which strengthens the finding. Gap and not unsafe stand. Who decides: the two derivable cases are the coordinator's to write as restatement; the answer while an earlier call runs and the retention period are new obligations, the owner's, or the coordinator's under ruling 71 point 4 if kept small.


## FR-223

**The fence ignores an onward request that the engine itself dispatched for the invocation**

X07/F7 · gap · unsafe · verifier: weakened, medium · decides: owner · known-new-situation

**Promises.** OPS-014, OPS-018, IFC-020, ACC-019, IFC-028

**Where.** spec/conventions/operations.yaml:348

**Quote.** "An implementation that has answered or whose process has ended may still have its own request on its way to another service, whose effect can land after failed is recorded and be applied again by the repeat that OPS-024 dispatches; the fence covers Conduit's own dispatch, and each hop handles its own idempotency."

**Situation.** Resource R serves invocation I and sends an onward request J to target T through the engine, naming I in `serving` (IFC-020). R answers unknown after its own wait, or its process ends: I's attempt is fenced. J is still in progress at T. The engine dispatched J and records it as serving I (REC-002), and ACC-019 stops only requests made after I ended. The actor reads T and finds no effect, records I failed, and the repeat of I is dispatched (OPS-024). R sends T a new onward request without the earlier key, and both land. The limit's own words do not hold here: J is Conduit's own dispatch, and the engine can see it.

**Counterargument.** The owner accepted that each hop handles its own idempotency. An R that derives its onward key from I's ID makes the second onward request a repeat under OPS-008, which reports conflict while J runs.

**Least change.** OPS-014: 'An invocation is fenced only when every attempt is fenced, and every invocation that names it in `serving` has ended and is fenced', as line 371 says of a setup's parts. Or, text only, in IFC-020: 'An instance that serves a keyed invocation gives each onward request an idempotency key derived from that invocation's ID, so that a new attempt's onward request is a repeat.'

**Precedent.** OPS-014's own rule for parts; structured concurrency, where a parent has not ended while its children run.

**Who decides.** owner. The first form narrows a limit the owner accepted on 2026-11-12 (ruling 75's later answers). The IFC-020 sentence alone adds no behavior and is the coordinator's.

**Verifier.** weakened, medium. Quote verbatim at spec/conventions/operations.yaml:348-352. The text decides that I is fenced: 'An invocation is fenced only when every attempt is fenced' (:317); J is another invocation (IFC-020, spec/interface/implementation.yaml:558-566), neither an attempt of I nor a part, and the parts rule names only 'a setup, retirement, open or close' (:371-372). So the case is the stated limit, 'each hop handles its own idempotency' (:351-352), which the owner accepted: 'not a server's onward call to another service: each hop handles its own idempotency' (docs/proposals/ruling-75/README.md:121-122); IN_FORCE.md:150; the brief calls it 'a server's onward call' (docs/guides/full-review.md:122). ACC-019 stops only later requests (spec/conventions/access.yaml:355-357). What is new: IFC-020 states only the key's scope (implementation.yaml:564-566); no text tells a serving instance to key its onward requests.

**Verifier's corrections.** A known limit with one new situation, not a new unsafe gap. 'The limit's own words do not hold here' is wrong: 'Conduit's own dispatch' is the dispatch of the invocation being reconciled, and J is the next hop. The harm is the one accepted on 2026-11-12, so drop the unsafe mark as new. What survives: (1) the sentence at :348-352 pictures only an outside service and should say that an onward request through the engine is such a hop; (2) IFC-020 should say how a hop handles it. Written as a statement of what follows under OPS-008 and OPS-032 (a key derived from the served invocation's ID makes the new attempt's onward request a repeat), it adds no behavior: coordinator. Written as a must on implementations, or the first form (I stays unfenced while J runs), it changes the accepted limit: owner.


## FR-224

**The engine's duties for the fence sit in one observable and conflict with DL2 and OPS-008**

X07/F8 · contradiction · verifier: weakened, medium · decides: coordinator · new

**Promises.** OPS-014, OPS-008, DL2, LIF-028, OPS-022, PRV-005

**Where.** spec/implementation-defined.yaml:21

**Quote.** "Where the record is written before an implementation is called; how attempts are numbered; what is done with a result that arrives late; and how the digest of an invocation's contents is computed, which the engine keeps under OPS-008."

**Situation.** OPS-014's observable makes a late answer fence its attempt (spec/conventions/operations.yaml:352), and the vector requests-reconcile-fenced needs it. DL2 still leaves a late result to each engine, so one that discards it, as the shell did before 077, keeps DL2 and can never record failed after a timeout. OPS-008's list of what the engine keeps 'for every invocation' (lines 155-160) holds no attempts and no fence marks, so whether a fence survives a restart is undecided: engine A keeps it and accepts failed afterwards, engine B loses it and reports conflict for good. 'Every part it dispatched is fenced' (line 371) covers parts that are provider calls, `run.v1` and `stop.v1`, which begin no invocation and have no attempt (PRV-005): nothing says what fences them. About fourteen fence rules are sentences of one observable, against 'One obligation per promise' (spec/README.md:297).

**Counterargument.** Read conservatively, each case leaves the attempt unfenced, which is safe, and OPS-014's `then` does state the rule.

**Least change.** DL2: 'what is done with a result that arrives late, beyond fencing its attempt under OPS-014'. OPS-008's list: 'and, for each attempt, whether it is fenced'. Move the fence's events into a promise of their own, with one row for each kind of implementation (`engine`, the engine's own runner, a runtime provider, `remote`) and one for a part that is a provider call: fenced once the call is answered and the provider, asked afterwards, reports the run `stopped` or `running`.

**Precedent.** The specification's own form of a promise.

**Who decides.** coordinator. A contradiction and an ambiguity whose fix adds no behavior (rulings 37; 46 point 4).

**Verifier.** weakened, medium. DL2 quote verbatim at spec/implementation-defined.yaml:21-23. The file's head decides the first point: '# A choice must keep every promise.' (:3). An engine that discards a late answer and then answers 'not yet fenced' fails OPS-014's observable (spec/conventions/operations.yaml:352-355) and OPS-030's `when` (:728-729), so DL2 is stale, not in conflict. OPS-008: 'what repeats and reconciliations need, for every invocation:' and then a list with no attempt and no fence state (:155-160); the general clause covers it, the list does not. Provider calls: a dispatch includes a provider call (spec/definitions.md:209), a provider call begins no invocation (definitions.md:210; spec/promises/providers.yaml:134), an attempt is 'one dispatch of an invocation' (definitions.md:215), and :370-372 gives a fence only to a part 'whose implementation is `engine`'. The form rule is at spec/README.md:297. The first build's own row reads the same way (engine/CONFORMANCE.md:19). Not in any known-open record.

**Verifier's corrections.** Not a contradiction. It is three smaller points and one of form: DL2 lacks the pointer to OPS-014 (defect, low); OPS-008's list leaves open whether fence state survives a restart (ambiguity, in the safe direction, though one engine then answers conflict for good); a part that is a provider call has no stated fence (gap, safe either way, since PRV-007 asks for a run only where it is reported stopped); and the fourteen-odd rules in one observable are needless complexity. Not unsafe, as stated. Coordinator decides the first two and the move to a promise of its own. The proposed row for a provider call is a new rule, not a restatement: ruling 71 point 4, logged to strike.


## FR-225

**Never fenced is the usual case for a remote server, and a hook has no 'new request'**

X07/F9 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-195.

**Promises.** OPS-014, OPS-030, OPS-021, OPS-022, OPS-027, OPS-028

**Where.** spec/conventions/operations.yaml:373

**Quote.** "An attempt that none of these events fences, such as one sent to a plain server reached by `remote` that never answered, is never fenced: its invocation can be recorded as having occurred, or stays unknown under OPS-013, and the Agent's way on is a new request."

**Situation.** The example understates the class. A server reached by `remote` has no run (line 366), so only an answer fences it. A dropped Streamable HTTP connection closes the stream (line 616), and the protocol has a cancelled receiver send no response (line 614): after either, no answer can come, plain or aware. After a cancellation, an attempt sent to a run is fenced only when the run stops, which no request but a retirement asks for. For a request, the way on is a new request. A setup hook has none: its key is fixed for the instance (OPS-028) and no actor sends it. After one dropped connection during the setup hook of an aware remote Resource, 'did not occur' is refused for good, every further setup ends unknown (line 343), and the only moves are to record that the hook occurred, true or not, or to retire the instance and register a replacement. Neither is stated.

**Counterargument.** The owner accepted that such a call stays unknown. Staying unknown is safe, and retiring and replacing an instance exists.

**Least change.** State the class and the way on: 'After a dropped connection or a cancellation, an attempt sent to a server reached by `remote` is never fenced. For a hook, the way on is to retire the instance and register a replacement.' For the owner, as an option: declare `deduplicates: true` on `setup-hook.v1` and `retire-hook.v1`, so that OPS-012 sends an aware implementation a new attempt that it recognizes.

**Precedent.** Kubernetes runs a lifecycle hook at least once and asks handlers to be idempotent; Terraform replaces a resource whose provisioner did not finish.

**Who decides.** coordinator. Stating the consequence adds no behavior (rulings 37; 46 point 4). Making the hooks deduplicate is the owner's.

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:373-376. 'A server reached by `remote` has no run' (:366); a stopped receiver sends no response and a dropped connection 'closes its stream, and so cancels the invocation' (:613-618); 'a dropped connection or cancellation with no confirmation under OPS-027 ... fence nothing' (:359-360). Hook: the key is 'scoped to the instance alone' (:650-652); an unknown hook gets no further call (:450-452); 'and so does every further setup while a setup hook is unknown' (:343-344). A hook may be served by a `remote` item (spec/promises/templates.yaml:279-280). The same arises on a run: a setup's cancellation is passed to its hook (:624-627), a server that honors it sends no response, and a stop is asked only in a retirement (spec/promises/providers.yaml:278-284). The accepted limit has only the request case: 'the Agent's way on is a new request' (docs/proposals/ruling-75/README.md:123-124). No known-open record has the hook case.

**Verifier's corrections.** Stands as known-new-situation, gap, not unsafe. Two precisions. 'Refused for good' should read 'reports conflict under OPS-030 for good'. And the hook case is wider than `remote`: a setup cancelled while its hook runs on a local run leaves the hook never fenced until the run's process ends, which only a retirement asks for. The stuck state pushes an actor to record 'occurred' without evidence, so the way on should be said. Coordinator for the statement; owner for making the hooks deduplicate, as stated.


## FR-226

**The limits and the way on are not said where an actor or an Agent meets them**

X07/F10 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** OPS-030, OPS-014, IFC-050, IFC-033, LIF-013

**Where.** spec/conventions/operations.yaml:734

**Quote.** "For an invocation that is not yet fenced its message says so, explains what would fence its attempts under OPS-014, and says that recording that the effect occurred needs no fence."

**Situation.** An actor meets the fence in two places: the description of `reconcile.v1`, which IFC-033 presents as the tool's description (spec/core/operations.yaml:494-524), and OPS-030's conflict. Neither states the first limit: after a fence, a server's own onward call may still land, so the check should cover what the server calls. The message need not say that nothing can fence an attempt where the engine knows it, as for a `remote` attempt whose stream is closed, nor name the way on: the sentence at line 737 reads as commentary. The built message tells the actor to wait for 'the implementation's confirmation that it can no longer act', which line 368 says has no form. `unreconciled` lists IDs alone (spec/core/operations.yaml:381-384), so an actor learns whether an invocation is fenced only by trying. 'Limits of this version' (spec/README.md:331) holds neither limit.

**Counterargument.** IFC-050 already has every result say what happened and why in full, and the wording is the engine's choice.

**Least change.** OPS-030: 'Where no event can fence an attempt, the message says so, and says that the way on is a new request.' Give `reconcile.v1`'s description the two limits, one sentence each. Add a section on retries to 'Limits of this version' that cites OPS-014 and OPS-018. In OPS-014's `then` and the definitions row, name the one form the implementation's confirmation has, or drop the clause.

**Precedent.** IFC-050 and ACC-056's explicit refusals (ruling 52).

**Who decides.** coordinator. Ruling 52's rule applied to text that exists; no behavior is added (rulings 37; 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/operations.yaml:734-736. The next sentences state the consequence and ask nothing of the message (:737-739). reconcile.v1's description (spec/core/operations.yaml:494-524) names the fence at :506 and :513 and neither limit; IFC-033 puts the contract's `metadata.description` in the tool's description (spec/interface/implementation.yaml:910-911). `unreconciled` holds IDs only (spec/core/operations.yaml:381-384). spec/README.md has no word on the fence from :331 on. 'no other form of confirmation that the attempt can no longer act is defined here' (operations.yaml:368-369), and the one form named makes the outcome failed under OPS-027 (:608-612), so the second branch of the `then` (:316-317) never applies to an unknown invocation; the vector's message still offers it (conformance/conventions/requests/reconcile-fenced/case.yaml:1640). IFC-050 asks for 'what happened and why in full' (implementation.yaml:1339), not the way on. Not in any known-open record.

**Verifier's corrections.** Stands. It is a finding under ruling 52 about text that exists, not a behavior gap; not unsafe; coordinator. Add what F1 needs: the message and the description should tell the actor to check again once the invocation is fenced. IFC-050 does not require a result to name the way on, so that sentence is a small addition under ruling 71 point 4, not a pure restatement.


## Further points the verifier noticed

Not verified by a second reader.

- **A run's end fences an attempt, yet the provider may restart the run, and a server may finish after its restart a call it had taken** (spec/conventions/operations.yaml:361). Lines 361-363 fence on the process's end or a `stopped` report. PRV-010 lets the provider restart the run 'with the values it was given for that run' (spec/promises/providers.yaml:332-333), and OPS-018's `when` counts an implementation that 'stops and restarts' (operations.yaml:389-390). No promise forbids a server that journals the calls it accepts from completing one after its restart. The effect then lands after failed is recorded, and the repeat that OPS-024 dispatches applies it again. The stated limit (:348-352) names only a request on its way to another service. Gap, unsafe on the same footing as F3. Stating it as part of the limit is the coordinator's; asking an implementation to finish no call it received before a restart is the owner's.
- **An onward invocation can outlast the invocation it serves, and no promise decides it** (spec/conventions/access.yaml:364). ACC-019's observable says 'In this version no operation arranges, through the engine, work that outlasts its invocation', but its `then` (:355-357) stops only requests made after the served invocation ended. An onward request dispatched just before the serving instance answers or dies stays in progress: nothing cancels it. It is the dispatch that lands late in F7. Gap, low to medium. Stating it is the coordinator's; cancelling it would be new behavior, the owner's.

## The verifier's notes

Commit 2eb8f258 (2026-11-12), read only. Eight findings confirmed (F1, F2, F3, F4, F5, F6, F9, F10), two weakened (F7, F8), none refuted and none wholly known. Every quote is verbatim at the cited line.

The two that matter most are F1 and F5. F1 is stronger than the reviewer stated: an actor cannot learn that or when an invocation became fenced, so no actor procedure can place its check after the fence, and the proposed sentence alone does not close it. F5 is as stated; order 143's author and worker already worked around it in a package.

Corrections to the reviewer's assessment:
- "The double effect returns by five neighbouring routes" overcounts. F4 is a wrong failed outcome, not a second effect under one key, and its example needs changing (a store that is down gives unknown under OPS-004). F7 is the limit the owner accepted on 2026-11-12, applied to an onward request.
- "Only the 12 core operations declare the mark" is wrong outside spec/core: packages/scenario-s4-effect/contracts.yaml:9 and the specification's example package declare it.
- F2's bare HTTP status point is decided conservatively by the closed list of fencing events; the gate-authored JSON-RPC answer is what survives.
- F8 is not a contradiction: the head of spec/implementation-defined.yaml (line 3) makes every choice keep every promise.

Severity: I gave high only to F1 and F5. F2, F3 and F4 keep their unsafe marks but are medium, because each needs a component to behave in a way nothing shipped does, or two attempts to cross.

Not checked: nothing was built or run, so the vectors and the engine were read, not executed. The protocol facts the findings lean on (a cancelled receiver sends no response; closing a Streamable HTTP stream cancels) are taken from OPS-027's text, not rechecked against the Model Context Protocol.
