# G4-who-verifies: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** At commit 2eb8f258 the Codex verifier is a productive reader and should not be thrown away as a function: the Records on main credit it with at least 246 must-level faults in 117 merged orders, and with a fault in most first passes. A clean verifier is still not a test of being done: 14 of the 52 orders that ended on a clean verifier have a fault recorded later, and a smaller diff would have prevented none of the 14. What escaped sorts into three groups: what no reader can see (platform, timing, real clients, integration: 19 faults found by the coordinator or an author running things, 2 by CI after merge); what the verifier's brief excludes (the coordinator's own settlements and hand fixes: 17 of 25 fixes merged unread, and 6 of the 8 that were read had more to fix); and what a second reader with another brief found in work a Codex verifier had passed (66 changes to the plans; at least 78 findings of this review's first pass in the 64 promises the pipeline's specification orders wrote). So the owner's preference, reading on the coordinator's model, is right and is not enough by itself: checks have to run before the read (CI on the branch with a macOS leg, nested crates and required status; a Codex check run for fix-removal and real clients), and the coordinator's own text needs an independent read, or the same-model problem only moves to the other side. I recommend a design by kind and size that costs 0 to 4 reads on the coordinator's model per order (about 2 on the past mix, about 35 to 40 for the 16 orders to the pin) and about the same Codex sessions as now. No verifier must is recorded as wrong, and the Records could not show one; that count has to start being kept. The tree holds no like-for-like comparison of the two models, so A77 should pair both verifiers on the same diff for the next ten orders with the decision rule fixed first, and A78 should give the dogfood a verifier that is another client on another model. The preference itself is not in the tree and contradicts ruling 48's record and five other lines; it must be recorded, with a stop rule for the Claude allowance, which is now the binding limit and has already ended one session. I put no question to the owner: the model is the owner's stated preference and the rest is the coordinator's.

**What the analyst read.** Read in full: docs/guides/full-review.md; AGENTS.md; docs/implementation/HANDOVER.md; docs/implementation/WORK_ORDERS.md lines 1-222 and the Record of every order 001 to 066 (the condensed decisions under each were not studied); the `## Record` section of all 72 order files under docs/work-orders/ that have one; docs/work-orders/verify.md, verify.schema.json and README.md; .github/workflows/checks.yml; the records of rulings 48 (to line 120) and 71; IN_FORCE.md topic 15; docs/proposals/first-build/log-increment-4.md; the first pass's approach/P01-work-orders-lanes.md, approach/INDEX.txt and language/INDEX.txt.

Read in part: order 121 (head and the outline of its decisions), order 150's done-when, nightly.yml, ROADMAP.md rows A77 and A78 and 'Proposing an experiment', the settlements register rows 3-03 to 3-05, packages/agent-claude-code/contracts.yaml (the model input), the first pass's critic.md and P06 file by search, finding X07/F1 with its verdict, git subjects since 2026-11-08 and the bodies of four commits, `git log -S` for the commit that first added 22 promise IDs and one test.

Counted myself from the Records: verifier musts per order (246 on main; the first pass's table gives the same once its seven from unmerged branches are removed), faults by finder, the 14 clean-then-fault cases, Records naming a verifier that could not run checks (15). Reused after parsing by script: the first pass's per-order table (rounds, passes, final state), whose counts its verifier reproduced. The 78-finding count uses the first-cited promise of each first-pass finding as a proxy and includes duplicates the critic grouped.

Not read: the bodies of the orders above their Records; engine and package code; the runners, the worker and verifier reports and the private note (outside the repository); the unmerged branches of 049a1, 071, 080 and 144 (their state is taken from the handover); the other first-pass files beyond the indexes. Nothing was built or run, and nothing was written. The owner's preference of 2026-11-14 is taken from the task text; it is not in the tree.


## FR-491

**Changing the verifier's model closes one of three gaps; the Records put most escapes in the other two**

G4 I1 · assessment · verifier: partly · decides: none

What is working: the Codex verifier is a productive reader. The Records on main credit it with at least 246 must-level faults, and with a fault in most first passes (36 of 45 early orders, 10 of the 16 later admin orders that itemize, 23 of 48 lane orders). It catches tests that cannot fail by reading (at least ten orders). A refuting read belongs in every order.

What got past it falls in three groups, and only the third is about the model.

1. What a reader cannot see: platform, timing, real clients, integration. These were found by the coordinator or an author running checks, real clients or S1 (19), or by CI after merge (2). The verifier's sandbox cannot build (15 Records say so) and sits on one platform. One Record credits it with a fault found by running a command (`git diff --check`, order 139); five more say it reproduced results.
2. What its brief excludes: the coordinator's own text. verify.md holds the diff to the order's decisions. Nothing holds the decisions to the promises, or a coordinator's hand fix to anything. Of 25 coordinator fixes, 17 merged unread; of the 8 that a fresh reader checked, 6 had more to fix. The specification orders 077, 081, 082, 075 and 038c apply the coordinator's settlements; each ended on a verifier that found nothing in scope, and this review's first pass then found faults in exactly that text (ten on 077's fence, seven marked unsafe).
3. What a second reader with another brief finds: four split reviews produced 66 changes to plans whose Codex verifiers were clean or nearly so; the first pass's readers, on the coordinator's model, have 78 findings whose first-cited promise was written by a pipeline specification order (64 of 519 promises, 30% of the findings).

Size does not explain escapes: a smaller diff would have prevented none of the 14 clean-then-fault cases (I6). Size governs whether rounds converge (first pass, P01 I5), not what gets out.

Judgment: reading on the coordinator's model, as the owner prefers, is right, and is best spent where groups 2 and 3 sit: specification, engine, experiment designs. Alone it leaves group 1 untouched, and it re-creates the shared-model problem on the other side, because for specification orders and hand fixes the author is the coordinator. Three changes belong together: checks that run before the read (I3), a reader on the coordinator's model with a wider brief (I2), and an independent read of the coordinator's own text (I4).

Limit: no two verifiers of different models ever read one diff under one brief, so every comparison above mixes model, brief, reading time and number of readers. I8 fixes that.

**Evidence.** Verifier musts: my count from docs/implementation/WORK_ORDERS.md:224-1367 (124 in orders 001-066, severity unstated for 020-066), the 72 `## Record` sections under docs/work-orders/ (41 itemized in 037-082, 81 in 100-150); the first pass's table (approach/P01-work-orders-lanes.md, I4) parsed by script gives 253 including 7 on unmerged branches. Vacuous tests caught by reading: WORK_ORDERS.md:354-357 (006 'could not fail'), :445-446 (010), :705-706 (019), :905-906 (025), docs/work-orders/045-build-state-directory.md:51-54, 068:115-117, 111:236-240, 112:224-228, 113:199-200, 126:189-191, 138:126-128. Cannot build: 069:461, 100:192, 101:203, 102:193-194, 103:206-207, 104:177-178, 105:255, 107:231-232, 117:317-318, 118:337-338, 119:338-339, 120:373-374, 121:366-367, 135:82, 139:140. Reproductions: WORK_ORDERS.md:1047-1048 (031), 041a:69, 104:176-178, 105:254, 109:226-227; 139:139-140 (the `git diff --check` must). docs/work-orders/verify.md:8-10 ('For each decision the order states, check that the change applies it as written ... that it says no more than the decision'). Fixes read and unread: first pass P01 I4 and I7 with the verifier's recount (102:205-209, 108:258-260, 124:121-123, 128:150-152, 131:125-128, 136:148-150; 132:133, 140:140-141). First-pass findings: language/INDEX.txt rows X07/F1-F10, X08/F1-F9, X09/F3-F4, X04/F9, L09/F2; the 78 by script over INDEX.txt against the 64 IDs first added by orders 036a, 037, 038a, 038b, 038c, 075, 081, 082 (`git log --reverse -S'id: &lt;ID>' -- spec`). Order 121: docs/work-orders/121-plans-follow-up.md:13-15.

