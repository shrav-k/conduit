# P01-work-orders-lanes: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** The pipeline produces a great deal and records it well enough to audit: 125 orders in four days, 116 merged to main, each with a Record. It does not converge the way its rules assume. A fresh verifier is a new sample of the diff: wherever a second fresh pass ran it found new musts about half the time (16 of 31 in the lane, and both early orders that had one), and the early orders look clean only because nobody looked twice. Size is what separates orders that finish from orders that do not: lane code above 2,000 changed Rust lines still had musts at the second pass in 7 of 9 cases against 4 of 19 below, and the three engine orders left unmerged are all above 3,000. Finishing orders written after the coordinator reads the findings work (6 of 9 clean in one pass); raw third to fifth rounds do not (at least 9 of 13 found new faults). The lane more than doubled merged orders a day (26.8 to 56.8) and its model checking changed the specification, but engine merges a day halved (11.6 to 5.3), the admin's median time per order went from 45 to 287 minutes, batch 1's last six orders are unwritten, and the lane's providers sit behind an unmerged engine order with empty contract suites. The same day brought the second machine, a slow engine suite and ten rulings, so the Records cannot pin the slowdown on the lane alone. Three incidents reached main, all from shared refs and shared CI rather than file conflicts: lane merges stayed in their paths. I would change five things: a size limit checked on the round-1 diff; no raw third round; a fresh read of every coordinator fix; CI that actually gates (required checks, a macOS leg, engine shards a package crate cannot skip, no local squash); and a machine-written Record that includes the coordinator's own spend, which is what ran out. One question goes to the owner: what the lane does when it restarts.

**What the analyst read.** Read in full: docs/guides/full-review.md, docs/implementation/HANDOVER.md, AGENTS.md; docs/implementation/WORK_ORDERS.md lines 1-223 and the Record part of every order 001 to 066 (the condensed decisions under each were skimmed, not studied); the `## Record` section of all 72 order files under docs/work-orders/ that have one; docs/work-orders/verify.md, verify.schema.json and README.md; order 042's whole text; .github/workflows/checks.yml and nightly.yml; docs/proposals/ruling-71/README.md; IN_FORCE.md topic 15.

Read in part: orders 068, 071, 074, 075 and 121 (purpose and done-when); ruling 48 and ruling 74 records; docs/AUDIT.md (where things stand, the heads of Purpose and Layered claim, A80); ROADMAP.md (rows A77 and A78, proposing an experiment); docs/implementation/PROVIDERS.md lines 1-75 and its size rows; experiments/022-provider-contracts/README.md and suite indexes; increment 4's log for findings 4-18 and 4-19.

From git: first-parent subjects since 2026-11-08, the bodies of about 30 commits, the commit that added each order file and the one that first carried its Record, and numstat sizes of 63 merge commits. I also read the logs and diff sizes of the unmerged branches (049a1, 071, 080, 144, 050a, 076, 078); those are outside the pinned commit 2eb8f258 and are marked so where used.

Not read: the bodies of most orders, the worker and verifier reports and the runners (kept outside the repository), the private handoff note, the specification, and engine or package code. Nothing was built or run.

Limits of the numbers: for orders 020 to 066 the Records give faults without severity, counted here as must-level; several later admin Records do not itemize musts per round; worker times are mostly first rounds only; 038c's round count is ambiguous in its Record; Rust line counts are insertions plus deletions in .rs files at the merge commit, tests included; whether CI was green at each lane merge cannot be established from the repository.


## FR-264

**Rounds do not converge because a fresh verifier is a new sample of the diff; "a fresh verifier found nothing" is not a test of being done**

P01 I1 · assessment · verifier: partly · decides: none

Each round starts a new verifier that reads the whole diff cold and is told to refute it. On a large diff that is sampling, and the Records show it behaving as sampling.

- Wherever a second fresh pass ran, it found new musts about half the time: 16 of 31 lane orders (13 of 24 code, 3 of 7 paper).
- The early orders look converged only because nobody looked twice. Of the 45 orders 001 to 066, two record a second fresh verifier (022 and 025), and both found more (4 and 3 faults). The design then was one pass, one fix round, merge.
- Raw third and later passes on admin orders found new faults in at least 9 of 13: 038b, 041b, 049a1 (rounds 3, 4 and 5), 069, 070, 077, and 042's fifth. Three were clean.
- Later passes find faults that earlier passes read past. 042's fifth verifier found a sender matched through a reused name after four verifiers had passed the code. 128's architecture names were corrected one class per reader (LoongArch, then MIPS, then ARM, RISC-V and wasm). 100's second verifier found three more misclassified promises "on the same grounds" as the first five.
- Lane code averaged 1.48 verifier musts per 1,000 changed Rust lines (68 over 45,959), so a 3,000-line order carries four or five and each pass finds some.

Judgment: more rounds on the same large diff will not end. What ends it is a smaller diff (I5, I6), a fix aimed at the class of fault rather than the instance (I7), and acceptance by checks that run (vectors, tests, CI) rather than by a reader finding nothing.

**Evidence.** docs/implementation/WORK_ORDERS.md:84 "One more round at most: the admin sends the worker back once"; docs/implementation/HANDOVER.md:63 "A must or a should starts another worker round and a fresh verifier, up to `ROUNDS` (default 2)"; docs/work-orders/verify.md:8 "Try to refute the work." Second verifiers in 001-066: `grep -n -i 'second fresh verifier' docs/implementation/WORK_ORDERS.md` gives only lines 806 (022) and 908 (025). docs/work-orders/042-build-tombstones-and-inventory.md:59-61; docs/work-orders/128-a63-mp1-local-machine.md:146-153; docs/work-orders/100-safety-catalogue.md:183-184. Lane second-pass counts and the 1.48 figure: computed from the per-order table in I4 and from `git show --numstat --format= &lt;merge commit>` summed over .rs files for the 28 lane code deliverables. 042's third pass is inferred to have found faults because a fourth round followed.

**Waits.** I6, I7 and I9 rest on it

**Verifier.** partly. Recounted from the Records at 2eb8f258. Lane: 32 orders ran two rounds; leaving out 144 (not on main), 16 of 31 second passes left musts (13 of 24 code, 3 of 7 paper). `grep -n -i 'second fresh verifier' docs/implementation/WORK_ORDERS.md` finds Records only at :806 (022, four more) and :908 (025, three more). Third and later passes on admin orders: 13 in all; 9 recorded as finding faults, 3 clean (the fourth passes of 042, 069 and 070), 042's third inferred. docs/work-orders/128-a63-mp1-local-machine.md:146-152 and 100-safety-catalogue.md:183-185 read as quoted. 68 musts over 45,959 changed .rs lines reproduced with `git show --numstat` on the 28 lane merge commits. The claim is wider than stated: 037, 038a, 039, 040, 044, 045 and 067 also end 'then a second round' with no verifier after it, as docs/implementation/WORK_ORDERS.md:84-85 designs.

**Verifier's counterargument.** Sampling explains only part. The Records show a second source the item does not separate: faults the fix itself brought in (102-pre-b-s4-s5-s6-s9.md:190-192, the new task; 129-a63-rk-owed-stop-first.md:101-103, the lock added in round two; each new handoff in 146 and 149; 049a1's fix rounds of 434, 272, 409 and 551 Rust lines). There, more rounds fail because each adds unverified code, not because one diff is resampled. 'Will not end' is too strong: six of the seven admin orders that went past two rounds merged. If a pass is an independent sample that costs about five minutes, the existing answer is two independent verifiers on round one (capture-recapture inspection: their overlap estimates what is left), which the analyst did not weigh. At least 15 Records say the verifier could not rebuild or run tests in its read-only sandbox (for example 105:255, 139:140), so it is a reader by construction.

**Verifier's corrections.** By pass: the third pass was never clean (0 of 7 orders); the fourth was clean in 3 of 4. '042's fifth verifier found a sender matched through a reused name after four verifiers had passed the code': four had read it and only the fourth passed it, and the fifth pass followed the admin's own conflict resolution in access.rs and lib.rs and a fix round (042-build-tombstones-and-inventory.md:54-61), so the Record does not show the fault was there earlier.


## FR-265

**The lane more than doubled merged orders a day, but engine merges halved, its output is not yet integrated, and three incidents reached main**

P01 I2 · assessment · verifier: partly · decides: none

Throughput, from the commit that first carries each order's Record:

| | Before ruling 71 (47.5 h) | After (27.0 h) |
| --- | --- | --- |
| Orders merged a day | 26.8 (53, all admin) | 56.8 (64: admin 16, lane 48) |
| Engine orders merged a day | 11.6 (23) | 5.3 (6, one to the side branch) |
| Admin median time from order written to merged | 45 min | 287 min |

