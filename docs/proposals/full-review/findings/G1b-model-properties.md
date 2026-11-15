# G1b-model-properties: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** The models' properties are the promises for a minority of the rows I was given. Reading gates.rs, secrets.rs, records.rs and revisions.rs against the unsafe slice that the safety catalogue quotes for each promise, 6 of the 33 rows G5 counts as holds there rest on a property the search could refute, 3 restate a guard decided in the same atomic step, and 24 rest on a predicate that nothing the search varies can reach: a constant, a value copied from the model's own expectation, or a formula the property recomputes. MC7b is the weakest: none of its 14 properties relates two steps, its state spaces are 26 to 161 states, and its one concurrency finding came from a witness the author wrote, not from a property. MC5b and the connection property of MC5a are real checks and should be kept; five of MC5a's six properties are not. retry.rs, harness.rs and mc2.rs credit no promise. Of the 42 static rows that have a vector, 26 have a step that would fail if the catalogue's slice were broken, 3 partly, and 13 do not; with the 11 rows that have no vector, 24 of the 53 static rows are not shown. The manifest's promise names are unreliable in both directions: SEC-008 is named by four cases and tested by none, while FLEET-004 and PRV-024 are tested by cases that do not name them. MC3-001 and MC3-002 stand unchanged; CON-013 has no evidence of any kind, and ACC-034's own safety slice is in fact held by MC3's onward property. What I would change: grade every row before a rerun and report holds only for searched properties, turn MC7b-002's witness into a property for PKG-032, move the constant rows to vectors or to the provider-contract harness, and give the other 81 holds rows the same reading before any model is run again. None of this needs the owner: it is classification and registration, the coordinator's, logged for the owner to strike.

**What the analyst read.** Commit 2eb8f258, read only; no model, cargo or build was run, so every mark is from reading code and text, not from a checker.

Read in full: docs/guides/full-review.md; the first pass's X03-model-checking file; experiments/012-model-checking/src/gates.rs, secrets.rs, records.rs, revisions.rs, mc2.rs, lib.rs. Read in part: access.rs (lines 200-763: predicates, decisions, properties; not its types at 1-199 nor its tests); retry.rs (header, variants, property list); harness.rs (public surface only); G5.md (lines 1-110, every row credited to gates, secrets, records, revisions and access, 310-427); docs/implementation/HANDOVER.md lines 200-226; docs/implementation/PROVIDERS.md lines 55-63; docs/implementation/SAFETY_CATALOGUE.md header and the rows of the 83 promises in scope; conformance/README.md lines 1-200.

Experiment README (6,652 lines), read: MC2 revisions registration and results (263-368); MC3 registration (571-611), supplemental control (768-785), results and findings (786-885); MC5a registration to results (2430-2620) and conclusions (2707-2737); MC5b registration, accounting, configurations, limits, results, conclusions (2738-3037, 3267-3282); MC7b registration to property text (5610-5770), findings and results (5855-5999), conclusions and admin-review registration (6348-6410), final-verifier results (6550-6652). Not read: any trace listing; MC7b 5770-5855 and 6410-6549; the MC2 operations, MC4, MC6 and MC7a sections.

Specification: full `when` and `then` (observables in part) of PRV-003, 011, 013, 017, 018, 022 to 026, 028, 029; SEC-007, 010 to 018, 020, 023; REV-002, 010, 011, 012, 014; REC-001, 003 to 006; OBS-001, 003, 004, 006; PKG-024, 025, 027, 032; CON-013, ACC-034, OPS-013; and `when`/`then` of the 42 static promises that have a vector. Text compared between ed8043a1 and 2eb8f258 by script for 46 promises.

Conformance: every one of the 494 cases was parsed by script for the steps that cite each static promise; I opened in detail the cases named in item I7 (about 35). For promises with many cases (ACC-033 45, PKG-023 39, OPS-023 25, ACC-053 24, REC-002 23) I relied on the step summaries and opened one or two cases each; a Yes there means at least one step fails, not that every situation of the promise is covered.

Not done: operations.rs, placement.rs, lifecycle.rs and controls.rs were not graded (81 holds rows), nor access.rs's nine holds rows beyond what MC3-001 and MC3-002 needed; the 11 exclusions, the 10 MC9 rows and the 11 static rows without a vector were not re-examined (X03/F8 and F9 cover them); engine tests were searched by promise ID only, which shows naming, not coverage; the first pass's other reader files and IN_FORCE.md were not read beyond the lines cited.


## FR-456

**24 of the 33 holds rows credited to gates.rs, secrets.rs, records.rs and revisions.rs rest on a property the search cannot reach, and should no longer be called holds**

G1b I1 · assessment · verifier: partly · decides: coordinator

Method. For each row I took as the promise's safety clause the unsafe slice that the safety catalogue quotes for it, and read the property G5 credits. Marks: S, the property states the slice and relates events of different steps, so an ordering or a fault that the search explores could break it. G, the property restates a guard: the decision and what the property reads come from one expression in one atomic step; only the antecedent varies. K, constant: in the correct model nothing the search varies reaches the predicate; it fails only where a mutant writes the violating value. L, states less than the slice. G and K are both 'cannot fail'.

Result for the 33 rows G5 counts as holds on these four sources (gates 9, secrets 13, records 8, revisions 3):

| Mark | Rows | Count |
| --- | --- | --- |
| S | PRV-028; PRV-026, SEC-012, SEC-017, SEC-023; REV-012 | 6 |
| G | SEC-010, SEC-016; REV-002 | 3 |
| K | PRV-003, PRV-011, PRV-018, PRV-022, PRV-023, PRV-024, PRV-025, PRV-029; SEC-007, SEC-011, SEC-013, SEC-014, SEC-015, SEC-018, SEC-020; PRV-013, OBS-001, OBS-003, OBS-004, OBS-006, REC-001, REC-004, REC-005; REV-011 | 24 |

The 24 K rows should move from holds to static (a decision shown by a vector or by a provider-contract test). 16 of them are named by no conformance case: PRV-011, PRV-018, PRV-023, PRV-024, PRV-025, PRV-029, SEC-011, SEC-013, SEC-014, SEC-015, SEC-018, SEC-020, PRV-013, OBS-001, OBS-004, OBS-006. For those the tree holds no evidence beyond the model restating the rule.

The 3 G rows should read 'restated at decision time'. Of the 6 S rows, four need a qualifier: PRV-028 (one start; order 081's clause on an existing run unchecked), PRV-026 and SEC-023 (a Conduit open or closed; no filter), REV-012 (dispatch order of changes only; the catalogue's slice, 'at most one is stored', is not a property). SEC-012 and SEC-017 stand as they are.

Effect on G5's count: 114 holds becomes at most 90 before the same reading is given to the 81 holds rows of operations.rs (6), access.rs (9), placement.rs (11), lifecycle.rs (37) and controls.rs (18), which I did not grade.

What is working: MC5a's connection property (proof before sending), MC5b's four cross-step predicates and MC2's serial-change property are real checks, and MC3 searches 3,228 to 150,702 states. What is not: MC7b as a whole and five of MC5a's six properties.

Counterargument: every property of a correct reference model holds by construction, and each mutant shows that its property can fail. Answer: 36 of the 50 mutant runs of MC5a, MC5b and MC7b leave the unique-state count of their configuration unchanged. They change a value that is written, not what can happen next, so the exhaustive search adds nothing that a one-line unit test would not. G5 itself says holds 'means only the bounded projection'; it does not say that for 24 rows the projection is a constant.

**Evidence.** experiments/012-model-checking/G5.md:33 '| holds | 114 | 67 |'; G5.md:24-25 '“Holds” means only the bounded projection in its source accounting'. Rows by source counted by script over G5.md's account table: gates 9 holds, secrets 13, records 8 of 14, revisions 3. Marks: items I3 to I6. Mutant state counts recomputed by script from experiments/012-model-checking/README.md:2604-2619 (MC5a: 7 of 9 equal, e.g. 'Base / Correct | 516' and 'Base / OpenGate | 516'), 3003-3029 (MC5b: 4 of 13), 6585-6603 (MC7b records: 10 of 11), 5928-5936 (observation: 7 of 7), 5942-5966 (installs: 8 of 10). Vector counts by script over conformance/manifest.yaml (494 cases).

**Waits.** G5's count against v0's condition (IN_FORCE.md:267); the first pass's question to the owner on what that condition requires (X03/F11) should be put with these numbers; the scope of the safety-vector orders (I13).

**Verifier.** partly. Recounted by script over experiments/012-model-checking/G5.md's account table: 217 rows; holds by source gates 9, secrets 13, records 8 (of 14), revisions 3 (33), and operations 6, access 9, placement 11, lifecycle 37, controls 18 (81). The 6/3/24 split adds up and matches I3 to I6. Of the 24 rows marked K, 16 are named by no case in conformance/manifest.yaml (494 cases), as stated. Mutant state counts recounted: MC5a 7 of 9 equal (README.md:2604-2619), MC5b 4 of 13 (:3003-3029), records 10 of 11 (:6585-6603), observation 7 of 7 (:5928-5936), installs 8 of 10 (:5942-5966), so 36 of 50. The code confirms the predicates that cannot fail: gates.rs:341-342 builds the run from expected_run; gates.rs:667 sends every Call with proof `true`; records.rs:578 sorts before R5; records.rs:582-590 computes the gaps that R6 recomputes; secrets.rs:725 makes `dispatched` a constant. G5.md:24-25 does say that holds means only the bounded projection.