**Waits.** I2, I3, I4 and I8 rest on it

**Verifier.** partly. Recounted at 2eb8f258. The counts on the first pass's order table (approach/P01-work-orders-lanes.md I4, parsed by script) reproduce: 124 + 41 + 81 = 246 musts in merged orders (253 with unmerged ones); a fault in the first pass in 36 of 45, 10 of 16 and 23 of 48 orders; 20 admin passes not itemized. 217 worker rounds and 183 verifier passes over 121 orders. verify.md:8-10 reads as quoted. Fifteen Records say the verifier could not build (e.g. docs/work-orders/105-mc2-operation-identity.md:255, 139-s2-l2-linear-adapter.md:140-141). The 64 promise IDs first added by 036a, 037, 038a, 038b, 038c, 075, 081 and 082 reproduce (FLEET-001 to 006 included). Over language/INDEX.txt, 78 findings cite one of them first: 50 confirmed, 24 weakened, 4 known, 14 UNSAFE. 121-plans-follow-up.md holds 66 numbered items.

**Verifier's counterargument.** Four of the supporting pieces do not hold as stated.
(1) '077's fence: ten findings, seven unsafe' is wrong. X07 has 6 UNSAFE (F1-F5, F7). X07/F5 and F6 are about CTR-002 at spec/promises/contracts.yaml:43, not 077's text. X07/F7 is the limit the owner accepted on 2026-11-12.
(2) 'Nothing holds the decisions to the promises' is overstated. verify.md:10 asks that the change 'agrees with the promises it touches', and a Codex verifier did fault the coordinator's own decision ('no exp' against DL13, docs/implementation/WORK_ORDERS.md:723-727).
(3) Group 3's evidence is inflated. The 66 are mostly re-splits, dependencies and the admin's four answers (121-plans-follow-up.md:83 'The admin's answers'), not faults. Of the 78 findings, 44 sit on 038a's 16 promises, 12 are the owner's to decide, and 'first-cited' is a proxy: 172 findings cite one of the 64 somewhere.
(4) The Claude readers behind group 3 were Opus, the outgoing session and its subagents (180 commits co-authored by Claude Opus 5.5). The lane coordinator's own Opus reviews found nothing in 132, 140, 145 and 150 (132:140, 145:129, 150:107-108), and 150's code later broke CI. So the evidence does not show that a reader on the coordinator's model catches more. The conclusion rests on the owner's preference, not on the Records.
Also, group 1's 131, 132 and 140 were caught before merge by the existing Mac step (docs/implementation/HANDOVER.md:81-84).

**Verifier's corrections.** Six unsafe, not seven. Say 'about eight on 077's fence text': F5 and F6 are on CTR-002. Call the 66 'plan changes, some of them faults', not faults.


## FR-492

**The verification design by kind and size of order: who reads, on which model, what runs, how many passes, and what it costs**

G4 I2 · recommendation · verifier: partly · decides: coordinator

Rules for every order:

- **Order of steps: worker, checks, read.** The runner already checkpoints the worker's commit. It then runs `tools/check_private_paths.py` and `git diff --check`, pushes the branch as a draft pull request, and CI's result goes to the reader. A Codex check run, in a writable copy of the checkpoint, does what CI cannot: the order's done-when commands on the Mac, removing each fix to see its test or vector fail, real clients where the order needs them. This is the 'test runs on Codex' half of the owner's preference.
- **The reader** is a fresh, read-only session on the coordinator's model, started with no user memory and no settings of the owner's (finding 3-04: Claude Code loads ancestor instruction files unless told not to). Its brief is verify.md with three additions: it may fault the order's decision itself, reported as an order fault; after each fault it searches for the class (order 014 found a sixth access hole that way); it names up to five targeted checks for the check run and runs nothing itself.
- **Round 2 is a delta read**: the fix and round 1's findings, each closed or not, and its class searched. No raw third round (first pass, P01 I7).
- **Two readers on round 1**, with different briefs, only where the table says. A: the order's decisions and the promises it cites. B: the tracker's unsafe test, tests that cannot fail, platform assumptions.

| Kind and size | Checks before the read | Reads on the coordinator's model | Sessions per order: Codex worker + Codex check / reads |
| --- | --- | --- | --- |
| Paper: sweep, condensation, results | CI's docs job | 1, and a delta read if a second round runs | 1.6 + 0 / 1.6 |
| Paper: plan or pre-registration | CI's docs job | 2 on round 1 (B asks, for each pass condition, which result would refute the claim), then delta | 1.6 + 0 / 2.6 |
| Specification | CI; the engine's suite and vectors; `tools/vector_delta.py` against the declared file; the model's rerun where experiment 012 models the promises touched | 1 on the settlements before dispatch; 2 on round 1; delta | 1.8 + up to 1 / 3.8 |
| Engine: one behavior, at most about 15 hand-written vectors and 1,000 changed non-test Rust lines, away from access, outcomes, secrets and stops | CI with the engine shards; check run on the Mac with fix-removal | 1; delta | 2 + 1 / 2 |
| Engine: larger, or touching those | the same | 2 on round 1; delta | 2 + 1 / 3 |
| Package with locks, processes, durable state or secret handling | CI on Linux and macOS; repeated runs of its timing tests; contract-suite cases as they come to exist | 1 (2 above about 1,000 non-test lines); delta | 1.7 + 0 to 1 / 1.7 to 2.7 |
| Package of contracts or fixtures only; harness stub or scripted client | CI, nested crates included; check run with fix-removal | none beyond the coordinator's own read of the diff | 1.7 + 1 / 0 |
| Harness: a model of promises | CI; each broken variant breaks its named property | 1, on the properties against the promises; delta | 1.7 + 0 / 1.7 |
| Finishing order, or a coordinator's hand fix | CI | 1 delta read | 1 + 0 / 1 |

**Cost.** Today an order takes 1.79 Codex worker rounds and 1.51 Codex verifier passes, and no session of its own on the coordinator's model. On the mix of the 121 orders run so far the table comes to about 2 reads per order on the coordinator's model (0 to 4 by kind), with Codex sessions about as now, since the verifier's pass becomes the check run or CI. For the 16 orders to the pin (12 unwritten, 4 to finish): about 35 to 40 reads. These are estimates from the Records' round rates, not measurements. What one read costs in the Claude allowance is recorded nowhere, so record tokens per read from the first order.

**What it buys.** Reads go where the unsafe class and the escapes sit (specification, engine, experiment designs), and stop being spent where a check decides.

**What it limits.** The Claude allowance ran short once already. Throughput is now bound by reads: nothing merges unread, and when the allowance nears its limit dispatch stops before reading does. That argues for one lane to the pin and against the 57 orders a day of 2026-11-11.

**Evidence.** Baseline: 217 worker rounds and 183 verifier passes over 121 orders, 80 of them with more than one round, by script over the first pass's table (approach/P01-work-orders-lanes.md, I4); by kind S 1.83/1.50, E 2.22/1.59, P 1.55/1.35, H 1.65/1.52, K 1.65/1.61. Musts left at a final verifier by kind: engine 7 of 13, specification 4 of 8, paper 9 of 25, harness 7 of 20, package 9 of 22 (P01 I4, verifier's recount). Class search: docs/implementation/WORK_ORDERS.md:552-555. Checkpoint commit: docs/implementation/HANDOVER.md:59. Finding 3-04: docs/proposals/first-build/README.md:86 ('Project sources load ancestor CLAUDE.md outside the private directory'). Size thresholds: P01 I5 and I6 with the verifier's corrections (at or below 1,000 changed Rust lines 5 of 9 lane pieces clean in one round; above 2,000, 7 of 9 with musts at the second pass). Recurring fault classes in packages: races 122/124, 129/130, 146/149 (docs/work-orders/149-ssh-r-drainer-finish.md:82-85); redaction 123, 125, 136, 142, 148; platform 131, 132, 140. Model-check faults in the checks themselves: 111:236-244, 113:199-204. Orders to the pin: HANDOVER.md:211-217. Usage: HANDOVER.md:4-5 ('because the owner's usage was near its limit').

