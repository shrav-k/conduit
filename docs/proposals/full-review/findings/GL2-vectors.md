# GL2-vectors: findings

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the language), at commit `2eb8f258`: what one reader returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The reader's assessment.** The count stands: 494 cases (225 under language/, 236 under conventions/, 33 under interface/), 1,717 steps. The failure after 042's rebase was a fault of the merge, not of the text and not of the vector: order 070 had replaced the whole-state clone used for the pre-dispatch trial of `register-replacement.v1` with a field list ending `..State::default()` (engine/crates/core/src/requests.rs:1066-1082, `git blame`), 042's new tombstone map was not in that list, so the trial saw no tombstone and the request was dispatched; LIF-041 and LIF-009 both state the conflict, the vector expects exactly that (conformance/conventions/inventory/e4/tombstone-register-replacement/case.yaml:704-723), and the fix is the one line at requests.rs:1070. It was not a language case: its three promises are all of the convention layer. Of the 60 sampled cases, 28 follow from `when` and `then` alone with nothing else asserted (a); 16 follow from `when` and `then` in their main expectation but carry a secondary expectation or a tag outside it; 12 follow only from an observable (b); 1 from an implementation-defined choice (c: DL10, with DL9, DL15 and DL8 as secondary pins in three others); 3 from none (d: node-readmit's LIF-041 step, duplicate-exports for PKG-012, schema-applicator-defaults). 51 of the 60 would fail if the clause they are named for were broken; 9 cite at least one promise whose breach they would not detect. As evidence the vectors are strong where a case submits documents and expects a verdict, which is most of language/ and interface/, and they caught a real regression in 042. They are weak in three ways: 59 cases assert error forms that IFC-025, IFC-050 and ACC-056 forbid, so for those promises a pass is evidence against conformance; tags overstate coverage (99 of 292 cited promises are named in no step, and six single-citing cases lack the promise's situation); and the suite cannot see what it never builds, as the `ownTools` fault shows. In four places the vectors already act as a second specification for the next builder: observables, exact wording, unlabelled implementation-defined choices, and the first build's internals.

**What the reader read.** Tree 2eb8f258, read only; nothing built or run; all counts are from one-off read-only scripts over the 494 `case.yaml` files (all parse as JSON; PyYAML rejects one, language/loader/strings, for a raw U+FFFF). Sampling rule. conventions/ (20): the three 042 cases (e4/tombstone-register-replacement, e4/inventory-reading, e4/node-readmit-no-tombstone); access/bounds, one case of 17 observations; all six OPS-014 cases; the two REV-010 cases that also hold a single-cited promise (e3b/reconcile-array-element, e3b/reconcile-control); two more LIF-041 cases (e4/tombstone-second-key, e4/tombstone-order); then one case from each second-level directory not yet represented, the alphabetically first holding a single-cited promise, else the first: changes/retirement, first-start/initial, list/from-first-start, records/authority, s1/whole, setup/pool-entry. language/ (20): quotas by directory size with at least one each (archives 3, contracts 1, first-start 1, fleets 1, loader 2, packages 5, registration 7), cases holding a single-cited promise first, alphabetical, every second one in registration. interface/ (20 of 33): the 17 even positions of the alphabetical list, which hold both cases whose promise only they cite, plus the two SCH-019 cases and the one case tagged language alone. No case cites ACC-056, PND-005 or SEC-013, so none could be sampled; ACC-056's absence is answered in F2. Read in full: docs/guides/full-review.md; AGENTS.md, What holds; order 042 and its Record; conformance/README.md lines 1-553; conformance/profiles/first-build.yaml; spec/implementation-defined.yaml DL1 to DL12 and DL15; the `when`, `then` and `observable` of about 75 promises the sample cites; every step's input and expectation in the 60 cases, with fixtures where an expectation depended on them. Not read in full: the `evidence`, `records` and `state` arrays of the sampled cases, which I pruned and checked only by key and count, except first-start/initial's; s1/whole, where I read steps 38 to 59 in outline and checked the ten single-cited promises, not all 59 steps against 67 promises; records/authority, setup/pool-entry and four interface cases (e05-core-replace, schema-literal-data, schema-pattern-property-ref, schema-reference-kind), classed (a) from their expectations and the promise's `then` without tracing every fixture. Of the engine I read only engine/crates/core/src/requests.rs:1040-1130, registration.rs:2536-2554, inventory.rs:308-345 and engine/crates/conformance/src/access.rs:30-80 and lib.rs:319-350, 965-971. The other 434 cases were reached only by the suite-wide counts. The pre-rebase state of 042 is not in git (the branch was squashed; `git diff e33c121f 57813f06` is empty), so the attribution of the failure rests on `git blame` of requests.rs:1064-1070, order 070's diff (`git show 1a34ccf9`) and the merge commit's message. Not read: conformance/manifest.yaml beyond its keys and a comparison of promise lists (no mismatch in 494), s1-promises.yaml, s1-safety-cases.md, README.md lines 554-727, HANDOVER.md beyond its row for 042, and the model sources. Checked against the first pass: L12/F1, L12/F3, L13/F1, X01/F1, X01/F5, X03/F8, X03/F9, X07/F1, X07/F8, L03/F4, L07/F10 and L04/F10 cover parts of F2 to F4 and F10, marked known-new-situation where so; I did not re-report the keyword-location pin of L12/F1 or the SUB-005 ruleId of L07/F10, though sign-in-logins, own-tools and conduit-as-invalid show both.


## FR-546

**No vector holds an Agent whose Type states `toolsDirectory`; the first build can never report `stated: true`, and all 494 vectors pass**

GL2/F1 · gap · verifier: confirmed, medium · decides: coordinator · new

**Promises.** LIF-013, CTR-014, ACC-027

**Where.** conformance/conventions/inventory/e4/inventory/case.yaml:563; spec/conventions/lifecycle.yaml:266

**Quote.** "The inventory holds `ownTools` for every Agent and no other instance. Its required `stated` says whether the Agent's Type states `toolsDirectory` under CTR-014."

**Situation.** A gap of the vectors, not of the text: the promise decides the case. The only expectation on `ownTools` in the suite is `{"stated": false}` (e4/inventory/case.yaml:563-565), for an Agent whose Type states nothing. The only fixture that states `toolsDirectory` is conformance/language/loader/own-tools/agent.yaml, which is loaded and never registered or inspected. The schema makes the member an object, `{directory: &lt;pointer>}` (spec/schemas/contracts.yaml:198-203). The first build reads it as a string: `ty["spec"]\["toolsDirectory"].as_str()` (engine/crates/core/src/inventory.rs:320), which is None for every document the loader accepts, so inspection answers `stated: false` for every Agent, and `targetOwnTools` likewise (inventory.rs:363). The engine therefore reports that a Type declares nothing about its own tools' reach when it declares it, which is the report ruling 75 B1 and B2 asked for, and the suite cannot tell. The same single case is CTR-014's only citation (loader/own-tools/case.yaml:7) and tests only the schema's closed shape and the family: the required path, the string-or-null place and the ban on a control that reaches the place have no vector, and no source under engine/ or tools/ names CTR-014 (`grep -rn CTR-014 engine tools`: nothing).

**Counterargument.** No package under packages/ or experiments/ states `toolsDirectory` at this commit (`grep -rl toolsDirectory packages experiments`: nothing), so no user is misinformed today; the Agent packages that will state it (orders 050a, 076, 078) are unmerged. Order 075 changed no engine code by design, and 042's scope was the inventory's form.

**Least change.** Three vectors and a one-line fix. (1) An Agent whose Type states `toolsDirectory: {directory: /directory}` with a string there: inspect expects `stated: true`, `pointer`, `directoryNamed: true`, `directory`, `enforcement: neither`; the same with null expects `directoryNamed: false`; a holder of an open Conduit listing a `startsModelWork` operation expects `targetOwnTools`. (2) Three registrations rejected under CTR-014: place not required, place not string-or-null, a control whose `changes` reaches it. (3) Read `toolsDirectory.directory` in inventory.rs. Do it before 050a merges.

**Precedent.** Kubernetes conformance tests read each status field back through the API after setting what produces it; mutation testing (action item A44) is the general check that a vector can fail.

**Who decides.** coordinator. A finding of the first build, fixed with a vector (ruling 46 point 4); it adds no behavior (ruling 37).

**Verifier.** confirmed, medium. Quote verbatim at spec/conventions/lifecycle.yaml:266-267. The schema makes the member an object: spec/schemas/contracts.yaml:198-203 (`toolsDirectory: type: object, properties: {directory: pointer.v1}, required: [directory]`). The first build reads it as a string: engine/crates/core/src/inventory.rs:320 `let Some(pointer) = ty["spec"]\["toolsDirectory"].as_str() else { return Ok(json!({"stated":false})); };`. Any Type the loader accepts holds an object there, so every Agent reports `stated: false`, and so does `targetOwnTools` (inventory.rs:363). `git blame` attributes lines 313-330 to 57813f06 (order 042). The only `ownTools` expectation in the suite is conformance/conventions/inventory/e4/inventory/case.yaml:563-565 (`stated: false`). `grep -rn '"stated":true'` over engine and conformance finds only inventory.rs:324. `grep -rn CTR-014 engine tools` finds nothing. The tool column for CTR-014's registration checks is `—` (spec/schemas/README.md:204). Order 075's Record says "The order changes no engine code" (docs/work-orders/075-spec-own-tools-reach.md:637), and that order says the admin adds `toolsDirectory: {directory: /directory}` to both `.v2` Types on the 050a/076 branch (same file, :56-58). That is when the wrong report starts to matter. Not in the first pass: X04/F9 is about the mild report for a Type that states nothing, not about this reading. Not in docs/AUDIT.md "Known to be open" or IN_FORCE.md:199.

**Verifier's corrections.** Three fixtures state `toolsDirectory`, not one: conformance/language/loader/own-tools/agent.yaml:25-27, resource.yaml:25-27 and limits.yaml (the last two are rejected). None is registered or inspected, so the conclusion stands. It is not unsafe under the review's test, because the engine that reports wrongly breaks LIF-013. Severity medium today, since no package states the member (`grep -rl toolsDirectory packages experiments` finds nothing). It becomes high the moment the `.v2` Agent Types add it, because the report ruling 75 B1/B2 asked for would then say the reverse of the truth for Agents whose own tools run unconfined.


## FR-547

**59 cases expect error results in a form IFC-025, IFC-050 and ACC-056 no longer allow: no diagnostic, or a text block of one bare word**

GL2/F2 · contradiction · verifier: weakened, medium · decides: coordinator · known-new-situation

**Promises.** IFC-025, IFC-050, ACC-056

**Where.** conformance/conventions/requests/reconcile-false/case.yaml:1546; spec/interface/implementation.yaml:670

**Quote.** "Where its `isError` is true, its `content` holds first one text block that states its reason or outcome and each error diagnostic's message, under IFC-050, disclosing only what the rest of the result says."

**Situation.** Counted over all 494 cases with a one-off script. 60 steps in 39 cases pin the first text block to exactly `refused`, `invalid`, `conflict`, `unavailable`, `failed` or `unknown` (reconcile-false/case.yaml:1546 beside an OPS-030 diagnostic whose message the block must state; :1631). 51 steps in 38 cases assert that a result ending refused, failed or unknown has no `structuredContent` at all (reconcile-false/case.yaml:1673; conventions/records/expected-id/case.yaml:684-689), against IFC-050's observable (implementation.yaml:1343-1345: every reason, and an engine-decided failed or unknown outcome, carries a diagnostic). 39 steps in 16 cases assert a core reply with a reason and no diagnostics, or `diagnostics: []` (conventions/access/bounds/case.yaml:1552; language/archives/unavailable/case.yaml:28-29; language/archives/s1/case.yaml:365-366). The runner itself rejects a refused or unavailable access reply that holds anything but `reason` and `restriction` (engine/crates/conformance/src/access.rs:52-63; conformance/README.md:316). Of 112 steps that expect refused, none asserts ACC-056's diagnostic; no case cites ACC-056; 25 of the 59 cases cite IFC-025 and expect the opposite of its `then`. A builder who keeps the three promises fails 59 cases; the first build passes them by breaking the promises. For these promises a green suite is evidence against conformance.

**Counterargument.** The build of ruling 52 is order 046, not yet written, and its plan foresees about 45 changed cases (docs/proposals/spec-batch-1/README.md:545-554); the cold readers' guide warns not to read a written promise as built. AGENTS.md lets vectors lag only where the specification is pending, and here it is written (IN_FORCE.md:142).

**Least change.** Before 046: delete the negative assertions (`absent` of `structuredContent` or `diagnostics`, `diagnostics: []`, the bare-word text) from the 59 cases, which keeps every reason and outcome asserted and stops the suite from requiring the old form; list the 59 in the manifest as awaiting 046. 046 then adds the positive assertions. Correct README.md:316 and the runner's allow-list with it.

**Precedent.** Web Platform Tests and pytest's xfail record a known failure as metadata beside the test; neither rewrites the expectation to the failing behavior.

**Who decides.** coordinator. Vectors follow spec/ (AGENTS.md, What holds, point 2); a correction that adds no behavior (rulings 37 and 46 point 4).

**Verifier.** weakened, medium. Counts reproduced exactly with a one-off script over all 494 case.yaml: 60 steps in 39 cases pin the first text block to a bare reason or outcome word; 51 steps in 38 cases list `/reply/structuredContent` in `absent` for a refused, failed or unknown result; 39 steps in 16 cases expect a reply with a `reason` and `diagnostics: []` or `/reply/diagnostics` absent; the union is 59 cases; 25 of them cite IFC-025; ACC-056 and IFC-050 are cited by no case. The runner compares strings exactly and arrays by exact length (engine/crates/conformance/src/lib.rs:317-349). It rejects any refused or unavailable access reply holding more than `reason` and `restriction` (engine/crates/conformance/src/access.rs:52-63; conformance/README.md:315-316). ACC-056's `then` requires "one error diagnostic, whose ruleId is the refusing promise" (spec/conventions/access.yaml:915). IFC-025's quote is verbatim at spec/interface/implementation.yaml:670-673. 21 steps directly break IFC-025's `then`: they hold a diagnostic and still pin a bare word, e.g. conformance/conventions/requests/reconcile-false/case.yaml:1546 beside OPS-030, plus key-scope, reason-order and reconcile-true.

**Verifier's corrections.** Class: not a contradiction. Vectors bind nothing (AGENTS.md, What holds), and the promises agree with each other. This is a defect of the non-binding vectors: the lag expected before order 046, which builds ruling 52. 046's plan already counts "about 45 cases (about 85 steps) changed" and "the runner: the new assertions" (docs/proposals/spec-batch-1/README.md:545-554). The first pass already notes that IFC-050 has no case and that 046 builds it (P07 I7's verifier). What is new, and should go to 046's order, is that the negative assertions (`absent`, `[]`, the exact bare word) and the runner's allow-list make a promise-keeping engine fail. A rewrite that only adds positive assertions would leave that in place. Overcount: 7 of the 51 no-structuredContent steps are failed outcomes an implementation gave (aware-failed, read-is-error, read-capability-error, read-invalid-parameters, read-parameter-headers, failed-read-repeat, cancel-confirmed), for which IFC-050's observable requires no diagnostic (spec/interface/implementation.yaml:1344-1345 limits it to an "engine-decided failed or unknown outcome"). Refused steps counted by any mention: 125 by my method, against the stated 112; none asserts a refusal.v1 diagnostic.


## FR-548

**`requests-restart-interrupted` expects the engine to ignore a late complete result and then accept that the effect did not occur**

GL2/F3 · contradiction · unsafe · verifier: known, high · decides: coordinator · known-new-situation

**Promises.** OPS-014, OPS-013, OPS-018, OPS-024, OPS-030

**Where.** conformance/conventions/requests/restart-interrupted/case.yaml:1606; spec/conventions/operations.yaml:352

**Quote.** "An answer that ends an attempt's call at the implementation fences it: a result, an error or a JSON-RPC error, including one that arrives after the outcome became unknown, which changes no outcome under OPS-013."

**Situation.** Steps to the harm, as the vector expects them. Step 2: an Agent's keyed `file.write` to a plain server is dispatched. Step 3: the engine restarts; the attempt is unknown. Step 5 (case.yaml:1606-1622): the server's answer arrives late, a complete result that is not an error (`"result": {"content": []}`), and the expected reply is `"ignored": true`. Step 6 (:1642-1693): root reconciles with `occurred: false`; expected outcome succeeded and a record `"outcome": "failed"`. The sibling case shows what follows a recorded failure: the same key is dispatched again as attempt 2 (conventions/requests/reconcile-false/case.yaml:1884). The write lands twice for one key, against OPS-018, while the engine held the answer that said it had landed. X07/F1 records the fault in the text and cites `reconcile-fenced`, where the late answer is an error; this case is the stronger one, a late success, and nothing in the first pass names it. The vector is faithful to OPS-014 as written; the consequence is that an engine which refuses step 6, the safe behavior and X07/F1's proposed fix, fails conformance today.

**Counterargument.** The text says exactly this, so the vector is right about the text. A reconciliation is the actor's statement, and an actor that checks after the fence sees the file.

**Least change.** With X07/F1's fix to OPS-014 and OPS-030: change step 6 to expect conflict under OPS-030 with a diagnostic saying the implementation answered that the call completed, add a step where `occurred: true` succeeds, and add the repeat with the key expecting no second dispatch. Change `reconcile-fenced` only in its title.

**Precedent.** Stripe's idempotency keys and the IETF Idempotency-Key draft treat the server's stored answer for a key as the authority over a client's belief.

**Who decides.** coordinator. As X07/F1: a small refusal that closes an unsafe contradiction (ruling 71 point 4), logged for the owner to strike.

**Verifier.** known, high. The vector is as described: conformance/conventions/requests/restart-interrupted/case.yaml step 5 completes the attempt with `{"kind":"result","result":{"content":[]}}` and expects `"ignored": true` (:1606-1622). Step 6, root's `reconcile.v1` with `occurred: false` (requests.json index 3), succeeds and the invocation is recorded failed (:1642-1693). reconcile-false/case.yaml:1870-1895 then dispatches attempt 2 for the same key. But X07/F1 (confirmed, high, UNSAFE) states this chain already, with a late success and not only an error: its step (5) reads "the answer, a complete result, reaches the engine late: the attempt is fenced and the outcome stays unknown". Its least change targets exactly "a plain server's complete result that is not an error", and its verifier widened the unsafe mark. The text basis is the same: spec/conventions/operations.yaml:352-355 and :313-315.

**Verifier's corrections.** Known as X07/F1. The only new fact is that restart-interrupted is a second vector that encodes the accepted `occurred: false` after a late success. It belongs in the scope of X07/F1's fix, which must change this case's step 6 as well as reconcile-fenced. That is not a new situation. The engine restart adds nothing here: OPS-014's observable already says recovery under IMP-007 fences nothing (operations.yaml:360).


## FR-549

**Twelve of the 60 sampled cases, and single steps of three more, expect what only an `observable` states, the events that fence an attempt among them**

GL2/F4 · ambiguity · unsafe · verifier: known, medium · decides: coordinator · known-new-situation

**Promises.** OPS-014, ACC-029, LIF-036, LIF-013, SCH-009, SCH-010, SCH-019, IFC-033, PKG-018, SUB-005

**Where.** conformance/conventions/requests/reconcile-unfenced-restart/case.yaml:1526; spec/conventions/operations.yaml:358

**Quote.** "Stopping waiting under OPS-007, DL3 or DL10, a dropped connection or cancellation with no confirmation under OPS-027, and recovery of an in-progress invocation under IMP-007 fence nothing; another event described here must fence the attempt."

**Situation.** L13/F1 and X01/F1 record that the form does not say whether `observable` binds. The new fact is that the vectors, which do not bind, have already decided it, case by case. Class (b) in the sample: requests/reconcile-fenced (case.yaml:1705, a late error fences), reconcile-unconfirmed-cancel (:1526, :1638), reconcile-unfenced-restart (:1526, :1621), restart-interrupted (:1622), all from operations.yaml:352-361; inventory/e4/inventory-reading (:475, from lifecycle.yaml:254); language/contracts/sign-in-logins (keyword locations, SUB-005's observable); language/packages/private-export (case.yaml:35-38, from packages.yaml:164); interface/schemas/schema-conditional-default (:29) and schema-root-ref-default (:30), from schemas.yaml:145-150; schema-map-default (schemas.yaml:333); e05-root-key (:29) and schema-nested-key-allowed (implementation.yaml:940-941). Single steps: access/bounds step 10 (case.yaml:1960), where a Conduit to `access` with no bound opens anything, rests on access.yaml:491 alone; changes/retirement step 13 (:629-633), removal after a stop that ended unknown, on lifecycle.yaml:769; fleets/e3b/reconcile-control step 7 (:605-615). Two readings: observables bind (the vectors'), or they say how to check the `then`. On the second, OPS-014's `then` says only that an attempt is fenced once no copy of its dispatch remains in flight; a builder may hold that after its own restart none does, record failed, and dispatch the repeat under OPS-024 while the first call still runs: the effect twice. Only a non-binding vector stops it. The harm path is L13/F1's and X07/F8's; tally it once.

**Counterargument.** Each expectation is the natural reading, and most builders would read observables as binding. L13/F1 already asks for the form to say so.

**Least change.** When L13/F1 is settled, move the sentences these vectors test into a `then`: the fencing and non-fencing events into OPS-014 (or a promise of their own), the absent bound into ACC-006 and ACC-030, removal after a failed stop into LIF-036. The list above is the minimum set, because a vector already depends on each.

**Precedent.** IETF documents keep requirements in normative text under RFC 2119 and examples outside it; the Kubernetes API conventions separate a field's rule from its commentary.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior (rulings 37 and 46 point 4); it restates what the first build and its vectors do.

**Verifier.** known, medium. Quote verbatim at spec/conventions/operations.yaml:358-361. L13/F1 (confirmed, high, UNSAFE) records that the form does not say whether `observable` binds. It also records that the build and the vectors already take the binding reading: "the first build and its vectors (S1-archive-escape) treat it so". X07/F8 records that the fence's events sit in one observable, notes that "the vector requests-reconcile-fenced needs it", and proposes moving them into a promise of their own, the same least change. X01/F1 repeats the form point. The reviewer itself says the harm path is L13/F1's and X07/F8's. One instance the first pass does not name checks out: ACC-006's `then` requires an `access` Conduit "whose bound covers the new Conduit's target" (spec/conventions/access.yaml:107-109), and only ACC-029's observable says "A Conduit to `access` that applies no `bound.v1` has no bound and is unlimited" (access.yaml:490-492). conformance/conventions/access/bounds step 10 rests on it.

**Verifier's corrections.** Known (L13/F1, X07/F8, X01/F1). The list of vectors is useful as the minimum set for L13/F1's sweep, with the unbounded-access sentence added to it. The new unsafe claim is weaker than stated. For the absent bound, the `then`-only reading refuses, which is the safe direction. For the fence, a builder who treats a delivered request as no longer "in flight" contradicts the definition's own pointer: spec/definitions.md:216 says "OPS-014 states the evidence". That is L13/F1's harm and should not be counted twice.


## FR-550

**Six promises' only citing case does not hold the promise's situation, four more are half tested, and 99 of 292 cited promises are named in no step**

GL2/F5 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** INS-024, PKG-012, OBJ-001, IFC-011, PRV-022, OPS-002, CTR-014, CON-011, LIF-036, OBJ-022

**Where.** conformance/language/registration/arguments-default/project/documents.json:26; spec/promises/instantiation.yaml:335

**Quote.** "An Interface, Agent, Resource or Pool template writes no `config`."

**Situation.** 72 promises are cited by one case only; the sample covered 40. Not exercised at all: INS-024, whose case's one template writes `"config": {}` (documents.json:26) and which the case still requires the engine to cite (arguments-default/case.yaml:98); PKG-012 (two things exported under one name, at installation, spec/promises/packages.yaml:115), whose case loads a manifest that lists `public.v1` twice (language/packages/duplicate-exports/duplicate-conduit.json:6,9) and expects SUB-005, which is MAN-007's observable (spec/promises/manifest.yaml:154), uncited; OBJ-001, whose case gives every object a valid name and breaks only `as` (language/registration/e1/conduit-as-invalid); IFC-011, whose case publishes a tool definition and passes no handle through the engine (interface/schemas/e05-handle/case.yaml:8); PRV-022 and OPS-002, which bind a runtime provider and an implementation that s1/whole scripts (conventions/s1/whole/case.yaml:66, :40; the run intent at :2758 is the first build's own form). Half tested: CTR-014 (schema shape only, see F1); CON-011 (the consumer stays ready is asserted, that its status reports the closed Conduit is not); LIF-036 (the stop is awaited, its instance ID and machine are not asserted); OBJ-022 (only through `as`). None of these cases would fail if the promise were broken. Suite-wide: 764 of 1,610 citations are tags only, the ID appearing in no step's `required_promises` or `ruleId`; 99 of 292 cited promises are named in no step of any case citing them, 38 of the 72 single-cited ones. The `layers` tag differs from the layers of the promises cited in 278 of 494 cases; 150 tagged `interface` cite no interface promise, e4-tombstone-register-replacement among them (case.yaml:4-7). The contents table's counts (spec/README.md:201-250) and rule 5 (spec/README.md:301) rest on these tags.

**Counterargument.** spec/README.md:193 says a tag tests a slice of a promise. Several situations are met elsewhere without the tag: templates with no `config` in the e4 fixtures, an invalid object name in loader/schema-attribution under SUB-005. A tag-only citation is often a real test (ACC-006 in access/bounds).

**Least change.** Remove the six tags, or move each to a case that holds the situation. Then make the inventory test that already checks the manifest fail on a citation that no step names, so a tag must point at an assertion; drop `layers` from the case file and derive it from the promises cited.

**Precedent.** Ruling 68 point 2 already defines a safety vector as one with a field naming the promise it tests; requirements traceability in DO-178C and the Kubernetes conformance metadata both link a test to one stated behavior and check the link by tool.

**Who decides.** coordinator. A correction to non-binding coverage records (ruling 37); the counts in spec/README.md follow.

**Verifier.** weakened, low. Suite-wide counts reproduced exactly: 1,610 citations, 764 named in no step's `required_promises` or `ruleId`; 99 of 292 cited promises named in no step of a citing case; 38 of the 72 single-cited ones; `layers` differs from the cited promises' layers in 278 cases; 150 tagged `interface` cite no interface promise, including e4-tombstone-register-replacement (case.yaml:4-12). INS-024 is confirmed. Its only case's one template writes `"config": {}` (conformance/language/registration/arguments-default/project/documents.json:26). The other Agent is an instance (`from: agent`). The case still requires the citation (case.yaml:94-98). IFC-011, PRV-022, OPS-002, CON-011, LIF-036 and OBJ-022 are each cited by one case only.

**Verifier's corrections.** PKG-012 is refuted. The specification says a name written twice in `exports` is PKG-012's situation and is reported under SUB-005: spec/schemas/manifest.yaml:52-53 "`uniqueItems` does reject a name that `exports` writes twice (PKG-012)"; spec/promises/manifest.yaml:154-155; SUB-005's observable at spec/promises/submissions.yaml:67-69. L09/F12's verifier already said so. OBJ-001 holds in part: OBJ-013 makes `as` "a name under OBJ-022" (spec/promises/objects.yaml:207), and OBJ-022 applies OBJ-001's rule (:400-402). So the rule is exercised, though not OBJ-001's own situation. IFC-011's `when` does hold, because the fixture's operation has a `handle.v1` input (conformance/interface/schemas/e05-handle/fixture.json); only its `then` goes unexercised. PRV-022 and OPS-002 bind a provider and an implementation, which no engine vector can test by construction. That is the binding-party point of X01/F2 and L13/F4, not a mis-tag. INS-024's quote is at spec/promises/instantiation.yaml:332, not :335. Known-new-situation rather than new: X01/F5 (confirmed) records coverage kept in five places and proposes the case tags as the one source. This finding is the new fact that those tags are themselves unreliable, which X01/F5's fix must handle.


## FR-551

**The two vectors 042's Record names: `e4-inventory-reading` asserts one empty list, and `e4-node-readmit-no-tombstone` cites LIF-041 without any instantiation**

GL2/F6 · defect · verifier: weakened, low · decides: coordinator · known-new-situation

**Promises.** LIF-013, ACC-044, LIF-040, LIF-041

**Where.** conformance/conventions/inventory/e4/inventory-reading/case.yaml:475; spec/conventions/lifecycle.yaml:237

**Quote.** "The result holds what was declared, the instance's status and its records, and, for an instance other than a Conduit, its inventory."

**Situation.** The Record (docs/work-orders/042-build-tombstones-and-inventory.md:61-62) says the first holds no Fleet holder and the second injects the node. Both are weaker than that. inventory-reading has three steps: register, root opens `subject`, root inspects Interface `viewer` and expects `"inventory": {"tombstones": []}` (case.yaml:463-476). Nothing else is asserted: an engine that returned an empty inventory for every instance, or kept no provenance, passes, so the case tests neither LIF-013 nor ACC-044 (case.yaml:9-10), for any holder. Class (b) at best, and it cannot fail on the clause it is named for. node-readmit: step 1 retires the engine-made node and expects no tombstone (case.yaml:91-104), which follows from LIF-040's `then` (class a). Step 2 is a fixture patch that appends an object named `worker-host` with a new ID and `engine_made: true` and then inspects it (conventions/inventory/e4/node-requests.json:4-27, :267; case.yaml:109-118). No request "would instantiate an instance", which is LIF-041's `when` (lifecycle.yaml:857), so the expectation follows from the fixture and from no promise (class d); an engine whose tombstones blocked readmission would pass.

**Counterargument.** The Record keeps both for later. The content of the inventory is asserted in full by e4-inventory, e4-inventory-scope and the four provenance cases; a real readmission needs order 049a1's admission, which is not on main.

**Least change.** inventory-reading: have `viewer`, holding `viewer-fleet`, inspect Agent `a` and assert its `consumes` entry with provenance, and `outsider` refused; or delete the case and its two tags. node-readmit: drop the LIF-041 citation and retitle it as LIF-040's exemption for an engine-made node, until 049a1 lets a node be made again by request.

**Who decides.** coordinator. Small findings on non-binding vectors (ruling 71 point 4).

**Verifier.** weakened, low. conformance/conventions/inventory/e4/inventory-reading/case.yaml has three steps: register; root opens; root inspects Interface `viewer` and expects only `{"inventory":{"tombstones":[]}}` (:455-476). It cites LIF-013 and ACC-044 and asserts nothing of either beyond an empty list. In node-readmit-no-tombstone, step 2 is a `lifecycle-fixture` whose patch appends `worker-host` with `engine_made: true` (conformance/conventions/inventory/e4/node-requests.json:4-27). It then inspects it and expects `tombstone_count: 0`. No request instantiates anything, so LIF-041's `when` (spec/conventions/lifecycle.yaml:856-859) never holds. The quote from LIF-013 is verbatim at lifecycle.yaml:237-238.

**Verifier's corrections.** The node-readmit half is the Record's own statement: "node-requests' readmission injects the expected node instead of testing readmission" (docs/work-orders/042-build-tombstones-and-inventory.md:61-62). Only the proposal to drop the LIF-041 tag is new. What is new and real is that inventory-reading tests no part of LIF-013 or ACC-044 for any holder, not only for a Fleet holder. Step 1 of node-readmit follows from LIF-040's exemption of engine-made objects (lifecycle.yaml:832), class (a).


## FR-552

**45 steps in 24 cases compare a diagnostic's message or the text block with the first build's exact words**

GL2/F7 · defect · verifier: confirmed, low · decides: coordinator · new

**Promises.** IFC-050, IFC-025, OPS-030, FST-006, OPS-029

**Where.** conformance/conventions/requests/reconcile-fenced/case.yaml:1640; spec/interface/implementation.yaml:1348

**Quote.** "Wording is the engine's choice; the ruleId, reason and refusal.v1 fields are observable."

**Situation.** The runner compares strings for equality (engine/crates/conformance/src/lib.rs:319-349). In the sample: reconcile-fenced/case.yaml:1640 and :1649, reconcile-unconfirmed-cancel/case.yaml:1638 and reconcile-unfenced-restart/case.yaml:1621 pin the whole sentence "reconcile.v1 cannot record invocation ... as failed: it is not yet fenced (OPS-030). Every attempt needs an answer ending its call ..."; language/first-start/protected/case.yaml:832 and :1148 pin "protected core template github.com/shrav-k/conduit/core#system". Outside it, conventions/records/expected-id/case.yaml:812 pins "Resource/missing does not exist. A closed Conduit and a removed instance no longer exist." A second engine that says the same in other words fails 24 cases. The suite's own README says "Library prose is not a portable expectation" (conformance/README.md:90), and the loader cases already have the right tool: `message_contains` (language/loader/depth/case.yaml:91).

**Counterargument.** Pinned text is how the first build guards ruling 52's explicitness against regressions, and the cases carry `profile: first-build`.

**Least change.** Replace each pinned text with `message_contains` facts for what the message must name under IFC-050: the operation, what the request named, the promise ID. Keep exact text only in the first build's own tests under engine/.

**Precedent.** SARIF separates `ruleId` from `message.text` for this reason; the JSON Schema Test Suite asserts valid or invalid and never an error message.

**Who decides.** coordinator. A correction to non-binding vectors that adds no behavior (rulings 37 and 46 point 4).

**Verifier.** confirmed, low. Quote verbatim at spec/interface/implementation.yaml:1348-1349. The runner compares strings for equality (engine/crates/conformance/src/lib.rs:317-349). Leaving out passed-through implementation content ("server answer", s1/whole's file text), my count is 33 steps in 24 cases that pin engine-written diagnostic messages or text blocks. Examples: reconcile-fenced, reconcile-unconfirmed-cancel and reconcile-unfenced-restart ("conflict: reconcile.v1 cannot record invocation ..."); language/first-start/protected and protected-authorized ("protected core template ..."); conventions/records/expected-id step 6 and step 8; language/archives/*-escape; the release-*-default-rejected cases; interface/schemas/schema-conflicting-defaults. Counting condition messages in setup/review-conditions-builtin and -provider brings it to 42 steps in 26 cases. The repository says the opposite of what the cases do: conformance/README.md:90 "Library prose is not a portable expectation", and engine/CONFORMANCE.md:28 "Vectors compare the required rule/location facts rather than validator prose". The tool already exists: `message_contains` (conformance/language/loader/depth/case.yaml:87-93). Not in the first pass or the known-open lists.

**Verifier's corrections.** The step count depends on the method (33 to 42 against the stated 45); the case count (24) matches. Low severity, since every case names `profile: first-build`, but it blocks A39's adapter for any second engine.


## FR-553

**Cases pin implementation-defined choices without naming the item: diagnostic order (DL9, absent from the profile), the depth limit (DL10), generated names (DL15) and record actions (DL8)**

GL2/F8 · defect · verifier: weakened, low · decides: coordinator · new

**Promises.** SUB-011, PKG-018, NAM-028, FLEET-006, REC-002

**Where.** conformance/language/packages/private-export/case.yaml:35; spec/implementation-defined.yaml:75

**Quote.** "The order in which several failed checks that give the same reason are reported."

**Situation.** Class (c) in the sample. language/loader/depth is the one case that is (c) throughout: depth 65 to 128 is accepted and 129 rejected with a message containing `128` (case.yaml:91-93), the profile's DL10 choice; an engine at the floor of 64 fails four of its ten steps while keeping SUB-011. Secondary (c): packages/private-export expects `[PKG-018, NAM-028]` in that order (case.yaml:35-38), DL9; suite-wide 22 cases expect two or more diagnostics as an exact array and 8 fix the order of different rules. language/fleets/names step 6 expects the generated name `fleet-00001` (case.yaml:2551), DL15. conventions/first-start/initial expects 16 records with the actions `registered` and `lifecycle-changed`, DL8. No case says which expectation rests on which item: the case schema has no field for it, and conformance/profiles/first-build.yaml has no DL9 entry at all (the choice is written only in engine/CONFORMANCE.md:28). An adapter for another engine cannot tell a promise's expectation from a choice it is free to make differently.

**Counterargument.** Every case names `profile: first-build`, and the README says the profile states the admin's choices. L12/F1 already covers the keyword location under a union, which CONFORMANCE.md also files under DL9.

**Least change.** Add DL9 to the profile. Give a step an optional `choices` list of DL items, and have the runner skip or adapt those assertions for another profile; compare multi-diagnostic arrays as sets unless OPS-023 orders them.

**Precedent.** The JSON Schema Test Suite keeps behavior a validator may choose under `optional/`; Kubernetes conformance separates conformance tests from provider-specific ones by tag.

**Who decides.** coordinator. A build decision on the runner (rulings 37 and 48).

**Verifier.** weakened, low. DL9's text is at spec/implementation-defined.yaml:72-75. conformance/profiles/first-build.yaml has the keys id, status, DL2, DL15, DL4, DL12, DL8, DL10, DL6, record_types, implemented_here, reserved_for_later_orders, DL7 and D19, so there is no DL9. The choice is written only in engine/CONFORMANCE.md:28. The counts reproduce: 22 cases expect two or more diagnostics as an exact array, and 8 fix the order of different rules (e.g. conformance/language/packages/private-export/case.yaml:31-38 `[PKG-018, NAM-028]`). language/loader/depth accepts up to 128 and checks `message_contains: ["128"]` (case.yaml:80-93), against DL10's floor of 64 (implementation-defined.yaml:79-80). language/fleets/names expects `fleet-00001` (case.yaml:2550-2552), a DL15 choice.

**Verifier's corrections.** The profile every case names already states DL10 (`nesting: 128`), DL15 (suffix length and alphabet) and DL8 (`event_actions`, including `registered` and `lifecycle-changed`). Those choices are named for the profile. What is missing is DL9 in the profile and any link from a step to the item it rests on. Of the 8 cases ordering different rules, only those whose rules give the same reason fall under DL9. Where the reasons differ, OPS-023's order binds. The text-side question of which rule decides when several refuse is L11/F6 and X06/F5.


## FR-554

**452 cases require the first build's own promise citations and 283 its state form; lists are compared in an order no promise states**

GL2/F9 · needless-complexity · verifier: known, low · decides: coordinator · new

**Promises.** OPS-036, OPS-014, LIF-040, LIF-013

**Where.** conformance/conventions/fleets/e3b/reconcile-control/case.yaml:613; conformance/README.md:76

**Quote.** "Object order in this runner follows the supplied first-start allocations; it is not an inspection ordering promise."

**Situation.** Counted over 494 cases and 1,717 steps. `required_promises` (905 steps, 452 cases) is checked against the first build's `decisions[*].promises` (engine/crates/conformance/src/lib.rs:965-971): which promises a transition cites is stated by no promise, and F5 shows the engine citing INS-024 where its `when` does not hold. `state` (576 steps, 283 cases) uses the first build's field names (`first_start`, `engine_made`, `local_name`, `open`). `invocation_facts` (32 cases) reads `attempt_fences` (reconcile-control/case.yaml:613-615), a fact no promise shows to any actor (L03/F4). `tombstone_count` (21 cases) and captures from `/tombstones/0/id` read the store directly. `list.v1` results are compared as ordered arrays (conventions/list/from-first-start/case.yaml:1145, :1264) although OPS-036 states no order; the README concedes it. In the sample these are secondary expectations of class (d) in first-start/initial, list/from-first-start, setup/pool-entry, reconcile-control and s1/whole. For action item A39 ("any engine can run them through an adapter") they are a second specification of an engine's internals.

**Counterargument.** The README opens "Not binding" and calls itself a convention of the first build's runner. State assertions are what catch a hidden side effect, and its planned work already proposes replacing whole expected states with projections, for size.

**Least change.** Split each `expect` into a portable part (the reply in IFC-025's form, records in `record.v1`, and what a following `inspect.v1` or `list.v1` request returns) and a profile part (`state`, `required_promises`, `invocation_facts`, `tombstone_count`). Compare list results as sets. Assert a tombstone through `list.v1` or the inventory, as tombstone-visibility already does.

**Precedent.** Kubernetes conformance tests act only through the API server; `kubectl get` sorts by name where an order is wanted.

**Who decides.** coordinator. A build decision on the runner (ruling 48). If an order is wanted for `list.v1`, that adds behavior and is the owner's.

**Verifier.** known, low. Counts reproduced: `required_promises` in 905 steps of 452 cases, checked against the core's `decisions[*].promises` (engine/crates/conformance/src/lib.rs:965-975); `state` in 576 steps of 283 cases; `invocation_facts` in 54 steps of 32 cases; `tombstone_count` in 24 steps of 21 cases. The quote is verbatim at conformance/README.md:76-77, and OPS-036 states no order (spec/conventions/operations.yaml:864-886). But P07 I7 already recommends what the least change asks: inputs and assertions at the public boundary, no state patches, the 494 cases tagged internal, and A39's adapter as the aim. It cites the same runner lines (lib.rs:965-982). Its verifier adds that a Decision-level field is "one more internal thing a second engine must copy".

**Verifier's corrections.** Known (P07 I7, partly supported). The new details are the counts and the ordered comparison of `list.v1` results, which no promise orders. Those belong in P07 I7's runner order. Splitting the existing cases into portable and profile parts goes further than P07 I7, which leaves the old cases alone. That choice is the coordinator's under the case form (conformance/README.md:3-4).


## FR-555

**`schema-applicator-defaults` decides L12/F3's open question in a vector: defaults are filled through `patternProperties` and `additionalProperties`**

GL2/F10 · ambiguity · verifier: refuted, low · decides: coordinator · known-new-situation

**Promises.** SCH-009, SCH-010, SCH-016, SCH-008

**Where.** conformance/interface/schemas/schema-applicator-defaults/case.yaml:48; spec/promises/schemas.yaml:125

**Quote.** "A value is supplied where a schema governs it, and a property that has a `default` in that schema is absent from a map that is present in the value."

**Situation.** The fixture's schema states defaults only inside the subschemas of `patternProperties` and `additionalProperties` (conformance/interface/schemas/schema-applicator-defaults/fixture.json). For the value `{a: {}, &lt;emoji>: {}, fixed: {}}` the case expects `a` filled from `additionalProperties` (`n: 1`, `additionalOnly: 7`, case.yaml:48) and the emoji member filled from two matching patterns at once (`n: 1`, `m: 2`). SCH-009 names eight keywords that give no default and is silent on these two. Builder A fills through `properties` and list items only, as SCH-009's examples show and as Ajv's `useDefaults` does, leaves `a: {}` and reports the sample invalid. Builder B fills wherever a subschema governs a present map. The vector takes B. L12/F3 records the ambiguity in the text; the new fact is that a vector has settled it, and settled the two-pattern merge with it. Class (d): the expectation follows from no promise.

**Counterargument.** SCH-009's observable says "This is one rule for every schema that governs a supplied value", which favors B; the README announces the vectors (conformance/README.md, "Authored schemas and E05").

**Least change.** Write the vector's rule into SCH-009: a member matched by `additionalProperties` or by one or more keys of `patternProperties` is a present map and takes the defaults of each matching subschema's `properties`; the conflicting-defaults check covers two patterns that disagree. Or take L12/F3's other road and have `schema.v1` reject a `default` there, and delete the case.

**Precedent.** Kubernetes structural-schema defaulting fills through `properties`, `items` and `additionalProperties` and has no `patternProperties`; Ajv fills through `properties` and `items` only.

**Who decides.** coordinator. An ambiguity whose fix adds no behavior where it states what the first build and its vector do (rulings 37 and 46 point 4); the rejecting alternative removes a feature and would be the owner's.

**Verifier.** refuted, low. SCH-009's `when` covers any present map that a schema governs: "a property that has a `default` in that schema is absent from a map that is present in the value" (spec/promises/schemas.yaml:125-126). Its observable says "This is one rule for every schema that governs a supplied value" (:135-136). The exclusions name only eight keywords, and neither `additionalProperties` nor `patternProperties` is among them (:145-148). A member that a pattern or `additionalProperties` matches is governed by that subschema, so it takes that subschema's defaults. A member that two patterns match is governed by both, so it takes each one's defaults. The vector's two patterns default different properties (`n` and `m`; conformance/interface/schemas/schema-applicator-defaults/fixture.json), so no merge rule is needed. L12/F3's verifier already decided this at the same commit: "That covers `additionalProperties`, `patternProperties`, `items` and `prefixItems`, so Reading B is not a sound reading", narrowing L12/F3 to `contains`.

**Verifier's corrections.** The case is class (a): it follows from SCH-009's `when` and `then`. Builder A's reading is not sound. The real residue is different and no vector decides it: two applicators that both govern one member and give different defaults for the same property (see extra).


## Further points the verifier noticed

Not verified by a second reader.

- **Overlapping applicators with different defaults for one property are neither rejected nor ordered** (spec/promises/schemas.yaml:155-158; engine/crates/core/src/schemas.rs:612-652 and :561-570). SCH-009 rejects a document only where schemas that apply to one place "unconditionally, through a `$ref` and its siblings or through `allOf`, give different values of `default` there". A member that both an explicit `properties` entry and a matching `patternProperties` key govern, or two matching patterns, can each give a different `default` for the same sub-property. Both subschemas govern the member under SCH-009's `when`, and no promise says which default is filled. The first build fills the first applicable schema and skips the rest: `!object.contains_key(name)` at schemas.rs:618-621, applied in order properties, then patterns in map order (:637-652). Its admission check groups children by `/properties/&lt;name>` and `/patternProperties/&lt;pattern>` separately (:561-570), so it never compares them. Another builder could fill the last match or reject the document, and the stored configuration and the arguments sent would differ. Class: gap (ambiguity at fill time). Not unsafe. Coordinator under rulings 37 and 46 point 4, with a vector. Least change: extend the admission check to an explicit property name that a pattern matches (statically decidable), and for two patterns either reject overlapping keys with differing defaults at fill time under SCH-009 or state the order. Precedent: Kubernetes structural schemas allow no `patternProperties`, so the question cannot arise there. Not in L12/F3 or any known-open list.

## The verifier's notes

Commit 2eb8f258 (2026-11-12), read only. All counts were recomputed in memory with one-off /usr/bin/python3 scripts over every case.yaml; no file was written. Reproduced: 494 cases (225 language, 236 conventions, 33 interface), 1,717 steps. The reviewer's account of the failure after 042's rebase holds. engine/crates/core/src/requests.rs:1066-1082 is order 070's field list ending `..State::default()` (git blame: 1a34ccf9, order 070), and only the tombstone line :1070 comes from 57813f06 (order 042). The vector expects LIF-041's conflict (conformance/conventions/inventory/e4/tombstone-register-replacement/case.yaml:704-723). So it was a fault of the merge, not of the text or the vector.

Outcome: one new confirmed finding of weight (F1, the first build's `ownTools` reading, which matters before the .v2 Agent Types state `toolsDirectory`). F7 is confirmed and low. F2, F5, F6 and F8 are real but smaller or of another class. F3, F4 and F9 are already recorded (X07/F1, L13/F1 with X07/F8, P07 I7). F10 is refuted by SCH-009's own `when`, as L12/F3's verifier had found.

Read in full: docs/guides/full-review.md lines 72-229; AGENTS.md; docs/work-orders/042-build-tombstones-and-inventory.md; the first-pass entries X07/F1, X07/F8, L13/F1, L12/F3, L09/F12, X01/F5 and P07 I7, with greps of both INDEX files and critic.json; docs/AUDIT.md \"Known to be open\"; the promises IFC-025, IFC-050, ACC-056, OPS-013, OPS-014, OPS-018 (when/then), OPS-024 (when/then), OPS-036, CTR-014, SCH-009, LIF-013 (then and the ownTools paragraph), LIF-036 (when/then), LIF-040, LIF-041, INS-024, PKG-012, MAN-007 (part), OBJ-001, OBJ-013 (then), OBJ-022 (when/then), IFC-011, OPS-002, CON-011 and ACC-029 (when/then); engine/crates/core/src/inventory.rs:300-375, engine/crates/conformance/src/lib.rs:300-350 and :960-975, access.rs:40-70, and schemas.rs:520-660. Cases summarised by script or partial read: restart-interrupted, reconcile-false (parts), inventory-reading, node-readmit-no-tombstone, own-tools, arguments-default, duplicate-exports, conduit-as-invalid, e05-handle, private-export, depth (part), schema-applicator-defaults, tombstone-register-replacement (parts) and s1/whole (one intent).

Not read: the remaining cases in full; the reviewer's 60-case sample, which was not given case by case, so its class counts (28/16/12/1/3) were not rechecked; IN_FORCE.md beyond lines 136-146 and 199; HANDOVER.md and cold-readers.md beyond greps; the increment logs beyond a grep for own tools; experiments/012-model-checking/G5.md; and engine code beyond the lines named.