**Verifier's counterargument.** The direction holds: most of the 33 rows rest on predicates that cannot fail. Four parts of the argument do not. (a) Equal state counts do not mean that a mutant changed only a written value. PurgeIndex/Base (63) and ForgetIndex/Crash (121) keep their counts, yet they change what the repeat does: the effect is applied again and the repeat emits records of kinds 2 and 3 instead of kind 1 (records.rs:413-445). The module's own test checks exactly this without any search (records.rs:743-772). An equal count shows only that the refutation needs no interleaving. (b) 'Static' describes a promise that is decided in one step, not a weak model. REC-001 (crash atomicity of a record), REC-005 (order at a source) and PRV-025/PRV-011 (provider contracts) are not one-step decisions. I10 itself keeps REC-001 in the model. A standing such as 'modelled vacuously', with a destination, describes these rows better. (c) The line between S and G is a judgment. B, credited to PRV-028, restates the Send guard (gates.rs:576-579 against :711-717). The takeover case is excluded by current(), which reads M's live generation (gates.rs:291-296). REV-012 is marked S here and 'states less' in I6.

**Verifier's corrections.** 'For those the tree holds no evidence beyond the model' is false for at least three of the 16 rows. PRV-023: engine/crates/shell/tests/runner.rs:864-878 asserts that GITHUB_TOKEN is absent from a run's environment. PRV-029: engine/crates/shell/src/tests.rs:81-83 (`private_state_directory_is_empty_distinct_and_stable_after_restart`) and engine/crates/shell/tests/changes.rs:364. PRV-024: the runner check at engine/crates/conformance/src/records.rs:72-75, plus setup/review-provider-records, as I8 itself says. If REV-012 counts as L, consistent with I6, at most 5 rows are S. If the G and L rows also leave 'holds', the count falls from 114 to at most 86, not 90.


## FR-457

**What the rerun orders must change: register each row against its slice, make findings properties, and stop checking values the model copied from its own expectation**

G1b I2 · assessment · verifier: partly · decides: coordinator

The queue holds an MC2 rerun, MC4's re-registration and 'the models of orders 109, 110, 112 and 113 reviewed against order 081's promises' (docs/implementation/PROVIDERS.md:59-61). For 109 (MC5a) and 113 (MC7b) a review or rerun that keeps the present properties will report holds again for the reason in I1, and will have tested nothing that orders 081 and 082 wrote. Six changes, in order of weight:

1. Register each row against the catalogue's unsafe slice, quoted, with its mark (S, G or K) fixed before the run. Only an S row may come back as holds.
2. A concurrency finding must be a property. MC7b-002 was reached by a witness; the registration says 'no extra aggregate "one first digest wins" safety promise is invented'. PKG-032 now states that clause, so it must be a safety property that the present model refutes (I10).
3. No property may read a value the model took from its own expectation. Three do: gates.rs builds the delivered run from `expected_run` and recomputes only the variables; records.rs sorts the read before checking that it is sorted, and computes the gaps with the formula the property repeats.
4. Each clause that orders 077, 081 and 082 added needs a property and a mutant that changes transitions: PKG-032's one-at-a-time rule; 'Where the call starts no run under PRV-019' in PRV-011, PRV-022 and PRV-028; REC-003's engine acts (an interrupted write, a rollback); REC-006's 'the engine can tell'. None is in any model today.
5. Report, per mutant, whether its unique-state count differs from the correct model's in the same configuration. A mutant that leaves it equal changed a value, not a behavior, and does not make its row searched.
6. A row that stays K leaves the model: it goes to a vector, or, for what a runtime provider or its gate must do, to the suites of experiment 022.

The specific registrations are I10 (MC7b), I11 (MC5a and MC5b) and I12 (MC2 revisions and MC3). I14 gives the rule as a change to the approach.

**Evidence.** docs/implementation/PROVIDERS.md:59-61; experiments/012-model-checking/README.md:5864-5867 '`different-firsts` searches for that witness; no extra aggregate "one first digest wins" safety promise is invented'; spec/promises/packages.yaml:463-465 'Of requests that name different digests, at most one installs its package, and nothing is put in the place of an installed package'; experiments/012-model-checking/src/gates.rs:341-342 'fn deliver_run(instance: u8, setup: u8) -> RunLog { let mut run = expected_run(instance, setup);'; src/records.rs:577-578 'records.sort_by_key(|r| r.id);' against :663-667; :582-590 against :668-678. Text changed since the experiment's last commit (script diff of ed8043a1 against 2eb8f258): PRV-011, PRV-022, PRV-028 `then`; REC-003 `then`; REC-006 `when`; PKG-024, PKG-025, PKG-027 observable; PKG-032 new.

**Waits.** The MC2 rerun, MC4's re-registration and the review of orders 109, 110, 112, 113 should not be written before this is settled.

**Verifier.** partly. Point 3 is confirmed: gates.rs:341-342; records.rs:578 against :663-667; records.rs:582-590 against :668-678. Point 2 is confirmed: README.md:5864-5867 ('no extra aggregate ... safety promise is invented'), and spec/promises/packages.yaml:463-465 now states that clause. The clauses in point 4 exist: spec/promises/providers.yaml:355-356, :684-686, :944-946; spec/conventions/records.yaml:103-106. The queue is at docs/implementation/PROVIDERS.md:59-61.

**Verifier's counterargument.** Point 5 is not a sound test, as I1 shows: equal counts occur for mutants that do change behavior (PurgeIndex, ForgetIndex). A better test is whether the mutant's violation is reached on every schedule. Run each mutant with every interleaving fault switched off: a property refuted on that single schedule is a check made in one step, whatever the state count. Point 1 should register the expected mark and then verify it once the model exists. A mark fixed on paper before the model changes can be wrong.

**Verifier's corrections.** The list of changed text is incomplete. A diff of ed8043a1 against HEAD also shows REC-003's and REC-006's `observable` changed, and PRV-019's `then` and `observable` changed. PRV-019 bears directly on the 'second run.v1' additions in I11.


## FR-458

**records.rs (MC7b): none of its 14 properties relates two steps; all 14 rows cannot fail in the correct model, and four also state less than the slice**

G1b I3 · fact · verifier: supported · decides: none

Lines are of experiments/012-model-checking/src/records.rs. Standing is G5's. Slice is the safety catalogue's quotation.

| Row (standing) | Slice | Property | Mark | Why |
| --- | --- | --- | --- | --- |
| REC-001 (holds) | 'states that the request was permitted, or that it was not dispatched and the reason' | R1, 604-610 | cannot fail; states less | An occurrence and its record are written in one step (437-445, 508-511, 325-333); no crash lies between. R1 asks only that a record exists: `Record` has id, time, kind, purge (160-165) and no verdict. |
| REC-003 (ambiguity MC7b-001) | 'The engine removes or alters it other than by a purge' | R3, 611-625 | cannot fail | No correct transition alters a record; crash erases nothing (478-482). Order 082's engine acts (interrupted write, compaction, rollback) are absent. |
| REC-004 (holds) | 'no effect is applied a second time' | R4, 626-662 | cannot fail | The saved key is written once (418-424) and cleared only by the mutants PurgeIndex (309-311) and ForgetIndex (479-481). The purge's postconditions are computed in the purge's own step (295-347). |
| REC-005 (holds) | 'in the order in which they were made at that source' | R5, 663-667 | cannot fail | The reader sorts by id (577-578) before R5 checks that ids ascend. ReverseRead reverses after the sort (579-581). |
| REC-006 (ambiguity MC7b-001) | 'It is never presented as complete' | R6, 668-678 | cannot fail | `gaps` is computed by the expression that R6 recomputes (582-590). The source tells the reader what should exist: `expected` rides in every frame (357-372). |
| OBS-001 (holds) | 'declared definition, configuration or revision changes because of it' | O1, 1122-1126 | cannot fail | `declared` is written only by AdoptObserved (998-1000). |
| OBS-003 (holds) | 'The earlier value is not shown as current' | O3, 1127-1140 | cannot fail | One linear script; the expected conditions are literals; crash does nothing in the correct model (1070-1074). 26 states, depth 12. |
| OBS-004 (holds) | 'No setup, control, open or retire is dispatched because of it' | O4, 1141-1145 | cannot fail | `dispatches` is filled only by AutoRepair (1001-1003). |
| OBS-006 (holds) | 'An effect occurs, or the actor that inspects it gains access to anything' | O6, 1146-1152 | cannot fail | Effects and access change only under InspectEffect and InspectAccess (1021-1028); lifecycle changes in no variant. |
| PKG-024 (gap MC7b-002) | 'the packages that are installed are as they were' | I24, 1718-1723 | cannot fail; states less | Before and after are read inside the failing step (1517-1553), not against the request's start. |
| PKG-025 (gap MC7b-002) | 'nothing is fetched for an address and version that is already installed' | I25, 1724-1733 | cannot fail; states less | Decision time only (1478-1513). 'Nothing is put in its place' is no property, and the correct model breaks it: the second completion overwrites the slot (1542). It is reached only as the witness 'I different-firsts' (1829-1843). |
| PKG-027 (gap MC7b-002) | same | I27, 1734-1750 | cannot fail; states less | As I25. |
| PRV-013 (holds) | 'each is owned by `system`' | I13, 1751-1766 | cannot fail | Three registrations with fixed candidates (1592-1598), each decided in one step (1603-1617); nothing retires; the machine-provider half is excluded. |
| PRV-017 (gap MC7b-002) | 'Either way the archive is checked against the digest under PKG-026' | I17, 1767-1787 | cannot fail | Check and Use are pushed in one step (1531-1543); `f.request == f.given` compares a value with itself (1464-1466). |

