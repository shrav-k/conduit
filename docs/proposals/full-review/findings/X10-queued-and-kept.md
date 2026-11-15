# X10-queued-and-kept: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Commit 2eb8f258 (2026-11-12). All seven queued items are confirmed against spec/ at this commit; none is refuted.

- Three are contradictions inside spec/ today (F1 to F3). F2 is the worst: no provider can keep PRV-014 and PRV-015 through the list-machines.v1 and place.v1 contracts, and a contract-valid report cannot say `pending`. The staged patch 049b still applies cleanly, but it does not say how `local`, which holds no machine id, is named.
- Three of the queued fixes are one missing promise. "Acceptances one at a time", "races across addresses" and PKG-032 are each a case of deciding against a state that another request changes. DL17 already states the rule, with etcd's precedent, but inside an implementation-defined item for an engine of several parts, outside the promise form and untested. One promise (F6) is smaller than three one-off rules, and it also shows that ACC-033's retired-owner branches decide an unreachable state (F15).
- Three findings are marked unsafe: F9, F6 and F12. F9 is the one I would act on first, because it was held nowhere but a Record.

Kept-for-later items. I read all 62 Records: order 042, and 61 of the 67 order files numbered 067 to 150 (071, 076, 078, 080, 144 and 147 have no Record). By my count they hold about 75 leftover items; the number depends on what one calls an item. About 32 concern spec/:

| Standing at this commit | Count |
| --- | --- |
| Still true and undecided | 17 |
| Closed by a later order, with no mark in the Record that holds it | 7 |
| Spec pending or open in IN_FORCE | 4 |
| Settled, or a stated limit | 3 |
| A build gap against a promise (081's: the engine's own runner and PRV-019, PRV-030) | 1 |

The seven closed silently are SC-F2 to SC-F4 (order 077 took them into its scope), 101's G4 (038b), 117's G6 (038c), and the kit questions of 122 and 124 (081 and 141).

Where the 17 still-true items are tracked:

| Tracked in | Count |
| --- | --- |
| A tracker list (AUDIT known-open, IN_FORCE, or the increment log) | 5 |
| Only the handover's queue (full-review.md and HANDOVER row 7) | 3 |
| Only a non-binding plan, reference or package README | 4 |
| Nowhere but a Record | 5 |

The five held only in a Record are the purge and gap question, recreating a lost record, the bound on a fetch, the dedup scope (F9) and a Resource that starts model work (F19). I did not trace where the roughly 43 non-spec items (build, vectors, experiments, tooling) are tracked.

What this says about Records as a tracking tool: they record history well and track badly.

- A Record is written once and nothing closes an item in it. Seven of the 32 spec items were already fixed, and the only way to learn that was to re-read spec/.
- There is no index. Leftovers sit in prose at the foot of 62 files, and the same item recurs: the safety field in 101, 102, 103 and 117; the DL5 hostname in 127 and 128.
- Routing is by intention: "for the admin", "for order 081", "the admin's next spec order". HANDOVER row 7 points back at "081's Record's leftovers".
- The handover's own process (HANDOVER.md:75-77) makes the Record the home of "the worker's findings kept for later", with no step that carries them on. AGENTS.md sends a first-build finding to the open increment's log, but that log stops at 4-20 (order 074) and holds nothing from orders 122 to 150. Its row 4-08 still says its promises are "still to be written", though ACC-056, IFC-050 and OPS-023 hold them.

The repository already has the right form: a table with a settlement column, as in log-increment-4.md. Give each kept finding a row there, or in the tracker's known-open list for spec findings, have the Record cite the row, and have the order that closes it strike the row.

**What the reader read.** Read at 2eb8f258:

- The brief and the records of what is known: docs/guides/full-review.md in full; AGENTS.md, "What holds"; IN_FORCE.md's lines marked spec pending or Open, and topic 10; docs/AUDIT.md, "Known to be open"; docs/guides/cold-readers.md, "What is known to be open"; docs/implementation/HANDOVER.md (queue, decisions the owner may strike, the Record rule).
- spec/README.md: the form of a promise, Conformance, and the limits on the short form, the stated limit on a run beside a later instance, and records.
- Promises read in full: PRV-002 to PRV-030; ENR-001 to ENR-012; REC-001 to REC-006; PLC-007, 008, 013, 023, 026 and 027; OPS-003, 004, 008, 009, 012 to 014, 018, 020 to 022, 024 and 034; REV-005 to 007, 009, 010, 012 and 014; LIF-011, 020, 021, 036 and 040; ACC-033, 056 and 057; CTR-002 and 013; PKG-024, 025 and 032; SEC-015 to 017; PND-013 in part; IFC-006 and 023; SCT-004 and 015; INS-013; OBJ-002.
- Core contracts: run.v1, stop.v1, inspect-run.v1, list-machines.v1, place.v1, set-up.v1, open.v1, inspect.v1, reconcile.v1, purge.v1, retire-with-resources.v1 and the six enrollment operations; condition.v1.
- spec/implementation-defined.yaml: DL5, DL6, DL8, DL10 and DL17. spec/definitions.md: the rows cited.
- Part B sources: the Record of order 042 and of every order file numbered 067 to 150 that has one (62 Records); docs/proposals/first-build/log-increment-4.md in full.
- Supporting, non-binding: the plan-and-apply reference (replacement, safety cases, open points); S4's plan and pre-registration; ruling 42's later answer; the kit's README (recovery and lifecycle); packages/provider-local-machine (src/lib.rs and README); engine/crates/shell/src/lib.rs:690-729; both staged patches under docs/work-orders/038b-staged, checked with `git apply --check`, which writes nothing.

Not done:

- Nothing was built or run.
- 049a1's branch is not in this tree, so its one-live-node decision is known only from the handover; the "log" it cites is outside the repository.
- Go's GOARCH names were not checked against Go's source. I took them from the provider's table, which Record 128 says a reader checked.
- I did not verify 081's claim that the shell's own runner lacks PRV-019's and PRV-030's guarantees, nor trace the tracking of the roughly 43 non-spec leftovers.
- I did not consult the MCP text for 126's question on a second `initialize` through a stdio bridge, which I class as a runtime contract's matter.
- Kept items that IN_FORCE already holds as spec pending or open were confirmed as listed and not re-examined: the rule on secrets for an Agent that others drive, the safety-vector field, PLC-024's loss cause, a repeated argument under MAN-010, and the CLI's current context.
- Findings 4-01 and 4-11 are the owner's and were not re-argued, beyond F7's narrow case.
- The Records' model-checking findings (MC2-001, MC4-001, 002 and 009, MC6-001 and 002, MC7b-001 and 002) are tracked in G5.md and were taken as settled by orders 077, 081 and 082 without re-running any model.


## FR-245

**PRV-021's owed-stop sentence says failed where PRV-019 requires unknown for the start's own instance**

X10/F1 · contradiction · verifier: confirmed, medium · decides: coordinator · known-new-situation

**Promises.** PRV-021, PRV-019, PRV-007, PRV-030, OPS-022

**Where.** spec/promises/providers.yaml:662 (repeated at spec/core/operations.yaml:886)

**Quote.** "Under such a contract, a start under `run.v1` that finds a stop still owed fails, naming that stop, and may be retried once the stop is carried out (PRV-019)."

**Situation.** Queued item 1, confirmed. The sentence merges three cases. (a) The stop still owed is for the start's own ID: a retirement's stop was held, and a further retirement asks for the run again for its retire hook (TPL-018). A run of that ID may still be running, so PRV-019's `then` requires unknown ("Where it cannot tell whether that run is running, it starts none and the call ends unknown", providers.yaml:579-581), while this sentence and run.v1's description say failed. (b) The provider cannot read its own run's state: the same. (c) Only another instance's stop is owed on the machine and the provider knows no run of the start's ID is running: failed is right under PRV-019's "cannot run the item on that machine". A provider author who follows PRV-021 answers failed, "and nothing is run", in (a) and (b). The kit did so until order 141 (docs/work-orders/124-a63-rk1-finish.md:132, docs/work-orders/141-a63-rk-own-owed-stop.md:126-129) and now answers unknown; a second author would diverge. On the engine's side failed ends the part failed (OPS-022) where unknown keeps it for reconciliation after reading the condition Running (PRV-009).

**Counterargument.** PRV-019's `then` already decides (a) and (b), and a `then` outranks an `observable` and an Operation's description; the shipped kit already implements the three cases; failed is literally true of the call, which started nothing.

**Least change.** Replace providers.yaml:661-664 with: "A provider's contract may promise to carry out every stop it still owes on a machine before starting any run there. Under such a contract a start under `run.v1` that finds such a stop starts nothing. Where a stop is owed for the start's own ID, or the provider cannot read the state of its own run, a run of that ID may still be running: the call ends unknown under PRV-019, and its answer names each stop owed for that ID with its machine and, where the state cannot be read, each stop owed on the call's machine. Where only stops of other instances are owed on that machine and the provider can tell that no run of the start's ID is running, the call ends failed, nothing is run, and its answer names each such stop by instance and machine. Either answer says that the start may be tried again once those stops are carried out. This is the provider's obligation under its contract, not an engine rule." In run.v1's description (operations.yaml:885-887): "a start that finds an owed stop starts nothing: it ends unknown, naming the stops, where one is owed for its own instance or its own state cannot be read, and failed, naming them, where only other instances' stops are owed there; it may be retried once they are carried out (PRV-019, PRV-021)". Vectors: three cases in the runtime suite of experiments/022-provider-contracts (own owed stop gives unknown naming it; unreadable state gives unknown naming every stop on the machine; another instance's stop gives failed naming it), run against provider-local-process and provider-ssh-runtime; one engine vector in which run.v1 ends unknown in a retirement's hook part, the part ends unknown and the stop is still asked for (OPS-022, TPL-009).

