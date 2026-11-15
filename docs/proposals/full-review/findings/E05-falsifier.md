# E05-falsifier: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** The layered claim has not been tested where it matters. Its descriptive half is already contradicted by the repository's own counts: the convention layer is the largest (245 of 519 promises, up from 179 six days earlier while the language layer went from 220 to 226), its promises carry procedures, and model checking refuted or contested 29 of the 143 promises it modelled. Its central half, that spec/ alone determines what documents mean, has never been measured: six reading rounds were none of them clean, the two-clean-rounds bar was withdrawn, and the one direct test (A34) is last in the order behind an eleven-condition gate and does not gate v0. Swappability cannot be tested for weeks: no contract suite holds a case, three of the four contracts have one implementation, the engine on main serves no provider call, and S10 as accepted permits the very edits that the roadmap's own wording ("the same documents") would count as failure. The fastest falsifier is candidate (b), narrowed to verdicts: fresh readers predict held-out vectors from spec/ alone, scored against the recorded expectations, with a third reader denied the `observable` fields. It needs no owner time, one small Rust tool and, by my estimate, two to three days; a failure is decisive and a pass only suggestive, which is what a falsifier should be. Two companions are nearly free: S10's edit ledger written on paper now, and the MC2 and MC4 re-checks on the text that orders 077 and 081 changed. What is working: the engine's code names no provider or scenario, a fresh author finished S1 from spec/ alone in 46 minutes, model checking found real contradictions in a day at no cost to the owner, and the scenario plans send contract gaps to packages as the claim predicts. What I would change: write no new promise until the determinacy rate is known, move binding text out of `observable` (69,486 words against 22,622 words of obligation), and treat S3's real run as a demonstration of providers, not a test of the claim.

**What the analyst read.** Tree: the repository root at 2eb8f258 (2026-11-12). Nothing was built, run, written or fetched; every count is from a read-only script or grep on that tree.