Sizes: records 63 to 161 unique states, observation 26 and 51, installs 55 to 142. 25 of the 28 mutant runs keep the state count of their configuration. No partition, duplication or permanent loss in any part.

**Evidence.** experiments/012-model-checking/src/records.rs at the lines in the table; e.g. :1542 's.nodes[E].data.installed[r.address as usize] = Some(r.digest);', :1464-1466 'request: r, given: r,'. README.md:5905-5966 (state counts, e.g. :5907 'Correct/Base | 63 | 24', :5928 'Correct/Base | 26 | 12', :5942 'Correct/Provider | 55 | 23'); README.md:5623 'Crash/recovery pauses E and erases no saved state'; README.md:5669-5671 'compares immediately before/after that failure, not against a request-start snapshot'. G5.md:106-108, 148, 151, 298-308. Slices: docs/implementation/SAFETY_CATALOGUE.md:170-173, 316, 320, 528-532, 540-545. spec/promises/packages.yaml:308 'nothing is put in its place'; spec/conventions/records.yaml:103-106.

**Waits.** I10.

**Verifier.** supported. Reread records.rs at the cited lines. Each event and its keep happen in one step (records.rs:437-445, :508-511). Crash clears only `key`, and only under ForgetIndex (:478-481). R5 checks an order the reader has already sorted (:578, :663-667). R6 recomputes the gaps by the same expression (:582-590, :668-678). Observation fields are written only by mutants (:998-1003, :1021-1028). Decisions are made at decision time only (:1478-1514). A completion overwrites the slot (:1542). I17 compares `request: r, given: r` (:1464-1466). The sizes and the 25 of 28 equal counts match README.md:5905-5966 and :6585-6603. Faults: crash, recovery, loss and reorder (and timer for installs); no partition, duplication or permanent loss.

**Verifier's corrections.** On R1, 'no verdict' overstates. `kind` separates a dispatch (2) from a request that ended without dispatch (1) (records.rs:437-445). What is missing is the reason, so 'states less' still holds. Add that the installs crash is a no-op (records.rs:1644) and pending decisions are saved, so the installs Crash part exercises nothing that a crash would erase.


## FR-459

**gates.rs (MC5a): one of its nine rows has a searched property (PRV-028); the other eight cannot fail in the correct model**

G1b I4 · fact · verifier: partly · decides: none

Lines are of experiments/012-model-checking/src/gates.rs. All nine rows stand as holds in G5.

| Row | Slice | Property | Mark | Why |
| --- | --- | --- | --- | --- |
| PRV-003 | 'delivers them as PRV-011 and PRV-022 have a runtime provider deliver them' | R, 752-760 | cannot fail | R compares each run with `expected_run`. The delivered run is built from `expected_run` itself (341-342); only its variables are recomputed, from a fixed table (345-362). Files, state directory, mode and emptiness are compared with themselves. 'On no other' machine is excluded. |
| PRV-011 | 'only over a channel that authenticates that machine and keeps the value confidential' | C, 761-769; R | cannot fail | Both flags are `variant != Unauthenticated` (406, 416): true in the correct model. Order 081's clause for a call that starts no run is absent: one remote start. |
| PRV-018 | 'Only what the request presents establishes the instance' | A, 721-735 | cannot fail | The instance is attributed if the token flag is set (511-520); I always sends it set (679-680), X never (688). A stolen token and any basis but the address are absent. |
| PRV-022 | 'It puts them nowhere else' | H, 736-751; C; R | cannot fail | Identity appears only at provider, gate and I's run (473-476); X observes only where a flag is false (477-479); the remote identity is always `identity_i()` (407, 471). 081's clause absent. |
| PRV-023 | 'passes no token exported in the shell that started the engine' | R | cannot fail | One evaluation of an allow-list over a fixed environment (11-19, 345-352): a unit test. |
| PRV-024 | 'in a record, an inspection result, an observation, a diagnostic, any result' | H | cannot fail | The six public places are logged once with the class 'Nothing' (386-396). The model emits no record or result content. |
| PRV-025 | 'only where the request carries cryptographic proof that an enforcement point sent it' | G, 698-707 | cannot fail | `reached` is written only as (I, true), under that very condition (498-501), or by the mutant OpenGate (671-672). X's probe is refused by a fixed branch (673-674). No credential is an input: an instance's own credential, a token for another audience and a replayed proof never reach the gate. The silence bound is excluded. |
| PRV-028 | 'before it is sent anything, that it serves the instance' | B, 708-720 | states the clause; states less since order 081 | A send needs proof on this connection (575-584); proof is volatile and cleared on reset and on connect (536-539, 628-630). Loss and reordering of proof and reset, X taking the address, cut and crash all bear on it; SendFirst and StaleProof change the state graph. Missing: the certificate 'when that run started, including where `run.v1` answers with an existing run'. Assumed, not checked: 'an old wire call cannot reach a new process'. |
| PRV-029 | 'passes it to no other run' | R | cannot fail | The directory is `state/L{instance}`, copied from the expectation (298-299, 342). Nothing saved holds the binding, so a restart cannot lose it (550-557). Removal is absent. |

7 of the 9 mutants keep the unique-state count of their configuration (516; 5,110 for OtherState); SendFirst (3,772) and StaleProof (1,112) do not. The README says of itself: 'Authentication and confidentiality are checked flags in this model.'