**Precedent.** The kubelet, which carries out what its Node should no longer run before it starts Pods (already cited by PRV-021); the kit's own README, order 141.

**Who decides.** coordinator. A contradiction whose fix adds no behavior (rulings 37 and 46 point 4). The split is the owner's MC4-002 answer (ruling 42, later answer of 2026-11-12) as the kit built it.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:661-664 and repeated at spec/core/operations.yaml:885-887. PRV-019's `then` (providers.yaml:579-581): "Where it cannot tell whether that run is running, it starts none and the call ends unknown". Nothing else in spec/ splits the cases. It is the queued item (docs/guides/full-review.md:132-134) and order 141's kept finding (docs/work-orders/141-a63-rk-own-owed-stop.md:132-133). The kit already answers in three cases (packages/runtime-provider-kit/README.md:201-211). Not unsafe: the call starts nothing either way.

**Verifier's corrections.** (1) A third copy is missed: spec/core/README.md:418-420, "a start that finds an owed stop fails, naming it". The least change must amend it too. (2) Case (a) is stated too broadly. Where the provider can tell that the run is running, PRV-019 gives "the call succeeds" (providers.yaml:578-579); where the start's own drain carries out the stop, the start proceeds (kit README:183-185). Word case (a) as "an owed stop of its own ID that it could not carry out, so it cannot tell whether that run is running", so the new text rests on PRV-019's clause and does not contradict it. (3) docs/work-orders/124-a63-rk1-finish.md:130-133 records a different symptom (the start's own drain cancelled the start, answered failed), not the unsplit answer.


## FR-246

**PRV-014 and PRV-015 cannot be kept through the list-machines.v1 and place.v1 contracts; the staged fix does not say how `local` is named**

X10/F2 · contradiction · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** PRV-015, PRV-014, PLC-026, PLC-008, ENR-002, ENR-004

**Where.** spec/core/operations.yaml:1096 (schema at :1110 and :1084; against spec/promises/providers.yaml:469)

**Quote.** "`hostname` names the machine, and is absent where the provider places the instance on none."

**Situation.** Queued item 2, confirmed, with two further situations. (1) PRV-015: "the provider names by id one machine"; place.v1's output schema holds only `hostname`. A provider cannot keep both: packages/provider-local-machine keeps the schema and says so (its README:21-24). (2) PRV-014 requires each machine's `id` and the state `pending`; list-machines.v1's schema has no `id` and its `state` is `{enum: [reachable, unreachable, gone]}`. A report that says pending fails validation, is read under PRV-005 and is not used, so no contract-valid report can drive ENR-004. (3) New: once place.v1 returns an id, `local` cannot be named. PLC-008 gives machine {provider, id, hostname} only "for an admitted node", and PLC-026 has local represent "only the engine's hostname". A provider that reports the engine's own machine under an id (provider-local-machine does, from a key) names an id no node holds. One builder maps it to local through the latest report's hostname; another reports unavailable under "names a machine for which no node represents yet"; after a restart and before the first report the first builder has nothing to map by.

**Counterargument.** IN_FORCE topic 10 records this as spec pending, with the exact patch staged (docs/work-orders/038b-staged/049b.patch) and held until 049a1 on purpose; builds follow spec/ until it is written. The staged patch still applies cleanly at this commit (git apply --check, which writes nothing).

**Least change.** Apply the staged hunks for list-machines.v1 and place.v1 (docs/work-orders/038b-staged/049b.patch:37-99) with one addition: place.v1's output holds `id` and `hostname`, both as list-machines.v1 reports that machine. PRV-015 gains: "The node that represents the machine is the node that is not retired whose status holds that provider, id and hostname, or `local` where the hostname is the one that `local` represents; an id and hostname that name neither report unavailable under this rule." Vectors: placement by id on an admitted node; placement on local by hostname; a pending id (unavailable with its diagnostic); an unrepresented id; a report holding a pending machine that validates against list-machines.v1.

**Precedent.** Kubernetes: the scheduler binds a Pod to a Node by name (`spec.nodeName`), while the Node carries the provider's own id in `spec.providerID`.

**Who decides.** coordinator. Ruled and marked spec pending (IN_FORCE topic 10; rulings 50, 51 point 8, 56 and 68 point 3). The sentence for `local` settles an ambiguity without adding behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, medium. Quote verbatim at spec/core/operations.yaml:1096-1097. Schemas at :1080-1084 (state enum without `pending`, no `id`) and :1108-1110 (output `hostname` only), against PRV-014 (providers.yaml:423-428) and PRV-015 (:467-469, "the provider names by id one machine"). Parts 1 and 2 are already recorded as spec pending with the exact patch: IN_FORCE.md:224, "049b.patch lands list-machines.v1, place.v1, machine-provider.v1", and docs/guides/full-review.md:135. The staged patch changes the output to `id` only (docs/work-orders/038b-staged/049b.patch:96-99).

**Verifier's corrections.** Only part 3 is new, and it is smaller than stated. At this commit place.v1 returns the hostname, so `local` can be named; the question arises only once 049b lands with an id-only output. Builder B's reading is not sound: PRV-015's observable presupposes that local's machine can be named (providers.yaml:488-490, :493-494, and :486-487, "The provider places only on a machine it has reported"), so the id resolves through the report's hostname. The real residue is the window after a restart before the first report (placement.yaml:325-326), when local's status holds no id (placement.yaml:129-131). Class of the new part: ambiguity in a staged, non-binding patch, low; carry it as a note to order 049b. I did not run `git apply --check`; the hunks' context lines are present at this commit (spec/core/README.md:42, :153-155; spec/core/capabilities.yaml:23; tools/check_schemas.py:167).


## FR-247

**Three passages say a node's status holds the labels "last reported"; ENR-010 fixes them at admission**

X10/F3 · contradiction · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** PRV-027, LIF-036, ENR-010, PLC-008, PLC-026

**Where.** spec/promises/providers.yaml:925 (also :929, spec/conventions/lifecycle.yaml:759, spec/core/operations.yaml:996 and :1071)

**Quote.** "`run.v1` is given `machine` in the same form, under PRV-007, since a node that is not retired holds the labels last reported for the machine it represents, under PLC-026."

**Situation.** Queued item 3, confirmed. ENR-010: "No later report adds, changes or removes a node's labels". A machine admitted as amd64 under one hostname later reports another architecture or name: the node keeps the fixed labels and gets a condition. PRV-027's `then` gives "the labels that the status holds", the fixed ones, but its observable, LIF-036's observable ("its status still holds the labels last reported for its machine") and stop.v1's description ("those last reported where the node is retired") say the latest report's. Builder A sends the fixed labels in stop.v1 and inspect-run.v1. Builder B, reading stop.v1's contract, keeps the latest report for a retired node and sends that. A provider that keeps no state finds the run by `machine`; given B's hostname it looks under the wrong name, finds nothing, and "Stopping a run that is not running succeeds". A fifth place, list-machines.v1's "Labels are reported status and never configuration" (operations.yaml:1071), contradicts ENR-010's "not freely mutable reported status".

**Counterargument.** PRV-027's `then` is unambiguous and right; the passages are explanation; where no report differs from admission the readings coincide. Order 038b logged the wording and 081's Record repeats it.

**Least change.** PRV-027 observable: "since a node that is not retired holds the labels fixed for it under ENR-010" and "its status still holds the labels fixed for it, under ENR-010 and PLC-008". LIF-036 observable: "its status still holds the labels fixed at its admission, under ENR-010, so the stop names the machine as `run.v1` did". stop.v1 description: "as `run.v1` gives them, the labels fixed for the node under ENR-010, which a retired node's status still holds". The list-machines.v1 sentence goes with 049b.patch (F2). Vector: a report after admission differs (a condition is added, ENR-010); a later retirement's stop.v1, and an unregister's stop after the node is declared lost, carry the fixed labels (PRV-027, LIF-036).

**Precedent.** ENR-010 itself; no outside standard is needed.

**Who decides.** coordinator. A contradiction whose fix adds no behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/promises/providers.yaml:924-925; also :929, spec/conventions/lifecycle.yaml:758-759 and spec/core/operations.yaml:994-996, against ENR-010 (spec/promises/enrollment.yaml:248-249), "No later report adds, changes or removes a node's labels". Already queued (docs/guides/full-review.md:136) and logged (docs/work-orders/081-spec-one-run-per-instance.md:627).