The lane's own median was 39 minutes. By calendar day the merges were 19, 13, 58 and 27.

What the lane bought: model checking (13 orders in under six hours) produced eight findings in the promises, which 077, 081 and 082 wrote into the specification. That is the most direct gain for a provable language in the two days. Lane merges also stayed inside their paths; the only file outside was ROADMAP.md, by leave.

What it cost:
- Three incidents on main: the squash of 041b deleted the lane's MC8 files (restored in #190 seven minutes later); lane order files 122 and 123 broke the docs job for 44 minutes (#205); a lane test has made CI red at times since #260, which skips the engine shards, so 077 merged on local evidence.
- 27 of 50 lane order files cite an admin decision or "the admin's word".
- The lane coordinator was a subagent of the admin session, and both stopped for the owner's usage limit.
- Its providers are unexercised: the four contract suites hold no case, and 080, which makes the shell call runtime providers, is unmerged.
- Stale claims: G5's account predates 077, 081 and 082; the kit implements three owed-stop cases PRV-021 does not state.

The same day also brought the second machine, a slow engine suite (fixed by 070) and ten rulings, so the Records cannot attribute the admin's slowdown to the lane alone. Batch 1's last six orders on the rerun path are still unwritten.

**Evidence.** Throughput: `git log --reverse -S'## Record' --format=%ct -- &lt;order file>` per order, split at the commit "Ruling 71:" (#141) and measured from "Ruling 48:" (#17) to HEAD 2eb8f258; 001-004 by their merge commits (#18, #22, #20, #24). Lead time: same command against `git log --reverse --diff-filter=A --format=%ct -- &lt;order file>`. Incidents: `git log -1 --format=%B ed8043a1` ("The squash of 041b was made on a main that had moved to f8adc17, so it removed f8adc17's three files"), `git log -1 --format=%B 128ca08a` ("so main's docs job failed"), docs/implementation/HANDOVER.md:198-205, :210. Admin's word: `grep -c -i "admin's word\|admin's decision\|admin's answer\|admin decided\|admin accepted\|admin's leave\|admin's request" docs/work-orders/1[0-5]\[0-9]-*.md` (27 files, 49 mentions). Paths: `git show --name-only` over the lane's commits since ruling 71 finds only ROADMAP.md outside the lane (e94c0571, 752b4830). Usage: HANDOVER.md:4-5, :19; docs/implementation/PROVIDERS.md:36. Suites: experiments/022-provider-contracts/suites/{runtime,machines,package-source,secrets}/index.yaml each `cases: []`. 080: HANDOVER.md:212. Findings: docs/work-orders/116-mc8-g5-account.md:160-176. Unwritten: HANDOVER.md:217. Kit ahead of text: docs/work-orders/141-a63-rk-own-owed-stop.md:134-135.

**Waits.** I3

**Verifier.** partly. Reproduced with `git log --reverse -S'## Record'` per order file, split at cfd7b233: 53 orders in 47.5 h before (26.8 a day), 64 in 27.0 h after (56.8 a day; admin 16, lane 48); engine 23 then 6 (11.6 and 5.3 a day); by day 19, 13, 58, 27; lane median 39 minutes. ed8043a1 (18:13:51) restores what 1091a71b (18:06:29) removed; bb84a464 (20:30) to 128ca08a (21:14) is 44 minutes; docs/implementation/HANDOVER.md:198-205, :210. The four suites' index.yaml hold `cases: []`. 116-mc8-g5-account.md:179-180 and 141-a63-rk-own-owed-stop.md:134-135 read.

**Verifier's counterargument.** Orders a day counts unlike things: 12 of the lane's 48 are paper and 9 are finishing orders, with a median worker run of 14 minutes against 105 for the admin's. The admin's slowdown began before the lane: the last seven admin orders merged before ruling 71 (037, 038a, 039, 040, 041a, 044, 045) took 123 to 342 minutes each, and they are spec batch 1 on the slow engine. The engine rate before is mostly increment 2's 20-minute orders. So the Records show neither that the lane slowed the admin nor that it shortened the path to the S1 rerun: it added 48 merges beside that path. Also, 20 of the lane's 48 merges were made before CI returned (#198, 19:23 on 2026-11-11), while 104-mc1-model-checking-harness.md:202 and 105-mc2-operation-identity.md:287 say CI was paused; IN_FORCE.md:308 lets a lane merge only 'once CI is green'.

**Verifier's corrections.** Admin median lead time, by the stated method: 57 minutes before (n=49) and 451 after (n=16); 297 if split by when the order was written (n=13). 45 and 287 do not reproduce. The 'admin's word' grep gives 24 files and 45 mentions, not 27 and 49. Model-checking orders are 12 (104, 105, 107 to 116); 13 only with the safety catalogue, 100. Lane commits also changed docs/README.md (e94c0571, 1d633c55), docs/AUDIT.md (e94c0571) and docs/implementation/SAFETY_CATALOGUE.md (c2a1d063, 1f302eca), not ROADMAP.md alone. Twelve orders, not six, are unwritten on the rerun path (HANDOVER.md:217; docs/proposals/spec-batch-1/README.md:664-666). 'Unexercised' means by a contract suite or the engine; each provider passes its own crate's tests.


## FR-266

**What should the lane do when it restarts?**

P01 I3 · question-for-owner · verifier: partly · decides: owner

The lane coordinator stopped with the outgoing session and must be started again. Ruling 71 allows it; what it works on affects the order of the work, which is yours.

(a) Restart it as it ran: its queue is S4's hook server, SSH-R's key sweep and loopback tier, more scenario servers, then the keychain, Docker and Git providers.

(b) Restart it for proof and integration only, until the S1 rerun's build is pinned: fix the test that makes CI red; write contract-suite cases for the providers already merged; rerun the models that 077, 081 and 082 made stale (MC2, MC4, and the review of four others); safety vectors once 071 lands. No new provider or scenario crate until 080 is merged and one merged provider has passed its suite through the engine.

(c) No lane until the rerun; one coordinator.

Recommendation: (b). The lane's proof work changed the specification (eight findings, three specification orders). Its provider work is ahead of anything that can exercise it, and three orders (129, 130, 141) were already rework after the specification moved under the kit. More crates before the bar is judged risks more of that. Under (b) the lane still runs in parallel and the admin's time goes to batch 1.

This can be a dated later answer under ruling 71; it needs no new ruling.

**Evidence.** docs/implementation/HANDOVER.md:97-117 (restarting the lane), :222 (the lane's queue); docs/implementation/PROVIDERS.md:44-63 (queue in order); experiments/022-provider-contracts/suites/*/index.yaml `cases: []`; HANDOVER.md:212 (080 checkpointed with three musts); docs/work-orders/122-a63-rk1-runtime-kit-server.md:194-196 ("the kit's choices until 081 settles them"), 129, 130 and 141 Records; docs/work-orders/116-mc8-g5-account.md:160-176; docs/proposals/ruling-71/README.md:22-29.

**Waits.** The brief for the new lane coordinator and the lane's first orders. The CI fix and 071 do not wait.

**Verifier.** partly. HANDOVER.md:97-117 and docs/implementation/PROVIDERS.md:36-63 (the lane stopped; its queue). Suites empty; 080 checkpointed with three musts (HANDOVER.md:212). 129-a63-rk-owed-stop-first.md:98-100 and the Records of 130 and 141. Ruling 71's record takes dated later answers, so no new ruling is needed.

**Verifier's counterargument.** Three points. (1) Who decides: the admin has set the lane's dispatch order before without the owner (121-plans-follow-up.md:392-393, 'Dispatch now follows the admin's order'). Only option (c), and the usage a second coordinator spends, are the owner's; the choice between (a) and (b) is one a coordinator may settle and log. (2) Option (b) is thin. In PROVIDERS.md's orders table the runtime suite waits on A63-H2a (an owner step, separate OS users) and SV1 (after 071), and the machine suite on 049a1 and 049b. What can start is the CI fix, the package-source and secret-store suites (both size S), the MC2 rerun, MC4's re-registration and the review of four models: about a day at the lane's rate, after which (b) becomes (c) until the admin lands 071 and 080. (3) A missing option: batch 1's own plan gives 043a, 043b, 050, 051a, 051b, 052a, 052b and 053 to 'other lanes' (docs/proposals/spec-batch-1/README.md:104-105), and six of them are unwritten on the rerun path. A lane coordinator may write and dispatch them with the admin merging (ruling 71, point 1). That puts the second coordinator on the path to the rerun. They touch the hot files, so 071 must land first.

**Verifier's corrections.** 129 and 130 implement the owner's answer of 2026-11-12 on MC4-002, a requirement that arrived after the kit; only 141 follows text that moved (081). The stronger fact is that MC4-002 was already open (108 merged 15:35 on 2026-11-11) when 122 was dispatched at 20:30. 'Passed its suite through the engine' joins two things: the harness drives a provider directly (experiments/022-provider-contracts/README.md:3-6); 080 is the engine calling one.


## FR-267

**The table: 125 orders, their rounds, musts and outcomes**

P01 I4 · fact · verifier: partly · decides: none

125 orders were written (75 admin, 50 lane). 116 merged to main and one (079) to the side branch. Four are checkpointed with musts left (049a1, 071, 080, 144), three are held for order 050 (050a, 076, 078), and 147 was never dispatched.

Rounds per order, for the 121 with a recorded run: one round 41, two 72, three 2, four 4, five 2. Counting a finishing order with the order it finishes (112 pieces of work): 35, 61, 7, 6 and 3, so 16 (14%) needed more than two.

Ended their own rounds with musts or faults left: 36 of 121 (30%). Admin 001 to 066: 11 of 45, nine of them one-round paper or tool orders the admin corrected by hand. Admin 037 to 082: 8 of 27. Lane: 17 of 49.

Needed a coordinator's fix or a further order: 35 of the 117 merged (30%). 25 coordinator fixes (22 after a verifier, 3 after the Mac run) and 10 that needed a finishing, follow-up or redesign order.

By kind:

| Kind | Orders | Over two rounds | Musts left at the end |
| --- | --- | --- | --- |
| Engine | 32 | 022, 041b, 042, 049a1, 070 | 7 |
| Specification | 12 | 038b, 077 | 4 |
| Paper | 31 | 069 | 9 |
| Harness | 23 | none | 7 |
| Package | 23 | none | 9 |

The lane never exceeded two rounds because its runner stops there; it wrote nine finishing orders instead.

What converges: results and paper orders; specification orders where the coordinator wrote the settlements out (075, 081, 082: one round each, clean); small code orders; a mass change made by a script the verifier reruns (041a, 29,517 vector changes, one round). What does not: engine orders that bundle several behaviors, new concurrent provider code, and code above about 2,000 changed Rust lines (I5).

Recorded worker time: lane 11.4 hours over 40 orders (median 14 minutes); admin 037 to 082, 54.6 hours over 22 orders (median 105 minutes).

**Evidence.** Built from docs/implementation/WORK_ORDERS.md:185-222 (table) and :224-1372 (Records), and the `## Record` section of each of 72 files under docs/work-orders/; 049a1, 071, 080 and 144 from docs/implementation/HANDOVER.md:211-214. Legend: id; kind S spec, E engine, K package, H harness, P paper; machine M Mac, W Windows, WM both, M* Mac through Conduit, ? not recorded; rN rounds; m musts (or faults of unstated severity, 020-066) per verifier pass, ? not itemized; outcome; w worker seconds recorded.
001 S M r1 m1 fix w1260; 002 H M r2 m5 clean w1020+1500; 003 P M r2 m2 clean w1020+300; 004 E M r2 m2 clean w1020; 005 P M r1 m0 clean w660; 006 E M r2 m4 clean w1080; 007 H M r1 m0 clean w480; 008 P M r1 m0 clean w420; 009 E M r2 m3 clean w1380; 010 H M r2 m0 clean w780; 011 P M r1 m0 clean w600; 012 E M r2 m9 clean w1500; 013 E M r2 m4 clean w840; 014 E M r2 m5 clean w1320; 015 E M r2 m6 clean w1560; 016 E M r2 m8 clean w1380; 017 E M r2 m3 clean w2460; 018 P M r1 m1 fix w480; 019 E M r2 m3 clean w960; 020 E M r2 m3 clean w2220; 021 E M r2 m5 clean w2940; 022 E M r4 m3/4 clean w2880; 023 K M r2 m4 clean w1920; 024 E M r1 m0 clean w1920; 025 E M r2 m4/3 follow-up 027 w3660+2400; 026 P M r2 m1 clean w540; 027 E M r2 m1 clean w2580; 028 E M r2 m2 clean w3000; 029 P M r2 m3 clean w840; 030 H M r2 m2 clean w1320; 031 H M r1 m0 clean w840; 032 S M r1 m2 fix w1800; 033 E M r1 m2 fix w1260; 034 P M r1 m4 fix w1320; 035 P M r1 m0 clean w600; 036a S M r2 m4 clean w1320; 036b S M r2 m4 clean w1740; 036k H M r1 m1 fix w480; 037 S M r2 m5 clean w1920; 038a S M r2 m4 clean w1920; 038b S M r3 m2/?/1 fix w5160; 038c S M r2 m?/0 clean+rebase w5940; 039 E M r2 m1 clean w2760; 040 E M r2 m2 clean w6720; 041a E M r1 m0 clean w8280; 041b E M r3 m3/2/1 fix w9780; 042 E M* r5 m?/?/?/0/1 fix w11966; 044 E M r2 m2 clean w4560; 045 E M r2 m2 clean w4320; 049a1 E ? r5 m?/?/?/?/1 checkpoint; 050a K held for 050; 060 P M r1 m1 fix w960; 061 P M r1 m3 fix w1260; 062 P M r2 m2 clean w840; 063 P M r1 m6 fix w1200; 064 P M r2 m2 clean w840; 065 P M r1 m2 fix w720; 066 P M r1 m0 clean w720; 067 P M r2 m1 clean w540; 068 H M r2 m?/2 fix w17160+9360; 069 P M r4 m2/1/7/0 clean w1440; 069b P W r1 m0 clean w480; 070 E WM r4 m?/?/1/0 clean w18000; 071 E W r2 m?/1 checkpoint; 072 E WM r2 m?/0 clean; 073 P ? r2 m?/0 clean w660; 074 E M r1 m0 clean w28800; 075 S W r1 m0 clean w12600; 076 K held for 050; 077 S WM r4 m?/?/1/? fix w19800; 078 K held for 050; 079 E WM r2 m?/0 clean (s3/m2) w12600; 080 E W r2 m?/3 checkpoint; 081 S WM r1 m0 clean w1200+10440; 082 S W r1 m0 clean; 100 P M r2 m5/3 fix w1526+187; 101 P M r2 m0/0 clean w721+134; 102 P M r2 m0/1 fix w920+136; 103 P M r1 m0 clean w857; 104 H M r1 m0 clean w740; 105 H M r2 m1/0 clean w1013+319; 107 H M r2 m1/0 clean w1312+270; 108 H M r2 m4/2 fix w1897+1087; 109 H W r1 m0 clean w842; 110 H W r2 m0/0 clean w?+273; 111 H W r2 m4/4 finish 114 w?+825; 112 H W r2 m2/0 clean w?+477; 113 H W r2 m1/2 finish 115; 114 H W r1 m0 clean; 115 H W r1 m0 clean; 116 P W r1 m0 clean; 117 P M r1 m0 clean w1049; 118 P M r2 m1/1 fix w1218+260; 119 P M r2 m0/0 clean w1053+92; 120 P M r2 m1/0 clean w935+88; 121 P M r2 m1/0 clean w1184+104; 122 K M r2 m4/3 finish 124 w1312+333; 123 H M r2 m6/3 finish 125 w1453+431; 124 K M r2 m0/1 fix w445+187; 125 H M r2 m1/0 clean w565+252; 126 K M r2 m0/0 clean w?+275; 127 K M r2 m2/0 clean w?+661; 128 K M r2 m1/1 fix w?+234; 129 K M r2 m1/1 finish 130 w?+493; 130 K M r1 m0 clean; 131 K W r2 m1/0 Mac fix; 132 K W r1 m0 Mac fix w690; 133 K M r1 m0 clean; 134 P M r1 m0 clean; 135 H M r2 m0/0 clean w?+135; 136 K M r2 m2/2 fix w?+281; 137 K W r2 m4/2 finish 140 w2121+535; 138 K M r2 m0/0 clean w647+278; 139 K M r2 m2/0 clean w1089+250; 140 K W r1 m0 Mac fix w842; 141 K M r1 m0 clean w606; 142 H M r2 m3/3 finish 148 w1371+392; 143 K M r1 m0 clean w1422; 144 K ? r2 m?/2 checkpoint; 145 K W r1 m0 clean w818; 146 K W r2 m1/1 finish 149 w1198+289; 147 K not dispatched; 148 H M r1 m0 clean w385; 149 K W r2 m1/1 redesign 150 w505+410; 150 K W r1 m0 clean w648.
Caveats: 038c's Record reads "then two rounds" and may mean three; times for later rounds are mostly not recorded; 144's Record is on its branch, not on main.

**Verifier.** partly. Parsed the analyst's table by script and checked each row against its Record (docs/implementation/WORK_ORDERS.md:224-1372; the 72 `## Record` sections). Every count reproduces: 125 orders (75 and 50); 116 on main and 079 on s3/m2; rounds 41, 72, 2, 4, 2; by piece of work 35, 61, 7, 6, 3; 36 of 121 with musts left (11 of 45, 8 of 27, 17 of 49); 35 of 117 needing a fix or a further order (22, 3 and 10); the by-kind table; 11.4 h over 40 lane orders and 54.6 h over 22 admin orders.

**Verifier's counterargument.** The headline share misleads. 33 of the 121 have no verifier after their last worker round: 26 of orders 001 to 066, and 037, 038a, 039, 040, 044, 045 and 067. 'Clean' in their rows means unverified. Over the 88 orders whose final state a verifier read, 36 ended with musts left: 41%, not 30%. By kind: engine 7 of 13 (54%), specification 4 of 8, paper 9 of 25 (36%), harness 7 of 20, package 9 of 22. Engine is then the worst kind, where the item's table shows it best (7 of 32), and 'paper orders converge' is not shown.

**Verifier's corrections.** Ten, not nine, one-round orders of 001 to 066 were corrected by the admin (001, 018, 032, 033, 034, 036k, 060, 061, 063, 065). The lane's 14-minute median mixes in orders with only a round-two time; first-round medians are 15.6 minutes for the lane (n=31) and 92.5 for the admin's later orders (n=22). 050a, 076 and 078 did run (one worker commit each under origin/wo/078-agents-codex-path-item) and have no Record.


## FR-268

**The six suspected causes, ranked by what the Records show**

P01 I5 · assessment · verifier: partly · decides: none

1. **Size: the strongest signal.** Lane code above 2,000 changed Rust lines (tests included) still had musts at the second pass in 7 of 9 pieces of work, and none was clean in one round. At or below 2,000 it was 4 of 19; at or below 1,000, five of nine were clean in one. The three engine orders left unmerged are 3,911 (049a1), 4,314 (080) and 3,059 (071) lines. 042 asked for five behaviors and about 36 vectors in one step and ran five rounds. Engine and specification orders with at most about 15 hand-written vectors all finished in two rounds (039, 040, 044, 045, 075, 081, 082). The plans' size estimates were low by 2.2 to 6 times (RK1 planned at 650 to 850 lines, merged at 2,913; SCR1 350 to 550, merged at 2,147).

2. **Fixes aimed at the instance, not the class.** GOARCH names in 128, classifications in 100, the delivery path in 118, and redaction faults in five orders (123, 125, 136, 142, 148). In 014 the admin had the worker search for every other instance after the first fix, and it found a sixth hole.

3. **A mechanism that is wrong, not code that is wrong.** Hand-built locking and handoff: 122 and 124, 129 and 130, 146 and 149 (four passes, a race in each, replaced by 150 in one clean round), then the CI failure after 150.

4. **Moving bases.** At least nine orders paid for a rebase or a stack (022, 038c, 040, 042, 049a1, 075, 077, 080, 137): an extra round in 022 and 042, hand recounts of the promise totals three times, one revert. Since 2026-11-11, 12 commits touched the manifest, 17 spec/README.md and 25 IN_FORCE.md.

5. **Ambiguous or missing specification text** shows up as worker findings, coordinator decisions and rework orders, rarely as repeated musts. The kit made its own choices "until 081 settles them", and 129, 130 and 141 reworked it.

6. **The verifier outside the order: once** (077's third pass, a vector of 041b that the new text made wrong). That was a scope fault of the order, not a wandering verifier.

Load and slow tests cost hours rather than musts: 074 took 8 hours for 150 lines, 068 7.4 hours, 075 3.5 hours with no engine code changed. 070 removed most of that. A skeptic before dispatch is I10.

**Evidence.** Sizes: `git show --numstat --format= &lt;merge commit>` over .rs files (122+124 38c0b4f7 +2913; 123+125 752b4830 +2733; 126 c9b6240c +2156; 127 48c76a79 +2125; 137+140 6ce8d136 +4404; 142+148 f266adfb +2147; 128 952e9f51 +981); unmerged branches, outside the pinned commit: `git diff --numstat $(git merge-base origin/wo/&lt;branch> 2eb8f258) origin/wo/&lt;branch>`; 049a1's rounds added 2381, 434, 272, 409 and 551 Rust lines. Estimates: docs/implementation/PROVIDERS.md:113, :121-123, :130, :133; experiments/015-s4-partial-setup/plan.md:86. docs/work-orders/042-build-tombstones-and-inventory.md:20-30. docs/implementation/WORK_ORDERS.md:552 (014's search). docs/work-orders/149-ssh-r-drainer-finish.md:82-85; docs/work-orders/150-ssh-r-one-drainer.md:97-104. Rebases: docs/work-orders/038c-spec-sign-in-return.md:666-675; docs/implementation/HANDOVER.md:209, :211; docs/implementation/WORK_ORDERS.md:126. Hot files: `git log --oneline --since=2026-11-11 -- conformance/manifest.yaml | wc -l` (12), spec/README.md (17), IN_FORCE.md (25). docs/work-orders/122-a63-rk1-runtime-kit-server.md:194-196. docs/work-orders/077-spec-reconcile-fenced.md:592-596. Load: docs/work-orders/074-shell-session-idle-limit.md:82-84; 068:101-102; 075:629.

**Verifier.** partly. Sizes reproduced with `git show --numstat` over .rs files: above 2,000 lines, 9 pieces, 7 with musts at the second pass; at or below 2,000, 19 pieces, 4; at or below 1,000, 9 pieces, 5 clean in one round (Fisher's exact p about 0.01). 049a1: 3,911 lines, rounds of 2381, 434, 272, 409 and 551. 071: 3,059 (+2,411 -648, a file split). Rebases and stacks read in the Records named.

**Verifier's counterargument.** Size is confounded with kind. The group above 2,000 is model-checking models and new concurrent servers; the group at or below 1,000 is mostly small changes to an existing crate (129 with 130, 138, 141, 145). Cause 3 (a wrong mechanism) and size cannot be told apart on 28 pieces, and the threshold was chosen after seeing the data. Sizes are taken at merge, after fix rounds and finishing orders. At round one, where the branches survive, 113 was 1,982 lines (win/wo/113-mc7b-records-and-packages, 32df5e18), 146 was 1,199 and 144 was 1,542: all under 2,000 and none converged.

**Verifier's corrections.** 080 is 2,889 lines of its own; 4,314 includes order 079's commit 4c8dc763 (1,427) beneath it, so 'all above 3,000' fails. 144 (1,674 lines, two musts left) is missing from the lower group: 5 of 20. Every admin order that ran past two rounds and merged is small in Rust: 041b 878, 042 986, 070 1,431, 077 440; their bulk is vectors (042: 46,118 conformance lines). 'At most about 15 hand-written vectors all finished in two rounds': 039 wrote 14 and re-expected 6, and 039, 040, 044 and 045 record no verifier after round two. Hot-file commits since midnight on 2026-11-11 are 12, 19 and 26; the command as written takes the current time of day. The verifier went outside the order twice: also 033's tracked bytecode files.


## FR-269

**A size limit, set on scope and checked on the round-1 diff**

P01 I6 · recommendation · verifier: partly · decides: coordinator

Three rules:

1. **One behavior per order.** One promise family or one provider operation set, with its tests. An order whose "What to do" names several features (042 named tombstones, lift, LIF-041, the inventory, ACC-055, ACC-044, listing, the runner's projections and inspection) is split before dispatch.

2. **A vector budget for engine and specification orders.** About 15 vectors written or re-expected by hand. A mass change goes through a migration script that the verifier reruns, as 041a's did.

3. **A check after round 1, because estimates cannot hold the limit.** If the worker's diff is over about 2,000 changed Rust lines, the coordinator splits it before any second round: merge the part that stands alone, write a new order for the rest. The target is about 1,000.

What it costs: more orders, each with its fixed overhead (the order, a verifier of about five minutes, a review, a pull request, CI). The lane's small orders had a median of 39 minutes from written to merged. It also means more stacking between parts, which needs I11.

What it buys: the group that finishes. At or below 1,000 lines, five of nine lane pieces were clean in one round and two had musts at the second pass; above 2,000, none and seven of nine. Finishing orders, which are small by construction, were clean in one pass in six of nine.

The thresholds come from 28 lane code pieces and about ten engine orders; treat them as a starting point and let I9's record correct them.

**Evidence.** Counts from I4's table and I5's sizes. docs/work-orders/042-build-tombstones-and-inventory.md:20-30 (one Build step, "about 36" vectors) against its five rounds (:47-62). docs/work-orders/041a-build-defaults-and-owners.md:66-69 ("29,517 changes ... the verifier reproduced the migration exactly"). Vector counts: 039 Record (14 added, 6 re-expected, two rounds), 040 (10 added, 3 flipped, two rounds), 041b (157 declared changes, three rounds and a fix), 077 (123 declared changes, four rounds), 081 (2 vectors, one round), 082 (1, one round). Lead time: `git log` timestamps as in I2. Finishing orders: 114, 115, 130, 140, 148, 150 clean in one pass; 124, 125, 149 not.

**Waits.** The next orders: 046, 047 and the finishing orders for 071, 080 and 049a1

**Verifier.** partly. As for I5. docs/work-orders/041a-build-defaults-and-owners.md:66-69 and 042-build-tombstones-and-inventory.md:20-30 read as quoted. Finishing orders: 114, 115, 130, 140, 148 and 150 clean in one pass; 124, 125 and 149 not. No ruling in force is contradicted, and the coordinator decides.

**Verifier's counterargument.** Rule 3 would not have caught the cases that cost most. No admin order that ran past two rounds and merged exceeds 1,500 Rust lines, and 113, 144 and 146 were under 2,000 at round one. It would have caught 111, 137, 049a1, 080 and 071. So rules 1 and 2 carry the engine orders, and rule 3 fits new crates only. A limit on lines that include tests rewards fewer tests (139: 531 implementation lines, 1,011 test lines; 143: 666 and 959): count non-test lines or operations. Splitting after round one spends the coordinator's attention, which is what ran out, and makes stacks that I11 limits. A cheaper form: split by behavior when the order is written, and for a new crate order the skeleton with one operation first. 'Finishing orders clean in six of nine' does not show that small diffs are right: a finishing order's verifier checks that order's decisions only (docs/work-orders/verify.md:3-6), and 140 then failed 20 tests on the Mac (140-ssh-r-finish.md:135-137) while 150's test fails on CI (HANDOVER.md:198-203).

**Verifier's corrections.** The '15 vectors' evidence rests on seven orders, four of them with no verifier after round two. The 39-minute median is the lane's whole, not its small orders'.


## FR-270

**No raw third round: after a second pass with musts the coordinator reads, then chooses one of three; every coordinator fix gets a fresh read**

P01 I7 · recommendation · verifier: partly · decides: coordinator

Keep two verifier passes per order. After a second pass with musts, the coordinator reads the findings and the branch and records one of:

- **Same class as the first pass: the mechanism is wrong.** Write an order that replaces it. 146 and 149 spent four passes on drainer handoff, each finding a race; 150 replaced the handoff in one clean round of 648 seconds. 130 did the same for 129's admission lock.
- **New, local and small: the coordinator fixes it, and a fresh reader checks the fix before merge.** Of eight lane fixes a fresh reader checked, the reader found more to fix in six (102, 108, 124, 128, 131, 136) and nothing in two (132, 140). In 131 the reader overturned the coordinator's diagnosis. The admin's own fixes (038b, 041b, 042, 068, 077) record no fresh read.
- **Otherwise: a finishing order** that names each finding and its class, and tells the worker to search for every other instance of the class.

A piece of work that reaches a fourth pass stops and is split or redesigned, with the reason in its Record.

Evidence that this beats another round: finishing orders were clean in one pass in six of nine; raw third and later rounds found new faults in at least 9 of 13. 049a1's fix rounds grew rather than shrank (434, 272, 409, 551 Rust lines), over 13 hours, and it is still unmerged. 042 took 33 hours from written to merged.

Cost: one coordinator read per order that does not converge, which the lane already practised. The handover states the rule ("Don't start finishing orders blindly"); this makes it the only path after pass two, for the admin's orders too.

**Evidence.** docs/implementation/HANDOVER.md:302-305; docs/work-orders/146-ssh-r-drainer.md:143-147; docs/work-orders/149-ssh-r-drainer-finish.md:82-85; docs/work-orders/150-ssh-r-one-drainer.md:97-99; docs/work-orders/130-a63-rk-owed-stop-first-finish.md:98-103. Fresh readers: docs/work-orders/102-pre-b-s4-s5-s6-s9.md ("A fresh reader then checked these edits. It found one must"), 108 ("Its one remaining finding"), 124 (two shoulds), 128 (two shoulds), 131-a63-ps-local-directory.md:126-128 ("a fresh reader showed that diagnosis wrong"), 136 (two more faults), 132 and 140 (nothing). Admin fixes: Records of 038b, 041b, 042, 068, 077. 049a1: `git log` and `git diff --numstat` on origin/wo/049a1-build-enrollment-core (rounds committed 2026-11-11 20:48 to 2026-11-12 09:39), outside the pinned commit. 042 lead time: order file added 920b0769, Record in 57813f06 (2,005 minutes).

**Waits.** The finishing orders for 071 (one must), 080 (three), 049a1 (one must, one should) and 144 (two)

**Verifier.** partly. Fresh readers of lane fixes: found more in six (102:205-209, 108:258-260, 124:121-123, 128:150-152, 131:125-128, 136:148-150), nothing in two (132:133, 140:140-141); 100 and 118 record no reader. The admin's fixes in 038b, 041b, 042, 068 and 077 record none. 049a1's rounds were committed from 20:48 on 2026-11-11 to 09:39 on 2026-11-12. 150-ssh-r-one-drainer.md:97-98 and 130's Record read.

**Verifier's counterargument.** The Records carry 'read and classify before another round', 'the same class twice means the mechanism is wrong' (146, 149, 150; 129, 130) and 'a fresh read of every coordinator fix'. They do not carry a ban on a third round or a stop at the fourth pass: the fourth pass was clean in 3 of 4 orders (042, 069, 070), and six of the seven admin orders that went past two rounds merged. A cap at four would have stopped three orders one pass before clean. 070's fourth round already followed a coordinator decision (070-build-state-copies.md:354-362). Finishing orders against raw rounds is confounded three ways: lane against admin, new crates against the engine, and the verifier's scope (the finishing order's verifier reads only the finishing diff). An alternative that follows from I1: two independent verifiers on round one for engine and specification orders, about ten minutes, with the class-search instruction of order 014 (WORK_ORDERS.md:552-554) in round two.

**Verifier's corrections.** The second branch is not new for the lane: IN_FORCE.md:308 already lets a lane merge only when 'a fresh verifier is clean', and 100 and 118 merged after a coordinator fix with no fresh read. HANDOVER.md:91-95 cites ruling 71, point 4 for fixing by hand, but point 4 delegates small findings that add a feature; the leave to finish by hand is WORK_ORDERS.md:84-85. The item should say so. Of 042's 33 hours, at least 13.6 passed before 041b, which it builds on, had merged.


## FR-271

**Make CI the gate: required checks, engine shards that a package crate cannot skip, a macOS leg, repeated runs for timing tests, and no local squash**

P01 I8 · recommendation · verifier: partly · decides: coordinator

"Once CI is green" is kept by hand, and it failed three times in 26 hours (#205, #260, and 077 merged with the engine shards skipped). On 2026-11-11 the lane was also merging while CI was red or paused. Five changes:

1. **Required status checks.** Branch protection today is pull requests only. Make `docs` and `rust` required on `main`. This is a repository setting on the owner's account: one line in the message to the owner, not a ruling.

2. **Decouple the engine shards.** `engine-tests` needs the whole `rust-dir` matrix, so one failing test in any of 17 package or experiment crates skips every engine shard. Build the engine in its own job and let the shards need only that; the summary job `rust` still requires everything. This is why a lane test stopped the engine's gate.

3. **A macOS leg for `packages/`.** Every job runs on `ubuntu-latest`. Three of the five package pieces built on Windows failed on the Mac after a clean verifier (131 and 132 did not compile; 140 failed 20 of 33 tests), each fixed by hand. The opposite also happened: 150's test passes on the Mac and fails on CI's Linux. A matrix leg replaces the manual Mac run. Confirm that standard macOS runners are free for this public repository.

4. **Repeat timing tests before merge.** A new or changed test that uses locks, timers or processes runs many times (say 20) in its pull request. The handover already asks the CI fix to be "stable over three CI runs".

5. **Stop squashing locally.** The guard exists because `git reset --soft origin/main` on a moved ref deleted another order's files (#190). Push the branch as it is, check scope with a diff from the merge base, and let GitHub's squash merge apply it against the current `main`, with the hand-written message passed to it. The whole class goes away.

Cost: one small order for 2 to 4, a setting for 1, a runner change for 5.

**Evidence.** docs/proposals/ruling-71/README.md:26-27; docs/proposals/ruling-74/README.md:26-29 (pull requests only, no force-push; no status checks named). .github/workflows/checks.yml:187-189 (`engine-tests: needs: rust-dir`), :223-225, :252-254; `runs-on: ubuntu-latest` at :27, :82, :110, :190, :226, :256; `ls packages/*/Cargo.toml experiments/*/Cargo.toml | wc -l` gives 17. docs/implementation/HANDOVER.md:81-84, :198-205, :291-295. docs/work-orders/131-a63-ps-local-directory.md:123-125; 132-a63-fs-file-store.md:131-133; 140-ssh-r-finish.md:135-137. CI red or paused on 2026-11-11: docs/work-orders/068-test-gate-and-ci.md:12 ("CI on `main` is red"), 104-mc1-model-checking-harness.md:202, 105-mc2-operation-identity.md:286-287. `git log -1 --format=%B ed8043a1`, `128ca08a`, `575a1bcd`.

**Waits.** The first fix in the handover's queue (CI); every later merge. Item 1 needs the owner's hand on the repository setting.

**Verifier.** partly. .github/workflows/checks.yml:187-189, :223-225 and :252-255 read; six `runs-on: ubuntu-latest`; 17 crates under packages/ and experiments/ at one level. 131:123-125, 132:131-133 and 140:135-137 confirm three of five Windows-built package pieces failing on the Mac. `git log -1 ed8043a1` confirms the squash. docs/proposals/ruling-74/README.md, decision 1, names no status check. 068-test-gate-and-ci.md:51 calls the repository public.

**Verifier's counterargument.** Items 1 to 3 stand. Item 3 should include the engine's loader, core and conformance suites: nine engine or specification orders ran at least partly on WSL (070, 071, 072, 075, 077, 079 to 082), and their Mac runs were by hand (075:637-638, 082:734). Item 4: twenty repeats on an idle runner will not reproduce a race that needs load; use nextest's repeated run for the changed crate, in keeping with 068's 'no retries anywhere', and test a lock handoff deterministically, as the project's own Stateright harness in experiments/012 does for the promises. Item 5 is mechanically right, but the one-command guard has held since #190, so its gain is small.

**Verifier's corrections.** There are two distinct failures on main since #198 (the docs job, #205; the SSH-R test since #260); 077's merge on local evidence is a consequence of the second. 'Branch protection today is pull requests only' cannot be read from the tree; it is inferred from ruling 74 and from 077 merging while `rust` failed. Missing from the item: the matrix glob at checks.yml:94 is one level deep, so experiments/013-s2-shared-login/linear-stub and experiments/015-s4-partial-setup/scripted-client (orders 135, 142 and 148) are never built or tested by CI.


## FR-272

**A77: give it a pass condition and a record a script can read, including the coordinator's own spend**

P01 I9 · experiment-change · verifier: partly · decides: coordinator

**Row:** A77 in ROADMAP.md. It asks whether Codex's work is as good and whether the admin spends less. Its measure is each order's row, "reviewed after five orders". It has no pass condition and no baseline, so it can neither pass nor fail, and 117 orders have merged without the review. Admin time was never recorded. The one fact on spend is that the coordinator's usage ran out on day four and stopped both the admin and the lane.

**Claim to test instead:** orders kept under the size limit finish in two verifier passes without a coordinator fix, do not break `main`, and cost a known amount of coordinator effort.

**Pass condition, fixed before the next order:** over the next 20 merged orders, at least 16 end within two passes with no coordinator fix or finishing order; no merge leaves CI red; every field below is filled for all 20. The baseline from this review is 59% for the lane (23 of 39) and 63% for the admin's later orders (17 of 27).

**Record for each order, as a small YAML block the runner writes and the coordinator completes:**
- kind, lane, machine, base commit, what it is stacked on;
- times: written, dispatched, each worker and verifier round, review, merged;
- the coordinator's spend per phase (write, review, fix, merge), in tokens or minutes;
- each verifier pass: musts, shoulds, nits; each must's class (order fault, specification gap, implementation, test weakness, out of scope, environment); accepted or rejected;
- the pre-dispatch reviewer's findings and how many changed the order;
- changed lines by class (Rust, tests, specification, vectors);
- rebases and conflicts; coordinator fixes and whether a fresh reader checked them;
- CI's result at merge, on Linux and macOS;
- escapes: faults found after merge and traced back to the order.

Today the Records hold worker time for first rounds, verifier severity for about half the orders, and nothing on rejected findings or escapes. The lane's Records, with seconds and musts per pass, are the better model than prose.

**Evidence.** ROADMAP.md:70 (A77's row; "admin time was not recorded, so no complete spend comparison is available"); ROADMAP.md "Proposing an experiment" ("Each experiment states, before it runs, what it must show and how each measure is counted (ruling 45)"); docs/implementation/WORK_ORDERS.md:156-160; docs/implementation/HANDOVER.md:4-5 ("because the owner's usage was near its limit"), :19; docs/implementation/PROVIDERS.md:36. In WORK_ORDERS.md:185-222, 14 rows say "rounds not recorded" and rows 020 to 065 give no must, should or nit counts. Baselines from I4's table. Escapes that exist but are not tallied: finding 4-19 (docs/proposals/first-build/log-increment-4.md:30, order 040's list.v1 refused by the shell), three bytecode files with local paths tracked since order 032 (WORK_ORDERS.md, order 033's Record), the CI failure after 150.

**Waits.** A77's done-when; the evidence for I6's thresholds and I10

**Verifier.** partly. ROADMAP.md:70 read: no pass condition, and 'admin time was not recorded'. The baselines reproduce from the table: 23 of 39 lane pieces and 17 of 27 admin orders ended within two passes with no coordinator fix or further order.

**Verifier's counterargument.** The pass condition counts a coordinator's fix as a failure while I7 recommends that fix for small faults: the two pull against each other, and the measure rewards not looking. Count faults found after merge and a red main instead. 'The next 20 merged orders' leaves out the ones that never merge, which are the costly ones (049a1): count dispatched orders. The item replaces A77's question (ruling 48: is the work as good, and does the admin spend less) and does not answer it. 'As good' can be measured by escapes; 'spends less' needs the coordinator's tokens for each order, which only the session's transcript holds, outside the repository: a script over the transcript, not a field filled by hand. The proposed block has some 25 fields, and filling them falls on the coordinator. A smaller record: written by the runner, the times of each phase, musts for each pass, changed lines and CI's result; written by hand, each must's class and the escapes.

**Verifier's corrections.** '117 orders have merged without the review': lessons were recorded (WORK_ORDERS.md, 'What this round taught', and the row's own state); what is missing is a verdict and the spend. Usage was 'near its limit' (HANDOVER.md:5), not run out. The admin baseline of 63% includes seven orders with no verifier after round two; like for like it is 10 of 20 (50%). A change to the question A77 settles should be told to the owner, since its source is ruling 48.


## FR-273

**A reviewer before dispatch: the Records show no effect on rounds, and never say what it found**

P01 I10 · assessment · verifier: supported · decides: none

The lane had a fresh reviewer read 47 of its 50 order files before dispatch (all but 114, 115 and 134). The admin did so for four of its 30 later orders (070 to 073).

Outcomes do not separate. Lane orders ended their second pass with musts in 17 of 49 cases (35%). The admin's later orders without a reviewer did so in 7 of 23 (30%). Of the four admin orders with one, 070 ran four rounds, 071 is unmerged with a must, and 072 and 073 took two. Lane code orders had no must on the first pass more often (11 of 28) than engine orders (3 of 25 where the count is known), but those are new standalone crates against a core checked by 519 promises, so the comparison is not like for like.

The reason is in the kind of fault. Most musts are implementation faults (races, redaction, edge cases) that no reading of the order could catch. A reviewer of the order can catch order faults, and those exist: 020 ("no exp" against DL13), 023 (an input MAN-017 forbids), 070 (two demands that could not both be met, found in round three), 107 (properties worded too broadly), 061 and 062 (a guard that stopped the first dispatch), 081 (two dispatches failed for a missing schema and a leftover branch).

No Record says what a reviewer found; commit messages say only "a fresh review applied". The one visible case is 141, where the review changed 71 lines of the order and raised two extensions the admin decided.

Judgment: keep the reviewer for orders that state decisions or readings (specification, engine, a new provider), skip it for finishing orders, and record its findings (I9) so the next review can measure it. Do not expect it to reduce rounds.

**Evidence.** `git log -1 --format='%s %b' $(git log --reverse --diff-filter=A --format=%h -- &lt;order file> | head -1)` for each order file: "A fresh reviewer read it/them", "fresh skeptic reviewed" or "fresh review(s) applied" for 47 lane files; none for 114 (d3ee4fec), 115 (e9141e6d), 134 (311ecffc); for admin files only 1fb3a48f ("Order files, each reviewed by a fresh skeptic", 070 to 073). Outcomes from I4's table. Order faults: docs/implementation/WORK_ORDERS.md order 020's Record ("The third was the admin's: this order said 'no exp', against DL13"), order 023's, 061's and 062's; docs/work-orders/070-build-state-copies.md:354-356; 107-mc3-access-over-time.md ("which is the coordinator's fault, not the spec's"); docs/implementation/HANDOVER.md:305. 141: `git show --stat 28b8ca61` (45 insertions, 26 deletions) and a9b81be1.

**Verifier.** supported. Reran the commit-message check on the commit that added each order file: 47 of 50 lane files name a fresh review; 114 (d3ee4fec), 115 (e9141e6d) and 134 (311ecffc) do not; for the admin only 1fb3a48f (070 to 073). Lane code with no must on the first pass, 11 of 28; engine, 3 of 25 where the count is known. The order faults cited were read in the Records of 020, 023, 061, 062, 070 and 107.

**Verifier's counterargument.** The case against expecting much is stronger than stated: the two order faults that arose under a reviewer were not caught by it (070-build-state-copies.md:354-356, two demands that could not both be met; 107-mc3-access-over-time.md:242-249, properties worded too broadly). Only 141 shows a review changing an order.

**Verifier's corrections.** There is a second visible case, and a large one: order 121 is the accepted findings of the four reviews of the plans' splits, and they added three admin orders (121-plans-follow-up.md:14-16, :388-390). '7 of 23' for the admin includes seven orders with no verifier after round two.


## FR-274

**Take the conflicts out: land 071 next, check the totals by tool, keep stacks one deep, and write the promise before the provider code**

P01 I11 · recommendation · verifier: partly · decides: coordinator

1. **071 is the second fix, after CI.** It splits the three files nearly every build order edits (the 8,333-line manifest, the case schema, the runner's one function) and was written for exactly this. It is checkpointed with one must. It is 3,059 Rust lines, so finish it by I7 (a finishing order for the one finding), not a third round. Until it lands, every engine and specification order rebases through the manifest; 049a1's rebase already conflicts there.

2. **Check the promise totals by tool.** The totals in spec/README.md were recounted by hand at three rebases (516, 517, 519), once "correcting two figures that were already stale on main". No tool compares them with the promise files. Add the comparison to the schema check, or generate the sentence.

3. **One unmerged order under any order.** Today 078 sits on 076 on 050a, all held for order 050, which is unwritten; 080 sits on 079's branch; 079 lives on the side branch `s3/m2`, where 049a1 and 080 are also bound, and 049a1's rounds ran from `main` instead. Each day these live they diverge further. Re-examine whether `s3/m2` is needed: it is a coordinator decision on the owner's strike list.

4. **Promise first, provider code second.** The specification alone binds, yet the kit was built before its rules were written: 122 made its own choices "until 081 settles them", 129, 130 and 141 reworked it after the owner's answer and 081, and PRV-021 still lacks the three cases the kit implements. A lane order that implements a behavior waits for the order that writes its promise.

Cost: 2 is a few lines in a checker; 3 and 4 are sequencing rules that slow the lane when the specification is behind. They buy fewer hand merges and no rework orders of the 129, 130, 141 kind.

**Evidence.** docs/work-orders/071-conformance-manifest-pieces.md:12-34; `wc -l conformance/manifest.yaml` (8333); docs/implementation/HANDOVER.md:211 (049a1's rebase conflicts in the manifest and server.rs), :213 (071's one must), :277. Totals: docs/work-orders/075-spec-own-tools-reach.md:640-641, 038c-spec-sign-in-return.md:669-671, 081-spec-one-run-per-instance.md:620-621; `grep -rn -i 'in all' tools/*.py` finds no check, and `grep -n 519 tools/*.py` nothing. Stacks: `git log $(git merge-base origin/wo/078-agents-codex-path-item 2eb8f258)..origin/wo/078-agents-codex-path-item` shows 050a, 076 and 078 stacked; the same for 080 on 079 (outside the pinned commit); docs/work-orders/050a-agents-v2-own-tools.md:38. Kit: docs/work-orders/122-a63-rk1-runtime-kit-server.md:194-197; 129, 130 and 141 Records; docs/guides/full-review.md:132-134.

**Waits.** 043a (stacked on 071), SV1 in the lane, the admin's next specification order (PRV-021)

**Verifier.** partly. 071-conformance-manifest-pieces.md:12-34 read; conformance/manifest.yaml has 8,333 lines; HANDOVER.md:211 and :213. Totals recounted by hand at 075:640-641, 038c:669-671 and 081:620; `grep -n 519 tools/*.py` finds nothing. origin/wo/078-agents-codex-path-item holds the worker commits of 050a, 076 and 078; origin/s3/m2 holds 079 alone, with main 49 commits ahead.

**Verifier's counterargument.** Points 1 to 3 stand. Point 1 is more urgent than stated: 071's branch starts at a005dab0, and 081, 077 and 042 have since added cases to the one manifest list, so 071 already needs a rebase through the files it splits; land it before 046. Point 4 is not carried by the Records. Order 122 put each open choice behind one function (122-a63-rk1-runtime-kit-server.md:194-196); conforming to 081 took 141 one clean round of 606 seconds and 272 lines, and 129 with 130 took 522 lines. Waiting for 081 (merged 06:54 on 2026-11-12) would have held the kit and the providers built on it for about nine hours. A narrower rule fits the evidence: where the text is silent a lane order may choose only behind one function, logged as a finding, and the provider is not counted as passing its contract until the promise lands. What remains wrong is that PRV-021 still lags the kit.

**Verifier's corrections.** None to the facts. The coordinator decides each point; the side branch s3/m2 is already on the owner's strike list (HANDOVER.md:277).


## FR-275

**A78: run small, paired orders through Conduit, so that its friction is separated from an order's size**

P01 I12 · experiment-change · verifier: partly · decides: coordinator

**Row:** A78 in ROADMAP.md. Done when five orders have run through Conduit with each friction logged as a finding of the first build.

**What happened:** the first order chosen, 042, was the largest engine order of batch 1 (five behaviors, 46,118 changed vector lines). It took five worker rounds and 33 hours from written to merged. One turn was lost to a network outage the plain runner survives, each fresh conversation cost an engine restart of 171 to 191 seconds, and its first worker turn ran 11,966 seconds. None of the frictions is yet in increment 4's log. With no like order on the plain runner, the five rounds cannot be divided between the order's size and Conduit's cost.

**Claim to test:** running an order through Conduit's Agents adds a bounded, known overhead and loses no turn that the plain runner would have kept.

**Change:** the next four orders through Conduit are under I6's limit, and each is paired with an order of the same kind and similar size on the plain runner (046 and 047 are the natural first pair).

**Measures, fixed before the first runs:** for each turn, set-up seconds (engine restart included) and work seconds; turns failed, by cause; guard trips; rounds and musts per pass, as in I9.

**Pass condition:** no turn lost to a cause the paired runner survives, and overhead per turn under a bound the coordinator states beforehand. Each friction is entered in increment 4's log before the next order is dispatched, as ruling 70 asks.

The row's "five orders" and its scope stay as they are.

**Evidence.** ROADMAP.md:71 (A78's row); docs/implementation/HANDOVER.md:136-139, :143-152 ("client turn 1 failed: client failure: other; session cannot restart implicitly"; "171 to 191 s"; "None is yet a finding in increment 4's log"), :153-154 (re-pin, then 046 and 047); docs/work-orders/042-build-tombstones-and-inventory.md:20-30, :47-62; `git show --numstat --format= 57813f06` (conformance 46,118 changed lines; engine +952 −63); lead time 2,005 minutes from 920b0769 to 57813f06.

**Waits.** The dogfood engine's re-pin and the choice of the next order through Conduit

**Verifier.** partly. ROADMAP.md:71 (A78's row); HANDOVER.md:136-154; docs/proposals/first-build/log-increment-4.md holds only 4-20 from A78's planning and none of the four frictions. `git show --numstat 57813f06`: conformance +46,107 -11, .rs +936 -50.

**Verifier's counterargument.** Pairing unlike orders cannot separate the runner from the order: rounds vary from one to five between orders of one kind, so four pairs show nothing. Putting one of 046 and 047 on the plain runner takes an order off Conduit, against ruling 70's priority and the handover's plan to run both through it. The frictions can be measured with no control: for each turn, set-up seconds, restarts, turns failed by cause and guard trips, with a bound stated beforehand. If a control is wanted, run one small order twice from the same base, once each way. Keeping the next orders small is right in itself.

**Verifier's corrections.** The 33.4 hours run from the order file's commit (920b0769, 04:31 on 2026-11-11). 042 builds on 041b, merged at 18:06, and needed 074, merged at 21:43, so at least 13.6 hours are queue, not rounds. The restart of 171 to 191 seconds was measured on an engine pinned before 070 (HANDOVER.md:153-154); measure again after the re-pin. The first worker turn of 11,966 seconds is the worker's time, inside its deadline, not a friction.


## FR-276

**Pipeline documents that are stale or understate the Records**

P01 I13 · fact · verifier: supported · decides: coordinator

Small, and the coordinator's to fix in the docs pass that A80 already owes:

- **docs/work-orders/README.md** lists five orders as still in the directory; 80 order files are there. It says no `*-declared.yaml` file is present; 13 are.
- **The handover's list of finishing orders** names six (124, 125, 130, 148, 149, 150). The Records show nine: 114 on 111, 115 on 113 and 140 on 137 are missing.
- **The handover's list of coordinator fixes** names five. The Records show 25, among them the Mac fixes of 131 and 132, 077's vector, 042's sender match, and the lane's fixes to 100, 102, 108 and 118.
- **"Packaged local providers built and passing the contract suites"** in the handover's list of decisions reads as a result. It is the decision's wording: the four suites hold no case.
- **Four checkpointed orders have no Record on main** (049a1, 071, 080, 144). Their state is only in the handover, and PROVIDERS.md points at a Record of 144 that is on its branch.
- **72 Records are prose in their own files.** A80 owes carrying them into the table; I9's block would let a script write it.

**Evidence.** docs/work-orders/README.md:9-19, :27 against `ls docs/work-orders/[0-9]*.md | wc -l` (80) and `ls docs/work-orders/*-declared.yaml | wc -l` (13). docs/implementation/HANDOVER.md:68-70, :91-93, :270, :78-80. Records of 114 ("stacked on order 111's last commit"), 115, 140. experiments/022-provider-contracts/suites/*/index.yaml `cases: []`; experiments/022-provider-contracts/README.md:3-4 ("the harness, with no suite yet"). `grep -c '^## Record'` finds none in 049a1, 071, 080 and 144; docs/implementation/PROVIDERS.md:44-45. docs/AUDIT.md:486 (A80 open).

**Verifier.** supported. docs/work-orders/README.md:9-19 lists five orders and :27 says no declared file is present; `ls docs/work-orders/[0-9]*.md | wc -l` gives 80 and 13 `*-declared.yaml` files exist. HANDOVER.md:68-70 names six finishing orders (114, 115 and 140 missing), :91-93 five fixes, :270 the providers' wording. No `## Record` in 049a1, 071, 080 or 144; 144's is on its branch (origin/wo/144-s4-srv2-hook, ef3554ce). docs/AUDIT.md:486 shows A80 open.

**Verifier's counterargument.** The handover gives its five fixes as examples of the practice, not as a full list, so 'names five' against 25 overstates the fault.

**Verifier's corrections.** Also without a Record: 050a, 076 and 078, which ran and are held, and 147.


## What the verifier says the analyst missed

- **CI never builds or tests two merged lane crates** The matrix glob at .github/workflows/checks.yml:94 is `packages/*/Cargo.toml experiments/*/Cargo.toml`. experiments/013-s2-shared-login/linear-stub (order 135) and experiments/015-s4-partial-setup/scripted-client (orders 142 and 148, S4's scripted client) sit one level deeper and match nothing. For them 'CI green' was empty. The assignment asked what CI must gate; I8 should add this. The coordinator decides.
- **The lane merged outside the two conditions in force** IN_FORCE.md:308 (ruling 71, point 1): a lane merges 'once CI is green and a fresh verifier is clean'. 20 of the lane's 48 merges predate CI's return (#198), and 104:202 and 105:287 say CI was paused. Seven merged after a coordinator's fix with the last verifier not clean (100, 102, 108, 118, 124, 128, 136), two of them with no fresh read (100, 118). The Records cite 'ruling 71' for those fixes, but point 4 delegates small findings that add a feature, not a merge without a clean verifier. Either the practice changes or the owner is asked to amend point 1; the analyst flagged neither.
- **The share that ended with musts left is 41%, and engine orders are the worst kind** 33 of the 121 orders had no verifier after their last worker round, so they cannot count as clean. Over the 88 that did: 36 (41%); engine 7 of 13, specification 4 of 8, paper 9 of 25, harness 7 of 20, package 9 of 22. This reverses the item's by-kind reading and weakens every comparison built on the 30% figure (I9's baseline, I10's 7 of 23).
- **The verifier cannot run the checks** At least 15 Records say the verifier could not rebuild, rerun tests or bind loopback in its read-only sandbox (069:461, 100:192, 101:203, 102:193, 103:206, 104:177, 105:255, 107:231, 117:318, 118:337, 119:338, 120:373, 121:366, 135:82, 139:140). That is a concrete cause of 'a reader finding nothing', and a cheap change: give the verifier a writable build directory outside the tree, so it can run the order's 'Done when' and remove a fix to see its test fail.
- **Twelve unwritten orders stand before the S1 rerun, and batch 1's plan already gives eight orders to other lanes** HANDOVER.md:217 and docs/proposals/spec-batch-1/README.md:664-666: 046, 047, 048a, 048b, 050, 053 in series, and 043a, 043b, 052a, 052b, 051a, 051b joining. The summary's 'six' halves it. Lines 104-105 of the plan assign 043a, 043b, 050, 051a, 051b, 052a, 052b and 053 to 'other lanes'. Whether a restarted lane takes them is the most consequential pipeline choice, and it is absent from I3's options.
- **The admin's slowdown started before the lane** The last seven admin orders merged before ruling 71 (037, 038a, 039, 040, 041a, 044, 045) took 123 to 342 minutes from written to merged, against a median of 57 for all orders before it. The step is at spec batch 1's build orders on the slow engine, not at the lane's start. The owner's pause of the Windows computer also cost three orders on or near the path a worker run each (070 about 5 hours, 077 about 5.5, 081 20 minutes; their Records' first lines). The item names the confounds but not this timing.
- **Parallel verifiers on round one were not considered** I1's own model (each pass an independent sample that finds about half of what is left, at about five minutes) points to sampling more at round one, not to fewer rounds: two verifiers at once, with their overlap as an estimate of what remains. WORK_ORDERS.md's lesson 'A skeptic per angle pays for itself' says the same. It is cheaper in coordinator attention than a split after round one.

## The verifier's notes

Tree: the repository root at 2eb8f258 (2026-11-12). Read only; no build, no cargo, nothing written inside the tree (scratch files only in the session scratchpad).

Read in full: docs/guides/full-review.md, docs/implementation/HANDOVER.md, AGENTS.md, the `## Record` section of all 72 order files that have one, the Records of orders 001 to 066 in docs/implementation/WORK_ORDERS.md (not their decision lists), rulings 71 and 74, .github/workflows/checks.yml in the parts cited, PROVIDERS.md lines 1 to 140, the roadmap's order of work and the A77 and A78 rows.

Not read: the order bodies above their Records except where cited; engine and package code; the runners, which are outside the repository, so how a later round's diff is shown to its verifier is not established. GitHub's branch-protection setting cannot be read from the tree.

Branch facts (049a1, 071, 080, 144, 078, s3/m2, and the round-one sizes of 111, 113, 137, 146) come from remote-tracking refs origin/* and win/*, which are outside the pinned commit; their tips are dated 2026-11-12.

Overall: the analyst's table is accurate against the Records, and every count in it reproduces. The faults are in what is built on it. The 30% share and the by-kind table treat unverified second rounds as clean. The size threshold is measured at merge and misses every admin order that ran long. The lead-time medians and three smaller counts do not reproduce. The summary's 'six unwritten orders' should be twelve. Its strongest findings stand: a fresh pass finds more about half the time; coordinator fixes need a fresh read (six of eight readers found more); the engine shards can be skipped by any package crate; the contract suites are empty.
