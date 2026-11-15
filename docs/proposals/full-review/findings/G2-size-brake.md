# G2-size-brake: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** At 2eb8f258 nothing in the tree can stop the specification growing. An addition needs one accepted recommendation; a removal needs a ruling for each promise, only after S3 to S9 and both independent builds, and the scoring that would feed it (A14) is scheduled after the prune. Since 2026-11-06 the catalogue went from 434 to 519 promises (152 added, 67 removed, none removed since 2026-11-07), `then` from 13,517 to 22,622 words, `observable` from 38,184 to 69,486, and core Operation contracts from 20 to 37, while implementation-defined items went from 15 to 17. The growth is text written ahead of the build: four text-only orders added 60 promises with no case, while the three changes led by a failing model check added 2. A24, adopted now as an entry rule (a promise merges with a check that cites it, and no order writes promises alone), is the brake that fits these facts: it needs no owner ruling, would have held 63 of 64 additions at entry, and 46 of them would still be spec-pending lines today; it limits the rate and removes no scope. A net-zero rule by count would have needed about 58 removals of ruled behavior for batch 1 and leaks into words, so I would not adopt it. A prune before batch 2 by A18's own test would present 97 promises (67 language, 18 convention, 12 interface) that are mostly the short form, unported validity rules and the purpose's own negative rules, with only 18 of the 64 recent additions among them; it is the wrong list and needs the owner three ways. The shortcut table should hold eleven rows from the ledger, five of them with no trigger for replacement. The one question for the owner is whether to close v0's feature list at what is ruled today, so that later additions go to after v0 unless the owner says otherwise; I recommend yes. What works and should be kept: changes led by evidence are small and arrive with cases, and build orders cite new promises within hours when the build is in step.

**What the analyst read.** Read in full: docs/guides/full-review.md; AGENTS.md; docs/AUDIT.md; the first pass's P08 I8 with P08's summary and verifier's notes; E05 I5, I6 and I10 with its verifier's notes; L13-framing; X01-catalogue-layers except F6, of which I read only the opening; the approach critic.md; the ledger's rows 1 to 77 and its history of rulings 1 to 37; ROADMAP.md lines 1 to 75 and the scenario table; the records of rulings 70, 74 and 77 (to line 60), ruling 75's decisions A to C, amendments and later answers, and the cited passages of rulings 38, 40, 44, 51, 53, 71, 72 and 76; CONTRIBUTING.md 'Writing promises'; spec/README.md 'The form of a promise'; S3's README to the end of 'What it covers'. Read in part: docs/implementation/HANDOVER.md (the queue, lines 196 to 232); IN_FORCE.md by grep (spec pending lines, provisional wording, topic 1); docs/proposals/spec-batch-1/README.md lines 40 to 100; the Fleet brief lines 1105 to 1150; the heads of 'What it covers' in four other scenario READMEs. By script (/usr/bin/python3 with PyYAML, read-only git): all 519 promises' fields; conformance/manifest.yaml and the 494 case files' promise keys; the README and plan of experiments 013 to 021 for promise IDs; conformance/s1-promises.yaml; promise IDs named in engine/crates, packages/ and G5.md; the 34 commits that touched spec/ and nine round-end commits; the manifest's cases at eleven commits against their parents; the first pass's language/INDEX.txt. Not read: the promise texts beyond about 30 `then` fields; the contents of any case; the engine's code; the scenario plans in full; ruling records other than those named; the first pass's other files and critic.json. Limits: a scenario 'cites' a promise where its ID appears in the text, with ranges and slash lists expanded; that is a plan's statement, and no scenario of S2 to S10 has run. The first pass's findings were assigned by slice, so their density by promise is indicative only. The comparisons with TC39, Kubernetes and W3C rest on my own knowledge, not on the repository. Nothing was written, built or run, and no git command wrote.


## FR-470

**Nothing in the tree can stop the specification growing: adding takes one accepted recommendation, removing takes a ruling per promise after S3 to S9 and both builds**

G2 I1 · assessment · verifier: partly · decides: none

The guard is one-sided. An addition enters spec/ when the owner accepts a ruling. Of the 19 ledger rows for rulings 47 and 49 to 77 that quote the owner, 17 are agreement with what the coordinator recommended (all but 47 and 60). A removal needs 'an explicit owner ruling (A18)', only after S3 to S9 have run. A18 is step 8, after both independent builds, and A14, which would score each promise exercised, contradicted or never needed, is scheduled after it. Ruling 71 point 4 is a second door for additions: a coordinator may settle a small finding that adds a feature.

What that produced, from git:

| | 2026-11-06 (e271d1c7) | before batch 1 (963e40b5) | 2eb8f258 |
|---|---|---|---|
| Promises | 434 | 455 | 519 |
| Words of `then` | 13,517 | 17,533 | 22,622 |
| Words of `observable` | 38,184 | 57,185 | 69,486 |
| Core Operation contracts | 20 | 23 | 37 |
| Implementation-defined items | 15 | 15 | 17 |

- 152 promises were added and 67 removed. All 67 removals are in one commit of 2026-11-07, where rulings 40 and 41 removed graphs, policies and the runtime model. Since then: 67 added, none removed.
- The only other shrink is order 032 (2026-11-10): spec/ went from 22,399 to 19,666 lines by removing narrative, and no promise changed. Both shrinks were deliberate passes; no standing rule produced either.
- Where the growth came from: four text-only orders (036a, 038a, 037, 038b) added 60 promises and 3,972 words of `then` with no case in the same change. The three changes led by a failing model check (077, 081, 082) added 2 promises and 474 words, each with 1 to 3 new cases.
- The 64 promises added since 2026-11-10 are 12% of the catalogue and hold 20% of its `then` words (mean 70.5 words, against 39.8 for the older 455).
- A count hides part of it: since 2026-11-10, 115 of the 455 older promises had their `then` or `observable` changed (+574 and +3,647 words), and order 077 added no promise and 796 words.