**Verifier's corrections.** Real as stale wording, but no builder divergence follows. Both obligations already say which labels are sent: PRV-027 `then` (providers.yaml:920-922), "as the node's status holds it, whether or not the node is retired", and LIF-036 `then` (lifecycle.yaml:746-748). A status holds one set of labels and ENR-010 fixes it, so builder B would have to break PRV-027's `then`. The wrong-machine stop does not arise for a promise-keeping engine. Class: defect of explanatory text, not a contradiction that splits builders. The fifth place (operations.yaml:1070-1071) is already removed by the staged patch (docs/work-orders/038b-staged/049b.patch:53-58). No new situation beyond the queue; the drafted wording is fine.


## FR-248

**Nothing keeps one machine to one live node: a second admission in flight, and a machine id admitted again under a new hostname**

X10/F4 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

**Promises.** ENR-006, ENR-007, PLC-026, PRV-015, ENR-004, PLC-023, ENR-012

**Where.** spec/promises/enrollment.yaml:135

**Quote.** "(3) The hostname is neither local's, a non-retired node's, nor another pending machine's."

**Situation.** Queued item 4, confirmed; the decision exists only in the handover (docs/implementation/HANDOVER.md:266). Two paths give two non-retired nodes for one machine with every promise kept. (a) Root's admit-machine.v1 is dispatched and its accept-machine.v1 call is in flight; a second admit-machine.v1 with another idempotency key arrives. ENR-006's checks read an open enrollment, a pending machine and no node, so it is dispatched too; both accepts succeed and ENR-007 makes a node each time ("Once it succeeds, the engine registers a node"). (b) No race: a machine admitted as (provider, id X, h1) is renamed and joins again by a new enrollment, reported pending under h2. Check 3 tests the hostname only, so X is admitted again while its first node is not retired. Then PRV-015's "the node that represents that machine" has two answers in (a), and ENR-008's conflict "naming the node" and ENR-012's one removal question presume one. Related and undecided: PRV-014 lets a provider report `pending` a machine that a node represents; ENR-004 then says "No node represents it", PLC-026 says the node represents it "until it is retired", and PLC-023 holds requests back only for unreachable or omitted.

**Counterargument.** 049a1's build already decides acceptances one at a time and refuses the second with a conflict naming the node; DL17's commit rule covers (a) on an engine of several parts; (b) needs a provider that reports a known id pending again.