**Evidence.** experiments/012-model-checking/src/gates.rs at the lines in the table; :406 'let authenticated = self.variant != Variant::Unauthenticated;'; :500-501 'if conn.process == Process::I && proof { s.ledger.reached[id as usize] = Some((Process::I, proof));'. README.md:2604-2619; README.md:2463 'an old wire call cannot reach a new process'; README.md:2719. G5.md:140, 146, 152, 155-158, 161, 162. Slices: docs/implementation/SAFETY_CATALOGUE.md:306, 314, 321, 324-327, 330, 331. spec/promises/providers.yaml:355-356, 684-686, 944-946 (order 081's clauses).

**Waits.** I11.

**Verifier.** partly. The eight K rows are confirmed. Every engine Call carries proof `true` (gates.rs:667), so the gate's admission at :500-501 never sees an unproved engine request. X's probe is refused by a fixed branch (:670-675). The channel flags derive from the variant (:406, :416). Runs are built from expected_run (:341-342). IdentifyI always sends the token, IdentifyX never (:678-689). The six public places are logged once as Nothing (:385-396). 7 of 9 mutants keep 516 or 5,110 states (README.md:2604-2619), and README.md:2719 calls authentication and confidentiality checked flags.

**Verifier's counterargument.** The PRV-028 row overstates what is searched. B's first clause restates the Send guard (gates.rs:576-579 against :711-717). Losing or reordering Reset cannot bear on B, because current() compares the connection's generation with M's live generation (gates.rs:291-296). The stale connection stops being current the instant X takes over, whatever happens to the Reset. The takeover is therefore excluded by an oracle, which is the README's own assumption (README.md:2462-2463). What the search does exercise is proof loss and reordering and per-connection proof matching (gates.rs:486-497).

**Verifier's corrections.** PRV-028: 'restates the send guard, takeover excluded by current()'s oracle; proof loss and reordering searched'. Drop 'reset' from the faults said to bear on it.


## FR-460

**secrets.rs (MC5b): four of 13 rows have a searched property, two restate a guard, seven cannot fail in the correct model**

G1b I5 · fact · verifier: supported · decides: none

Lines are of experiments/012-model-checking/src/secrets.rs. All 13 rows stand as holds in G5.

| Row | Slice | Property | Mark | Why, and what is missing |
| --- | --- | --- | --- | --- |
| PRV-026 | 'the filters on that Conduit as they stand when the read is made' | F, 380-422 | states the clause; states less | P's new run must follow a read of the same epoch through an open Conduit (396-410); calls after a restart must follow P's rerun (411-419). Crash timing and the closing of P's Conduit bear on it. Missing: a filter is only open or closed; one setup of P, so 'the setup that last asked' is fixed. |
| SEC-007 | 'The request is not dispatched' | P, 313-315 | cannot fail | The request is logged at script stages 6 and 7 with the answer refused and `dispatched: onward && Onward` (716-727): false in the correct model, and for I's own request false in every variant. |
| SEC-010 | 'through the instance's own open Conduit to that Resource' | P, 298-312 | cannot fail (same-step guard) | The Conduit is the instance's own except under Borrow (518-522); `dispatched` is computed from `open` in the step the property reads (523-525). An instance with more than one such Conduit is absent. |
| SEC-011 | 'It has that invocation's caller as its caller' | P | cannot fail | Caller and invocation are functions of the read's number (179-184, 531-536). The invocation has no mutant. Filters are absent. |
| SEC-012 | 'The engine asks for no run, makes no connection and sends no HTTP request on a connection' | S, 337 | states the clause | Every piece of work must have a succeeded source read; read faults and recovery (901-914) bear on it. Missing: that the request which waited reports unavailable. |
| SEC-013 | 'only to the item's URL' | H, 426 | cannot fail | An HTTP send is logged with URL 0 at the one correct site (703-711); the redirect branch exists only for the mutant (802-822). Missing: 'and no longer' (a later setup, retirement, a lost node). |
| SEC-014 | 'Closing the Conduit ends every later read through it' | F, 385-395 | cannot fail; states less | A further read for I exists only in RefreshI (950-952). I's Conduit never closes (only P's and R's do, 881-892), so the slice's situation never arises. |
| SEC-015 | 'The record holds the value read, or any part of it' | V, 362-370 | cannot fail | A value at a record is logged only by RecordValue (790-792). |
| SEC-016 | 'It is not dispatched' | S, 327-336 | cannot fail (same-step guard) | `dispatched` requires `record_kept` in the same expression (524-525); the unavailable ending is logged in that step, in the mutant too (551-565). |
| SEC-017 | 'each a request with its own access decision and record' | V, 372-375 | states the clause | A value used must come from a read of the same epoch (I's run excepted). A crash clears values and the header (830-838); CachedP and CachedHeader refute it. |
| SEC-018 | 'The result of the read reaches the engine and no instance' | V | cannot fail; states less | Destinations are fixed (604-640, 702). No event stands for a read's result returned to an instance; inspection, observation, diagnostic, result and `_meta` are not places in the model (151-157). |
| SEC-020 | 'The engine passes no token that the user has exported in the engine's environment to any run' | P, 316-321 | cannot fail | A value with no source read is logged only by Environment (623-631); a read answered by the engine only by EngineAnswer (566-577). |
| SEC-023 | 'the filters on that Conduit as they stand when the read is made' | H, 426-430 | states the clause; states less | Every HTTP send must use a succeeded read of the same epoch through R's open Conduit; crash, the closing of R's Conduit and the re-read bear on it. Missing: filters beyond open or closed. |

4 of the 13 mutants keep the state count of their configuration (Borrow, Caller, Environment, RecordValue). No partition in any part.

**Evidence.** experiments/012-model-checking/src/secrets.rs at the lines in the table; :524-525 'let dispatched = (open || (self.variant == Variant::ClosedP && id == 5)) && (record_kept || self.variant == Variant::Unrecorded);'; :725 'dispatched: onward && self.variant == Variant::Onward,'. README.md:3003-3029 (e.g. 'Base / Correct | 57' and 'Base / Borrow | 57'); README.md:2909-2910 'filters with predicates beyond current Conduit openness ... are omitted'; G5.md:337 'MC5b/MC6/MC7a/MC7b omit partitions'. G5.md:159, 174, 176-186. Slices: docs/implementation/SAFETY_CATALOGUE.md:328, 363, 366-374, 376, 379.

**Waits.** I11.

**Verifier.** supported. Confirmed. secrets.rs:524-525: `dispatched` is computed from `open` and `record_kept` in one step, and the unavailable End is logged in that same step (:551-565). Only P's and R's Conduits close (the Close actions in local_actions). The only further read for I is RestartI under RefreshI. `Get.dispatched` is `onward && Onward` (:725). Values with no read appear only under Environment, and engine answers only under EngineAnswer. Crash clears values and the header except under CachedHeader. 4 of 13 mutants keep their count (Borrow 57, Caller 2,111, Environment 57, RecordValue 57; README.md:3003-3029). The only faults are crash, recovery, loss and reorder.


## FR-461

**revisions.rs (MC2 revisions): REV-012's property states less than its slice; REV-002 restates a guard; REV-011 cannot fail. retry.rs, harness.rs and mc2.rs credit no promise**

G1b I6 · fact · verifier: supported · decides: none

Lines are of experiments/012-model-checking/src/revisions.rs. All three rows stand as holds in G5.

| Row | Slice | Property | Mark | Why |
| --- | --- | --- | --- | --- |
| REV-002 | 'The change is not dispatched' | V, 259-282 | cannot fail (same-step guard) | The request, its answer and its dispatch are logged in one step from one expression (158-184). The current revision does vary with the interleaving. |
| REV-011 | 'No change is applied to whatever revision is current' | V | cannot fail | A request that names no revision is answered invalid without reading any state (160-161); the script has one (243-246). It shares one mutant, Unchecked, with REV-002. |
| REV-012 | 'Of several requests to change one object that name the same revision, at most one is stored' | J, 284-303 | states less | J says that no second change is dispatched between a dispatch and its end. An unknown end counts as an end, so a second change is dispatched while the first is still on the wire. No property counts the changes stored per named revision: a model that stored a late reply after an unknown end (the guard at 190-194) would keep V and J true and store two changes that name r0. Setups and retirements are absent. |

This model is searched: its mutants change the state count in every part (for example 1,367 and 1,671 against 426 under timers). Parts: timers, loss, reordering, engine crash; no duplication, cut or permanent loss.

retry.rs is MC1, the harness's own test: 'Harness self-test only: these rules claim no Conduit promise'; its five properties are tagged `harness`. G5 credits no row to it. harness.rs is the fault harness and the checking functions; mc2.rs holds MC2's configuration. Neither has a promise property.

**Evidence.** experiments/012-model-checking/src/revisions.rs:284-303, :190-194 'if s.nodes[1].data.pending.contains(&id) { self.end(s, id, Answer::Succeeded, "reply"); }'. README.md:270 'unknown ends a change for REV-012 even if its dispatch can still arrive'; README.md:275-278; README.md:354-366 (state counts). spec/conventions/revisions.yaml:194-195 'Of several requests to change one object that name the same revision, at most one is stored'. src/retry.rs:1, :158-188; G5.md:325 'No Conduit promise is counted from this model'; `grep -c 'retry::' G5.md` gives 0. G5.md:286, 293, 294. Slices: docs/implementation/SAFETY_CATALOGUE.md:509, 516, 517.

**Waits.** I12.

**Verifier.** supported. revisions.rs:157-184 decides a change, logs it and dispatches it from one expression. :160-161 answers invalid without reading state. The guard at :190-194 ignores a late reply. J (:284-303) removes a change from flight at any End, including unknown. The mutant counts differ in every part (README.md:354-366). README.md:270 says 'unknown ends a change for REV-012 even if its dispatch can still arrive'. retry.rs:1 says 'Harness self-test only' and tags its properties `harness` (:161-188). G5.md:325 counts no promise from it, and `grep -c 'retry::'` on G5.md gives 0. mc2.rs and harness.rs claim no promise.

**Verifier's counterargument.** The REV-012 analysis is right, but it stops at a hypothetical mutant. By reading, with no checker run, the correct model already applies two changes that name r0. In the Timers part: Change(0) is dispatched; a timer ends it unknown (revisions.rs:200-204); REV-004 leaves the revision at r0, so Change(1) passes V and is dispatched; both Dispatch messages produce Effect events (:186-187). V and J both hold throughout. See the missed item.


## FR-462

**Static rows with a vector: of 42, 26 have a step that would fail if the catalogue's slice were broken, 3 partly, 13 not**

G1b I7 · fact · verifier: supported · decides: none

For each static row of G5 that a manifest case names, I took the catalogue's slice and looked for a step whose expectation fails when it is broken. 14 of the 42 are cited by no step of the cases that name them. Case paths are under conformance/.

| Row | Slice (short) | Vector opened | Would it fail? |
| --- | --- | --- | --- |
| TPL-011 | a failing value 'is not dispatched' | interface/schemas/e05-control | Partly. Pins the published schema and that 75 is invalid; sends no request. |
| TPL-013 | 'The request is not dispatched' | interface/schemas/e05-control-absent-template, -absent-type | Partly. The tool is withheld with reason invalid; no request step. |
| NAM-023 | 'outputs fail that version has the outcome unknown' | interface/schemas/e05-target-version | No. No result is processed. |
| NAM-026 | 'not dispatched' | conventions/access/capability-skew, step 2 | Yes. |
| PKG-023 | 'not dispatched' | language/archives/traversal and 37 others | Yes for a rejected archive (29 steps). No step for a `fetch` that is relative, holds user information, is absent or has an unaccepted scheme. |
| PKG-029 | project 'keep that address' | language/archives/template-contract-holder | Yes. |
| SUB-009 | 'replaces one template and registers nothing' | conventions/records/replace, steps 2, 5, 7 | No. The successful replace pins one object's facts, not the object count. |
| POOL-015 | 'by that instance's own open Conduit to the Pool' | conventions/setup/pool-entry | No. Sender and caller are both root. |
| FLEET-004 | ownership permits no request | conventions/fleets/consumer; fleets/e3b/manages-agent-own-fleet | No. The case that does refuse on ownership, fleets/e3b/ownership-permits-nothing, names ACC-033 and ACC-001. |
| FST-002 | 'with root as consumer' | conventions/first-start/initial | Yes: exact array of 8 objects. |
| FST-003 | no other implementation called | same | Yes: 8 objects, no intents. |
| FST-007 | 'A generated domain is never a shared fixed name' | conventions/first-start/trust-domain | No for the slice: the runner supplies the domain. Yes for 'never changes'. |
| PRV-005 | 'that request's caller' | conventions/setup/review-provider-records | Partly. Caller pinned; sender and caller are one instance. |
| SEC-008 | 'only where the root Interface sent it on its own behalf' | language/registration/e1/root-consumer-open, -onward, -register, -register-onward-invalid | No. Every step cites ACC-054; no Conduit in them lists `get-secret.v1`. |
| LIF-011 | 'They never change the lifecycle state' | conventions/setup/review-conditions-builtin, -provider | No. Facts hold id and status.conditions only. |
| LIF-012 | template 'returns no lifecycle state' | conventions/records/inspection, step 5 | Yes: status asserted absent. |
| LIF-013 | pending-action disclosure and evidence withheld | conventions/records/inspection and 12 others | No. No case has a pending action. |
| LIF-034 | 'not dispatched' | conventions/first-start/lifecycle; access/root-local-lost | Yes: 17 refusals, no intents. |
| LIF-037 | 'compared only after the access decision' | language/registration/e1/expected-id-access-order, step 1; records/expected-id, step 3 | Yes. |
| ACC-003 | 'exactly the operations it lists' | conventions/access/s1-exact-access, step 14 | Yes. |
| ACC-006 | 'whose bound covers the new Conduit's target' | conventions/access/bounds, steps 3, 4, 8, 9 | Yes (they cite ACC-029). |
| ACC-009 | onward request, carried filters | conventions/access/s1-exact-access | No. Every request is direct; every `filters` is empty. |
| ACC-012 | bound covers the Conduit to be closed | conventions/access/bounds, steps 17, 18 | No. One close inside the bound; the refused one is refused under ACC-024. |
| ACC-024 | 'not dispatched' | same, step 18 | Yes. |
| ACC-029 | 'covers no later object under that name' | conventions/inventory/e4/bound-kept-id, step 11 | Yes. |
| ACC-030 | 'only as it is or narrower' | conventions/access/bounds, steps 13-16 | Yes. |
| ACC-032 | 'only if the actor is the root Interface' | conventions/records/purge-nonroot | Yes. |
| ACC-033 | authority by the first fitting row | conventions/first-start/lifecycle and 15 others | Yes: 53 steps cite it. |
| ACC-053 | 'checked against the kept IDs' | conventions/inventory/e4/bound-kept-id, step 11 | Yes. |
| ACC-054 | 'only where root sent it on its own behalf' | language/registration/e1/root-consumer-open | Yes: 17 steps. |
| ACC-055 | 'sent by that consumer' | conventions/inventory/e4/reconcile-sender-other, step 9 | Yes. |
| ACC-057 | Agents registered only by root | conventions/fleets/e3a/register-agent-non-root | Yes. |
| OPS-006 | 'The result states its outcome' | conventions/requests/aware-failed, -unknown, -malformed, step 3 | Yes. |
| OPS-019 | access applies to management verbs | conventions/s1/whole | No by its named case: its three refusals are file operations. |
| OPS-023 | 'nothing about any other reason' | conventions/access/s1-exact-access; requests/reason-order | Yes: details asserted absent. |
| OPS-026 | 'The value so filled is what is sent' | conventions/requests/defaults, step 2 | Yes. |
| OPS-029 | a Pool's authority learns nothing of other names | conventions/records/expected-id, steps 6, 8 against 7, 9 | Yes. |
| OPS-033 | 'the result holds no part of it' | conventions/requests/reason-order, step 4 | No. The invalid input is `path: 4`, not a secret; the diagnostic is a projection that an echoed value would pass. |
| OPS-036 | 'only items that the actor may inspect' | conventions/list/management | Yes. |
| REV-001 | 'compared only for equality' | conventions/s1/whole | No. Case-level name only. |
| OBS-002 | 'the time it was made and its source' | conventions/setup/review-conditions-builtin | Yes. |
| REC-002 | 'no part of its `identity`' | conventions/setup/review-provider-records, steps 3, 4 | Yes: the only two steps in the suite that assert it. |

With the 11 static rows that no case names (X03/F9), 24 of the 53 static rows are not shown for their slice, 3 partly, 26 shown. A Yes means at least one step fails; for the rows with many cases I opened one or two.

**Evidence.** Script (/usr/bin/python3) over conformance/manifest.yaml (494 cases) and every case file: steps whose `expect.required_promises` holds the ID, with their reply, `external_intents`, `stateUnchanged` and `absent`. Zero citing steps in their named cases: TPL-011, NAM-023, POOL-015, FLEET-004, PRV-005, SEC-008, LIF-011, ACC-006, ACC-009, ACC-012, OPS-006, OPS-019, REV-001, OBS-002. `grep -rci secret` over the four root-consumer case directories gives 0 in each. conformance/README.md:71-78 'Maps are required-field projections ... An empty intent list rejects every dispatch'; :192-193 'The conformance suite claims these schema checks, not the later transitions that call them'. conformance/conventions/access/bounds/observations.json, observations 15 and 16: the same close, the Conduit's consumer ...109 then ...005. conformance/conventions/setup/pool-entry/case.yaml step 5: caller and sender both 00000000-0000-4000-8000-000000000005. Slices: docs/implementation/SAFETY_CATALOGUE.md:94-545 (rows of the 42).

**Waits.** I8, I13.

**Verifier.** supported. Recounted by script: 53 static rows; 42 named by a case, 11 named by none (the same list as X03/F9). The 14 rows with no citing step in their named cases are exactly as listed. Spot checks agree with the table. SEC-008: no step among the 494 cases cites it, and no case directory contains 'secret'. POOL-015: step 5 of conventions/setup/pool-entry has caller = sender = ...005. ACC-012: in access/bounds, step 18 is refused under ACC-024. OPS-033: reason-order step 4 pins only the diagnostic, while the catalogue's slice is the secret literal (SAFETY_CATALOGUE.md:498). FST-007: the input supplies trustDomain. LIF-013: no pending-action term anywhere under conformance/. PKG-023: no install request carries a `fetch`. OBS-002: review-conditions-builtin asserts lastTransitionTime and source.

**Verifier's counterargument.** The Yes verdicts for rows with many cases rest on one or two cases each, so 26/3/24 is an estimate, as the medium confidence says.

**Verifier's corrections.** SEC-008 detail: two steps in its cases do not cite ACC-054. Step 2 of root-consumer-open cites nothing, and step 4 of root-consumer-register cites OBJ-013 and IFC-012.


## FR-463

**The manifest's promise names do not say what a vector tests: SEC-008 is named four times and tested nowhere; the safety field must rest on steps that fail**

G1b I8 · assessment · verifier: supported · decides: coordinator

Three things follow from I7.

1. Names overstate. SEC-008 guards who may open a Conduit that reads secrets. Four cases name it; each of their steps cites ACC-054; none contains a Conduit that lists `get-secret.v1`; no step in the 494 cases cites SEC-008. The engine has the branch (it returns refused, SEC-008), so the code path is built and unexercised by any vector. The same holds, less sharply, for ACC-012 (no close outside a bound), ACC-009 (no onward request, no filter), POOL-015 (sender and caller are one instance), LIF-011 (lifecycle state not asserted) and OPS-033 (no secret literal).

2. Names understate. FLEET-004's clause is tested by fleets/e3b/ownership-permits-nothing, which names ACC-033 and ACC-001. PRV-024's clause (no identity in the record of `run.v1`) is asserted by setup/review-provider-records, and no case names PRV-024.

3. So counting 'static rows named by a case' (42 of 53) measures the wrong thing, and SV2, which tags existing vectors, would carry the error into the safety field that v0's condition requires. The condition defines a safety vector as 'one whose failure would be an unsafe finding'. The runner already has the right unit: a step's `required_promises`, with an expectation that pins a refusal, an empty intent list or an absent pointer.

Which static rows to write first, by harm: SEC-008 (a secret-reading Conduit opened by someone other than root), then ACC-012, POOL-015, ACC-009 (access), OPS-033 (a secret echoed in a result), LIF-013 (pending-action disclosure), then NAM-023, SUB-009, LIF-011, OPS-019, REV-001, FST-007, FLEET-004 (rename only). TPL-011 and TPL-013 need one request step each; scenario S5's E10 plans it.

Existing paradigm: a conformance suite credits a requirement only through a test that fails without it, as the OpenID Connect certification suite's negative tests and the Kubernetes conformance suite's per-test requirement tags do.

**Evidence.** engine/crates/core/src/access.rs:824-832 'if operation == context.qualified("get-secret.v1") && (instance["id"] != root["id"] ... return Ok(Verdict::reason("refused", "SEC-008"));'; script count of steps citing SEC-008 across all 494 cases: 0; conformance/manifest.yaml:4376, 4385, 4395, 4418 (the four cases that name SEC-008). conformance/manifest.yaml entry 'fleets-ownership-permits-nothing': promises ['ACC-033', 'ACC-001']. conformance/conventions/setup/review-provider-records/case.yaml steps 3 and 4: `absent` ['/records/N/data/inputs/identity', '/records/N/data/inputs/secrets']; manifest cases naming PRV-024: 0. IN_FORCE.md:265 'A safety vector is one whose failure would be an unsafe finding as the tracker defines it; each carries a field naming the safety promise it tests'. docs/work-orders/117-plan-s2-s3.md:134-136 (SV1 to SV4). ROADMAP.md:176 (S5's E10).

**Waits.** SV2's tagging rule and SV3's scope (I13); v0's condition at IN_FORCE.md:265.

**Verifier.** supported. The SEC-008 branch at engine/crates/core/src/access.rs:824-832 is reached by no vector. The only other case with a get-secret Conduit, conventions/access/collision-secret, is an IFC-014 invalid registration. The manifest names SEC-008 at lines 4376, 4385, 4395 and 4418. fleets/e3b/ownership-permits-nothing names ['ACC-033','ACC-001']. No case names PRV-024. IN_FORCE.md:265 defines a safety vector as quoted.

**Verifier's counterargument.** The case offered for FLEET-004, ownership-permits-nothing, pins only `reply.reason: refused` in steps 2 and 3. It has no `external_intents` and no `stateUnchanged`, so it would not count under the analyst's own rule in I13.

**Verifier's corrections.** 'Each of their steps cites ACC-054' should read 'every citing step cites ACC-054'. CON-013 is a third case where the names understate (see I9).


## FR-464

**MC3-001 and MC3-002 still stand at this commit, unchanged; CON-013 has no evidence of any kind, and ACC-034's own slice is held by the onward property**

G1b I9 · fact · verifier: partly · decides: none

Both findings stand exactly as recorded. Neither the model nor the text they rest on has changed: access.rs was last changed by MC3's own commit (511d606c, 2026-11-11), and the `when`, `then` and observable of CON-013, ACC-034, OPS-013, OPS-032 and IMP-007 are identical at the experiment's last commit (ed8043a1) and at 2eb8f258. OPS-009's `when` gained 'is not implemented by a plain server' in order 077; MC3's operation is not plain, so the trace is untouched. Orders 077, 081 and 082 did not address either finding, and neither was a finding against a promise.

MC3-001. The property P is 'every request through an open, readable Conduit to a ready target is dispatched'. It is refuted on the correct model with one wire copy, because a repeat in progress answers conflict. It holds in the other 10 correct parts. Two things the record does not say: P is a permission, not a safety clause; and the catalogue's slice for CON-013 is 'without setting up the consumer again', which no property can state, because the model has no setup of the consumer at all. A qualified P would still not be CON-013's safety clause. CON-013 is named by no conformance case. It is therefore neither 'held by a named model' nor 'listed with its reason'.

MC3-002. The literal property F ('every reply delivers the outcome') is refuted in three correct parts (timers, engine crash, target crash), where the invocation had already ended unknown. The control Q holds in all 11 correct parts. The catalogue's slice for ACC-034 is 'An onward request made after the close', and that is the property O, which relates the close to a later onward decision, holds in all 11 correct parts and is refuted by five mutants. So ACC-034's safety slice is held by a searched property; the standing 'property over-broad' describes F, which concerns delivery, not the slice.

**Evidence.** `git log -2 -- experiments/012-model-checking/src/access.rs`: 511d606c 2026-11-11. Script diff of promise fields between ed8043a1 and HEAD: CON-013, ACC-034, OPS-013, OPS-032, IMP-007 unchanged; OPS-009 `when` changed. experiments/012-model-checking/src/access.rs:646-652 '.all(|d| !d.open || !d.readable || !d.ready || d.dispatched)'; :447-448 'Some(Outcome::Progress) => Some(Answer::Conflict)'; :200-204 (F); :660-672 (Q); :675-688 (O). README.md:817-827 (results: Copies / Correct 'H X H H H H H'; Timers, EngineCrash, TargetCrash 'H H X H H H H'). G5.md:131 'There is no separate corrected-P control'; G5.md:244. spec/promises/conduits.yaml:176-178; docs/implementation/SAFETY_CATALOGUE.md:273 (CON-013: 'without setting up the consumer again'), :453 (ACC-034: 'An onward request made after the close'). Manifest cases naming CON-013: 0; ACC-034: 2.

**Waits.** I12.

**Verifier.** partly. Both findings stand. access.rs last changed in 511d606c (2026-11-11); the experiment's last commit is ed8043a1. A diff shows the `when`, `then` and `observable` of CON-013, ACC-034, OPS-013, OPS-032 and IMP-007 unchanged. In README.md:817-834, P fails only in Copies/Correct, F only in Timers, EngineCrash and TargetCrash, and O and Q hold in all 11 correct parts. O is refuted by Borrow, Chain, DropFilter, Caller and Ended. The catalogue slices are at SAFETY_CATALOGUE.md:273 and :453.

**Verifier's counterargument.** (a) 'CON-013 has no evidence of any kind' is wrong. Across the manifest, 39 steps call an access.open tool with `external_intents: []`; 27 of them are root opening root-workspace, e.g. conventions/setup/base/case.yaml step 2. That shows CON-013's slice, 'without setting up the consumer again', for a ready consumer, though no step names it and none asserts the consumer's status. (b) The claim that ACC-034's slice is held by a searched property conflicts with the analyst's own rubric. O's `closed` is computed in the decision step and stored with it (access.rs:487-503, :519-520). That is the same-step guard pattern marked G for SEC-010. G5.md:244 already says that O and Q hold.

**Verifier's corrections.** OPS-009's `observable` also changed, not only its `when`; the trace is unaffected. ACC-034 should read 'held at decision time by O (G); delivery by Q'.


## FR-465

**MC7b's rerun: the install race as a safety property under PKG-032, records with a write that a crash can interrupt, and the observation rows dropped**

G1b I10 · experiment-change · verifier: partly · decides: coordinator

Register row: ROADMAP.md:75 (experiment 012, A40 and G5). It replaces 'MC7b's rerun with PKG-032' in order 082's Record and the review of order 113 in the queue. Fixed before it runs:

Claim 1 (PKG-032). Of requests to install one address and version that name different digests, at most one installs, and nothing replaces an installed package, under loss and reordering of requests and fetch messages and an engine crash with a fetch pending.
- Model: the correct engine begins to decide a request only when no earlier request for that slot is undecided. The present model, which lets both decide against an empty slot, is kept as the mutant Unserialized.
- Properties: I32a, a slot's digest never changes once set; I32b, at most one request per slot ends succeeded with a dispatch among those naming different digests. The witness 'I different-firsts' stays, expected unreachable in the correct model.
- Pass: I32a and I32b hold in all 15 install parts; Unserialized refutes I32a in the Provider part in 12 actions (the length of today's witness); a new witness, 'the second request is decided after the first has ended', is reached.
- Result that matters: I32a refuted in the Crash part on the serialized model. PKG-032 says that waiting 'leaves no state that outlives the request'; a decision that survives a restart and completes against a slot filled meanwhile would need text.

Claim 2 (REC-001, REC-003 after 082). No occurrence is left without its record by the engine's own crash.
- Model: the occurrence and the record's write become two steps with the crash enabled between them.
- Registered expectation: refuted unless the write comes before the dispatch. This is order 082's open question (what an engine owes when it cannot store state), put as a trace, and is the result that matters.

Claim 3 (REC-005, REC-006). Only if kept in the model: the reader must not sort, and must not be told what should exist. Add a part in which the source cannot tell a loss, with R6 registered as not applicable there, so that the stated limit is visible. Otherwise both rows are relabelled static.

Dropped from the model and relabelled static: OBS-001, OBS-003, OBS-004, OBS-006, PRV-013, PRV-017, REC-004.

Measures, per configuration: unique states, depth, each property's mark (S, G, K), and for each mutant whether its state count differs from the correct model's.

**Evidence.** ROADMAP.md:75; docs/implementation/PROVIDERS.md:59-61; X03/F7 (082's Record lists 'MC7b's rerun with PKG-032'). spec/promises/packages.yaml:450-452, :463-465, :468-469 'waiting leaves no state that outlives the request'. experiments/012-model-checking/src/records.rs:1478-1514 (decide), :1542 (overwrite), :1829-1843 (witness); README.md:5975-5977 'The first BFS witness has 12 actions'. spec/conventions/records.yaml:103-106, :166-167. docs/guides/full-review.md:138 'Order 082's questions: state that cannot be stored; races across addresses'.

**Waits.** The admin's next spec order (HANDOVER item 7) should wait for claim 2's trace; S7 (an outside package) relies on the install rule.

**Verifier.** partly. The basis is confirmed. spec/promises/packages.yaml:450-452, :463-465 and :468-469 ('waiting leaves no state that outlives the request'). records.rs:1478-1514 decides against an empty slot; :1542 overwrites; the witness is at :1829-1843. README.md:5975-5977 gives the 12-action witness. Order 082's Record lists the rerun (docs/work-orders/082-spec-records-and-installs.md:742). The coordinator decides, and no ruling is contradicted.

**Verifier's counterargument.** (1) The installs crash is a no-op and pending decisions are saved (records.rs:1644). Claim 1's Crash result can arise only if the serialization wait is modelled as volatile state that a crash clears while pending fetches survive; register that explicitly. (2) '12 actions' as a pass condition is too precise: register the refutation and report its length. (3) Dropping REC-004 discards the one race the model could host: a purge while an invocation is in progress, before its key is saved. Today's script purges only after the invocation has ended (records.rs:519-560). Extend the script instead. (4) HANDOVER item 7 holds five unrelated spec fixes. Holding all of them for claim 2's trace delays them; write item 7 now and let the trace inform one line later.


## FR-466

**MC5a and MC5b reruns: keep the searched properties, add order 081's second `run.v1`, real filters and the closing of I's Conduit; move the constant rows to experiment 022 and to vectors**

G1b I11 · experiment-change · verifier: partly · decides: coordinator

Register rows: ROADMAP.md:75 (experiment 012) for the models; ROADMAP.md:76 (experiment 022, A63) for what leaves them. It replaces 'the models of orders 109, 110 ... reviewed against order 081's promises'. Fixed before either runs:

MC5a (gates.rs). Keep only the connection property B and what bears on it.
- Claim: the engine sends nothing to what answers at a run's `url` until this connection has proved the instance's certificate, across a second `run.v1` that an existing run answers (PRV-019) and across a later run of the same ID.
- Additions: a second `run.v1` answered by the existing run, with a holder-set property 'the second call's `secrets` and `identity` are held by nothing' (the clause 081 added to PRV-011 and PRV-022); the assumption 'an old wire call cannot reach a new process' lifted in one part, so that a dispatch sent before a run ended can be delivered to a later run of the ID (the trace of X03/F1).
- Pass: B holds in all parts; the holder-set property holds; in the part without the assumption the registered expectation is that a dispatch reaches the later run, which confirms X03/F1 by a checker.
- Leaving the model: PRV-003, PRV-011's channel flag, PRV-018, PRV-022's holders, PRV-023, PRV-024, PRV-025, PRV-029. For a runtime provider and its gate they become tests in experiment 022's suites (a request without the engine's proof for this instance reaches nothing; an instance's own credential does not pass; a token for another audience does not pass). For the engine's own runner they become tests of the shell. Existing paradigm: the Container Runtime Interface's validation suite (critest) and the CSI sanity suite test each provider against its contract.

MC5b (secrets.rs). Keep S, V's epoch clause, F and H.
- Additions, each with a mutant that changes transitions: (a) I's Conduit closes after its run starts, then the provider restarts the run; claim: no read is made for the run, and closing recalls nothing (SEC-014); mutant RefreshOnClose. (b) A filter on P's and on R's Conduit changes between the setup and the re-read; claim: the re-read is decided by the filter as it stands then (PRV-026, SEC-023); mutant StaleFilter. (c) A second setup of P by another caller before the crash; claim: the re-read belongs to the setup that last asked (SEC-011, PRV-026); mutant FirstSetupCaller. (d) A second `run.v1` for I answered by the existing run (X03/F6). (e) A cut between the engine and the secret Resource, or the reason recorded why none is needed.
- Pass: the predicates hold in every correct part; each new mutant refutes its named property and changes the state count.
- Leaving the model for vectors: SEC-007, SEC-013's URL clause, SEC-015, SEC-018, SEC-020.

Measures for both: unique states, depth, each row's mark, and each mutant's state count against the correct model's.

**Evidence.** ROADMAP.md:75, :76; docs/implementation/PROVIDERS.md:59-61. spec/promises/providers.yaml:355-356 'Where the call starts no run under PRV-019, its values reach no run and are kept nowhere', :684-686, :944-946. experiments/012-model-checking/README.md:2446-2455 (one remote start), :2463, :2909-2910. src/secrets.rs:881-892 (only P's and R's Conduits close), :179-184. spec/promises/secrets.yaml:373-375 (SEC-014), :565-569 (SEC-023). X03/F1 and X03/F6 for the two traces this would put to a checker.

**Waits.** S3's partition cells and S4's lost-response replay rest on the fence that X03/F1 questions; experiment 022's suites gain the gate tests.

**Verifier.** partly. The basis is confirmed: README.md:2446-2455 (one remote start) and :2462-2463; the clauses at spec/promises/providers.yaml:355-356, :684-686 and :944-946; secrets.rs closes only P and R. Experiment 022 already plans 'trust fixtures, run certificates, a gate and fault controls' for H2 (experiments/022-provider-contracts/README.md:64-65), which supports moving the gate rows there. The runtime suite is empty today (suites/runtime/index.yaml: `cases: []`).

**Verifier's counterargument.** packages/provider-ssh-runtime/README.md:21-22 states 'PRV-025's isolation is unmet until each Agent has a user of its own' (ruling 77). Experiment 022 must register that this provider's isolation case is expected to fail, or the move credits nothing it can pass. Real filters (b) are a large addition to MC5b. Applying a filter as it stands at read time is decided in one step and fits a vector; the model can keep only the restart interleaving. PRV-019's changed text belongs in the registration of the second `run.v1`.


## FR-467

**MC2 revisions and MC3: a stored-once property for REV-012; CON-013 becomes a vector; ACC-034 stands on the onward property**

G1b I12 · experiment-change · verifier: partly · decides: coordinator

Register row: ROADMAP.md:75 (experiment 012). Fixed before the runs:

MC2 revisions.
- Claim: of requests to change one object that name the same revision, at most one is stored (REV-012's catalogue slice), under timers, loss, reordering and an engine crash.
- New property K1: among the endings that store a change, at most one per named revision.
- New mutant LateStore: a reply that arrives after its change ended unknown is stored. Registered expectation: LateStore refutes K1 while V and J still hold. That shows that K1 says something V and J do not.
- Pass: K1, V and J hold in the four parts; duplication and a cut are added, or the reason is recorded why they add nothing.
- The row's label becomes 'changes only' unless setups and retirements are added.
- REV-011 is relabelled static; it has one vector.

MC3.
- CON-013 leaves the model. Its slice ('without setting up the consumer again') is a decision made in one step. Vector: open a Conduit whose consumer is ready; expect no intents and the consumer's status unchanged; then a request through it permitted. Pass: the vector fails on an engine that dispatches a setup at the open.
- ACC-034's standing is taken from O (its slice) and Q (delivery). The literal F is kept as a labelled control, registered as expected refuted in the timers, engine-crash and target-crash parts. No rerun is needed for this; it is a relabelling in G5.
- The qualified P that the first pass asked for (X03's work item 8) can be dropped: it would test a permission, not CON-013's safety clause.

Measures: unique states, depth, each row's mark, each mutant's state count against the correct model's.

**Evidence.** ROADMAP.md:75. experiments/012-model-checking/src/revisions.rs:284-303, :190-194; README.md:270, :275-278. docs/implementation/SAFETY_CATALOGUE.md:517 (REV-012), :273 (CON-013), :453 (ACC-034). src/access.rs:646-652, :675-688; README.md:817-827. Manifest cases naming REV-011: 1; CON-013: 0.

**Waits.** G5's rows for CON-013 and ACC-034; the MC3 rerun in the first pass's work list.

**Verifier.** partly. The REV-012 basis is confirmed (revisions.rs:190-194, :284-303; README.md:270). CON-013 is named by no case. The slices are at SAFETY_CATALOGUE.md:273, :453 and :517.

**Verifier's counterargument.** K1, as written, counts the engine's storing endings, and it would hold on the correct model. The catalogue's harm for REV-012 is a lost update at the implementation ('concurrent storage of both loses one authorized update', SAFETY_CATALOGUE.md:517). By reading, the correct model already applies two changes that name r0 (I6's counter). Register instead a property over Effect events per named revision, expected to be refuted in the Timers, Loss, Reorder and Crash parts. That refutation is the result that matters, and LateStore becomes unnecessary. For CON-013, the 39 existing open steps with empty intent lists need a name and a status assertion for a ready consumer that is not root, not a new vector written from scratch.

**Verifier's corrections.** Expected verdict for the new REV-012 property: refuted on the correct model, not held.


## FR-468

**Safety vectors: credit a static row only through a step that fails; write the 13 uncovered rows, SEC-008 first**

G1b I13 · experiment-change · verifier: partly · decides: coordinator

Register: the safety-vector orders SV1 to SV4 (docs/work-orders/117-plan-s2-s3.md:134-136), which serve v0's condition at IN_FORCE.md:265; the vectors are A39's. Fixed before SV2 tags anything:

Claim tested: each static row's catalogue slice is shown by at least one step that fails when the slice is broken.

Rule for the safety field (SV1, SV2): the field is set on a step, not on a case, and only where the step's expectation would differ if the slice were broken: a pinned refusal with `external_intents: []` and `stateUnchanged`, an exact array, or an `absent` pointer. A case-level promise name gives no credit.

Scope added to SV3 (or an SV5), in this order:
1. SEC-008: a non-root actor, and root in an onward request, each ask to open a Conduit that lists `get-secret.v1`; a Conduit of that kind whose consumer is an Interface. Expect refused, no intents, state unchanged.
2. ACC-012: a close whose bound does not cover the Conduit's target, and one whose bound misses an operation.
3. POOL-015: a Pool setup where the sender has no open Conduit to the Pool and the original caller has one.
4. ACC-009: a direct request that another Conduit's filter would forbid (permitted), and an onward request limited by a carried filter.
5. OPS-033: a literal at a place `secret-reference.v1` governs; assert that the result holds no part of it.
6. LIF-013 (after pending actions are built), NAM-023 (a result whose outputs fail the target's version ends unknown), SUB-009 (object count pinned across a replace), LIF-011 (lifecycle state asserted beside a changed condition), OPS-019, REV-001, FST-007's format.
7. Renames, no new step: FLEET-004 onto fleets/e3b/ownership-permits-nothing; PRV-024 onto setup/review-provider-records.
8. One request step each for TPL-011 and TPL-013.
Then the 11 static rows that no case names (X03/F9).

Pass condition: for each row, the vector fails against a build with the clause removed. Where a mutant build is not at hand, the order's verifier states which expectation would differ and why.

Measures: static rows shown, partly shown and not shown (today 26, 3 and 24 of 53); steps carrying the safety field; rows credited by a rename only.

**Evidence.** docs/work-orders/117-plan-s2-s3.md:134-136; IN_FORCE.md:265; item I7's table and counts; engine/crates/core/src/access.rs:824-832 (SEC-008's branch); conformance/README.md:71-78 (assertion forms). G5.md:405-407 'This paper order neither writes vectors nor audits whether existing vectors cover them'.

**Waits.** SV1 waits on order 071 (HANDOVER item 5); SV2 should not tag before the rule is fixed.

**Verifier.** partly. The rule matches IN_FORCE.md:265's definition. SV1 to SV4 are at docs/work-orders/117-plan-s2-s3.md:134-137. SV1 waits on 071 (docs/implementation/HANDOVER.md, queue item 5). G5.md:405-407 audits no vectors. A field set per step is compatible with 'each carries a field' if the case aggregates it; that is SV1's design, and the coordinator's to settle.

**Verifier's counterargument.** The rename of FLEET-004 fails the rule I13 sets out: ownership-permits-nothing steps 2 and 3 pin only the reply. ROADMAP.md:176's E10 does ask for 75 (TPL-011), but for the excluded control it plans only that no tool is presented, not a request step (TPL-013). CON-013 belongs on the rename list.

**Verifier's corrections.** FLEET-004 needs `external_intents: []` and `stateUnchanged` added to those steps, not a rename only. Today's 26/3/24 rests on sampled Yes judgments.


## FR-469

**Before any model runs again: grade every holds row on paper, and keep model checking for races**

G1b I14 · recommendation · verifier: partly · decides: coordinator

Change to the approach, in three parts.

1. A paper order first. Give the 81 holds rows of operations.rs, access.rs, placement.rs, lifecycle.rs and controls.rs the reading that I3 to I6 give the other 33: the catalogue's slice, the property, and a mark of S, G, K or L. The first pass already found a clause in MC4 that cannot fail and a property that joins four clauses (X03/F5), and reports 47 MC6 rows on 15 properties, so the same pattern is likely there. Fold it into the first pass's work item 1 (G5 recounted on paper).

2. One vocabulary in G5. Replace 'holds' by three standings: 'holds, searched' (S), 'restated at decision time' (G), and 'static' for K rows, each with its vector or contract test or 'none'. Ruling 75 E1 asks that each promise be 'held by a named model or listed with its reason'; a K row meets it honestly as 'listed with its reason: a decision made in one step, shown by a vector'.

3. A rule for every registration (ruling 45): a row is registered as searched only if its property relates events of different steps and a registered mutant that changes a guard or the order of two steps refutes it; the mutant's state count is reported beside the correct model's.

Existing paradigm. Model checking is used where interleavings decide the result: TLA+ and Stateright practice is to check consensus, leases and retries, and to distrust an invariant until a changed model has violated it. Decisions made in one step are shown by conformance tests: the Kubernetes conformance suite, critest for the Container Runtime Interface, the CSI sanity suite. Mutation analysis (PIT, cargo-mutants) separates a mutant that changes behavior from one a test merely detects; the queued MX0 should adopt that distinction.

Cost: one paper order reading about 9,900 lines of Rust and the matching README sections, with a fresh verifier; no model run. G5's holds count falls from 114 to 90 at most, probably further, and the static list grows from 53 to at least 77, which is more vector work for SV3 and for experiment 022.

What it buys: v0's condition is counted on evidence that could have failed; the checker's time goes to the handful of real races (the fence, one run per ID, the install race, secret reads across a restart); and the reruns that 077, 081 and 082 made necessary are written once, against the right properties.

No question for the owner arises: the catalogue's classification and an experiment's registration are the coordinator's. The reclassification should be logged for the owner to strike, and the new counts given with the first pass's question on what v0's condition requires (X03/F11).

**Evidence.** Line counts by `wc -l`: operations.rs 922, access.rs 907, placement.rs 2,123, lifecycle.rs 4,023, controls.rs 1,918 (9,893). G5 holds rows by source (script): operations 6, access 9, placement 11, lifecycle 37, controls 18. X03/F5 ('`restart_only` is `false` at every site that writes it'); X03's assessment ('MC6: 47 rows on 15 properties', not verified by its verifier or by me). docs/implementation/SAFETY_CATALOGUE.md:5-6 'The classification is the coordinator’s, open to the owner to strike'. IN_FORCE.md:267 (ruling 75 E1's words). docs/implementation/PROVIDERS.md:61 (MX0).

**Waits.** Every model-checking rerun in the queue; G5's restatement; the count reported against IN_FORCE.md:267.

**Verifier.** partly. `wc -l`: operations.rs 922, access.rs 907, placement.rs 2,123, lifecycle.rs 4,023, controls.rs 1,918 (9,893 lines). The 81 holds rows are recounted. The catalogue's classification is the coordinator's (SAFETY_CATALOGUE.md:5-6). Ruling 75 E1's wording is at IN_FORCE.md:267 and docs/proposals/ruling-75/README.md:94-95. Folding this into X03's work item 1 avoids a second order.

**Verifier's counterargument.** Instead of a hand reading of 9,900 lines, mark K mechanically at the next run: run every mutant on the configuration with all interleaving faults switched off, and mark K any property that this single schedule refutes. A hand reading is then needed only for the rest. The state-count rule in point 3 should be replaced as described under I2.

**Verifier's corrections.** PIT and cargo-mutants report killed or caught, survived or missed, timeout, and unviable. Neither separates mutants that change behavior from mutants a test merely detects, and neither decides equivalence. MX0 appears once in the tree (docs/implementation/PROVIDERS.md:61) and is defined nowhere, so the assumption that it is a mutation order is unsupported. If the G and L rows leave 'holds' too, the count falls to at most 86.


## What the verifier says the analyst missed

- **By reading, MC2 revisions' correct model already applies two changes that name the same revision** Read from revisions.rs:157-204, not checked by a model run. In the Timers part, Change(0) is dispatched and a timer ends it unknown. REV-004 (spec/conventions/revisions.yaml:52-54) leaves the revision at r0, so Change(1), which also names r0, passes V and is dispatched. Both Dispatch messages produce an Effect, and under Reorder Effect(0) can land last. V and J hold throughout. REV-012 and REV-014 hold a change back only while another 'has not ended' (revisions.yaml:186, :232), and OPS-001 counts unknown as an end (operations.yaml:20-23). This is the same family as X03/F1 and F2 and the honest case in L02/F1's verifier, but at REV-014 rather than in reconciliation. The question is whether holding a change back until the earlier one is fenced adds behavior: if it does, it is the coordinator's under 71 point 4 and logged, or the owner's if it changes ruled behavior.
- **G5 counts PRV-025 as holds, while the shipped SSH provider documents it as unmet** packages/provider-ssh-runtime/README.md:21-22 and contracts.yaml:16: 'PRV-025's isolation is unmet until each Agent has a user of its own' (ruling 77). G5.md:158 gives holds, from a property that cannot fail (gates.rs:667, :670-675). The dogfood provider (ruling 70) therefore runs against a row the account calls holding. G5 should note the limit, and experiment 022's registration should expect the failure.
- **Evidence outside the manifest was not counted** Engine tests and runner checks cover several rows marked K: PRV-023 (engine/crates/shell/tests/runner.rs:864-878), PRV-029 (engine/crates/shell/src/tests.rs:81-83; tests/changes.rs:364; engine/CONFORMANCE.md:39) and PRV-024 (engine/crates/conformance/src/records.rs:72-75). Any reclassification should name these as evidence or say why they do not count.
- **Only one of the six searched rows saw a partition** Of the rows marked S, only PRV-028 (MC5a's Cut part) was checked under a partition. MC5b and MC2 revisions have none (secrets.rs fault_allowed; README.md:275-278). That matters for IN_FORCE.md:267's 'under loss, reordering and partition' and should go with X03/F11's question to the owner.
- **CON-013 is shown by steps that do not name it** 39 open steps pin an empty intent list, e.g. conventions/setup/base/case.yaml step 2. CON-013 belongs with FLEET-004 and PRV-024 as a case where the names understate the tests.

## The verifier's notes

I read in full: X03's assessment, its F1 and F2, and the headings of its findings; gates.rs; secrets.rs from line 140 to 1000; records.rs at every cited region (records 150-780, observation 960-1180, installs 1455-1850); revisions.rs from line 60 to 310; mc2.rs; the harness's timer gating; access.rs's onward decision and its properties. Also: G5's counts and the rows for the four sources, the catalogue rows cited, and the MC5a, MC5b, MC2-revisions, MC3 and MC7b results tables in the README. I recounted by script: G5 by module and standing; manifest naming and citing steps for all 42 static rows and the 24 rows marked K; a field diff of the spec between ed8043a1 and HEAD. I opened the vectors for SEC-008, POOL-015, ACC-012, OPS-033, FST-007, FLEET-004, OBS-002 and LIF-013, and grepped PKG-023. I did not open every Yes row of I7, and I did not read controls.rs, lifecycle.rs, placement.rs or operations.rs. I ran no model: the REV-012 trace is a reading of the code. Overall: the analyst's central finding holds. Most of the 33 rows rest on predicates the search cannot reach, the recounts are exact, and the experiment changes are well aimed. Five points weaken it. The state-count test misreads mutants that change behavior. The analyst overlooked evidence outside the manifest: engine tests for PRV-023 and PRV-029, a runner check for PRV-024, and 39 open steps for CON-013. Its marks are inconsistent in places (PRV-028, ACC-034's O, REV-012). The paradigm claim about mutation tools is wrong. The REV-012 property it proposes would hold on a correct model that already applies two changes naming the same revision.