**Waits.** The runner's change (a second verifier command and the draft pull request); verify.md's text; every order from 046 on, and the finishing orders for 071, 080, 049a1 and 144

**Verifier.** partly. The baseline reproduces: 1.79 worker rounds and 1.51 verifier passes per order; by kind S 1.83/1.50, E 2.22/1.59, P 1.55/1.35, H 1.65/1.52, K 1.65/1.61. Other citations check out: the class search at docs/implementation/WORK_ORDERS.md:552-555; the checkpoint commit at HANDOVER.md:59; finding 3-04 at docs/proposals/first-build/README.md:86. Weighting the table by the past mix gives about 2.0 reads per order.

**Verifier's counterargument.** Four problems.
(1) The row with no reader for 'harness stub or scripted client' contradicts the item's own evidence. SCR1 (142) had 3 then 3 Codex musts, including redaction and case-insensitive collisions that CI and fix-removal would not show (142-scr1-scripted-client.md Record). The contract harness 123 had 6 then 3, and it fits no row of the table.
(2) A simpler design was not weighed: one verifier that can run checks. Give it a writable CARGO_TARGET_DIR outside the tree, or a nextest archive built by the runner. 104's verifier already 'ran the release binary's 8 tests again' (104-mc1-model-checking-harness.md:177). For fix-removal, use the existing tool cargo-mutants, which can mutate only the diff, instead of a second Codex session plus a reader that hands off up to five checks.
(3) '16 orders to the pin, 4 to finish' miscounts. 049a1 and 080 go to s3/m2, which merges after the pin (HANDOVER.md:221), and 144 is the lane's. Only 071 is on the pin path, and rows 7 and 8 (the next specification order and the small engine order) are left out.
(4) Cost is given in sessions, not in the Claude allowance, and that allowance binds: the request that started this run says 'we ran into our limits'.
Two smaller points. 'One lane to the pin' touches ruling 71 point 1 and P01 I3's open question. '57 orders a day' mixes the 56.8-a-day rate with the 58 merges of 2026-11-11.

**Verifier's corrections.** Give stubs and scripted clients that redact or parse untrusted input at least one read. Add a row for the provider-contract harness. Count the orders to the pin as about 14 or 15 (the 12 unwritten in row 9, 071, rows 7 and 8). Hold the lane question as the owner's, or as P01 I3's.


## FR-493

**What CI must gate in place of a reader, and which escape each gate would have stopped**

G4 I3 · recommendation · verifier: partly · decides: coordinator

A reader should not be asked to find what a machine can. Nine gates, most of them small:

| Gate | The escape it answers |
| --- | --- |
| 1. `docs` and `rust` required on `main` (a repository setting, the owner's hand) | Order files 122 and 123 failed `main`'s docs job for 44 minutes; 077 merged with the engine shards skipped; 20 lane merges were made while CI was paused |
| 2. A macOS leg for `packages/` and for the engine's loader, core and conformance suites | 131 and 132 did not compile on the Mac and 140 failed 20 of 33 tests there, each after a clean verifier on WSL |
| 3. Engine shards that no package crate can skip | Since #260 one SSH-R test skips every engine shard when it fails |
| 4. The matrix finds crates one level deeper | `experiments/013-s2-shared-login/linear-stub` and `experiments/015-s4-partial-setup/scripted-client` (orders 135, 142, 148) are never built or tested |
| 5. `tools/vector_delta.py` against the order's declared file, on any pull request that touches `conformance/` | 'Every flip declared' is checked by hand today |
| 6. The promise totals in `spec/README.md` computed, not typed | Recounted by hand at three rebases |
| 7. Repeated runs of changed tests that use locks, timers or processes | The test that fails at times on CI's Linux runner passed once for the worker, the verifier and the Mac |
| 8. A nightly vector run on the differential build | It takes about 2.4 hours on the Mac and runs only when someone starts it |
| 9. A boundary test for each class an escape shows, as 072 added | Order 040's `list.v1` was in the core and refused by the shell; the vectors run the core alone, so none showed it |

Cost: one harness order for 2 to 8 (one or two Codex worker sessions, one read) and a setting for 1. Gate 9 is a habit, not an order.

What it buys: four of the 14 clean-then-fault cases (131, 132, 140, 150) and both failures CI found only after merge. It also lets the table in I2 drop the reader for contracts-only packages, stubs and sweeps.

Gates 1 to 3, 6 and 7 are the first pass's (P01 I8 and I11, with its verifier's corrections); 4 is that verifier's; 5, 8 and 9 and the mapping to escapes are added here.