What is working: changes led by evidence are small and arrive with cases, and when the build is in step, build orders cite new promises within 3 to 37 hours (17 of batch 1's). What is not: text is written days ahead of anything that can fail on it, and the direction is the opposite of the layered claim's. Convention promises went from 179 to 245 while implementation-defined items, where the claim sends an engine's own behavior, went from 15 to 17. What I would change is in I2, I3 and I9.

**Evidence.** docs/AUDIT.md:316-321 (scope guard), :476 (A14), :482 (A18); ROADMAP.md:33-34 (step 8, then A14); docs/proposals/ruling-log.md:55 and :57-85 (rows 47, 49 to 77; the owner's words column); docs/proposals/ruling-71/README.md:43-45. Commands: `git log --reverse --format='%h %ad %s' -- spec/` (34 commits); for each, `git show &lt;commit>:&lt;file>` of every file under spec/promises, spec/conventions and spec/interface parsed with /usr/bin/python3 and PyYAML (IDs, layers, word counts of `then` and `observable`), `^kind:` documents in spec/core/operations.yaml, `^- id: DL` in spec/implementation-defined.yaml; `git show &lt;commit>:conformance/manifest.yaml` against `&lt;commit>~1` for new cases (036a, 036b, 038a, 037, 038b: 0 new cases; 075: 1; 038c: 1; 082: 1; 081: 2; 077: 3; 042: 48). Per-order `then` words: 036a +843, 038a +1,180, 037 +874, 038b +1,075, 075 +113, 038c +530, 082 +70, 081 +319, 077 +85 (sum 5,089 = 22,622 - 17,533).

**Waits.** I2, I3 and I9 act on it.

**Verifier.** partly. Every number reproduces. I parsed every promise file at all 34 commits of `git log --reverse -- spec/` and at 2eb8f258, using /usr/bin/python3 with PyYAML on `git show &lt;c>:&lt;file>`. Promises: 434 at e271d1c7, 455 at 963e40b5, 519 now. `then` words: 13,517 / 17,533 / 22,622. `observable` words: 38,184 / 57,185 / 69,486. `^kind:` in spec/core/operations.yaml: 20 / 23 / 37. `^- id: DL`: 15 / 15 / 17. Eleven commits added 152 promises; all 67 removals are in c4d7d332; 67 were added after it and none removed. `then` added per order: 843, 1,180, 874, 1,075, 113, 530, 70, 319, 85, which sums to 5,089. Order 077 added 85 words of `then` and 711 of `observable` (796). spec/ lines: 22,399 at f1cb4734, 19,666 at 963e40b5, 24,902 now. The 64 promises added since 963e40b5 average 70.5 words of `then` against 39.8 for the older 455, and hold 20.0% of `then` words with 12.3% of the count. Of the older promises, 115 changed `then` or `observable` (39 changed `then`), net +574 and +3,647 words. Ledger rows 47 and 49 to 77: 19 quote the owner. The 17 first citations, 3 to 37 hours after entry, reproduce from the manifest at each commit. docs/AUDIT.md:316-321, :476, :482, ROADMAP.md:33-34 and docs/proposals/ruling-71/README.md:43-45 read as cited.

**Verifier's counterargument.** The headline overstates. Owner rulings already act as a brake. Rulings 40 and 41 removed 67 promises in one commit (c4d7d332), on design grounds, before S3 to S9 and with no A18 list. Ruling 66 point 6 declined a Scope object and a grouping shorthand. Ruling 76 point 1 dropped the prune flag. Ruling 55 left B open, and ruling 62 held C6 to C12. So 'a ruling per promise, only after S3 to S9' describes A18's route only. It is not the only way out, and the analyst's own I6 asks for removal by group, as 40 and 41 did. The contrast with changes 'led by evidence' is also weaker than stated. 038c, 082 and 081 each came with cases, but none of those cases cites the promise the change added. PND-013, PKG-032 and PRV-030 are uncited today and sit on the analyst's own list of 97. On the layers, E05 I5's verifier already showed that the seven convention files grew only from 127 to 132 promises. The topic files grew from 52 to 113, mostly from ruled placements: provider and secret contracts under ruling 40, then pending actions and enrollment under rulings 49, 50 and 56. 'The opposite of the layered claim' is too strong. P08 I8's verifier also found that about 25 of the 64 trace to sign-in and enrollment rulings made before the bar failed.

**Verifier's corrections.** Title: 'No standing rule limits growth', not 'Nothing in the tree can stop the specification growing'. Replace '17 are agreement with what the coordinator recommended' with: '17 accept the recommendation, 7 of them with an exception, condition or deferral (55, 62, 66, 68, 72, 74, 75), 10 without.' Replace 'a removal needs a ruling for each promise, only after S3 to S9' with: 'A18's prune needs a ruling per candidate, after S3 to S9, at step 8; an owner ruling can remove a whole mechanism at any time, as 40 and 41 did.'


## FR-471

**Adopt A24 now, worded as an entry rule on spec/: a promise merges with a check that cites it, and no order writes promises alone**

G2 I2 · recommendation · verifier: partly · decides: coordinator

A24 ('a new ruling carries a vector that can fail') has been open since the plan audit. Its done-when is one sentence in the contributor guide, and its roadmap row names no dependency on the owner.

Proposed wording, as rule 7 of CONTRIBUTING.md 'Writing promises': "A new promise enters spec/ in the change that adds a check citing it: a case in the manifest; for a promise that binds a provider, a case of its contract suite (experiment 022); for a convention-layer safety promise, a named property of a model in experiment 012. Until then its ruling stays a spec pending line in IN_FORCE.md. No order writes promises alone. A change that adds no promise is exempt." The same rule covers the coordinator's own additions under ruling 71 point 4 and any fix from this review that adds a promise ID.

Against history (the table is in I4):
- Of the 64 promises added since vectors exist, one (CTR-014, order 075) arrived with a case citing it.
- The rule would have changed five commits: the four text-only orders and 036b.
- 17 promises would have entered 3 to 37 hours later than they did, with build orders 039, 040, 041a, 042, 044 and 075.
- 46 would still be spec pending lines today: SCT 15, PND 13, ENR 12, ACC-056, IFC-049, IFC-050, PRV-029, PKG-032, PRV-030. spec/ would hold 473 promises.
- Those 46 are 9% of the catalogue and the first-listed promise of 59 of the first pass's 263 findings, and of 10 of the 44 rows marked unsafe.

Cost:
- A build order carries its promises, so its diff grows, and large orders already converge slowly (049a1 took five rounds). The alternative is a pending list in the runner, so that text and failing cases merge together and the build turns them green: one small change to the conformance runner.
- The next spec order in the handover's queue (item 7) is text only and would have to name its cases.
- Batch 2 keeps its shape: ruling 71 point 2 already has its text and core written together on a branch.

What it buys: the count of promises that no check cites (227) stops rising, and the rate of new text is tied to the build. What it does not buy: it delays text and removes no scope. I also found no sign that a cited promise draws fewer findings (3.2 mentions per cited promise in the first pass's index, 2.5 per uncited), so I claim it as a limit on rate and a test that a promise can be checked, not as a cure for defects.

Existing paradigm: TC39 admits a proposal to the standard only with test262 tests and two engines passing them; a Kubernetes enhancement merges with a test plan, and conformance covers only what has tests. In the tree, rule 5 of the form already says a promise with no vector 'does not count as covered', and ruling 46 point 4 already requires a build finding's fix 'with a vector'. This extends that to new text.

No owner ruling is needed under the scope guard: the rule drops and narrows nothing, and IN_FORCE.md already holds ruled decisions until they are written. The coordinator adopts it and logs it for the owner to strike.

**Evidence.** docs/AUDIT.md:484 (A24: 'Adopt a review rule: a new ruling carries a vector that can fail. | D8 | The contributor guide states it. | open'); docs/archive/plan-audit-history.md:95 (D8: 'The review process generated its own queue'); ROADMAP.md:25 (A24 among step 2's repairs; 'Needs: The owner, for A20 and A21'); CONTRIBUTING.md:120-132 (six rules, none on vectors for new text); spec/README.md:301; IN_FORCE.md:11; docs/proposals/ruling-71/README.md:30-33; docs/implementation/HANDOVER.md:215 (item 7, 'Not written'), :221 (batch 2 'Not begun'); AGENTS.md 'Where things go' (ruling 46 point 4). Commands as in I4. First-cited dates of the 17, from the manifest at each commit: ACC-054 at order 039 (2026-11-11 04:30); PKG-031 at 044 (07:04); seven at 040 (07:44); three at 041a (10:12); CTR-013 at 075 (18:59); four at 042 (2026-11-12 13:57). The 46: 3,370 words of `then`, 6,540 of `observable` (26 convention, 16 language, 4 interface). Findings: the first pass's language/INDEX.txt, first-listed promise of each row against the 46. The TC39 and Kubernetes comparisons are from my own knowledge.

**Waits.** The next spec order (handover item 7); batch 2's plan; every fix from this review that adds a promise ID; the checker in I9.

**Verifier.** partly. A24 is at docs/AUDIT.md:484 and D8 at docs/archive/plan-audit-history.md:95. ROADMAP.md:25 says 'Needs: The owner, for A20 and A21'; A24 is not named there. CONTRIBUTING.md:120-133 has six rules, none on vectors. spec/README.md:301 is rule 5. I diffed the manifest at each commit against `&lt;c>~1`. 036a, 038a, 037 and 038b add no cases. 075 adds one case citing CTR-014. 038c adds one case citing CTR-013 and SUB-005. 082 adds one citing REC-006. 081 adds two citing LIF-024, OPS-022, PRV-008 and TPL-009. 077 adds three. So only CTR-014 entered with a case citing it. The 17 later citations reproduce, as does the list of 46. The 46 hold 3,370 words of `then` and 6,540 of `observable`, split 26 convention, 16 language and 4 interface. In the language INDEX, one of the 46 is the first-listed promise of 59 of 263 findings and of 10 of the 44 rows marked UNSAFE. Mentions average 3.18 per cited promise and 2.54 per uncited one.

**Verifier's counterargument.** (1) The rewording drops A24's operative words, 'that can fail'. A tag in the manifest satisfies 'a check that cites it', and the tags are self-declared (X01/F5 found coverage recorded in five places that disagree). Make the analyst's fallback the rule: the text merges with a case listed as an expected failure, and the build turns it green. That is test262's practice: tests land before engines ship the feature. (2) The strict variant holds ruled text out of spec/ while build orders implement it. That runs against IN_FORCE.md:11 ('Builds and vectors follow spec/ until the change is written'). If cases lag, it could also push batch 1's rest past the rerun, though IN_FORCE.md:171 has batch 1 written before the rerun (61 B1, 71 point 2). That is an effect on the order of the work, which is the owner's. The expected-failure variant avoids both problems. (3) The analyst leaves out a supporting point: G2 (docs/implementation/INDEPENDENT_BUILDS.md:112) already requires a vector or an implementation-defined entry for every promise before the readiness gate. The 227 uncited promises are a debt that must be paid anyway; A24 at entry makes it pay-as-you-go. The coordinator may adopt the rule and log it for the owner to strike.

**Verifier's corrections.** 'The rule would have changed five commits: the four text-only orders and 036b' should read 'seven commits: 036a, 038a, 037, 038b, 038c, 082 and 081'. 036b adds no promise, so the proposed wording exempts it. This also contradicts the analyst's own I4 ('held 63 of 64'). '46 would still be spec pending lines' should read 45 under the wording's own route through a model property: PRV-029 holds in `gates::R` (experiments/012-model-checking/G5.md:162). spec/ would hold 474 promises, not 473.


## FR-472

**Should v0's list of features be closed at what is ruled today?**

G2 I3 · question-for-owner · verifier: partly · decides: owner

In plain words: today a new feature enters v0 whenever a ruling accepts it, and nothing leaves before step 8. Between 2026-11-10 and 2026-11-12 that added 64 promises, 14 core operations and 10 core templates. Batch 2 (about 17 new promises and 23 changed, by the Fleet brief) and the metrics batch (about 18 new and 20 amended, by its proposal) are ruled and unwritten. The scope guard makes dropping anything yours to rule. Nothing makes adding to v0 yours to rule in the same explicit way.

(A) No change. Rulings add to v0 as they come; removal waits for step 8.

(B) Close the list at what is ruled today. Everything ruled stays as ruled: batch 1 as written, batch 2 as ruling 61 gives it, enrollment's bindings, the Agents' homes and logins, metrics where ruling 72 put it, and every fix. From now on, a ruling that adds a core operation, a core template, a promise file, a kind of pending action or a conformance level is recorded as 'after v0' unless your words in that ruling say 'in v0'. Each such question reaches you with its size (promises, words, core operations) and the failing check or run that prompts it.

(C) B, and bring the prune forward: before batch 2's text is written, the coordinator brings groups of promises, not single ones, for you to move to after v0 in one ruling. This sets aside the scope guard's condition that S3 to S9 run first.

Recommendation: B.
- It changes nothing you have ruled and costs you one sentence.
- It gives additions the guard that removals already have. This is the Kubernetes enhancements freeze: after it, a feature enters the release only by an explicit exception.
- The measured contrast supports it: changes that came from a failing check added 2 promises, and text written from design added 60.
- Not C now: the scope guard's reason still holds (no scenario unlike S1 has run), A18's own list is the wrong list (I5, I6), and the first pass's verifier found that the proposed relocations have no binding destination today.

What it changes: one line beside the scope guard in IN_FORCE.md (topic 1) and in the tracker's scope guard. Nothing in spec/ or in the roadmap's order.

**Evidence.** docs/AUDIT.md:316-319 ('Moving anything to later scope, or dropping it, takes an explicit owner ruling (A18)'); IN_FORCE.md:30, :98, :148, :171; docs/proposals/projects/brief.md:1125-1126 ('SPEC BATCH 2: authority, about 40 to 46 substantive promises on paper', 'New, about 17'; 'Changed, about 23' follows); docs/proposals/metrics/general.md:116 ('about 18 new and 20 amended promises'); docs/proposals/ruling-72/README.md:22-23 ('Metrics are not part of v0's gate. They are built right after batch 2'). Per-commit script: `^kind:` documents in spec/core/operations.yaml 23 at 963e40b5 and 37 at 2eb8f258; spec/core/templates.yaml 7 and 17. The first pass's X01/F8 verifier: option (b) 'has no binding destination today'. The Kubernetes comparison is from my own knowledge.

**Waits.** How the coordinator words batch 2's plan and every later question that adds behavior; where this review's proposed additions are filed (a `binds` field, a fourth conformance class, a level for tools: L13/F4 and L13/F7); A66 and A67. It can be answered in the same ruling as the gate question (the first pass's group 3) and the question on the convention layer's claim (X01/F8, L13/F6); it re-asks neither.

**Verifier.** partly. Between 963e40b5 and 2eb8f258, core operations went from 23 to 37 and core templates from 7 to 17 (`^kind:` counts). docs/proposals/projects/brief.md:1125-1126: 'about 40 to 46 substantive promises', 'New, about 17'. docs/proposals/metrics/general.md:116: 'about 18 new and 20 amended'. docs/proposals/ruling-72/README.md:22-23: metrics are outside v0's gate. docs/AUDIT.md:318-319 and IN_FORCE.md:30 hold the scope guard. The X01/F8 verifier said option (b) 'has no binding destination today'.

**Verifier's counterargument.** (1) 'Nothing makes adding to v0 yours to rule' is inaccurate. IN_FORCE.md topic 15 ('Who decides') already reserves to the owner every finding that adds a feature, except ruling 71 point 4's small ones. What option B changes is which version a ruled addition lands in by default. (2) The evidence does not bear on B. The 60 promises 'written from design' implement rulings made before batch 1 (49 to 68), and B keeps all of them, plus batch 2 (about 17 new) and the metrics batch (about 18 new). Had B stood on 2026-11-10, it would have stopped at most CTR-014 and PND-013 (rulings 70, 74, 75 and 73, 75). Both served dogfooding, which ruling 70 makes the first priority. (3) B does not close the second door that I1 names: features the coordinator settles under ruling 71 point 4 are not 'a ruling'. (4) The owner nearly always accepts the recommendation (I1's own count), so the brake that operates is the coordinator's recommendation. Without any ruling, the coordinator can recommend 'after v0 unless a run or dogfooding needs it' and attach I9's size line to every question. (5) B overlaps group 3 of the approach critic (v0's gate) and P07 I6. P07 I6's recommended option B makes appointment, peer engines and metrics stated limits of v0, which is narrower than I3's 'everything ruled stays as ruled'. Asked as two questions, they can get conflicting answers, and the owner wants rulings few (ruling 67). The question is the owner's (scope), but it should not stand alone.

**Verifier's corrections.** Fold this into group 3's single question as one option. For example: 'v0 is (a) everything ruled, as now; (b) what is ruled today, with later additions after v0 unless your words say in v0; (c) what the three central claims need (P07 I6 B).' Give the recommendation and the size of each option. Restate the premise as 'additions are yours to rule already; this sets their default version'. Whichever option is chosen, extend it to small features settled under ruling 71 point 4.


## FR-473

**The three brakes tested against what was added to spec/ from 2026-11-06 to 2eb8f258**

G2 I4 · fact · verifier: partly · decides: none

Eleven commits added promises: 152 added, 67 removed. Vectors exist since 2026-11-09; after that, 64 promises were added in 8 commits.

| Commit (order) | Promises added | New cases in the same change | New IDs cited by one of them | Cited at 2eb8f258 |
|---|---|---|---|---|
| 57222f8e (036a) | 15 | 0 | 0 | 15 |
| d08fb8bb (038a) | 16 | 0 | 0 | 1 (CTR-013) |
| 9294c49d (037) | 16 | 0 | 0 | 1 (PKG-031) |
| c8e9aa20 (038b) | 13 | 0 | 0 | 0 |
| 55cf7473 (075) | 1 | 1 | 1 (CTR-014) | 1 |
| 719a8992 (038c) | 1 (PND-013) | 1 | 0 | 0 |
| 1be89f96 (082) | 1 (PKG-032) | 1 | 0 | 0 |
| 35d2e45a (081) | 1 (PRV-030) | 2 | 0 | 0 |
| 575a1bcd (077) | 0 | 3 | n/a | n/a |

| Brake | What it would have stopped or forced | Cost | Owner ruling under the scope guard? |
|---|---|---|---|
| A24 at entry (a promise merges with a check that cites it) | Held 63 of 64 at entry; 60 are the four text-only orders. 17 would have entered 3 to 37 hours later, with their build orders. 46 would still be spec pending lines (3,370 words of `then`); spec/ would hold 473 promises, 292 of them cited. | No text-only orders: larger build diffs, or a pending list in the runner. Text held on a branch drifts. It delays text and cuts no scope. | No. It drops and narrows nothing; IN_FORCE.md:11 already holds ruled, unwritten decisions; A24's row names no owner dependency. Coordinator, logged to strike. |
| Net-zero for a batch (a promise added names one removed or merged) | Batch 1 added 64 and removed none, so it would have had to name 64. Merges with no change of behavior supply about 6 (the three clear duplicates of X01/F13 and the three version rules of X01/F4). The other 58 are removals of ruled behavior. | About 58 owner rulings, or batch 1 unwritten. It also leaks: 077 added no promise and 796 words; 115 older promises were amended. It pushes text into fewer, longer promises and into `observable` (already 3.07 words per word of `then`), against rule 1 of the form and ruling 52. | Yes, for every removal (docs/AUDIT.md:318-319; IN_FORCE.md:30); CONTRIBUTING.md:128 records a removed ID with the ruling that removed it. Merges alone are the coordinator's. |
| A first prune before batch 2's text | Nothing at entry. By A18's test the list was 206 of 455 before batch 1 (no scenario plan existed) and is 97 now; it holds 18 of the 64 additions, 15 of them the short form, which the rerun exercises. | 97 candidates, each 'for an owner ruling'; a yield of about 6. | Yes, three ways: the S3-to-S9 condition (docs/AUDIT.md:320-321), the order of the work (ROADMAP.md:33) and each candidate (docs/AUDIT.md:482). |

The one removal in the period (67 promises on 2026-11-07) came from two rulings that removed whole mechanisms, not from a rule on promises.

**Evidence.** Commands: per-commit promise IDs by `git show &lt;commit>:&lt;file>` and PyYAML over the 34 commits of `git log --reverse -- spec/`; cases by `git show &lt;commit>:conformance/manifest.yaml` against `&lt;commit>~1`; cited at head by the union of `promises` over the 494 cases of conformance/manifest.yaml. Uncited at 963e40b5: 206 of 455 (language 89, convention 95, interface 22), from the manifest at that commit (367 cases). Older promises amended since 963e40b5: `then` changed in 39, `then` or `observable` in 115, net +574 and +3,647 words. docs/AUDIT.md:318-321, :482; IN_FORCE.md:11, :30; ROADMAP.md:25, :33; CONTRIBUTING.md:128-130; spec/README.md:297. The first pass's X01/F13 verifier ('The clear duplicates are LIF-022 with LIF-034, PRV-018 with IFC-022, and ENR-011') and X01/F4 verifier ('Three survive: IFC-005, IFC-029 and OBJ-024').

**Verifier.** partly. The per-commit table reproduces. Added promises: 15, 16, 16, 13, 1, 1, 1, 1, then 0 for 077. New cases in the same change: 0, 0, 0, 0, 1, 1, 1, 2, 3; order 042 added 48. Only 075's case cites a new ID. At head, 18 of the 64 are cited. At 963e40b5, 206 of 455 were uncited (89 language, 95 convention, 22 interface) against 367 cases. 115 older promises were amended. docs/AUDIT.md:318-321 and :482, CONTRIBUTING.md:128, ROADMAP.md:33 and IN_FORCE.md:11 and :30 read as cited.

**Verifier's counterargument.** The yield of 'about 6' is mislabelled. IFC-005, IFC-029 and OBJ-024, which survived the X01/F4 verifier, are rules about later versions that cannot be tested while one version exists. Deleting or moving them changes the language, so it is the owner's; it is not a merge with no change of behavior. The three clear duplicates (LIF-022 with LIF-034, PRV-018 with IFC-022, ENR-011) are not among the 97. LIF-022, PRV-018 and ENR-011 are named in scenario plans; LIF-034 and IFC-022 are cited by cases. So A18's list yields no change free of behavior, which strengthens the analyst's verdict on it. On net-zero: c4d7d332 (rulings 40 to 43) added 85 and removed 67. When a mechanism is replaced, a ruling-led rewrite comes close to net zero, so the rule fails only where features are added outright.

**Verifier's corrections.** Replace 'Merges with no change of behavior supply about 6' with 'about 3 (the X01/F13 duplicates, none on A18's list); removing the three rules on later versions is the owner's'. 'The other 58' becomes 61. In the A24 row, read 45 still pending and 474 promises, because of PRV-029's model property (see I2).


## FR-474

**The candidate list A18 would present: 97 promises that no vector and no accepted scenario cites (67 language, 18 convention, 12 interface)**

G2 I5 · fact · verifier: supported · decides: none

Four readings, each by layer:

| Reading | Promises | Language | Convention | Interface |
|---|---|---|---|---|
| No case cites it (494 cases; the case files and the `vectors` fields add none) | 227 | 97 | 104 | 26 |
| ... and it is in no 'What it covers' list of S2 to S10 | 110 | 68 | 29 | 13 |
| ... and it is named nowhere in the README or plan of S2 to S10: the list A18 would present | 97 | 67 | 18 | 12 |
| ... and neither engine/ nor packages/ names it | 60 | 34 | 14 | 12 |

The other 130 uncited promises (30 language, 86 convention, 14 interface) rest on plans alone: no scenario of S2 to S10 has run. Of the 117 that sit in a 'What it covers' list, 57 sit in one scenario's list only (S3 16, S5 11, S6 7, S10 7, S7 5, S2 4, S9 4, S4 2, S8 1).

The 97, by layer and file:

- Language, 67:
  - sections.yaml, 15 of 15: SCT-001 to SCT-009, SCT-011 to SCT-016
  - naming.yaml, 9: NAM-001 to NAM-006, NAM-012, NAM-015, NAM-025
  - objects.yaml, 9: OBJ-004, OBJ-011, OBJ-012, OBJ-014, OBJ-015, OBJ-017, OBJ-018, OBJ-020, OBJ-024
  - schemas.yaml, 9: SCH-002, SCH-003, SCH-005, SCH-006, SCH-014, SCH-017, SCH-018, SCH-020, SCH-021
  - manifest.yaml, 4: MAN-002, MAN-003, MAN-005, MAN-012
  - conduits.yaml, 3: CON-020, CON-021, CON-027
  - contracts.yaml, 3: CTR-007, CTR-008, CTR-009
  - instantiation.yaml, 3: INS-001, INS-017, INS-021
  - secrets.yaml, 3: SEC-003, SEC-021, SEC-022
  - implementation.yaml, 2: IFC-043, IFC-044
  - placement.yaml, 2: PLC-006, PLC-028
  - pools.yaml, 2: POOL-011, POOL-012
  - implicit.yaml IMP-003; submissions.yaml SUB-007; templates.yaml TPL-001
- Convention, 18:
  - access.yaml, 3: ACC-022, ACC-023, ACC-028
  - observation.yaml, 3: OBS-001, OBS-004, OBS-006
  - pools.yaml, 3: POOL-003, POOL-007, POOL-017
  - implicit.yaml, 2: IMP-001, IMP-008
  - naming.yaml, 2: NAM-016, NAM-017
  - lifecycle.yaml LIF-023; packages.yaml PKG-032; pending.yaml PND-013; providers.yaml PRV-030; secrets.yaml SEC-004
- Interface, 12:
  - implementation.yaml, 11: IFC-002, IFC-003, IFC-004, IFC-005, IFC-015, IFC-029, IFC-030, IFC-031, IFC-037, IFC-039, IFC-040
  - packages.yaml PKG-016

By file, all layers: sections 15 of 15; implementation 13 of 46; naming 11 of 28; objects 9 of 24; schemas 9 of 21; pools 5 of 15; manifest 4 of 17; secrets 4 of 24; access 3 of 35; implicit 3 of 5; observation 3 of 5; conduits 3 of 22; contracts 3 of 12; instantiation 3 of 23; packages 2 of 24; placement 2 of 21; lifecycle, pending, providers, submissions and templates 1 each; enrollment, first-start, fleets, operations, records and revisions none.

The 13 more under the second reading: ENR-001 to ENR-012 and PND-008, which S3's README or the plans name outside any list.

Of the 97: 80 are `must`, 15 `must not`, 1 `should`, 1 `may`; 48 are in S1's frozen inventory and 49 are not; 18 were added since 2026-11-10.

**Evidence.** Command: /usr/bin/python3 with PyYAML over the 27 files under spec/promises, spec/conventions and spec/interface (519 records; `vectors` non-empty only in CTR-013 and SCH-009, both also in the manifest); the union of `promises` over the 494 cases of conformance/manifest.yaml (292 IDs, none unknown); the keys `promises` and `required_promises` in the 494 case files (the same 292); promise IDs in README.md and plan.md of experiments/013 to 021, with 'X-001 to X-005', 'X-001–005' and 'X-001/002' expanded, and separately in each README's section 'What it covers' (266 IDs in the nine lists; per scenario S2 74, S3 100, S4 62, S5 56, S6 61, S7 38, S8 32, S9 50, S10 60). experiments/013-s2-shared-login/README.md:74 and experiments/014-s3-two-machines/README.md:74 ('Promise IDs exercised, by file, as written today'); each of the nine READMEs says 'Accepted under ruling 75 D on 2026-11-11' and 'No result is recorded here'. conformance/s1-promises.yaml:1-2 ('Inventory only; inclusion does not prove coverage of a whole promise'). `git grep -hoE '[A-Z]{2,5}-[0-9]{3}' -- engine/crates` names 344 promise IDs; the same over packages/.

**Waits.** A18; I6 judges it.

**Verifier.** supported. My recount is identical, using the manifest union, a regex over every case file, and the IDs in README.md and plan.md of experiments 013 to 021 with ranges expanded. The four readings: 227 uncited (97 language, 104 convention, 26 interface). Of those, 110 are in no 'What it covers' list (68/29/13), and 97 are named nowhere in those READMEs and plans (67/18/12). 60 are also absent from engine/ and packages/ (34/14/12). The nine lists name 266 IDs: S2 74, S3 100, S4 62, S5 56, S6 61, S7 38, S8 32, S9 50, S10 60. 117 uncited promises are in a list; 57 are in one list only (S3 16, S5 11, S6 7, S10 7, S7 5, S2 4, S9 4, S4 2, S8 1). Second reading less third: ENR-001 to ENR-012 and PND-008. Levels: 80 must, 15 must not, 1 should, 1 may. Every per-file count and its file total match. 48 of the 97 are in conformance/s1-promises.yaml, and the manifest's s1_coverage gives each a reason and no vector. Only CTR-013 and SCH-009 have non-empty `vectors`, and both are in the manifest.

**Verifier's counterargument.** The counts hold; the reading has two caveats. First, S2 to S10's unrun plan lists count as citations while S1's inventory does not. That is defensible, because S1 ran and the manifest records a reason for each of the 48, but the item should say so; counting S1 the same way would cut the list to 49. Second, A18's literal test is 'exercised'. This is the list before S2 to S10 run, not the one A18 would present at step 8. 'As written today' appears in only 2 of the nine READMEs (013 and 014).


## FR-475

**A18 as written cannot be the brake: its test lists validity rules and the purpose's own negative rules, misses the growth, and its done-when points at a dead list**

G2 I6 · assessment · verifier: partly · decides: coordinator

What is on the list of 97:
- The 15 short-form promises, which the S1 rerun's own path exercises (the translator, order 043a) and which no scenario of S2 to S10 cites.
- 48 rules from S1's frozen inventory whose cases were never ported.
- The purpose's negative rules, which a plan seldom cites and a vector hardly shows: OBS-004 (nothing acts on a difference without a request), IMP-001, IMP-003 and IMP-008 (model work only through a marked operation), LIF-023, ACC-023, ACC-028, IFC-030, IFC-031, IFC-040, SEC-003 and SEC-004.

What is not on it: the growth. Of the 64 promises added since 2026-11-10, the list holds 18 (the 15 short-form ones, PKG-032, PND-013 and PRV-030). The rest are cited by a case or by a scenario plan written the same day as the text: nine pre-registrations were accepted on 2026-11-11 with lists of promise IDs 'as written today'. A feature ruled today gets a plan's citation today, so 'no scenario cites it' cannot catch it. The scope guard already says so: never exercised 'is evidence about sequencing, not about scope'.

Its mechanics fail too:
- A18's done-when points at 'deferrable scope', a list marked 'Historical ... Not maintained', written in concepts that later rulings removed (enforced formats, graph-backed operations, snapshot-to-feed handoff).
- It asks a ruling for each candidate: 97 at the least, against the aim of few rulings.
- The scoring it needs (A14) is scheduled after it.
- Its yield today is about 6 promises that could be merged or moved with no change of behavior, which the coordinator may already do under rulings 37 and 46 point 4.

What I would change, all in the tracker and the roadmap, which bind nothing:
- A18's unit becomes a group (a file or a mechanism, as rulings 40 and 41 removed graphs and policies), put to the owner in one ruling (ruling 67).
- Its test becomes the layered claim's own (LD1, and ruling 40's connection and access), with 'not reached in any scenario that ran' as supporting evidence.
- Its done-when names the list that I10's scoring produces.
- The roadmap says A14's scoring feeds step 8 instead of following it.

The scope guard's words stay as the owner ruled them.

**Evidence.** docs/AUDIT.md:318-319, :476, :482 ('Each candidate under deferrable scope has a ruling'); docs/archive/plan-audit-history.md:114-126 ('Historical. Recorded at the audit, before rulings 13 to 37. Not maintained.'); ROADMAP.md:33-34; IN_FORCE.md:30. Counts from I5's command; the 18 are the intersection of the 97 with the 64 IDs absent at 963e40b5. Obligations read with PyYAML: OBS-004 'Anything acts to remove the difference without an explicit request' (must not); LIF-023 'The instance is retired' (must not); ACC-028 'That ownership permits any request' (must not); IFC-030 'The request carries what a consumer presented to the engine to identify itself' (must not). The first pass's X01/F13 and X01/F4 verifier's corrections for the yield.

**Waits.** A18's and A14's rows; I10 supplies the list.

**Verifier.** partly. The citations hold. docs/AUDIT.md:482 gives A18's done-when as the 'deferrable scope' list. docs/archive/plan-audit-history.md:114-126 marks that list 'Historical ... Not maintained', and it names enforced formats, graph-backed operations and snapshot-to-feed handoff. ROADMAP.md:33-34 puts step 8 before A14. 18 of the 64 additions are among the 97. The negative rules cited (OBS-004, LIF-023, ACC-028, IFC-030) read as quoted.

**Verifier's counterargument.** (1) 'Misses the growth' comes from the plan-citation proxy. A18 literally presents promises no scenario exercised; once S2 to S10 run, promises listed in a plan but not reached would appear on it. I10 depends on exactly that. The fault is when A18 runs and where A14 sits, not its test. (2) 'Which the S1 rerun's own path exercises (the translator, order 043a)' rests on a plan: 043a is 'Not written' (docs/implementation/HANDOVER.md:217). (3) The yield claim fails, as in I4. (4) 'The roadmap says A14's scoring feeds step 8 instead of following it' changes the order of the work, which the owner rules. The coordinator can add report-only scoring to the scenario runs (I10) without moving A14. Making A18's unit a group and rewriting its done-when are the coordinator's changes to the tracker, in line with ruling 67's batching; each removal stays the owner's.

**Verifier's corrections.** Replace 'Its yield today is about 6 ... the coordinator may already do' with 'none on this list; three duplicates outside it are the coordinator's to merge'. Who decides: the coordinator for A18's wording, the owner for moving A14 relative to step 8.


## FR-476

**The shortcut table, filled from the ledger: eleven rows, five with no trigger for replacement**

G2 I7 · fact · verifier: partly · decides: coordinator

The scope guard requires each shortcut to be logged 'with the general behavior it stands in for'. The table has held one empty row at every commit checked since 2026-11-06. The rows below are every ruling whose owner's words, in the ledger or in the record the ledger points to, say 'for now' or the like.

| Ruling | The owner's words | The shortcut | Stands in for | Trigger for replacement | Status |
|---|---|---|---|---|---|
| 70, point 2 | "for now we should be overly permissive in order to test conduit, we can work in more strict sandboxing and isolation later" | An Agent's own tools run as the engine's operating-system user, the client's sandbox and prompts off, unconfined; since 75 B1, on where the template names a directory | Enforcement opted into through the language: a runtime provider that confines its consumer, chosen with `runtime` (Purpose; preservation test 9; ACC-027, PRV-006) | In part: a dedicated macOS user before a coordinator Agent drives workers (75 B3, kept on 2026-11-12). The first confining provider 'is built later' (75 B2): none | In use since 2026-11-11 |
| 74, point 3 | "the flag for 2 should be temporary and we should discuss a more durable solution" | `ownTools: false` in S1's Agent templates | A durable way to state and bound an Agent's own tools | Met by 75 B1 on 2026-11-11: the folder is the grant | Replaced |
| 77, point 1 | "Let's try your recommended setup for now, we can modify later if need be" | The Windows computer joins through an SSH machine provider and an SSH runtime provider | The owner's words in ruling 50, "tailscale should be a resource in the pool" | None; Tailscale is 'a candidate machine provider for later' | In use; the machine provider unbuilt |
| 77, point 2 | the same | One run user, `conduit-run`, for every run there | A user for each Agent; PRV-025's isolation, which the provider's contract states unmet | None | In use |
| 77, point 3 | the same | `tailscale serve` ends TLS for the engine's address | The engine's own https listener and certificate (order 079, on the side branch) | None | Set up in part |
| 76, point 6 | "agree with all recommendations, let's try this out" | Work-order workers run on the Windows computer over SSH | Workers as Agents placed on an enrolled node (A78; orders 049a and 049b) | Stated: 'Until the Windows computer is enrolled as a node' | In use |
| 53, point 4 | "4. Yes for now" | Plan and apply made of today's requests; the command is the approval (55 A4) | How a change is approved away from a terminal and by people other than root (53 point 2, 'Open'; 55 A4, 'later work'); an update in the language (finding 4-01) | None in the ruling; 4-01 and 4-11 stay open until S1's bar is judged | Reference written, tool unbuilt |
| 38, point 6 | "I don't think this is necessary for v0, drop it for now" | No operation arranges work that outlasts its invocation; no scheduled start of model work | Later work, and wake | None; A10 asks whether automatic wake enters v0, and A17 is open | In spec/ as a stated omission |
| 40, point 2 | "Agreed for now, but we may push in this direction for future versions" | Identity, isolation, record storage and the engine's state are each engine's choice | Provider contracts for them, swappable by the user | 'future versions' only | In spec/ as implementation-defined items |
| 51 | "We may be on a local net for now but eventually we may allow for multiple machines to communicate without direct control" | A partner's caller chain is recorded and not used in decisions (decision 10, 'for now'); no capability tokens (decision 6) | Authorization across parties | v1 only | A stated limit |
| 33 and 34 | (the coordinator's summary) 'until there is a domain the names come from this repository' | Names under the repository's own address | Names under a domain the project controls | Stated: the organization exists; one command moves them | In place |

Triggers: three stated and concrete (74, already met; 76 point 6; 33 and 34), one stated in part (70), two that name only a later version (40, 51), five with none (77 points 1, 2 and 3; 53 point 4; 38 point 6).

Provisional in the owner's words but not a shortcut in the guard's sense: 56 ("acceptable for v0", enrollment's path; no general behavior is named), 62 ("let's revisit the rest after your tests": C6 to C12, after the S1 rerun), 65 ("unable to take those actions quite yet": the owner's account steps), and 2 (replaced by 16).

Provisional by the decision's text, accepted whole: 75 C1, C3 and C4 (each 'until Agents are confined' or 'revisited in v1', so each hangs on the first row); 44 point 2 (the limits of delegation, which S1 and S2 are to measure); and the powers kept to root in v0.

Writing the rows into the tracker is the coordinator's: the scope guard already requires it. Whether rows 70 and 77 are replaced before v0 is the first pass's group 5 question, which I do not ask again.

**Evidence.** docs/AUDIT.md:322-327 (the rule and the empty table); `git grep -n -A2 'Shortcut taken for the first slice' &lt;commit> -- docs` at e271d1c7, c4d7d332, 2ca84f36, 963e40b5, d08fb8bb and 2eb8f258: one row of dashes each time. Ledger: docs/proposals/ruling-log.md:41-42, :46, :48, :59, :61, :64, :70, :73, :78, :82, :84, :85, and :132-133 (rulings 33 and 34 as given). Records: docs/proposals/ruling-70/README.md:22-25, :33-41; ruling-74/README.md:21-22, :33-35; ruling-77/README.md:15-18, :30-31, :35, :40; ruling-76/README.md:16, :34; ruling-53/README.md:10-11, :20; ruling-38/README.md:21; ruling-40/README.md:19; ruling-51/README.md:9, :32, :35; ruling-56/README.md:13; ruling-62/README.md:10; ruling-75/README.md:39-48, :52-62, :125-128; ruling-55/README.md:19. IN_FORCE.md:25, :55, :225, :228, :243, :312; docs/AUDIT.md:139-141, :265, :306; spec/README.md:90. A scan of all 77 ledger rows and every ruling record for 'for now', 'temporary', 'let's try', 'revisit', 'acceptable for v0', 'quite yet', 'until' and 'later'.

**Waits.** The scope guard's table in docs/AUDIT.md; the first pass's group 5 question uses rows 70 and 77.

**Verifier.** partly. docs/AUDIT.md:322-327 has an empty table. `git grep -n -A2 'Shortcut taken for the first slice'` finds one row of dashes at each commit: e271d1c7 (docs/AUDIT.md:707), c4d7d332 and 963e40b5 (docs/archive/plan-audit-history.md:149), and 2eb8f258 (docs/AUDIT.md:325). The owner's words check out in each record: ruling-70/README.md:22-25 and point 2; ruling-74/README.md:21-22 and point 3; ruling-77/README.md:15-18 and points 1 to 3; ruling-76/README.md:16 and point 6; ruling-53/README.md:10-11 ('4. Yes for now'); ruling-38/README.md:21; ruling-40/README.md:19; ruling-51/README.md:9 and decision 10. Ruling 75's B2, B3, C1, C3 and C4, and its later answer on B3's timing, also check out.

**Verifier's counterargument.** (1) One shortcut in the owner's words is missing. Ruling 50: 'I can do a manual set up for testing but adding machines should be much simpler with conduit' (docs/proposals/ruling-50/README.md:9-12). The record adds: 'The manual setup of the second machine is for testing only: A74 ... and S3 until enrollment exists' (:35-36). It stands in for enrollment through Conduit (ENR-001 to ENR-012), its trigger is stated, and it is the parent of row 76.6. (2) Rows 38 point 6, 40 point 2 and 51 are limits of v0's scope, not 'simplifications made to reach the scenario sooner'. They are already recorded at docs/AUDIT.md:339-341 ('Outside v0'), in the implementation-defined items and in spec/README.md's limits. Listing them again duplicates them. (3) The row for 77 point 2 says 'None', but the record names the replacement condition: PRV-025's isolation is unmet 'until each Agent has a user of its own'. It names no event that would trigger it. Filling the table is the coordinator's, as stated.

**Verifier's corrections.** Add the ruling-50 row, with its trigger: enrollment through Conduit (orders 049a1 and 049b). That gives nine shortcut rows proper: 70.2, 74.3 (replaced), 77.1, 77.2, 77.3, 76.6, 50, 53.4, and 33 with 34. Rows 38.6, 40.2 and 51 become one row pointing to 'Outside v0'. For 77.2, read 'condition stated, no trigger'.


## FR-477

**Stable numbers at this commit and at the commits where rounds ended**

G2 I8 · fact · verifier: partly · decides: none

| Commit | Date | What ended | Promises (language / convention / interface) | `then` words (language / convention / interface) | `observable` words | Core contracts (operations / templates) | Implementation-defined | Cases; promises cited; uncited |
|---|---|---|---|---|---|---|---|---|
| e271d1c7 | 11-06 | Rounds 1 to 4; first promises | 434 (220 / 179 / 35) | 13,517 (5,829 / 5,854 / 1,834) | 38,184 | 53 (20 / 7) | 15 | none yet |
| c4d7d332 | 11-07 | Round 5 | 452 (202 / 207 / 43) | 16,967 (5,532 / 8,669 / 2,766) | 52,795 | 61 (23 / 7) | 15 | none yet |
| 2ca84f36 | 11-07 | Round 6 | 454 (202 / 208 / 44) | 17,359 (5,622 / 8,749 / 2,988) | 55,874 | 61 (23 / 7) | 15 | none yet |
| d27d5efc | 11-09 | Increment 1 | 455 (203 / 208 / 44) | 17,388 | 56,438 | 61 (23 / 7) | 15 | none yet |
| d3897474 | 11-10 | Increment 2 | 455 | 17,529 | 57,084 | 61 (23 / 7) | 15 | 362; 246; 209 |
| 788c2843 | 11-10 | Increment 3 | 455 | 17,533 | 57,185 | 61 (23 / 7) | 15 | 364; 246; 209 |
| 963e40b5 | 11-10 | Before batch 1 | 455 (203 / 208 / 44) | 17,533 (5,756 / 8,752 / 3,025) | 57,185 | 61 (23 / 7) | 15 | 367; 249; 206 |
| 719a8992 | 11-11 | Batch 1's text written | 517 (226 / 243 / 48) | 22,148 (7,159 / 11,540 / 3,449) | 67,589 | 95 (37 / 17) | 17 | 440; 284; 233 |
| 2eb8f258 | 11-12 | The review's commit | 519 (226 / 245 / 48) | 22,622 (7,159 / 12,014 / 3,449) | 69,486 | 95 (37 / 17) | 17 | 494; 292; 227 |

Derived:
- Mean `then` words per promise: 31.1, 38.2 (round 6), 38.5 (before batch 1), 43.6 now.
- Promises whose `then` exceeds 100 words: 23, 43, 44, 59.
- Convention promises in the seven convention files against topic files: 127 and 52, then 123 and 85, now 132 and 113.
- `observable` words per word of `then`: 2.82, 3.22, 3.26, 3.07.
- Promise files: 22, 23, 23, 27. Families: 6 now. Conformance levels: 3.
- Core contracts now: 37 operations, 17 templates, 13 reference schemas, 12 result schemas, 8 capabilities, 6 types, 2 requirements.
- Uncited promises by layer now: 97 language, 104 convention, 26 interface. Before batch 1: 89, 95, 22.

A33's done-when asks for 'word counts before and after', and its status says 'word counts not done'. This table supplies them.

**Evidence.** Command: for each commit, `git ls-tree -r --name-only &lt;commit> -- spec/ conformance/manifest.yaml`, then `git show &lt;commit>:&lt;file>` parsed with /usr/bin/python3 and PyYAML: promise IDs and `layer`; whitespace-split word counts of `then` and `observable`; `^kind:` lines in each spec/core/*.yaml; `^- id: DL` lines in spec/implementation-defined.yaml; the union of `promises` over the manifest's cases against the commit's own promise IDs. Commit subjects from `git log -1 --format='%ad %s'`: e271d1c7 'The audit of the plans, and the specification as numbered promises (#6)'; c4d7d332 'Rulings 39 to 43, the rewrite around provider contracts, and round 5 (#7)'; 2ca84f36 'Round 6: the Codex read, ruling 44, and its changes (#8)'; d27d5efc 'First build, increment 1'; d3897474 'increment 2 done'; 788c2843 'increment 3 done'; 963e40b5 'Work order 032'; 719a8992 'Work order 038c'. docs/AUDIT.md:441 (A33). The counts at 2eb8f258 equal the first pass's (519; 226, 245, 48; 22,622; 69,486; 17; 292 and 227).

**Waits.** I9 sets rules on them; A33's status line.

**Verifier.** partly. Every cell recomputes identically at all nine commits, and the subjects are confirmed by `git log -1`: d3897474 'Work order 019 ... increment 2 done (#54)', 788c2843 'Increment 3 closeout ...'. Derived figures: mean `then` 31.1 / 38.2 / 38.5 / 43.6; promises with `then` over 100 words 23 / 43 / 44 / 59. Convention promises in the seven files against topic files: 127/52, then 123/85, now 132/113. `Observable` to `then` ratio: 2.82 / 3.22 / 3.26 / 3.07. Promise files: 22 / 23 / 23 / 27. Core contracts now total 95: 37 + 17 + 13 + 12 + 8 + 6 + 2. There are six families (spec/definitions.md:19) and three conformance levels (spec/README.md:312-322). Uncited promises (language/convention/interface): 89/95/22 before batch 1, 97/104/26 now.

**Verifier's counterargument.** Only the closing sentence about A33 fails. A33's done-when (docs/AUDIT.md:441) is 'Every topic is converted, with word counts before and after (G2)': each topic's prose before conversion against its promises after. e271d1c7 is the first commit with spec/, so the table starts after conversion and supplies no 'before'.

**Verifier's corrections.** Drop 'This table supplies them'. A33 stays 'word counts not done', unless the coordinator rescopes its done-when.


## FR-478

**Print a size line with every change to spec/ and every question that adds behavior; hold one number, report the rest, and do not cap the ones the fixes must raise**

G2 I9 · recommendation · verifier: partly · decides: coordinator

A checker in tools/ (Python is allowed there) prints the numbers of I8 before and after a change. The same line goes into every question put to the owner that adds behavior, so that an acceptance is given with its size in view.

What to do with each number:
- Hold: promises that no check cites, 227 today. The checker fails when it rises. This is A24 as a number (I2).
- Hold if the owner takes option B of I3: the feature counts. 6 families, 27 promise files, 37 core operations (about four more with batch 2's make, keep, withhold and acting for a Fleet), 17 core templates, 3 conformance levels.
- Report, expecting a fall: `observable` words (69,486, or 3.07 per word of `then`), as the binding sentences the first pass found there move out (X01/F1, L13/F1).
- Report, with no cap: `then` words and the promise count. Moving binding sentences into `then` and tagging or splitting promises with several obligations (X01/F3) raise both, and a cap would work against those fixes and against ruling 52's explicit results.
- Report as the direction the layered claim asks for: implementation-defined items (17). A new convention promise says in its change why it is not an engine's own choice or a provider's contract (LD1; ruling 40).

Cost: one small order for the checker, and one line in each pull request and each question. It buys the owner and the coordinator a number at the moment a decision adds text, where today the count is made after the fact; it also closes A33's 'word counts not done'.

The coordinator decides it: a checker and a reporting habit add no behavior.

**Evidence.** I8's table and command. docs/AUDIT.md:441 (A33: 'Every topic is converted, with word counts before and after (G2)'; status 'word counts not done'); AGENTS.md 'Rules' ('The only exception is the Python checkers in `tools/`'); IN_FORCE.md:98 (batch 2: `make.v1`, 'the keep and withhold operations, and acting for a Fleet'); spec/README.md:297, :301; the first pass's X01/F1, X01/F3 and L13/F1 with their verifier's corrections; I1's finding that order 077 added no promise and 796 words.

**Waits.** A33's row; the form of the questions in the review's message to the owner; I3's option B uses the feature counts.

**Verifier.** partly. The basis is I8's numbers, which hold. AGENTS.md 'Rules' allows Python checkers in tools/. tools/ already has vector_delta.py, which reports case changes by stable id, and check_docs.py, which reads the coverage index. IN_FORCE.md:98 lists batch 2's core additions. The coordinator is the right party.

**Verifier's counterargument.** A hold on 'promises no check cites' rewards tagging. The manifest's promise lists are written by whoever writes the case, so a tag on an unrelated case lowers the count without testing anything. X01/F5 found coverage recorded in five places that disagree, and the brief warns that 'a passing vector shows its slice'. Hold instead on uncited promises that have neither an entry in an expected-failure list nor a recorded reason, and sample tags in each verifier round. Extend vector_delta.py or check_docs.py instead of writing a new checker. The claim that this closes A33 fails, as in I8. Holding the feature counts depends on I3 option B, which I would fold into group 3.

**Verifier's corrections.** Remove 'it also closes A33's word counts not done'.


## FR-479

**S1's rerun, S2 to S10 and A14: each dated result scores the promises its plan lists, so that A18's list comes from runs and not from plans**

G2 I10 · experiment-change · verifier: partly · decides: coordinator

Register rows: the S1 rerun (roadmap step 3, A68, and its measurement plan); S2 to S10 (the scenario table and experiments 013 to 021); A14.

The claim it tests: that what v0's scenarios need is what the catalogue holds, which is the layered claim's 'small' language and 'few' conventions and the premise of A18. Today the only evidence is each README's list of promise IDs 'as written today', a plan's statement.

The measure, fixed before any run and report-only: for every promise in the scenario's 'What it covers' list, one of A14's three scores from that run.
- Exercised: a case that the scenario's orders added cites it and passes, or a record or diagnostic of the run carries its ID as the deciding promise (ACC-056).
- Contradicted: a finding of the run against it.
- Not reached.

The result also lists the IDs the run reached that its list did not name. A script counts all of it from the manifest's difference and the run's records.

Pass condition: none. It gates nothing and amends nothing accepted under ruling 75 D or ruling 71 point 3. It belongs in the one pre-registration the first pass's critic asked for, not as a separate one.

What it changes:
- A18's list becomes the promises not reached in any scenario that listed them, plus those no scenario listed.
- A14's scoring no longer waits until after step 8.
- A cut to a scenario shows at once: S3 alone is the planned exerciser of 16 unvectored promises, and S5 of 11.

Cost: one script; no time of the owner's. The coordinator decides it as a report-only measure and logs it for the owner to strike.

**Evidence.** ROADMAP.md:27 (step 3), :33-34 (step 8, then A14), :139-148 (S1 to S10); docs/AUDIT.md:476 (A14: 'Each promise is scored exercised, contradicted or never needed'), :381-382 (the layered claim's rows); experiments/013-s2-shared-login/README.md:74 and experiments/014-s3-two-machines/README.md:74 ('Promise IDs exercised, by file, as written today'); nine READMEs with 'No result is recorded here'; I5's count that 57 of the 117 uncited promises in a 'What it covers' list sit in one scenario's list only (S3 16, S5 11); spec/conventions/access.yaml:915 (ACC-056: 'whose ruleId is the refusing promise'), as the first pass's X01/F6 quotes it; the first pass's approach critic.md, 'The rerun's report' (added measures 'must be report-only').

**Waits.** Order 053's pre-registration for the rerun; the results files of experiments 013 to 021; A18's restated done-when (I6).

**Verifier.** partly. ROADMAP.md:27 and :33-34 and docs/AUDIT.md:476 read as cited. The 57 single-list promises (S3 16, S5 11) reproduce. The approach critic, under 'The rerun's report', says added measures 'must be report-only' and must be one pre-registration in order 053's plan, which the item respects. The coordinator is the right party for a report-only measure.

**Verifier's counterargument.** The 'exercised' score is mostly a case tag written by the scenario's own orders. That is the same declared evidence the analyst discounts when it comes from plans. Its second leg, a refusal's ruleId, rests on ACC-056, which no case cites and whose choice of refusing rule is undetermined (X01/F6, L01/F4). A stronger measure already exists in the code. The core 'cites its promises' in every decision (engine/crates/core/Cargo.toml:4; `git grep -nE '"[A-Z]{2,5}-[0-9]{3}"' -- engine/crates` gives 497 lines). Recording which promise-citing decisions executed during a run gives coverage by promise ID, as cargo-llvm-cov gives coverage by line. Also, with no pass condition, the item should state what the result decides (ROADMAP.md:96, 'what it must show'). For example: a promise a scenario listed but did not reach after its runs joins A18's grouped candidates.

**Verifier's corrections.** Define 'exercised' by the run's own trace, a core decision citing the ID, with the case tag as secondary evidence. Add the sentence on what the result decides.


## What the verifier says the analyst missed

- **A24's 'can fail' is the brake, not citation** The analyst never names the property A24 exists for. D8 ('the review process generated its own queue') is answered by requiring that each ruling come with a vector that can fail. A rule satisfied by a manifest tag does not do that. The operative form is an expected-failure case that merges with the text and that the build turns green. The coordinator can adopt it with a small change to the runner.
- **G2 already sets a late coverage gate** G2 (docs/implementation/INDEPENDENT_BUILDS.md:112) requires every promise to have a vector or an implementation-defined entry before the readiness gate (step 7). The 227 uncited promises are therefore a debt the project must pay before the independent builds. This is the strongest argument for A24 at entry (pay as you go), and none of the costings uses it.
- **I3 duplicates group 3 and P07 I6** P07 I6's recommended option B (approach/P07-engine-build.md) puts appointment, peer engines and metrics among v0's stated limits. I3 option B keeps them in v0. The two v0-scope questions must be one, inside the critic's group 3 (v0's gate), or the owner may give answers that conflict.
- **Ruling 50's owner-worded shortcut** 'I can do a manual set up for testing but adding machines should be much simpler with conduit', with its trigger 'S3 until enrollment exists' (docs/proposals/ruling-50/README.md:9-12, :35-36). It is missing from the shortcut table, and it is the parent of row 76.6.
- **Evidence-led orders also left their new promises uncited** 038c, 082 and 081 each added one promise (PND-013, PKG-032, PRV-030) with cases that cite other promises. Ruling 46 point 4's practice of a fix 'with a vector' lapsed even where a model check led the change. The analyst's contrast between 2 and 60 needs this qualification, and A24 should apply to these orders too.
- **Owner rulings already brake by group** P08 I8's verifier listed ruling 66 point 6 (no Scope object, no grouping shorthand) and ruling 76 (no prune flag). Rulings 40 and 41 removed 67 promises in one commit before S3 to S9 had run. So a grouped removal by the owner, on design grounds, is an existing route that sits outside A18's sequencing. The analyst's I1 and I3 option C treat removal as possible only after S3 to S9.
- **A run-time measure of 'exercised' exists in the core** engine/crates/core/Cargo.toml:4 says every decision cites its promises, and 497 lines of engine/crates quote promise IDs. Recording which cited decisions executed during S1's rerun and S2 to S10 would score A14's 'exercised' from the run itself, not from tags or plans.
- **Net-zero against the ruling-led rewrite** c4d7d332 (rulings 40 to 43) added 85 promises and removed 67. Net-zero was tested only against batch 1. Tested against this commit too, it shows that a replacement of a mechanism comes close to net zero and that the rule binds only where features are added outright. That is a fairer cost.

## The verifier's notes

Pinned at 2eb8f258 (2026-11-12). I recounted every number in the ten items using /usr/bin/python3 with PyYAML over `git show` at all 34 commits that touch spec/, and over the manifest and the case files. Every count reproduces exactly: 519 promises (226/245/48); 227 uncited (97/104/26); 97, 110 and 60 under the three narrower readings; 152 added and 67 removed; the 64 recent additions; the 17 later citations; the 46 held; 59 and 10 findings; and the full table of I8.

The arithmetic is not where the item fails. It fails on how the numbers are read:
- The rewording of A24 drops 'that can fail', which is its point.
- The claim of 'five commits' contradicts the item's own 63 of 64. It is seven commits.
- The yield of 'about 6' counts rules about later versions as merges with no change of behavior.
- The A33 claim misreads 'before and after'.
- The question to the owner rests on evidence that does not bear on it: the 60 promises were already ruled, and B keeps them. It also overlaps group 3 and P07 I6.

My overall judgment of the area: the analyst's diagnosis is right in substance. There is no standing rule on growth. The only prune path sits at step 8, with its done-when pointing at a dead list. The shortcut table is empty despite rulings 70 and 77. Text has run ahead of anything that can fail on it.

The best brake is A24 in its own words, as an expected-failure case merged with the text. The coordinator can adopt it and log it to strike, and G2 already demands that coverage before the gate.

The owner question should be one option inside group 3's question on v0, not a separate ruling. The prune test should come from runs, scored by the core's own promise citations.

Coverage: read in full the brief, AGENTS.md 'What holds' and 'Where things go', docs/AUDIT.md:295-345 and :436-490, ROADMAP.md:20-40 and :89-110, IN_FORCE.md:1-35, :98, :148, :171 and :302-330, the ledger rows 1-12 and 47-85, docs/implementation/HANDOVER.md:205-225, CONTRIBUTING.md:105-135 and spec/README.md:286-305. In the ruling records I read the parts cited (38, 40, 50, 51, 53, 66, 70, 71, 72's later answers, 74, 75 B, C and later answers, 76, 77). From the first pass I read P08 I8, E05 I5 and I6, every verdict of X01, L13 F4 to F7, and the critic's groups. I did not read the bodies of the scenario plans, beyond the ID scans and S2's 'What it covers'. Nor did I read the other first-pass files, or G5 beyond the rows checked.