**Least change.** ENR-006, check 3: "(3) The hostname is neither local's, a non-retired node's, nor another pending machine's, and no node that is not retired holds this provider and machine id." ENR-006 `then`, added last: "The engine decides requests to admit one at a time: it begins to decide one only once every request to admit that it began to decide before has ended, and decides it from the enrollments, the latest report and the nodes as they then stand, as PKG-032 decides installs." ENR-006 observable: "Of two requests to admit one machine, the second reports conflict under checks 2 and 3 once the first has made its node, with a diagnostic naming that node; where the first failed, it is decided afresh." PLC-026 `then`: "At any time at most one node that is not retired holds a given provider and machine id, and at most one a given hostname." Vectors: a second admission after the first succeeded (conflict naming the node); the same id under another hostname (conflict under check 3); a second admission after a failed accept (succeeds; 049a1's branch has it as admit-accept-failed).

**Precedent.** Kubernetes: one Node object per node name, a second create answered 409 AlreadyExists. PKG-032, which cites Cargo's package-cache lock, for deciding one at a time.

**Who decides.** coordinator. Logged as the coordinator's decision for the owner to strike (HANDOVER, "Decisions the owner may strike"); a small finding that adds a check (ruling 71 point 4), off the S1 rerun's path since enrollment sits on `s3/m2`.

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/enrollment.yaml:135. (a) The enrollment stays open until the accept succeeds (ENR-007, enrollment.yaml:165-167, "Once it succeeds, the engine registers a node"), and no promise orders two admissions; PND-008's "already answered actions report conflict" (pending.yaml:352-353) covers only a request decided after the answer. (b) Check 3 tests the hostname only; PLC-026 leaves the old node "omitted" for "an id under another hostname" (placement.yaml:402-404) but does not forbid a second non-retired node with that id. The decision exists only at docs/implementation/HANDOVER.md:266 and in the queue (docs/guides/full-review.md:137).

**Verifier's corrections.** None on substance. (a) is one case of the general race in F6; (b) is the part the queue did not have. The pending-while-represented remark is also right: ENR-004 (enrollment.yaml:93, "No node represents it") against PLC-026 (placement.yaml:392, "until it is retired"), with PLC-023's `when` covering only unreachable or omitted. Not unsafe.


## FR-249

**What a request reports when the engine cannot keep its state is left to each engine (DL10); only a read of a secret is decided**

X10/F5 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

**Promises.** OPS-008, LIF-040, ACC-044, REC-001, SEC-016, OPS-013

**Where.** spec/promises/secrets.yaml:416 (and spec/implementation-defined.yaml:88)

**Quote.** "How the engine behaves for other requests when its storage is exhausted is DL10."

**Situation.** Order 082's first question, confirmed. The store refuses writes (disk full, read-only volume). A read of a secret is decided: SEC-016, not dispatched, unavailable. Everything else is DL10's "behavior when storage is exhausted": a request whose invocation state cannot be kept (OPS-008); a close or retirement whose tombstone cannot (LIF-040); an open whose provenance cannot (ACC-044); an install. OPS-008 and LIF-040 are musts, so a promise-keeping engine cannot act unrecorded, but no promise says what the request reports (unavailable, failed, unknown or a protocol error), that the state is kept before the effect it guards, or how a retirement ends that dispatched its hook and then cannot keep its tombstone. A caller's next step differs with each answer: ask again, repeat as a new attempt, or reconcile.

**Counterargument.** DL10 leaves exhaustion to the engine on purpose and "A choice must keep every promise" bounds it; the tracker already lists the question as the coordinator's.

**Least change.** A new promise after OPS-036, with the next unused ID (OPS-037; OPS-035 is spoken for in the batch plans): "when: A request would be dispatched, or the engine would register, open, close, retire, remove, install or lift, and it cannot keep the state that a promise has it keep for that: the invocation's state (OPS-008), a record (REC-001), a tombstone (LIF-040), provenance (ACC-044) or an installed package. then: Nothing is dispatched and nothing is changed, and the request reports unavailable. observable: The engine keeps that state before the dispatch or the change it belongs to, as a database writes its log ahead of the change, so nothing takes effect that it could not keep; SEC-016 is this rule for a read. The diagnostic names this rule and what could not be kept, and no path. What cannot be kept once an invocation has been dispatched, such as its end, leaves its outcome unknown, under OPS-013. The same request made once the engine can store is decided as any other." DL10's last clause becomes "and how the engine learns that its storage is exhausted and recovers; what a request then reports is OPS-037". Vectors: a store fault injected before dispatch for a request, a close, an open and an install (unavailable, no dispatch, state unchanged); one fault after dispatch (unknown).

**Precedent.** Write-ahead logging (PostgreSQL fails a transaction whose log cannot be written); etcd's NOSPACE alarm, which refuses writes and keeps reads; SEC-016 inside the specification.

**Who decides.** coordinator. docs/AUDIT.md:238-241 gives order 082's remaining questions to the coordinator "to settle as defects, logged for the owner to strike" (rulings 37 and 46 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/secrets.yaml:416-417; DL10 at spec/implementation-defined.yaml:88, "and behavior when storage is exhausted". No other text in spec/ decides it (search for exhaust, cannot keep, cannot store). Already recorded with this exact scope at docs/AUDIT.md:238-241 and docs/guides/full-review.md:138; the tracker gives it to the coordinator.

**Verifier's corrections.** Class: this is behavior that spec/ leaves to each engine on purpose (DL10), reopened by the tracker, not a situation no text addresses. The drafted promise needs one addition (see extra 1): with a store that refuses writes the engine also cannot keep the record of the refused request, which REC-001 requires (spec/conventions/records.yaml:17-23), and SEC-016 already has that tension. The new promise must say which records are not owed then, or it cannot be kept. It adds an obligation on every engine, so log it for the owner to strike as the tracker says.


## FR-250

**Races across addresses: the rule that would decide them is stated only in DL17, for an engine of several parts, outside the promise form**

X10/F6 · gap · unsafe · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** CTR-013, PKG-032, PND-013, OBJ-002, PRV-013, LIF-041

**Where.** spec/promises/contracts.yaml:313 (rule at spec/implementation-defined.yaml:199-208)

**Quote.** "The engine compares the client IDs of a Type's logins with every Type it holds at another address: every installed package's Types and every submission's Types held by a template or instance (SUB-008, PKG-029)."

**Situation.** Order 082's second question, confirmed. Steps: root installs package A (a Type whose login states clientId X) while package B at another address (a Type with clientId X) is being installed, or while a project whose Type states X is being registered. Each decision reads the Types held; neither holds X; both pass CTR-013 with no `confirmClientIds`; both are stored. Two Types at different addresses now share X unconfirmed, the case PND-013 relies on CTR-013 to stop ("a Type claiming another Type's client", pending.yaml:281-282). PKG-032 orders only requests for one address and version. The general rule exists: "A decision that admits one of several competing changes ... is committed against the state that it read" (DL17). But it sits in an implementation-defined item whose subject is an engine of several parts, is no promise and has no vector, so an engine of one part that decides two requests on two tasks is bound by nothing. The same hole lies under every check against present state: OBJ-002, PRV-013, LIF-041 at register, ENR-006 (F4), and LIF-020 against a member's registration (F15).

**Counterargument.** Promises worded as invariants (PRV-013's "at any time at most one") bind the outcome however requests interleave; DL17's sentence opens "Every decision that a promise has the engine make", which can be read generally; installs are root's and rare.

**Least change.** One convention promise in place of further one-off rules (next unused ID, OPS-038): "when: The engine decides a request, and what another request stores, after the state this decision read and before what it admits is stored, would change how a promise decides it. then: What the decision admits is stored only against the state that it read; where that state has changed, the request is decided again from the state as it then stands. observable: Requests are answered as if each were decided, and what it admits stored, one after another in some order, on an engine of one part as of several (DL17), as an etcd transaction compares a key's revision before it writes and the Kubernetes API server answers a stale `resourceVersion` with 409. Of two installs at different addresses whose Types share a client ID, or an install and a project's registration, the one stored second is decided against the first and reports conflict under CTR-013 unless it confirms; of two registrations of one name one is rejected under OBJ-002. A decision that makes a provider call before it stores, an install's fetch under PKG-032 and an admission's accept under ENR-007, is decided one at a time as those promises state." DL17 then cites this promise in place of restating it. Vectors: both serial orders of the two-address install and of install against registration (the second reports conflict naming CTR-013, the client ID and both Types); the interleavings go to model checking (MC7b's rerun with PKG-032, experiment 012).

**Precedent.** etcd transactions and Kubernetes' optimistic concurrency (`resourceVersion`, 409 Conflict), both already cited by DL17; serializable isolation in SQL.

**Who decides.** coordinator. docs/AUDIT.md:238-242: order 082's remaining questions are the coordinator's to settle as defects, logged for the owner to strike (rulings 37 and 46 point 4). It states for every engine what DL17 already asks of one of several parts.

**Verifier.** weakened, medium. Quote verbatim at spec/promises/contracts.yaml:313-315. PKG-032 orders only one address and version (spec/promises/packages.yaml:450-452, :465-466). The general rule sits in DL17 (spec/implementation-defined.yaml:199-208). No promise serializes decisions otherwise (search for one at a time, linearizable, in some order). The scenario is the one already recorded at docs/AUDIT.md:238-242, "races across addresses, between two Types claiming one client ID or an install and a project's registration", and docs/guides/full-review.md:138.

**Verifier's corrections.** (1) No new situation: the DL17 observation locates a fix; the scenario is the tracker's. (2) The unsafe mark is too strong. Worked through: the racing actor must hold `install.v1` (root) or `register.v1` on `management`, and spec/README.md:467-470 already trusts that actor "to choose the contracts and the implementation items, programs and `remote` URLs"; spec/README.md:423-424 says a process of the engine's OS user can read every state directory. The race grants nothing those actors could not take. What is lost is root's confirmation, a silent result under ruling 52. Call it a gap, not unsafe. (3) The harm is a write skew that only when-form checks allow (CTR-013, ENR-006, LIF-020 against registration); promises worded as invariants (PRV-013, providers.yaml:398-399; OPS-008) hold however requests interleave. (4) The drafted promise binds every decision of every engine, which is wider than the two races the tracker assigns; a narrower fix extends PKG-032's one-at-a-time rule to installs and project registrations whose Types state signIn. Either goes on the review list for the owner to strike.


## FR-251

**spec/ names no request for an edited configuration value: apply may call the control, show a difference and do nothing, or replace the instance**

X10/F7 · gap · verifier: confirmed, medium · decides: owner · known-new-situation

**Promises.** OPS-020, REV-007, REV-011, CTR-006, SCT-015, INS-013

**Where.** spec/README.md:345

**Quote.** "After apply has completed its explicit requests, the stored definitions correspond to the translation apart from IDs, held package pins and configuration values changed by controls (INS-013, NAM-010, REV-007)."

**Situation.** Queued item 6, confirmed; the fault is in a statement of limits and in the non-binding reference. An author edits one configuration value of an existing instance in the project (S4's T1: `blockedBy`) and runs `conduit apply`. spec/ says plan and apply are "composed of existing requests" and names none for this edit. Builder A calls the control whose `changes` holds that pointer, with the instance's revision (OPS-020, REV-011), the only request that changes a stored configuration in place. Builder B follows the reference: "A changed control value (REV-007) is a shown difference, never reverted by this planner, under any flag. No control call is inferred from a file edit here" (docs/implementation/plan-and-apply/README.md:214-215), so the edit never takes effect and no flag makes it. Builder C replaces: retire, remove, lift, register again under `--replace`, losing the run, an Agent's sign-in and the Conduits' IDs. S4's plan counts the author's direct call of the control meanwhile (experiments/015-s4-partial-setup/plan.md:206-214). This is finding 4-01's question, changing what exists, in the one case the language already answers.

**Counterargument.** Plan and apply are tooling, stated as limits and not promises; ruling 76 fixed five readings and this was not among them; a file's value and a value a control set may differ on purpose, so inferring a control risks reverting a deliberate change.

**Least change.** In spec/README.md, "What the short form's meaning covers", after the sentence quoted: "Where an edit changes, on an instance that exists, only values at places that a control of its type lists in `changes` (CTR-006), the plan prints and apply sends that control with those values and the instance's current revision (OPS-020, REV-011), through an open Conduit of the caller that lists it; with no such Conduit the plan says so and stops. Where the stored value differs from the one last applied because a control changed it since, the plan prints the difference and sends nothing without `--adopt`. Any other changed value is a replacement." The reference (lines 214-215) and its golden cases follow, with S4's T1 repair as a new case.

**Precedent.** Terraform: an attribute its provider can update in place is updated, and any other forces replacement. Kubernetes server-side apply: a field another manager set is a conflict unless forced.

**Who decides.** owner. It adds a behavior to apply beyond ruling 76's five readings, and may move S1's counts where a task's edit is at a control's place, which ruling 71 point 4 withholds from the coordinator until the rerun. IN_FORCE holds 4-01 open to the owner.

**Verifier.** confirmed, medium. Quote verbatim at spec/README.md:345-347. spec/README.md:349 says only "Plan and apply are tooling composed of existing requests" and names no request for an edited value. The reference: docs/implementation/plan-and-apply/README.md:214-215. S4's plan counts the direct control call meanwhile (experiments/015-s4-partial-setup/plan.md:206-214). Queued at docs/guides/full-review.md:139-141. Owner decides: finding 4-01 is held open to the owner (IN_FORCE.md:310).

**Verifier's corrections.** Builder B is misdescribed. The reference's "shown difference, never reverted" is about a value a control changed in the store. A file edit is a definition difference, which the reference plans as a replacement under `--replace` (reference :102, :114, and T6 at :180-181); spec/README.md:359-361 states replacement as the route. So B and C coincide, and the divergence is control in place against replacement, not "the edit never takes effect". I found no golden case for an edit of an instance's configuration alone, so treat that reading as probable, not certain. The text at fault is a statement of limits, not a promise.


## FR-252

**The engine's first-start report passes Rust architecture names through where PRV-014 asks for Go's, and ENR-010 then fixes the wrong label for life**

X10/F8 · defect · verifier: confirmed, low · decides: coordinator · known-new-situation

**Promises.** PRV-014, PRV-003, ENR-010, PLC-007

**Where.** engine/crates/shell/src/lib.rs:718 (against spec/promises/providers.yaml:428)

**Quote.** "`kubernetes.io/os` and `kubernetes.io/arch`, whose values are those that Go gives `GOOS` and `GOARCH`"

**Situation.** Queued item 7, confirmed; a defect of the build, spec/ is right. `local_machine` (lib.rs:705-729) maps only aarch64, x86_64, x86, powerpc and powerpc64 and ends `arch => arch`. A LoongArch host gets kubernetes.io/arch: loongarch64 where Go says loong64; little-endian MIPS gets mips and mips64 for mipsle and mips64le; riscv32 stays riscv32 for riscv; big-endian ARM gets arm and arm64 for armbe and arm64be. packages/provider-local-machine/src/lib.rs:24-43 holds the full table, with tests at :179-190. New situation: ENR-010 fixes local's labels at first start, "no operation changes a node's labels after it is made", and `local` is made only at first start. A store first started with the wrong name keeps it: a type's `nodeSelector` of kubernetes.io/arch: loong64 never matches `local` (PLC-007), and a later correct report only adds a condition. Correcting the table repairs no existing store.

**Counterargument.** The architectures the project runs on (arm64 macOS, amd64 Linux) map correctly; no one has first-started on an affected host.

**Least change.** No change to spec/. Replace the match at engine/crates/shell/src/lib.rs:717-726 with the provider's table, as one shared function if both can depend on it, and copy the provider's table test. Log it as a finding of the first build in the open increment's log. If existing stores are to be repaired, that is DL6's "how repair works", stated in engine/CONFORMANCE.md. No portable vector can test a host-dependent value; a unit test of the mapping does.

**Precedent.** Kubernetes' well-known label kubernetes.io/arch, whose value is Go's runtime.GOARCH (already cited by PRV-014).

**Who decides.** coordinator. A defect of the build, fixed in the build and logged (ruling 46 point 4; ruling 71 point 4).

**Verifier.** confirmed, low. engine/crates/shell/src/lib.rs:717-725 maps only aarch64, x86_64, x86, powerpc and powerpc64, then `arch => arch`. The provider's full table is at packages/provider-local-machine/src/lib.rs:24-43, tested at :176-190. The spec quote is verbatim at spec/promises/providers.yaml:427-429. ENR-010 (spec/promises/enrollment.yaml:246-249) fixes local's labels at first start, so the added situation holds: correcting the table repairs no existing store. Queued at docs/guides/full-review.md:142; also in docs/work-orders/128-a63-mp1-local-machine.md:155-157 and packages/provider-local-machine/README.md:17-18.

**Verifier's corrections.** None. A build defect; spec/ is right. No affected host is in use.


## FR-253

**An implementation that deduplicates cannot reproduce the scope of an idempotency key: it is sent the key and never the sending instance**

X10/F9 · gap · unsafe · verifier: confirmed, high · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-221.

**Promises.** CTR-002, OPS-008, OPS-012, OPS-018, OPS-024, IFC-006

**Where.** spec/promises/contracts.yaml:43

**Quote.** "`deduplicates`, the mark of an operation whose implementation recognizes an idempotency key that it has already been sent and applies the effect at most once for it"

**Situation.** Kept in docs/work-orders/143-s4-srv1-effect.md:330-332 and nowhere else; still true. OPS-008 scopes a key to caller, target and sending instance. IFC-006 sends the implementation the caller's raw key, the caller and the Conduit, never the sending instance. Steps: Interfaces P and Q each call a `deduplicates: true` operation of one Resource with the key "k1" (keys are callers' own strings, so a naming scheme makes this ordinary). The engine holds two invocations and dispatches both. An implementation written to CTR-002's words recognizes "k1", applies nothing for Q and answers Q with P's result: Q's effect is lost, Q is told succeeded and may be shown P's outputs. An author who scopes by caller and Conduit instead applies an effect twice when a repeat arrives through the replacement of a closed declared Conduit, which OPS-024 calls the same invocation ("OPS-008 does not put the Conduit in the scope of an idempotency key"). An author who scopes by caller alone merges an Agent's own request with a Resource's onward request for that caller. Only the engine's ID for the invocation, which every attempt of one invocation carries, matches OPS-008's scope; S4-SRV1 used it for want of a rule.

**Counterargument.** OPS-018 states the scope for at-most-once ("one instance sends for one caller to one target"), and a careful author would at least add the caller; the engine never dispatches a second attempt of an effectful invocation that succeeded.

**Least change.** CTR-002 and the definitions row for `deduplicates`: "the mark of an operation whose implementation recognizes an invocation that it has already been sent, by the engine's ID for it in `io.github.shrav-k.conduit/invocation`, and applies the effect at most once for it". IFC-006 observable: "Every attempt of one invocation carries the same `invocation`, and no other invocation carries it, so an implementation that deduplicates keys on it. `idempotencyKey` is the caller's own name for the request, unique only within the scope of OPS-008, and an implementation does not key on it alone." Vectors: two callers send one key to one deduplicating implementation and both effects are applied; a new attempt through a replaced Conduit carries the first attempt's `invocation`.

**Precedent.** The IETF draft of the Idempotency-Key header, which the definitions already cite, leaves the key's uniqueness scope to the server and recommends a fingerprint beside the key; Stripe scopes a key to the account.

**Who decides.** coordinator. A gap whose fix states which key identifies an attempt and adds no engine behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, high. Quote verbatim at spec/promises/contracts.yaml:43-44, repeated at spec/definitions.md:217. OPS-008 scopes a key to caller, target and sending instance (spec/conventions/operations.yaml:153-155) and says "The same idempotency key sent by another caller, by another instance for the same caller, or to another target, belongs to another invocation" (:164-165). IFC-006 sends instance, operation, invocation, caller, filters, config, revision, idempotencyKey and conduit, and no sender (spec/interface/implementation.yaml:128-136). OPS-024 lets a repeat arrive through another Conduit (operations.yaml:540-544). No text in spec/ tells an implementation what to key on. Recorded only at docs/work-orders/143-s4-srv1-effect.md:330-331; nothing in AUDIT, IN_FORCE, the logs, G5 or the ledger.

**Verifier's corrections.** None on substance; the unsafe mark stands. An implementation written to CTR-002's words can drop the second caller's effect and answer with the first caller's result; one that keys on caller and Conduit can apply an effect twice after OPS-024. The harmed callers need no special authority. The drafted fix is sound: every attempt of one invocation is "a further dispatch of the same invocation" (spec/definitions.md:215), so the engine's invocation ID matches OPS-008's scope, hooks under OPS-028 included (operations.yaml:653-655). It adds no engine behavior, so the coordinator may settle it.


## FR-254

**A registered machine provider that omits the engine's machine, or names it by the OS hostname under DL5, makes every default-placed local instance unregistrable**

X10/F10 · gap · verifier: confirmed, medium · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-049.

**Promises.** PRV-015, PRV-014, PRV-019, PRV-013, PLC-007, PLC-023

**Where.** spec/promises/providers.yaml:488

**Quote.** "For an instance that has an item that is run and names no `runtime`, the selector holds the hostname of the machine that `local` represents, so the provider names that machine or none"

**Situation.** Kept in docs/work-orders/127-a63-rp1-local-process.md:209 and 128-a63-mp1-local-machine.md:158, and in packages/provider-local-machine/README.md; still true. (a) The engine's hostname is configured (DL5, as `--hostname-override`; needed after a move under DL17). A machine provider runs on the engine's machine (PRV-002) and reports it under the operating system's hostname, as provider-local-machine does. The selector asks for the configured name, the provider names none, and every instance that runs on the engine's own runner with no placement is "not instantiated", its document rejected. A runtime provider there is given a `machine.hostname` it does not know as its own and ends the run failed (PRV-019). (b) New: a machine provider that reports only another machine, as an SSH provider of one remote computer would. PLC-023 allows a report that omits local's machine, PRV-013 allows one machine provider, and PRV-015 has "A provider of one machine ... name that machine or none". Registering it turns default placement on `local`, which the engine's own provider gave, into a rejection that names a selector nothing matched. No promise tells a provider the engine's hostname or requires it to report the engine's machine.

**Counterargument.** An author can write `placement: local`; a provider's contract can take a hostname in its configuration; S1 registers no machine provider.

**Least change.** PRV-015 `then`, after the selector is given: "Where the selector holds the hostname of the machine that `local` represents, the engine does not ask: the placement is `local`." (PLC-007 already makes `local` the only node that can match, and PRV-015 already has a case in which "the provider is not asked".) PRV-014 observable: "A provider that reports the machine the engine runs on reports it under the engine's own hostname (DL5), which its configuration gives it where the engine is configured with one, as its contract states." Vectors: with a registered provider that omits local's machine, an instance with a run item, no `runtime` and no placement is placed on `local` with no place.v1 call; the same with a type that names another hostname is not instantiated.

**Precedent.** Kubernetes: `--hostname-override` is given to the kubelet, the agent that reports the name; a Pod whose `spec.nodeName` is already determined is not sent to the scheduler.

**Who decides.** coordinator. A small finding whose fix adds a behavior (the engine not asking), off S1's path since S1 registers no machine provider (ruling 71 point 4).

**Verifier.** confirmed, medium. Quote verbatim at spec/promises/providers.yaml:488-490. (a) PLC-026: local represents "the engine's hostname, the operating system's unless configured otherwise (DL5)" (spec/promises/placement.yaml:396-397), while a provider's "contract states which name of a machine it reports" (providers.yaml:431-432); nothing gives a provider the configured name. (b) PRV-015: "Where the provider names none, the instance is not instantiated" (:472), and PLC-023 lets a report omit local's machine (placement.yaml:331-332). Held only in Records and a README: docs/work-orders/127-a63-rp1-local-process.md:209-210, 128-a63-mp1-local-machine.md:158-159, packages/provider-local-machine/README.md:22-24.

**Verifier's corrections.** Class: (b) is not a gap. PRV-015 decides it ("names that machine or none", then not instantiated); the fault is that the decided outcome is needless machinery, since the engine asks a provider a question whose only acceptable answer it already knows. Only (a) is a true gap. It matters more than the finding says: ruling 77's SSH machine provider for S3 reports the second machine (IN_FORCE.md:227), so once it is registered every default-placed instance on the engine's own runner is rejected unless that provider also reports the engine's machine under the engine's name. The fix adds behavior (the engine not asking); the coordinator may settle it under ruling 71 point 4 because S1 registers no machine provider.


## FR-255

**Reconciliation has no true answer for an effect that occurred in part, and spec/ does not say so**

X10/F11 · gap · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** OPS-014, OPS-003, OPS-004, OPS-013, REV-010, OPS-021

**Where.** spec/conventions/operations.yaml:313

**Quote.** "Recording that the effect occurred makes the invocation's outcome succeeded. Recording that it did not occur makes the outcome failed only once the invocation is fenced"

**Situation.** Kept in docs/work-orders/102-pre-b-s4-s5-s6-s9.md:213; row 4-12 of increment 4's log closed it with the owner's agreement that v0 keeps `reconcile.v1` as it is. Still true that spec/ states nothing. S4's E01: a plain server makes part of an effect and answers an error; the outcome is unknown (OPS-004 names "occurred only in part"). The actor who reconciles has two answers. `occurred: false`, once fenced, records failed, which OPS-003 defines as no effect "in whole or in part", and lets a repeat dispatch a new attempt that redoes the part already done (OPS-024). `occurred: true` records succeeded with a part missing and, for a control, stores every value that was sent (REV-010). Neither is true. For a setup hook the unknown blocks every further setup until someone asserts one of them (OPS-021). Nothing tells the actor to leave it unknown, repair, then record.

**Counterargument.** OPS-003 and OPS-004 already define failed and unknown for the implementation, so "repair first, then record" follows for a careful reader; the owner chose not to change `reconcile.v1`.

**Least change.** OPS-014 observable, one sentence: "Where the effect occurred only in part, neither record is true: the invocation stays unknown, under OPS-013, until the actor has completed or undone the part, by the type's own operations or outside Conduit, and then records what it checked; this version records no part." The same as one line under spec/README.md's limits. Vector: S4's T2, a reconciliation after the part was completed.

**Precedent.** Kubernetes finalizers, which PLC-027 already cites: an object stays terminating until its work is done or someone removes the finalizer by hand, asserting that it is.

**Who decides.** coordinator. It writes the limit the owner agreed on 2026-11-12 (docs/proposals/first-build/log-increment-4.md:23) and adds no behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, low. Quote verbatim at spec/conventions/operations.yaml:313-314. OPS-003 (:45) and OPS-004 (:55-56) as cited. Already settled in direction: docs/proposals/first-build/log-increment-4.md:23, "Its reconcile question closes with it: v0 keeps `reconcile.v1` as it is"; and S4's pre-registration asks it as Q5 with the recommendation "leave it unknown, without whole-effect assertions from partial evidence" (experiments/015-s4-partial-setup/README.md:238-240), accepted under ruling 75 D1 (docs/proposals/ruling-75/README.md:77-79).

**Verifier's corrections.** No undecided behavior remains. OPS-014 already has the actor record only what it checked, and its observable says an invocation "can be recorded as having occurred, or stays unknown under OPS-013" (operations.yaml:376-377). What is missing is one sentence stating the accepted limit. Class: an unwritten statement of a limit, not a gap. The drafted sentence is right and the coordinator may write it.


## FR-256

**Restoring an older copy of the store is classed outside the engine for records only; the same copy reopens closed Conduits and forgets keyed invocations**

X10/F12 · gap · unsafe · verifier: weakened, low · decides: owner · known-new-situation

**Promises.** REC-003, REC-004, REC-006, OPS-008, OPS-018, LIF-040, CON-008

**Where.** spec/conventions/records.yaml:107

**Quote.** "A record removed or altered by a fault outside the engine is not removed or altered under this promise: the medium failing, someone other than the engine restoring an older copy of the store, or another process removing or changing its file."

**Situation.** Order 082's Record keeps "whether records restored from a copy taken before a purge may be returned" (docs/work-orders/082-spec-records-and-installs.md:738). Steps: an operator restores the store from last night's copy. (1) Records purged since are back and the purge's own record is gone. The tracker says the owner kept this as a limit, "read as found, and root purges them again" (docs/AUDIT.md:242-243), but spec/README.md:613-617 speaks only of loss and alteration and does not say it. (2) New: the copy also holds the engine's own state. A Conduit closed since is open again and its tombstone gone (CON-008, LIF-040), so its consumer's requests are permitted; a keyed invocation that succeeded since is unknown to the engine, so its repeat is dispatched and the effect applied a second time (OPS-008, OPS-018); revisions and enrollments revert. The engine keeps every promise throughout, and no statement of limits covers it.

**Counterargument.** Every stateful system has this; it is an operator's act with the operator's authority, and "the engine's OS user reaches every token and state directory" is already a stated limit; DL8 lets an engine say which outside losses it can tell.

**Least change.** spec/README.md, "What the promises on records cover", one more point: "Restoring an older copy of the engine's store is outside every promise. Records purged since are read again as found, and root purges them again. Closings, retirements, tombstones, revisions and the state that repeats need, made since the copy, are lost with it: a Conduit closed since is open, and a keyed request repeated since is dispatched anew. Whoever restores inspects and closes again; this version detects no restore." No vector, as for every stated limit.

**Precedent.** etcd's disaster-recovery procedure: a restore starts a new cluster from the snapshot, and Kubernetes' guidance warns that state written since is lost and components must be restarted.

**Who decides.** owner. What the promises cover is scope, and the owner kept the records half of this limit on 2026-11-12; extending it to access and effects is the owner's.

**Verifier.** weakened, low. Quote verbatim at spec/conventions/records.yaml:106-109. The records half is already held as a limit the owner kept: docs/AUDIT.md:242-243, "they are read as found, and root purges them again". spec/README.md:611-617 speaks only of loss and alteration. No text in spec/ covers a restored copy of the engine's own state (search for restor, older copy, backup: only records.yaml:105-108 and DL13's backups).

**Verifier's corrections.** Part 1 is known; its only residue is that spec/README.md does not carry the sentence the tracker attributes to it. Part 2 is real as an unstated limit, but the unsafe mark should go. The cause is an act outside the engine by its operator, whom spec/README.md:500-503 already places inside the domain of trust ("can get past every gate"). REC-003 drew the same line for records, and no promise can hold over state replaced from outside. Class: a missing statement of a limit; the owner decides its scope, as stated.


## FR-257

**Whether reading records after a purge says anything, and whether a lost record may be written again, are undecided**

X10/F13 · ambiguity · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** REC-004, REC-006, REC-003, LIF-013

**Where.** spec/conventions/records.yaml:171

**Quote.** "A result that omits such records says so. It is never presented as complete."

**Situation.** Kept only in docs/work-orders/082-spec-records-and-installs.md:740 ("whether a purge leaves a gap, recreating a lost record"); still true. Root purges records made before a time; an actor then inspects an instance whose earlier records were purged. Reading A: purged records no longer "should exist", so the result lists what remains and says nothing; the purge's record "can be read", but REC-004 does not say which object it names, and where it names none that exists DL8 leaves reading it to the engine. Reading B: a result that omits records "says so", so the result marks a gap at the purge. Under A an actor cannot tell a history that begins at registration from one a purge cut, a silent result under ruling 52. Second question: after an outside loss that the engine can tell, may it write the record again from its own state (OPS-008)? REC-003 forbids removing and altering; REC-006 speaks only of a record that "can be read again". conformance/conventions/records/purge asserts nothing about a later inspection.

**Counterargument.** REC-006's "such records" plainly means unreadable or lost ones, and "A loss of records is not presented as a purge" keeps the two apart; how a gap is shown is already left to the vectors (spec/core/README.md:704).

**Least change.** REC-004 observable: "A purge is not a gap under REC-006. The record of the purge names `management` as its target, so inspecting `management` returns it, and the result of reading the records of an object made before the purge's time says that records before that time were purged, naming that record." REC-006 observable: "The engine writes no record again in the place of one that was lost: a record is made once, when what it records happens." Vector: extend conformance/conventions/records/purge with an inspection after the purge.

**Precedent.** etcd compaction: a read below the compacted revision is answered "required revision has been compacted" (410 Gone in Kubernetes), which says that history was cut on purpose; an append-only audit log makes each event once.

**Who decides.** coordinator. Order 082's Record gives both to the coordinator; an ambiguity whose fix makes a result explicit (rulings 37, 46 point 4 and 52 point 2).

**Verifier.** weakened, low. Quote verbatim at spec/conventions/records.yaml:171. REC-006's `when` (:166-167): "Records that should exist cannot be read, or were lost", and :174-175: "A loss of records is not presented as a purge". The purge's record: REC-004 (:127-128), and purge.v1 is "authorized against `management`" (spec/core/operations.yaml:701). Held only at docs/work-orders/082-spec-records-and-installs.md:739-741.

**Verifier's corrections.** Reading B is not sound: "such records" refers to REC-006's `when`, and purged records no longer should exist, so no gap is owed. The purge's record names `management` as its target (REC-002) and is found by inspecting `management` (REC-001, records.yaml:25), so the DL8 worry does not arise. The residue is a ruling 52 point: an actor who inspects through a Pool or Fleet cannot tell a purged history from a short one. The second question is mostly decided: a loss the engine can tell must be reported as a gap (REC-006 `then`, :169), so a rewritten record cannot stand in for it. Class: a small gap in explicitness, not an ambiguity.


## FR-258

**No bound on a fetch: one install whose fetch never ends holds every later install of that address and version**

X10/F14 · gap · verifier: confirmed, low · decides: coordinator · known-new-situation

**Promises.** PKG-032, PKG-024, PRV-017, PRV-005

**Where.** spec/promises/packages.yaml:450

**Quote.** "It begins to decide a request only once every request to install that address and version that it began to decide before has ended, and decides it from what is installed then."

**Situation.** Kept only in docs/work-orders/082-spec-records-and-installs.md:740 ("a bound on a fetch's duration"); still true. Root installs P at v from a `fetch` whose server accepts the connection and never sends the archive, or through a package-source provider whose fetch-package.v1 never answers. "Deciding includes fetching the archive, before dispatch", and nothing bounds it: DL10's "time allowed for a call" is stated for a call whose part "ends unknown", not for a fetch before dispatch, and the engine's own https fetch is no call. Every later request to install P at v, a correct one with a working `fetch` among them, waits behind it without end: "waiting leaves no state that outlives the request, and nothing wakes on its own".

**Counterargument.** Any engine would bound its fetches, and DL10 can be read to cover a provider call; cancelling the first request frees the others.

**Least change.** PKG-024 `when`: "and its archive cannot be fetched now, or has not been fetched within the time that DL10 gives a fetch". PKG-032 observable: "A fetch is bounded, under PKG-024 and DL10, so a request waits no longer than the fetches before it may take." DL10: add "the time allowed for the fetch of an archive, by a provider or by the engine". Vector: an install whose fetch never ends reports unavailable at the bound, and the waiting install is then decided.

**Precedent.** Cargo's `http.timeout`; the kubelet's image-pull progress deadline.

**Who decides.** coordinator. Order 082's Record gives it to the coordinator; a gap whose fix uses an existing reason and an existing implementation-defined item (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/promises/packages.yaml:450-452; "Deciding includes fetching the archive, before dispatch" (:467) and "nothing wakes on its own" (:469-470). DL10's only time limit is "the time allowed for a call ... after which its part ends unknown" (spec/implementation-defined.yaml:83-84), which names a part and not a fetch before dispatch; PRV-017 has the engine fetch `https` and `file` itself (spec/promises/providers.yaml:535-536), which is no call. Held only at docs/work-orders/082-spec-records-and-installs.md:740-741.

**Verifier's corrections.** None. It is a liveness fault, not a safety one; every real engine bounds its fetches, so the cost is that PKG-032's wait has no stated end.


## FR-259

**ACC-033's retired-owner branches decide a state that no sequence of requests reaches**

X10/F15 · needless-complexity · verifier: confirmed, low · decides: owner · known-new-situation

Same fault, or overlapping: FR-008, FR-212.

**Promises.** ACC-033, LIF-020, LIF-021, LIF-035, POOL-004, FLEET-003, PND-009

**Where.** spec/conventions/access.yaml:653

**Quote.** "Inspection of its remaining members and reconciliation of their invocations, their setups and retirements included, go through `management`."

**Situation.** Kept in docs/work-orders/102-pre-b-s4-s5-s6-s9.md:215 and as Q4 of S4's pre-registration (experiments/015-s4-partial-setup/README.md:214-217, 235); still true. Rows 4, 4A, 6, 6A, 14, 14A and 17 of ACC-033, with inspect.v1, reconcile.v1 and PND-009, each carry a branch for a Resource or Agent that exists while its owning Pool or Fleet is retired. LIF-020 refuses to retire an owner while any member is registered, ready or retired with shutdown unconfirmed; LIF-021 dispatches the owner's retirement only once every member's shutdown is confirmed; LIF-035 removes a member on confirmation; registration into a retired owner is invalid (POOL-004, FLEET-003). So a retired owner has no member. The one path is a member's registration decided while the owner's retirement is decided, which F6's promise would close. S4's E18 (ROADMAP.md:175: "the owning Pool retired, the Resource inspected and reconciled") tests exactly this state and can prepare it only by seeding the store.

**Counterargument.** The branches are cheap defence for a state that a fault, a race or a restored store can produce, they keep "exactly one target authorizes a request at any moment" total, and a later version may allow orphans.

**Least change.** With F6's promise written: remove the retired-owner branch from rows 4, 4A, 6, 6A, 14, 14A and 17 and from the observable's "remaining members" sentence, keeping row 12 (the instance no longer exists) and row 13's branch for a retired owner's own verbs; E18 becomes the reachable case, owner retired, member removed, its invocation reconciled through `management` under row 12. Without F6: keep the rows and add to ACC-033's observable "A retired owner has a member only where the member's registration and the owner's retirement were decided together", and label E18's preparation a boundary seed.

**Precedent.** Kubernetes: foreground deletion holds an owner until its dependents are gone, and orphans arise only from an explicit `--cascade=orphan`; Conduit has no orphaning operation.

**Who decides.** owner. It removes rules and changes a case of a scenario the owner accepted (ruling 75, D); S4's Q4 already asks the owner how E18 is prepared.

**Verifier.** confirmed, low. Quote verbatim at spec/conventions/access.yaml:653-655. LIF-020 (spec/conventions/lifecycle.yaml:413-414), LIF-021 (:423-424, :442), LIF-035 (:723) and POOL-004 (spec/promises/pools.yaml:61) together leave a retired owner with no member by any sequence of requests; I found no other path (Pools and Fleets have no placement, so PLC-017 and PLC-024 retire members only). Recorded as G2 and Q4 at experiments/015-s4-partial-setup/README.md:214-217 and :235-237, and at docs/work-orders/102-pre-b-s4-s5-s6-s9.md:215.

**Verifier's corrections.** Two limits on the claim. (1) Today the state is reachable by a race (a member's registration decided beside the owner's retirement), because nothing serializes decisions (F6), so the rows are not dead until that is fixed. (2) Ruling 75 D1 accepted S4's pre-registration with its listed answers (docs/proposals/ruling-75/README.md:77-79), and Q4's recommendation was "boundary fixture and reachable path, labelled separately", so the least change's fallback branch is already ruled. Removing the rows is the owner's, as stated.


## FR-260

**`expectedId` pins which object a set-up or an open names, not the configuration that the plan showed**

X10/F16 · gap · verifier: confirmed, low · decides: coordinator · known-new-situation

**Promises.** LIF-037, REV-012, REV-011, OPS-020

**Where.** spec/conventions/revisions.yaml:192

**Quote.** "Setting up and retiring an instance, which for a Conduit are opening and closing it, name no revision, change neither its definition nor its configuration, and do not advance its revision."

**Situation.** Kept as WO069-O2 in docs/work-orders/069-spec-cli-plan-apply.md:458 and as O2 of the non-binding reference; still true. A plan prints "open Conduit c: Agent a may write to `shared`" with `shared`'s root as last inspected. Before apply sends open.v1 with `expectedId`, another actor's control changes `shared`'s root (same ID, new revision), or changes an Agent's `toolsDirectory` before its set-up.v1. The request carries only an ID, so it is dispatched: the command approved one thing and opened another. Narrower than the Record says: a Conduit has no control and a re-registered one has a new ID, so `expectedId` suffices for the Conduit itself; the hole is the configuration of its ends and of an instance being set up.

**Counterargument.** In v0 the command is the approval, and the tool can inspect again just before it sends; no comparable system has a precondition on another object; the window is one request wide.

**Least change.** A stated limit in spec/README.md's paragraph on plan and apply: "A plan shows the configuration of an instance and of a Conduit's ends as last inspected; a control between that inspection and the request is not caught, since `expectedId` pins identity and not revision (LIF-037)." If the owner wants it closed, `set-up.v1` takes an optional `revision` for the instance it names, compared as REV-002 compares one.

**Precedent.** Kubernetes' `preconditions`, which carries `resourceVersion` beside `uid` (LIF-037 already cites `preconditions.uid`).

**Who decides.** coordinator. The limit sentence adds no behavior (rulings 37 and 46 point 4); the optional input would be a small feature under ruling 71 point 4, or the owner's.

**Verifier.** confirmed, low. Quote verbatim at spec/conventions/revisions.yaml:192-194. LIF-037 compares only the ID (spec/conventions/lifecycle.yaml:797-801). Held as O2 of the non-binding reference (docs/implementation/plan-and-apply/README.md:466-468) and at docs/work-orders/069-spec-cli-plan-apply.md:457-459; docs/guides/full-review.md:160-162 points the review at O1 to O8.

**Verifier's corrections.** None. The reviewer's narrowing is right: the hole is the configuration of a Conduit's ends and of an instance being set up, one request wide. A sentence in the statement of limits is the coordinator's; an optional `revision` input is a feature.


## FR-261

**The cascade sequence in spec/README.md tells a tool to close a Conduit that CON-008 has already closed and removed**

X10/F17 · defect · verifier: confirmed, low · decides: coordinator · known-new-situation

**Promises.** CON-008, LIF-035, LIF-021, OPS-029, LIF-040

**Where.** spec/README.md:354

**Quote.** "Each cascade is one command, printed and counted under one flag: register its Conduit, open it, call the cascade operation, close the Conduit, then root lifts its own tombstone."

**Situation.** Kept as WO069-O4 in docs/work-orders/069-spec-cli-plan-apply.md:458-459; the reference's O4 says "the specification is not amended here". Still true. The cascade Conduit's target is the Pool or Fleet being retired. When the cascade succeeds the target is retired, so CON-008 closes the Conduit "at once" and "A closed Conduit is removed, under LIF-035". A tool that follows the README and then sends close.v1 names a Conduit that no longer exists and is answered invalid (OPS-029), on the success path of a dangerous command. Only where the cascade does not retire the owner does the Conduit stay open for the tool to close.

**Counterargument.** The passage is a statement of limits and not a promise, and the reference already prints the engine's own closure.

**Least change.** "...register its Conduit, open it, call the cascade operation, which closes that Conduit when the owner retires (CON-008); where the cascade does not retire the owner, close the Conduit; then root lifts the Conduit's tombstone."

**Who decides.** coordinator. A defect whose fix adds no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/README.md:354-356. CON-008 (spec/promises/conduits.yaml:109-115): a Conduit whose target is retired "is closed at once" and "A closed Conduit is removed, under LIF-035"; a later request naming it "reports invalid under OPS-029" (:118-119). Held as O4 of the reference (docs/implementation/plan-and-apply/README.md:474-476, "the specification is not amended here") and at docs/work-orders/069-spec-cli-plan-apply.md:457-459.

**Verifier's corrections.** None. The sentence is in a statement of limits, not a promise, so the defect misleads tool builders only; the reference already prints the engine's own closure (reference :240-243).


## FR-262

**Finding 4-17 (no form for an ended condition) is decided by the paradigm LIF-011 already adopts, which the text does not state: one condition per type**

X10/F18 · gap · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** LIF-011, OBS-002, PRV-009, LIF-013

**Where.** spec/conventions/lifecycle.yaml:210

**Quote.** "They are reported in `conditions` in the instance's status, beside its lifecycle state, each in the shape of Kubernetes `metav1.Condition` (`type`, `status`, `reason`, `message`, `lastTransitionTime`) with `lastHeartbeatTime` and `source`, which are Conduit's additions"

**Situation.** Open row 4-17 of docs/proposals/first-build/log-increment-4.md:28 ("the specification is to say how an ended condition is shown, if at all"), also in the tracker's known-open list; still true. A run is reported stopped and then running. One engine appends a second condition of type `Running` and keeps the first, which is what 4-16 saw: an earlier RunStopped beside a ready state. Another replaces it. LIF-011, `condition.v1` and inspect.v1's schema (a plain array) allow both. New: no new form is needed. Kubernetes keys `status.conditions` by `type` (list type map, key `type`), so one condition of each type exists, its `status` says whether it holds and `lastTransitionTime` since when. LIF-011 took the shape and left out that rule.

**Counterargument.** The CLI already sets ended conditions apart (order 028), and a history of conditions has some use; the records already hold it.

**Least change.** LIF-011 `then`, one sentence: "`conditions` holds at most one condition of each `type`: a later observation of that type replaces the earlier one, and `lastTransitionTime` is when its `status` last changed, as Kubernetes keys a status's conditions by `type`." Observable: "A condition that no longer holds is that condition with `status` `False`; no ended condition is listed beside it, and what it was before is in the records." Vector: an inspection after stopped then running shows one `Running` condition with status `True`.

**Precedent.** Kubernetes API conventions for `status.conditions` (KEP-1623: `+listType=map`, `+listMapKey=type`).

**Who decides.** coordinator. Row 4-17 is the coordinator's to settle and the owner's to strike (ruling 46 point 4); the fix applies the adopted shape and adds no feature.

**Verifier.** weakened, low. Quote verbatim at spec/conventions/lifecycle.yaml:210-212. Already in the known-open list: docs/AUDIT.md:244 ("4-17, whether and how ended conditions appear"), docs/guides/cold-readers.md ("The form of ended conditions (4-17) remains open") and docs/proposals/first-build/log-increment-4.md:28.

**Verifier's corrections.** No new situation: keying by `type` is a proposed settlement of 4-17. The example is already decided. PRV-009 has the status report "a condition whose `type` is `Running` and whose `status` is `True`, `False` or `Unknown` as the state ... last gave" (spec/promises/providers.yaml:310-312), and OBS-003 says "The earlier value is not shown as current" (spec/conventions/observation.yaml:50). The open part is only conditions a Type defines (LIF-011) and whether ended ones are listed at all. The proposal fits those promises and Kubernetes; note that `source` is Conduit's addition (OBS-002), so say whether two sources may each report one type before fixing one condition per type.


## FR-263

**A Type of family Resource may offer an operation marked `startsModelWork: true`, though model work is defined as an Agent's and the Agent rules key on family**

X10/F19 · ambiguity · verifier: confirmed, low · decides: owner · known-new-situation

**Promises.** IMP-001, IMP-003, CTR-004, ACC-057, CTR-014, FLEET-002

**Where.** spec/definitions.md:225

**Quote.** "A turn of an Agent's model, started or continued. It starts only as the effect of an operation whose contract is marked `startsModelWork: true`, dispatched through an open Conduit that lists that operation (IMP-001, IMP-003)."

**Situation.** Kept only in docs/work-orders/143-s4-srv1-effect.md:332 ("a Resource here offers an operation that starts model work, which is an Agent's by definition"); still true. packages/scenario-s4-effect ships a Resource Type whose capability lists such an operation, and it installs. Reading A: the mark is per operation (IMP-003) and any family may offer it. Reading B: model work is by definition an Agent's, so CTR-004 should reject the Type. Under A, a Resource that runs a model is registered by any actor holding a Pool's and `management`'s Conduits (ACC-033 row 1), where ACC-057 lets only root register an Agent; it has no Fleet, no `ownTools` in inspection or in a plan (CTR-014 is for an Agent Type), and batch 2's rule on secrets for an Agent that others drive will not name it.

**Counterargument.** The family is the package author's declaration either way: a Resource's program can call a model with no mark at all, so forbidding the mark only hides it, and a model gateway is a fair Resource.

**Least change.** The owner chooses. (A) definitions.md: "model work | A turn of a model, an Agent's or one that a Resource's implementation runs, started or continued. ...", and IMP-003's observable: "An operation of any family's Type may carry the mark." (B) CTR-004: "A Type whose family is not Agent, and that offers a capability listing an operation marked `startsModelWork: true`, is rejected when its package is installed or its project is submitted."

**Who decides.** owner. What an Agent is, and what the Agent-only rules (ACC-057; ruling 59 A1) are meant to cover, is purpose and scope.

**Verifier.** confirmed, low. Quote verbatim at spec/definitions.md:225. IMP-003 is family-agnostic (spec/conventions/implicit.yaml:52-53), CTR-002 allows the mark on any Operation (spec/promises/contracts.yaml:42), and no promise rejects it on a Resource Type (search of spec/ for startsModelWork). The package ships one: packages/scenario-s4-effect/contracts.yaml:14-18, with Resource Types at :61-83. Held only at docs/work-orders/143-s4-srv1-effect.md:331-332; no ruling or IN_FORCE line settles it.

**Verifier's corrections.** Class: not an ambiguity that splits builders, since no rule rejects the Type and every engine accepts it. It is a mismatch between the definition ("an Agent's model") and promises that key on the mark alone, and a question of purpose. Decides: owner, as stated.


## Further points the verifier noticed

Not verified by a second reader.

- **REC-001 cannot be kept when the store refuses writes, and SEC-016 already runs into it** (spec/conventions/records.yaml:17-23 and :30-32; spec/promises/secrets.yaml:411-413). REC-001 requires a record when "a request ends without being dispatched" and names one exception only ("Nothing else is required to be recorded"). SEC-016 answers a read whose dispatch record cannot be kept with "It is not dispatched, and reports unavailable"; that refusal is itself a request that ended without dispatch and an access decision, each owing a record the full store cannot take. So no choice under DL10 keeps every promise. This is one more facet of order 082's known question (docs/AUDIT.md:238-241); the promise drafted in F5 inherits it unless it says which records are not owed while the store cannot be written.
- **A third copy of the unsplit owed-stop sentence** (spec/core/README.md:418-420). "a start that finds an owed stop fails, naming it, and may be retried once that stop is carried out (PRV-019, PRV-021)". F1's least change amends providers.yaml and operations.yaml only; this copy would be left saying failed for every case.

## The verifier's notes

Commit checked: 2eb8f258 (2026-11-12). Every quote in F1 to F19 is verbatim at the cited place, allowing for line wraps; a few cited line numbers are one line off.

How I treated the queued and kept-for-later items: the reviewer was told to verify them and report each still-true one, so a record in the queue or a Record did not by itself earn the verdict known. I gave confirmed where the fault is real and as stated, and weakened where the class, the unsafe mark or the claimed divergence did not survive; each verdict cites the record that already holds the point.

Outcome: 11 confirmed, 8 weakened, none refuted, none known.
- Unsafe marks: F9 stands and is the one high. F6 and F12 lose the mark: F6's racing actors already hold install or register authority, and F12's cause is the operator's own act outside the engine.
- Smaller than stated: F2 (headline already spec pending at IN_FORCE.md:224; only the `local` point is new, and it concerns the staged patch), F3 (stale wording; both `then`s already decide), F11 (owner's answer and ruling 75 D1 already settle the direction), F13 (reading B is not sound), F18 (known 4-17; its example is decided by PRV-009 and OBS-003).
- Least changes that need a repair before use: F1 (third copy in spec/core/README.md; case (a) must rest on PRV-019's "cannot tell"), F5 (records owed while the store is full), F6 (wider than the two races the tracker assigns), F7 (Builder B misdescribed).

Not verified: I did not run `git apply --check` on 049b.patch, keeping to the listed read-only git commands; the hunks' context lines are present at this commit. I did not recount the assessment's item totals (about 75 leftovers, 32 on spec/, 17 still true). I did confirm its file counts: 67 order files numbered 067 to 150, 61 with a Record (071, 076, 078, 080, 144 and 147 have none), plus 042.
