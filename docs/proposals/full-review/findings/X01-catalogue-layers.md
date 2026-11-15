# X01-catalogue-layers: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 1, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** Pinned at 2eb8f258 (2026-11-12). The counts are right: 519 promises, 226 language, 245 convention, 48 interface, and every per-file count in the README's contents table matches (levels: 466 must, 48 must not, 2 should, 3 may; no duplicate ID, no dangling reference to a promise or a DL item).

The catalogue is growing, not converging. The first commit with promises (2026-11-06) had 434; the week ended 2026-11-08 at 454; the pinned commit has 519. Since the rewrite of 2026-11-07, 67 promises were added and none removed; 64 of them landed on 2026-11-11 and 12. Promise text grew from 59,341 to 102,201 words in six days (+72%). The growth is in the convention layer (179 to 245, +37%) and the interface layer (35 to 48); the language layer is flat (220 to 226). The owner made 77 rulings in eight days (37 on 2026-11-05, 6, 1, 3, 20, 9, 1), 48 of them since amended, and about a dozen spec-pending lines in IN_FORCE.md (batch 2, metrics, order 050's logins, 049a and 049b) queue more text, nearly all of it convention-layer.

The convention layer has outgrown the claim. 'The few behaviors every part must agree on, stated as invariants' now describes 245 promises with 12,014 words of `then` and 39,577 of observable. By my grouping 141 govern the seven named conventions; 104 govern the setup and retirement procedure (16), provider contracts (26), secrets (15), enrollment (10), pending actions and sign-in (12), placement and machine loss (7), package installation (10), and registration and first start (8). Half of the layer's `then` text is in the procedural groups, and the implementation-defined list, where the claim sends an engine's own behavior, has 17 items. Either the claim's words change or content moves (F8 to F13); that is the owner's choice.

The form has slipped with the growth. 'One obligation per promise' fails for about a quarter (137 `then` fields of three or more sentences). The observable field holds three times the text of `then` and carries obligations, and nothing says whether it binds, though 153 of the 394 safety classifications rest on it (F1). No field says who a promise binds (F2). The same rule is written up to four times across promise, observable, contract description and core README, and the copies already differ (F11).

Coverage is thin where the growth was: 292 of 519 promises are cited by a vector; 155 of the 394 classified safety promises have none; enrollment, pending and sections have none at all; and the 17 newest promises are in no safety classification.

**What the reader read.** Method. Read the brief and 'What holds'; read every `when` and `then` of all 519 promises; read observables by script (lengths, reasons, cross-references) and about 90 in full; ran /usr/bin/python3 with PyYAML over spec/, conformance/manifest.yaml, the safety catalogue and read-only git. Nothing was built or run. Not checked: the content of any vector, the engine, the 28,239-word definitions, most observables line by line. The grouping of the 245 and the lists of 'clearest' and 'most consequential' are my judgment.

(1) Recount. 519 total; language 226, convention 245, interface 48; all 27 file counts match the README. Also matching: 147 exported core documents (95 in core/, 52 in schemas/), 37/8/6/2/13/12/17 contracts and templates, 14/74/41 example files, 17 DL items. Mismatches: the README's vector column in five rows (objects 13 vs 15, conduits 12 vs 14, lifecycle 18 vs 25, access 23 vs 25, revisions 4 vs 9; sum 274 vs 292); README:268 says two topic files hold interface promises, four do; conformance/README.md:453 says 217/91 against the manifest's 226/82; the `vectors` field is filled in 2 of 519 promises. Twelve skipped IDs (ACC-045 to ACC-052, IFC-048, LIF-038, LIF-039, OPS-035) are in no head register; they were proposal IDs never in spec/, used with other meanings in docs/proposals/projects/brief.md.

(4) Coverage from the manifest's 494 cases. Cited 292, uncited 227: language 97 of 226, convention 104 of 245, interface 26 of 48. By file (uncited/total): sections 15/15, pending 13/13, enrollment 12/12, secrets 22/24, providers 21/30, placement 14/21, interface 23/46, lifecycle 13/38, naming 11/28, schemas 11/21, access 10/35, objects 9/24, conduits 8/22, contracts 8/12, manifest 7/17, pools 5/15, revisions 5/14, packages 4/24, templates 4/14, instantiation 3/23, implicit 3/5, observation 3/5, operations 2/32, submissions 1/11, first-start 0/6, fleets 0/6, records 0/6. 153 of the 227 have no recorded reason. Safety catalogue: 394 safety rows, 155 with no vector (convention 84 of 217, language 48 of 133, interface 23 of 44; by kind access 69, effect 44, secret 29, result 13); 72 promises are cited by exactly one case; 86 cases have no `safety_rows` key.
The twenty most consequential untested: ACC-056 (what a refusal discloses), ACC-039 (a retired consumer's Conduit), ACC-019 (nothing dispatched for an ended invocation), ACC-027 (enforced or mediated, preservation test 9), IMP-001 (model work only through a marked operation), OBS-004 (no background reconciler), LIF-023 (no retirement without a request), SEC-010 (the engine's read through the instance's own Conduit), SEC-018 (a secret goes nowhere else), SEC-015 (no value in a record), PRV-019 (one run per instance), PRV-030 (a stop that succeeded means stopped), PRV-025 (the gate), PRV-028 (TLS proof at a run's url), IFC-017 (nothing sent before proof), IFC-019 (an implementation applies filters), IFC-030 (a consumer's credential is never forwarded), ENR-006 (admission checks), PND-004 (codes and fingerprints only in the panel), SCT-015 (the short form means its translation).

(5) Confirmed by script: exactly 17 promises are absent from the safety catalogue: CTR-014, ENR-001 to ENR-012, PKG-032, PND-008, PND-013, PRV-030 (14 convention, 2 language, 1 interface). The catalogue's other 502 IDs all exist, with matching layer and level. Sixteen of the 17 also have no vector (CTR-014 has one).

(6) Growth. Promises at each commit touching spec/ (34 commits): 2026-11-06: 434 (220/179/35); 2026-11-07: 452 then 454 (202/208/44; the rewrite added 85 and removed 67); week ending 2026-11-08: 454; 2026-11-09 and 10: 455; 2026-11-11: 470, 486, 502, 515, 516, 517, 518; 2026-11-12 (pinned): 519 (226/245/48). Rulings by date from the ledger (77 rows, none missing): 2026-11-05: 37; 11-06: 6; 11-07: 1; 11-09: 3; 11-10: 20; 11-11: 9; 11-12: 1. Status: 25 in force, 48 in force and amended, 3 done, 1 replaced.

Known points not re-reported: the 502-promise safety catalogue, the list-machines.v1 and place.v1 contracts lagging PRV-014 and PRV-015 (spec pending, 049b), the owed-stop split of PRV-021 (queued), coverage beyond the build's slice (reported only with the new situation in F5).


## FR-153

**The observable field carries obligations, and the form does not say whether it binds**

X01/F1 · defect · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-140.

**Promises.** ACC-056, ENR-001, ENR-006, ACC-029, ACC-053, REC-004, PRV-029

**Where.** spec/README.md:291

**Quote.** "| `then` | The single obligation. For `must not`, the thing that must not happen. |
| `observable` | What can be checked to tell whether the promise was kept. |"

**Situation.** By script over the 519 promises: `then` holds 22,622 words (1,139 sentences), `observable` 69,486 words (2,887 sentences). 103 observables state a reason (refused, invalid, conflict, unavailable) that their own `then` never names; 41 cite an implementation-defined item (DL) that their `then` does not; 142 are at least three times the length of their `then` and over 100 words. Two `then` fields point into their observable for the rule itself: ACC-056 says "the cause in the refusing rule's row below" (spec/conventions/access.yaml:918) and the 59-row table is in the observable (:939); ENR-001 says "meet the checks below" (spec/promises/enrollment.yaml:13) and the checks are in the observable. ENR-006's `then` lists eight checks; which reason a failed check reports is only in the observable (spec/promises/enrollment.yaml:148-149). ACC-029's observable alone says how a bound is written, that an empty bound covers nothing and that no bound means unlimited (spec/conventions/access.yaml:483-507). Reading A (a builder who takes the README at its word): `then` is the obligation, the observable is a test hint, so these rules are optional. Reading B (the coordinator's own safety catalogue): the observable binds; 153 of its 394 safety rows quote the observable as the unsafe slice (docs/implementation/SAFETY_CATALOGUE.md, e.g. :449, :498, :599). Two builders diverge on reasons, diagnostics and disclosure wherever the rule is observable-only.

**Counterargument.** Most observable-only sentences restate consequences derivable from other promises' `then` fields, and every engine builder so far has been the same session, which reads both fields as binding. I found no safety behavior that is provably absent from every `then`.

**Least change.** Add one sentence to 'The form of a promise': "What `observable` states binds as `then` does; where the two differ, `then` governs." Then, as files are next touched, move reasons and conditions that are obligations into `then` (or a split promise) and leave in `observable` only what a test reads.

**Precedent.** IETF practice: normative text is marked as such and examples or test notes are marked non-normative (RFC 2119 with 'This section is non-normative'); W3C specifications do the same.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a defect of form whose fix adds no behavior; the reading chosen is the one the safety catalogue and the vectors already use.

**Verifier.** confirmed, medium. Quote verbatim at spec/README.md:291-292. Recount at 2eb8f258 matches: 519 promises, `then` 22,622 words, `observable` 69,486; 103 observables name a reason their `then` does not. spec/conventions/access.yaml:918 `then` says "the cause in the refusing rule's row below" and the table starts at :939, inside the observable. spec/promises/enrollment.yaml:13 `then` says "meet the checks below"; the checks are in the observable (:26-31). ENR-006's `then` (:131) names no reason; "A failed check 8 ... reports invalid; every other failed check reports conflict" is at :148-149, observable only. docs/implementation/SAFETY_CATALOGUE.md: 153 of 394 rows quote `observable` (recounted). Nothing in spec/README.md or spec/definitions.md says whether the observable binds (grep). Practice reads it as deciding: conformance/README.md:619-621 "OPS-029's observable said ... So the case was not undecided". Not in any known-open list.

**Verifier's corrections.** Mark known-new-situation, not new: docs/proposals/reaudit/round-6/design-critique.md:595-596 already said "Treat `observable` as part of the obligation review: several important decisions live there rather than in `then`"; D21's decision (round-6/DECISIONS.md:125) applied reading paths only. Class is nearer ambiguity than defect. Reading A is weaker than stated: README:292 makes the observable the test of whether the promise was kept, so an engine that fails it has not kept the promise. Unsafe stays false: the cases I tried (OBJ-010, ACC-029, ENR-003, ACC-056 tier 1) have a `then` that covers the safety slice. Small count differences: 46 observables cite a DL their `then` does not (41 stated); 140 are three times their `then` and over 100 words (142 stated). Coordinator decides, as stated.


## FR-154

**No field says which party a promise binds, although the conformance levels are defined by it**

X01/F2 · gap · verifier: weakened, low · decides: coordinator · new

Same fault, or overlapping: FR-143.

**Promises.** PRV-015, PRV-019, PRV-025, ENR-002, IFC-001, OPS-002, NAM-024, PKG-031, OBJ-024

**Where.** spec/README.md:322

**Quote.** "Every promise, whatever its layer, whose obligation falls on the implementation and not on the engine. Examples are IFC-008, IFC-009, IFC-019, IFC-026, IFC-031, IFC-047, SEC-006 and SEC-019."

**Situation.** The Engine and Implementation levels (spec/README.md:321-322) are sets of promises selected by "whose obligation falls on" the engine or the implementation. A promise has seven fields (id, layer, level, when, then, observable, vectors) and none names the bound party; the README gives examples, not a list. By script, at least 10 promises name a provider as the actor (ENR-002, ENR-005, PRV-011, PRV-012, PRV-019, PRV-021, PRV-022, PRV-025, PRV-028, PRV-030), at least 20 an implementation (OPS-002 to OPS-004, OPS-006, IFC-008, IFC-009, IFC-019, IFC-026, IFC-031, IFC-047, SEC-006, SEC-019, PKG-016 and others), 6 the specification's publisher or a package's author (OBJ-024, IFC-029, NAM-024, NAM-025, PKG-012, PKG-031), and about 10 bind the engine and a provider in one `then` (PRV-015: the engine asks and the provider chooses; ENR-001, ENR-004, ENR-006, PRV-005, PRV-021, PRV-026). A provider author (packages/provider-ssh-runtime) or an independent engine builder cannot compute the list of promises their claim covers; two authors will draw it differently, and a promise that binds two parties cannot be assigned to one level.

**Counterargument.** The prose of each promise usually makes the subject plain, and the README lists PRV-010 to PRV-012, PRV-014, PRV-019, PRV-021, PRV-022, PRV-025 and PRV-028 for a provider. With one engine and in-repository providers nobody has yet needed the computed list.

**Least change.** Add a field to the form: "| `binds` | Who must keep it: `engine`, `implementation`, `provider`, `package` or `specification`; a list where a promise binds more than one. |" Fill it for the 519, split the promises that bind two parties, and replace "Examples are" by "the promises whose `binds` names it".

**Precedent.** W3C QA Framework, Specification Guidelines: name the classes of product and say which class each requirement applies to. POSIX, which the specification already follows for its conformance document, separates implementation conformance from application conformance.

**Who decides.** coordinator. Rulings 37 and 46 point 4: a gap in the form; the field adds no behavior and makes the existing conformance levels computable.

**Verifier.** weakened, low. Quote verbatim at spec/README.md:322; all 519 promises have exactly the seven keys (script). PRV-015 binds two parties in one `then`: spec/promises/providers.yaml:468-470 "The engine asks the machine provider with `place.v1` ... and the provider names by id one machine". spec/README.md:161 tells a provider author that "Conformance says, at the Implementation level, which promises bind what you write", and Conformance gives examples only. Already recorded: docs/proposals/docs-audit/README.md:78 "The Implementation level names four promises as examples and does not list them all | Which promises bind an implementation follows from each promise's own wording"; docs/proposals/reaudit/round-5/DECISIONS.md:164 (M87) "No fourth level is needed and no behavior changes".

**Verifier's corrections.** Known-new-situation, not new: examples in place of a list is a logged editor's choice, accepted under ruling 58 point 1 (docs/AUDIT.md:137). What is new is the promises that bind two parties in one `then` and README:161's unmet promise. Class: a defect of form or ambiguity, not a gap (no runtime situation is left undecided; the prose of each promise names its actor). The least change is larger than needed: a generated list by ID under README:321-322 gives the computable levels without a new field on 519 promises; splitting two-party promises is separate work. Would rise to medium once an outside provider author (S7) or an independent builder must make the claim. Coordinator.


## FR-155

**'One obligation per promise' fails for about a quarter of the catalogue; the twenty clearest, with splits**

X01/F3 · defect · verifier: confirmed, low · decides: coordinator · new

**Promises.** ACC-033, IFC-025, IFC-033, IFC-041, IFC-049, IFC-017, PRV-007, PRV-019, PRV-022, PRV-025, PRV-014, PRV-015, PND-010, PND-013, ENR-006, ENR-001, OPS-008, OPS-014, LIF-021, CON-008

**Where.** spec/README.md:297

**Quote.** "1. One obligation per promise."

**Situation.** By script: 137 `then` fields have three or more sentences, 49 have five or more, 59 exceed 100 words (ACC-033: 486 words, 29 sentences). Coverage is counted per promise, so one tag on ACC-033 (cited by 45 cases) says nothing about which of its 21 rows is tested. The twenty clearest, with the split I would make: (1) ACC-033, spec/conventions/access.yaml:583: one promise per verb family: registering (rows 1, 1A, 2, 3); set-up and retire (4, 4A, 5); inspect (6, 6A, 7); cascades (8, 8A); install and reserved (9); open, close, lift (10); control (11); reconcile (12 to 14A); list (15); sign-in (16); answer (17); and 'no request is authorized against a retired Pool or Fleet'. (2) IFC-025, spec/interface/implementation.yaml:648: outcome key; reason key; restriction key; invocation key; isError; structuredContent; first text block; plain content. (3) IFC-033, :891: schemas and narrowing; `$defs` bundling and `$ref` rewriting; description; annotations; no contract, no tool. (4) IFC-041, :1102: headers; loopback limit; https only; proof of an enforcement point. (5) IFC-049, :1352: capability and -32021; enrollment without it; result shape; message content; URL limits; what is never relayed. (6) IFC-017, :439: how each item is reached; nothing sent before proof; no redirect; proof on every request. (7) PRV-007, spec/promises/providers.yaml:207: when the engine asks; value conversion and null; secrets and identity; how the server is reached; relative-path command; `unknown` gives unavailable. (8) PRV-019, :570: no second run; running gives the url; cannot tell gives unknown; only the given machine; cannot run gives failed. (9) PRV-022, :667: the names; delivery over other values; nowhere else; CONDUIT_URL whole; no-run case. (10) PRV-025, :798: what the gate admits; nothing local goes around it; what the contract states; closing a silent stream. (11) PRV-014, :419: id; enrollment and evidence; hostname; labels; state. (12) PRV-015, :460: the engine's call; the provider's choice; the placement; not instantiated against unavailable. (13) PND-010, spec/promises/pending.yaml:159: who may call; the Type must state signIn; no other consumer's Conduit lists it; unknown login; port clash; which login starts. (14) PND-013, :226: one promise per check and reason; the one-use rule. (15) ENR-006, spec/promises/enrollment.yaml:125: enrollment standing (1, 2); hostname and labels (3, 4); evidence minimum (5, 6); fingerprint (7, 8). (16) ENR-001, :5: the provider call; what opens; what the enrollment holds; expiry; result. (17) OPS-008, spec/conventions/operations.yaml:146: key scope and one invocation; the state kept; no purge touches it. (18) OPS-014, :305: recording 'occurred'; 'did not occur' only once fenced; the definition of fenced. (19) LIF-021, spec/conventions/lifecycle.yaml:417: the order; the container last; which retirements are dispatched; what a failure holds back. (20) CON-008, spec/promises/conduits.yaml:102: what closes a Conduit; when; requests to a retiring target; removal. Also MAN-010, SUB-009, TPL-012, TPL-018, LIF-009, CTR-013 and PLC-017.

**Counterargument.** A table such as ACC-033's is easier to read whole than as twelve promises, and consolidation pass 1 deliberately folded promises together. Splitting adds IDs to a catalogue that is already large.

**Least change.** Split each listed promise, the first obligation keeping the ID and the rest taking new IDs (never reused); a table stays a table in one promise only where its rows are tagged (ACC-033 row 4A) and a vector cites the row. Where splitting is not wanted, change rule 1 to say what a promise may hold.

**Precedent.** Kubernetes conformance tests: each test documents one behavior and is tagged for it; IETF requirement lists, one MUST to a numbered item.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the text breaks its own rule; a split adds no behavior.

**Verifier.** confirmed, low. spec/README.md:291 "The single obligation" and :297 "1. One obligation per promise." verbatim. Recount: 137 `then` fields of three or more sentences, 49 of five or more, 59 over 100 words; ACC-033 486 words, 29 sentences, cited by 45 cases. All twenty cited path:line values are the promises' `- id:` lines (checked). Spot reads of PRV-015, PRV-019, PND-010, PND-013, ENR-001, ENR-006, TPL-012 confirm several independent obligations each. No earlier record of the breach found (grep of docs/, IN_FORCE.md, the logs).

**Verifier's corrections.** ACC-033 as one table is ruled: docs/proposals/ruling-44/README.md:62-63 "Management routing (ACC-033) restated as one decision table, with no change of behavior (D18)"; round-6/DECISIONS.md:122 "A list that replaces the prose keeps one source". Splitting it by verb family reverses that and is the owner's; keep the table and tag its rows. "About a quarter" is a sentence-count proxy, not a count of obligations. Splitting renumbers IDs cited in IN_FORCE.md, the safety catalogue, G5 and the manifest's cases; the no-behavior fix a coordinator can make cheaply is to reword rule 1 and tag rows so a vector can cite one.


## FR-156

**Seven promises have an observable that names nothing a test could see**

X01/F4 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** IMP-001, IMP-003, IFC-005, IFC-029, OBJ-024, NAM-025, PKG-016

**Where.** spec/conventions/implicit.yaml:28

**Quote.** "Every record of model work starting names such an invocation, its caller and its Conduit."

**Situation.** No promise lacks an `observable` (0 of 519). Seven have one no vector can use, and none is cited by a vector. IMP-001: REC-001 lists what is recorded (spec/conventions/records.yaml:17-21) and 'model work starts' is not in it; the engine sees an invocation of a marked operation, not a model's turn, so the observable reduces to 'a record of an invocation names that invocation'; the party that could break the promise is an Agent's implementation, which starts a turn on an unmarked call. IMP-003: the observable is 'reading the contracts ... tells', a reading and not a check that an unmarked operation starts none. IFC-005 and OBJ-024 bind whoever publishes a later version of the specification; IFC-029 holds only once a second version changes the prefix; there is one version. NAM-025 (should) asks that a contract 'means the same' across releases, which no check decides. PKG-016 asks that an implementation's results not depend on having served another instance, a property of all its runs that no finite vector shows. Under rule 5 these seven stay 'not yet testable' for ever and lower every coverage figure.

**Counterargument.** IMP-001 and IMP-003 state the purpose's sixth preservation test and are worth keeping as promises even if only the engine's half (IMP-002, IMP-008) is tested. Version rules have to be written somewhere before a second version exists.

**Least change.** Move IFC-005, IFC-029 and OBJ-024's publication rule to the README's Conformance section as statements about versions; make NAM-025 guidance beside NAM-024; rewrite IMP-001 to bind the implementation ("An implementation starts or continues a turn only while it serves an invocation of an operation marked `startsModelWork: true`") with the provider-contract harness as its observable; give PKG-016 an observable a harness can run (two instances, interleaved calls, equal results).

**Precedent.** The specification's own rule 2; POSIX separates rationale and future directions from testable assertions.

**Who decides.** coordinator. Rulings 37 and 46 point 4, under the form's own rule 2 (spec/README.md:298): "If nothing could tell, it is not a promise."

**Verifier.** weakened, low. spec/conventions/implicit.yaml:28 verbatim; none of the seven is cited by a case (script over conformance/manifest.yaml). But the manifest records reasons and a plan for four: conformance/manifest.yaml:7467-7470 IMP-001 "Increment 3 must register real wrapper contracts and show explicit authorized turns only"; IMP-003 "A portable omitted-versus-false startsModelWork pair is absent; add it with wrapper contract vectors"; NAM-025 "a should recommendation for package authors"; OBJ-024 has a reason too. experiments/012-model-checking/G5.md:309 has IMP-001 holding in `controls::W`. PKG-016's observable, spec/promises/packages.yaml:132-133, "The results such an implementation returns for one instance are the same whether or not it has also served others", is what the reviewer's own fix asks a harness to run. IFC-029's observable (spec/interface/implementation.yaml:812-815) names a checkable answer once two prefixes exist.

**Verifier's corrections.** Three survive: IFC-005, IFC-029 and OBJ-024, rules on a later version or its publisher that nothing can test while one version exists. "Stay not yet testable for ever" is wrong for IMP-001, IMP-003 and PKG-016. The rewrite proposed for IMP-001 duplicates IMP-002, which already forbids model work when "an operation not marked as starting model work is invoked on an Agent, whatever it delivers to the Agent" (implicit.yaml:35-41). IMP-001's unstated bound party belongs under F2. NAM-025's point is recorded (manifest reason; docs/proposals/spec-batch-1/areas/short-form.md:349). Coordinator.


## FR-157

**Vector coverage is recorded in five places that disagree, and 153 untested promises have no recorded reason**

X01/F5 · defect · verifier: confirmed, low · decides: coordinator · known-new-situation

Same fault, or overlapping: FR-152.

**Promises.** CTR-013, SCH-009

**Where.** spec/README.md:301

**Quote.** "5. A promise with no vector is not yet testable and does not count as covered."

**Situation.** Recounted at 2eb8f258. (a) The `vectors` field (spec/README.md:293, "The conformance vectors that test it") is empty in 517 of 519 promises; only CTR-013 and SCH-009 list paths. By rule 5, 517 promises are not covered. (b) The manifest's case tags (conformance/manifest.yaml, 494 cases) cite 292 promises: language 129 of 226, convention 141 of 245, interface 22 of 48. (c) The README's contents table sums to 274 and is wrong in five rows against the tags: objects 13 (15), conduits 12 (14), lifecycle 18 (25), access 23 (25), revisions 4 (9) (spec/README.md:201, 215, 243, 244, 246); its lead-in calls the figures "the recorded S1 coverage as of 2026-11-10" (:193) while line 7 gives today's 494 vectors. (d) The manifest's `s1_coverage` covers only the 308 promises frozen at increment 2: 226 with vectors, 82 with a reason; 8 of those reasons are stale, since a case now cites the promise (OBJ-001, OBJ-016, OBJ-019, OBJ-022, OBJ-023, CON-031, LIF-032, REV-003), and 74 rows list fewer vectors than the tags. (e) conformance/README.md:453 says "217 rows with vectors and 91 with reasons". The README says the manifest "records the coverage and its gaps" (spec/README.md:8): of the 227 promises no vector cites, 153 are outside the S1 inventory and have no reason anywhere (convention 89, language 40, interface 24), 122 of them classed safety. A reader asking 'is PRV-019 tested?' gets no from the promise, no from the README row, and silence from the manifest.

**Counterargument.** The tracker already says coverage beyond the build's slice is open and that counts lag; the README dates its column. The runner checks that every vector named in `s1_coverage` declares its promise, so the lists are consistent subsets, only stale.

**Least change.** Keep one source, the case tags. Drop `vectors` from the form or generate it; have a checker in `tools/` write the README's per-file column and fail when it differs; extend the reasons list from the 308 S1 promises to all 519; correct spec/README.md:8 and conformance/README.md:453.

**Precedent.** Kubernetes conformance: tests carry the tag, `test/conformance/testdata/conformance.yaml` is generated from them and CI fails when it is stale.

**Who decides.** coordinator. Rulings 37 and 46 point 4: stale and conflicting statements; the fix adds no behavior. The safety-vector field of ruling 68 point 2 is spec pending and could be written in the same change.

**Verifier.** confirmed, low. spec/README.md:301 verbatim. Recount: 494 cases cite 292 promises (language 129, convention 141, interface 22); the README's column sums to 274 and differs in five rows (objects 13 against 15, conduits 12/14, lifecycle 18/25, access 23/25, revisions 4/9); `s1_coverage` has 308 rows, 226 with vectors and 82 with reasons, 8 reasons stale (OBJ-001, OBJ-016, OBJ-019, OBJ-022, OBJ-023, CON-031, LIF-032, REV-003); conformance/README.md:453 "217 rows with vectors and 91 with reasons"; 227 promises uncited, 153 of them outside the S1 inventory (convention 89, language 40, interface 24), 122 classed safety; `vectors` non-empty only in CTR-013 and SCH-009. docs/AUDIT.md:186-188 says "The manifest states the coverage gaps", which is untrue for those 153.

**Verifier's corrections.** Known-new-situation, as the reviewer marked it. Point (a) overreads: an empty field is the recorded form, CONTRIBUTING.md:122-123 "`vectors: []`", and docs/work-orders/075-spec-own-tools-reach.md:185-186 says the case "is tied to them by the coverage index"; the defect is spec/README.md:293 against that practice and the two promises that do list paths, not 517 uncovered promises. Point (c): README:193 dates its column ("recorded S1 coverage as of 2026-11-10"), so it is stale as dated, not wrong. I count 66 S1 rows listing fewer vectors than the tags (74 stated). Coordinator.


## FR-158

**Which promise ID a refusal carries is undetermined where several promises state the same refusal**

X01/F6 · ambiguity · verifier: confirmed, medium · decides: coordinator · new

Same fault, or overlapping: FR-004, FR-126, FR-210.

**Promises.** ACC-056, ACC-012, ACC-024, LIF-022, LIF-034, REC-004, ACC-032, ACC-001, ACC-028

**Where.** spec/conventions/access.yaml:915

**Quote.** "Its result carries one error diagnostic, whose ruleId is the refusing promise and whose properties.refusal has the form refusal.v1."

**Situation.** ACC-056 gives one ruleId per refusal and a cause per rule, and REC-001's observable has the record hold 'the deciding promise's ID'. The observable then says "Promises that restate a refusal reuse the deciding access rule's cause; the ruleId remains the deciding rule" (spec/conventions/access.yaml:935-936), but 22 of its 59 rows are for promises whose cause defers to another rule (ACC-023, ACC-028, FLEET-004, OPS-024 and PND-011 to ACC-001; REC-004 to ACC-032; PRV-025 to IFC-022; SEC-010 to SEC-007; IFC-025, OPS-005 and OPS-023 say 'use the deciding rule'), and nothing says which of two stating promises is 'the deciding rule'. Situation 1: root asks `close.v1` for its declared Conduit to `management`. Reading A: ruleId ACC-012, as ACC-024's own observable says ("reports refused under ACC-012", spec/conventions/access.yaml:428). Reading B: ruleId ACC-024, as LIF-022 and LIF-034 say ("refused under ACC-024", spec/conventions/lifecycle.yaml:471 and :712) and as the table's separate ACC-024 row allows. Situation 2: an Interface other than root asks to purge records: REC-004 (its row exists) or ACC-032 (the row it cites). Situation 3: an actor with a Conduit to a Pool asks an operation of its Resource: ACC-001 or ACC-028. Two engines write different ruleIds into the diagnostic, the first text block and the record; a portable vector that compares the ruleId passes on one and fails on the other. No vector cites ACC-056.

**Counterargument.** The human-readable cause is the same under either ID, and ruling 52's aim (say what happened and why) is met by both readings. The sentence at :935 shows the intent that the base access rule is reported.

**Least change.** Add to ACC-056's `then`: "Where more than one promise states the refusal, the ruleId is the one named in the last column of that rule's row." Give the table a third column, 'ruleId reported', filled for all 59 rows (ACC-001 for ACC-023, ACC-028, FLEET-004; ACC-032 for REC-004; and so on), and make access.yaml:428 and lifecycle.yaml:471, :712 agree.

**Precedent.** SARIF 2.1.0, which `diagnostic.v1` reduces: one `ruleId` per result, and `relationships` between reporting descriptors to say that one rule restates another.

**Who decides.** coordinator. Rulings 37 and 46 point 4: an ambiguity whose fix picks one of two existing readings and adds no behavior; ruling 52 asks that a refusal be explicit.

**Verifier.** confirmed, medium. spec/conventions/access.yaml:915-916 verbatim; :935-936 "Promises that restate a refusal reuse the deciding access rule's cause; the ruleId remains the deciding rule." The text contradicts itself on situation 1: ACC-024's observable, access.yaml:428, "reports refused under ACC-012"; LIF-022's, spec/conventions/lifecycle.yaml:471, "is refused under ACC-024"; LIF-034's, :711-712, "reports refused under ACC-024"; the table has a row for both ACC-012 and ACC-024. The ruleId is meant to be compared: spec/interface/implementation.yaml:1349 "the ruleId, reason and refusal.v1 fields are observable"; REC-001 has the record hold "the deciding promise's ID". No case cites ACC-056 and no vector yet asserts an ACC ruleId on a refusal. No record of the point in the logs, IN_FORCE.md or the tracker.

**Verifier's corrections.** Situation 1 is a contradiction between observables, stronger than an ambiguity. Situations 2 and 3 are settled if :935-936 binds and a row's parenthesis names the deciding rule (REC-004 to ACC-032, ACC-028 to ACC-001); but then the deferring rows' own cause text is never used ("reuse the deciding access rule's cause"), so what those rows are for is unclear. I count 20 deferring rows of 59 (22 stated). The deciding sentence is observable-only, so this rests on F1. Coordinator; the fix adds no behavior.


## FR-159

**Four promises carry a layer that LD1's test does not give them, so the Language level does not decide document validity**

X01/F7 · defect · verifier: weakened, low · decides: coordinator · new

Same fault, or overlapping: FR-056.

**Promises.** ENR-005, PRV-002, PRV-013, SEC-024

**Where.** spec/promises/enrollment.yaml:105

**Quote.** "fingerprint is derived from the machine's key and has a string value of 6 to 8 characters, the same on the machine and in the panel. The panel shows the value exactly as the provider reported it."

**Situation.** The Language level is "Every promise whose layer is `language`: documents in, resolved objects or diagnostics out" (spec/README.md:320), and LD1 puts a rule in the language "if it can be answered by reading the declarations alone" (docs/AUDIT.md:406). ENR-005 has layer `language` and binds what a machine provider reports at run time and what the engine's panel shows: nothing resolving can answer, so a Language-level engine cannot keep or test it. In the other direction three `convention` promises decide whether a document is accepted: PRV-002 ("makes the document that gives it rejected", spec/promises/providers.yaml:51-53), PRV-013 (a second machine provider "is not instantiated", though CON-025 and POOL-004, which also read an existing object's state, are `language`) and SEC-024 ("The template is rejected", spec/promises/secrets.yaml:608). Two engines that both claim the Language level may differ on whether a Resource document of a runtime-provider Type that gives `runtime` is valid, and the independent builds that LD4 has cover 'the language layer in full' would not be checked on it. Separately the README says of topic files that "two hold promises of the layer `interface`" (spec/README.md:268); four do: enrollment.yaml (ENR-003), packages.yaml (PKG-016), pending.yaml (PND-004) and secrets.yaml (SEC-006, SEC-019).

**Counterargument.** SEC-024 depends on how the engine is configured (DL17), which is why it is not `language`; ENR-005 mostly defines the form of evidence items, which is a document shape. No engine yet claims the Language level alone.

**Least change.** Set ENR-005 to `interface` (its form half could stay `language` as a split). Split PRV-002 and PRV-013 so that the rejection of the document is a `language` promise beside CON-025; keep SEC-024 as `convention` and say in the README's Conformance section that it is the one validity rule that depends on an engine's configuration. Correct README:268 to four.

**Precedent.** LD1 itself (docs/AUDIT.md:406); CON-025 and POOL-004 as the existing pattern for validity that reads an existing object's state.

**Who decides.** coordinator. Rulings 37 and 46 point 4: the layer field is wrong by LD1 (ruling 14); re-layering changes no behavior of the engine level.

**Verifier.** weakened, low. ENR-005: spec/promises/enrollment.yaml:105 `layer: language`, :108 "A machine provider reports evidence for enrollment", :111-113 verbatim; spec/README.md:61 has the language layer "answered by resolving ... and runs nothing". spec/README.md:268 "two hold promises of the layer `interface`"; four topic files do (enrollment ENR-003, packages PKG-016, pending PND-004, secrets SEC-006 and SEC-019; script). Already decided for the other three: docs/proposals/reaudit/round-5/DECISIONS.md:57 (M05) "SEC-024 takes the layer `convention`, as PRV-002 and PRV-013 already reject documents as conventions. Widening what resolving reads would change the Language level."; docs/proposals/ruling-43/README.md:207.

**Verifier's corrections.** PRV-002, PRV-013 and SEC-024 are known: M05 chose their layer on purpose, and ruling 58 point 1 accepted it; drop them. The situation given is wrong: two engines at the Language level do not differ, since neither is bound by a convention promise; the difference is between the Language and Engine levels, which M05 accepted. What survives is ENR-005's layer (drafted as language for the evidence shape, docs/proposals/spec-batch-1/areas/machines-and-trust.md:48, though its panel and provider sentences are not answered by resolving) and README:268's count. Coordinator.


## FR-160

**The convention layer no longer matches its own definition: 245 promises, 104 of them outside the seven conventions or step by step**

X01/F8 · needless-complexity · verifier: weakened, medium · decides: owner · new

Same fault, or overlapping: FR-145.

**Promises.** TPL-012, PRV-007, PRV-022, SEC-013, ENR-006, PND-013, PLC-017, PKG-032, SUB-009

**Where.** spec/README.md:62

**Quote.** "The few behaviors every part must agree on, stated as invariants. There are seven shared conventions: idempotency keys and outcomes; revision-checked change; access; declared against observed state; ownership and lifecycle; records; and that nothing starts model work or repeats an effect implicitly."

**Situation.** Grouped by what each of the 245 governs (every ID assigned once, checked by script). Within the seven named conventions, 141: outcomes, reasons, keys and repeats 23; reconciliation 8; verbs, controls and listing as operations 3; revisions and controls 14; the access decision 15; 'this permits nothing' restatements 6; delegation and authority 12; what inspection and refusals disclose 5; lifecycle states, retirement and removal 21; nested replacement, request forms and tombstones 11; Conduit state 8; declared against observed 5; records 6; nothing implicit 4. Outside them or procedural, 104: the setup and retirement procedure and hooks 16; the runtime-provider contract 16; the engine's own runner and built-in providers 5; machine and package-source providers and the provider call 5; secrets 15; enrollment and nodes 10; placement and machine loss 7; pending actions and sign-in 12; package installation and version skew 10; registration, replacement and first start 8. The seven procedure-heavy groups (setup, the three provider groups, secrets, enrollment, pending) hold 6,007 of the layer's 12,014 `then` words in 79 promises. 113 of the 245 live outside spec/conventions/. The layered claim says a convention is 'Specified as invariants, not step by step' (docs/AUDIT.md:382) and that other behavior goes to type contracts, a provider's contract or the conformance document; the implementation-defined list has 17 items. The layer grew from 179 promises on 2026-11-06 to 245 on 2026-11-12. An independent engine builder, told the shared conventions are few, must implement all of it: there is one built-in core package and no engine may leave its operations out.

**Counterargument.** LD1 also makes a rule a convention where two independently written parts would break 'a safety promise' by disagreeing, and an engine and a provider are such parts; ruling 40 put the provider contracts in the specification on purpose, and the README says so (spec/README.md:65-73). Much of the 104 is therefore placed where a ruling put it; what is false is the words 'few' and 'invariants', not necessarily the placement.

**Least change.** One question to the owner with three options. (a) Keep the content and correct the claim: README:62 and the tracker say the layer holds the seven conventions, Conduit's provider contracts and the behavior of the core package's own operations. (b) Keep the claim and move: what one core operation does goes to that operation's contract in `core/`, what a provider must do to its capability's contract, the own runner's choices to DL16, leaving invariants in `convention` (F9 to F13 list the IDs). (c) Besides (b), hand enrollment and relayed sign-in to providers. I recommend (b), and (c) only after S3 shows what enrollment needs.

**Precedent.** Kubernetes keeps three texts: the API conventions (short, invariants), each resource's API reference (what one kind does), and the CRI, CSI and CNI specifications (what a provider must do).

**Who decides.** owner. Ruling 13 (the layered claim) and ruling 14 (LD1, LD3, LD5) are the owner's; the choice changes scope and what an Engine-level claim covers.

**Verifier.** weakened, medium. spec/README.md:62 verbatim; docs/AUDIT.md:382 "Specified as invariants, not step by step"; :406 LD1. Recount: 245 convention promises, 113 outside spec/conventions/, 12,014 words of `then`, 39,577 of observable; 17 implementation-defined items. Growth by git: 434 promises at e271d1c7 (2026-11-06; convention 179), 454 at 2ca84f36 (2026-11-07; 208), 519 now (245); 64 added on 2026-11-11 and 12. Group sizes I could check agree (providers 26, secrets 15, pending 12, enrollment 9 plus PLC-026, setup 16). Against the count of 104: spec/README.md:65-73 "The provider contracts are not a fourth layer ... what the engine asks of a provider ... and what a provider must do are `convention`"; docs/AUDIT.md:395-400, ruling 40, "The contracts belong to the specification". Earlier record: docs/proposals/reaudit/round-6/design-critique.md:588-589 "'Few shared conventions' names seven topics, not a small behavioral specification."

**Verifier's corrections.** Known-new-situation: round 6 (D21) said it at 207 convention promises, and ruling 44 answered with reading paths only; the new situation is the 37 added since, nearly all procedural. 104 overstates what is outside the claim: the 26 provider and 15 secrets promises are placed there by README:65-73 and rulings 40, 41 and 43 and pass LD1's second test (engine and provider are independently written parts); about 63 remain, themselves features ruled by 49, 50, 56, 73 and 75. Part of the first day's growth (179 to 207) was relayering: language fell 220 to 202 in the same commit. Option (b) has no binding destination today: OBJ-018 (spec/promises/objects.yaml:307) gives `description` no meaning, spec/core/README.md:3-5 says "The promises are the source of truth", spec/definitions.md:404 calls a contract's prose what "no engine reads", and LD5 (docs/AUDIT.md:410) writes conventions "as promises with vectors"; (b) therefore needs the owner to change the form first. Owner decides, as stated.


## FR-161

**Group: pending actions and sign-in (12 promises) put a login protocol in the convention layer**

X01/F9 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** PND-005, PND-006, PND-010, PND-011, PND-013

**Where.** spec/promises/pending.yaml:238

**Quote.** "third, value is at most 8192 bytes and all printable ASCII, else malformed; fourth, for redirect, value parses once as an absolute URL with no user information, fragment or repeated parameter, else malformed"

**Situation.** The group is PND-001, 002, 003, 005, 006, 007, 008, 009, 010, 011, 012, 013: 1,017 words of `then`, 1,696 of observable, no vector. Five of them are step by step and matter only to Types that state `signIn`, which LD1 gives to a Type's contract: PND-013 (seven ordered checks on a pasted browser return, with reasons not-accepted, already-returned, malformed, address-mismatch, state-mismatch, issuer-mismatch; its observable has the engine rebuild the redirect query from code, state, iss and scope); PND-011 (compare the account ignoring ASCII case, undo a wrong account with a sign-out hook); PND-010 (which login may start, clashes between fixed ports); PND-005 (the evidence list of a sign-in action); and PND-006, which computes a 'setup action' from facts LIF-004 already puts in the instance's status. A login to a vendor's account is neither connection nor access (ruling 40). I would move PND-011, PND-013, PND-010's login and port clauses and PND-005's evidence list to the contracts of `sign-in-hook.v1`, `account-hook.v1` and `sign-out-hook.v1` and of the Agent Type's package, and cut PND-006. I would keep as conventions: PND-001 (the form), PND-002 (raised at Interfaces, never at an Agent), PND-003 and PND-009 (answering is ordinary access), PND-012 (records), PND-007 and PND-008 (which events raise one), and one invariant from PND-005: while a sign-in action is unanswered, no operation that starts model work is dispatched to that instance.

**Counterargument.** Rulings 49, 73 and 75 chose sign-in through the engine and the relayed return, because the person's browser is not on the Agent's machine and a model must never see the code; the checks in PND-013 keep a pasted value from reaching the wrong login, which a package cannot be trusted to do for itself. A convention is the only place that binds every engine to them.

**Least change.** Ask the owner whether the engine should relay returns at all in v0 or only show the URL and device code (RFC 8628) and leave the return to the Type's hook; either way move the five promises' procedure text to the hook operations' contracts and keep the invariants listed above.

**Precedent.** Kubernetes client-go credential plugins: the API fixes a small ExecCredential contract and the login flow belongs to the plugin. OAuth 2.0 Device Authorization Grant (RFC 8628) for a login on a machine without the person's browser; RFC 8252 and RFC 9207 for what a native app checks on a redirect.

**Who decides.** owner. Rulings 49, 73 and 75 point C ruled the feature; relocating or removing it is the owner's (tracker, 'Who decides what').

**Verifier.** weakened, low. spec/promises/pending.yaml:238-240 verbatim. Recount: 12 promises, 1,017 words of `then`, 1,696 of observable, no citing case. The feature and its place are ruled: IN_FORCE.md:188 "Sign-in is part of the language."; :192 "the browser login's return, PND-013 and IFC-049 (73, amended by 75 C1 to C7)"; docs/proposals/ruling-75/README.md:54-55 (C2) "The return goes to the Interface that received the sign-in", :73 (C6) "The engine relays nothing else", C7 "The account check stays strict"; ledger rows 49, 73 and 75 (2026-11-10 and 11).

**Verifier's corrections.** The question proposed for the owner (should the engine relay returns at all) re-asks rulings 73 and 75 C, made the day before the pinned commit, with no new situation beyond cost; use the counts as evidence under F8 and do not ask it separately. The move as written is unsafe in direction: PND-013's checks are the engine's and exist so a pasted value cannot reach the wrong login; a hook's contract is kept by the package, and a core contract's description does not bind (OBJ-018; spec/core/README.md:3-5). Cutting PND-006 removes part of ruling 50 point 3's one pending-action mechanism: owner.


## FR-162

**Group: machine enrollment (10 promises) specifies a join protocol step by step that ruling 40 gives to the machine provider**

X01/F10 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** ENR-001, ENR-002, ENR-004, ENR-006, ENR-007, ENR-008, ENR-009, ENR-012, PLC-026

**Where.** spec/promises/enrollment.yaml:141

**Quote.** "(7) Except for headless with acceptWeakerCheck, the provider reports exactly one fingerprint item for this machine. (8) Except for that same exception, the request carries fingerprint. When check 7 holds, it equals that item's value character for character."

**Situation.** The group is ENR-001, 002, 004, 006, 007, 008, 009, 010, 012 and PLC-026: 904 words of `then`, one of the ten cited by a vector, none in the safety catalogue. They fix what `enroll.v1` returns per operating system, single-use secrets, the kinds of evidence, eight admission checks, headless and `--accept-weaker-check`, expiry, refusal, and a removal question. The README says the machines 'and where an instance is placed on them' are a provider's (spec/README.md:33-36). The invariants that every engine and provider must share are four: no node represents a machine and nothing is placed on it until root admits it (first sentence of ENR-004; ENR-007's first two sentences); labels are fixed at admission (ENR-010); the join action is never recorded or relayed (ENR-003); joining or admission permits nothing (ENR-011, already ACC-001 and IFC-022). I would keep those and move the rest (ENR-001's evidence-kind checks, ENR-002, ENR-006 checks 3 to 8, ENR-008, ENR-009, ENR-012, PLC-026's naming rule) into the contracts of `enroll.v1`, `accept-machine.v1` and `remove-machine.v1` in spec/core/operations.yaml, which already describe them in prose. The engine bindings are still spec pending (IN_FORCE.md:224), so the move costs no built code.

**Counterargument.** Rulings 50, 56 and 68 point 3 put enrollment in the language and made the fingerprint comparison the engine's minimum, whatever the provider; admitting the wrong machine is an unsafe outcome, and LD1 makes a rule a convention where a disagreement between engine and provider would break a safety promise.

**Least change.** Ask the owner whether the engine's minimum can be stated as two invariants (root admits; the evidence shown includes one fingerprint or attestation that root confirmed) with the procedure in the machine-provider capability's contracts; if yes, move the listed promises there unchanged in wording.

**Precedent.** Kubernetes TLS bootstrapping: the API holds only the CertificateSigningRequest and its approval; tokens, join commands and what an approver checks belong to kubeadm and the approver. Tailscale's device approval is the same split.

**Who decides.** owner. Rulings 50, 56 and 68 ruled enrollment and its checks; moving them out of the convention layer changes what an Engine-level claim covers.

**Verifier.** weakened, low. spec/promises/enrollment.yaml:141-143 verbatim. Recount: 10 promises, 904 words of `then`; ENR-001 to ENR-012 absent from the safety catalogue; only PLC-026 cited by a case. Ruled in detail: docs/proposals/ruling-56/README.md, decision 2 "The minimum the engine enforces, whatever the provider ... at least one item of evidence is derived from the machine's key or from attestation", decisions 3, 4, 9, and 10 "A fingerprint of 6 to 8 characters"; ruling 68 point 9; IN_FORCE.md:220-223.

**Verifier's corrections.** ENR-005 and ENR-006's checks 5 to 8 are ruling 56's engine minimum; the proposed question re-asks its decision 2. The destination is wrong: ENR-006 binds the engine's `admit-machine.v1` (enrollment.yaml:129), while `enroll.v1`, `accept-machine.v1` and `remove-machine.v1` are the provider's operations (ENR-007, :171, "The engine calls accept-machine.v1"). Moving the checks there hands the engine's minimum to the provider; applied, it could admit a machine no person verified. Those three contracts' descriptions are 83, 44 and 37 words and do not "already describe" the checks. "Costs no built code" overlooks order 049a1, five rounds in and checkpointed (docs/guides/full-review.md:19-21). Fold into F8; owner.


## FR-163

**Group: provider contracts and core operations (26 promises) are written two to four times, and the copies already differ**

X01/F11 · needless-complexity · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** PRV-011, PRV-012, PRV-014, PRV-019, PRV-021, PRV-022, PRV-023, PRV-029, PRV-030, OBJ-018

**Where.** spec/core/operations.yaml:884

**Quote.** "Where a provider's contract promises to carry out every stop it still owes on a machine before starting any run there, a start that finds an owed stop fails, naming it, and may be retried once that stop is carried out (PRV-019, PRV-021)."

**Situation.** What a provider must do is stated as promises (PRV-011, 012, 014, 019, 021, 022, 025, 028, 030, ENR-002), again in the `metadata.description` of the core Operation contracts (run.v1 alone: 783 words, spec/core/operations.yaml:876-927; the 37 descriptions total 7,846 words), again in spec/core/README.md (8,944 words; its 'Providers' section at :393), and often a fourth time in the promise's observable. OBJ-018 says a description has no 'meaning to the language' (spec/promises/objects.yaml:307), so only the promise binds and the other copies can only drift. They have: the owed-stop rule quoted above is in run.v1's description, in PRV-021's observable (spec/promises/providers.yaml:661-664) and in core/README.md:418, and in no `then` (the brief's queued item); PRV-014 requires `id` and the state `pending` (spec/promises/providers.yaml:419-432) while the contract's schema has neither (spec/core/operations.yaml:1081-1085; known, spec pending 049b). The same triple statement holds for the management verbs (register.v1 577 words, inspect.v1 540, reconcile.v1 452). Within the group, PRV-023 (the six variables HOME, LOGNAME, PATH, SHELL, TERM, USER) and PRV-029 (a state directory of mode 0700) say how one built-in runtime provider starts a process, which the README gives to 'the provider that does it' (spec/README.md:77-80) and DL16 already half-holds.