**Evidence.** .github/workflows/checks.yml:94 (`for manifest in engine/Cargo.toml packages/*/Cargo.toml experiments/*/Cargo.toml`), :187-189 (`engine-tests` needs `rust-dir`), every job `runs-on: ubuntu-latest`; `ls experiments/*/*/Cargo.toml` gives the two nested crates; `grep -n -i 'differential\|vector_delta\|declared' .github/workflows/*.yml` gives nothing. `git log -1 128ca08a` ('so main's docs job failed'); docs/implementation/HANDOVER.md:198-205 (the SSH-R test; 'the engine shards are skipped'), :209 ('the differential vector run takes about 2.4 hours on the Mac'). docs/work-orders/131-a63-ps-local-directory.md:123-125; 132-a63-fs-file-store.md:131-133; 140-ssh-r-finish.md:135-137. The failing test was added by 137 and 140: `git log --reverse -S'persistent_part_owner_lock_and_short_socket_fallback_without_network' -- packages` gives 6ce8d136. docs/proposals/first-build/log-increment-4.md:30 (4-19: 'The vectors run the core alone, so none showed it'). Lane merges while CI was paused and the hand recounts: first pass P01 I2 and I11, verifier's text; docs/work-orders/104-mc1-model-checking-harness.md:202-203.

**Waits.** The first fix in the handover's queue (CI). Gate 1 needs the owner's hand on the repository setting: one line in the message to the owner, not a ruling

**Verifier.** partly. Checked at 2eb8f258. .github/workflows/checks.yml:94 holds the one-level glob. `ls experiments/*/*/Cargo.toml` gives the two nested crates. Every job runs on `ubuntu-latest`. engine-tests has `needs: rust-dir` (:187-189). Nothing in .github/workflows mentions differential, vector_delta or declared, though tools/vector_delta.py exists. `git log -1 128ca08a` says main's docs job failed. HANDOVER.md:198-205 and :209 read as quoted. The flaky test was added in 6ce8d136 (#241) and fails since #260.

**Verifier's counterargument.** Three of the claims need correcting.
(1) 'What it buys: 131, 132, 140' overstates. All three were caught before merge by the existing manual Mac run (HANDOVER.md:81-84). The macOS leg automates that run; it stops no escape to main.
(2) 'Passed once for the worker, the verifier and the Mac' is unsupported for the verifier. Verifiers cannot run tests, and 150's Record names no verifier run (150-ssh-r-one-drainer.md Record).
(3) Order matters. Making `rust` required before the SSH-R race is fixed blocks every merge, so gates 3 and 7, and the handover's CI fix, come before gate 1.
Gate 2 depends on IN_FORCE.md:300 ('only on free runners', 64 point 3), which holds only while the repository is public; say so. For gate 7, a deterministic test of the lock handoff is better than repetition, and the project already uses Stateright in experiments/012; loom is the Rust standard.

**Verifier's corrections.** Frame 131, 132 and 140 as a manual step made automatic. Drop the verifier from gate 7's sentence. Sequence: fix the race, decouple the shards, then require the checks.


## FR-494

**The coordinator's own text gets an independent read: every hand fix before merge, every specification settlement before dispatch**

G4 I4 · recommendation · verifier: partly · decides: coordinator

The pipeline checks the worker against the order. It does not check the order, or what the coordinator writes by hand after the last verifier. The Records show both are fault sources.

- **Hand fixes.** 25 coordinator fixes in 117 merged orders. A fresh reader checked 8; in 6 it found more to fix, and in 131 it overturned the coordinator's diagnosis. The other 17 merged unread, among them the admin's fixes to 038b, 041b, 042, 068 and 077.
- **Decisions.** At least eight of the coordinator's own decisions were wrong, each found by someone else or by a run: 020's 'no exp' against DL13 (the Codex verifier), 023's input against MAN-017 (the worker), 107's two over-broad properties (the worker), 070's two demands that could not both be met (round 3), 027's decision 6 (a real client), 131's diagnosis (a fresh reader), 009's licence claim, 017's settlement against OPS-029 (the admin itself, afterwards).
- **The coordinator's own review is not that read.** Nine lane Records say its review found nothing to change. The admin's Records credit its reading with about twelve faults in all, most of them mechanical.

Change:

1. A hand fix is read before merge by a fresh reader on the coordinator's model with none of the coordinator's context. Cost: one read per fix, about 0.2 per order at the past rate.
2. A specification order's settlements are read before dispatch by a fresh reader on the coordinator's model (already in I2's table), and by the model's rerun where a model exists.
3. Keep the cross-model check that costs nothing: the order's preamble tells the Codex worker to report any decision that contradicts a promise it reads, beside the choices it made itself. Orders 020, 023 and 107 show Codex doing this unprompted.
4. The lane's rule in force, 'once CI is green and a fresh verifier is clean', is then kept as written: the delta read after a fix is that verifier. Seven lane merges did not meet it.

This is where 'worker and verifier share a model' would otherwise return. Once the reader runs on the coordinator's model, the coordinator's own text is the work whose author and reader share one.

**Evidence.** Fixes and fresh readers: first pass, approach/P01-work-orders-lanes.md I4 and I7 with the verifier's recount; docs/work-orders/131-a63-ps-local-directory.md:125-128 ('a fresh reader showed that diagnosis wrong'); 038b:398-400; 041b:57-59; 042:59-61; 068:101-102, :111-117; 077:595-598; 100:183-185; 118:346-351. Wrong decisions: docs/implementation/WORK_ORDERS.md:726-727 (020), :836-839 (023), :421-423 (009), :655-658 (017), :955-958 (027), :118-119 ('Verify the admin's own decisions'); docs/work-orders/107-mc3-access-over-time.md:242-249; 070-build-state-copies.md:354-357. Reviews that found nothing: 103:225-226, 132:140, 138:137-138, 139:148-149, 141:132, 143:327-328, 145:128-129, 148:85-86, 150:107-108. The lane's rule: IN_FORCE.md:308. Lane merges after a fix with the last verifier not clean (100, 102, 108, 118, 124, 128, 136): first pass, P01, 'What the verifier says the analyst missed'.

**Waits.** The finishing fixes for 049a1 (one must, one should) and the admin's next specification order (PRV-021, PRV-015, the labels wording)

**Verifier.** partly. Holds: 25 coordinator fixes, 8 read by a fresh reader, 6 of those with more to fix (first pass P01 I7 with its verifier). 131-a63-ps-local-directory.md:125-128 reads as cited. The nine lane reviews that found nothing check out (103:225-226, 132:140, 138:137, 139:149, 141:132, 143:328, 145:129, 148:85, 150:108). IN_FORCE.md:308 has the merge rule. The wrong decisions in 020, 023, 070, 107, 009 and 017 read as cited (WORK_ORDERS.md:723-727, :834-839, :419-423, :652-658; 070:354-357; 107:242-249).

**Verifier's counterargument.** The remedy reads the coordinator's text on the same model the item names as the problem ('the coordinator's own text is the work whose author and reader share one'). The reader with another model for Claude-written text is the Codex verifier. It costs no Claude allowance, and it has already caught a coordinator decision (020's 'no exp' against DL13). The item applies I9's own principle, a verifier on another model than the author, to workers but not to the coordinator. The Opus coordinator's reviews passing 145 and 150 also weaken 'a fresh reader on the coordinator's model' as the only check.
Point 3 mostly exists already: AGENTS.md says 'apply the nearest sound reading and say so in your report', and WORK_ORDERS.md:96-97 asks the report to state the worker's own choices apart.
027's decision 6 and 107's properties were found by running things, not by reading.

**Verifier's corrections.** Read hand fixes and specification settlements with the Codex verifier, under verify.md plus 'fault the decision itself', and add a fresh read on the coordinator's model only for specification settlements.


## FR-495

**Faults by who found them, from the Records and git log**

G4 I5 · fact · verifier: partly · decides: none

Counted at 2eb8f258 from the Record of each order and from git log. 'Fault' is a must, or a fault of unstated severity (orders 020 to 066 give none).

| Who found it | Faults | Notes |
| --- | --- | --- |
| The Codex verifier | at least 246 | 124 in orders 001 to 066; 41 itemized in 037 to 082 (twenty passes there give no count); 81 in the lane |
| The coordinator, reading a diff, a report, or the code while planning | about 12 | 002 (two), 005, 017, 023, 024, 060, 038c (two, at its rebase), 4-19 and 4-20 after merge; 009's finder is not stated. Nine lane reviews found nothing |
| The coordinator or an author, running checks, real clients or S1 | 19 | 9 before merge (002, 010, 027, 042, 131 twice, 132, 140 twice); 3 in increment 3's real-client closeout (3-03 to 3-05); 7 in increment 4's author runs (4-07, 4-09, 4-10, 4-13 to 4-16) |
| Readers the coordinator ran | 9, and 66 changes to plans | 9 by fresh readers of eight coordinator fixes; 66 numbered changes from four split reviews and the admin's four answers (order 121). What 51 reviews before dispatch found is not recorded. Before the pipeline, round one's reviews: 4 defects, 6 counts, 2 coordinator errors |
| CI | 2, both after merge | #205 (the docs job) and the SSH-R test since #260. None is recorded before a merge |
| A later order's worker or verifier, in merged work | 13 | 033's verifier (1), 077's verifier (1), 025's worker (4-05), 128's worker (2), 137's and 124's workers (1 each), 100's worker (4), 034's and 035's workers (1 each). Apart: the model-checking orders found 8 faults in the promises, and 026's cold read found 25 stale statements |
| This review's first pass, after merge | 78, and 14 more | 78 findings whose first-cited promise a pipeline specification order wrote (50 confirmed, 24 weakened, 4 known; 14 marked unsafe); 10 on 077's fence text and 4 on PRV-021 as 081 amended it |

Escapes that reached `main`, by any finder, in at least 13 groups: 032's tracked bytecode with local paths; 040's `list.v1` refused by the shell; the squash of 041b that deleted MC8's files; 122 and 123's order files; the SSH-R test; five proposal files with local paths (#138); the first-start report's GOARCH names (open); the kit's unknown answer that named no owed stop; the plan-and-apply reference that calls no control; 3-03 to 3-05; increment 4's defects of the build; two lane crates CI never builds; the first pass's findings in pipeline-written promises.

Reading the table: the verifier is the largest finder by far. Everything it cannot see was found by running something. The coordinator's own reading found little. CI found nothing before a merge because it was not a gate.

**Evidence.** Verifier: as I1. Coordinator reading: docs/implementation/WORK_ORDERS.md:268 (002), :334-335 (005), :655-658 (017), :844-846 (023), :875-876 (024), :1229-1231 (060); docs/work-orders/038c-spec-sign-in-return.md:669-673; docs/proposals/first-build/log-increment-4.md:30 ('Found by the admin on 2026-11-11, while planning'), :31. Running: WORK_ORDERS.md:268, :447-450 (010), :955-958 (027); 042:57-58; 131:123-125; 132:131-133; 140:135-137; docs/proposals/first-build/README.md:85-87 (3-03 to 3-05, 'real Claude closeout'); log-increment-4.md:18-27. Readers: 102:205-209, 108:258-260, 124:121-123, 128:150-152, 131:125-128, 136:148-150; 121-plans-follow-up.md:13-15 and its 66 numbered items in sections A to H (`grep -n '^[0-9]*\. \*\*'`); WORK_ORDERS.md:99-102; reviews before dispatch: first pass P01 I10. CI: `git log -1 128ca08a`; docs/implementation/HANDOVER.md:198-203. Later orders: WORK_ORDERS.md:1097-1098 (033), :914-915 (4-05); 077:592-593; 128:155-158; 137:244-246; 124:132-134; 100:213-222; WORK_ORDERS.md:1123-1124, :1141-1142; model checking: 116-mc8-g5-account.md:161-177; 026: WORK_ORDERS.md:935-936. After merge: `git log -1 ed8043a1`, `git log -1 9ee84fe2`; .github/workflows/checks.yml:94. First pass: language/INDEX.txt by script (first-cited promise among the 64 IDs first added by 036a, 037, 038a, 038b, 038c, 075, 081, 082; a proxy, and the critic's duplicate groups are not removed).

**Verifier.** partly. The verifier's 246 reproduces from the table. The later-order row sums to 13 (1+1+1+2+1+1+4+1+1). 66 and 78 (50/24/4, 14 UNSAFE) reproduce. Running finds check out: 3-03 to 3-05 (docs/proposals/first-build/README.md:85-87) and 4-07, 4-09, 4-10, 4-13 to 4-16 (log-increment-4.md).

**Verifier's counterargument.** (1) CI found at least 3 faults after merge, not 2: 068 began because 'CI on `main` is red' with the crash tests on a slow runner (068-test-gate-and-ci.md:12).
(2) 'None is recorded before a merge' says nothing about what CI found. No Record ever states a CI result: a grep of the Record sections finds CI only in 068, 104 and 105, as 'paused'. Meanwhile the admin 'waits for CI' at each pull request (HANDOVER.md:74). That is an absence of record, not evidence that CI found nothing.
(3) The coordinator's 'about 12' is a floor: its findings are folded into round-2 instructions without credit (WORK_ORDERS.md:84-85).
(4) The rows count unlike units: musts, plan changes, and review findings of any class and severity. Their sizes cannot be compared.
(5) '10 on 077's fence' includes two on CTR-002.

**Verifier's corrections.** CI: at least 3, all after merge; whether it caught anything before merge is unknown. Say that rows count different units.


## FR-496

**A clean verifier, then a fault: 14 of 52 orders, and what would have caught each**

G4 I6 · fact · verifier: partly · decides: none

52 orders ended with a final verifier that found nothing. For 14 of them (27%) the tree records a fault found later in that order's work. A fifteenth, 042, was clean at its fourth pass and gained two faults after the coordinator's rebase. Apart from these, 33 orders had no verifier after their last worker round, and 36 ended with musts left.

| Order | Later fault, and who found it | A verifier that ran the checks? | A reader of another model or brief? | A smaller diff? |
| --- | --- | --- | --- | --- |
| 131, 132 | Tests do not compile on macOS; the coordinator's Mac run | Yes, on the Mac only; both sessions ran on WSL | Unlikely | No |
| 140 | 20 of 33 tests fail on the Mac (GNU-only `chmod` and `find` use); Mac run | Yes, on the Mac only | Plausible, with a platform line in the brief | No |
| 150 | An earlier test fails at times on CI's Linux runner; CI, after merge | Only with repeated runs | Plausible: readers found a race in each of four earlier passes over this code | No (a 648-second order) |
| 117, 119, 120 | 66 changes to the plans; four split reviews by fresh skeptics | No | Yes: it is who found them | No |
| 069, 069b | The reference shows a changed control value and calls no control (OPS-020); the admin, later | Only if a program produced the plans; none does | Plausible | No |
| 038c | The client-ID confirmation is keyed by an address any installer can claim; first pass | No | Yes | No |
| 075 | An Agent Type that declares nothing gets the mildest report; first pass | No | Yes | No |
| 081 | PRV-021 has no split by instance (141's worker); nine first-pass findings, one confirmed unsafe at high severity | In part: MC4's re-registration on the new text has not run | Yes | No (two vectors) |
| 082 | REC-003's line between the engine's acts and outside faults is undecided; first pass | No | Yes | No (one vector) |
| 130 | The kit answered unknown before naming owed stops; 137's worker | No | Plausible | No |

By kind: platform 3, timing 1, specification 4, plans and a reference 5, a provider's explicitness 1.

By remedy: a check run on the right platform or repeated would have caught 3 and perhaps a fourth. A second reader with another brief found 9 in fact (7 through reviews the coordinator ran). A smaller diff would have caught none. 042's failing vector was found by running the vectors after the rebase, which is the argument for running checks after every rebase, not only after every round.

077 is not among the 52: its third verifier found one out-of-scope fault and its fourth pass is not itemized. The first pass has ten findings on its fence text, seven marked unsafe, one of them an eight-step trace to two effects for one key that the order's own vector and MC2's fenced control both pass over.

**Evidence.** 52, 33 and 36: by script over the first pass's table (approach/P01-work-orders-lanes.md I4) and its verifier's recount. Clean verifiers: docs/work-orders/131-a63-ps-local-directory.md:118-119; 132:126-127; 140:123-125; 150:97-98; 117:316-317; 119:333-334; 120:369-370; 069:451; 069b:85; 038c:652-653; 075:629-630; 081:616-617; 082:727-728; 130:98-99. Later faults: 131:123-129; 132:131-133; 140:135-141; docs/implementation/HANDOVER.md:198-203; 121-plans-follow-up.md:13-15; HANDOVER.md:221 and docs/guides/full-review.md:136-138 (the control gap), :122-124 ('The admin found these'); 038c:661-662 with first pass language/INDEX.txt L09/F2; X04/F9 (075); 141:134-135 and X08/F1-F9, X08/F5 confirmed unsafe high (081); X09/F3 (082); 137:244-246 (130's kit). 042: docs/work-orders/042-build-tombstones-and-inventory.md:52-61. 077: 077-spec-reconcile-fenced.md:592-598; first pass language/X07-retries-fence.md, F1 ('MC2's Fenced control could not see this'; the vector's title 'late answer permits actor retry'). MC4 not re-registered: docs/work-orders/081-spec-one-run-per-instance.md:622-624.

**Verifier.** partly. Reproduced by script: 52 orders whose final verifier found nothing, 33 with no verifier after the last round, and 36 with musts left. All 14 named orders are among the 52. 042's Record (042-build-tombstones-and-inventory.md:53-61) and 150's test history (6ce8d136, then #260) are as stated.

**Verifier's counterargument.** Of the 14, only about 7 are escapes of a defect.
- 131, 132 and 140 were caught before merge by the Mac step.
- For 117, 119 and 120, the 'fault' is order 121's set of plan changes, mostly re-splits and the admin's answers.
- For 069 and 069b, the reference deliberately says 'A changed control value (REV-007) is a shown difference, never reverted by this planner ... No control call is inferred' (docs/implementation/plan-and-apply/README.md:214-215). S4's planning later needed it: a new need, not a miss.
- 038c's L09/F2 is the owner's to decide.
For 150, the lane coordinator's own Claude review also found nothing (150:107-108), so 'another model: plausible' is weakened. The seven unsafe on 077's fence are six.

**Verifier's corrections.** State two counts: faults that reached main after a clean verifier, and faults the existing pipeline caught before merge. Six unsafe on 077.


## FR-497

**Verifier musts that were wrong: none is recorded, and the Records could not show one**

G4 I7 · fact · verifier: supported · decides: none

No Record names a verifier must that the coordinator rejected or that proved wrong. Four Records say the opposite in words: the faults of orders 020, 021, 022 and 023 were 'each real' (17 faults).

That is not a measured precision of 100%.

- The design lets the coordinator pass on only 'the verifier's findings it accepts', and no Record has a place for the ones it did not.
- The verifier's raw reports are outside the repository.
- Orders 020 to 066 give no severities, and twenty passes of the later admin orders give no counts.

The nearest cases:

- 077's third verifier raised a fault outside the order's scope; the admin accepted it and widened the scope.
- 120's verifier held a plan to order 081 before 081 had merged; the Record treats that as right.
- In 131 it was the coordinator's diagnosis that was wrong, not a verifier's finding.

The one place precision was measured is this review's first pass, on the coordinator's model: of 263 language findings none was refuted and 91 (35%) were weakened by the verifier; of 182 approach items 127 were supported only in part. So a reader on the most capable model overstates too, and its musts need checking against the files before a round is spent on them.

What follows: the Record gains two numbers for each pass, musts accepted and musts rejected, each rejection with its reason. I8 fixes how they are counted.

**Evidence.** `awk` over the Record sections of all 72 order files and docs/implementation/WORK_ORDERS.md:224-1367 for rejected, declined, not accept, overrul, wrong, mistaken, false, withdrawn, dismiss: no verifier finding is so described. 'Each real': WORK_ORDERS.md:723, :763, :797, :839. 'The verifier's findings it accepts': WORK_ORDERS.md:84-85. docs/work-orders/077-spec-reconcile-fenced.md:592-595; 120-plan-providers.md:370-373 (081 merged later, 35d2e45a on 2026-11-12); 131-a63-ps-local-directory.md:125-128. Reports outside the repository: docs/implementation/HANDOVER.md:54-66 (the runners are kept outside it). First pass: the task's counts (263 findings: 165 confirmed, 91 weakened, 7 known, none refuted; 182 items: 55 supported, 127 in part), which language/INDEX.txt reproduces (165, 91, 7).

**Waits.** I8's measures

**Verifier.** supported. I grepped the Record sections of all order files and WORK_ORDERS.md:224-1372 for reject, declin, not accept, overrul, mistak, withdr, dismiss, disagree and 'not a fault'. No verifier finding is described as rejected. 'Each real' appears for 020, 021, 022 and 023 (3+5+3+6 = 17). WORK_ORDERS.md:84-85 says 'the verifier's findings it accepts'. 120:370-373 reads as cited. Of the first pass's language findings, 91/263 = 34.6% were weakened.

**Verifier's counterargument.** 'Weakened' in the first pass means narrowed, not wrong. It is not a false-positive rate, so the comparison of precision is loose.


## FR-498

**A77: both verifiers read the same diff for the next ten orders, with the decision rule fixed first**

G4 I8 · experiment-change · verifier: partly · decides: coordinator

**Row:** A77 in ROADMAP.md. It asks whether Codex's work is as good and whether the admin spends less; it has no pass condition, and the admin's spend was never recorded.

**Why:** every comparison of the two models in the tree is confounded (I1). No two verifiers ever read one diff under one brief.

**Claim to test:** on the same diff and the same brief, a reader on the coordinator's model finds the must-level faults the Codex reader finds, and more; the Codex reader then adds too little to keep.

**Method, fixed before the first order.** For the next ten dispatched orders (at least 2 specification, 3 engine, 3 package or harness, 2 paper), two fresh read-only readers read the round-1 checkpoint, neither seeing the other's report: the Codex verifier as it runs today, and the coordinator's-model reader, both under verify.md as it stands. The coordinator checks every must against the files before any round, and records it as accepted or rejected with the reason.

**Measures, per order:**
- accepted musts by each reader, and those found by both;
- rejected musts by each (the count I7 lacks);
- minutes and tokens per read;
- faults found later in that order's work by the check run, CI, a later round or after merge, each traced to the round-1 diff or to a later fix;
- where both found some, the estimate of what is left: n1 times n2 over the overlap, less those found (capture and recapture, as inspection teams use it).

**Decision rule:**
- (a) The Codex read is dropped for a kind if, over the sample, it adds fewer than one accepted must in five orders that the other reader missed. Otherwise it stays as that kind's second round-1 reader, at Codex's cost rather than the coordinator's.
- (b) A second reader on the coordinator's model stays for a kind where the estimate of what one reader leaves is one must or more per order.
- (c) A77 gets its number: later faults per merged order and reads per order, against this review's baseline of 14 in 52 clean orders and 1.51 verifier passes.

**Cost:** ten extra Codex reads and the coordinator's adjudication. What merges does not change: from the first order the coordinator's-model reader is the verifier of record, as the owner prefers, and the Codex read is the control.

**Limit:** ten orders show only large differences. Extend the sample by kind where the rule is close.

This adds measures to A77 and does not replace its question. The first pass's proposal for A77 (P01 I9, as its verifier corrected it) stands beside it.

**Evidence.** ROADMAP.md:70 (A77: 'reviewed after five orders'; 'admin time was not recorded, so no complete spend comparison is available'); ROADMAP.md:96-97 ('Each experiment states, before it runs, what it must show and how each measure is counted (ruling 45)'). No paired reads: every second verifier in the Records read a changed diff (docs/implementation/WORK_ORDERS.md:806, :908; the lane's second passes). docs/implementation/WORK_ORDERS.md:99 ('A skeptic per angle pays for itself'). Baseline: I6 and I2. The first pass's verifier proposed two independent verifiers on round one (approach/P01-work-orders-lanes.md, I1 and I7, counterarguments).

**Waits.** The next ten orders; the choice in I2's table between one and two readers per kind. Tell the owner, since A77's source is ruling 48

**Verifier.** partly. ROADMAP.md:70 has no pass condition and says admin time was not recorded. ROADMAP.md:96 requires measures fixed before a run. No Record shows two verifiers reading one diff: the second verifiers of 022 and 025 read changed diffs (WORK_ORDERS.md:806, :908).

**Verifier's counterargument.** The design cannot decide what it sets out to decide.
(1) The rule is applied per kind, but ten orders give 2 or 3 per kind, so 'fewer than one unique must in five orders' cannot be judged.
(2) Lincoln-Petersen with two readers is unstable and runs low when the readers find different kinds of fault, as two models will. Inspection studies (Eick et al. 1992; Briand et al. 2000) found two-inspector estimates unreliable.
(3) Both readers work under 'verify.md as it stands', while I2 adopts a wider brief at once. That tests a configuration the pipeline will not use.
(4) The coordinator, who shares one reader's model, judges acceptance. Adjudication should be blind to which reader found what.
(5) No rule says when two musts are the same fault.
Better: run until a stated number per kind (or pool kinds), use the brief that will be used, strip reader labels before adjudication, and fix the matching rule in advance.

**Verifier's corrections.** Pool the kinds or extend the sample to at least five per kind. Use the I2 brief for both readers. Blind adjudication. A matching rule fixed before the first order.


## FR-499

**A78: in the dogfood the verifier becomes another client on another model, with its reach declared**

G4 I9 · experiment-change · verifier: partly · decides: coordinator

**Row:** A78 in ROADMAP.md.

**What exists:** stage 1 runs `worker-1` and `verifier-1`, both `codex.v2` Agents of the `conduit-core` Fleet. The verifier's read-only limit is a harness check after each turn, where the plain runner has a read-only sandbox. The first pass (P06 I1) found that Conduit decided nothing about either Agent beyond root's right to take turns.

**Change, at the re-pin of the dogfood engine:**

- Add a verifier slot that is a Claude Code Agent on the coordinator's model. The Type already has a `model` input, and `claude-code.v2` passed the smoke test.
- Give it a Conduit to the worker's tree that lists reads, and one to a scratch tree of its own for builds.
- Keep `verifier-1` for I8's paired reads.
- For orders 4 and 5, write into the row that the coordinator Agent, which reviews and decides, runs on the coordinator's model. The row says only 'a coordinator Agent of that Fleet', and both slots today are Codex.

**Claim:** Conduit can carry the pipeline the owner prefers: a worker and a verifier that are different clients on different models, with the verifier's reach declared instead of assumed.

**Measures, fixed before the first such order, per verifier turn:**
- set-up seconds (the engine's restart included) and work seconds;
- whether the report is valid against its schema at the first try;
- failed turns by cause, and guard trips;
- files changed outside the verifier's scratch tree;
- accepted musts.

**Pass condition:** over the next four orders through Conduit, every verifier turn ends with a schema-valid report and no file outside its scratch tree changes. A change voids that read, which is repeated on the plain runner, and is logged as a finding of the first build before the next order. Under one operating-system user the guard is an observation, not proof, and each result says so.

**What it buys:** the dogfood exercises both of S1's client kinds daily, and the verifier's independence becomes something a document states.

**What it costs:** reads through Conduit draw on the owner's primary Claude login, which Agents share until order 050; the second accounts stay reserved; each fresh conversation costs an engine restart of 171 to 191 seconds today; `claude-code.v2` has already written inside its slot's `.git` unprompted.

What orders 4 and 5 should test is the owner's question in the first pass (P06 I4). This item only says which model the appointed coordinator runs on.

**Evidence.** ROADMAP.md:71 (A78's row); docs/implementation/HANDOVER.md:121-126 ('two `codex.v2` Agents of the `conduit-core` Fleet, `worker-1` (4-hour deadline) and `verifier-1` (1 hour)'; 'checks guards after each turn: the slot's HEAD unmoved, the verifier changed nothing'), :134 ('`claude-code.v2` runs with no prompt'), :147-148, :153-159 (the re-pin; the store initialized again after 050); IN_FORCE.md:314 ('A coordinator Agent of the conduit-core Fleet, appointed by root, dispatches orders 4 and 5'); packages/agent-claude-code/contracts.yaml:18 (`model: {type: string, default: ''}`); docs/proposals/ruling-72/README.md:80 and docs/guides/full-review.md:41-43 (second accounts 'never as capacity for work orders'); docs/proposals/ruling-48/README.md:30 (point 2: Agents share the owner's login until each has a home of its own); docs/work-orders/042-build-tombstones-and-inventory.md:49-52. First pass: approach/P06-dogfooding.md I1 ('A read-only verifier. Plain: a read-only sandbox. Conduit: full access, checked afterwards by a guard'), I4 and I10.

**Waits.** The dogfood engine's re-pin; the next order through Conduit (046). The content of orders 4 and 5 waits on the owner's answer to the first pass's P06 I4

**Verifier.** partly. These cite correctly: HANDOVER.md:121-126 (worker-1 and verifier-1, both codex.v2), :134 ('claude-code.v2 runs with no prompt', one of six smoke checks), :147-148; packages/agent-claude-code/contracts.yaml:18 (the `model` input); IN_FORCE.md:314.

**Verifier's counterargument.** (1) 'A Conduit to the worker's tree that lists reads' presupposes an answer to the owner's open question, first pass P06 I4: since ruling 75 B1 the tree is a fixed folder input reached by the Agent's own tools (CTR-014), not a grant. And while own tools are unconfined (ruling 70), a declared read-only reach limits nothing.
(2) The measures omit what P06's verifier asked for: what the verifier could read of the worker's conversation. Under one OS user, a Claude verifier can read the Codex worker's sessions.
(3) It draws on the same Claude login and allowance this run exhausted.
(4) Slots are set up once by root (IN_FORCE.md:314). The natural point to add a slot is 050's re-initialization (HANDOVER.md:157-158), not the re-pin.
(5) The 171 to 191 s restart was measured on the old pin.

**Verifier's corrections.** Add the slot at 050's re-initialization. Measure what the verifier can read of the worker. Leave the tree's reach to the owner's answer to P06 I4.


## FR-500

**Record the owner's preference, change the six lines that say the opposite, and state what it does not move**

G4 I10 · recommendation · verifier: partly · decides: coordinator

The owner's preference of 2026-11-14 (reviews, verification and decisions on the most capable model; implementation and test runs on Codex or a cheaper Claude model) is not in the tree. The tree says the opposite in six places:

- ruling 48's record gives 'verifications' to Codex sessions;
- IN_FORCE.md topic 15 says Codex sessions carry out work orders, of which a verification is one kind;
- WORK_ORDERS.md defines the verifier as 'a second, fresh Codex session';
- the handover says so twice;
- verify.md opens 'You are a Codex verifier'.

The reason ruling 48 gave was the owner's own: 'so as to not burn all my tokens here'.

1. **Record it.** It changes who verifies, a decision in force, so it is more than a clarification under ruling 67. Carry it in the ruling this review produces, in the owner's words, with one change that rewrites those six lines. If the owner calls it a clarification, it is a dated later answer under ruling 48 instead. Either way the words must be the owner's, and the coordinator does not have them in the tree.
2. **Say what it does not move.**
   - The fresh author of the S1 rerun stays Codex, and both runs must pass (ruling 48, point 4; ruling 71, point 3).
   - Fresh reads by Codex stay the test of what a reader without context can do. They are an experiment's subject, not verification.
   - The second accounts stay reserved and are not capacity for reads (ruling 72's later answers).
   - The lane's merge rule stands as written. Ruling 71 says 'a fresh verifier' and names no model.
3. **Log one rule with it, for the owner to strike.** When the Claude allowance nears its limit, dispatch stops before reading does, nothing merges unread, and the owner is told. That is what the owner ruled for the Codex allowance (ruling 71, point 6, and its later answer: 'just pause and let me know'), applied to the allowance that verification now draws on.

Cost: one docs change. It buys a pipeline whose documents say what it does. The next coordinator would otherwise read WORK_ORDERS.md and start a Codex verifier.

**Evidence.** docs/proposals/ruling-48/README.md:37-39 ('Experiments, increments, sweeps, verifications and fresh reads are given to Codex sessions as work orders, so that the coordinator's own session is spent on planning and decisions'), :18-19 ('so as to not burn all my tokens here'), :31 (point 4); IN_FORCE.md:309, :308, :313; docs/implementation/WORK_ORDERS.md:77; docs/implementation/HANDOVER.md:20, :61; docs/work-orders/verify.md:3; docs/proposals/ruling-71/README.md:26-27 ('once CI is green and a fresh verifier is clean'), :37 ('both runs must pass: the coordinator's and the fresh Codex author's'), :48 (point 6) and its later answer of 2026-11-12; docs/proposals/ruling-72/README.md:80; AGENTS.md, 'Where things go' (an answer that changes no decision gets no number); docs/implementation/HANDOVER.md:4-5.

**Waits.** The owner's words for the ledger; the edits to WORK_ORDERS.md, the handover, verify.md and IN_FORCE.md topic 15

**Verifier.** partly. The six lines exist: docs/proposals/ruling-48/README.md:37-39; IN_FORCE.md:309; WORK_ORDERS.md:77; HANDOVER.md:20, :61; verify.md:3. 'So as to not burn all my tokens here' is in the owner's words at ruling-48 :18-19. ruling-71 :27, :37 and :67, and ruling-72 :76-80, read as cited.

**Verifier's counterargument.** The owner's own words in ruling 48 (README:16-21) give Codex 'some of these experiments' and keep 'all of the meticulous planning and decisions' with the coordinator. 'Verifications' entered at :37 as the coordinator's paraphrase. So the 2026-11-14 preference reverses no decision of the owner's. Under AGENTS.md ('A line that differs from the owner's words ... is a defect') and ruling 67, it belongs as a dated later answer under ruling 48, with the paraphrase corrected. A new ruling is not needed, and the owner's aim is few rulings. The item's first choice, a new ruling, is the weaker one.
IN_FORCE.md:309 does not mention verification. docs/guides/full-review.md:107-109 also ties orders to a read-only verifier.
The stop rule should also cap reads, not only order dispatch.

**Verifier's corrections.** Make the later answer under ruling 48 the primary form, and correct :37's paraphrase. The coordinator decides the form; the owner confirms the wording.


## FR-501

**What the tree cannot show about verification**

G4 I11 · fact · verifier: supported · decides: coordinator

Seven limits on every number above, each something the Record should start to hold.

1. **The verifier's model and flags.** The tree gives the worker's dispatch command, with `model_reasoning_effort="high"`, and not the verifier's. No Codex model is named anywhere. The runners are outside the repository.
2. **Who the other readers were.** The lane's 'fresh reader' and 'fresh skeptic' have no model recorded. The lane coordinator was a subagent of a Claude Opus session, so they were probably Claude; the tree does not say.
3. **What reviews before dispatch found.** 51 order files were reviewed before dispatch; the Records name findings for one (141) and the plans' four.
4. **Rejected findings.** None is recorded (I7).
5. **Severity and counts.** Orders 020 to 066 give faults with no severity; twenty passes in 037 to 082 give no count.
6. **The coordinator's spend.** Never recorded, though it is the allowance that ran short and the one verification will now draw on.
7. **Orders that did not merge.** 049a1, 071, 080 and 144 have no Record on main; 049a1's five rounds with new musts in each are known only from the handover and the review's brief.

The smallest record that would answer this review's questions next time: for each pass, the reader's model, minutes and tokens, musts accepted and rejected; for each order, CI's result at merge on each platform, and later faults traced back to it. The runner can write all but the last.

**Evidence.** docs/implementation/WORK_ORDERS.md:50-55 (the worker's command only), :156-158 ('The admin's own time was not recorded'); `grep -rn 'model_reasoning_effort'` over docs/ finds that one line; docs/implementation/HANDOVER.md:54-66 (runners kept outside the repository), :99 ('The lane coordinator was a subagent of the outgoing session'), :211; `git log --format=%b | grep -o 'Co-Authored-By: [^<]*' | sort | uniq -c` gives Claude Opus 5.5 on 180 commits; docs/guides/full-review.md:4-5, :28-29 (049a1: 'five rounds, new musts in each'); reviews before dispatch: first pass, approach/P01-work-orders-lanes.md I10 (supported); `grep -c '^## Record'` finds none in the files of 049a1, 071, 080 and 144.

**Waits.** I8's measures; the first pass's P01 I9 (A77's record)

**Verifier.** supported. WORK_ORDERS.md:50-55 gives only the worker's command. Grepping docs/, experiments/, ROADMAP.md and IN_FORCE.md for Codex model names finds none. 180 commits are co-authored by Claude Opus 5.5. HANDOVER.md:99 says the lane coordinator was a subagent. No `## Record` exists in 049a1, 071, 080 or 144.

**Verifier's corrections.** `model_reasoning_effort` appears twice under docs/: WORK_ORDERS.md:53, and 050a-agents-v2-own-tools.md:237, which is the Agent package's flag. WORK_ORDERS.md:77 does state the verifier's read-only sandbox, so it is the verifier's other flags that are missing.


## What the verifier says the analyst missed

- **The Claude readers in the evidence were Opus, and the owner prefers the most capable model for review** The outgoing session, the lane coordinator and its fresh readers and skeptics were Claude Opus (180 commits co-authored by Opus 5.5; HANDOVER.md:99). The owner's preference names the most capable model, which now differs from Opus. Opus readers also passed 132, 140, 145 and 150 (their Records' 'review found nothing'). No item separates which Claude model did a read, so nothing in the tree measures the yield of the model the owner chose.
- **The cheapest change: a verifier that can run the checks** P01's verifier proposed it: a writable build directory outside the tree. 104's verifier already reran prebuilt test binaries (104-mc1-model-checking-harness.md:177), and CI already builds a nextest archive. The analyst adds a separate Codex check session and a reader that 'runs nothing', without weighing one session that reads and runs.
- **Existing paradigms for the checks are not named** Removing a fix to see its test fail is mutation testing, which cargo-mutants does on a diff. Lock-handoff races call for deterministic concurrency testing (loom, or the project's own Stateright) rather than repetition. The brief asks that the system be named.
- **Cost in the owner's allowances** The critic asked for cost in the allowances (approach/critic.md:74). The analyst gives sessions and says tokens are unrecorded. The request that started this run ('we ran into our limits') shows the Claude allowance binding again during the review itself. That bears directly on 2 reads per order on the coordinator's model.
- **Absence of CI results in the Records** No Record states CI's result. The inference that CI found nothing before merge is unfounded; the handover says the admin waits for CI at each pull request (HANDOVER.md:74). Recording CI's result per platform at merge is in I11's list, but I5's reading should be withdrawn.

## The verifier's notes

The analyst's arithmetic holds; most of the claims built on it are overstated. Every count I reran from the first pass's table reproduces: 246 and 253 musts; 217 rounds and 183 passes; 52, 33 and 36 final states; 64 promises and 78 findings (50, 24, 4, 14 unsafe); 66 plan items.

What fails:
- 077's fence carries 6 unsafe findings, not 7, and two of its ten findings are on CTR-002.
- Three of the 14 'clean, then a fault' cases (131, 132, 140) were caught before merge. Five more (117, 119, 120, 069, 069b) are re-plans or a deliberate design, not misses.
- The 'other model' column rests on Opus readers and different briefs. The Opus lane coordinator's reviews passed 145 and 150.
- I4's remedy keeps the coordinator's own text inside one model.
- I2 drops the reader where the Codex verifier found redaction musts, and miscounts the orders to the pin.
- I8 cannot decide per kind on ten orders.
- I10 should be a dated later answer under ruling 48, not a new ruling: verifications were never in the owner's words.

No item asks the owner again what the owner has answered. I9 does presuppose the open P06 I4 question.

What I read: the full-review brief and AGENTS.md; P01 in full; P06 I3, I4 and I7; the approach critic's section; verify.md; WORK_ORDERS.md:40-130 and the Records cited; HANDOVER.md:1-160 and :190-230; IN_FORCE.md topic 15; the records of rulings 48 and 71; checks.yml; ROADMAP.md rows A77 and A78; the cited Records of 038b, 038c, 041b, 042, 045, 068, 069b, 070, 077, 100, 103 to 105, 107, 109, 111 to 113, 117, 119 to 121, 124, 126, 128, 131, 132, 135, 137 to 140, 142, 148 to 150; language/INDEX.txt. I also ran git commands to find which order first added each promise ID.

Not read: order bodies beyond those cited, and the runners and reports outside the repository. Nothing was built.