Read in full: docs/guides/full-review.md; docs/implementation/HANDOVER.md; AGENTS.md "What holds"; docs/AUDIT.md "Purpose", "Preservation tests", "Layered claim", "Layer decisions" and the P0 and P1 action items (A34's row among them); ROADMAP.md (the longest register rows cut at 2,500 to 4,000 characters); experiments/021-s10-swapped-providers README.md and plan.md; experiments/014-s3-two-machines/README.md; experiments/022-provider-contracts/README.md and its four suite indexes; IN_FORCE.md topics 2 and 13; the Records of orders 077, 081, 129, 133, 135, 136, 137, 139, 141, 142, 143, 145, 148 and 150, and the purpose sections of 079, 080 and 145.

Read in part: spec/README.md lines 1 to 105 and 160 to 335 (not the sections under "Limits of this version"); docs/implementation/PROVIDERS.md lines 1 to 260 of 385; experiments/014-s3-two-machines/plan.md lines 1 to 75; experiments/012-model-checking/G5.md lines 1 to 60 and 341 to 427 (not the per-promise account); docs/implementation/INDEPENDENT_BUILDS.md lines 1 to 165; engine/CONFORMANCE.md lines 1 to 70 of 397; conformance/README.md lines 1 to 166 and 531 to 556; ruling 40's record lines 1 to 60; ruling 47 "When to publish"; docs/proposals/reaudit/README.md lines 1 to 60; round 6's critique at D21 and D22; the first-build register lines 9 to 75; increment 4's fresh-author report lines 14 to 75; the "What spec/ does not yet decide" sections of experiments 013 and 015 to 020.

Promises read as text: PRV-001 to PRV-004, PRV-006, PRV-007, PRV-016, PRV-019, PRV-021, PRV-026, PRV-030, SEC-005 and LIF-013. All other promises were only counted (layer, level, words of when, then and observable, vectors field). The 494 cases were parsed for step kinds, initial state and expectations; three were opened.

Outside the pinned commit, read-only: git grep and diff --stat on origin/wo/080-shell-runtime-provider-calls and origin/s3/m2, used only for the search in I8 and so marked.

Not read: the engine's source beyond the Executor trait, one line of lifecycle.rs and the searches; the source of the kit, the providers and the experiments; conformance/manifest.yaml (coverage was computed from the case files instead); the S2 and S4 to S9 plans; the ledger and ruling records other than 40 and 47; the safety catalogue; experiment 012's README; spec/definitions.md, the schemas and the core package. PRV-013 was not read, so I9's paper reading of the package-source swap rests on PRV-003 and PRV-016 and is marked as mine. The dockershim comparison in I8 is from general knowledge, not from the repository. The time and cost figures in I1 are estimates.


## FR-436

**Chosen falsifier: fresh readers predict held-out verdicts from spec/ alone (A34, narrowed). Proposed register row A83, experiment 026**

E05 I1 · experiment-change · verifier: partly · decides: owner

REGISTER ROW (ROADMAP.md, Experiments and probes). Item: A83, experiments/026-blind-verdicts (024 and 025 are taken by S3's and S10's plans). Question it settles: does spec/ alone determine the verdicts the reference engine records? Needs: no engine run, no owner time, no account beyond the primary Codex and Claude sign-ins; spec/ and conformance/ at one pinned commit. Feeds: the layered claim's first clause; whether and how A34 runs; the form of A59's next read; G7. Done when: dated results hold every card, every prediction, the blind adjudication of every difference, and measures M1 to M7. State: proposed. Source: this review; rulings 10, 13, 45. It is a read, not a build, so G1 to G11 do not gate it.

THE CLAIM AND ITS PREDICTION. Ruling 13: the language fully determines what a set of documents means. So a reader holding only spec/ gives, for any set of documents, the verdict the vectors record, and any difference is the reader's slip, shown by quoting one promise.

METHOD. (1) Frame: the 264 cases whose every step is a document step (read, load, resolve, install, register, submit): 336 steps with an accept-or-reject expectation, 139 accept and 197 reject. Stratum A: 151 cases from an empty state. Stratum B: 113 from a state snapshot. (2) Sample, sealed before the run: 60 cases by a recorded seed, 36 from A and 24 from B, proportional by directory, about 76 scored steps; the list is hashed into the pre-registration. (3) Cards: a Rust tool renders each case's inputs with the case ID, title, directory, promise list and every expectation removed; files get neutral names except where a name has meaning (conduit.yaml); a snapshot is shown as the objects it holds, in object.v1 form. A card whose documents equal a file under spec/schemas/examples is dropped and counted. (4) Readers, each a fresh session in a directory holding only its copy of spec/, the DL choices of conformance/profiles/first-build.yaml and its cards, with no network and no repository: R1 (Codex) and R2 (Claude) with spec/ whole; R3 (Codex) with every `observable` field removed. For each step a reader answers accepted or rejected; if rejected, the reason and the promise whose `then` rejects it; if accepted, the objects made, by family; or "undetermined", naming the promises that conflict or the rule that is missing. (5) Every answer that differs from the expectation goes to a fresh adjudicator, who sees the card, spec/ and the answers unlabelled, the expectation among them, and must quote the text that decides or say that none does. Classes: reader's slip; expectation wrong against the text; ambiguity; gap; contradiction; outside spec/ (the answer needs the engine's code, a runner convention, or a document outside spec/ and the profile). The coordinator reviews each adjudication.

PASS: across R1 and R2, at most 2 scored steps classed ambiguity, gap or contradiction (about 3%), none unsafe by the tracker's test, none classed outside spec/. FAIL: 6 or more such steps (about 8%), or one that is unsafe, or 2 or more classed outside spec/. BETWEEN: not shown; run the remaining 204 cases before concluding anything.

MEASURES, fixed before the run. M1 verdict agreement by reader and stratum, and for accepted steps the objects by family where the case asserts them. M2 on rejections, whether the reader's promise is among those asserted. M3 "undetermined" answers. M4 the class of every difference, with promise IDs. M5 steps where R1 and R2 differ from each other. M6 R3 against R1: steps right with the whole text and wrong without `observable`. M7 per card: time, tokens and the number of promises the reader cited.

WHAT A FAILURE WOULD CHANGE. (a) No new promise enters spec/ until a rerun passes; batch 1's remaining spec orders wait. (b) The owner is asked whether, for the language layer, a promise's vectors bind its meaning, as CommonMark's embedded examples and WebAssembly's reference interpreter do; today a vector "binds a conformance claim, not meaning". (c) A34 is recut to one Language-level build, run only after the rate passes. (d) If M6 shows a drop of 5 steps or more, every binding sentence in `observable` moves to `then` or is deleted before anything else is written.

WHAT A PASS WOULD CHANGE. The scored form replaces the open-ended round 7 read. Stage 2 follows: the convention layer (the 406 steps whose expected reply carries an outcome or a refusal reason, with state cards), and the 227 promises no case cites, where two readers' agreement on reader-written situations stands in for an expectation.

COST AND LIMIT. Two work orders (the card tool; the run), about 18 reader batches of ten cards, one adjudication per difference. My estimate is two to three days and no owner time. Readers from the model families that wrote the text can share its blind spots, so a pass is suggestive and a failure decisive. The thresholds are my proposal; change them before the sample is sealed, never after. My forecast, stated so that it can be checked: stratum A passes, and the differences come from stratum B's registration cases and from which promise a rejection is attributed to.

**Evidence.** Counts by /usr/bin/python3 over conformance/**/case.yaml at 2eb8f258: 494 cases; 264 with only document step kinds; expect.reply.accepted or expect.accepted: True 139, False 197; initial recipe empty 151, snapshot 113; outcomes in reply._meta: succeeded 171, unknown 75, failed 39; reasons: conflict 47, refused 36, invalid 33, unavailable 5. Case shape: conformance/language/registration/template-family/case.yaml expects reply {accepted: false, reason: invalid, diagnostics: [{ruleId: INS-002}]}. docs/AUDIT.md:375 "the language fully determines what a set of declarations means". IN_FORCE.md:43 "A vector ... binds a conformance claim, not meaning". docs/implementation/INDEPENDENT_BUILDS.md:4-5 "must not start until the readiness gate passes"; :46 "A session that helped write the language is not a cold reader". ROADMAP.md:89-101 (proposing an experiment; ruling 45). experiments/021-s10-swapped-providers/plan.md:142 names experiments/025-1password and A81's planned 024.

**Waits.** A34's form; the form of the round 7 read; whether batch 1's remaining spec orders are written before the determinacy rate is known

**Verifier.** partly. Recounted at 2eb8f258 with /usr/bin/python3 over conformance/**/case.yaml. Holds: 494 cases; 336 accept-or-reject steps, 139 accept and 197 reject; 151 empty and 113 snapshot; `_meta` outcomes 171/75/39 and reasons 47/36/33/5 over 406 steps. docs/AUDIT.md:375, IN_FORCE.md:43, docs/implementation/INDEPENDENT_BUILDS.md:4-5 and :46, experiments/021-s10-swapped-providers/plan.md:142 and experiments/014-s3-two-machines/plan.md:141 read as cited. A82 is the highest action item and experiments/ stops at 023, so A83 and 026 are free. Records show small tool orders taking 11 to 24 worker minutes (docs/work-orders/073, 104, 123), so two to three days is credible.

Does not hold:
- The 264 frame needs twelve step kinds. The six the body names give 221 cases and 321 scored steps; 264 also takes schema-case, schema-admission, schema-evaluation, validate-values, first-start and registration-orders. 34 of the 264 have no accept-or-reject step.
- The frame cites 148 promises (115 language, 30 convention, 3 interface), and its rejections name 64 rules, 62 of them language. A pass therefore says nothing of 371 promises.
- 91 of the 113 snapshot cases start from conformance/fixtures/root-system-state.json. It is the engine's own state form (`from`, `first_start`, `config`), and conformance/README.md:66 calls it "not a state the engine can reach". Rendering it as object.v1 is an interpretation that needs the engine's code.
- Failure consequence (a) names "batch 1's remaining spec orders". None remains: 036a, 036b, 037, 038a, 038b and 038c are merged (git log), and 046, 047, 048a, 048b, 050 and 053 are build orders (docs/proposals/spec-batch-1/README.md:545-657).

**Verifier's counterargument.** The frame is where a failure is least likely. Every vector was written by the pipeline that wrote and then repaired the promise it cites (53 first-build findings settled into the text), so agreement there measures the fit after repair. The analyst's own forecast is that stratum A passes.

Better: add a stratum of documents the text's authors never saw. A fresh reader mutates accepted fixtures or writes situations for uncited language promises and predicts the result; the reference engine is the oracle. That is LD4's "generated documents" (INDEPENDENT_BUILDS.md:26), and an engine case with no handler is itself a finding (A38).

Rulings: consequence (a) puts a reading gate on writing spec/. Ruling 38 point 4 replaced such a gate (docs/proposals/ruling-38/README.md:19) and ruling 44 point 1 chose a build over "another round of review of the prose" (docs/proposals/ruling-44/README.md:20). The item names neither. Consequence (b) reverses ruling 17 NS4 (docs/proposals/normative-surface/README.md:59-61); the item does say so.

Who decides: a fresh read is the coordinator's (ruling 48; docs/guides/full-review.md:107), and experiments 012, 022 and 023 were registered without a ruling (docs/work-orders/123-a63-h1-provider-contract-harness.md:190). Only consequences (a) to (c) are the owner's.

Power: at a true rate of 5% over about 76 steps, by my binomial arithmetic the chance of landing "between" is about 56%, so the first pass will probably not decide.

**Verifier's corrections.** - Frame: state the twelve kinds, or use the 221 cases; drop the 34 cases with nothing to score.
- Sample: proportional to 151 and 113 is 34 and 26, not 36 and 24.
- Reach: stage 1 is bounded to 148 promises, not 292.
- Consequence (a): name what it would pause, which is the admin's next spec order (docs/implementation/HANDOVER.md:215) and batch 2's text, not batch 1.
- Precedent: WebAssembly's reference interpreter does not bind meaning; its specification does (my knowledge, not the repository). The common practice is today's rule: the specification is normative and the tests check a conformance claim.


## FR-437

**Run the blind-verdict test now, before more of the specification is written?**

E05 I2 · question-for-owner · verifier: partly · decides: owner

The one direct test of the layered claim (two independent builds, A34) is the last step of the order and does not gate v0. A narrowed form can run now, in parallel, at no cost in your time: fresh readers given only spec/ predict the verdicts of 60 existing vectors, and every difference is adjudicated against the text (I1).

A. Run it now, as row A83, with I1's pass and fail conditions and its four consequences of a failure accepted in advance. Recommended.
B. Run it after the S1 rerun's pin, so that the text read is batch 1's.
C. Leave A34 where it is, at step 7.

Recommendation: A. It costs two work orders and Codex allowance, and none of your time. The text has grown by about 30% since the last cold read (78,627 words of promise text at round 6; 102,201 now) with no fresh read since, and batch 1 will add more. If spec/ does not determine verdicts, every further promise is written on the wrong footing, and the rerun, S2 to S10 and model checking would not tell you so. B delays the answer by the whole of batch 1 and saves nothing, since a rerun on the new text is cheap once the tool exists.

**Evidence.** ROADMAP.md:32 (step 7: "The readiness gate, then the two engine builds ... later"); docs/AUDIT.md:468 (A34 "open: blocked by the readiness gate"); IN_FORCE.md:282 ("Independent builds inform the owner's judgment and do not gate it"); docs/proposals/reaudit/round-6/design-critique.md:583 (78,627 words at 452 promises); script sum of when, then and observable words over the 519 promises at 2eb8f258: 10,093 + 22,622 + 69,486 = 102,201.

**Waits.** I1 runs or not; the spec orders of batch 1 (046, 047, 048a, 048b, 050, 053) either wait for a failure's consequences or proceed

**Verifier.** partly. Holds: ROADMAP.md:32 (step 7, later); docs/AUDIT.md:468 (A34 blocked by the gate); IN_FORCE.md:282. docs/proposals/reaudit/round-6/design-critique.md:577-585 gives 78,627 words of when, then and observable at 452 promises, counted the same way as my recount of 102,201 (10,093 + 22,622 + 69,486), so growth is 30%. docs/AUDIT.md:59: "Text changed since has not been read cold in full."

Does not hold:
- "Batch 1 will add more" text. Batch 1's six spec orders are merged, and IN_FORCE.md:68-97 marks its lines "In spec/". Text still to come is batch 2, the metrics batch (IN_FORCE.md:98, :123, :148, :171, :200, :255) and the admin's next spec order.
- Option B ("so that the text read is batch 1's") therefore has no reason behind it: the text is already batch 1's.
- The "waits" field calls 046, 047, 048a, 048b, 050 and 053 spec orders. They are engine and package build orders on the S1 rerun path.
- "Codex allowance, and none of your time" leaves out Claude. R2 and the adjudication run on the owner's Claude usage, which was near its limit at the handover (docs/implementation/HANDOVER.md:4-5).

**Verifier's counterargument.** The question need not go to the owner. The coordinator may run a fresh read as a logged decision, with thresholds fixed before the sample is sealed. The owner is asked only if it fails, with the numbers in hand, about the consequences that are the owner's: the order of the work and NS4. That removes one question (full-review.md:101-103: do not ask the owner what a coordinator may settle).

As posed, option A has the owner accept four consequences before any data exists. One reverses a ruled rule of form. Another would hold the rerun path that ruling 71 points 2 and 3 ordered, if (a) is read as pausing 046 to 053.

**Verifier's corrections.** - Drop option B, or restate it as "after batch 2's text".
- Rename the orders in "waits" as build orders.
- Add Claude usage to the cost.
- "No fresh read since" needs one qualifier: a fresh Codex author worked from spec/ plus two pages on 2026-11-10 (docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md:12-27).


## FR-438

**The five candidates weighed: (b) is the fastest and the only one whose failure would be decisive**

E05 I3 · assessment · verifier: partly · decides: none

(a) S10. Predicts that a swap keeps tasks, access, results and attribution with only provider-specific edits. The cheapest contradicting observation is an edit outside D16's list, but that list already allows instance `runtime` selections and re-created instances and Conduits, and a forbidden edit leads to "a revised plan", not to a verdict on the claim. Built: none of Docker, Git, keychain or 1Password; the engine serves no provider call; the contract suites are empty. Owner's time: Docker, a Git repository, stores, sign-ins, and twelve replays of about an hour each. Time: after batch 1, the rerun, the bar and batch 2; 20 of the 24 admin items its plan lists are not on main. Failure: weakly decisive. Pass: informative about providers and author cost, not about determinacy.

(b) Meaning from spec/ alone. Predicts that a reader with only spec/ gives the recorded verdict. Cheapest contradicting observation: one case where a sound reading of the text gives another verdict, or where the verdict needs something outside spec/. Built: 494 vectors with expectations, an engine that passes them, spec/ as one directory. Owner's time: none. Time: two to three days. Failure: decisive for the first clause. Pass: suggestive, and bounded to the 292 promises a case cites.

(c) S3 under a partition. Predicts that the conventions hold as invariants across a cut. The cheapest contradicting observation is a short trace in a model, and that has already been made: MC2-001 in 7 actions with timers alone, MC4-002 in 33 and 38 actions with a cut (I7). The real run is the least built (enrollment checkpointed after five rounds, SSH-M and Docker absent, A74 not started) and the dearest in the owner's time (18 actions a run, 72 for four runs, about an hour of presence each). It can show a provider or the engine wrong; it cannot refute the claim further than the model did.

(d) A contract that needs a special case in the engine. Predicts that no engine code depends on which provider serves. The cheapest observation is a search, and I made it: no provider, tool or scenario is named in the engine's source, on main or on order 080's branch (I8). Through experiment 022's harness the test waits for suites that hold no case and for H2; its one pair, local-process and SSH-R, shares the kit, so a pass says little. A failure would be fixed as a defect.

(e) Mine. None better than (b). Two companions cost almost nothing and bear on the other clauses: S10's edit ledger on paper (I9) and the MC2 and MC4 re-checks (I7).

**Evidence.** experiments/021-s10-swapped-providers/README.md:133-145 (allowed edits), :157 ("Record a forbidden edit as a failed comparison ... and seek a revised plan"); experiments/021-s10-swapped-providers/plan.md:3-27 (24 admin items; 042, 077, 081, 082 merged per git log; 079 only on origin/s3/m2), :101-102 (ten and two replays, "about 1 hour each"); engine/CONFORMANCE.md:41 ("Runtime provider calls and remote connections are unserved"); experiments/022-provider-contracts/suites/*/index.yaml:2 (`cases: []`); experiments/012-model-checking/G5.md:353, :356; docs/implementation/HANDOVER.md:211-213, :232; ROADMAP.md:69 (A74 "open: no one has started it"); ls experiments/ shows no 006.

**Verifier.** partly. Every citation reopened and holds:
- experiments/021-s10-swapped-providers/README.md:131-158 (D16).
- plan.md:4-27: 24 admin items; only 042, 077, 081 and 082 are on main (git log).
- plan.md:101-102 (ten and two replays).
- engine/CONFORMANCE.md:41 ("Runtime provider calls and remote connections are unserved").
- The four experiments/022-provider-contracts/suites/*/index.yaml hold `cases: []`.
- experiments/012-model-checking/G5.md:353 and :356 (7 actions; 33 and 38 actions).
- docs/implementation/HANDOVER.md:211-213 and :232.
- ROADMAP.md:69 (A74 not started).
- experiments/014-s3-two-machines/plan.md:270 (18 actions a run, 72 for four).

The evidence carries "(b) is the cheapest to run now": everything it needs exists and it needs no owner presence. It does not carry "the only one whose failure would be decisive".

**Verifier's counterargument.** - (b): by the project's rule a divergence "is a defect in the language, and is fixed in the language" (docs/implementation/INDEPENDENT_BUILDS.md:30-31). Six rounds and 53 findings were handled that way. A failure is decisive only if the owner accepts I1's consequences first.
- (a): the analyst's own I9 says the strict reading "fails on paper today". That is a contradicting observation available in a day with no engine, faster than (b). It bears on the roadmap's wording "the same documents behave the same on either" (ROADMAP.md:131), not on the claim's operational form (two implementations pass the same vectors, ROADMAP.md:116-117).
- (c): the models are bounded projections (G5.md:420: no claim spans "actual cryptography, filesystem persistence or engine conformance"). The claim says the conventions hold "on one machine and across several" (ROADMAP.md:114-115). So S3 tests something the models cannot.
- (d): the search is of an engine that serves no provider call. The first real observation is order 080 merged and one packaged provider driven through the engine.

**Verifier's corrections.** - (a): the plan gives "about 1 hour each" as each replay's length; only S3's rows say "owner presence" (experiments/014-s3-two-machines/plan.md:86-89).
- (b): the bound is the 148 promises the frame cites, not 292.
- (c): say "modelled", not "answered".


## FR-439

**The claim's central clause has never been measured, and its one test is last in the order and does not gate v0**

E05 I4 · fact · verifier: partly · decides: none

Six rounds of cold readers found about 170, 125, 44, 38, 119 and 6 problems; none was clean, and ruling 38 withdrew the two-clean-rounds bar because "four rounds showed reading alone does not reach" it. The first build then logged 53 findings on S1 alone (18, 8, 7 and 20 by increment). No one has measured how often a reader with only spec/ reaches the engine's verdict. The method the roadmap names for "unambiguity", two independent builds (A34), is step 7, "later", behind G1 to G11; G2 (a vector for each promise), G5, G6, G8, G9, G10 and G11 are unmet; and under ruling 47 its result informs and does not gate. So S1 to S10, the vectors and model checking can all pass with the first clause untested. No held-out vector set exists: `conformance-held-out/` is a name in a plan.

**Evidence.** docs/proposals/reaudit/README.md:11 ("none was clean"), :15-21; docs/AUDIT.md:446 (A59: "six rounds, none clean"), :468; docs/proposals/first-build/README.md settlement register: grep of row anchors gives 18, 8 and 7 for increments 1 to 3; log-increment-4.md has 20 rows 4-NN; ROADMAP.md:32, :133; docs/implementation/INDEPENDENT_BUILDS.md:86, :112-116, :132-134; docs/proposals/ruling-47/README.md:31; IN_FORCE.md:282; grep -rni 'held-out' finds only AUDIT.md:464, :468 and INDEPENDENT_BUILDS.md.

**Verifier.** partly. Holds:
- docs/proposals/reaudit/README.md:11-22: about 170, 125, 44, 38, 119 and 6 findings; "none was clean".
- The first build's findings: 18, 8 and 7 anchors in docs/proposals/first-build/README.md and 20 rows in log-increment-4.md, 53 in all.
- ROADMAP.md:32 and :133.
- docs/implementation/INDEPENDENT_BUILDS.md:86 (no held-out directory), :112-121 (the gate), :132-134 (G8 to G10 not met).
- docs/proposals/ruling-47/README.md:29-31 and IN_FORCE.md:282 (the builds inform and do not gate).
- grep finds "held-out" only in AUDIT.md:464, :468 and INDEPENDENT_BUILDS.md; no such directory exists.
- G2 is unmet by my recount (227 promises with no case); G5 is not declared met (experiments/012-model-checking/G5.md, "What G5 still needs").

Overstated: "never been measured" and "its one test".

**Verifier's counterargument.** Three things already bear on the same clause:
- The fresh-author run of 2026-11-10. A Codex session that had never seen the project, given spec/, a task page and a commands page, wrote 46 documents with 10 mistakes and logged 14 places where it was unsure (docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md:12-27). That is an author-side rate.
- The roadmap's own Read row. Round 7 by cold readers who see only spec/ comes after step 3 and before step 4 (ROADMAP.md:28; A59 at docs/AUDIT.md:448). It is a second scheduled test, well before step 7.
- The first build itself: workers built from spec/ and logged every place the text failed.

What is true is narrower: no scored, blind rate exists, none has been taken since the text grew 30%, and the one test by independent implementation does not gate v0.

**Verifier's corrections.** - A59's row is docs/AUDIT.md:448; line 446 is A53.
- Round 6 was one Codex reader (docs/proposals/reaudit/README.md:22).
- Title: "never scored" in place of "never measured"; "its one test by independent builds" in place of "its one test".


## FR-440

**"A few shared conventions, as invariants, not step by step" is contradicted by the catalogue**

E05 I5 · fact · verifier: partly · decides: none

The convention layer is the largest: 245 promises of 519 (47%), against 226 language and 48 interface. It grew from 179 on 2026-11-06 to 245 on 2026-11-12 (+37%); the language layer went from 220 to 226. Its obligations run to 12,014 words against the language layer's 7,159; 34 convention promises have a `then` of more than 100 words, and ACC-033 has 486. Only 132 of the 245 sit in the seven convention files; 113 sit in topic files, 63 of them in providers, secrets, enrollment and pending. Many are procedures: PRV-007 says how a number is turned into a string for a run; PRV-026 says what the engine does, in order, after it restarts. Round 6 said it on 2026-11-07, at 207 convention promises: "'Few shared conventions' names seven topics, not a small behavioral specification"; the count has risen by 38 since.

LD1's test (a rule is a convention if two parts, or a safety promise, would break by disagreeing) admits all of this, so the test and the description disagree. I would not ask for a ruling now. If I1 fails, the claim's wording is restated with it; if it passes, the coordinator reports provider-contract promises apart from the seven conventions' own, so that the count the owner reads is the count of each kind.

**Evidence.** Script over spec/promises, spec/conventions, spec/interface at 2eb8f258: Counter(convention 245, language 226, interface 48); per file: access 35, lifecycle 36, operations 32, revisions 14, records 6, observation 5, implicit 4 (132); providers 26, secrets 15, pending 12, enrollment 10, placement 10, conduits 9, pools 6, packages 6, templates 6, naming 4, first-start 3, fleets 2, objects 2, instantiation 1, submissions 1 (113). git grep of `layer:` at each of the 19 commits that changed those directories: e271d1c7 (2026-11-06) 179/220/35; 575a1bcd (2026-11-12) 245/226/48. docs/AUDIT.md:382 ("The few behaviors every part must agree on ... Specified as invariants, not step by step"), :406; spec/README.md:62, :253; spec/promises/providers.yaml PRV-007, PRV-026; docs/proposals/reaudit/round-6/design-critique.md:585-589.

**Waits.** The wording of the layered claim, after I1's result

**Verifier.** partly. Every number reproduces.
- Layers now: 245 convention, 226 language, 48 interface.
- At e271d1c7 (2026-11-06): 179 / 220 / 35. At c4d7d332 (round 6): 207 / 202 / 43.
- Convention promises: 132 in the seven convention files, 113 in topic files (providers 26, secrets 15, pending 12, enrollment 10).
- `then` words: convention 12,014, language 7,159. 34 convention promises have a `then` over 100 words; ACC-033 has 486.
- docs/AUDIT.md:382 and :406, spec/README.md:62 and :253, and design-critique.md:585-588 read as cited. PRV-007 and PRV-026 say what the item reports.

My own decomposition, by git show at each commit:
- The seven files went 127, then 122, then 132: up 4% in six days.
- Almost all the growth is in topic files, 52 to 113: providers 0 to 26 and secrets 0 to 15 by 2026-11-07 (ruling 40), then pending 12 and enrollment 10 (batch 1).
- Only a net two promises changed layer; the rest are new.

**Verifier's counterargument.** "Contradicted" is too strong for "few". The table counts conventions, and there are still seven (LD3; spec/README.md:62). Ruling 40 put provider contracts into the specification as a layer "beside these" (docs/AUDIT.md:395-400), and spec/README.md:64-70 says their promises "fall in the three layers". So 41 of the 113 are there by ruling, and pending and enrollment are features the owner ruled.

The evidence does carry two narrower points. LD1's test now files provider contracts and feature procedures as `convention`, so the layer's count no longer measures the seven conventions. And the layer's obligations doubled in words (5,854 to 12,014) and are not written only as invariants.

The analyst's remedy, reporting provider-contract promises apart, fits this and is the coordinator's.

**Verifier's corrections.** - Add the split: seven files 127 to 132; topic files 52 to 113.
- "The language layer went from 220 to 226" hides a dip to 202 at round 6 (67 promises removed and 85 added between 2026-11-06 and 11-07). Since round 6 the language layer grew by 24 and the convention layer by 38.
- Title: "the convention layer as LD1 assigns it is neither few nor invariants only; the seven conventions themselves are stable".


## FR-441

**Obligations are 22,622 words; the `observable` fields are 69,486, and they carry binding text**

E05 I6 · fact · verifier: supported · decides: coordinator

The form of a promise says `then` is "the single obligation" and `observable` is "what can be checked to tell whether the promise was kept". In 470 of the 519 promises the `observable` is longer than the `then`; in 257 it is more than three times as long. The `observable` fields hold 2,524 references to other promises. They state behavior that the `then` does not: LIF-013's `then` is 22 words and its `observable` 705, among them that the status lists every unreconciled invocation and that whoever may inspect sees the whole inventory; PRV-003's `observable` states "the one exception to the `url` that `run.v1` requires"; PRV-021's states what a start does when a stop is owed. A builder cannot tell from the form whether such a sentence binds. Round 6 reported this ("several important decisions live there rather than in `then`"); the accepted remedy changed reading paths only. Also, 517 of the 519 promises carry `vectors: []` in their own file; coverage is recorded only in the manifest and the cases. I1's third reader (M6) measures how much of the meaning sits outside `then`.

**Evidence.** Script: words of when 10,093, then 22,622, observable 69,486; observable longer than then in 470 of 519; more than 3x in 257; 2,524 matches of [A-Z]{2,5}-\d{3} inside observable fields. spec/README.md:291-292, :297; spec/conventions/lifecycle.yaml:231-261; spec/promises/providers.yaml:85, :661-664; docs/proposals/reaudit/round-6/design-critique.md:595; docs/proposals/reaudit/round-6/DECISIONS.md:125; grep -c 'vectors: \[\]' over the promise files sums to 517.

**Waits.** Whether binding sentences move into `then` (a fix that adds no behavior: rulings 37; 46 point 4), after M6

**Verifier.** supported. Recounted:
- `then` 22,622 words; `observable` 69,486; `when` 10,093.
- `observable` is longer than `then` in 470 of 519 promises, and more than three times as long in 257.
- 2,524 promise references sit inside `observable` fields.
- 517 promises carry `vectors: []`; only CTR-013 and SCH-009 list any.

Read as cited:
- spec/README.md:291-292 and :297.
- LIF-013 at spec/conventions/lifecycle.yaml:231-261: `then` 22 words, `observable` 705, with the `unreconciled` list and the whole-inventory rule only in `observable`.
- spec/promises/providers.yaml:661-664 (a start that finds a stop owed) and the lines after it ("the one exception to the `url`").
- design-critique.md:593-595; DECISIONS.md D21 (reading paths only).

grep finds no sentence in spec/README.md, IN_FORCE.md, AGENTS.md or docs/guides/cold-readers.md saying whether `observable` binds. The ratio is old: `observable` was 38,184 words against 13,517 on 2026-11-06.

**Verifier's counterargument.** The remedy and who decides it need care.
- Moving sentences into `then` collides with the ruled form: "A promise with 'and' in its `then` is two promises" (ruling 17 NS2, docs/proposals/normative-surface/README.md:88; spec/README.md:297). Each moved sentence becomes a new promise, and the owner wants fewer.
- Whether a sentence in `observable` binds today is undetermined, so moving one can add behavior. Then it is not a no-behavior fix under rulings 37 and 46 point 4.

A cheaper alternative follows the usual normative and informative split (RFC 2119 sections; W3C's "non-normative"). One sentence in the form states that `when` and `then` bind and `observable` says only how to check. A mechanical sweep then lists every `observable` sentence that states behavior found in no `then`. Each is deleted, or put to whoever decides as a gap. The sentence about the form touches ruling 17, so it is at least logged for the owner to strike.

**Verifier's corrections.** "decides: coordinator" holds for the sweep and for deletions that change nothing. It does not hold for a move that makes a sentence binding.


## FR-442

**Candidate (c) has been answered by model checking; re-check the changed text first, and treat S3's real run as a demonstration**

E05 I7 · recommendation · verifier: partly · decides: coordinator

What exists. On 2026-11-11 the models refuted or contested 29 of the 143 promises they modelled, against 114 that held in bounded projections. MC2-001, a contradiction between OPS-014, OPS-024 and OPS-018, appears in 7 actions with timers alone: no partition is needed. MC4-002, two runs of one instance side by side, appears in 33 and 38 actions with a cut and no declared loss. The remedies added ordering rules, not invariants: a fence before a negative reconciliation (077, with two limits the owner accepted), PRV-030 and the changed PRV-019 (081), and for MC4-002 a stated limit plus an option in each shipped runtime provider's own contract ("This is the provider's obligation under its contract, not an engine rule"). Holding that option cost the kit three orders (129, 130, 141) and SSH-R a drainer rebuilt over three (146, 149, 150), with its state in an operator's directory outside the engine's store, because PRV-029 deletes the only state place the specification gives a provider and PRV-021 requires one that survives the provider's retirement. G5's standings predate 077, 081 and 082: on the text as it stands, no model has been run.

The change. Move the lane's queue item 7 (the MC2 rerun on 077's text; MC4's P1 registered again against 081) to first place. Cost: about two lane orders, no owner time. It buys the second clause checked on the current text and G5 counted on 219 safety promises. Do not spend the owner's 72 barrier actions on S3 until those models pass: S3's real run shows whether SSH-R and the engine do what their text says, and its P4 ("zero pending stops cannot pass") tests a provider, not the claim. S3-OF can wait until then.

**Evidence.** experiments/012-model-checking/G5.md:33 (holds 114), :49 ("114 `holds` and 29 refuted or contested"), :346 ("Orders 077, 081 and 082 have **not yet run**"), :353, :356; docs/work-orders/077-spec-reconcile-fenced.md Record; docs/work-orders/081-spec-one-run-per-instance.md:625 and Record ("PRV-030 is one more convention-layer safety promise for G5 (219 with order 082's PKG-032)"); spec/promises/providers.yaml:627-664 (PRV-021), :986-1004 (PRV-030); docs/work-orders/145-ssh-r-durable-dir.md:15 onward ("retiring the provider removes its instance, and PRV-029 (LIF-035) then removes that directory"); docs/implementation/HANDOVER.md:303 ("149's drainer handover raced in every round"); docs/implementation/PROVIDERS.md:59 (queue item 7); experiments/014-s3-two-machines/README.md M1 ("zero pending stops cannot pass P4").

**Waits.** S3-OF and S3's fault harness can wait on it; MC9 is unaffected

**Verifier.** partly. Holds:
- experiments/012-model-checking/G5.md:33 and :49: 114 hold; 29 refuted or contested (7+2+3+4+2+4+3+2+1+1).
- G5.md:346 (077, 081 and 082 "not yet run"); :353 (MC2-001, 7 actions, timers alone); :356 (MC4-002, 33 and 38 actions in two model versions).
- spec/promises/providers.yaml:664 ("the provider's obligation under its contract, not an engine rule").
- docs/work-orders/081-spec-one-run-per-instance.md:625-626 (the shell's runner implements neither guarantee; 219).
- docs/work-orders/145-ssh-r-durable-dir.md:14-22 (PRV-029 removes the directory PRV-021 needs).
- Orders 129, 130, 141 and 146, 149, 150 exist; docs/implementation/HANDOVER.md:303.
- docs/implementation/PROVIDERS.md:59-61 (queue item 7, "unblocked").
- experiments/014-s3-two-machines/README.md:155 ("zero pending stops cannot pass P4").

Does not hold as worded:
- "The remedies added ordering rules, not invariants." OPS-014 ("failed only once the invocation is fenced"), PRV-019 ("starts no other run") and PRV-030 (a `must not`) are safety conditions, which is the form of an invariant.
- "On the text as it stands, no model has been run." The ruled reading was modelled and held: "MC2 Fenced, MC7a Correct and MC6 Correct hold on that reading" (G5.md:353). What is unchecked is 077's wording with its two limits, and 081's PRV-019, PRV-021 and PRV-030.

**Verifier's counterargument.** For the change: moving queue item 7 first is cheap, needs no owner time and is the coordinator's. It checks the repaired text before more is built on it.

Against the framing:
- The 29 are a count from 2026-11-11, before the fixes. Seven of them "hold under ruling 75 A1", five are ambiguities and two are over-broad properties of the models. The result shows the method finding defects in a day, not the second clause refuted.
- S3 cannot be relabelled a demonstration without the owner. The roadmap calls it "Central to the claim since ruling 40" (ROADMAP.md:141) and ruling 47 requires S1 to S10 to pass. It tests what the models leave out (G5.md:420).
- Holding back the 72 barrier actions costs nothing today: S3 waits on 049a1, 080 and SSH-M in any case.
- The lane coordinator has to be restarted before its queue can be reordered (HANDOVER.md:92-110).

**Verifier's corrections.** - Say "the fixes are stated as safety conditions; whether they hold together is unchecked on the merged text".
- Say "the ruled reading was modelled; the merged wording was not".
- Keep S3 as required by ruling 47, sequenced after the re-checks.


## FR-443

**Candidate (d): the engine's code names no provider; the special cases are in the specification and in the built-ins**

E05 I8 · fact · verifier: partly · decides: none

Working. A search of the engine's source (core, shell and loader, tests excluded) for ssh, local-process, docker, tailscale, keychain, 1password, linear, codex and claude finds nothing, on the reviewed commit and on order 080's unmerged branch. The core emits the same operation for a built-in run and a provider's run and differs only in the intent's kind. No lane order (providers, scenario servers) changed engine/ or spec/. The one tool named in the engine is npm, in the built-in runner, which DL16 declares.

Not working. (1) The specification special-cases the engine's own runner: it returns no `url`, "the one exception"; no gate stands before it; a relative `command` item runs only on it; providers themselves always run on it. (2) The built-ins are a second implementation in the engine's tree, never run against the contract suite ("A63-H drives no built-in"), and 081's Record says "the shell's own runner implements neither new provider guarantee". Kubernetes took dockershim out of the kubelet in 1.24 to stop carrying exactly this. (3) The specification followed provider code at least twice: PRV-021's three cases are to be written "matching kit order 141", and `place.v1` gives a hostname where PRV-015 names a machine by ID. (4) Scenario work found gaps on paper before any run: S2 G1, S3 G4 and G5, S4 G1 and G2, S7 G1 in the language or conventions; order 143 found that IFC-006 gives a server the target instance and not the sender, though OPS-008 scopes a key by sender. S2 G2 and S8 G2 and G3 went to package contracts, and S6's and S9's openings to DL1 and DL6, as the claim predicts.

**Evidence.** grep -rniE 'ssh|local-process|docker|tailscale|keychain|1password|linear|codex|claude' engine/crates/{core,shell,loader}/src minus test files: 0 lines; git grep of the same on origin/wo/080-shell-runtime-provider-calls (outside the pinned commit): 0 lines; grep -c npm engine/crates/shell/src/runner.rs: 30. engine/crates/core/src/lifecycle.rs:800 (`if runtime.is_some(){"provider-call"}else{"builtin-runner"}`); engine/crates/shell/src/lib.rs:76-85. spec/promises/providers.yaml:66-85 (PRV-003), PRV-002, PRV-007. experiments/021-s10-swapped-providers/plan.md:176; docs/work-orders/081-spec-one-run-per-instance.md:625; docs/work-orders/141-a63-rk-own-owed-stop.md:134; docs/guides/full-review.md:132-137; docs/work-orders/143-s4-srv1-effect.md:330; experiments/013-s2-shared-login/README.md:213-220; experiments/014-s3-two-machines/README.md:229, :234; experiments/015-s4-partial-setup/README.md:211, :214; experiments/018-s7-outside-package/README.md:187; experiments/019-s8-filtered-search/README.md:187, :191. git log since 2026-11-11: no commit touching packages/ or experiments/ together with engine/, spec/ or conformance/ other than admin orders 039 to 041b, 063 and 067.

**Verifier.** partly. Holds, and more strongly than stated:
- grep -rniE 'ssh|local-process|docker|tailscale|keychain|1password|linear|codex|claude' over engine/crates/{core,shell,loader}/src returns 0 lines with the tests included, and 0 in engine/crates/conformance/src. The same search on origin/wo/080-shell-runtime-provider-calls (outside the pinned commit) returns 0.
- engine/crates/core/src/lifecycle.rs:800 and engine/crates/shell/src/lib.rs:76-85 read as cited.
- npm appears 30 times in engine/crates/shell/src/runner.rs.
- PRV-002, PRV-003 and PRV-007 state the four special cases of the engine's own runner.
- experiments/021-s10-swapped-providers/plan.md:176, docs/work-orders/081-spec-one-run-per-instance.md:625, docs/work-orders/143-s4-srv1-effect.md:330 and the scenario gap lines in experiments/013, 014, 015, 018 and 019 read as cited.
- Since 2026-11-11, the only commits touching experiments/ or packages/ together with engine/, spec/ or conformance/ are admin orders (041a and 041b in my check).

**Verifier's counterargument.** Two sub-points are weaker than they read.
- "No lane order changed engine/ or spec/" is true by rule, not by evidence. A lane coordinator may merge only its own paths (docs/implementation/HANDOVER.md:19), so every engine change the providers need became an admin order: 079, 080, PSD1a, PSD1b, PSD2, PSD3, SEC-D, FLT-D and A63-MDIS (docs/implementation/PROVIDERS.md:6-10; experiments/021-s10-swapped-providers/plan.md:4-11). Those orders are the place to look for provider-shaped engine machinery, and the item does not look there.
- The engine on main serves no provider call (engine/CONFORMANCE.md:41; lib.rs:83 returns Missing). The absence of special cases is the absence of the code that would hold them.

The dockershim comparison is apt as history. Taking the built-ins out would run against ruling 46 and PRV-003, and the item rightly does not propose it.

**Verifier's corrections.** - Point (3): the specification has not "followed provider code". Neither change is written (HANDOVER.md:215, "Not written"). Both are defects in the text that a provider build surfaced (order 141's worker finding; docs/guides/full-review.md:132-135), which is the feedback the layered claim expects.
- `node` is named too (engine/crates/shell/src/runner.rs:495); DL16 declares both.


## FR-444

**S10 (ROADMAP.md row S10, experiment 021): add a paper edit ledger now, and swap the pair that passes the suite**

E05 I9 · experiment-change · verifier: partly · decides: owner

THE CLAIM IT TESTS. "Every provider can be swapped"; the roadmap's method says "the same documents behave the same on either".

WHAT IS WRONG NOW. (1) Nothing can run: the four contract suites hold no case; secrets, package source and machines have one implementation each; the engine on main serves no provider call (080 is checkpointed with three musts). (2) The two runtime providers share the kit: 3,642 source lines of kit against 871 for local-process and 2,883 for SSH-R, so "two implementations pass the same vectors" would show one kit on two backends. (3) As accepted, P4 can hardly fail: D16 allows instance `runtime` selections, package `fetch` URIs and re-created instances and Conduits. Under the stricter reading in the review's brief (any edit to another document fails), S10 fails on paper today, on my reading of PRV-006, PRV-003 and PRV-016: an instance's own document names its runtime for life, and a manifest's `fetch` scheme selects the package source. (4) Decision (b) runs the suites on the packaged local providers and the replays on the built-ins, so P3 and P4 are shown on different implementations.

THE CHANGES. 1. Add S10-P0, on paper, now. From spec/ and the S1 and S2 documents as they stand, write D16's ledger for each of the three swaps: documents edited other than the provider's own Resource document and its package pin; instances replaced; Conduits re-created; each with the promise that forces it. Measure: those three counts per swap. Pass condition P0: no edit to any other document; replacements are counted, not failed. One order, about a day, no engine and no owner time. If P0 fails, the owner sees the number before a second provider is built, and the claim's wording is corrected to what holds. 2. Make S10's first configuration the packaged local providers, named explicitly, in place of the built-ins, so that the pair that passes the suite is the pair that is swapped; S1 already exercises the built-ins. With a provider named in the first configuration, a swap can reuse its name and leave every instance document as it was. 3. In D15, report the kit as shared work and the runtime result as one kit on two backends, unless one runtime provider is written without the kit.

WHO DECIDES. 1 and 3 are the coordinator's: a plan, and a measure D15 already defines. 2 is the owner's: ruling 75 D accepted the five configurations, and decision (b) is on the list the owner may strike. I recommend 2. It can ride with question I2 as one more letter in the same answer.

**Evidence.** ROADMAP.md:113-116, :131, :148; experiments/021-s10-swapped-providers/README.md:44-48 (P3), :49-51 (P4), :133-145 (D16), :264 (limits); experiments/021-s10-swapped-providers/plan.md:28, :176, :219; experiments/022-provider-contracts/suites/{runtime,machines,package-source,secrets}/index.yaml (`cases: []`); experiments/022-provider-contracts/README.md:107 ("No real provider has been tested and there are no contract cases yet"); ls packages/ (provider-file, provider-local-directory, provider-local-machine, provider-local-process, provider-ssh-runtime, runtime-provider-kit); find &lt;crate>/src -name '*.rs' | xargs cat | wc -l: kit 3,642, local-process 871, ssh-runtime 2,883, file 242, local-directory 373, local-machine 476; engine/CONFORMANCE.md:41; engine/crates/shell/src/lib.rs:83; docs/implementation/HANDOVER.md:212; spec/promises/providers.yaml:169 onward (PRV-006: "which the instance holds ... for its whole life, and no operation changes it").

**Waits.** A63-DR, A63-GS and the second store need not start before P0's number is known

**Verifier.** partly. Holds:
- ROADMAP.md:113-117, :131, :148.
- experiments/021-s10-swapped-providers/README.md:44-51 (P3, P4) and :131-158 (D16).
- plan.md:28, :176, :219 (decision (b); "A63-H drives no built-in").
- Four suites with `cases: []`; experiments/022-provider-contracts/README.md:107.
- Source lines: kit 3,642; local-process 871; ssh-runtime 2,883; file 242; local-directory 373; local-machine 476. Both runtime crates depend on the kit by path, and the planned Docker provider will too (docs/implementation/PROVIDERS.md:159).
- PRV-006 (spec/promises/providers.yaml, `then`): the instance holds its `runtime` "for its whole life, and no operation changes it".
- engine/examples/s1 names no `runtime`, so a move off the built-in runner edits every run instance's document.

Does not hold:
- "P4 can hardly fail." P4 also requires the same results and attribution and "No provider-specific branch in relationship semantics" (README.md:149), and a forbidden edit is "a failed comparison" (:157). It cannot fail on the number of allowed edits; it can fail on the rest.
- "From the S1 and S2 documents as they stand." No S2 author documents exist (experiments/013-s2-shared-login holds README, plan and linear-stub; packages/scenario-s2-* are packages). S1 has no secret (grep finds 0 in engine/examples/s1). The second providers' Types do not exist. The secret-store ledger cannot be written today.

**Verifier's counterargument.** - Change 1: the count is M2/D16 already (README.md:124); only doing it now, on paper, is new. That count is the coordinator's. As a pass condition, "no edit to any other document" is stricter than the D16 the owner accepted under ruling 75 D (README.md:25-27), so it amends an accepted pre-registration and is the owner's. Its result is also already known by the analyst's own reading, which makes it a finding and not an experiment. State the finding: under PRV-006 and PLC-009, "the same documents" is false for a runtime swap off the default. Then ask whether the roadmap's wording follows D16.
- Change 2 follows Kubernetes: a Pod names a `runtimeClassName` and the handler changes behind the name. But it removes the transition an author meets first, from no provider named to a named one, which is the dearer. Better to keep both: built-in to second counted as the cost of leaving the default, and packaged-local to second judged for P3 and P4. Either needs order 080 on main.
- Change 3 is right and the coordinator's. One provider written without the kit, ideally not in Rust (IN_FORCE.md:91 lets an outside author use any language), is what would show "any tool can implement".

**Verifier's corrections.** - Limit P0 to S1's runtime and package-source swaps until S2's documents exist.
- Call P0 a reported count, or send it to the owner as an amendment to D16.
- SSH-R is S3's provider; S10's runtime pair is local process and Docker (plan.md:171-174).


## FR-445

**Any vector-based result is bounded: 292 of 519 promises are cited by a case**

E05 I10 · fact · verifier: supported · decides: none

By layer: language 129 of 226, convention 141 of 245, interface 22 of 48. No case cites any promise of sections.yaml (15), enrollment.yaml (12) or pending.yaml (13); secrets has 2 of 24, providers 9 of 30, placement 7 of 21. The 227 uncited promises are where the text has grown fastest (the short form, enrollment, pending actions, providers), and they are the ones I1's stage 2 must reach by another route. G2 (each promise has a vector or is on the implementation-defined list) is unmet by 227. Rule 5 of the form already says a promise with no vector "does not count as covered"; taken at its word, v0's shown claim covers 56% of the catalogue.

**Evidence.** Script: union of `promises` over the 494 case files against the 519 promise IDs: 292 cited, none unknown; per file: sections 0/15, enrollment 0/12, pending 0/13, secrets 2/24, providers 9/30, placement 7/21, contracts 4/12, implementation 23/46, operations 30/32, records 6/6. spec/README.md:301; docs/implementation/INDEPENDENT_BUILDS.md:112.

**Verifier.** supported. Recounted from the `promises` lists of the 494 case files against the 519 IDs.
- 292 cited, none unknown. Step-level `required_promises` and conformance/manifest.yaml give the same 292.
- By layer: language 129 of 226, convention 141 of 245, interface 22 of 48.
- By file: sections 0/15, enrollment 0/12, pending 0/13, secrets 2/24, providers 9/30, placement 7/21, contracts 4/12, implementation 23/46, operations 30/32, records 6/6.
- spec/README.md:301 (rule 5) and docs/implementation/INDEPENDENT_BUILDS.md:112 (G2) read as cited.

**Verifier's counterargument.** 56% is an upper bound on promises touched, not covered. The specification's own words are "vectors test slices of N promises", and the brief says "A passing vector shows its slice, not the whole promise" (docs/guides/full-review.md:75-78). Read strictly, the shown share is smaller than 56%.

**Verifier's corrections.** The binding spec/README.md lags the cases. Its per-file coverage statements (spec/README.md:191-252) sum to 274, 18 behind: objects 13 against 15, lifecycle 18 against 25, access 23 against 25, revisions 4 against 9, conduits 12 against 14. This is a coordinator's fix.


## What the verifier says the analyst missed

- **Batch 1's specification is already written; the orders the analyst would pause are build orders** 036a, 036b, 037, 038a, 038b and 038c are merged (git log), and IN_FORCE.md:68-97 marks batch 1's lines "In spec/". 046, 047, 048a, 048b, 050 and 053 build the engine and the Agent packages (docs/proposals/spec-batch-1/README.md:545-657). This undoes I1's consequence (a) as worded, I2's option B and I2's "waits". If (a) means those orders, it holds the S1 rerun path that ruling 71 points 2 and 3 ordered, and neither item says so.
- **The first clause's wording has already moved, and no experiment is needed to see it** LD1 files a rule as language "if it can be answered by reading the declarations alone" (docs/AUDIT.md:406). spec/README.md:61 now says resolving "reads the documents against what already exists, the lifecycle state and the status of the objects". In my recount, 113 of the 264 document-only cases need a state snapshot and 139 are tagged language and convention together. The meaning of a set of documents is a function of the documents and the existing state. The analyst's stratum B rests on this and does not name it. It is a wording question for the owner only if the claim is restated.
- **The frame is not held out from the text's authors; a mutated or generated stratum would be** The existing vectors were written with the promises and repaired with them. Documents a fresh reader mutates or writes, judged by the reference engine, are the only inputs the text was never fitted to. LD4 already names this method ("generated documents, where any difference in output is a finding", docs/implementation/INDEPENDENT_BUILDS.md:26), and A44 (mutation testing) is its registered sibling. The analyst defers it to stage 2.
- **An existing base rate was not used** The fresh-author run of 2026-11-10 gave 10 mistakes and 14 logged uncertainties in 46 documents, from spec/ plus a task page and a commands page (fresh-author-run-2026-11-10.md:12-27). It is the nearest existing measurement to I1 and could set I1's thresholds. The summary's "from spec/ alone in 46 minutes" is loose on both counts: 45 min 48 s, and not spec/ alone.
- **The search of the Records for engine machinery added for one provider or scenario was not done as asked** The analyst searched engine source for names. Candidates the Records give:
- order 045: PRV-029's state directory, "which is an Agent's home" (docs/work-orders/045-build-state-directory.md);
- order 038c: four login kinds in the specification for particular clients' logins (HANDOVER.md:266-267);
- order 074: a session idle limit for hours-long Agent turns;
- DL16: the built-in runner runs npm and stdio only (engine/CONFORMANCE.md);
- admin orders that exist only because providers need them: 079, 080, PSD1a to PSD3, SEC-D, FLT-D, A63-MDIS.
Each is general in form, so none is a special case. Together they show spec/ and engine growth driven by particular providers, which is the evidence the assignment asked to be weighed.
- **Rulings that bear on a reading-based test were not weighed** For it: ruling 12 ("All identifiable language defects are to be closed first, and the builds wait until success is very likely", docs/proposals/ruling-log.md:111) and the gate's stated reason (INDEPENDENT_BUILDS.md:106-107). Against a reading gate: ruling 38 point 4 and ruling 44 point 1. The pre-registration should cite both sides.
- **The test will probably not decide on its first sample** With about 76 scored steps, pass at 2 or fewer and fail at 6 or more, a true rate near 5% lands "between" a little more than half the time (my binomial arithmetic). Either seal a larger sample or narrow the band before the run.

## The verifier's notes

Verified at 2eb8f258 (2026-11-12), read-only. Nothing was written, built or run beyond read-only git, grep and /usr/bin/python3 counts.

**Overall.** The analyst's numbers are sound: every count I recomputed reproduces. The weaknesses are in what is built on them:
- I1 and I2 rest on a wrong picture of batch 1. Its specification is merged, and the orders named are builds.
- The chosen falsifier samples the part of the text least likely to fail, and its failure is decisive only by prior agreement.
- I5 and I7 state as refutations what the evidence shows as growth by ruling and defects found and fixed.
- Two "decides" assignments are off: a new pass condition for S10, and moving `observable` text into `then`.

The cheapest sound path is for the coordinator to run the blind-verdict read now as an observational row, with a mutated stratum added, and to ask the owner only on a failing result.

**Read in full:** docs/guides/full-review.md; docs/implementation/HANDOVER.md; AGENTS.md "What holds"; docs/AUDIT.md "Layered claim", "Layer decisions", "Purpose"; docs/implementation/INDEPENDENT_BUILDS.md; IN_FORCE.md topics 2 and 13 to 15.

**Read in part:** ROADMAP.md (order of the work, register rows, proposing an experiment, program, scenarios); spec/README.md (what this is, contents, form, conformance); experiments/021 README and plan; experiments/012 G5.md (counts, findings, what G5 still needs); PROVIDERS.md lines 1-66 and grep; spec-batch-1 README (the work orders); the promises PRV-002, 003, 006, 007, 016, 019, 021, 026, 029, 030, OPS-014, OPS-024, LIF-013, CTR-013; the Records of 073, 104, 116, 123, 142; rulings 38, 44 and 47 by grep and section.

**Not checked:**
- Experiments 013 to 020 beyond the cited lines; rulings 75 and 77 in full; the engine's code beyond the cited lines and searches.
- How many cards I1 would drop as equal to spec/schemas/examples.
- The claims about CommonMark, WebAssembly, Kubernetes RuntimeClass and dockershim, which rest on my own knowledge and not the repository.
- The search of origin/wo/080 is outside the pinned commit.
