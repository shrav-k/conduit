# G5-round6-evidence: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** All 31 rows that ruling 44 put into the program are still placed: S1 holds six (with vectors or dated results), S2 eight, S3 two, S4 seven, S5 five, S7 one, S10 five, and D22 rides on every run; S6, S8 and S9 hold nothing but D22. Taken as written, the first pass would leave 9 rows untouched, keep 10 with a tightening or fewer repetitions, and move or drop 12 in whole or part; every such drop is the owner's under ruling 44 and ruling 75 D1, and none of the 12 has run. The first pass's proposals contradict each other on E17, on E09's replay, on E06 and on S10's single swaps, and together they would leave D20's local-against-isolated comparison and D01's adapter row with no run at all, which no analyst or verifier named. On ruling 45, 21 of the 31 rows have a counting rule fixed in accepted text; of the 23 measures of S2 to S7, 6 can be counted by a third party as written, 15 in part, and D20 (in S2 and in S3) not at all. What is missing is lists, rubrics and expected values, which the coordinator may add as dated, labelled amendments; the only gaps that need the owner already wait for the owner (S3-OF, S2's plain server). A large share of the owner-level changes ask again what the owner answered on 2026-11-11 or 2026-11-12 with nothing new since: one author where D3 has two, S4's replay moved off the second account, fewer real-client runs for S3, S8 and S10, S10's twelve replays cut again, the owed stop reopened. My judgment is that no round-6 row should leave its scenario: the savings the first pass wants come from run counts, the owner's hand in S3 and unfixed counting rules, and the last two can be removed without cutting a case. I return no new question for the owner from this area; I return one recommendation and two experiment changes for the coordinator, the table as eight fact items, and the three-class split for S2 to S10.

**What the analyst read.** Commit 2eb8f258 (2026-11-12), read only; nothing built, run or written.

Read in full: docs/guides/full-review.md; AGENTS.md; ROADMAP.md; the records of rulings 44, 45, 72 and 75; the first pass's E01-s2-s3-s4, E02-s5-s6-s7 and E03-s8-s9-s10 files; its items E05 I9, P04 I13, P05 I2, I3, I10 and I11; its approach critic; the READMEs of experiments 007, 013, 014 and 015; the READMEs of 016 (from line 28), 017 (from line 34), 018 (from line 20) and 021 (from line 22).

Read in part: the READMEs of 019 and 020 (pass conditions, coverage, measures, answers, limits); the plans of 013 (lines 1-30, 56-72, 96-150), 014 (255-300), 015 (196-216) and 021 (118-132, 160-180, 216-236), and all nine plans by grep for the case and measure IDs and for decisions logged to strike; docs/implementation/FIRST_BUILD.md lines 98-195; the increment 4 measurement plan lines 1-110; IN_FORCE.md topic 13 and lines 22, 115 and 244-246; docs/AUDIT.md lines 120-175 and 196-206; docs/implementation/HANDOVER.md lines 205-262 and 268-272; ruling 42's later answer, ruling 77 lines 55-75, ruling 71's later answers, ruling 47 lines 36-40; round 6's evidence table (design-critique.md 655-705) and DECISIONS.md by grep; conformance/s1-safety-cases.md lines 1-75 and four passages of conformance/README.md; CON-024 and OBS-004 in spec/; packages/scenario-s2-linear-restrictions/README.md lines 60-80; experiments/008-real-client/results-2026-11-09.md lines 60-66; first-pass items E05 I7, P08 I16, E04 I10 and P03 I7, and the other first-pass files only by grep for the IDs.

Not read: the nine plans in full; the READMEs of 019 and 020 in full; round 6's D-sections; the handover in full; any source code beyond one Cargo.toml; spec/ beyond two promises; the Records of the lane's orders; experiment 008's results beyond the lines above. Where I rely on a first-pass verifier's check that I did not reopen (P03 I7's dispatch claim, the first pass's line counts and git times), the item says so. The pairing of E11 and E12 with particular vector directories is by the conformance README's sentence, not by opening the cases.


## FR-502

**No round-6 case or measure needs to leave its scenario; what the first pass wants to save sits in run counts, the owner's hand and unfixed counting rules**

G5 I1 · assessment · verifier: partly · decides: coordinator

Ruling 44 put 31 rows into the experiment program: E01 to E20, the tables D01 and D02, the measures D05, D08, D09, D10, D15, D16, D19 and D20, and D22 on every run. Set against the first pass's changes (the tables in I16 to I23, the net in I2): 9 rows are untouched, 10 are kept with a tightening or fewer repetitions, and 12 would be moved or dropped in whole or part. None of the 12 has run.

I would carry none of those 12 to the owner as a drop. Row by row:
- D22 (S5, S6, S7). The five counts are by-products of the author's log. Dropping them saves nothing, and 'concepts met', which the first pass drops from S7, is the count that S1's two runs failed to report (I23).
- E15's revocation axis and D20 in S3's cells. They cost the owner's hand only under S3-OF option A. With the cut made on the Mac (E01 I8, P05 I2) and D20's attempts scripted (I5), keeping both costs harness time, not owner time. S3-OF decides this and already waits for the owner.
- E03, E04, E10, E16 (S5). The conformance runner depends on the core and the loader only, so 'vectors alone' removes the shell's evidence. The harness can drive each case once on the engine. Whether two authors also work them by hand is ruling 75 D3's matter, answered on 2026-11-11.
- E17 (S2) and E09's replay (S4). Both sit inside runs that happen anyway, so moving them saves no run; and two first-pass items add cases to the very observations that two others remove (I2).
- D01 in S7, D05's onward case, E18's boundary half. Each lacks a definition: S7 has no task in which an adapter is written, neither built S2 implementation makes an onward call, and E18's state has no shown public preparation. That is a counting rule to write or a lane order to place, not a reason to cut the row.

What is working: the nine accepted texts are unchanged since acceptance but for a two-line link to the plan; 21 of the 31 rows have a counting rule a third party can apply; S1's four cases exist as vectors and D19 has a dated sample; E17 has dated evidence from A71.

What is not: 10 rows, and 17 of the 23 measures of S2 to S7, lack something needed to count them (I3); four pairs of first-pass proposals pull against each other on one row and two sets compound into drops nobody named (I2); and several proposals marked for the owner ask again what the owner answered on 2026-11-11 or 2026-11-12 (third class in I7 to I15).