**Counterargument.** The descriptions are what a provider author reads first and what a client shows a model as the tool's description, so they must be complete; the promises are what vectors cite. PRV-023 guards against the engine's environment leaking into a run, which is a safety rule every engine must share.

**Least change.** State each rule once. Keep the promise and cut each description to what the operation is for plus the promise IDs that govern it, with a checker that every ID cited exists; or, following CRI, make the Operation contract the one text and reduce the promise to 'a provider keeps the contract of `run.v1`'. For PRV-023 and PRV-029 keep one invariant (no variable of the engine's environment reaches a run except those its conformance document lists; SEC-020 has most of it) and move the lists to DL16.

**Precedent.** The Container Runtime Interface, which providers.yaml's head names as its model: one `api.proto` whose comments are the contract; the CSI specification likewise states each RPC's duties once.

**Who decides.** coordinator. Rulings 37 and 46 point 4 for single-sourcing, which adds no behavior; moving PRV-023's and PRV-029's lists to DL16 changes what binds every engine and goes to the owner with F8.

**Verifier.** weakened, low. spec/core/operations.yaml:884-886 verbatim. Recount: 37 descriptions, 7,846 words (run.v1 783, register.v1 577, inspect.v1 540, reconcile.v1 452); spec/core/README.md 8,944 words. The owed-stop rule is in spec/promises/providers.yaml:660-664 (observable), spec/core/README.md:418 and run.v1's description, and in no `then`. OBJ-018 at spec/promises/objects.yaml:307. Which copy governs is decided: spec/core/README.md:3-5 "The promises are the source of truth: where a document here and a promise differ, the document is wrong." Earlier record: docs/proposals/reaudit/round-6/design-critique.md:594-595 "Keep one normative source for each rule; avoid three paraphrases that each acquire exceptions".

**Verifier's corrections.** No new drift is shown: both differences cited are known (the owed-stop clause is queued, docs/guides/full-review.md:132-134; PRV-014 against the schema is spec pending 049b, IN_FORCE.md:224), and the owed-stop copies agree with each other. The duplication and its size are real and partly recorded (round 6, D21). The CRI-style alternative (the contract as the one text) conflicts with OBJ-018 and the core README and is the owner's. Moving PRV-023's list to DL16 is unsafe in direction: PRV-023 (providers.yaml:747-750) admits further variables only from "a list set by the user"; an invariant keyed to what an engine's conformance document lists would let an engine document any variable, a token among them, and still keep the promise. Trimming descriptions needs care: a Type's description is carried into tool descriptions (spec/promises/contracts.yaml:362-364).


## FR-164

**Group: the setup and retirement procedure (16 promises) is step by step, and starting a stopped run to serve its retire hook costs six promises**

X01/F12 · needless-complexity · verifier: weakened, low · decides: owner · new

**Promises.** TPL-009, TPL-012, TPL-018, CON-004, POOL-013, PLC-027, ACC-039, SEC-011, SEC-013

**Where.** spec/promises/templates.yaml:304

**Quote.** "Where the hook is to be dispatched, its implementation is an item that is run and its runtime provider reports the run `stopped`, the engine first asks for the run, under PRV-007"

**Situation.** The group is TPL-009, 012, 017, 018, CON-004, POOL-007, 013, 015, ACC-018, OPS-021, 022, 028, LIF-025, 027, PLC-019, 027: 1,138 words of `then`. Nine are invariants and fit the claim (OPS-021, OPS-022, OPS-028, ACC-018, LIF-025, LIF-027, POOL-007, POOL-015, TPL-017). Five are a procedure: TPL-012 ("The setup is these parts, in this order: first ... then ... then ... then", spec/promises/templates.yaml:211), TPL-009, TPL-018, CON-004 and POOL-013; a builder assembles one algorithm from them plus ACC-005, PRV-007, IFC-041, SEC-010, SEC-013, PND-005 and PND-011. The costliest step is the quoted one: to retire an instance whose run has stopped, the engine starts the run again (reading its secrets again), dispatches the retire hook, then stops it. That one choice is why these exist or carry extra clauses: PLC-027 (no hook and no run where the node is lost), ACC-039 (a retired consumer's Conduit still serves its retirement, and the engine's reads for it), SEC-011 and SEC-013 (reads and connections that belong to a retirement), TPL-009's 'apart from the run or the connection that TPL-018 asks for', and CON-008's rule that a retired consumer's Conduits stay until its shutdown is confirmed.

**Counterargument.** A retire hook that releases something outside (a lease, a registration) must run even if the process crashed earlier, or the release is lost silently; the order of parts is what an author's hook relies on, and Kubernetes does fix the order of init containers and lifecycle hooks. The parts are also what OPS-022 composes an outcome from.

**Least change.** Write the order once, in the contracts of `set-up.v1` and `retire.v1`, and restate TPL-012, TPL-009 and TPL-018 as invariants (a setup hook is dispatched only once every Conduit not marked optional is open and the run is reported running; a retirement that succeeded has asked for the stop). Ask the owner whether a retire hook is dispatched only where the run is running, with a part that reports unavailable otherwise, which would remove the start-to-retire path and the clauses that serve it.

**Precedent.** Kubernetes `preStop` and systemd `ExecStop=` run only for a workload that is running; cleanup that must happen regardless is a finalizer, held by something that is itself running.

**Who decides.** owner. The behavior of retirement was ruled (ruling 41 and its items on hooks; ruling 42's answers on runs); dropping the start-to-retire path removes behavior.

**Verifier.** weakened, low. spec/promises/templates.yaml:304-306 verbatim; TPL-012's ordered parts at :211-216. Recount: 16 promises, 1,138 words of `then`. The start-to-retire path is ruled: docs/proposals/ruling-41/README.md:61 (P4) "a retirement asks for one only where a retire hook is selected and the run is not running (4c)", accepted with "Accept all with p2 changed" (ledger row 41); docs/proposals/reaudit/round-5/DECISIONS.md:86 (M29) kept it.

**Verifier's corrections.** The cost is overstated. ACC-039 (spec/conventions/access.yaml:773-776) permits "an onward request that serves the consumer's retire hook" whichever way the run came to be running; CON-008's rule that a retired consumer's Conduits last until shutdown is confirmed, and PLC-027's no hook on a lost node, are needed by any retire hook. Dropping the path removes clauses of SEC-011, SEC-013, ACC-039, TPL-009 and PLC-027, and no whole promise. Writing the order into the descriptions of `set-up.v1` and `retire.v1` would not bind (OBJ-018). Reopens ruling 41 P4 with no new situation; evidence for F8 only. Owner.


## FR-165

**Group: eight promises restate another promise's refusal and add no obligation**

X01/F13 · needless-complexity · verifier: weakened, low · decides: coordinator · new

**Promises.** ACC-023, ACC-028, FLEET-004, PLC-012, ENR-011, PRV-018, LIF-022, ACC-024

**Where.** spec/conventions/lifecycle.yaml:707

**Quote.** "It reports refused whoever makes it, the root Interface included, although an open Conduit of the actor lists the operation: refused means that the request is not permitted, whoever asks. LIF-022 is why."

**Situation.** LIF-022 (must not: a first-start object 'is retired or unregistered') and LIF-034 (must: the same request 'is not dispatched, and reports refused') have the same `when` and one obligation; each cites the other. ACC-024 is the `must not` of a clause that ACC-012's `then` already states. ACC-023 (a new Interface 'has no access until a Conduit ... is opened'), ACC-028 and FLEET-004 ('That ownership permits any request', must not) and PLC-012 (sharing a node) say only that ACC-001 has no exception; ACC-056's table gives each the cause '(ACC-001)'. PRV-018 repeats IFC-022's words ('not from the address, machine, network, runtime provider or run it came from', spec/interface/implementation.yaml:597) and ENR-011 repeats both. The form's rule 3 ("No sentence of the form 'this does not imply'. State the correct behavior as its own promise", spec/README.md:299) produces them. Cost: eight IDs, six safety-catalogue rows that count one safeguard twice, one vector between them (FLEET-004), and the ruleId ambiguity of F6. Several carry real rules only in their observable (FLEET-004: a Fleet is never a caller and a bound naming a Fleet covers that Fleet alone), which F1 would move.

**Counterargument.** Each names a place where a reader might expect an exception (ownership, a shared machine, network membership), and a separate ID lets a safety vector target that expectation. The catalogue says it 'counts overlapping safeguards separately' on purpose.

**Least change.** Cut the eight; put each sentence in the observable of the promise it restates (ACC-001, ACC-012, IFC-022, LIF-034) as a named case a vector can cite, and move FLEET-004's observable-only rules to FLEET-001 or ACC-029. Reword rule 3: "State the behavior that holds as a case of the promise that decides it."

**Precedent.** SARIF's one rule per result; Kubernetes RBAC states 'permissions are purely additive (there are no deny rules)' once and does not repeat it per resource.

**Who decides.** coordinator. Rulings 37 and 46 point 4: removing a restatement removes no behavior; logged for the owner to strike.

**Verifier.** weakened, low. spec/conventions/lifecycle.yaml:706-708 verbatim. LIF-022 (:460-472) and LIF-034 (:700-713) have the same `when` and cite each other. ACC-012's `then`, spec/conventions/access.yaml:213-215, states ACC-024's rule. PRV-018 (spec/promises/providers.yaml:552-553) repeats IFC-022 (spec/interface/implementation.yaml:596-598). spec/README.md:299, rule 3. Against the stated cost: the manifest's tags cite ACC-024 in four cases (access-bounds, first-start-root-system, close-refused, root-closes-own) and FLEET-004 in two; seven of the eight have a safety-catalogue row (ENR-011 has none). No earlier record of the point found.

**Verifier's corrections.** "One vector between them (FLEET-004)" is wrong: six cases cite two of the eight. ACC-024 is not a bare restatement: ACC-012 sends the reader to it ("ACC-024 adds the condition") and it alone holds the exact `when` (root's declared `management`, `access` and `system` Conduits and FLEET-005 defaults). The clear duplicates are LIF-022 with LIF-034, PRV-018 with IFC-022, and ENR-011. The eight exist because rule 3 of the form asks for them, so the fix is to the form; cutting IDs also touches IN_FORCE.md:25 (ACC-023 for preservation test 8), the catalogue and G5's standings. Coordinator, logged to strike.


## FR-166

**No promise decides the state directory under a runtime provider other than the engine's own runner**

X01/F14 · gap · verifier: weakened, medium · decides: coordinator · new

**Promises.** PRV-029, MAN-018, CTR-004, PRV-007

**Where.** spec/promises/providers.yaml:979

**Quote.** "Another runtime provider supplies such a directory only where its contract states it does. A Type whose implementation needs one states that requirement; setup under a runner that gives none fails with an explicit diagnostic."

**Situation.** PRV-029's `then` binds only 'the engine's own runner'. The sentence quoted is in its observable and cannot be followed: CTR-004 gives a Type contract no key in which to state that its implementation needs a state directory; `run.v1` has no input for one (spec/core/operations.yaml:928-975); MAN-018 forbids a template to bind `CONDUIT_STATE_DIR` itself (spec/promises/manifest.yaml:399-408); and the engine has no way to learn that a runner 'gives none', so it cannot fail the setup or write the diagnostic. Situation (S3): an Agent whose `runtime` names a runtime provider on a second machine. Whether its run gets a private directory that persists, what happens to it when the instance is unregistered, and what the engine reports if there is none are undecided. The two shipped providers already answer differently from PRV-029: the engine's runner removes the directory with the instance (spec/promises/providers.yaml:975), the SSH provider's contract says 'kept across starts and never removed' (packages/provider-ssh-runtime/contracts.yaml:42-43). PRV-029's observable says Agent packages use the directory for the Agent's own home, so an Agent that gets none falls back to the run user's home, shared with every other Agent run by that user, against the purpose's 'Private and shared stay separate'.

**Counterargument.** The README already states that one operating-system user reaches every state directory, so isolation is mediated in v0 whatever the provider does; the separate homes are order 050, not yet written, and that order could settle it. Both shipped providers do supply the directory.

**Least change.** Make it every runtime provider's duty, in a `then`: "A runtime provider gives each run a directory that is the same at every later run of that instance's ID and that no run of another ID is given, names it in `CONDUIT_STATE_DIR`, and its contract states when it removes it." Delete the two observable sentences. If a provider may decline, add a key by which a Type states the need and have `run.v1` end failed, naming it.

**Precedent.** Nomad task drivers, named in providers.yaml's head as a model: every driver gives a task `NOMAD_TASK_DIR` and `NOMAD_ALLOC_DIR`. systemd's `StateDirectory=` and the kubelet's volume mounts through CRI are the same duty placed on the runtime.

**Who decides.** coordinator. Ruling 71 point 4: a small finding that adds a feature (it cannot move S1's author counts), logged for the owner to strike; otherwise the owner's with order 050.

**Verifier.** weakened, medium. spec/promises/providers.yaml:979-981 verbatim; the `then` (:969-975) binds only "The engine's own runner"; spec/definitions.md:279 defines the state directory as "The engine runner's private persistent directory". CTR-004 (spec/promises/contracts.yaml:98-105) lists no key for the need; MAN-018 (spec/promises/manifest.yaml:402-406) reserves the variable. packages/provider-ssh-runtime/contracts.yaml:42-43 "kept across starts and never removed (PRV-029)"; packages/provider-local-process/contracts.yaml:17-18 "reused across starts and never removed". LIF-035's observable, spec/conventions/lifecycle.yaml:732-733, says without condition "Removing or unregistering an instance removes its state directory and contents under PRV-029". Not in the known-open lists; IN_FORCE.md:187 leaves only "Agent client-specific homes and login choices" to order 050.

**Verifier's corrections.** Two parts are decided by text the reviewer missed. A Type can state the need: spec/definitions.md:404 "Where a promise says that such a contract states something that no key of a contract document holds, it means this prose". Whether another provider supplies a directory is its contract's matter (:979-980), and both packaged providers do. What stays undecided: who fails the setup and how the engine could write "an explicit diagnostic" from prose that "no engine reads"; and removal, where LIF-035's sentence promises it on unregistration while both packaged providers never remove, so an unregistered Agent's home, its login included, stays on the machine. Who decides: qualifying LIF-035's sentence to the engine's runner and striking or assigning the diagnostic sentence adds no behavior (coordinator); a new duty on every runtime provider is a feature, ruling 71 point 4 at most, and fits better in order 050's pending text. Unsafe stays false: the README's limit already concedes one operating-system user reads every state directory.


## Further points the verifier noticed

Not verified by a second reader.

- **LIF-035 promises removal of the state directory for every instance; both packaged runtime providers never remove it** (spec/conventions/lifecycle.yaml:732). "Removing or unregistering an instance removes its state directory and contents under PRV-029" is unconditional, but PRV-029's `then` binds only the engine's own runner. packages/provider-ssh-runtime/contracts.yaml:42-43 and packages/provider-local-process/contracts.yaml:17-18 both say "never removed", the first citing PRV-029. An Agent's home and login therefore outlive its unregistration under either provider, and the README's limit (spec/README.md:424-425) speaks only of the engine's machine. The reviewer cited the SSH contract but not LIF-035's sentence or the local-process provider.
- **ACC-024 and LIF-022/LIF-034 name different promises for the same refusal** (spec/conventions/access.yaml:428). "reports refused under ACC-012" against spec/conventions/lifecycle.yaml:471 and :711-712 "refused under ACC-024". The reviewer used this as a case of ambiguity in F6; it is a direct contradiction between observables and can be fixed on its own, whatever is done with ACC-056's table.
- **ENR-006's `then` names no reason for a request that is not dispatched** (spec/promises/enrollment.yaml:131). "The request is dispatched only if all applicable checks hold." The reasons (invalid for check 8, conflict for the rest, unavailable where the provider is not ready) are only in the observable (:148-162). CONTRIBUTING.md:126-127, rule 5, asks that every such promise name exactly one reason. It is the clearest single instance of F1 and breaks a written rule of the repository's own.
- **No binding place exists for engine-kept behavior outside a promise, which the fixes of F8(b), F9, F10, F11 and F12 assume** (spec/core/README.md:3). "The promises are the source of truth: where a document here and a promise differ, the document is wrong"; OBJ-018 (spec/promises/objects.yaml:307) gives `description` no meaning; spec/definitions.md:404 calls a contract's prose what "no engine reads". Moving a rule the engine must keep from a promise into a core Operation contract removes it from what binds. Any plan to shrink the convention layer by relocation needs the owner to change the form first (LD5, docs/AUDIT.md:410).

## The verifier's notes

Checked at 2eb8f258 (2026-11-12), read only, with /usr/bin/python3 and read-only git. Nothing was built or run.

**Recounts.** The reviewer's numbers hold. 519 promises (226 language, 245 convention, 48 interface; 466 must, 48 must not, 2 should, 3 may), and every per-file count matches the README. The manifest has 494 cases citing 292 promises, and 155 of the 394 safety rows have no vector. The 17 promises absent from the safety catalogue are exactly the ones the brief names. Growth by git is 434 (2026-11-06), 454 (2026-11-07) and 519 now. The ledger has 77 rulings, dated 37, 6, 1, 3, 20, 9 and 1. Every quote is verbatim at its cited line. I did not verify "48 rulings since amended" or the full assignment of all 245 convention promises to groups; the group sizes I could check agree.

**Outcome.** Nothing is refuted outright and nothing is unsafe as found.
- Confirmed: F1 (medium), F6 (medium), F3 (low), F5 (low).
- Weakened: F8 and F14 (medium); F2, F4, F7, F9, F10, F11, F12, F13 (low).

**Three patterns in what was weakened.**
1. Earlier records the reviewer did not cite. Round 6's critique (D21) already recorded F1's point, F8's and F11's. The docs audit logged examples in place of a list for the Implementation level (F2). Round 5's M05 chose the layers of PRV-002, PRV-013 and SEC-024 (F7). Ruling 44's D18 made ACC-033 one table (F3).
2. F9, F10 and F12 reopen rulings 73 and 75 C, 56 and 41 P4 with cost as the only new fact. They are better used as evidence under F8's single question to the owner than as questions of their own.
3. Several proposed fixes move engine obligations into contract descriptions, which do not bind. For F10 the move also goes to the wrong party's operations, and for F11 moving PRV-023's list to DL16 loosens it. Those two, if applied as written, would weaken a safety rule.

**The reviewer's assessment.** "Growing, not converging" stands on the counts. "The convention layer has outgrown the claim" stands for the README's one-line description, but the figure of 104 promises outside it is nearer 63 once the provider and secrets promises that the README and rulings 40, 41 and 43 place there are set aside.
