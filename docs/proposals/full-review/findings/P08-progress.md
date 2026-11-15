# P08-progress: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** At 2eb8f258 (2026-11-12) the project is two weeks old and has real assets: an engine that ran S1 end to end with real Claude Code and Codex Agents, 519 promises, 494 vectors, and a verified work-order pipeline that merged 116 orders in four days. Against its own plan it is wide and shallow: roadmap steps 3 to 6 all have work in flight, one of ruling 47's six required rows is met (A8), and the step everything else waits on, the S1 rerun and the bar, still has 12 planned orders unwritten (18 by the admin's split). The last two days went mostly elsewhere: 48 of 84 merged orders were lane orders for scenarios and providers the engine cannot yet call, and on 2026-11-12 one merged order of 26 was on the rerun path, while the owner's usage ran low enough to force the handover. Scope grew after the bar failed: 30 rulings and 64 promises in three days, none removed, and the gate itself gained conditions. The bar as ruled passes on paper with no margin (T1, T2, T4 and T5 must all be fewer in both runs, T5 by a single line), so a fail by a fresh author on a two- or three-line task is more likely than not and the rerun must report enough for the owner to judge in one pass. Beyond the pin the plans name 41 engine orders, 46 lane orders, 13 owner steps and 33 counted runs, with batch 2 unplanned; the repository supports no date for v0, and the one estimate on record (three to six weeks, ruling 71) predates those counts. I would change three things: one lane until the pin, with the author's surface (translator, plan and apply) written first; sign-in taken off the pin's path; and a gate trimmed to what the layered claim names, with S2 as the next tracer in series and Fleet authority, carried logins, peer engines and metrics after it. Two questions go to the owner (I2, I3); everything else is the coordinator's.

**What the analyst read.** Read in full: docs/guides/full-review.md, docs/implementation/HANDOVER.md, AGENTS.md, ROADMAP.md, docs/AUDIT.md, docs/implementation/INDEPENDENT_BUILDS.md, the records of rulings 47, 70, 71, 72 and 75, the first-build log of increment 4 and its measurement plan, the plans of S2 and S3 (experiments/013 and 014 plan.md). Read in part: docs/proposals/spec-batch-1/README.md lines 1-190 and 395-684 (not 191-394, the order of the spec edits); IN_FORCE.md topics 13 to 16 in full and its spec-pending lines by grep; the ledger by row count, dates and rows 44-77; ruling 61's decision table; docs/implementation/PROVIDERS.md lines 1-180 and 334-378; work order 117 lines 56-254 (the admin's split and size classes); FIRST_BUILD.md lines 28-123; the order tables, run rows and owner sections of the plans of S4 to S10, and the run and owner-time passages of their READMEs; the first lines of the Records of orders 037-082 and the timing lines of 100-150; WORK_ORDERS.md's record table; G5.md's counts and 'What G5 still needs'; experiment 023's counts and forecast; engine/CONFORMANCE.md by grep and three passages. Counted, not read: promises (IDs, layers, per commit), the manifest's coverage (PyYAML), Rust lines per directory, conformance case sizes, commits, pull requests and merged orders per day from git log, diff statistics of four unmerged branches. Not read: the promise texts (except ACC-057), the engine's code (two greps), conformance cases, the scenario READMEs beyond the cited passages, rulings other than those named, the review list's items, the model-checking code, the branches' contents, and the private handoff note, which is outside the repository. Nothing was built or run; origin/main as this worktree sees it is the reviewed commit, so nothing after 2026-11-12 14:50 -0800 is covered. The time arithmetic in I5 uses the planners' size classes and is not a measurement.


## FR-360

**The step everything waits on is starved: on the last full day one merged order of 26 was on the rerun path**

P08 I1 · assessment · verifier: partly · decides: coordinator

What works: 116 work orders merged in four days, each checked by a fresh verifier; S1 ran end to end with real Claude Code and Codex Agents and its failure of the bar was recorded as a failure; model checking found a real fault (MC2-001, a keyed effect applied twice) and the text was fixed within two days.

What does not: the work is wide, not deep. Roadmap steps 3, 4, 5 and 6 all have work in flight, and nothing has closed since step 2 was called done 'as the coordinator's note'. 32 P0 and P1 action items are still open, in progress or ruled; four items have been completed since ruling 39, one of them a build item (A43). Of ruling 47's six required rows one is met (A8), one partly (model checking), four not (I6).