What I would change: the coordinator writes the missing counting rules as dated, labelled amendments before any harness order is written (I4, I5, I6); the one question that amends the pre-registrations lists its drops by round-6 row and leaves the third class out; and the owner is asked only where something changed after acceptance: S3-OF (ruling 77), S2's plain server and what 'adapter' means (order 139), and whether a rewritten item in a service's package fails a swap (E03 I2 in its verifier's form).

**Evidence.** Row counts: from the tables in I16 to I23. docs/proposals/ruling-44/README.md:22-24 (D22), :66-68 (the cases and measures). ROADMAP.md:161-168 ('None has been run. Every run also records D22's author measurements'), :172-178. engine/crates/conformance/Cargo.toml:10-11 (conduit-core and conduit-loader only). `git diff --stat d30ebbd6 HEAD` on the nine READMEs of experiments/013 to 021: '9 files changed, 18 insertions(+)'. `ls` of experiments/013 to 021: README.md and plan.md, plus linear-stub and scripted-client; no results file. docs/proposals/ruling-75/README.md:87-88 (D3). experiments/014-s3-two-machines/plan.md:259-298 (S3-OF).

**Waits.** The merged question that amends S2 to S10 (the first-pass critic's group 4); the harness orders S2-H to S10-H, whose size follows which branches run.

**Verifier.** partly. Holds: ruling 44 put 31 rows into the program (ROADMAP.md:161-178; docs/proposals/ruling-44/README.md:22-24, :66-68), and none has run. `git diff --stat d30ebbd6 HEAD` on the nine READMEs gives '9 files changed, 18 insertions(+)'. The conformance runner depends only on conduit-core and conduit-loader (engine/crates/conformance/Cargo.toml:11-12). D3 is at docs/proposals/ruling-75/README.md:87-88 and S3-OF at experiments/014-s3-two-machines/plan.md:259-298. Does not hold: (1) '21 of the 31 rows'. The item's own tables (I16-I23) mark 23 rows yes and 8 not yes (E07, E08, E17, D01, D05, D10, D15, D20), and the 10 are never named. (2) 'E15's revocation axis and D20 in S3's cells ... cost the owner's hand only under S3-OF option A' misstates the first pass. E01 I3 dropped them as unable to fail: every decision is made at M1, since ACC-027's enforcement point 'in this version is the engine alone' and 'packets to M2 alone do not make M1's state unavailable' (experiments/014-s3-two-machines/README.md:131-132); and 'whether a file can be read does not vary with a partition'. E01 I3's verifier agreed it is 'exercising code'. The item answers cost only. (3) 'S7 has no task in which an adapter is written' is overstated. T3 qualifies 'the plain, adapter and aware variants by D01' (experiments/018-s7-outside-package/README.md:90-91), M2 keeps 'new adapter code' apart (:129-130), and the admin's logged reading defines S7's adapter as code wrapping upstream (experiments/013-s2-shared-login/plan.md:114-121; docs/implementation/HANDOVER.md:272). (4) One of the questions it would keep for the owner, whether a rewritten item fails a swap, is already answered by accepted D16: a conversion before the freeze 'is provider preparation, recorded in D15' (experiments/021-s10-swapped-providers/README.md:138-140).

**Verifier's counterargument.** Keeping cells that by construction cannot differ (revocation decided at M1; credential readability during a cut) adds harness branches and run time for no evidence, against the owner's aim of a concise, provable language. The defensible form of 'no row leaves': keep them only where scripting makes them nearly free, label them as code-path exercise, and otherwise put the drop to the owner with E01 I3's validity reason. Do not present that reason as a matter of owner time.

**Verifier's corrections.** '21 of the 31' becomes 23 of 31 by the item's own tables, or name the 10. Replace the owner-hours sentence on E15 and D20 with E01 I3's validity argument and an answer to it. Say 'no S7 task says to write an adapter; T3 qualifies an adapter variant, and the admin's logged reading defines it'. Remove the D16 item boundary from the owner's list, or say that only the count of items to rewrite is new.


## FR-503

**The net of the first pass on the 31 rows: 9 untouched, 10 kept, 12 moved or dropped; four contradictions and two compound drops**

G5 I2 · fact · verifier: partly · decides: none

Untouched (9): E01, E05, E08, E11, E12, E19, E20, D08, D19.
Kept with a tightening or fewer repetitions (10): E02, E06, E07, E13, E14, D02, D09, D10, D15, D16.
Moved or dropped in whole or part (12), each the owner's decision under ruling 44, and under ruling 75 D1, which accepted each scenario's allocation:

| Row | What the change does | First-pass item |
| --- | --- | --- |
| D22 | all five counts dropped from S5 and S6; 'concepts met' dropped from S7 | E02 I7 point 7; E02 I5 point 8; E02 I2 |
| D01 | dropped from S7; in S2 the adapter row rebuilt, or dropped so that two rows remain | E02 I2; E01 I2 item 3 |
| D05 | the onward-call case gets an implementation, or 'dependent calls' leaves S2's row | E01 I2 item 4 |
| D20 | S2's isolated runs go and the isolated half moves to S3; in S3 it leaves the partition cells and stays before the loss and after the return; S3's isolated profile goes to v1 | E01 I2 item 7 and its runs; E01 I4; P08 I16 |
| E15 | the revocation axis is dropped (16 of 32 cells), one check after the heal kept | E01 I4 |
| E17 | M1 leaves S2; experiment 008's harness is rerun only if the engine's wire answers differ or a client's version changed | E01 I2 item 8 |
| E09 | the real-client replay leaves S4 for experiment 008's stub, on the primary accounts | E01 I6 item 3 |
| E18 | the boundary-fixture half is dropped if the lifecycle model shows the state cannot be reached | E01 I6 item 4 |
| E03, E04, E10, E16 | out of S5's author run, carried by vectors V1 and V3 alone | E02 I7 point 2 |

Contradictions between first-pass proposals on one row:
1. E17. E01 I2 item 8 takes M1 out of S2; E02 I5 point 9 adds a case to S2's E17 (a call held in flight while its Conduit is closed).
2. E09's replay. E01 I6 drops S4-E09; E03 I6 adds 'engine restarted mid-call' to that same replay.
3. E06. E03 I5 runs it once; P08 I16 keeps it once per author.
4. S10's single-kind swaps. E03 I5 keeps them on scripted clients; P08 I16 removes them and keeps only the all-swapped replays, which leaves E14's 'each provider swapped' with no per-provider edit ledger at scenario level.

Compound drops that no item or verifier named:
1. D20's comparison of a local and an isolated deployment. E01 I2 removes S2's two isolated runs and sends the isolated half to S3; P08 I16 sends S3's isolated profile to v1; E01 I4 keeps one D20 observation on M2 and, as its verifier noted, does not say whether an isolated lab remains. Together no run of v0 compares the two deployments, which is what D20 is.
2. D01's adapter row. E02 I2 drops D01 from S7 and E01 I2 item 3 lets S2's table fall to two rows. Together no run counts 'the adapter work' that the row exists for.

Also: every cut of an author run (E01 I6 item 2, E02 I8, E03 I6, E01 I2's runs, E01 I4's two-author split) lowers the number of runs that record D22, without dropping it.

**Evidence.** First pass: E01 I2 (items 3, 4, 7, 8, 'Runs'), E01 I4 ('Dropped: the revocation axis (16 of 32 cells), D20 in every cell, four real-client runs of fourteen replayed tasks, and the two-author split'), E01 I6 (items 1 to 4), E02 I2, E02 I5 (points 8 and 9), E02 I7 (points 2 and 7), E03 I5 ('Run E06 once, not once per author'), E03 I6 ('Add engine restarted mid-call to S4's one real-client replay'), P08 I16 ('replays from twelve to four ... with E06 once per author'; 'the isolated profile on the second machine moves to v1'), and the verifier's note on E01 I4 ('say whether it is one lab or two'). ROADMAP.md:173 ('D20: what a local deployment and an isolated one of the same relationships each protect'), :173 and :177 (D01, 'with the adapter work counted (also S7, A63)'), :178 (E14 'with each provider swapped'). docs/proposals/reaudit/round-6/DECISIONS.md:105 (D01 under S2 and S7), :124 (D20 under S2 and S3).

**Waits.** I1; the three-class split in I7 to I15.

**Verifier.** partly. The four contradictions hold. E01 I2 item 8 takes M1 out of S2 while E02 I5 point 9 adds a case to S2's E17. E01 I6 drops S4-E09 while E03 I6 adds a restart to that replay. On E06, E03 I5 says once and P08 I16 says once per author (experiments/021-s10-swapped-providers/README.md:185). E03 I5 keeps the single-kind swaps on scripted clients and P08 I16 removes them. 9+10+12=31. Does not hold: (a) E14 belongs among the rows dropped in part. By the item's own contradiction 4, P08 I16 removes the per-provider swaps that E14 names ('with each provider swapped', ROADMAP.md:178), so the split is 9/9/13. (b) D08 is called untouched here, while I16 lists additions from P02 I8, P08 I13 and P03 I11. (c) D20 compound. P08 I16's verifier already said that dropping the isolated profile 'empties' S3's P6. And S10's T3 and T5 replay S1 and S2 on Docker (experiments/021-s10-swapped-providers/README.md:85-86, :89-90), the same runtime as S2's isolated profile (experiments/013-s2-shared-login/README.md:182-184). Both deployments therefore still run in v0; only the D20 measure goes, and it could ride on those replays, as E01 I2's verifier suggested. (d) D01 compound. S8 still counts adapter work around existing tools ('behind small adapters that are counted', experiments/019-s8-filtered-search/README.md:215; :116), and D22's 'adapters written' is counted on every run (ROADMAP.md:166-168). What would be lost is D01's three-row qualification on one task, not every count of adapter work.

**Verifier's corrections.** Move E14 to the moved or dropped rows (P08 I16). Mark D08 as kept with additions. D20: 'no v0 run measures D20; S10's Docker replays could carry it at no extra run'. D01: 'the three-row qualification is lost; adapter work is still counted in S8 and by D22'. The D20 compound was half named, by P08 I16's verifier.


## FR-504

**Ruling 45, measure by measure, for S2 to S7: of 23 measures, 6 can be counted by a third party as written, 15 in part, and D20 (twice) not at all**

G5 I3 · fact · verifier: partly · decides: none

All 23 were written before any run, and none has run, so the letter of ruling 45 is kept. The table says whether the accepted text fixes how each is counted. Short forms: 013R to 018R are the README.md of experiments/013-s2-shared-login, 014-s3-two-machines, 015-s4-partial-setup, 016-s5-control-contention, 017-s6-revoke-in-flight and 018-s7-outside-package; 013P and 015P are the plan.md beside them; FB is docs/implementation/FIRST_BUILD.md.

| Scenario | Measure | Defined before the run? | What is missing |
| --- | --- | --- | --- |
| S2 | M0 (013R:127-137) | in part | It carries FB's table whole. 'Inspection effort' is what it takes 'to answer T3' (FB:155), and S2's T3 is another task (013R:112). No bar or comparison reads the numbers (013R:136-137) |
| S2 | D01 (013R:143) | in part | The count is defined. What the adapter row is was settled in the plan and logged to strike (013P:110-124); the plain server is unchosen (013P:126-142) |
| S2 | D05 (013R:144) | in part | Five assertions are named. The cases are not enumerated, so 'passed, failed and unexercised' has no denominator; the rule for each case is in a package's README (packages/scenario-s2-linear-restrictions/README.md:68-76); no shared case file exists |
| S2 | D09 (013R:145) | yes | |
| S2 | D20 (013R:146) | no | Locations are named. No list of attempts (who tries, by what command, how often), so 'successful reads over attempts' has no denominator |
| S2 | M1, E17 (013R:147) | in part | Categories and the three cases (013R:113) are fixed. Trials per client and case, and the prompts, are not |
| S3 | M0 (014R:138-148) | in part | As S2 |
| S3 | D20 (014R:154) | no | It inherits S2's missing list, and does not say who makes an attempt on the second machine during a cut of all traffic |
| S3 | M1 (014R:155) | yes | Countable given witnesses. 'Unavailable reasons' wait for a portable cause field (014R:234-236). The cells cannot be prepared as accepted (014R:123-125) since ruling 77 |
| S3 | M2 (014R:156) | yes | Seven panel actions are listed. Its list of A74's hypotheses depends on a probe nobody has started (ROADMAP.md:69) |
| S4 | M0 (015R:124-134) | in part | As S2 |
| S4 | M1, D02 (015R:136-141) | yes | Twelve cells, one trial each, fields listed. The expected calls and effects per cell are in words only (015R:59-61) |
| S4 | M2, D10 (015R:142-146) | in part | 'Correct explanations' has no rubric and no named judge; the rows are not enumerated; the rule for T1's repair is in the plan (015P:206-216) and, by P03 I7, names a call that CON-024 refuses |
| S4 | M3 (015R:147-150) | yes | |
| S5 | M0 (016R:117-127) | in part | As S2 |
| S5 | M1, D10 (016R:129-134) | in part | Five classes with their rule IDs. The right next step for each class, and the judge, are not registered |
| S5 | M2 (016R:135-140) | in part | E03, E04, E10 and E16's seven cases are countable. 'Every task branch' has no list |
| S6 | M0 (017R:125-135) | in part | As S2 |
| S6 | M1 (017R:137-141) | yes | The expected value, no forbidden dispatch, follows from P2 and P3 (017R:48-55) but is not written as a number |
| S6 | M2 (017R:142-147) | in part | 'Retained record/data checks' is not a fixed list; the branches are not enumerated |
| S7 | M0 (018R:109-122) | in part | As S2 |
| S7 | M1, which is D01 (018R:123-126) | in part | Defined by reference to S2's D01, now accepted. S7 has no task in which an adapter is written (018R:86-99), so its middle row is undefined |
| S7 | M2 (018R:127-130) | in part | 'Extra author intervention' is defined nowhere, and P3 sets no limit on it (018R:48) |

Yes: 6 (S2's D09; S3's M1 and M2; S4's M1 and M3; S6's M1). In part: 15. No: 2. Everything missing can be written by the coordinator without changing an accepted condition, except D01's plain row (the owner's, already waiting as S2's plain server) and S3's fault method (the owner's, already waiting as S3-OF).

**Evidence.** The cited lines of the six READMEs, read in full. docs/implementation/FIRST_BUILD.md:155 ('Inspection effort | The files and commands that a person must read or run to answer T3'). experiments/013-s2-shared-login/plan.md:110-124, :126-142. packages/scenario-s2-linear-restrictions/README.md:68-76 (:75 'D05's onward-call case remains unexercised'). experiments/015-s4-partial-setup/plan.md:206-216. spec/promises/conduits.yaml:321-338 (CON-024: 'A control is not a management verb'). `grep -n -i intervention` over docs/implementation/FIRST_BUILD.md and docs/proposals/first-build/increment-4/measurement-plan.md: no match; in the READMEs only experiments/018-s7-outside-package/README.md:48, :128, :155 and 013's D09. docs/proposals/ruling-77/README.md:69-73. docs/proposals/ruling-45/README.md:84-86.

**Waits.** I4.

**Verifier.** partly. Recounted: 23 measures (S2 6, S3 4, S4 4, S5 3, S6 3, S7 3), and the 6/15/2 arithmetic holds. These hold: 'Inspection effort' is defined for T3 (docs/implementation/FIRST_BUILD.md:155), S2's M0 refuses 'T3's reach rule' (experiments/013-s2-shared-login/README.md:136), and S4's repair rule is in experiments/015-s4-partial-setup/plan.md:206-216. `grep -c -i intervention` returns 0 on FIRST_BUILD.md and on the measurement plan. Does not hold: (a) D05's 'cases are not enumerated, so ... no denominator'. D05's own rule lists five assertions (013 README:144), and packages/scenario-s2-linear-restrictions/README.md:68-76 enumerates exactly those five cases, which gives a 5 × 2 matrix with a freeze rule. What is missing is the shared case file and the sub-cases. (b) D20 is rated 'no' and D05 'in part' under two different standards. D20 fixes the five locations, four outcome classes per Agent process and the ratio 'successful reads over attempts' (013 README:146); only the attempt procedure is missing, which makes it 'in part'. (c) S3's M1 is rated 'yes' although 'unavailable reasons' wait for G5's field (experiments/014-s3-two-machines/README.md:234-236) and missing witnesses leave cells unmeasured (:155). (d) S7's M1 reason is overstated (see I1).

**Verifier's corrections.** Either rate D20 'in part' (0 no, 17 in part, 6 yes) or rate D05 and S3's M1 by the same standard as D20. D05's gap is the shared case file, not its denominator.


## FR-505

**Before any harness order is written, add one dated, labelled amendment to each pre-registration that fixes only what a count is missing**

G5 I4 · recommendation · verifier: partly · decides: coordinator

Change. Under each accepted README, a section 'Amendments since acceptance', dated, each line marked as the coordinator's and open to the owner to strike, changing no accepted condition. It holds the counting rules that I3 and the tables show missing:
1. D20 (S2, S3): the fixed list of attempts (I5).
2. M1 for E17 (S2): the trials per client and case, and the prompts.
3. D05 (S2): the cases enumerated in the README from the package's table, and one case file run on both non-plain rows, which the roadmap already asks for ('vectors shared by two implementations').
4. D02 (S4): a twelve-row table of expected calls, effects and outcome.
5. D10: for S4's M2, the rubric and who judges it, blind to the harness; for S5's M1, the right next step for each of the five classes.
6. M2 of S5 and of S6: the list of branches; S6's list of retained-data checks.
7. D01 in S7: what its middle row is, or 'not applicable' with two rows reported.
8. S7: what counts as an intervention.
9. D15 (S10): the implementations the 'six' are; `lift.v1` requests added to D16's ledger, which predates order 042.
10. M0 in every scenario: either define 'inspection effort' for that scenario's own tasks or state that it is not carried.
11. E07 and E08: the measure ruling 44 point 2 asked for (I6).
12. E09's real replay: labelled an observation with no pass condition, since nothing it shows can fail P4.
Two corrections ride with it: S4's plan counts a repair that cannot be dispatched (P03 I7), and S9's M2 expects a hook attempt at restart that OBS-004 forbids (I14).

The roadmap's round-6 table gains two columns, where each row is held and whether its rule is fixed, so that the next reader does not recount.

Cost: one documents order across nine READMEs and the roadmap, with a fresh verifier; no ruling and no owner time. The existing paradigm is a registered report's dated amendment before data collection: the accepted text stays, the amendment is visible and separately dated.

Buys: ruling 45 met measure by measure and not only in form; harness orders sized against fixed lists; and the owner's one amendment question (the critic's group 4) shrinks to the changes that really cut accepted text.

What it must not do: move a case, drop a run, or turn a recorded observation into a new pass condition. Those are the second and third classes of I7 to I15.

**Evidence.** I3's table and the row tables I16 to I23. ROADMAP.md:173 ('D05: the first restrictions as complete contracts with vectors shared by two implementations'). experiments/021-s10-swapped-providers/README.md:118-123 ('Cover all six implementations') against plan.md:173-178 (the suites run on the packaged local providers, the replays on the built-ins). experiments/015-s4-partial-setup/README.md:59-61 ('unkeyed calls are new work'), :246-248 (one replay). First pass: verifier on E01 I7 ('the coordinator adds every tightening that changes no accepted condition as a dated, labelled amendment') and on E01 I13 ('a dated section headed as amendments since acceptance, each marked as the coordinator's and open to strike').

**Waits.** S2-H, S3-H, S4-H, S5-H, S6-H, S7-H1 and S7-H2, S8-H1 and S8-H2, S9-H and S10-H2a should be written after it.

**Verifier.** partly. The basis holds. E01 I7's and E01 I13's verifiers both endorse dated, labelled amendments under the accepted text. Ruling 45's rule is met if the counting is fixed before the run (docs/proposals/ruling-45/README.md:84-86). Most of the twelve lines fill a missing counting rule, which is the coordinator's under rulings 37, 46 point 4 and 71 point 4. Three lines are not tightenings. Item 10's 'or state that it is not carried' removes part of an M0 accepted under ruling 75 D1 (experiments/013-s2-shared-login/README.md:127-137). Item 12 relabels the replay that Q3's answer says 'governs' (experiments/015-s4-partial-setup/README.md:246-248). The S9 correction changes an accepted value. That is justified, since spec/ governs (AGENTS.md 'What holds'), but the engine fix must be scoped: DL16 leaves to the engine what happens to its processes when it starts again (spec/implementation-defined.yaml:182-186).

**Verifier's counterargument.** The paradigm the item names, a registered report's amendment, has the accepting editor approve protocol changes made after acceptance. Here that role is the owner's list of decisions to strike, which stops at item 29 (`grep -c '^| [0-9]' docs/proposals/first-build/to-review-with-the-owner.md` = 29; docs/implementation/HANDOVER.md:252). Without a row there, 'no owner time' becomes 'the owner unaware'. Better: one amendment section per README, plus one row per scenario on the review list.

**Verifier's corrections.** Mark items 10 and 12 as the owner's or drop them. Add that each amendment gets a row on the review list. Limit the S9 engine fix to setup dispatched at start for registered or failed instances under root's identity.


## FR-506

**D20: one fixed list of attempts, kept in S2 and S3 where ruling 44 placed it, and shared with A5 and A7**

G5 I5 · experiment-change · verifier: partly · decides: coordinator

Register rows: ROADMAP.md:140 and :173 (S2), :141 and :174 (S3); it feeds :64 (A7) and, if the sign-in test of E04 I1 is adopted, :62 (A5).

Why this row. D20 is the one measure with no countable rule (I3), and four first-pass items move or thin it (I2). E04's verifier also found that the isolation matrix proposed for A5 duplicates it.

The claim it tests: what a runtime protects is a property of the runtime, and Conduit says so. On the local profile a process of an Agent can read the listed credentials; on an isolated runtime it cannot; on both, inspection reports every restriction as mediated (ACC-027).

The change, an addition to D20's counting rule that alters no accepted condition:
- Locations: the five the README names (the Agent's own identity, the other Agent's identity, the service key or its store, the runtime channel, the engine's state).
- Attempts, for each Agent and each location: (1) a harness probe started as that Agent's run, with its operating-system user, environment, working directory and mounts, tries to read a synthetic marker placed at the location; (2) the real client is asked once, with a registered prompt, to read it with its own tools. Outcomes: read, not read, declined, not tested.
- Count: reads over attempts by location, profile and phase. Five locations, two Agents and two means make 20 attempts per profile and phase. Markers are synthetic only, as the README already requires; no real keychain item is read.
- Phases: S2, once per profile. S3, at the phases the accepted text names (before, during and after each cut, and after the loss and return), with the probe scheduled on the second machine before a cut and its result read after the heal; the intervals without access stay as accepted.

Pass condition: unchanged. P6 records what is readable and infers no confinement. The local result goes to A7's security claim. A condition of 'no read on the isolated profile' is a claim about a runtime provider and belongs in that provider's contract suite, as the verifier of E01 I2 said.

Measures: reads over attempts by location; the not-tested cells listed; the interval list.

Cost: one probe binary in S2-H, in Rust, and four prompts. Buys: D20 can be counted; repeating it in S3's cells becomes free once it is scripted, so the first pass's reason to drop it there goes; one list serves S2, S3, A5 and A7.

**Evidence.** experiments/013-s2-shared-login/README.md:146 (D20: 'Count successful reads over attempts by location ... absence of an attempt proves nothing'), :63-65 (P6: 'No visibility or confinement is inferred from a successful call'), :123 ('Use synthetic secret markers for leak checks'). experiments/014-s3-two-machines/README.md:154 ('Repeat the credential-location attempts and requests before/during/after each partition and after loss/return'), :64-66. ROADMAP.md:173-174. docs/proposals/reaudit/round-6/DECISIONS.md:124. First pass: E01 I1 ('D20 (no attempt procedure: who tries, with which commands)'); verifier on E01 I2 ('a provider's contract claim ... belongs in A63-DR's suite'); verifier on E04 I1 ('The isolation matrix duplicates S2's accepted measure D20 ... reuse D20's definition').

**Waits.** S2-H's design; S3-H; E04 I1's matrix, which should cite this list.

**Verifier.** partly. These hold: experiments/013-s2-shared-login/README.md:146, :63-65 and :123; experiments/014-s3-two-machines/README.md:154; ROADMAP.md:62, :64 and :173-174; and 5 locations × 2 Agents × 2 means = 20. The coordinator is the right party. Weak points: the second means, a real client asked to read with its own tools, is A7's design ('An Agent is told to reach data it is denied', ROADMAP.md:64). It exists only where the template names a directory (docs/proposals/ruling-75/README.md:39-42), and a model may decline. On the local profile the probe's result is largely known beforehand: one trusted OS user and local readability (013 README:259-261), the keychain's access rules apart. Round 6 also asked of D20 'whether a user sees [the trust limits] at the point of choosing a runtime' (docs/proposals/reaudit/round-6/DECISIONS.md:124), which the list does not measure. 'Free once scripted' in S3 assumes a cut the coordinator controls, while S3-OF is open.

**Verifier's counterargument.** Keep means 1 (the scripted probe) in S2 and S3, and leave means 2 to A7, which owns it. Attach the same probe to S10's Docker replays (experiments/021-s10-swapped-providers/README.md:85-86), which run S2's relationships on the isolated runtime anyway. That protects D20 against the compound drop in I2.

**Verifier's corrections.** Name A7 as the place for the real-client attempt. Add the 'user sees the trust limits when choosing a runtime' observation, or state that D20 leaves it out.


## FR-507

**E07 and E08: ruling 44 point 2 asked S1 and S2 to measure whether a broker is good enough, and no measure exists**

G5 I6 · experiment-change · verifier: partly · decides: coordinator

Register rows: ROADMAP.md:172 (E08, in S1) and :173 (E07, in S2).

Fact. Ruling 44 chose option a for narrow delegation and narrow provisioning, and said 'S1 and S2 measure whether that is good enough; if it is clumsy, a focused constraint, such as a list of allowed templates on access to management, comes back to the owner'. IN_FORCE.md carries the sentence. E08 exists as a vector whose broker is simulated: the fixture 'does not test that adapter's approval implementation'. E07's P4 has the engine permit what ACC-030 permits and records the author's intended grant, with nothing to compare it with. The word 'broker' appears in no measure document. So the trigger the owner set can never fire.

The claim it tests: v0's two stated limits of delegation are good enough. A bound does not limit the filters of the Conduits its holder opens (E07); access to management cannot be narrowed to one template, so narrow provisioning goes through a broker (E08).

The change, which adds measures and changes no accepted condition:
- E07, inside S2's T3. Before the attempt each author writes, from inspection output alone, which Conduits Agent A may open. Recorded per author: match or miss against the engine's decision. A miss is logged as a finding against inspection under ruling 52. It is not a pass condition of P4.
- E08, on paper, in the manner of experiment 023. Write the documents an author needs to offer 'start only reviewer X' through a broker (its contract, Type, template and Conduits) and count files and lines by ruling 63's count, with the broker's implementation listed as provider work, not yet counted. Beside it, the same need under the alternative ruling 44 names, a list of allowed templates on access to management. No engine and no run, so S1's counts are not touched (ruling 71 point 4).

Pass condition: none. The two counts and the match-or-miss record go to the owner, who judges 'clumsy'.

Cost: one small documents order for E08; one line in S2's amendment for E07. Buys: the owner's own condition in ruling 44 becomes answerable before v0, instead of silently lapsing.

**Evidence.** docs/proposals/ruling-44/README.md:26-32 (point 2). IN_FORCE.md:115 ('S1 and S2 measure whether that is good enough; if it is clumsy, a focused constraint ... comes back to the owner'). docs/AUDIT.md:199-204. conformance/README.md:415-416 ('The broker fixture supplies the trusted adapter's approved onward request; it does not test that adapter's approval implementation'). conformance/s1-safety-cases.md:67 (E08-broker: 'trusted adapter's approval check is separate provider evidence'). experiments/013-s2-shared-login/README.md:55-58 (P4). `grep -rn -i broker docs/implementation/FIRST_BUILD.md docs/proposals/first-build/increment-4/*.md experiments/013-s2-shared-login/README.md`: no match. ROADMAP.md:172 (E08 'with its trusted code and setup counted (D04)'). First pass: E01 I1 and its verifier's correction (f) ('P4 has the author record the intended grant; it lacks a threshold').

**Waits.** S2's amendment (I4); the owner's later judgment under ruling 44 point 2.

**Verifier.** partly. Holds: docs/proposals/ruling-44/README.md:26-32; IN_FORCE.md:115; conformance/README.md:415-416 and conformance/s1-safety-cases.md:67 (the broker is simulated); and P4 records only the intended grant (experiments/013-s2-shared-login/README.md:55-58). `grep -rln -i broker` finds the word in no measure document; experiments/013-s2-shared-login/plan.md:44 names only an existing slice. Against the E08 change: ruling 45 point 4 dropped 'an author's effort counted on paper' (docs/proposals/ruling-45/README.md:76-78), and experiment 023 stands only as a design input that is 'never the bar' ('X2's dropped author-effort experiment supplies no authority', experiments/023-paper-count/README.md). Ruling 44 point 2 also says that S1 and S2 'measure' it, that is, in runs.

**Verifier's counterargument.** Narrow provisioning is met in practice in A78's orders 4 and 5 (ROADMAP.md:71). That design avoids it with 'fixed worker and tree slots set up once by root', and this avoidance is itself evidence for the owner's judgment of 'clumsy'. Record it beside any paper count. The same unmeasured-trigger pattern holds for D09: A67's restart verb comes back 'only if that is disproportionately disruptive' (docs/proposals/reaudit/round-6/DECISIONS.md:113), and no measure defines that.

**Verifier's corrections.** Frame the E08 count as a design input under a work order, as 023 is, not as an author run. Add A78's slot design as evidence. Extend the item to D09's trigger.


## FR-508

**S2: the proposed changes in three classes**

G5 I7 · assessment · verifier: partly · decides: coordinator

First class, tightenings that change no accepted condition (the coordinator's, as dated, labelled amendments):
- an oracle table, one row per task step, and a rule for a model that declines a prompted step (E01 I2 item 2);
- D05's cases enumerated and run from one shared case file on both non-plain rows (item 4, first half);
- the snapshot check after rotation, replacement and repair (item 6);
- D20's fixed list of attempts (item 7, first sentence; I5);
- E07's prediction written before the attempt, as a record and not a pass condition (item 5; I6);
- D01's statement of what each row adds and what it costs (item 3, the table half);
- one added E17 observation, a call held in flight while its Conduit closes, with no pass condition (E02 I5 point 9 in its verifier's form);
- M1's trials per case and its prompts.

Second class, changes that reverse or cut accepted text or an answer (the owner's):
- the two isolated runs dropped and the fresh author's run put on scripted calls (E01 I2, 'Runs'): the four-run matrix R1, with O1 'accepted', ruling 75 D1 of 2026-11-11. With the isolated runs goes D20's isolated half in S2 (ruling 44).
- E17 out of S2 (item 8): P6 and the roadmap's S2 row, ruling 44 and ruling 75 D1.
- the batch-2 orders out of the run gate: gap G1's 'Await it'; it also bears on ruling 71 point 2.
- D01's adapter row rebuilt around an unchanged server, or dropped: 'Three rows' in D01's definition (ruling 44). What is new: order 139 built the adapter as a second server from scratch on 2026-11-12, under an admin decision already logged for the owner to strike.
- D05's onward case, if 'dependent calls' leaves S2's row.
- D20's isolated half moved to S3 with a new 'no successful read' pass condition: ruling 44's placement in S2 and in S3, and S2's stated limit that no confinement is inferred.
- a baseline of today's way counted against S2 (item 1): the accepted M0 says 'Do not carry A70's comparison/bar', and ruling 45 put round 6's baseline in S1's row. As a new no-engine row beside A70, with no bar, it changes nothing in S2 and needs only its register row.
- Linear's hosted server, signed in by OAuth, as the plain row (E01 I9 option b): O4's answer 'the keychain alone' (2026-11-11), ruling 75 C1's limit on where a login package runs, and the pinning rule. What is new: the search of 2026-11-12 found no open-source server that runs unchanged against the stub. The plain server already waits for the owner.
- the key rotated by the harness instead of the owner's hand (E01 I11): T5 and the run text name the owner.

Third class, asking again with no new situation: none. S2's real clients are ruled by rulings 47 and 68 point 4, which name no number of runs, and no S2 proposal re-asks one of O2 to O5 without something new.

Judgment: the first class is where S2 gains a way to fail on its accepted conditions. Of the second class I would put only the adapter's meaning and the plain server, as one item that already waits. I would keep E17 and both halves of D20 in S2.

**Evidence.** experiments/013-s2-shared-login/README.md:55-58 (P4), :63-65 (P6), :114 (T5 'the owner rotates'), :136-137, :143 ('Three rows'), :144, :146, :174-175 (R1), :185 (pin), :189-190, :206 and :213-216 (G1, 'Await it'), :247 (O1), :251 (O4), :258-261 (limits). experiments/013-s2-shared-login/plan.md:110-124, :126-142. docs/implementation/HANDOVER.md:272 ('the small adapter is the glue ...'). docs/AUDIT.md:170. docs/proposals/ruling-75/README.md:52-54 (C1), :79-84 (D1). docs/proposals/ruling-45/README.md:61-66. docs/proposals/ruling-47/README.md:38-39. ROADMAP.md:140, :173. First pass: E01 I2 with its verifier ('the oracle table, a shared D05 case file ... and the P5 snapshot are tightenings the coordinator can add'), E01 I9, E01 I11, E02 I5 point 9.

**Waits.** S2-H; S2-PL; the owner's answer on the plain server.

**Verifier.** partly. The cited lines of experiments/013-s2-shared-login/README.md (55-65, 114, 136-137, 143-146, 174-175, 185, 206, 213-216, 247, 251) and plan.md:110-142 read as stated. The run cut (R1, O1) is correctly put in the second class, and third class 'none' stands. Not as stated: 'the adapter's meaning and the plain server, as one item that already waits'. Only the plain server waits (docs/AUDIT.md:170). The adapter's meaning is an admin decision logged for the owner to strike (013 plan.md:110-121; docs/implementation/HANDOVER.md:272), not a waiting question. The first-class 'added E17 observation' adds a case to real-client runs with 45 minutes of the owner's time (013 README:190-191). It is small, but it is an addition, not a tightening.

**Verifier's corrections.** Say: the plain server waits; the adapter's meaning is on the strike list. Label the E17 case 'an addition reported apart'.


## FR-509

**S3: the proposed changes in three classes**

G5 I8 · assessment · verifier: partly · decides: coordinator

First class (the coordinator's, as dated, labelled amendments):
- three orderings of one keyed call fixed by barriers, with an attempt at each stage to record that the effect did not occur and then a repeat, expecting conflict before the fence and at most one effect per key (E01 I4, its PA2): it adds to the effect cells and to V2;
- the time from a cut to 'unavailable' against the provider's stated bounds, with the verifier's corrected bounds (PA3);
- the diff of S1's and S2's documents from their one-machine form, by kind of line, as a measure (PA1);
- T6 reports a completion that finds no unit as 'nothing left to stop'; the unit's state before and after each heal and any overlap interval are recorded (P05 I3 items 3 and 5);
- a loss declared during a cut with a run still alive, as an added, labelled phase reported apart from P4 (P05 I3 item 1; E01 I4, its PA4), once the cut method is settled;
- the cost of adding the machine, recorded once as D15's operator-setup field applied to S3's machine provider (P05 I11 in its verifier's form);
- a disposable Linux node for development and rehearsal, never counted as S3 evidence (P05 I10 in its verifier's form);
- S3's runs placed after the MC2 and MC4 re-checks (E05 I7, without relabelling S3);
- V7 cites the enrollment promises that now exist, as the README says to do.

Second class (the owner's):
- the fault method (E01 I8 option D; P05 I2 options D and E): it replaces the accepted fault preparation (ruling 75 D1, 2026-11-11). There is a new situation: ruling 77, later on 2026-11-12, turned interop and sudo off, so the accepted method cannot be followed. It already waits as S3-OF, whose three offered options are all on the Windows side.
- the revocation axis dropped (E01 I4): E15's second axis (ruling 44) and half the accepted cells.
- D20 out of the cells (E01 I4), and the isolated profile on the second machine moved to v1 (P08 I16): P6 and 'D20, under a partition' (ruling 44).
- fault cells on scripted clients, the two-author split dropped, runs cut from four to two or one (E01 I4, P08 I16): the run matrix R1 and O1 'accepted', ruling 75 D1. The new situation, ruling 77's hand actions, applies to the cells only.
- M1's rule for P4 rewritten to require that an owed stop end a live unit (P05 I3 item 3, second half).
- `wsl --terminate` in place of `wsl --shutdown` (P05 I3 item 4): O4's answer of 2026-11-11; ruling 72's later answer of 2026-11-12 already names that command for a pause.
- A74 registered again around the SSH provider, its stand-ins not built (E01 I4, P05 I5, P04 I12, P08 I14): ruling 43's probe and the accepted prerequisite that A74's dated observations exist first.
- S3 called a demonstration (E05 I7): the roadmap's 'Central to the claim since ruling 40' and ruling 47's gate.

Third class (asked and answered, nothing new):
- the real-client replay cut to one read and one write by each Agent (E01 I4). On 2026-11-12 the coordinator put that 'S3, S7, S8 and S10 keep their real-client runs on the second accounts'; the owner: 'Agree with all decisions except need more info on 2'. The same day: 'I want to use this within conduit as a test of agent subscription isolation and our entire model'.
- the carried Codex login (A81, V6, and the two orders for the return) moved to v1 (P08 I16). The owner, 2026-11-11: 'for C5 can't we just do browser through ssh as well?' and 'also confirm C5 and accept all other rulings'.
- whether PRV-021 needs a stored stop at all (E01 I4, E01 I10, P04 I4). Put on 2026-11-12: 'Every runtime provider that Conduit ships carries out a stop it still owes on a machine before it starts any run there'; the owner agreed in the same answer. Measuring the seconds from a cut to a run's own exit is first class; reopening the answer waits for that number.

Judgment: settle S3-OF first, with a cut the coordinator can drive from the Mac. That removes the owner-hours argument for dropping the revocation axis and D20 from the cells, so both stay. The full real-client replay on the second accounts stays once, as the first pass's own verifier proposed.

**Evidence.** experiments/014-s3-two-machines/README.md:51-54 (P3), :55-59 (P4), :64-66 (P6), :123-134 (fault preparation; :126-127 the four cells), :154-156, :168 (V2), :173 (V7), :184-186 (R1), :192, :215-216 (A74 first), :259-264 (O1 to O4). experiments/014-s3-two-machines/plan.md:259-298 (S3-OF, options A to C). docs/proposals/ruling-77/README.md:69-73. docs/proposals/ruling-75/README.md:16-20 (the owner's words), :63-65 (C5), :114-120 (later answer). docs/proposals/ruling-72/README.md:42-64 (the order of the answers of 2026-11-12), :63-64 (`wsl --terminate`), :76-82. docs/proposals/ruling-42/README.md:275-286. IN_FORCE.md:244. ROADMAP.md:69, :141, :174. First pass: E01 I4 and I8 with their verifiers; P05 I2, I3, I10, I11 with theirs; E05 I7; P08 I16 and its verifier ('dropping the isolated profile empties it').

**Waits.** S3-OF; S3-H; the four S3 runs.

**Verifier.** partly. The S3 citations hold: experiments/014-s3-two-machines/README.md:51-66, :123-134, :154-156, :184-186, :215-216, :259-264; plan.md:259-298; docs/proposals/ruling-42/README.md:279-284; docs/proposals/ruling-75/README.md:16-20; docs/proposals/ruling-72/README.md:63-64. Does not hold: (a) Cutting the real-client replay (E01 I4) is classed as re-asking the 2026-11-12 answer. That answer says only that S3 keeps its real-client runs on the second accounts (docs/proposals/ruling-75/README.md:117-120), which E01 I4's S3b does. The number of replays was fixed by R1 and O1 on 2026-11-11 (014 README:184-186, :259), so this is the second class, as the matching S2 cut is in I7. The quote 'a test of agent subscription isolation and our entire model' answered whether the second accounts could add work-order capacity (docs/proposals/ruling-72/README.md:76-82), not how large S3 is. (b) The first class includes additions that add cells (PA2's three orderings) and, under option A, owner actions: P05 I3's verifier counts 8 more per run. They are the coordinator's only once S3-OF gives a cut the coordinator can drive. (c) The judgment 'removes the owner-hours argument ... so both stay' answers cost, while E01 I3's reason was validity (014 README:131-132).

**Verifier's corrections.** Move E01 I4's replay cut to the second class (R1 and O1, ruling 75 D1, 2026-11-11). Make PA2 and T6b conditional on S3-OF. Answer E01 I3's validity argument before keeping the revocation axis.


## FR-510

**S4: the proposed changes in three classes**

G5 I9 · assessment · verifier: partly · decides: coordinator

First class (the coordinator's):
- a twelve-row table of expected calls, effects and outcome for D02 (E01 I6 item 1, first half);
- a rubric for M2 (no unsafe repeat suggested; no effect asserted failed without evidence) and a judge who has not seen the harness (item 2, the rubric);
- the lifecycle model asked whether a Resource can outlive its retired Pool (item 4, first half);
- E20 left as it is, an observation of a stated limit (item 5);
- the two lost-response cases on experiment 008's stub, run as an extension of A71 with ruling 75 D4 untouched (the verifier's form of item 3). It spends the primary subscriptions, so the owner is told;
- the plan's rule for counting T1's repair corrected, since the call it counts is refused while the instance is not ready (P03 I7; the rule is the admin's own, logged to strike, and not accepted text);
- which of E01's fixture or a missing rule is at fault where a plain server's operation is marked as starting model work (the verifier on E01 I14), as a language finding.

Second class (the owner's):
- the twelve cells run once and not once per author (item 1, second half): 'twelve per author run' in M1, ruling 75 D1.
- 'engine restarted mid-call' added to the one real-client replay (E03 I6): Q3's answer fixed one replay of one thing.
- S9's T4 moved into S4 as a restart variant (E03 I6): it changes two accepted task lists.
- ACC-033's retired-owner rows removed if the state cannot be reached (item 4, last clause): a change to the language that removes behavior.

Third class (asked and answered, nothing new):
- the coordinator's run not counted (item 2). Ruling 75 D3, 2026-11-11: 'S4, S5, S6 and S9 are each written twice, by the coordinator and by a fresh Codex author, with scripted clients'. The README already said 'coordinator knowledge is a limitation' when it was accepted.
- S4's replay dropped for experiment 008's stub on the primary accounts (item 3). Ruling 75 D4, 2026-11-11: 'Beyond S1 and S2, the only real-client run is S4's one lost-response replay on a second account, which also gives A6 its real client's trace'; confirmed on 2026-11-12. The fact offered as the reason, that neither client declares Conduit's extension, dates from 2026-11-09.
- E18's boundary fixture dropped (item 4, middle). Q4's answer, 2026-11-11: 'a boundary fixture and the reachable path, labelled apart'. The doubt about reaching the state was in the accepted text as gap G2.

Judgment: S4's seven rows all stay. The table and the rubric are what the scenario lacks. Run the stub probe if the allowance permits and let its result, not the review, decide whether the replay still shows anything.

**Evidence.** experiments/015-s4-partial-setup/README.md:59-61 (P4), :62-64 (P5), :136-141 (M1, 'twelve per author run'), :142-146 (M2), :179-180 ('coordinator knowledge is a limitation'), :214-217 (G2), :245 (Q2), :246-248 (Q3: 'this one replay governs'), :249 (Q4). experiments/015-s4-partial-setup/plan.md:196-216. spec/promises/conduits.yaml:321-338 (CON-024). docs/proposals/ruling-75/README.md:87-90 (D3, D4), :117-119. experiments/008-real-client/results-2026-11-09.md:63-64 ('Neither client's requests declare Conduit's extension in its client capabilities'). docs/implementation/HANDOVER.md, queue row 13 (apply calling a control is batch 2's, 'for S4's T1'). First pass: E01 I6 and its verifier ('do not substitute, sequence'); E03 I6; P03 I7 (verifier: supported); the verifier on E01 I14.

**Waits.** S4-H; S4-ACCOUNT; order 144's finishing order.

**Verifier.** partly. The S4 citations hold: experiments/015-s4-partial-setup/README.md:59-64, :136-146, :179-180, :214-217, :245-249; plan.md:196-216; docs/proposals/ruling-75/README.md:87-90, :117-119. The third-class S4-E09 entry holds. The URL-seam cost was in the plan by the evening of 2026-11-11 (`git blame -L 274,278 experiments/015-s4-partial-setup/plan.md`: 1ecdf3d10, 2026-11-11 20:15), before the owner's reconfirmation of 2026-11-12. Does not hold: the E18 drop as 'asked and answered, nothing new'. E01 I6 makes it conditional on a lifecycle-model result (MC6) that does not exist yet. If that result shows the state cannot be reached, that is a new situation, and the drop then goes to the owner (E01's verifier, correction d).

**Verifier's corrections.** E18 boundary drop: 'the owner's, after MC6 reports', not the third class.


## FR-511

**S5: the proposed changes in three classes**

G5 I10 · assessment · verifier: partly · decides: coordinator

First class (the coordinator's):
- the right next step registered for each of M1's five conflict classes (E02 I7 point 5; the rule IDs are already registered);
- M2's list of branches fixed in the README (point 7, first half);
- a mode of the fixture that applies another value than it was sent and reports success, as a measure with no pass condition, or better as one case in the package's own contract check, 'after a success the readback equals what was sent' (the verifier's form of point 4);
- a branch in which an Agent's control lands between an author's plan and apply, once apply calls a control (the verifier's addition; batch 2).

Second class (the owner's):
- T3, T4 and T5's other branches carried by vectors alone (point 2): E03, E04, E10 and E16 leave S5's run (ruling 44's S5 row; P4 and the run text of ruling 75 D1). Vectors drive the core only, so the shell's evidence and ruling 47's 'runs end to end' would go with them.
- M0 reduced to time, mistakes and interventions (point 7): it drops D22's five counts (ruling 44 point 1, 2026-11-07).
- the disobedient fixture as a pass condition on the author (point 4 as written): a new condition on P3, probing ruling 41's decision 15.
- 'dispatches per revision at most one' as P2's pass (point 3 as written): stricter than REV-012, so it can fail with the claim true. A burst without barriers also sits against the stated limit that a scripted race is a deterministic interleaving.
- the author's tasks cut to T1, T2 and one branch of T5 (point 1): 'each doing every task and listed branch once'. The list also leaves out T6, where M1's five diagnoses are made, while point 5 still scores them.

Third class (asked and answered, nothing new):
- one fresh author, the coordinator's run dropped (E02 I8 option b). Ruling 75 D3, 2026-11-11, and Q2's answer in the README: 'written twice, by the coordinator and by a fresh Codex author, with scripted clients (D3)'.

Judgment: E03, E04, E10 and E16 stay in S5, driven once by the harness on the engine; the registered next steps make D10 countable. I would not shrink the author work without the owner, and I would not ask.

**Evidence.** experiments/016-s5-control-contention/README.md:52-55 (P3), :56-59 (P4), :102-107 (T3, T4), :112-113 (T6), :129-134 (M1), :135-140 (M2), :166-168 ('each doing every task and listed branch once'), :177-178, :229 (Q2), :236-237 ('A scripted race is a deterministic interleaving, not a contention-load benchmark'). ROADMAP.md:176. docs/proposals/ruling-44/README.md:22-24. IN_FORCE.md:271, :274. docs/proposals/ruling-75/README.md:87-88. engine/crates/conformance/Cargo.toml:10-11. docs/proposals/ruling-47/README.md:38. First pass: E02 I7 and its verifier (its points 1 to 6), E02 I8, and 'What the verifier says the analyst missed' (apply against a live control).

**Waits.** S5-SRV and S5-H.

**Verifier.** partly. The cited lines hold: experiments/016-s5-control-contention/README.md:52-64, :102-113, :129-140, :166-168, :177-178, :229, :236-237; docs/proposals/ruling-75/README.md:87-88. The second and third classes are right. One first-class item is not a tightening: a branch in which a control lands between plan and apply needs apply to call a control, which is batch 2's (docs/implementation/HANDOVER.md:221, row 13). S5's plan says batch 2 is not a prerequisite (experiments/016-s5-control-contention/plan.md:157).

**Verifier's corrections.** Mark the plan-and-apply branch as reported apart and not gating S5, or as the owner's because it adds a prerequisite.


## FR-512

**S6: the proposed changes in three classes**

G5 I11 · assessment · verifier: supported · decides: coordinator

S6 holds no E-case and no D-measure; D22 through M0 is its only round-6 row.

First class (the coordinator's):
- M1's expected value written as a number: no forbidden dispatch after a close or a retirement (E02 I5 point 2, first half);
- M2's retained-data checks as a fixed list (point 8, first half);
- the plan's 'T4 restart case' removed: the accepted T4 has no restart, so the plan added a dependency on S9's orders that the README does not have (point 7);
- the tailnet and HTTPS steps taken off S6's path where the accepted text says one local machine (point 5, in part);
- the child that outlives a stop, re-homed as its verifier advised: a gap in spec/ (nothing says what a run comprises) for the language list, one case for PRV-030 in the runtime provider's contract suite, and in S6 at most a measure with no pass;
- where the fixture keeps its ledger relative to the instance's state directory, fixed before the run, since removal deletes that directory (the verifier's point on P4).

Second class (the owner's):
- S6 split and its core run first among S2 to S10 (point 1): the order of the work, and the acceptance scenario is S1 and S2.
- the task fetch after a close as a core vector only (point 6), and the uncertain stop out of the run (point 5): both are in P2, P3 and the tasks accepted under ruling 75 D1.
- the order among unordered and cyclic members recorded, not judged (point 4): P5 says to judge 'the engine's declared DL1 choice', and Q4's answer of 2026-11-11 includes those retirements.
- P3 with a branch that must fail on the built-in runner and pass on a confining provider (point 3 as written): it conflicts with ruling 47's gate and puts ruling 75 B2's 'built later' provider on S6's path.
- M0 reduced to time, mistakes and interventions (point 8): it drops D22's five counts (ruling 44 point 1).
- a burst without barriers (point 2): against the accepted rule to inject at fixed barriers, and expected to pass on the shell's one writer lock by the analyst's own note.

Third class (asked and answered, nothing new):
- one fresh author, the coordinator's run dropped (E02 I8). Ruling 75 D3 and Q2's answer, 2026-11-11: 'written twice, by the coordinator and by a fresh Codex author, with scripted clients (D3)'.

Judgment: S6's best measure needs only its number written down. The escaped child is the real find, and it belongs to the language and to the provider's contract, not to a scenario fixture.

**Evidence.** experiments/017-s6-revoke-in-flight/README.md:48-55 (P2, P3), :60-64 (P5), :68-69 ('no E-case or D-measure is assigned to it'), :100-101 ('Inject failures at fixed barriers, not by timing sleep intervals'), :112-114 (T4), :137-147 (M1, M2), :181-185 (one local machine), :238 (Q2), :240 (Q4). experiments/017-s6-revoke-in-flight/plan.md:80, :138 ('S9-CUT2 for T4 restart'), :206-207. ROADMAP.md:152-153 ('The acceptance scenario is S1 and S2'). docs/proposals/ruling-47/README.md:38. docs/proposals/ruling-75/README.md:43-46 (B2), :87-88 (D3). First pass: E02 I4 and I5 with their verifiers (engine/crates/shell/src/runner.rs:268-275 reports a stop succeeded straight after the signal; 'the escaped child is a gap in the language before it is an experiment'), E02 I8.

**Waits.** S6-SRV and S6-H; the language list of Focus 1 for the gap on what a run comprises.

**Verifier.** supported. experiments/017-s6-revoke-in-flight/README.md:48-64, :68-69, :100-101, :112-114, :137-147, :181-185, :238, :240; plan.md:80 and :138 ('S9-CUT2 because T4's restart case uses its acknowledged lab repair cut'); ROADMAP.md:152-153; docs/proposals/ruling-75/README.md:43-46, :87-88. The classes follow E02's verifiers: the T4 restart is a plan defect and the coordinator's; the escaped child is re-homed as a language gap and a provider-suite case; the burst conflicts with 'fixed barriers'.


## FR-513

**S7: the proposed changes in three classes**

G5 I12 · assessment · verifier: partly · decides: coordinator

First class (the coordinator's):
- 'intervention' defined and counted, with the kinds that carry information named (the verifier on E02 I2);
- the author's pack frozen and hashed, the session's file-access log kept, and a sentence on whether the author may read the two Agent package archives it must install;
- added measures: each place the author was unsure and the promise consulted, each file read outside spec/, each tool of the server that could not be described, time;
- D01's middle row in S7 defined, or reported as not applicable with two rows (I3);
- a task in which the author checks its own package with a public tool before handing it over (the verifier's addition, from ruling 47's publication line);
- a second subject server with tools of its own, added beside the accepted candidate;
- a line in Limits that S7 is a fresh session standing in for ruling 47's outside-author check.

Second class (the owner's):
- S7 split into an authoring part with a scripted consumer and a later part with the real Agents (E02 I2): P3 has the real Agents use the packages in the same run (ruling 75 D1). The real-Agent part stays on the second accounts, so D2 is not contradicted; the scripted-consumer option was never put to the owner for S7.
- any help fails the run (E02 I2): the accepted P3 says extra guidance is recorded.
- D01's table dropped from S7: the roadmap's S7 row, ruling 44.
- the repeated-argument rejection, the conflicting selector, the wrong node and the filter on a plain server carried by vectors only, with the repeated argument an observation with no pass: P4, P5, T4 and T5, and round 6's real repeated-argument package, on which the open point in IN_FORCE waits.
- 'concepts met' dropped from M0: D22, ruling 44 point 1.
- the non-Rust plugin moved until after the decision to publish: P6 and T7 as accepted, though ruling 47's own table places the author check there.

Third class (asked and answered, nothing new):
- another server in place of the first candidate (E02 I2). Q3's answer, 2026-11-11: 'qualify mcp-remote first'. Nobody has yet tried to qualify it, and the accepted text already says to return a verified candidate if it proves ineligible.
- a second outside author from another vendor (E02 I3 option c). Q2 asked 'outside author Codex or Claude?' and the answer of 2026-11-11 is 'a fresh Codex session is the outside author'.

Judgment: define intervention and D01's rows, qualify the accepted candidate, and add a real server beside it. The split is the one second-class change here worth the owner's attention, because it takes S7's authoring evidence off the sign-in chain.

**Evidence.** experiments/018-s7-outside-package/README.md:45-48 (P3), :49-56 (P4, P5), :57-60 (P6), :64-65, :86-99 (T1 to T7; :90-91 'qualify the plain, adapter and aware variants by D01'), :103 ('gives the author no finished package'), :123-126 (M1), :127-130 (M2), :152-156, :165-170 (candidates 'unverified'), :195-199 (Q1, Q2), :200-202 (Q3: 'if ineligible, return a verified candidate'), :211-216 (answers). ROADMAP.md:177. IN_FORCE.md:246. docs/proposals/ruling-75/README.md:85-86 (D2). First pass: E02 I1, I2 and I3 with their verifiers ('Keep D22's five counts or ask the owner to amend ruling 44'; 'Option (b) bundles four separable decisions').

**Waits.** S7-H1 and S7-H2.

**Verifier.** partly. Holds: experiments/018-s7-outside-package/README.md:45-60, :86-105, :123-130, :152-170 and :195-216; IN_FORCE.md:246; docs/proposals/ruling-75/README.md:85-86. Both third-class entries quote the 2026-11-11 answers correctly (Q2 at :213 and Q3 at :214). Does not hold: two first-class items add work to an accepted run, which Q5 accepted (:216). 'A second subject server with tools of its own' and 'a task in which the author checks its own package' add tasks and author time; they are not tightenings. Also, 'D01's middle row ... defined' already exists as the admin's logged reading (docs/implementation/HANDOVER.md:272).

**Verifier's corrections.** Report the second server and the self-check task apart, or put them to the owner with the split. Cite the existing reading of 'adapter'.


## FR-514

**S8: the proposed changes in three classes**

G5 I13 · assessment · verifier: supported · decides: coordinator

S8 holds no E-case and no D-measure; D22 through M0 is its only round-6 row, and its filter questions use D05's method without claiming S2's D05.

First class (the coordinator's):
- the claim stated in two parts, the contract and the convention, in a labelled note (E03 I8);
- a ceiling for 'small' stated before the run, since 'small adapters that are counted' has no number;
- seeded leaks that the suite must fail (a hidden-only match returned, a count over hidden documents, a snippet from a hidden document);
- a hand-written expected table for the small corpus as the oracle (the verifier's alternative; the plan already asks for hand-reviewed cases);
- the result order fixed by a stated key in the contract that is still to be drafted;
- replacing A's Conduit with a narrower one, added beside the harness's change of permission data;
- the README and the plan reconciled on whether V1 to V3 wait for the contract;
- scripted exact-protocol checks beside the Agents' tasks, which the plan already has.

Second class (the owner's):
- a linear scan as the second implementation and as the oracle (E03 I8, A2): the accepted P3 has the oracle 'computed independently ... never from the implementation's answer', and Q2's answer names the filesystem and SQLite FTS; a scan the lane writes is not an existing tool (ruling 40).
- cache cases dropped, or ranking statistics stated as outside the contract (A3): P4 covers cached answers and has ranking use the visible corpus only.
- M0's time and mistakes dropped: the README marks that line ruled (ruling 46 point 3).
- the fresh author kept on T1 and T2 only: the accepted run.
- the contracts accepted by the coordinator instead of the owner: 'Owner-accepted concrete Requirement and search/storage Operation contracts', and the handover lists that acceptance among the owner's items.

Third class (asked and answered, nothing new):
- scripted clients in place of the real Agents (E03 I8 'Clients'; E03 I9 option a). Ruling 75 D2, 2026-11-11: 'S7 and S8 also use the second subscription accounts'; Q1's answer the same day; and on 2026-11-12 the owner agreed that 'S3, S7, S8 and S10 keep their real-client runs on the second accounts'.

Judgment: judge S8's pass conditions on the scripted checks the plan already holds, keep the real Agents' run as accepted, and use it to observe the one thing only a real client shows: whether an Agent refused by the filter reads the corpus with its own tools.

**Evidence.** experiments/019-s8-filtered-search/README.md:45-47 (P3), :48-52 (P4), :64-67, :104-105, :123 ('Ruled: observed time and mistakes precede any numeric target'), :125, :131, :150, :162, :178 ('Owner-accepted'), :213-217 (answers; :215 'behind small adapters that are counted'). experiments/019-s8-filtered-search/plan.md:117-122, :124-130. docs/implementation/HANDOVER.md, 'The owner's open items' (S8-A). docs/proposals/ruling-75/README.md:85-86, :117-120. First pass: E03 I7, I8, I9 with their verifiers ('The owner ruled the other way three times in two days'; 'A2 makes the linear scan both an implementation under test and the oracle').

**Waits.** S8-C and S8-H1, which the plan says can start now; S8-A.

**Verifier.** supported. experiments/019-s8-filtered-search/README.md:45-52 (an oracle 'computed independently ... never from the implementation's answer'), :123 (ruled), :178 ('Owner-accepted'), :213-217; docs/proposals/ruling-75/README.md:85-86, :117-120. The third class matches E03 I9's verifier ('The owner ruled the other way three times in two days'). The linear scan is correctly put in the second class, against P3 and ruling 40.

**Verifier's counterargument.** A ceiling for 'small' fixed before the run must be a stated expectation to report against. As a pass threshold it would add a condition to the owner's words 'small adapters that are counted' (:215).


## FR-515

**S9: the proposed changes in three classes, and one accepted number that the specification overrides**

G5 I14 · assessment · verifier: partly · decides: coordinator

S9 holds no E-case and no D-measure; D22 through M0 is its only round-6 row.

First class (the coordinator's):
- every recorded request names a caller that sent it (E03 I6, condition 2);
- a rubric for 'correct diagnoses' in M2;
- added measures: dispatches before the first actor request, by kind, with the provider calls the engine makes of its own accord excepted as the verifier corrected; requests from restart to ready; seconds from start to listening;
- the plan's stale account of what main holds.

One case apart. The accepted M2 counts hook attempts at restart as 'one in branch (a)'. That number was written to match the engine, whose `serve` sets instances up at each start; OBS-004 says no setup is dispatched without an explicit request, and IN_FORCE holds that nothing runs a background reconciler or wakes on its own. In form this reverses an accepted number, which would make it second class. In substance spec/ alone binds and a pre-registration binds nothing, so the coordinator fixes the engine as a build defect, corrects T4 branch (a), M2 and V4 by a dated amendment, and logs both for the owner to strike. The owner is asked only if the start-time setup is to be kept, which would amend rulings 15 and 17. Because the fix moves recovery steps, it follows the S1 rerun's pin.

Second class (the owner's):
- T1 and T5 carried by vectors only with no author run, T3's cuts run as engine tests outside the scenario, and T4 moved into S4 (E03 I6): P2, P3, P5 and the task list of ruling 75 D1; it re-centres S9 away from what the roadmap says it exercises, 'Root Interface and foundational setup'. T5's restart branch is also the only check that a closed declared Conduit is not reopened at start, which a core vector cannot show.
- T3's cuts taken out of S9: Q4's answer of 2026-11-11 names 'the documented initialization cuts' as S9's scope.

Third class: nothing from 2026-11-11 or 2026-11-12. E03 I4, put as a question, would ask what rulings 15 and 17 already hold.

Judgment: correct the one number, add the rubric, and leave S9's tasks as accepted.

**Evidence.** experiments/020-s9-start-and-repair/README.md:48-64 (P2 to P5), :68-69 ('no row of assigned E-cases or D-measures'), :114-118 (T4), :124 (T5), :148-151 (M2: 'one in branch (a); none in branch (b) before reconciliation'), :252 (Q2), :254 (Q4). spec/conventions/observation.yaml:55-67 (OBS-004: 'No setup, control, open or retire is dispatched because of it'). IN_FORCE.md:22 ('nothing runs a background reconciler or wakes on its own (15, 17; preservation test 6)'). ROADMAP.md:147. First pass: E03 I3, I4 and I6 with their verifiers (engine/crates/shell/src/main.rs:205-213 and :349-362 for the start-time setup; 'This is probably the coordinator's to settle'; 'Keep T5's restart branch in the run unless the startup setup goes').

**Waits.** The S1 rerun's pin for the engine fix; S9-H.

**Verifier.** partly. Holds: OBS-004 (spec/conventions/observation.yaml:55-68); IN_FORCE.md:22; experiments/020-s9-start-and-repair/README.md:114-118, :148-151 ('one in branch (a)'), :169; engine/crates/shell/src/main.rs:205-215 (setup at start) and :345-362 (registered instances, and ready ones with a runnable item). The routing holds: the coordinator corrects, logs it to strike, and asks the owner only if the setup at start is to stay.

**Verifier's counterargument.** DL16 leaves to the engine 'what becomes of the processes it started when the engine stops and when it starts again' (spec/implementation-defined.yaml:182-186), and OBS-004 itself sets provider restarts apart (:65-68). Only the setup dispatched at start for registered or failed instances, under root's identity, is plainly a defect. Restarting the run of a ready instance is an ambiguity (E03 I3's verifier). V4 is a core vector whose `restore-state` dispatches nothing, so the correction lands in T4a and M2. If the question is put at all, it goes under A17 (docs/AUDIT.md:455).

**Verifier's corrections.** Narrow the 'build defect' to the dispatch at start for registered or failed instances. Treat the ready-instance run as an ambiguity to state in spec/. Cite A17.


## FR-516

**S10: the proposed changes in three classes**

G5 I15 · assessment · verifier: partly · decides: coordinator

First class (the coordinator's):
- a predicted set of edits per swap, fixed before the run from a paper count, limited to S1's runtime and package-source swaps until S2's documents exist (the verifier's form of E03 I2 and I5; E05 I9 change 1 as a reported count): the accepted P4 already makes any unlisted edit a deviation;
- the class of each edited document added to D16's ledger, and `lift.v1` requests counted, since every replacement under a kept name now needs one;
- the implementations behind D15's 'all six' named, with the shared kit counted once, as D15 already requires (E05 I9 change 3; E03 I5);
- a sentence that the replays swap the built-in for the second provider and which pair the suites cover (P04 I13 part 1);
- an open ledger of behavior that differs across a swap, each line either equal or stated in the provider's contract (P04 I13 part 2, as a report; its verifier counts nine such differences today);
- a pass condition for a contract suite: every promise the contract cites has a case, and the suite fails seeded faults (E03 I10 change 1);
- stage R's conditions 2 to 6 as checks of D16's stated invariants.

Second class (the owner's):
- the allowed edits narrowed to the provider's own document, or to that and one runtime line per instance (E03 I2 options a and b; E05 I9's 'no edit to any other document' as a pass): D16's accepted list, including the conversion of an item 'before freezing the comparison' as provider preparation; ruling 44 (D16), ruling 75 D1, and for the first configuration ruling 46 point 5. The count against the packages as shipped is new; the item boundary itself was in round 6's D16.
- the packaged local providers as the first configuration (E05 I9 change 2; E03 I2 option a): the accepted table names the engine's own runner and fetching; the admin's decision (b) is on the list the owner may strike.
- the file store taken as the second store (E03 I5): it settles the 'or else' in Q2's answer of 2026-11-11, which the plan keeps for the owner.
- S1 dropped from the secrets-only configuration, E06 run once, and the package-source replays cut to an install, a digest comparison and one call (E03 I5): each against a sentence of the accepted run text.
- time and mistakes kept only for the fresh author's run: a line the README marks ruled (ruling 46 point 3).
- a count of engine code paths chosen by provider, to pass at zero (P04 I13): a new pass condition.

Third class (asked and answered, nothing new):
- replays cut from twelve to four, and single-kind swaps put on scripted clients (E03 I5, E03 I9 option a, P08 I16). Ruling 75 D1, 2026-11-11: 'twelve replays for S10 in place of twenty'; Q3's answer: 'accepted, with twelve replays in place of twenty (D1). The coordinator runs all five configurations; the fresh Codex author runs only the one with every provider swapped'; and on 2026-11-12 the owner agreed that S10 keeps its real-client runs on the second accounts. The owner cut this count once, the day before the reviewed commit.

Judgment: all five rows stay. The predicted edit set is what makes P4 able to fail, and it needs no ruling. The one point worth the owner's time is whether a rewritten item in a service's or an Agent's package counts as a failed swap or as preparation.

**Evidence.** experiments/021-s10-swapped-providers/README.md:49-51 (P4: 'any unlisted edit is a deviation'), :118-123 (D15: 'Count shared library work once and list which providers use it'), :133-140 (allowed edits; :138-140 the conversion before the freeze), :151-158, :182-186 (twelve replays; 'S1's no-secret replay is retained even in the secrets-only comparison'; 'E06 once per author'), :188-192, :117 ('Ruled: record time and mistakes before any numeric target'), :258-260 (Q2, Q3). experiments/021-s10-swapped-providers/plan.md:173-178 (decision (b)), and its 'For the admin' A4 on 'or else'. docs/implementation/HANDOVER.md:270. docs/proposals/ruling-75/README.md:84, :117-120. docs/proposals/reaudit/round-6/DECISIONS.md:120 (D16: 'the provider-specific edits and the replacements it allows'). First pass: E03 I2 and I5 with their verifiers ('Better alternative: keep ruling 44's D16 standard and make it able to fail without a new decision'; 'No S2 project exists'), E05 I9, P04 I13, P08 I16, E03 I10.

**Waits.** S10-H2a (the ledger tool), which the plan says can start now; the freeze (S10-H2b).

**Verifier.** partly. Holds: experiments/021-s10-swapped-providers/README.md:49-51, :117, :118-123, :133-158, :182-186, :188-192, :258-260; plan.md:173-181; docs/proposals/ruling-75/README.md:84. The third class (the owner's 'twelve replays in place of twenty') holds. Does not hold: (a) The first-class 'predicted set of edits ... the accepted P4 already makes any unlisted edit a deviation'. If the coordinator's predicted set becomes the list that P4 judges, it narrows D16's accepted categories (:133-137), which E05 I9's verifier calls the owner's. The item calls the set 'a reported count' and also 'what makes P4 able to fail', which cannot both be true. (b) The question kept for the owner, whether a rewritten item is a failed swap or preparation, is answered by accepted D16 (:138-140); only the count is new. (c) P04 I5, which puts the built-ins behind the same suite and reverses decision (b), touches D15 and P3 and is not placed.

**Verifier's corrections.** Keep the predicted set as a reported count beside D16, or send it to the owner as a narrowing. Drop the item-boundary question or say that only the count is new. Add P04 I5 to the D15 and E14 rows.


## FR-517

**Table, S1's rows: E05, E08, E11, E12, D08, D19**

G5 I16 · fact · verifier: supported · decides: none

Short forms: SC is conformance/s1-safety-cases.md; CR is conformance/README.md; FB is docs/implementation/FIRST_BUILD.md.

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| E05 | A presented tool's schema is a self-contained bundle | S1: ROADMAP.md:172; SC:12-28; 18 case directories named `e05-*` under conformance/interface/schemas | none in the items read: kept | yes: SC:21-28 |
| E08 | An agent may start only an approved reviewer, through a broker, with its trusted code and setup counted | S1: ROADMAP.md:172; SC:67; conformance/conventions/records/broker | none: kept | the vector's expectation yes (SC:67); the count of trusted code and setup no (CR:415-416; I6) |
| E11 | A template replaced while an old instance exists | S1: ROADMAP.md:172; SC:68; among the `records-*` vectors (CR:411-413) | none: kept | yes: SC:68 |
| E12 | A name reused before a follow-up request; a stale expected ID conflicts | S1: ROADMAP.md:172; SC:69; among the `records-*` vectors (CR:411-413) | none: kept | yes: SC:69 |
| D08 | The chapter on change and repair: T4 to T7, with requests and intervals without access | S1: ROADMAP.md:172; FB:138, :177-179 | P02 I8, P08 I13 and P03 I11 add columns to the S1 rerun's report: kept | yes: FB:144-158, :177-179; docs/proposals/first-build/increment-4/measurement-plan.md:60-63 |
| D19 | The bytes kept for each invocation | S1's core: ROADMAP.md:172; CR:422-424, :589-592; a dated sample at CR:629 | none in the items read: kept | yes: CR:589-592 |

All six have run, as vectors or in S1's runs, and no proposed change to S2 to S10 touches them. The comparison with a baseline in the same roadmap row is A70, done on 2026-11-09.

**Evidence.** ROADMAP.md:59, :172. conformance/s1-safety-cases.md:12-28, :67-69. conformance/README.md:411-416, :422-424, :589-592, :629 ('Measured D19 sample, 2026-11-09'), :698. docs/implementation/FIRST_BUILD.md:138 ('T4 to T7 are the chapter on change and repair of D08 of round 6'), :144-158, :177-179. `find conformance -type d -iname '*e05*'`: 18 under conformance/interface/schemas and 2 under conformance/language/registration/e1. `ls conformance/conventions/records`: broker, broker-filtered, expected-id, replace among 18 case directories.

**Verifier.** supported. conformance/s1-safety-cases.md:21-28 (E05) and :67-69 (E08, E11, E12); conformance/README.md:411-416, :422-424, :589-592, :629; docs/implementation/FIRST_BUILD.md:138, :144-158; ROADMAP.md:172.

**Verifier's corrections.** Minor: D08 is shown here with additions, yet listed as untouched in I2. P07 proposes a D19 projection under A14, outside the assigned items.


## FR-518

**Table, S2's rows: E02, E07, E13, E17, D01, D05, D09, D20**

G5 I17 · fact · verifier: partly · decides: none

Short forms: R is experiments/013-s2-shared-login/README.md; P is experiments/013-s2-shared-login/plan.md. Every drop or move is the owner's decision under ruling 44.

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| E02 | A read succeeds before a filter's deadline and is repeated after it: dispatched again, refused, no stored output | ROADMAP.md:173; R:55-56 (P4), R:112 (T3), R:120-122, V1 at R:158 | E01 I2: kept, an oracle row added. E01 I4: S3's replay of it shrinks with the real-client replay | yes: R:55-56, R:158; the harness fixes the clock (R:118) |
| E07 | An agent with a filtered read and a bound over the same operation opens an unfiltered Conduit, which the bound allows | ROADMAP.md:173; R:56-58 (P4), R:112, V2 at R:159 | E01 I2 item 5: kept, a prediction before the attempt added | the engine's decision yes (R:56-58, R:159); whether the limit is 'good enough' (ruling 44 point 2) no (I6) |
| E13 | The shared secret rotated with both agents working, the service replaced, restoring failing partway; no secret reaches an Agent's run | ROADMAP.md:173; R:59-62 (P5), R:114-115, V3 at R:160 | E01 I2 item 6: kept, a snapshot check added. E01 I11: the harness rotates, not the owner | yes: through D09, R:145 |
| E17 | What a real client and its model see after a revoke with cached tools, a protocol refusal and a tool failure | ROADMAP.md:173; R:63-64 (P6), R:113 (T4), M1 at R:147. Run early as A71 against a stub on 2026-11-09 (ROADMAP.md:60) | E01 I2 item 8: MOVED out of S2 to a conditional rerun of experiment 008. E02 I5 point 9: kept, one case added. The two conflict | in part: categories yes (R:147); trials per case and prompts no |
| D01 | A plain server, a small adapter and a Conduit-aware implementation qualified on one task, the adapter work counted | ROADMAP.md:173, also :177; R:143, R:111 (T2) | E01 I2 item 3: kept; a pass condition for the plain row; the adapter row rebuilt or DROPPED, leaving two rows. E01 I9 option b: the plain row becomes Linear's hosted server | in part: the count yes (R:143); the adapter's meaning is in P:110-124 and the plain server is unchosen (P:126-142) |
| D05 | The first restrictions as complete contracts with vectors shared by two implementations: changed authorization data, a repeat, onward calls, revocation | ROADMAP.md:173; R:144, R:111 | E01 I2 item 4: kept, one shared case file; the onward case implemented, or DROPPED with 'dependent calls' | in part: five assertions yes (R:144); the cases are not enumerated and no shared case file exists |
| D09 | Rotation by explicit replacement: timestamps, changed lines, requests, IDs, intervals without access | ROADMAP.md:173, also :178; R:145, R:114 | E01 I2 item 6: kept. E01 I9 option b would put the plain row on a second credential that D09 'on the keychain store' does not cover | yes: R:145 |
| D20 | What a local and an isolated deployment of the same relationships each protect, and which credentials an Agent process can read | ROADMAP.md:173, also :174; R:146, R:116 (T7), R:64-65 | E01 I2 item 7 and its runs: a list of attempts added; the local half reported to A7; the isolated half MOVED to S3 with a new pass condition; S2's two isolated runs DROPPED. Verifier of E04 I1: A5's matrix should reuse D20 | no: locations yes (R:146); no list of attempts (I5) |

S3 replays E02, E07, E13 and E17 as part of S2's workload (experiments/014-s3-two-machines/README.md:72, :113), and S10 repeats E13 with D09 on its second store (experiments/021-s10-swapped-providers/README.md:171). E17 is the only case of S2 to S10 with dated evidence, taken against a stub and not the engine.

**Evidence.** The cited lines of experiments/013-s2-shared-login/README.md, read in full. ROADMAP.md:60 (A71 'done on 2026-11-09 ... 68 runs ... 12 two-turn sessions'), :173. experiments/013-s2-shared-login/plan.md:110-124, :126-142. packages/scenario-s2-linear-restrictions/README.md:68-76. First pass: E01 I1 ('Not countable as written: D20 (no attempt procedure ...), M1 (trials per cell unstated)'), E01 I2, I9, I11; E02 I5 point 9; E01's verifier ('each crate has only its own tests').

**Verifier.** partly. Each row's lines in experiments/013-s2-shared-login/README.md (55-65, 111-123, 143-147, 158-160) and plan.md:110-142 read as cited. S3 replays the four cases (experiments/014-s3-two-machines/README.md:72, :113), and S10's V5 repeats E13 and D09 (experiments/021-s10-swapped-providers/README.md:171). The counting-rule column is weak for D05 and D20, as set out under I3.

**Verifier's corrections.** D05: 'yes at the level of five cases × two implementations; no shared case file'. D20: 'in part'.


## FR-519

**Table, S3's rows: E15 and D20 under a partition**

G5 I18 · fact · verifier: partly · decides: none

Short forms: R is experiments/014-s3-two-machines/README.md; P is experiments/014-s3-two-machines/plan.md. Every drop is the owner's decision under ruling 44.

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| E15 | A machine reported unreachable, then gone, with cuts before and after an effect and before and after a revocation; unavailable, unknown, retired and stopped stay distinct | ROADMAP.md:174; R:51-54 (P3), R:55-59 (P4), R:114-116 (T4 to T6), R:126-134 (the four cells and the loss) | E01 I4: the effect axis kept as three orderings; the revocation axis DROPPED but for one check after the heal; the loss kept once, with a live run. P05 I3: kept; a loss during a cut and a live-unit rule added. E01 I8 and P05 I2: kept; the fault method changes. P08 I16: kept; runs from four to two. E05 I7: kept; placed after the model re-checks. P05 I10: kept; a disposable node's cells reported apart | yes: M1 at R:155, with V1 and V2 at R:167-168. But the cells cannot be prepared as accepted (R:123-125) since ruling 77, and S3-OF is open (P:259-298) |
| D20, under a partition | S2's D20 repeated before, during and after each cut and after loss and return, with every interval without access | ROADMAP.md:174; R:64-66 (P6), R:154 | E01 I4: DROPPED from the cells, kept on the second machine before the loss and after the return. P08 I16: the isolated profile on the second machine goes to v1, which empties P6's comparison. E01 I2 item 7: S2's isolated half MOVED in | no: it inherits S2's D20, which has no list of attempts, and does not say who attempts on the second machine during a cut (I5) |

**Evidence.** The cited lines of experiments/014-s3-two-machines/README.md, read in full. experiments/014-s3-two-machines/plan.md:259-298. docs/proposals/ruling-77/README.md:69-73 ('With interop off the coordinator can no longer do either from inside the distro, so S3's cut and loss need the owner's hand at a barrier or another means, which S3's plan settles before its runs'). ROADMAP.md:174. First pass: E01 I3, I4, I8; P05 I2, I3, I10; E05 I7; P08 I16 with its verifier.

**Verifier.** partly. The E15 row holds: experiments/014-s3-two-machines/README.md:51-59, :114-116, :126-134, :155, :167-168; plan.md:259-298; docs/proposals/ruling-77/README.md:69-73. D20's moves are stated correctly (E01 I4; P08 I16 and its verifier). The 'no' rating for D20 has the same inconsistency as in I3.

**Verifier's corrections.** Rate D20 under a partition 'in part', and add that S10's Docker replays could carry D20 if the isolated profile leaves S3.


## FR-520

**Table, S4's rows: E01, E09, E18, E19, E20, D02, D10**

G5 I19 · fact · verifier: supported · decides: none

Short forms: R is experiments/015-s4-partial-setup/README.md; P is experiments/015-s4-partial-setup/plan.md. Every drop or move is the owner's decision under ruling 44.

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| E01 | A plain server makes part of an effect, then answers -32021, directly and as a failed task: unknown, and a repeat with the same key is not dispatched | ROADMAP.md:175; R:97-100, R:110-111 (T2), V2 at R:162 | none: kept. The verifier on E01 I14 raises its fixture (a plain server's operation marked as starting model work) as a finding | yes: R:110-111, R:162, counted in M2 (R:142-146) |
| E09 | A client that keeps its key and one that sends none, each repeating after a lost response | ROADMAP.md:175; the client axis of M1 at R:136-141; the one real replay at R:191-192 and R:246-248; P:101 | E01 I6 item 3: the real-client half MOVED to experiment 008's stub on the primary accounts, the replay DROPPED. E03 I6: a restart case added to that replay. The two conflict | the scripted cells yes (R:136-141). The real replay: what is recorded yes (R:195-196), one trial (R:247-248), no pass condition |
| E18 | The owning Pool retired, the Resource inspected and reconciled, the instance removed and reconciled again; each step as ACC-033's table says | ROADMAP.md:175; R:116-118 (T5), R:62-64 (P5), V5 at R:165 | E01 I6 item 4: the lifecycle model asked first; the boundary-fixture half DROPPED if the state cannot be reached | yes: R:62-64, R:165, with Q4's answer at R:249; gap G2 (R:214-217) says the state has no demonstrated public preparation |
| E19 | Records purged and outputs expired, an old keyed request repeated, an unknown invocation inspected; no effect repeated | ROADMAP.md:175; R:114-115 (T4), R:62 (P5), V4 at R:164; named only at R:69 | none: kept | yes |
| E20 | A hook's reply lost, reconciled failed, a new attempt for another caller, then the earlier attempt's late onward request, decided for the latest caller | ROADMAP.md:175; R:63-64, R:119-120 (T6), M3 at R:147-150, V6 at R:166 | E01 I6 item 5: kept, as an observation of a stated limit | yes: R:147-150 |
| D02 | The matrix of two kinds of client, two kinds of implementation and three situations: twelve cells | ROADMAP.md:175; R:59-61 (P4), R:112-113 (T3), M1 at R:136-141, V3 at R:163 | E01 I6 item 1: kept; a twelve-row oracle added; the cells run once and not per author, a cut of 'twelve per author run' | yes as a count (R:136-141); the expected calls and effects per cell are not tabulated |
| D10, S4's half | What the actor can conclude, verify and do next after an unknown outcome, a conflict and a partial setup | ROADMAP.md:175; R:51-55 (P2), M2 at R:142-146 | E01 I6 item 2: kept; the fresh author is the test; a rubric and a blind verifier added; the coordinator's run not counted. P03 I7: the plan's counted repair of T1 cannot be dispatched | in part: rows and fields yes (R:142-146); no rubric or judge for 'correct'; T1's repair rule is in P:206-216, outside the accepted text |

**Evidence.** The cited lines of experiments/015-s4-partial-setup/README.md, read in full. experiments/015-s4-partial-setup/plan.md:101, :196-216. ROADMAP.md:175. `grep -n -o -w E19` over the nine READMEs: experiments/015-s4-partial-setup/README.md:69 only. First pass: E01 I5, I6, I14 with their verifiers; E03 I6; P03 I7.

**Verifier.** supported. experiments/015-s4-partial-setup/README.md:59-64, :69 (E19's only mention; `grep -n -o -w E19` over the nine READMEs), :97-120, :136-150, :162-166, :191-196, :214-217, :246-249; plan.md:101, :206-216. The E09 conflict between E01 I6 and E03 I6 holds.


## FR-521

**Table, S5's rows: E03, E04, E10, E16 and D10**

G5 I20 · fact · verifier: supported · decides: none

Short form: R is experiments/016-s5-control-contention/README.md. Every move is the owner's decision under ruling 44.

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| E03, E04 | A control whose `changes` gives two inputs one setting, or one a setting inside another's; the control is rejected | ROADMAP.md:176; R:56 (P4), R:102 (T3), M2 at R:137, V1 at R:151 | E02 I7 point 2: MOVED out of the author's run, carried by V1 alone | yes: R:56, R:137, R:151 |
| E10 | A control narrowed by its template to 50 under a contract that allows 100, asked for 75, then left out of the template: the narrowed schema is shown and no tool is presented for the excluded control | ROADMAP.md:176; R:57-58, R:103-104, M2 at R:137, V3 at R:153 | the same: MOVED to V3 alone | yes |
| E16 | Seven cases of omitted and default inputs; keyed repeats compare one normalized value; a default reached only through a root `$ref` fills nothing | ROADMAP.md:176; R:58-59, R:105-107 (T4), M2 at R:137-138, V3 at R:153, R:177-178 | the same: MOVED to V3 alone | yes |
| D10, S5's half | What `ruleId`, the invocation's identity and its status let a frontend tell apart in a conflict: five classes | ROADMAP.md:176; R:60-64 (P5), M1 at R:129-134, T6 at R:112-113 | E02 I7 point 5: kept; the next step per class registered; five of five by the fresh author. Its point 1 leaves T6 out of the author's tasks. E02 I8: one author | in part: the classes and their rule IDs yes (R:129-131); the right next step and the judge no |

The split of D10 between S4 and S5 is round 6's own placement, and each half has its own measure.

**Evidence.** The cited lines of experiments/016-s5-control-contention/README.md. ROADMAP.md:175-176. docs/proposals/reaudit/round-6/DECISIONS.md:114 ('D10 ... Under S4 and S5'). First pass: E02 I6, I7, I8 with their verifiers ('E03, E04, E10 and E16 are round 6's evidence cases assigned to S5; meeting them by vectors alone is reasonable but changes that row').

**Verifier.** supported. experiments/016-s5-control-contention/README.md:56-64, :102-113, :129-138, :151-153, :177-178; ROADMAP.md:176; docs/proposals/reaudit/round-6/DECISIONS.md:114.


## FR-522

**Table, S7's row: D01**

G5 I21 · fact · verifier: partly · decides: none

Short form: R is experiments/018-s7-outside-package/README.md. The drop is the owner's decision under ruling 44.

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| D01, as in S2 | The outside author's packages qualified as plain, adapter and Conduit-aware variants on one fixture task, with the cost of each | ROADMAP.md:177; R:65, R:90-91 (T3), M1 at R:123-126 | E02 I2: DROPPED from S7 ('S2 owns it') | in part: defined by reference to S2's D01 (R:123-126), which is now accepted; no task of S7 has the author write an adapter (R:86-99), so the middle row is undefined |

S7 holds none of E01 to E20 (R:64-65). Round 6's row also gives S7 a real package whose server repeats a named argument (P4 at R:49-52, T5 at R:94-95, V2 at R:140). It is not one of the 31 rows, but E02 I2 would make it an observation with no pass condition and choose another subject server, and an open point in IN_FORCE waits for it.

**Evidence.** experiments/018-s7-outside-package/README.md:49-52, :64-65, :86-99, :123-126, :140. ROADMAP.md:177 ('None of E01 to E20. Round 6's decisions add a real package whose server repeats a named argument ... D01, as in S2 (A63)'). IN_FORCE.md:246. experiments/013-s2-shared-login/plan.md:114-115 ('S7's README reads an adapter as new code around unchanged upstream code'). First pass: E02 I1 ('M1 is D01 ... its three rows fit S2's real service and are contrived here'), E02 I2 and its verifier.

**Verifier.** partly. Holds: experiments/018-s7-outside-package/README.md:49-52, :64-65, :123-126, :140; ROADMAP.md:177; IN_FORCE.md:246. Does not hold: 'no task of S7 has the author write an adapter ... so the middle row is undefined'. T3 qualifies the adapter variant (:90-91), M2 counts 'new adapter code' apart (:129-130), and the admin's logged reading defines S7's adapter as code wrapping upstream (experiments/013-s2-shared-login/plan.md:114-121; docs/implementation/HANDOVER.md:272). The item cites that plan line itself.

**Verifier's corrections.** Say 'under-specified in the accepted text; defined by a logged admin decision; no task says to write it'.


## FR-523

**Table, S10's rows: E06, E14, D15, D16 and D09 on the swapped store**

G5 I22 · fact · verifier: partly · decides: none

Short forms: R is experiments/021-s10-swapped-providers/README.md; P is experiments/021-s10-swapped-providers/plan.md. No proposal drops one of these rows outright; the cuts are to repetitions and to the allowed edits.

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| E06 | Two package-source providers that would claim one scheme: a Type with a control changing the schemes is rejected and a second claimant is refused | ROADMAP.md:178; R:52-55 (P5), R:91-92 (T6), V4 at R:170, R:185 | E03 I5: kept, run once and not once per author. P08 I16: kept, once per author | yes: R:52-55 |
| E14 | S1 and S2 with each provider swapped, every provider-specific edit listed | ROADMAP.md:178; named at R:59; R:81-90 (T1 to T5), R:49-51 (P4), R:129-158 | E03 I2: kept, the allowed edits narrowed. E03 I5 and I9: kept; single-kind swaps on scripted clients, real Agents in the all-swapped replays only (twelve to four), S1 out of the secrets-only configuration, the package-source replays cut. E05 I9: kept; a paper ledger first; the packaged local providers as the first configuration. P04 I13: kept; a ledger of behavior differences. P08 I16: kept for the all-swapped replays only, the single swaps removed | yes: R:129-158 |
| D15 | For each provider: what it implements itself, what it delegates, what state it keeps, what the operator configures | ROADMAP.md:178; R:118-123 | E05 I9 change 3 and E03 I5: kept; the kit reported as shared; the set of implementations named. Verifier of P05 I11: its operator-setup field extended to S3's machine provider | in part: the four fields yes (R:118-123); 'all six implementations' names no set since the suites run on the packaged local providers and the replays on the built-ins (P:173-178) |
| D16 | The comparison defined before the run: the edits and replacements a swap allows, and what it keeps | ROADMAP.md:178; R:124, R:129-158 | E03 I2 options a and b: the allowed list cut; its verifier: a predicted set fixed by the coordinator. E03 I5: the document's class added. P04 I13: a behavior ledger, and a measure to pass at zero. E05 I9: 'no edit to any other document' as a pass | yes: R:151-158. Missing: a predicted set of edits, and `lift.v1` requests, which the ledger predates |
| D09, on the swapped store | S2's rotation count repeated on the second store | ROADMAP.md:178; M3 at R:125-127, R:83-84, R:89, V5 at R:171 | E03 I5: kept; the file store taken as the second store. P08 I16: kept in the all-swapped replays only | yes: by reference to experiments/013-s2-shared-login/README.md:145 |

**Evidence.** The cited lines of experiments/021-s10-swapped-providers/README.md. experiments/021-s10-swapped-providers/plan.md:173-178 ('A63-H drives no built-in: the engine's runner and file fetching stay S10's first configuration in the replays'). ROADMAP.md:178. `cat experiments/022-provider-contracts/suites/*/index.yaml`: four files, each `cases: []`. `grep -n -o -w E14` over the nine READMEs: experiments/021-s10-swapped-providers/README.md:59 only. First pass: E03 I1, I2, I5, I9, I11, I14; E05 I9; P04 I13; P08 I16, with their verifiers ('Lifts after order 042 are in neither S10's README nor its ledger').

**Verifier.** partly. Holds: experiments/021-s10-swapped-providers/README.md:49-55, :59, :81-92, :118-158, :170-171, :185; plan.md:173-178. Each of the four suite indexes under experiments/022-provider-contracts/suites holds `cases: []`. Does not hold: D15's 'all six implementations names no set'. The accepted table names six: keychain, 1Password or the file store, the engine's runner, Docker, the engine's fetching, and Git (:188-192). The ambiguity comes from plan decision (b), under which the suites run on the packaged doubles (plan.md:173-178). P04 I5, which reverses (b), touches D15 and is missing from the row.

**Verifier's corrections.** Say: 'six named in the accepted table; decision (b) splits the first implementations between the suites and the replays'. Add P04 I5.


## FR-524

**Table, D22 on every run; S6, S8 and S9 hold no other round-6 row**

G5 I23 · fact · verifier: partly · decides: none

| Row | What it is | Held now | Proposed changes | Counting rule fixed in accepted text |
| --- | --- | --- | --- | --- |
| D22 | The author measurements on every run: files edited, concepts met, grants made by hand, adapters written, recovery steps that need a person | ROADMAP.md:166-168; docs/proposals/ruling-44/README.md:22-24; IN_FORCE.md:271, :274. Carried into each scenario by its M0: experiments/013-s2-shared-login/README.md:127-137, 014-s3-two-machines/README.md:138-148, 015-s4-partial-setup/README.md:124-134, 016-s5-control-contention/README.md:117-127, 017-s6-revoke-in-flight/README.md:125-135, 018-s7-outside-package/README.md:109-122, 019-s8-filtered-search/README.md:112-124, 020-s9-start-and-repair/README.md:131-141, 021-s10-swapped-providers/README.md:105-117 | E02 I2: 'concepts met' DROPPED from S7. E02 I5 point 8 and E02 I7 point 7: M0 reduced to time, mistakes and interventions in S6 and S5, so all five DROPPED there. E03 I8 and E03 I5 drop or narrow 'time and mistakes' in S8 and S10, which is ruling 46 point 3 and not D22. Cuts of author runs lower the number of runs that record it. P02 I8 and P08 I13 add it to the S1 rerun's report | yes: docs/implementation/FIRST_BUILD.md:148-157; docs/proposals/first-build/increment-4/measurement-plan.md:46-55 |

Each drop is the owner's decision under ruling 44 point 1.

Two facts beside the row:
- The rule was fixed for S1 and not fully applied. Neither report of S1's two runs of 2026-11-10 contains the word 'concept', although the measurement plan defines 'Concepts met' at its line 48. Files edited, grants, adapters and recovery steps are reported.
- S6, S8 and S9 have no E-case and no D-measure. Their READMEs say so, and D22 through M0 is their only round-6 row.

A small disagreement between records, for the coordinator: ruling 44's record says the D-measures are 'measurements of S1, S2 and S10', while round 6's decisions, accepted with that ruling, place D10 under S4 and S5 and D20 under S2 and S3, as the roadmap's table does.

**Evidence.** ROADMAP.md:166-168 ('Every run also records D22's author measurements: the files edited, the concepts met, the grants made by hand, the adapters written and the steps of recovery that need a person'). docs/proposals/ruling-44/README.md:22-24, :66-68. IN_FORCE.md:271, :274. docs/implementation/FIRST_BUILD.md:148-157, :160 ('The first five measures include D22's'). `grep -c -i concept` on docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md and fresh-author-run-2026-11-10.md: 0 and 0; `grep -n` on the same files finds 'Files edited', 'Grants made by hand', 'Adapters written' and 'Recovery steps that need a person'. experiments/017-s6-revoke-in-flight/README.md:68-69; experiments/019-s8-filtered-search/README.md:64-65; experiments/020-s9-start-and-repair/README.md:68-69. docs/proposals/reaudit/round-6/DECISIONS.md:114, :124. First pass: E02's verifier ('Three proposed changes drop D22's ruled author counts without saying so'); P02 ('Concepts met ... appears in neither report').

**Verifier.** partly. Holds: ROADMAP.md:166-168; docs/proposals/ruling-44/README.md:22-24, :66-68; IN_FORCE.md:271, :274; docs/implementation/FIRST_BUILD.md:148-157; docs/proposals/first-build/increment-4/measurement-plan.md:46-55; the M0 lines of all nine READMEs; and the disagreement between ruling 44 (:66-68, 'S1, S2 and S10') and DECISIONS.md:114, :124. `grep -c -i concept` gives 0 on both reports. Does not hold: 'Files edited, grants, adapters and recovery steps are reported'. `grep -c -i grant` gives 0 on docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md and on its fresh-author-run/report.json. Only the coordinator's report has 'Grants made by hand' (coordinator-run-2026-11-10.md:41). The fresh author's run reported three of D22's five counts.

**Verifier's corrections.** The fresh author's run lacks both 'concepts met' and 'grants made by hand'. This strengthens the item's point that D22 was not fully applied in S1.


## What the verifier says the analyst missed

- **Other round-6 triggers have no measure, not only ruling 44 point 2's broker** D09 sends A67's restart verb to the owner 'only if that is disproportionately disruptive' (docs/proposals/reaudit/round-6/DECISIONS.md:113), and S2's D09 rule (experiments/013-s2-shared-login/README.md:145) defines no comparison for it. D20 asks 'whether a user sees [the trust limits] at the point of choosing a runtime' (DECISIONS.md:124), which no counting rule measures. Neither can fire, for the same reason as I6's broker trigger.
- **The fresh author's S1 run also lacks 'grants made by hand'** `grep -c -i grant` gives 0 on fresh-author-run-2026-11-10.md and fresh-author-run/report.json, so two of D22's five counts were not reported for the fresh author, not one.
- **S10 already runs S2's relationships on the isolated runtime** S10's T3 and T5 replay S1 and S2 on Docker (experiments/021-s10-swapped-providers/README.md:85-86, :89-90), the same runtime as S2's isolated profile (experiments/013-s2-shared-login/README.md:182-184). D20's isolated half can ride there at no extra run. That answers the compound drop in I2 and is the place E01 I2's verifier named.
- **Adapter work stays counted outside D01** S8 counts adapters around existing tools (experiments/019-s8-filtered-search/README.md:215) and D22 counts adapters written on every run (ROADMAP.md:166-168). The D01 compound drop loses the three-row qualification on one task, not the counting of adapter work.
- **P04 I5 touches D15 and S10's P3** P04 I5 puts the built-ins behind the same suite, reversing decision (b) (docs/implementation/HANDOVER.md:270). It changes which implementations D15 and P3 cover, and it appears in neither I15 nor I22.
- **Ruling 45 point 4 and experiment 023's framing for paper counts** Ruling 45 dropped 'an author's effort counted on paper' (docs/proposals/ruling-45/README.md:76-78). Experiment 023 runs only as a design input, 'never the bar'. I6's E08 count must take that framing.
- **A78 as the real test of narrow provisioning** A78's orders 4 and 5 have a coordinator Agent drive workers, yet the design uses 'fixed worker and tree slots set up once by root' (ROADMAP.md:71), which avoids narrow provisioning. That is evidence for the owner's 'clumsy' judgment under ruling 44 point 2.
- **A fourth class: additions** Several items classed as tightenings add cells, tasks or prerequisites to an accepted run: S3's three orderings and the T6b phase (with owner actions under option A), S7's second server and self-check task, and S5's plan-and-apply branch, which needs batch 2. They change no pass condition, but they do change the run matrices accepted under ruling 75 D1. They should be reported apart, or each conditioned on what removes its cost.
- **E01 I3's validity argument was never answered** The case for dropping E15's revocation axis and D20 in S3's cells is that both are decided at M1 by the only enforcement point (experiments/014-s3-two-machines/README.md:131-132; ACC-027). The analyst rebuts only the cost argument.
- **The strike list must carry the amendments** The review list stops at item 29 (docs/implementation/HANDOVER.md:252). The coordinator's amendments in I4 need rows there, or the owner cannot strike what the owner never sees.

## The verifier's notes

Overall: the analyst's facts are mostly right and the citations hold. I reopened every scenario README's measures and answers, ruling 44, 45, 72 and 75 and ruling 42's later answer, IN_FORCE topics 1, 6 and 13, the round-6 table, and the S1 run reports.

Weaker points:
- The headline count of 21 rows with a fixed rule does not match the analyst's own tables, which give 23.
- The counting-rule ratings use two standards: D20 is rated 'no' while D05 and S3's M1 get 'in part' or 'yes' for comparable gaps.
- E14 belongs among the rows dropped in part.
- Both compound drops are overstated. S10's Docker replays and S8's counted adapters remain.
- The third class over-reaches in S3 and on E18. The 2026-11-12 answer fixed which scenarios keep real-client runs, not how many, and E18's drop waits on a result that does not exist yet.
- The first class absorbs several additions that change accepted run matrices.
- The core judgment, that the first pass's cuts are the owner's and that most savings come from writing counting rules, is sound. But it does not engage E01 I3's argument that some cells cannot fail.

Recommendation I4 is right in substance, provided each amendment is logged on the owner's strike list.

Read in full: the nine READMEs' sections on what each must show, measures, runs and answers. Read in part: the S2, S4, S6, S10 and S8 plans at the cited lines; first-pass files E01, E02 and E03 almost whole, plus E05 I9, P04 I5 and I13, P05 I2 to I4 and I10 and I11, P08 I2 and I16, P03 I7, the E04 I1 verifier and the critic; spec OBS-004 and DL16; engine main.rs at the cited lines.

Not checked: the to-review list's contents beyond its row count; P02 I8 and P03 I11 in detail; the design critique's text for E01 to E20.