Step 3 is the only step the roadmap puts in series before the rest. Its rerun path has 12 planned orders unwritten (18 by the admin's split, I5). On 2026-11-11 six of its build orders merged; on 2026-11-12 one did (042), beside 22 lane orders and three orders that came out of model checking. The lane's output is for scenarios whose engine parts do not exist (I9). The handover itself happened because the owner's usage was near its limit.

What I would change: one lane until the pin (I4), sign-in off the pin's path (I3), and a gate trimmed to the claim (I2).

**Evidence.** Merged orders per day, parsed from `git log --first-parent --reverse --format='%cd|%s' --date=short` (last commit naming each order; 049a1, 050a, 071, 076, 078, 079, 080, 144, 147 excluded as unmerged on main): 11-09 18; 11-10 14; 11-11 58 (lane 100-126: 26; rerun-path build 039, 040, 041a, 041b, 044, 045); 11-12 26 (lane 127-143, 145, 146, 148-150: 22; admin 042, 070, 077, 081). docs/implementation/HANDOVER.md:217 'Batch 1's rest, on the rerun path | Not written: 046, 047, 048a, 048b, 050, 053; joining them, 043a, 043b, 052a, 052b, 051a, 051b'. ROADMAP.md:25 'done as a step, as the coordinator's note ... Its items are not done'; ROADMAP.md:27 'Increment 4 remains in progress: both author runs failed the bar'. docs/AUDIT.md:440-456 and 462-476 (17 P0 and 15 P1 rows, each in progress, ruled or open); docs/AUDIT.md:505-508 (A72, A69, A43, A21 done). HANDOVER.md:4-5 'because the owner's usage was near its limit'; docs/implementation/PROVIDERS.md:36 'at the admin's word, for the owner's usage limits'.

**Waits.** I2, I3 and I4 follow from it.

**Verifier.** partly. Recounted from `git log --first-parent --reverse --format='%cd|%h|%s' --date=short`, last commit naming each order, same nine exclusions: 11-09 18, 11-10 14, 11-11 58 (26 lane), 11-12 26 (22 lane; admin 042, 070, 077, 081); 116 in all. docs/AUDIT.md:438-456 and 462-476 hold 17 P0 and 15 P1 rows, none done. docs/implementation/HANDOVER.md:217 and :4-5 read as quoted. What does not hold: (a) 070 is the head of the admin's batch-1 chain (docs/work-orders/117-plan-s2-s3.md:76; experiments/013-s2-shared-login/plan.md:286 '070 -> S0b -> 042/046a') and merged on 11-12 at 06:15 (1a34ccf9), so two chain orders merged that day, not one; (b) 070 came from finding 4-18, not from model checking (its subject line); only 077 and 081 did; (c) 11-12 was not a full day: the last commit is 14:50 -0800 (2eb8f258); (d) docs/AUDIT.md:425-430 says experiments and probes are excluded from 'Done since ruling 39'; ROADMAP.md:59-61, :66, :72, :75, :77 show A70, A71, A4, A8, A79, experiment 012's harness and experiment 023 done since 11-09.

**Verifier's counterargument.** The rerun path was starved, but the evidence points at the admin's own queue more than at the lane. On 11-12 the admin ran 049a1 rounds 3 to 5 (branch commits 05:23, 07:14, 09:39), 079 (00:24) and 080 rounds 1 and 2 (12:38, 13:25), all for S3 on s3/m2 and all engine-building orders; `git log HEAD..origin/wo/049a1-build-enrollment-core` and the 080 branch show it. The 22 lane orders of 11-12 took about 14,500 s of first-round worker time in all (summed from the Records of 127 to 150), close to 042's single first turn of 11,966 s (HANDOVER.md:138). Order counts therefore overstate the lane's share of the allowance. The sharper fact is that 043a's prerequisites were all merged by 11-11 10:12 (5ed438b2) and 043b's by 18:06 (1091a71b), and neither was written in the 28 hours to the handover. The core chain is serial, so one or two chain merges a day is its throughput, not proof of diversion.

**Verifier's corrections.** 'One merged order of 26 on the rerun path' should read two (042 and 070) by the split the analyst uses for its count of 18. 'Three orders that came out of model checking' should read two (077, 081); 070 answers finding 4-18. 'Four items completed since ruling 39' is the tracker's table only; seven register rows were also completed. Add: 043a dispatchable since 11-11 10:12, unwritten; the admin's off-path s3/m2 orders (049a1, 079, 080) ran on 11-12 beside 042.


## FR-361

**What must v0's gate hold, and what comes first after the bar?**

P08 I2 · question-for-owner · verifier: partly · decides: owner

In plain words: as it stands, v0 must pass ten scenarios with every pre-registered run, Fleet authority in full (batch 2), second accounts signed in through Conduit on two machines, and metrics built right after batch 2. The plans put that at 41 engine orders after the pin, plus batch 2, which has no plan and no branch. Which of it does v0 need to show the layered claim and the purpose, and in what order?

(a) As ruled. Batch 2 in full, then metrics, then S2 to S10 fanned out, each with all its runs.

(b) Acceptance first, gate trimmed to the claim. After the bar:
1. From batch 2 build only ruling 61 B13 (an Agent that others can drive holds no secret), which S2, S3 and S10 need. The rest of Fleet authority (61 B2 to B11: Fleet-held Conduits, forwarding, appointment, make.v1, review, keep, withhold) and peer engines move to v1.
2. Then S2 end to end with real Agents, in series, before any other scenario's engine orders: S1 and S2 are the acceptance scenario, and the engine's hot files allow no parallel merges anyway.
3. Then S4, S5, S6, S9 (scripted), S7, S8, then S3 and S10 in reduced form: S3 keeps partition and loss on the two real machines with its local profile (2 runs, not 4); carrying a browser login across machines (S3-RET1, S3-RET2, A81, vector V6) moves to v1, and on the second machine Codex signs in by device code or by hand over SSH; S10 runs the all-swapped configuration by both authors (4 replays, not 12).
4. Metrics after the gate, not right after batch 2.

(c) One machine. As (b), and S3 with enrollment moves out of v0.

Recommendation: (b). Nothing it defers is named by the layered claim or a preservation test, and none of it is connection or access in ruling 40's sense except Fleet authority, which no scenario S1 to S10 exercises (the plans of S4 to S9 each say batch 2 is not a prerequisite). Delegation that only narrows stays, through bounds (ACC-029, ACC-030). It keeps 'across several machines' and 'two implementations per provider contract' in the gate. Not (c): S10's pre-registration says the machines contract's second implementation is S3's evidence, so (c) would break the provider claim. Under the scope guard each deferral is the owner's to rule (A18).

What it costs: A78's orders 4 and 5 are dispatched by root, not by an appointed coordinator Agent; S3 and S10 show less (no isolated profile on the second machine, no single-swap replays); the pre-registrations the owner accepted under ruling 75 D1 are amended.
What it buys: batch 2 shrinks from thirteen decisions to one rule; four S3 orders and probes and ten counted runs leave the gate; the owner sees S1 and S2 working before the remaining 30 or so engine orders are spent.

It would change: IN_FORCE topics 9 and 13 (Fleet authority in v0, the runs, metrics' place), rulings 61 B1, 72 point 1 and 75 D1 as amended, the roadmap's step 5 and the rows of S3, S10 and A78.

**Evidence.** IN_FORCE.md:171 'Fleet authority ships in v0, as the second specification batch'; docs/proposals/ruling-61/README.md:17-31 (B1 to B13). No scenario needs more than B13: experiments/013-s2-shared-login/plan.md:50 'B2-S writes ruling 61 B13'; experiments/016-s5-control-contention/plan.md:157 'B2-S, B2-C and B2-H are not prerequisites: no act-for/appointment/withhold'; 015 plan.md:251 'No batch-2 gate'; 017 plan.md:177; 018 plan.md:141; 019 plan.md:161-162; 020 plan.md:159, 172. HANDOVER.md:221 'Batch 2 | Not begun'; `git branch -r | grep -i batch` returns nothing. IN_FORCE.md:148 'Metrics are not part of v0's gate; they are built right after batch 2'. S3: experiments/014-s3-two-machines/plan.md:76-79 (RET1, RET2, A81a, A81b), 86-89 (four runs); ruling-75/README.md:57-65 (C3 to C5). S10: experiments/021-s10-swapped-providers/README.md:182-185 (twelve replays), 44-48 'The machines contract's second implementation is S3's evidence'. Serial engine work: experiments/013-s2-shared-login/plan.md:297 'There is no parallel requests.rs merge lane.' Purpose and claim: docs/AUDIT.md:262-263 (first user; teams not in v0), 316-319 (scope guard), ROADMAP.md:113-117 (claim), 152-153 'The acceptance scenario is S1 and S2 passing there'. 41 engine orders: I6.

**Waits.** Batch 2's plan and branch; the order of the 41 engine orders after the pin; the plans of S3 and S10; A78's done-when; the metrics batch. Nothing before the pin waits on it, so it can be answered with the bar.

**Verifier.** partly. Facts hold. IN_FORCE.md:171 (Fleet authority ships in v0 as batch 2) and :148 (metrics right after batch 2). docs/proposals/ruling-61/README.md B1 to B13 as quoted. Batch 2 dependencies in the plans are only B2-S/C/H for 61 B13: experiments/013-s2-shared-login/plan.md:50, :204; 014 plan.md:187; 021 plan.md:25-27; and 015:251, 016:157, 017:177, 018:141, 019:161-162, 020:159 say no batch-2 gate. 41 distinct admin orders recounted by parsing the 'ID | Kind' tables (46 rows, 42 distinct, FLT-D an alias): S2 8, S3 7, S4 6, S5 5, S6 5, S9 4, S10 1, providers 5; 24 M, 14 L, 3 smaller. `git branch -r | grep -i batch` is empty. S10 README.md:44-48 and :182-185 as quoted. ACC-029 and ACC-030 exist (spec/conventions/access.yaml:473, :513).

**Verifier's counterargument.** 1. The scope guard (docs/AUDIT.md:318-321) says 'never exercised by a scenario' is evidence about sequencing, not scope, and that the object model is checked against S3 to S9 before any pruning. Option (b) uses exactly that argument to move scope to v1 before S2 to S9 have run. 2. Ruling 40 names connection and access as the one place Conduit innovates; (b) defers the access machinery (forwarding, appointment, make.v1, review, keep, withhold) and keeps the commodity breadth. 3. The list of what it would change is incomplete: ruling 61 B12 (how A78 runs, and ruling 70 point 5 makes A78 the first priority), ruling 54's review (61 B9 to B11), ruling 62 C4 with 68 R3 (peer engines), ruling 66 (Fleet-held Pools in `manages`), rulings 73 and 75 C5 (the carried return) with the text 038c already merged into spec/, ruling 44's D20 placed in S3 (ROADMAP.md:173-174), and ruling 72 point 2 if a login is done by hand. 4. The owner ruled B1 'Yes' on 2026-11-10 and the 12 replays on 2026-11-11; asking to reverse both within days cuts against 'few new rulings'. Better alternative: ask only for order, not scope. After the bar, B13 alone, then S2 in series as the next tracer; the rest of batch 2 and the reductions of S3 and S10 are put to the owner after S2 has run, through A18, with evidence. That gets the main benefit (S1 and S2 working before 30 more engine orders) with one ruling on order and no amendment of accepted pre-registrations.

**Verifier's corrections.** The question mixes the gate with scope: Fleet authority is v0 scope (IN_FORCE.md:171), not one of ruling 47's gate rows, and metrics are already outside the gate (IN_FORCE.md:148). The 41 is a floor: it leaves out B2-S/C/H, SV1 to SV4, S0b, ADDR and DSP, 049a-2, 049b, 051c, SSH-M and the sign-in chain. Add the rulings listed above to 'It would change'.


## FR-362

**Should the S1 rerun wait for sign-in through Conduit?**

P08 I3 · question-for-owner · verifier: partly · decides: owner

In plain words: the plan puts the whole sign-in chain on the rerun's path. The bar counts the author's lines; a person's sign-ins are listed apart (your answer R1 b), and the plan itself says the second accounts do not gate the rerun. The orders that do move the count, the short form's translator and plan and apply, are unwritten and sit on the other chain.

(a) As planned. The pin waits for 047, 048a, 048b, 051b and 050's sign-in hooks.
(b) Pin without them. Each Agent gets its own home (050's first half); the person signs each home in once with the client's own login, listed apart exactly as panel sign-ins would be; the `account:` lines are still written and counted. The sign-in chain is built right after the pin, beside S2, and is tested as you asked: every account through the Interface, the second accounts only through Conduit (A82, A81, S2 onward).
(c) As (b), and also leave out refusals in two tiers (046).

Recommendation: (b). It removes six of the 18 orders (047, 048a-1, 048a-2, 048b-relay, 048b-join, 051b), 12 to 17 of the 30 to 40 first-round worker-hours the planners' size classes give, all of them serial merges through the core's shared files. The pin then waits only on the chain that decides the bar (043a, 043b, 051a, 052a, 052b, 053). The existing paradigm for a client's login is that the tool owns it (`gh auth login`, `docker login`); relaying it is the part of batch 1 furthest from connection and access.

Cost: the rerun does not exercise sign-in, and the pinned build says so as a stated limit; the account line is written but not checked on that build; order 053's gate ('no not-built stub remains') must name the two stubs it leaves; 050 splits in two and the `.v2` Types are not published as 0.2.0 until their sign-in declaration is final (NAM-024). Experiment 023 raises the counter-argument that dropping the two panel actions 'would falsely describe S1'; under (b) they are not dropped, they are the client's own login steps, listed apart.
Not (c): refusal text decides whether a fresh author recovers from a mistake (finding 4-08).

**Evidence.** docs/proposals/spec-batch-1/README.md:664 critical path '... 042 → 046 → 047 → 048a → 048b → 050 → 053'; :666 'Joining before 053: 037 → 043a → 043b → 052a → 052b; 044; 045; 051a; 051b'; :117 'R1 (b)'; :126 option (b) 'List actions a person takes at the panel apart from authoring lines'; :155 'A5 (your second accounts) does not gate the rerun. Scripted vectors prove the wrong-account undo first.'; :661 'Gate: no "not built" stub remains (grep)'. IN_FORCE.md:277. Stubs: engine/crates/core/src/requests.rs:1049-1050 ('answer.v1', '047'; 'sign-in.v1', '048a'). Size classes: docs/work-orders/117-plan-s2-s3.md:84-90 (047 L; 048a-1 M; 048a-2 L; 048b-relay M; 048b-join S-M; 051b M) and :196-198 (M 2 to 2.5 hours, L 2.5 to 3.5). experiments/023-paper-count/results-2026-11-11.md:165. Second accounts untouched: docs/proposals/ruling-72/README.md:76-82.

**Waits.** The order in which the rerun-path orders are written and dispatched (046 and 043a can start either way); the content of 050 and 053; the pin.

**Verifier.** partly. Facts hold. docs/proposals/spec-batch-1/README.md:664-666 (critical path through 047, 048a, 048b, 050), :117 and :126 (R1 b), :155 (A5 does not gate the rerun), :661 (053's stub gate). engine/crates/core/src/requests.rs:1049-1050 (the two stubs). Size classes at docs/work-orders/117-plan-s2-s3.md:84-90 and :196-198: the six orders sum to 12 to 17 hours and the 14 classed orders to 29.5 to 40. docs/proposals/ruling-72/README.md later answers as cited. Further support the analyst did not cite: the tracker itself already describes sign-in as beside the path (docs/AUDIT.md:100 'Beside it, the sign-in chain'), against HANDOVER.md:217, so the documents disagree and a decision is needed.

**Verifier's counterargument.** 1. The saving is allowance and admin merges, not necessarily days: after (b) the pin still waits on S0b, 046a, 046b and half of 050, and chain B (043a, 043b, 051a, 052a-1, 052a-2, 052b, 053) is then the longer chain. 'The pin then waits only on the chain that decides the bar' is not accurate. 2. Ruling 68 R1 (b) and ruling 76 point 4 ('when a set-up raises a sign-in, apply goes on') both presume panel sign-ins inside the rerun, and the paper forecast 21/5/0/3/2/2/2 rests on point 4 (docs/proposals/ruling-76/README.md:39). R1's rejected option (c) was rejected because it 'skips what ruling 49 tests' (spec-batch-1/README.md:127); (b) is close to it and should say so. 3. Signing a client in by hand into a home the engine made means typing a path with an instance ID, which ruling 63 could count as a line and which a fresh author can get wrong. A simpler variant: rerun on the shared primary logins exactly as on 2026-11-10 (packages/agent-codex/README.md:40-41 'The owner's subscription login is shared'), with the .v2 packages from the existing 050a/076/078 branch and no split of 050. 4. The owner asked twice that sign-in be tested through Conduit; (b) delays that test, it does not drop it, and should be put that way. The owner is the right decider.

**Verifier's corrections.** State that (b) changes what the rerun measures under 68 R1 and 76 point 4. Replace 'waits only on the chain that decides the bar' with the orders that remain on the core chain (S0b, 046a, 046b, the homes half of 050). Add the shared-login variant as an option.


## FR-363

**Until the pin, run one lane: the rerun path**

P08 I4 · recommendation · verifier: partly · decides: coordinator

Change: do not restart the lane coordinator until the build is pinned. The handover leaves that choice to the new session.

1. First, the two things that unblock everything: main's CI (the SSH-R test race, which is in the lane's paths), then order 071's last round (I17).
2. Then dispatch three orders that are ready today and touch different crates: 046a in the core (needs 042 and 038a, merged), 043a in the loader (needs 037, 040, 041a, merged), 051a in the shell (needs 042, merged). Write 043b, 052a and 052b next: they are the author's surface, which is what the bar counts, and nothing of them is written.
3. No specification order that is not a defect on the rerun path. The queued one (PRV-021's cases, PRV-015, labels, one live node per machine) concerns S3; park it.
4. Hold questions that would add a feature until the bar is judged; log them. The owner wants few rulings, and 30 arrived in the three days after the last bar.
5. Put a remaining-work table in the tracker: the gate's rows, the orders left for each, their state. The 'acceleration plan' that rulings 71 and 72 cite is not in the tree, and the roadmap's step states are stale (I18).
6. Keep the two model-checking reruns (MC2 on 077's text, MC4 against 081): they need no engine and no admin merge.

Cost: scenario preparation idles, and S3's parked branches age (I17). Ruling 71 set the lanes up to save about a week; tell the owner the lane is paused, and why.
Buys: the whole Codex allowance on 18 orders in place of a fifth of it; fewer rebases against the hot files; a green CI before engine merges.

**Evidence.** HANDOVER.md:99-101 'If the lane is to go on, start a new lane coordinator with this brief'; :198-205 (CI); :213 (071, one must); :215 (the next spec order's contents). Readiness: spec-batch-1/README.md:545 '046 E5 ... After 042 and 038a'; :598 '043a ... After 037 and 040; merges after 041a'; :629 '051a ... After 042'. `git grep -n -i 'acceleration plan'` finds only docs/proposals/ruling-72/README.md:6 and :32. docs/proposals/ruling-71/README.md:9 'This saves about a week.' PROVIDERS.md:59-61 (MC2 rerun and MC4 'unblocked').

**Waits.** I3's answer changes which six orders are written before the pin, not the first three dispatches.

**Verifier.** partly. HANDOVER.md:99-101 leaves the restart to the new session; :198-205 (CI); :213 (071, one must); :215 (the next spec order). Readiness by the batch plan holds: spec-batch-1/README.md '046 E5 ... After 042 and 038a', '043a ... After 037 and 040; merges after 041a', '051a ... After 042'; all merged. `git grep -n -i 'acceleration plan'` finds only docs/proposals/ruling-72/README.md:6 and :32. What does not hold: by the admin's split 051a follows S0c, which is unwritten (117-plan-s2-s3.md:87-90); 046 touches the core, the shell's server.rs and CLI, the runner and about 45 cases, 043a touches the runner and case.schema.json (a hot file, spec-batch-1/README.md:668), so the three do not 'touch different crates'; 'a fifth' of the allowance has no source; lane orders touch packages/ and experiments/, not the hot files, so pausing the lane does not reduce hot-file rebases (042's rebase was onto the admin's own 070, 081 and 077).

**Verifier's counterargument.** Pausing the lane reverses the mechanism the owner accepted in ruling 71 point 1 to go faster ('This saves about a week'), after finding three to six weeks unacceptable (docs/proposals/ruling-71/README.md:5). The coordinator may choose not to restart it, but the owner should be asked in the same message as the other questions, not told afterwards. The lane's worker time is small (22 orders, about 4 hours of first rounds on 11-12), so stopping all of it frees little. A narrower rule is better: stop scenario servers and harnesses, keep the lane work that is on a gate row and needs no engine: the model-checking reruns, the provider-contract suites and second implementations (ROADMAP.md:76 'each suite's dated run on both of its contract's implementations remains'), and safety vectors once 071 is in. Point 3 (no spec order off the rerun path) collides with AGENTS.md's goal that language work comes first and with the review itself, whose findings will need spec fixes; the queued spec order also holds 082's defects, which are not S3's alone.

**Verifier's corrections.** Dispatch list: 046a and 043a are ready; 051a waits for S0c under the admin's split. Drop 'a fifth' and 'fewer rebases against the hot files'. Add: put the lane's pause to the owner; narrow the lane to gate rows that need no engine instead of none.


## FR-364

**The critical path to the S1 rerun: 18 unwritten orders in two chains, about a third of the planned build done**

P08 I5 · fact · verifier: supported · decides: none

Before any of it: fix main's CI; finish 071 (one must); re-pin the dogfood engine if an order is to run through Conduit.

Left on the rerun path: 12 orders by the batch plan's numbers, 18 by the split the admin gave the planners on 2026-11-11. None is written.

Chain A, the core lane, serial (the orders share registration.rs, access.rs, requests.rs and the runner's lib.rs):
S0b core seams (M) → 046a refusal tiers (L), 046b closed Conduits from tombstones (M) → 047 pending-action core (L) → 048a-1 (M), 048a-2 sign-in as a pending action (L) → 048b-relay (M), 048b-join (S to M) → 050 the Agent packages 0.2.0, merged with 050a, 076 and 078 (M plus a real-client trial) → 053.

Chain B, the author's surface, which is what the bar counts:
043a translator core (1 to 1.5k lines) → 043b Fleet key, manages, templates (1 to 1.5k) → S0c CLI table (M), 051a shell commands (M) → 052a-1, 052a-2 plan and apply (L each; the batch plan left it unestimated, 'likely 1.5-2.5k') → 052b replacement and authoring commands (unsized) → 053.
Joining: 051b pending-action commands (M), after 047 and 048b.

Last: 053 (S1 in the short form, the guides, the measurement plan, the fresh author's pack), the pinned tag, the rerun by the coordinator and by a fresh Codex author with the owner present, then the owner's judgment of the bar with findings 4-01 and 4-11 and C6 to C12.

Size in the plan: 8.8k to 12.5k lines of Rust for the ten orders the batch plan sized (052b and 053 unsized), against 4.35k to 6.3k planned for the seven build orders already merged. By the planners' size classes the 14 classed orders are 30 to 40 first-round worker-hours; four are unclassed.

Measured against the plan: the batch plan targeted 'about one Codex hour' an order. The seven merged build orders took 46, 112, 138, 163, 76, 72 and 199 minutes for the first worker round, then up to three more rounds. Merge to merge on the core chain: 3.2 h (040), 2.5 h (041a), 7.9 h (041b), 19.9 h (042, the first order through Conduit, rebased onto three merged orders). No date follows: chain A is seven merges deep, chain B six, and the measured step is hours.

**Evidence.** spec-batch-1/README.md:102 '27 work orders. Each targets about one Codex hour'; :100 '14k-19k lines of Rust ... Plan and apply is unestimated ... (likely 1.5-2.5k) ... estimates run 1.5 to 2 times low'; :470 'CORE LANE (serial ...)'; sizes at :480, :499, :513, :525, :543, :554, :564, :574, :581, :603, :608, :615, :620, :627, :633, :640; :657-661 (053); :663-668 (critical path). The split and classes: docs/work-orders/117-plan-s2-s3.md:74-95 ('Batch 1's plan has only 046, 048a, 048b, 050 and 052a; the splits are: ...'), :196-198. First-round minutes: Records at docs/work-orders/039-build-local-names-and-expected-id.md:61 (46), 040-...:57 (112), 041a-...:62 (138), 041b-...:52 (2 hours 43), 044-...:51 (76), 045-...:47 (72); HANDOVER.md:138 'first worker turn took 11966 s' (042). Merge times: `git log --first-parent --format='%cd %h %s' --date=format:'%m-%d %H:%M'`: 879f3063 11-11 04:30 (039), b60112ed 07:44 (040), 5ed438b2 10:12 (041a), 1091a71b 18:06 (041b), 57813f06 11-12 13:57 (042). HANDOVER.md:217, :220 (rows 9 and 12); `ls docs/work-orders` holds no file for 043, 046, 047, 048, 051, 052 or 053.

**Verifier.** supported. `ls docs/work-orders` holds no file for 043, 046, 047, 048, 051, 052, 053, S0b or S0c. The split at docs/work-orders/117-plan-s2-s3.md:74-96 gives 18 unwritten: S0b, S0c, 046a, 046b, 047, 048a-1, 048a-2, 048b-relay, 048b-join, 050, 051a, 051b, 052a-1, 052a-2, 052b, 053, 043a, 043b. Planned sizes re-added from spec-batch-1/README.md: remaining ten sized orders 8.8k to 12.45k lines; merged seven 4.05k to 6.0k plus 0.3k of tooling. First-round times re-read in the Records: 46, 112, 138, 163, 76, 72 minutes and 11,966 s. Merge times from `git log -1 --format=%cd`: 04:30, 07:44, 10:12, 18:06 on 11-11 and 13:57 on 11-12, giving 3.2, 2.5, 7.9 and 19.9 hours.

**Verifier's corrections.** Add what was measured against the plan's sizes, which the analyst did not check. `git show --numstat` on the seven merges gives 3,784 lines of Rust (039 163, 040 699, 041a 293 plus 524 of Python, 041b 821, 042 936, 044 576, 045 296) against 4.05k to 6.0k planned: the line estimates held or ran high, while time ran one to three times the one-hour target. The exception is 049a1: 3,890 lines of Rust on its branch against 1.2k to 1.5k planned for all of 049a. So duration and rounds, not lines, are the quantity to plan with. The 19.9 hours for 042 spans a night, a rebase onto three merges and the largest order of the chain; it is not a clean step time.


## FR-365

**From the pin to v0's gate: ruling 47 is in force, one of its six rows is met, and 41 engine orders stand between**

P08 I6 · fact · verifier: supported · decides: none

Ruling 47 is in force (ledger row 47: in force, amended by 65, 68 and 71). Its six required rows, with what later rulings added, at 2eb8f258:

1. S1 to S10 end to end on the reference engine, vectors passing. S1 ran on 2026-11-10 on the build before batch 1. S2 to S10 are pre-registered and planned; none has run and no scenario engine order is written. The engine's conformance document says what is missing: runtime-provider calls and remote connections 'unserved', secret reads and delivery, package-source providers and https fetching 'unfinished', declare-lost and control dispatch 'unfinished', answer.v1 and sign-in.v1 stubs. The plans name 41 distinct engine, spec and vector orders for the admin after the pin (24 M, 14 L or 'at most L', 3 smaller): S2's secrets and filters 8, S3 7, S4 6, S5 5, S6 5, S9 4, S10 1, provider dispatch 5. Beside them: the node work on s3/m2 (049a-1 and 080 checkpointed with musts; 049a-2, 049b, 051c and SSH-M unwritten), 46 lane orders not yet built, 13 owner steps and 33 counted runs.
2. S1 and S2 with real Agents. S1 done once, to be redone at the rerun. S2's run gate lists the pin, B2-S/C/H, 050 and the whole sign-in chain, 052b, SEC-D, FLT-D0 to D2, S2-H, S2-O, the keychain store, the plain server (not chosen) and tagged safety vectors.
3. Every safety vector passes. Not yet definable: the field naming a vector's safety promise is spec pending; SV1 waits for 071; SV2 to SV4 are unwritten; G5 assigns 53 promises to vectors that do not exist.
4. Author measurements against A70. Failed on 2026-11-10 (332 and 646 lines against 64). Waits for I5.
5. Model checking. MC1 to MC7b ran on 2026-11-11: 114 of 217 hold in bounded projections, 7 under ruling 75 A1, 29 contested. The text fixes merged (077, 081, 082); the models have not been rerun against them. Ten promises wait for MC9 after batch 2; 11 exclusions wait for the owner.
6. Subscription terms (A8). Met on 2026-11-10; to be read again before publication.
Added since ruling 47: the replays of S3 and S10 (and S7 and S8) on second accounts signed in through Conduit, and every account through the Interface once. The chain is unwritten; A81 and A82 are open.

Order from the pin: rerun → the owner's judgment → batch 2 (no plan, no branch) and s3/m2 merge → engine orders, serial on the hot files → lane orders → owner steps → runs → A75. The deepest chain I traced is seven engine orders after the pin (SV1 → FLT-D0 → FLT-D1 → S6-ACCESS → S6-DUTY1 → S6-DUTY2 → S6-WIRE) before S6's harness and runs.

Not in the gate: the cold read (roadmap 'Read'), mutation testing (A44), the independent builds (step 7), metrics.

**Evidence.** docs/proposals/ruling-log.md:55 (row 47 'in force; amended by 65 ...; amended by 68 ...; amended by 71'); docs/proposals/ruling-47/README.md:36-48; IN_FORCE.md:261-269. engine/CONFORMANCE.md:41 'Runtime provider calls and remote connections are unserved'; :56 'https fetching and package-source providers remain unfinished'; :130 'Declare-lost.v1 and control dispatch remain unfinished public invocation paths'; :269; engine/crates/core/src/requests.rs:1049-1050. Order counts: a PyYAML-free Python parse of the 'ID | Kind | ... | Who' tables in experiments/013 to 021 plan.md and docs/implementation/PROVIDERS.md: 138 rows (137 distinct, FLT-D being an alias), 41 distinct admin orders, 58 lane orders of which 12 are built (S2-F, S2-L1, S2-L2, SCR1, S4-SRV1, A63-RK1, RK2, H1, RP1, MP1, PS, FS), 13 owner rows, 19 run rows (33 runs), 6 probes. S2's gate: experiments/013-s2-shared-login/plan.md:204-207. Safety vectors: IN_FORCE.md:265; HANDOVER.md:213; experiments/012-model-checking/G5.md:405-407. First runs: docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md:64-70; fresh-author-run-2026-11-10.md:44-50. G5.md:34-47, :395-415; PROVIDERS.md:59-61. ROADMAP.md:66 (A8). IN_FORCE.md:264; ROADMAP.md:73-74 (A81, A82 open). HANDOVER.md:221. S6 chain: experiments/017-s6-revoke-in-flight/plan.md order table (S6-ACCESS needs FLT-D1; S6-WIRE needs DUTY1/2) and 013 plan.md:83-84.

**Verifier.** supported. docs/proposals/ruling-log.md:55 (row 47 in force; amended by 65, 68, 71). docs/proposals/ruling-47/README.md table of six rows. IN_FORCE.md:261-269 carries the added conditions. engine/CONFORMANCE.md:41 ('Runtime provider calls and remote connections are unserved'), :56, :130; requests.rs:1049-1050. My parse of the plan tables: 142 rows, 138 distinct IDs, 41 distinct admin orders after removing the FLT-D alias; 13 owner rows; 19 run rows giving 33 runs; 6 probes; 58 lane orders. experiments/013-s2-shared-login/plan.md:203-205 (S2's run gate). experiments/012-model-checking/G5.md:31-46 and :391-415.

**Verifier's corrections.** G5's 29 refuted or contested promises include the 7 that hold only under ruling 75 A1 (G5.md:49-50: 143 modelled = 114 + 29); they are not a separate group. '53 promises assigned to vectors that do not exist' overstates: G5.md:51 and :405-407 say the rows assign work to vectors and that no one audited whether existing vectors cover them. The 41 is a floor (see I2). 'Engine orders, serial on the hot files' should read built in parallel, merged in series (117-plan-s2-s3.md:72-73 against 013 plan.md:297).


## FR-366

**The bar passes on paper with no margin, and both runs must pass**

P08 I7 · assessment · verifier: partly · decides: owner

The rule (ruling 71, point 3): in each of two runs, fewer lines than the baseline on most of S1's tasks and more than one and a half times on none. The baseline's counts are 33, 16, 0, 8, 3, 2, 2. Three things follow from the arithmetic, before any run:

- T3's baseline is 0. It can never count as fewer, and any counted line on it is 'much more'. Experiment 023 says so itself.
- 'Most' is four of seven. T6 and T7 have a baseline of 2, and the paper count puts Conduit at 2 and at 2 or 3. So T1, T2, T4 and T5 must all be fewer, in both runs.
- T5's baseline is 3 and the paper count is 2: one more line makes it equal and the bar fails. T7 at 3 'equals the threshold': a fourth line fails it.

The paper count was made by an author who knows the design, on plan and apply as designed, not built. The other run is a fresh Codex author, whose first run wrote 646 lines, twice the coordinator's 332. A pass needs that author to match a design-aware paper count to the line on T5, T6 and T7. I judge a fail on a two- or three-line task more likely than not, and such a fail would say nothing about the thesis under test: the baseline's small tasks are one-line edits of a permission file, with no relationship that can be inspected or revoked.

Why it matters to finishing: the gate requires the pass; batch 2's merge and findings 4-01 and 4-11 wait on the judgment; and the last failed bar was answered with 64 new promises (I8). The owner judges the bar and can weigh all this. What is needed before the run is I13, so that one rerun gives the owner enough to judge and a second is not needed.

**Evidence.** docs/proposals/ruling-71/README.md:34-41; IN_FORCE.md:266. experiments/023-paper-count/results-2026-11-11.md:19-28 (counts 23, 6, 0, 3, 2, 2, 3 against 33, 16, 0, 8, 3, 2, 2; thresholds 49.5, 24, 0, 12, 4.5, 3, 3); :115-118 ('Most' means at least four of seven. T3's equality to zero cannot count as fewer ... Both real runs must pass'); :128 'T7 | more (3 equals the threshold)'; :153 (an explicit plan before each apply: 'only three fewer; fails both parts'); :159 'Any later required command makes T3 much more than zero'; :168-171 (each request route fails). docs/work-orders/069b-spec-plan-apply-ruling-76.md Record: 'S1's tasks count 21, 5, 0, 3, 2, 2 and 2 lines, primary'. fresh-author-run-2026-11-10.md:44-50 (263, 67, 0, 32, 112, 44, 128); log-increment-4.md:22 '332 lines against the baseline's 64'. docs/guides/full-review.md:153-156 (the same doubt, as a question).

**Waits.** The reading of the rerun; batch 2's merge; findings 4-01 and 4-11.

**Verifier.** partly. docs/proposals/ruling-71/README.md point 3 and IN_FORCE.md:266 as quoted. experiments/023-paper-count/results-2026-11-11.md:19-28 (23/6/0/3/2/2/3 against 33/16/0/8/3/2/2), :115-118, :128, :159. First runs: 154/73/0/2/62/29/12 = 332 and 263/67/0/32/112/44/128 = 646. The arithmetic for T3 (never fewer; any counted line is much more) and T5 (one line from equal) holds. What does not: the analyst used the count from before ruling 76. docs/proposals/ruling-76/README.md:39 gives the forecast in force, 21, 5, 0, 3, 2, 2 and 2: T7 is equal with one line of slack, not at the threshold (:11 records that the owner saw T7 'exactly at one and a half times' and ruled partly for that reason). 069b's Record gives strict 25/6/0/3/2/2/3.

**Verifier's counterargument.** 'More likely than not' has no base: no fresh author has yet written the short form, and the 646 lines were canonical JSON on a build without plan and apply. A larger sensitivity than the one-line margins is missed: style. Experiment 023's block column is 40/18/0/3/2/2/7 and 'forecast[s] failure' (results :130): T1 and T2 exceed the baseline and T7 is much more. Ruling 63 makes flow the documented style and reports block 'beside the primary one', but does not say which reading is primary when the author writes block. So a pass depends on the fresh author writing flow style, which depends on `init`'s skeleton (052b) and the guides (053). That supports the analyst's doubt more strongly than T5 does: the bar can turn on line wrapping. If a fail is judged likely and uninformative, the consistent step is one question to the owner before the run (ruling 45 fixes measures beforehand), for example whether T3 counts toward 'most' and which style is primary; the analyst raises the problem and asks nothing.

**Verifier's corrections.** Use ruling 76's forecast: T1, T2, T4 and T5 must be fewer; T5 has no slack; T6 and T7 each have one line before 'much more'; T3 has none. Add the block-style reading and the unsettled primary style.


## FR-367

**After the bar failed, the 'simplification' added: 30 rulings and 64 promises in three days, none removed**

P08 I8 · assessment · verifier: partly · decides: owner

The first build's own table says what a failed first thesis calls for: 'Simplify the documents: fewer concepts, better defaults.' What followed between 2026-11-10 and 11-12:

- 30 rulings (47 and 49 to 77) and 13 dated later answers in eight records.
- 64 promises added, none removed (455 → 519); the specification from 19,666 to 24,902 lines.
- A sixth family (Fleets), 14 new core operations, 10 templates, four new promise files (fleets, sections, pending, enrollment).
- The gate grew too: safety vectors defined (68), both runs must pass (71), second accounts through Conduit's sign-in (72), every account through the Interface (72's later answers), all 217 convention-layer safety promises modelled (75 E1), nine pre-registrations each with its own runs (75 D).
- Queued and unsized: batch 2 (ruling 61's thirteen decisions, the grant review, peer engines), the metrics batch, the next spec order.

The author's surface did shrink on paper (39 lines against 332), but by adding a second form of the language and tooling beside it, not by removing concepts. Several additions are not connection or access: the relay of a client's sign-in, the carriage of a browser return, the evidence shown for a joining machine, metrics. Ruling 40 says such things sit behind a provider's contract; 25 of the new promises (PND-001 to 013, ENR-001 to 012) put them in the specification's own layers. Whether each belongs there is for the language analysts; from the plan's side the effect is plain: the specification grew 14 percent in two days while 227 of its 519 promises have no case in the manifest, and every addition lengthens both the pin and the gate.

**Evidence.** docs/implementation/FIRST_BUILD.md:32. Rulings per day: `grep -E '^\| [0-9]+ \| 20' docs/proposals/ruling-log.md | awk -F'|' '{print $3}' | sort | uniq -c` → 11-05 37, 11-06 6, 11-07 1, 11-09 3, 11-10 20, 11-11 9, 11-12 1 (77). Later answers: dated entries under '## Later answers' in rulings 42 (1), 52 (1), 67 (1), 68 (1), 71 (2), 72 (5), 75 (1), 76 (1). Promises: `git grep -hE '^- id: [A-Z]+-[0-9]+' &lt;commit> -- 'spec/promises/*.yaml' 'spec/conventions/*.yaml' 'spec/interface/*.yaml' | wc -l` → a08cf36b (11-10) 455, 2eb8f258 519; `comm` of the two ID lists: 64 new, 0 absent. Spec lines: 19,666 at a08cf36b, 24,902 now (yaml, md, json under spec/). spec-batch-1/README.md:59 '60 new, taking today's 455 to 515'; :72-75 (14 operations, 10 templates); :69 'Four new promise files'. Gate additions: IN_FORCE.md:264-267. Coverage: PyYAML over conformance/manifest.yaml and the promise files: 292 of 519 IDs appear in a case, 227 do not (language 97, convention 104, interface 26). experiments/023 results :28 (39 against 64).

**Waits.** I2 is the place to act on it.

**Verifier.** partly. Ledger rows per day recounted: 37, 6, 1, 3, 20, 9, 1; rulings 47 and 49 to 77 are 30. Promises by `git grep`: 455 at a08cf36b, 519 at 2eb8f258; `comm` gives 64 new (SCT 15, PND 13, ENR 12, FLEET 6, ACC 6, others 12) and none removed. Spec lines 19,666 and 24,902. Core package by PyYAML: operations 23 to 37, templates 7 to 17. Four new promise files exist. Manifest coverage recomputed: 292 covered, 227 not (language 97, convention 104, interface 26). 13 dated later answers in eight records.

**Verifier's counterargument.** The causal framing is too strong. By commit time on 11-10, rulings 49, 50 and 51 (09:59, 10:24, 11:02) precede the coordinator's run (11:39) and ruling 52 (12:27) precedes the fresh author's (12:41). So 26 rulings follow the failed bar, not 30, and sign-in (ruling 49) and adding machines (rulings 50, 51) were not answers to it. Of the 64 promises, the 25 in pending.yaml and enrollment.yaml trace to those rulings; about 39 trace to the simplification and Fleets. The owner also ruled to remove things in this period (ruling 66: no Scope object, no grouping shorthand; ruling 76: no prune flag), which a count of promise IDs does not show.

**Verifier's corrections.** 'After the bar failed: 30 rulings' should read 26 after both runs. '64 promises in answer to the bar' should read about 39; 25 come from sign-in and enrollment, ruled before or apart from it. '14 percent' is by promise count; by lines the specification grew 27 percent. Ruling 75 E1 requires each of the 217 to be held by a model or listed with its reason, not all modelled.


## FR-368

**Work done ahead of need: the lanes hold more Rust than the engine, for parts the engine cannot yet call**

P08 I9 · assessment · verifier: partly · decides: coordinator

What exists at 2eb8f258 that nothing can use yet:

- Four packaged local providers, the runtime kit and the SSH runtime provider: 18,055 lines of Rust in six crates. The engine's own conformance document says runtime-provider calls are 'unserved', package-source providers 'unfinished' and secrets unserved. SSH-R and its drainer took seven orders (137, 140, 145, 146, 149, 150; 147 open) and 6,731 lines, for S3, whose machine provider, enrollment core, dispatch, loss handling and secret delivery are unbuilt. It is a third runtime where the claim needs two.
- S2's stub, Conduit-aware server and adapter, and S4's effect server: 5,250 lines in four crates, for two scenarios whose 14 engine orders are unwritten.
- Nine pre-registrations and nine plans (4,404 lines), written against main at 31acd73 or 1b717c4. Their 'What main has now' sections are already out of date (S2's still says 042 and 070 are absent).

In all, packages hold 25,853 lines (the two Agent packages 2,548 of them) and experiments 25,372, against the engine's 35,245.

Costs already seen: main's CI is red because of a lane test, which skips the engine's shards, so 077 merged on local evidence; the allowance ran low while the rerun path had one merge; the servers and plans predate the decisions the bar will force (4-01, how what exists is changed; batch 2).

Why it happened is structural, not careless. A standalone crate is a 6 to 35 minute order (orders 122 to 150). An order in the engine's core takes 46 minutes to over three hours for its first round, a differential vector run takes 2.4 hours, and core orders are serial. Idle lanes filled with what was cheap.

Not all lane work was early. Model checking (15,800 lines) paid at once: it found MC2-001 and five more faults in the text, and 077, 081 and 082 fixed them. That is the lane to keep.

**Evidence.** Rust lines: `git ls-files &lt;dir> | grep -E '\.rs$' | xargs cat | wc -l` → provider-file 857, provider-local-directory 1,108, provider-local-machine 981, provider-local-process 2,125, provider-ssh-runtime 6,731, runtime-provider-kit 6,253; scenario-s2-linear 1,687, -adapter 1,565, -restrictions 387, scenario-s4-effect 1,611; agent-claude-code 1,285, agent-codex 1,263; engine 35,245; experiments 25,372 (012-model-checking 15,800). engine/CONFORMANCE.md:41, :56, :269. docs/AUDIT.md:471 'the SSH runtime ... (a third runtime beside the local-process and Docker pair)'. `wc -l experiments/01[3-9]*/plan.md experiments/02[01]*/plan.md` and the READMEs: 4,404. experiments/013-s2-shared-login/plan.md:34-37. HANDOVER.md:198-205 (CI), :210 ('CI's engine shards were skipped'), :209 ('about 2.4 hours on the Mac'). Lane order times: Records of 122 (22 minutes), 124 (7), 132 (690 s), 137 (2,121 s), 143 (1,422 s), 150 (648 s). G5.md:54-66 (findings); Records of 077, 081, 082.

**Waits.** I4.

**Verifier.** partly. Rust lines recounted with `git ls-files | grep .rs | xargs cat | wc -l`: the six provider and kit crates 18,055; the four scenario crates 5,250; Agent packages 2,548; packages 25,853; engine 35,245; experiments 25,372 (012 15,800). engine/CONFORMANCE.md:41, :56 as quoted. docs/AUDIT.md:471 ('a third runtime'). The nine plans and nine pre-registrations total 4,404 lines. HANDOVER.md:198-205, :209-210. Lane order first rounds range from 111 s (134) to 2,121 s (137).

**Verifier's counterargument.** The title joins two numbers that do not belong together. Code the engine cannot yet call is 23,305 lines, two thirds of the engine; the lanes exceed the engine only when model checking (15,800), which the analyst says paid at once, is added. Of the 48 lane orders merged on 11-11 and 11-12, 26 are provider, kit, harness or scenario code; 12 are model checking; 10 are the catalogue, pre-registrations and plans. SSH-R is not the lane's surplus: ruling 77 made SSH the way the second machine joins. The pre-registrations are required before a run by ruling 45 and were accepted under ruling 75 D, and the plans are the only inventory of remaining work; the analyst's own count of 41 comes from them. The provider-contract harness and suites serve a gate row (two implementations per contract passing the same vectors) and need no engine. What is ahead of need is narrower: the scenario servers for S2 and S4, and SSH-R's drainer before the machine provider and dispatch exist.

**Verifier's corrections.** Title: 'the lanes hold 23,305 lines of Rust that the engine cannot yet call'. The summary's '48 of 84 merged orders were lane orders for scenarios and providers the engine cannot yet call' should read 26 of 84.


## FR-369

**The owner's time still to come: 13 owner steps, 35 counted runs, 11 to 12 hours stated and more unsized**

P08 I10 · fact · verifier: supported · decides: none

Counted across the plans of S2 to S10 and the providers' plan:

- 13 rows of kind 'owner': S2-O, S2-LOG-O, S3-O1, S3-O2, S3-OF, S4-ACCOUNT, S7-O, S7-NO, S8-A, S8-O, S10-O, A63-OUS, A63-OKC.
- 33 counted runs of about an hour each (S2 4, S3 4, S4 3, S5 2, S6 2, S7 2, S8 2, S9 2, S10 12), plus the S1 rerun's two, 'with the owner present'.

Minutes the plans give:
- S2: about 60 (S2-O), 15 (register the Linear application), 30 (the Linear login probe), 45 active minutes for each of four runs: 285.
- S3: about 60 (S3-O2) and, under option A of S3-OF, 69 to 78 active minutes for each of four runs (18 commands by hand a run, 72 in all): 336 to 372.
- S4: about 60 reserved (S4-ACCOUNT).
Stated total: 681 to 717 minutes, 11.4 to 12 hours.

Not sized: 17 further runs that need the owner at sign-ins and lapses (S4's replay 1, S7 2, S8 2, S10 12; S10's plan says 'book owner time per replay'); the S1 rerun; eight of the 13 owner rows; ruling 77's part 2, the Mac's rename and HTTPS; the dedicated macOS user of 75 B3; the plain Linear server; the bar's judgment with 4-01, 4-11 and C6 to C12; G5's 11 exclusions; G8 and G11; A20.

The runs of S5, S6 and S9 and S4's two scripted runs need no synchronous presence.

One item on the waiting list is already answered: the reading of ruling 75 D4 (I18).

**Evidence.** experiments/013-s2-shared-login/plan.md:88 ('about 1 hour presence'), :90 ('about 15 minutes'), :91 ('about 30 minutes of owner presence'), :92-95; README.md:191 '45 minutes of active owner time per run'. experiments/014-s3-two-machines/plan.md:70, :265-273 ('18 command-and-confirm actions per run, 72 for four runs ... 9-18 extra active owner minutes per run ... 69-78 active minutes per run, 4 hours 36 minutes-5 hours 12 minutes for four'). experiments/015-s4-partial-setup/plan.md:99-101; README.md:196-198 ('Owner time is zero during scripted execution'). 016 README.md:182, 017 README.md:190, 020 README.md:201 (acceptance and review only). 018 plan.md:73-76; 019 plan.md:76, :82-84; 021 plan.md:100-102, :158 'book owner time per replay'. PROVIDERS.md:148-149. HANDOVER.md:220 ('the rerun with the owner present'), :227-248. docs/AUDIT.md:166-172.

**Verifier.** supported. My parse finds the same 13 owner rows (S2-O, S2-LOG-O, S3-O1, S3-O2, S3-OF, S4-ACCOUNT, S7-O, S7-NO, S8-A, S8-O, S10-O, A63-OUS, A63-OKC) and 19 run rows giving 33 runs (4, 4, 3, 2, 2, 2, 2, 2, 12). Minutes re-added: S2 60 + 15 + 30 + 4 x 45 = 285 (013 plan.md:88, :90, :91; README.md:191); S3 60 + 4 x (69 to 78) = 336 to 372 (014 plan.md:70, :265-273); S4 60. Total 681 to 717. Five owner rows carry a time, eight do not. The 17 unsized runs are 1 + 2 + 2 + 12.

**Verifier's corrections.** The title says 35 runs and the body 33; 35 includes the S1 rerun's two. The stated hours are planning allowances, not measurements (014 plan.md:262 says so).


## FR-370

**Status against each step of the roadmap**

P08 I11 · fact · verifier: supported · decides: none

Step 1 (A48, A47, A42): done; A42 stays in progress until G7.
Step 2 (the language and conventions): called done as a step; none of its items is. Every area is written as promises; 227 of 519 promises have no case in the manifest; A17 is open; A20 waits for the owner after A7.
In parallel: A70, A71, A4, A8 done. A22 partly (a real daemon untested). A5, A6, A7, A10, A73 open. A74 not begun (no experiments/006). A77 in progress, its done-when unmet (admin time not recorded). A80 open.
Step 3 (tracer, the first build): in progress. Increments 1 to 3 done (2026-11-09 and 10). Increment 4 open: both runs failed the bar on 2026-11-10. Batch 1: spec 6 of 6 merged (036a, 036b, 037, 038a, 038b, 038c); build 7 of 22 merged (039, 040, 041a, 041b, 042, 044, 045); 049a1 checkpointed; 14 unwritten. A78: one order of five through Conduit. A73 open.
Read (A59, A61): not begun.
Step 4 (second providers, S10): marked 'later', begun in fact. Built: the kit, four packaged first providers, SSH-R. No second implementation on a different tool for machines, package sources or secret stores; no contract suite (A63-VR, VM, VP, VS) written; S10 not begun.
Step 5 (S2 to S9): marked 'later', begun in fact. Nine pre-registrations and plans; S2's four packages, S4-SRV1 and SCR1 merged; no scenario engine order written; no run.
Step 6 (whole-model checks): in progress. Harness and MC2 to MC7b ran on 2026-11-11; fixes 077, 081, 082 merged; reruns and MC9 owed; mutation testing (A44) not begun.
Step 7 (readiness gate, two builds): not begun. G8, G9 and G10 assessed 'Not met' on 2026-11-11; G1 to G7 have no current assessment; build-base unpublished.
Step 8 (prune, A18), After (A14), Publication (A75, A76): not begun.
Later versions: A65 in progress; A66, A67 open.

A ruled step not carried out: ruling 71 point 2 says 'Batch 2 begins now, on a branch'; the handover says 'Not begun' and no such branch exists.

**Evidence.** ROADMAP.md:24-36 (states), :59-77 (register; :69 A74 'open: no one has started it'; :70 A77 'admin time was not recorded'; :71 A78). docs/AUDIT.md:60, :62 ('The confirmed assessment before rulings 40 to 43 met none ... it is historical'), :455 (A17 open), :465 (A44 open), :471 (A63). docs/implementation/INDEPENDENT_BUILDS.md:130-134 (G8, G9, G10 'Not met'). Batch 1: spec-batch-1/README.md:102-105; merges 57222f8e (036a, #122), 1f46dda6 (036b, #126), 9294c49d (037, #132), d08fb8bb (038a, #131), c8e9aa20 (038b, #180), 719a8992 (038c, #200), 879f3063 (039), b60112ed (040), 5ed438b2 (041a), 1091a71b (041b), 57813f06 (042), be729584 (044), 99b641a3 (045). `ls experiments | grep -c 006` → 0. `ls packages` (12 crates; none for Docker, Git, keychain or an SSH machine provider). experiments/022-provider-contracts row, ROADMAP.md:76 ('each suite's dated run on both of its contract's implementations remains'). ruling-71/README.md:30; HANDOVER.md:221; `git branch -r | grep -i batch` → none. Manifest coverage as in I8.

**Verifier.** supported. ROADMAP.md:24-36 states as quoted; :69 (A74 open), :70 (A77), :71 (A78). Batch 1: 5 spec orders in the plan plus 038c merged; 7 of 22 build orders merged; 14 unwritten (046, 047, 048a, 048b, 049b, 043a, 043b, 050, 051a, 051b, 051c, 052a, 052b, 053). `ls packages` shows no Docker, Git, keychain or SSH machine crate. docs/implementation/INDEPENDENT_BUILDS.md:125-134 (G8 to G10 'Not met'). HANDOVER.md:221 ('Batch 2 | Not begun').

**Verifier's counterargument.** On the 'ruled step not carried out': ruling 71 point 2 also says the branch is cut 'from the build that the S1 rerun pins' (docs/proposals/ruling-71/README.md, decision 2), and that build does not exist. Only the drafting of batch 2's plan could have begun; the branch could not.

**Verifier's corrections.** A79's state is stale in the roadmap and the status repeats nothing about it: ROADMAP.md:72 reports 0 of 14 exact states, while experiments/011-short-form-spike/README.md:94-95 records the 2026-11-11 run passing 14 of 14. A working spike of the translator therefore exists, which lowers 043a's risk.


## FR-371

**Timeline from git: fourteen days, four of them building**

P08 I12 · fact · verifier: supported · decides: none

Reachable commits: 677; on main's first parent 412; pull requests #6 (2026-11-06) to #265 (2026-11-12).

Per day (first-parent commits by commit date; orders merged; rulings by the ledger's date; promises in spec/ at the day's last commit):
- 10-30: 90 commits. 10-31: 30. 11-01 to 11-04: 6. Design in prose; no promises, no rulings recorded.
- 11-05: 1 commit; 37 rulings (the plan audit opens); 0 promises.
- 11-06: 1; 6 rulings; 434 promises on arrival (#6).
- 11-07: 2; 1 ruling; 452 (#7), then 454 (#8).
- 11-08: none.
- 11-09: 42 commits; 18 orders; 3 rulings; 455 promises. The first build begins.
- 11-10: 69; 14 orders; 20 rulings; 455. Both S1 runs fail the bar.
- 11-11: 120; 58 orders (32 admin, 26 lane); 9 rulings; 518.
- 11-12: 51; 26 orders (4 admin, 22 lane); 1 ruling; 519. Handover at 14:50 -0800.

By week: 10-30 to 11-05, no promises; 11-06 to 11-12, 519, of which 64 in the last two days. Rulings: 77 in eight days, 30 of them on 11-10 to 11-12, with 13 dated later answers.
Rerun-path build orders merged: 11-11 six; 11-12 one.
Vectors: 367 before batch 1's first build order, 494 now; the batch plan forecast about 215 new.
The conformance suite is 303,801 lines of case files (494 cases), about nine times the engine's Rust; order 041a alone migrated 167 cases with 29,517 changes.

**Evidence.** `git rev-list --count HEAD` 677; `git rev-list --count --first-parent HEAD` 412; `git log --first-parent --format='%cd' --date=short | sort | uniq -c`; pull-request numbers from subjects ending '(#N)': 6 to 265, 151 such commits. Promises per commit: the `git grep` count in I8, run over `git log --first-parent --reverse --format=%H -- spec/promises spec/conventions spec/interface`: e271d1c7 434, c4d7d332 452, 2ca84f36 454, d27d5efc 455, 57222f8e 470, d08fb8bb 486, 9294c49d 502, c8e9aa20 515, 55cf7473 516, 719a8992 517, 1be89f96 518, 35d2e45a 519. Rulings and orders per day: as in I8 and I1. Vectors: docs/work-orders/039-...md Record 'Fourteen vectors were added ... all 381 vectors'; docs/AUDIT.md:58 '494 portable vectors'; spec-batch-1/README.md:84. Case files: `git ls-files conformance | grep 'case.yaml$' | xargs cat | wc -l` 303,801; 041a's Record: '167 vector cases ... 29,517 changes'.

**Verifier.** supported. `git rev-list --count HEAD` 677; `--first-parent` 412; per-day first-parent counts 90, 30, 1, 3, 2, 1, 1, 2, 42, 69, 120, 51 by commit date and by author date; pull requests 6 to 265 in 151 subjects. Promise counts at the cited commits: 434, 452, 454, 455, 470, 486, 502, 515, 516, 517, 518, 519. Orders and rulings per day as in I1 and I8. Case files 303,801 lines in 494 files. 039's Record: fourteen vectors added, 381 in all. 041a's Record: 167 cases, 29,517 changes.

**Verifier's corrections.** 'No promises' before 11-06 reflects when pull request 6 merged, not when the text was written. The handover commit is the session's end, so 11-12 is about fifteen hours of work, not a day.


## FR-372

**S1 rerun: report the margins and what the baseline cannot do, and fix beforehand how each outcome is read**

P08 I13 · experiment-change · verifier: partly · decides: coordinator

Row: scenario S1 and step 3 of the roadmap; A68; the bar compares with A70. The measurement plan under docs/proposals/first-build/increment-4/ is where it is written.

Claim tested: the first build's first thesis, 'Declaring relationships costs an author less than configuring each agent'.

Pass condition: unchanged (ruling 71, point 3; ruling 63's counting). This changes what the rerun reports, not the bar.

Add to the measurement plan before 053, for each task and each run:
1. The margin in lines to 'fewer' and to the 1.5 times threshold, so a one-line miss is visible as one.
2. D22's five measures beside the baseline's (files edited, concepts met, grants by hand, adapters written, recovery steps that need a person). They are already owed on every run; put them in the same table as the lines.
3. What the baseline could not do, yes or no: no restart or lost session for the change; no Agent gains a tool it was not granted (T5); closing one Agent's access leaves the other working (T7); each interval without access.
4. For T3, every command the author typed and whether it carried a chosen value, since one counted line there fails the bar.

And write down, before the run, how three outcomes are read: a pass; a fail only on tasks whose baseline is three lines or fewer; a fail on T1, T2 or T4. The reading is the owner's to give; the coordinator drafts it. The purpose is that the owner can judge the bar from one rerun, and that a near miss is not answered with another additive batch.

Cost: an hour of the coordinator's writing inside 053. Buys: no second rerun for want of evidence.

**Evidence.** ROADMAP.md:139 (S1), :27 (step 3), :166-168 'Every run also records D22's author measurements'; docs/AUDIT.md:462 (A68). docs/implementation/FIRST_BUILD.md:32 (thesis), :110-116 (accepted criteria: 'Closing one agent's access leaves the other working'). docs/proposals/first-build/increment-4/measurement-plan.md:69-82 (the rerun's rules), :104-120 (questions it says are not settled, among them 'which run judges the bar'). coordinator-run-2026-11-10.md:24-26 (no restart, no lost session, no new tool on the upgrade, observed in the first run). ruling-71/README.md:32-33 'A second rerun, if one is needed, runs on a branch from the same pin'. I7 for the margins.

**Waits.** Order 053; the owner gives the reading of the three outcomes with I3's answer or at the bar.

**Verifier.** partly. ROADMAP.md:166-168 (D22 on every run). docs/proposals/first-build/increment-4/measurement-plan.md:42-66 already lists D22's measures, requests, intervals without access and the baseline's restarts and lost sessions beside them; spec-batch-1/README.md:657-660 puts 'windows without access, measured' in 053. So items 2 and 3 are largely owed already. Items 1 (margins per task) and 4 (T3's commands) are new and sound: experiments/023 results :159 says any required command makes T3 much more than zero. measurement-plan.md:113-115 lists 'which run judges the bar' as unsettled; ruling 71 point 3 has since settled it (both runs), so that line is stale, not evidence of an open question.

**Verifier's counterargument.** Writing down beforehand that a fail only on the small tasks is read differently from a fail on T1, T2 or T4 is, in effect, a change to the bar without a ruling. Ruling 71 point 3 fixed the rule 'before any count of the rerun is seen' so that it would not be argued afterwards. If the rule is ill-posed for baselines of 0, 2 and 3, the clean route is one question to the owner before 053: whether T3 counts toward 'most', and which style is primary (I7). That is the owner's, so 'decides: coordinator' is right only for the reporting additions.

**Verifier's corrections.** Split the item: reporting additions (margins, T3's commands, one table) are the coordinator's inside 053; the reading of outcomes, or any amendment of the bar's rule, is the owner's and belongs with the questions. Say that D22's measures and the intervals are already owed.


## FR-373

**A74: observe on the real pair with the parts now built, not with engine-free stand-ins**

P08 I14 · experiment-change · verifier: partly · decides: owner

Row: A74 in the register ('Across two machines, with no engine'); state 'open: no one has started it'. It feeds S3.

Claim tested: the mechanisms under S3 hold on real machines (hypotheses H1 to H10: a call without the engine's proof refused at a gate, a removed instance's credential refused, a stream after a silent cut, dispatch at most once, a gate behind NAT, and the rest).

Change: A74 was designed when there was no engine. S3's plan still orders a new crate of stand-ins for it (S3-A741, L, 600 to 850 lines) before any observation. The kit's gate, SSH-R and the engine's https address now exist. Run the observations on the real pair with those parts, during S3's stage 0 and first run; keep only the silent-cut fault tool as new code, since the provider harness (A63-H2b) reuses it. Record H5, H8, H9 and H10 as unmeasured, which is S3's plan's own recommendation.

Pass condition: none; ruling 47 lists A74 as observational. Measures, fixed before it runs: for each hypothesis, what was observed, on which parts and versions, dated, in experiments/006-across-machines; an unobserved hypothesis is written 'unmeasured', never as a pass.

Cost: observations made with Conduit's own gate cannot tell a mechanism's failure from a fault in that gate as cleanly as a stand-in could. Buys: one L order and one prerequisite off S3's path.

The owner accepted S3's pre-registration with 'A74 first', and S3's plan already asks the owner about the unmeasured cells; this rides with that answer.

**Evidence.** ROADMAP.md:69. experiments/014-s3-two-machines/plan.md:43 ('experiments/006-across-machines/ does not exist'), :68 (S3-A741 'L, 600-850 lines'), :72 (S3-A742), :91-98, :217-221, :251-255 ('permit explicitly unmeasured unused cells'). PROVIDERS.md:125 ('A63-H2b | Fault controls using A74's silent cut | ... S3-A741'). docs/proposals/ruling-47/README.md:45-46 ('The other probes are observational and do not block: ... A74'). Built parts: packages/runtime-provider-kit, packages/provider-ssh-runtime; order 079 on origin/s3/m2 (9b8f3326).

**Waits.** S3-A741's dispatch; the first owner step of S3 (S3-O1).

**Verifier.** partly. ROADMAP.md:69 (A74 open, 'with no engine'). experiments/014-s3-two-machines/plan.md:43, :68 (S3-A741, L, 600 to 850 lines), :72, :91-98, :217-221 ('A74 runs before any dependent engine task'), :251-255. docs/implementation/PROVIDERS.md:125 (A63-H2b depends on S3-A741). Ruling 47's record lists A74 among the observational probes. The kit's gate (order 126) and SSH-R are merged; 079 is on origin/s3/m2 only (9b8f3326); 080 is unmerged; SSH-M is unbuilt.

**Verifier's counterargument.** The built parts do not yet connect: the engine cannot call a runtime provider on main, so observations 'during S3's stage 0 and first run' would come after S3-L1 and S3-L2 are built. That gives up A74's purpose, which is to observe the mechanisms before engine work depends on them. A better change keeps 'A74 first' and 'no engine' and still drops most of S3-A741: use the kit's gate as the gate under test and the provider-contract harness (experiment 022) as the caller, and write only the silent-cut tool. That is a change to a non-binding plan and a lane order, within the coordinator's reach; the one point for the owner, the unmeasured cells, is already asked in S3's plan. H2 (another run on the machine cannot reach the server) should be recorded as known unmet under ruling 77's single run user, not left unmeasured.

**Verifier's corrections.** Decider: the coordinator for how A741 is built; the owner only for the unmeasured cells, already on the list. Replace 'during S3's stage 0 and first run' with 'at stage 0, before S3-L1'.


## FR-374

**A78: measure whether Conduit speeds the work, log its frictions, and keep it off the serial chain for now**

P08 I15 · experiment-change · verifier: partly · decides: coordinator

Row: A78 in the register. Done when: 'Five orders run through Conduit, each friction logged as a finding of the first build'. State: one order (042).

Claim tested: ruling 70 gives dogfooding two purposes in the owner's words, 'to speed up development' and 'to refine conduit through real usage'. Only the second is being observed; nothing measures the first.

Changes:
1. Measures, fixed before the next order: per order, wall-clock from dispatch to a clean verifier; engine restarts and their seconds; turns lost and why; guards tripped. Compare with an order of the same size class on the plain runner. Evidence so far: 042 took 19.9 hours merge to merge, against 2.5 to 7.9 for the three before it; a restart of 171 to 191 s for each fresh conversation; one turn lost to a 55-second network outage that plain workers survive.
2. Log the four known frictions in increment 4's log before the next order. The handover says none is logged; ruling 70 asks that each be.
3. Choose orders off the serial chain. The handover plans 046 and 047 through Conduit next; those are the core lane's critical orders. Run a joiner (051a or 043a) through it, and move the chain there once a fresh conversation needs no restart and a transient network loss does not fail a turn.
4. Done-when: orders 4 and 5 need appointment, which is batch 2. If the owner takes I2 (b), it becomes five orders dispatched by root.

Pass condition: none today; it is a count of orders. With change 1 it gains one: by the fifth order, an order through Conduit takes no longer than its class on the plain runner.

Items 1 to 3 are the coordinator's; item 4 rides with I2.

**Evidence.** ROADMAP.md:71. docs/proposals/ruling-70/README.md:23-25, :46-49 ('The work is ordered so that Conduit runs its own work orders (A78) as early as possible'). HANDOVER.md:136-139 (042's four rounds, 11966 s), :143-150 (the outage 'after 55 s ... session cannot restart implicitly'; 'Each fresh conversation costs an engine restart: 171 to 191 s'), :152-154 ('None is yet a finding in increment 4's log ... Next: re-pin the dogfood engine ... then run 046 and 047 through it; orders 4 and 5 need appointment (batch 2)'). Merge times in I5.

**Waits.** The next order through Conduit; the dogfood engine's re-pin.

**Verifier.** partly. ROADMAP.md:71 (done when five orders; one so far). docs/proposals/ruling-70/README.md decision 5 quotes both purposes and says the work is ordered so that Conduit runs its own orders 'as early as possible'. HANDOVER.md:136-139, :143-150, :152-154 as cited; none of the four frictions is in increment 4's log.

**Verifier's counterargument.** The evidence offered does not bear on speed. The 19.9 hours for 042 spans a night, the largest order of the chain and a rebase onto 070, 081 and 077. The restart of 171 to 191 s is about 2 percent of a 2.5-hour turn. The one material cost is the turn that cannot survive a network loss. A pass condition of 'no longer than its class on the plain runner' cannot be judged on five orders when first rounds in one class range from 46 to 199 minutes; measure the overhead itself (restart seconds, turns lost, repair turns, guards tripped) per order. Taking the serial chain off Conduit lowers the real load the owner asked for; the stronger order is to fix the lost-turn fault first (the handover already names it 'a fault fix before the rerun'), then keep 046 on Conduit.

**Verifier's corrections.** HANDOVER.md:143 says a brief outage failed the turn after 55 s; it does not say the outage lasted 55 seconds. The change to A78's done-when amends ruling 61 B12 and is the owner's, as the analyst says.


## FR-375

**S3, S10 and batch 2, if the owner takes I2 (b)**

P08 I16 · experiment-change · verifier: partly · decides: owner

These follow only from the owner's answer to I2; none is the coordinator's to make, because the owner accepted the pre-registrations under ruling 75 D1.

S3 (scenario row S3; experiments/014). Claim: the conventions hold across two machines, through a partition and a lost machine. Pass conditions P1 to P6 keep everything on partition, loss, owed stops, enrollment and unknown outcomes. Changes: runs from four to two (local profile, the coordinator and a fresh author); the isolated profile on the second machine moves to v1; vector V6 (a second account's sign-in carried across machines), S3-RET1, S3-RET2 and A81 move to v1, and the Agent on the second machine signs in there (Claude, as 75 C3 already rules) or by device code (Codex). Measures unchanged for what remains. Saves two L orders, two probes, and 138 to 156 of the owner's active minutes under S3-OF option A.

S10 (scenario row S10; experiments/021). Claim: every provider can be swapped. P3 is unchanged: each contract has two implementations on different existing tools passing the same suite. Change: replays from twelve to four (every provider swapped, S1 and S2, by each author), with E06 once per author. The all-swapped ledger still lists every provider-specific edit (E14), and the per-contract suites isolate each provider, which is what the single-swap replays were for. Saves eight replays of about an hour each.

Batch 2. Write and build only ruling 61 B13 (one small spec order, one core, one shell), from the pin. MC9 then covers the ten pending-action promises with no batch 2 to wait for.

Register and roadmap: the rows of S3, S10 and A78, step 5's order (S2 first, in series), and IN_FORCE topic 13's run lines change in the one ruling that records the answer.

**Evidence.** experiments/014-s3-two-machines/plan.md:76-79, :86-89, :135-139 (V6 belongs to RET1), :265-273; ruling-75/README.md:57-59 (C3), :79-90 (D1 to D4). experiments/021-s10-swapped-providers/README.md:44-48 (P3), :182-185 ('ten replays ... two replays, twelve in all, plus E06 once per author'); ROADMAP.md:178 (E14 'with every provider-specific edit listed'). ruling-61/README.md:31 (B13). experiments/012-model-checking/G5.md:395-398 ('MC9 after batch 2: the ten catalogue promises PND-001 ...').

**Waits.** I2.

**Verifier.** partly. experiments/014-s3-two-machines/plan.md:76-79 (RET1, RET2, A81a, A81b), :86-89 (four runs), :265-273 (69 to 78 minutes a run, so two runs are 138 to 156). experiments/021-s10-swapped-providers/README.md:182-185 (twelve replays). experiments/012-model-checking/G5.md:395-398 (MC9's ten promises). Ruling 75 C3, C5 and D1 as cited.

**Verifier's counterargument.** Three things the item says are kept cannot be. S3's P6 is D20, what local and isolated runtimes each protect during a partition (014 README.md:64-66; ROADMAP.md:173-174, placed by ruling 44); dropping the isolated profile empties it. S3's P5 and ruling 72 point 2 require the second accounts to sign in through Conduit's flow; 'by hand over SSH' is not that flow, so only the device-code route through a pending sign-in satisfies them. The carried return is already in spec/ (order 038c, merged as 719a8992); moving it to v1 needs a stated limit or the text removed, since spec/ alone binds. On S10, the single-swap replays are what attribute a difference to one provider at scenario level; the suites attribute it only at contract level. The owner cut the replays from twenty to twelve one day earlier.

**Verifier's corrections.** Say that P6 and D20 change, that the second machine's sign-in must still go through Conduit (device code), and what happens to 038c's text. List ruling 44 (D20), 72 point 2, 73 and 75 C5 among the rulings touched.


## FR-376

**Finish 071 now; leave 049a1, 080 and 144 parked until the pin**

P08 I17 · recommendation · verifier: supported · decides: coordinator

Four checkpointed branches are ageing against main:
- 071 (the manifest in pieces): 87 files, 8,485 insertions and 6,557 deletions, 32 commits behind; one must left.
- 049a1 (enrollment core): 76 files, 20,901 insertions, 84 commits behind, written from main and meant for a side branch that is itself 49 behind; one must, one should, after five rounds.
- 080 (the shell's provider calls): 20 files, 4,570 insertions, 49 behind; three musts in its HTTP bridge.
- 144 (S4's hook server): 19 files, 4,061 insertions; two musts.

Change: do 071 first, straight after the CI fix. It is one round, and it is the only one of the four on a gate row: SV1, and with it every safety vector, waits on it; every vector-adding merge in the meantime makes its rebase worse; and once the manifest is in pieces the manifest conflict disappears for every later branch, 049a1 included.

Leave the other three as they are until the pin. The handover lists finishing rounds for each as next steps; they serve S3 and S4, which are off the rerun path, and each round spends allowance the 18 orders need. If the owner takes I2 (c), 049a1 and the enrollment part of 080 are not needed for v0 at all.

Cost: 049a1's rebase gets harder with each core merge (it already conflicts in the manifest and the shell's server.rs). Buys: allowance on the path, and no polishing of work a ruling may defer.

**Evidence.** `git diff --shortstat origin/main...origin/&lt;branch>` and `git rev-list --count origin/&lt;branch>..origin/main`: wo/071-conformance-manifest-pieces (87 files, 8485+, 6557-; 32 behind), wo/049a1-build-enrollment-core (76 files, 20901+, 75-; 84 behind), wo/080-shell-runtime-provider-calls (20 files, 4570+, 59-; 49 behind), wo/144-s4-srv2-hook (19 files, 4061+; 5 behind); `git log origin/main..origin/s3/m2` → one commit, 9b8f3326 (079), 49 behind. HANDOVER.md:211 ('five rounds ... rebasing onto s3/m2 conflicts in conformance/manifest.yaml and the shell's server.rs'), :212, :213, :214. docs/work-orders/117-plan-s2-s3.md:134-137 ('SV1 adds the field naming a vector's safety promise, after order 071 ... merges').

**Waits.** The CI fix comes first.

**Verifier.** supported. Recomputed against 2eb8f258: wo/071 87 files, 8,485 insertions, 6,557 deletions, 32 behind; wo/049a1 76 files, 20,901 insertions, 75 deletions, 84 behind; wo/080 20 files, 4,570 insertions, 59 deletions, 49 behind; wo/144 19 files, 4,061 insertions, 5 behind; origin/s3/m2 one commit ahead (9b8f3326), 49 behind. HANDOVER.md:211-214 for the musts. docs/work-orders/117-plan-s2-s3.md:134-137 (SV1 after 071).

**Verifier's counterargument.** For 049a1 the cheapest moment to close is now: one must (the manifest's PRV-014 coverage) and one should, which HANDOVER.md:211 says to fix under ruling 71 point 4. Its branch holds 3,890 lines of Rust and 14,753 lines of case files verified through five rounds; parked through 18 merges on the same hot files it is likely to need rework that costs more than one small round. The analyst's own recommendation in I2 keeps enrollment in v0, so the work will be needed. Also, 071 changes the manifest's form, so 049a1's manifest edits must be rewritten once after it; merging 071 first is still right.

**Verifier's corrections.** Consider: 071 first, then 049a1's two findings fixed and merged into s3/m2; leave 080 and 144 parked.


## FR-377

**State lines that are stale at this commit, one of them an owner question already answered**

P08 I18 · fact · verifier: supported · decides: coordinator

1. The reading of ruling 75 D4 is listed as waiting for the owner in the tracker, the handover and the plans of S3, S7, S8 and S10 ('the owner confirms D4's reading'). The owner confirmed it on 2026-11-12; ruling 75's later answer, IN_FORCE and the providers' plan say so. Strike it from the waiting lists; the counted runs of S3, S7, S8 and S10 do not wait on it.
2. The roadmap marks steps 4 and 5 'later'; providers, scenario servers and plans for both have merged since 2026-11-11.
3. INDEPENDENT_BUILDS.md says conformance/ 'do[es] not exist yet' and its directory table marks conformance/, engine/ and packages/ as not existing. All three exist.
4. The readiness gate has no current assessment of G1 to G7; the tracker points at an assessment from before rulings 40 to 43 and calls it historical. Only G8 to G10 were assessed on 2026-11-11.
5. G5.md lists 077, 081 and 082 as 'not yet run'; all three merged. (The review brief names this trap.)
6. The scenario plans' 'What main has now' sections describe main at 31acd73 or 1b717c4: S2's says 042 and 070 are absent.
7. 517 of the 519 promises carry `vectors: []` in their own files while the manifest holds the coverage; A33's 'each with a vector' cannot be read from the promise files.

All are the coordinator's to fix in a docs pass; none changes behavior.

**Evidence.** 1: docs/AUDIT.md:171; HANDOVER.md:243-245; experiments/014-s3-two-machines/plan.md:249; 018 plan.md:164; 019 plan.md:187; 021 plan.md:236; against docs/proposals/ruling-75/README.md:114-120, IN_FORCE.md:273 ('D4 as its later answer reads it') and PROVIDERS.md:356 ('The owner confirmed it on 2026-11-12'). 2: ROADMAP.md:29-30 against docs/AUDIT.md:471 and HANDOVER.md:222. 3: docs/implementation/INDEPENDENT_BUILDS.md:7, :85-88; `git ls-files conformance | grep -c case.yaml` 494; `ls packages` 12 crates. 4: docs/AUDIT.md:62; INDEPENDENT_BUILDS.md:123-134. 5: experiments/012-model-checking/G5.md:399-404; docs/guides/full-review.md:218. 6: experiments/013-s2-shared-login/plan.md:18, :34-37. 7: `cat spec/promises/*.yaml spec/conventions/*.yaml spec/interface/*.yaml | grep -cE '^  vectors: \[\]'` 517.

**Verifier.** supported. 1: docs/AUDIT.md:171, HANDOVER.md:243-245 and the plans (014 plan.md:86-89, :249; 018:164; 019:187; 021:236) still wait on D4's reading; docs/proposals/ruling-75/README.md:114-120, IN_FORCE.md:273 and PROVIDERS.md:357 record the owner's confirmation of 2026-11-12. 2: ROADMAP.md:29-30. 3: INDEPENDENT_BUILDS.md:6-7 and :82-88. 4: docs/AUDIT.md:62; INDEPENDENT_BUILDS.md:123-134. 5: G5.md:399-404. 6: 013 plan.md:34-37. 7: PyYAML over the promise files: 517 of 519 have `vectors: []` (the two with entries are CTR-013 and SCH-009).

**Verifier's corrections.** The citation is PROVIDERS.md:357, not :356. Add three more stale lines: ROADMAP.md:72 (A79, see I11); measurement-plan.md:113-115 ('which run judges the bar', settled by ruling 71 point 3); docs/AUDIT.md:100 against HANDOVER.md:217 on whether sign-in is on or beside the rerun path.


## What the verifier says the analyst missed

- **The bar's largest sensitivity is YAML style, not a one-line margin** Experiment 023's block-style count is 40/18/0/3/2/2/7 against the baseline's 33/16/0/8/3/2/2 and forecasts a fail (results-2026-11-11.md:130). Ruling 63 documents flow style and reports block 'beside the primary one' but does not say which reading is primary when the author writes block. A pass by the fresh author therefore depends on `init`'s flow skeleton (052b) and the guides (053), and the bar can turn on line wrapping. This is stronger evidence for the review brief's doubt about the measure than the T5 margin, and it should be one question to the owner before 053.
- **The forecast in force is ruling 76's, not experiment 023's primary** docs/proposals/ruling-76/README.md:39 gives 21, 5, 0, 3, 2, 2 and 2 after the owner's five readings; the owner ruled knowing T7 sat at the threshold (:11). The analyst cites 069b's Record but argues from 23/6/0/3/2/2/3.
- **The admin's own off-path engine work competed with the rerun path** On 11-11 and 11-12 the admin wrote and ran 078, 079, 080 and 049a1 for the Windows node (pull request 174; branch commits to 11-12 13:25), with 049a1 taking five rounds. These are engine-building orders on the same machines and the same reviewer. Meanwhile 043a was dispatchable from 11-11 10:12 and was never written. The analyst attributes the starvation to the lane; lane orders were about 4 hours of first-round worker time on 11-12.
- **Planned sizes were not checked against what merged** The seven merged build orders added 3,784 lines of Rust against 4.05k to 6.0k planned, so the plan's line estimates held, while durations ran one to three times the target and rounds reached four. 049a1 is the outlier at 3,890 lines against 1.2k to 1.5k for all of 049a. The remaining path should be sized in rounds and hours, and the enrollment chain treated as the order most likely to overrun.
- **Enrollment as an unbuilt chain is not assessed as a risk** The assignment named it. The chain is 049a-1 (checkpointed after five rounds), 049a-2, 049b, 051c, SSH-M, A63-MDIS, A63-VM and MP2, on a side branch (s3/m2) that is 49 commits behind and is to merge only after the pin, across 18 merges to the same hot files. The analyst covers 049a1's branch age in I17 but not the long-lived side branch as a strategy or the chain's size.
- **The tracker and the handover disagree on whether sign-in is on the rerun path** docs/AUDIT.md:99-101 says 'Finish batch 1's build and pin it ... Beside it, the sign-in chain'; HANDOVER.md:217 and spec-batch-1/README.md:664 put 047, 048a, 048b and 050 on it. This supports asking I3 and is itself a defect in the state documents.
- **A translator spike already passes S1's states** experiments/011-short-form-spike/README.md:94-95: the 2026-11-11 run passed 14 exact translation states and 14 comparison probes. ROADMAP.md:72 still reports 0 of 14. This lowers the risk of 043a, the first order of the chain that decides the bar.
- **No narrower option for the lane was considered** Between 'restart the lane' and 'one lane until the pin' lies a lane limited to gate rows that need no engine: the model-checking reruns, the provider-contract suites and second implementations (the layered claim's own row), and safety vectors after 071. It keeps ruling 71's parallelism where it is cheap and stops the scenario servers.
- **No measure of the allowance exists** The analysis leans on the Codex allowance and the owner's usage as the binding constraint, but the repository records neither spend per order nor what remains; A77's done-when is unmet because admin time was not recorded (ROADMAP.md:70). Any plan that rations the allowance needs worker and verifier seconds per order summed from the Records, which exist for most orders.

## The verifier's notes

Method: everything was recounted at 2eb8f258 in the read-only checkout; nothing was written, built or fetched. Numbers in the analyst's items are, with few exceptions, exact: orders per day, rulings per day, promise counts, spec lines, Rust lines, manifest coverage, branch statistics, owner minutes and the 41 admin orders all reproduce. The weaknesses are in attribution and in what the recommendations leave unsaid, not in the counting.

Three corrections matter most. First, the rerun path's starvation is real but is caused less by the lane than by the admin's own off-path engine orders and by ready orders left unwritten; lane orders are small in worker time, so pausing the lane frees little. Second, the bar analysis uses the forecast from before ruling 76 and misses the style sensitivity, which is larger than any one-line margin; if the bar is judged likely to fail for reasons unrelated to the thesis, that is a question for the owner before the run, and the analyst asks none. Third, I2 (b) and I16 would touch more rulings than they list (61 B12, 54's review, 62 C4, 66, 73, 75 C5, 44's D20, 72 point 2) and lean on the argument the scope guard discounts; a sequence-only version (B13, then S2 in series, scope decided after S2 through A18) gets most of the benefit with one ruling on order.

Coverage. Read in full: docs/guides/full-review.md, docs/implementation/HANDOVER.md, ROADMAP.md, AGENTS.md, rulings 47, 49, 61, 63, 70, 71, 72, 75 (C, D, E and later answers) and 76, the ledger's rows by date and row 47, IN_FORCE.md topic 13 and the cited lines of topics 7 and 9, spec-batch-1/README.md lines 50 to 200 and 455 to 680, experiment 023's results, the measurement plan, docs/AUDIT.md lines 40 to 180, 236 to 330 and 425 to 522, INDEPENDENT_BUILDS.md's status, directory table and gate, order 117's admin section, G5.md's standings and needs, S2's and S3's plans in the cited parts, and every plan's order table by script. Checked by command: git history, branch divergence, promise and line counts, core package counts, manifest coverage. Not read: the promises themselves beyond ACC-029 and ACC-030, the engine's code beyond the cited lines, the plans of S4 to S10 beyond their tables and cited lines, the Records of most orders beyond their time lines, the private handoff note (outside the tree). I did not trace the seven-order S6 chain in I6 and did not verify that each of the 12 lane orders named as built matches a table ID one by one.

One observation outside the commit: the shared refs show a local branch named admin/ci-ssh-r-flock, so work on the CI fix may have begun after 2eb8f258; I did not open it.
