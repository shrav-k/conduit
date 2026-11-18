# The full review: questions for the owner

Answered on 2026-11-14 by [ruling 78](../ruling-78/README.md): every question as recommended.

Not binding. Part of [the full review](README.md) of 2026-11-14, at commit `2eb8f258`. The questions it puts to the owner, each with lettered options and a recommendation. The answers become one new ruling (the next number, 78 at the reviewed commit); an answer that changes no decision is added, dated, to the record it touches, as ruling 67 provides. You may answer "as recommended" for any or all of them; the [front page](README.md#the-questions) lists the recommendations in one place.

## The approach and the experiments

### 1. The order of the work until the S1 rerun's build is pinned

**Today.** Ruling 71 point 1 (2026-11-11) has one or two lane coordinators work beside the admin, to save about a week (`IN_FORCE.md:308`).

**What is new.** The lane stopped on 2026-11-12 because of your usage limits (`docs/implementation/PROVIDERS.md:36-38`). The Claude allowance ran out again on 2026-11-14 (your message). The review's verifiers also confirmed 27 findings of unsafe faults in the text (some faults found twice), some of them under batch 1's next orders. Under every option the coordinator runs one sequence of about 30 orders to the pin (the tag that fixes the build the S1 rerun measures), with about a dozen small ones beside it (table below). [orders.md](orders.md), which also counts the small paper and pipeline orders, has the pin wait on 45 orders, with 20 more beside them.

**Part 1. A second Claude session until the pin**
- **(a) As ruled, narrowed by the coordinator (D48: no further provider is built until one runs end to end):** a lane restarts for checks that need no engine. These are A74 (the two-machine checks with no engine), two provider suites, the lost-response probe, A73's desk check (1Password's unattended reads, from its documentation) and A7's row 7 (a bypass attempt by a plain process) (both use no model turns), and the blind-verdict read (fresh readers judge test steps from `spec/` alone) once the two specification orders SO-A and SO-B (step 5 below) land. That is about 5 to 7 orders and a day of work. It spends Claude allowance, and your primary subscriptions for the probe, on evidence the rerun does not need. If A74's result is read as deciding whether the engine must store the stops it owes, the record says that this reopens ruling 42's later answer.
- **(b) None:** one coordinator; the lane restarts at the pin.
- **(c) On the rerun path:** a second coordinator writes and reviews the author chain (043a to 052b: the short form's translator, the shell's commands, and plan and apply), and the admin merges. The pin may come sooner, if the allowance can carry two sessions.

**Part 2. Sign-in before the pin**
- **(a) As planned:** the pin waits for six sign-in orders (12 to 17 worker hours), and you sign each dogfood Agent in once. Order 048a-2 also waits for your answer to question 9, on "the engine's own panel".
- **(b) Pin without it:** the rerun uses the shared primary logins, as on 2026-11-10, and the `account:` lines still count. Sign-in follows the pin. This is close to option (c) of R1 in batch 1's plan, which the plan said "skips what ruling 49 tests"; you chose (b) (ruling 68 point 3).

**Recommended: 1(b) and 2(a).** While the allowance is the limit, a second session only splits it. If you can add capacity, take 1(c), not 1(a). 2(b) only moves the six orders, since S2 to S4 need sign-in right after the pin. The claim's draft and the corrected probe rows go ahead under every option.

**Changes:** 1(b) or 1(c) amends ruling 71 point 1 until the pin, and the roadmap's "In parallel" row. 2(a) changes nothing. 2(b) would change what the rerun measures under ruling 68 point 3 and ruling 76 point 4 (`IN_FORCE.md:277`, `:78`).

**Waits on it:** the lane's restart, and what orders 050 and 053 contain.

**Findings:** [FR-266](findings/P01-work-orders-lanes.md#fr-266), [FR-362](findings/P08-progress.md#fr-362), [FR-363](findings/P08-progress.md#fr-363), [FR-387](findings/E01-s2-s3-s4.md#fr-387), [FR-437](findings/E05-falsifier.md#fr-437), [FR-488](findings/G3-native-probes.md#fr-488), [FR-490](findings/G3-native-probes.md#fr-490).

**Supporting table: the sequence to the pin, under every option**

| Step | Work | Orders |
| --- | --- | --- |
| 1 | `main`'s CI passes three runs in a row. The fix merged after the reviewed commit (#266); the faults were in the tests | 0 (a check) |
| 2 | Order 071 finished: round 3, the Mac tests, merge. This unblocks SV1 | 1 |
| 3 | The fence's model rechecked (the fence is the evidence that a lost call can no longer arrive; question 6): a recount against each model's own commit, then MC2 (the model of a call's identity and retries) rerun on order 077's text. A refutation is registered as the expected result, with OPS-030's settled-reconcile mutant and a variant of DL2 | 5 for steps 3 and 4 |
| 4 | The dogfood engine fixed, then re-pinned once you answer question 6, since a re-pin to `main` brings in order 077's fence (until then it stays on its present pin). One engine order, which orders.md splits into #13 and #14: a stop is recorded only once the run's processes are gone, and a dispatch deadline cancels the call. The `.v2` wrappers stop their client's whole process tree, and failed turns carry fields | |
| 5 | SO-A (whether `observable` binds; a Pool named in a bound's targets; how sign-in holds a call), written before 046a, 047 and 048a-2. SO-B (the fence, retries, resending, reconciling), checked by MC2 before it merges. SO-B is the size of 077: a first round of 5.5 hours, four rounds in all. It carries the narrowing of the fence's evidence; the rest of the fence's rewrite, and the interim fixes F12 and F13, wait for your answer to question 6 | 2 |
| 6 | Batch 1's core chain, run through Conduit's own Agents: S0b, S0c, 046a, 046b, 047, 048a-1\*, 048a-2\*, 048b-relay\*, 048b-join\*, 050\* | 21 for steps 6 and 7 |
| 7 | The author chain, which 1(c) would give a second coordinator: 043a, 043b, 051a, 051b\*, 052a-1, 052a-2, 052b | |
| 6 and 7 | Four checks placed among them: the specification's by-rule examples run through the engine; A5's stage-0 probe of the real clients, before 050 is written; the sign-in model MC9a, registered when 048a-2 is drafted; the plan-and-apply cases run through the engine, before 052a | |
| 8 | Order 053, the tag, the pin | 1 |
| Total | 31 at the reviewed commit; 30 once #266 is counted | about 30 |
| Beside the path | About 13 small orders that the review's fix lists place before the pin: CI, records and checks; two documents 052a needs first; a wording order for `spec/`; the store's growth recorded; the core's purity checked. Under 1(b) the one coordinator reads them too | about 13 |
| Parked | 049a1, 080 and 144 wait for the pin, so each is rebased only once. The providers' and machines' specification orders (SO-C and SO-D in orders.md) run after the pin. A new conversation without an engine restart comes after the pin, so the packages the rerun pins stay as they are | |
| Under 2(b) | The six orders marked \* move after the pin. The `.v2` packages can ship before sign-in only if the later sign-in declaration needs no default (NAM-024). Otherwise the rerun takes them from their branch | |

### 2. S1's bar

**Today.** Under ruling 71 point 3, both runs, the coordinator's and a fresh Codex author's, must take fewer lines than the hand-made baseline (A70) on most of S1's seven tasks. On no task may a run take more than one and a half times the baseline (`IN_FORCE.md:266`).

**What the review found.** On ruling 76's forecast a run passes with no task to spare, and the same project written in block layout (YAML's usual one-key-per-line form, against the compact flow layout the forecast uses) fails (table below). Ruling 63 point 1 reports "the block-style reading" beside "the primary one", but no ruling says whether the primary count is the layout the author wrote or flow style. The specification's only short-form project, which the fresh author reads, is S1 itself.

**Options.** All are chosen before any count from the rerun exists.
- **(a) As now:** a single line can decide the result.
- **(b) Score ruling 45's two groups, in both runs:** fewer lines on setup (T1 and T2) and fewer on change and repair (T4 to T7). T3 and the per-task limit drop out. On the forecast, a run then fails only if it writes 23 more lines on setup or 6 more on change and repair. This is chosen knowing the forecast, and it makes a pass likelier.
- **(c) As (b), but only the coordinator's run gates**, since A70 was registered as one author writing both sides, the baseline and the Conduit project. The fresh author is judged on the success criteria in force. The gate then rests on the run that knows the design.
- **(d) A ceiling:** the coordinator's run stays within one and a half times each group's baseline. The fresh author is judged as in (c).
- **(e) No line rule:** lines are reported, and the success criteria judge.

(d) and (e) drop the thesis registered before the build, that Conduit "costs an author less", after two runs failed it.

**Recommended: (b).** It keeps the claim and both runs, as you ruled. It removes what a single line can tip, before the data. Its weakness: block layout still fails setup.

**With any answer, unless you strike it:** lines count as the author wrote them, with the other readings reported beside; T3's commands count as inspection effort; the fresh author's pack holds nothing of S1's design, so that example in `spec/` is replaced; measures added now never gate.

**Changes:** ruling 71 point 3 and ruling 47's bar row, with ruling 63 point 1 clarified; `IN_FORCE.md:266` and `:280`; one example in `spec/`.

**Waits on it:** the measurement plan of order 053. Batch 2's merge and findings 4-01 and 4-11 still wait on your judgment of the bar.

**Findings:** [FR-277](findings/P02-s1-bar.md#fr-277), [FR-281](findings/P02-s1-bar.md#fr-281), [FR-285](findings/P02-s1-bar.md#fr-285), [FR-366](findings/P08-progress.md#fr-366).

**Supporting table: lines per task and per group**

| Task | Group | Baseline (A70) | Limit (1.5 times) | Forecast, flow layout (ruling 76) | Block layout, paper count |
| --- | --- | ---: | ---: | ---: | ---: |
| T1 | Setup | 33 | 49.5 | 21 | 40 |
| T2 | Setup | 16 | 24 | 5 | 18 |
| T3 | Inspection | 0 | 0 | 0 | 0 |
| T4 | Change and repair | 8 | 12 | 3 | 3 |
| T5 | Change and repair | 3 | 4.5 | 2 | 2 |
| T6 | Change and repair | 2 | 3 | 2 | 2 |
| T7 | Change and repair | 2 | 3 | 2 | 7 |
| Setup | T1, T2 | 49 | | 26 | 58 |
| Change and repair | T4 to T7 | 15 | | 9 | 14 |
| All seven | | 64 | | 35 | 72 |

How to read the table:
- **Under (a):** the forecast is fewer on exactly four tasks (T1, T2, T4 and T5), which is the least that passes. One more line on T5 would fail the run, and T3, with a baseline of zero, can never count as fewer. Block layout is fewer on only two tasks, and T7 is over its limit, so it fails.
- **Under (b):** the forecast has 23 lines to spare on setup and 6 on change and repair. Block layout fails setup (58 against 49). It passes change and repair (14 against 15), although T7 has 7 lines against a per-task limit of 3.
- **The first runs** of 2026-11-10 wrote 332 lines (the coordinator's, in YAML) and 646 (the fresh author's, in JSON), against 64.

Sources: `experiments/007-s1-baseline/results-2026-11-09.md:19`; `docs/proposals/ruling-76/README.md:9-11`, `:39-40`; `experiments/023-paper-count/results-2026-11-11.md:19-28`; `docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md:34`. The block counts were made before ruling 76's readings, and no block recount exists since.

### 3. v0's gate and scope

This has three parts; answer each on its own. Nothing on the rerun path waits on any of them.

**3.1 v0's feature list.** *Today:* dropping anything from v0 needs your ruling (`IN_FORCE.md:30`). Adding a feature needs one too, except small ones under ruling 71 point 4. But nothing says which version a ruled addition lands in. Between 2026-11-10 and 2026-11-12, 64 promises were added.
- (a) As now: a ruled addition lands in v0, and v0 grows with each ruling.
- (b) Close the list at what is ruled today. Batch 2, metrics and every fix stay as ruled. A later addition, small ones included, lands after v0 unless your words say "in v0", and it reaches you with its size. The cost is one line.

*Recommended: (b).* Nothing you ruled changes. *Changes:* one line beside the scope guard. *Waits:* batch 2's plan. *Findings:* [FR-470](findings/G2-size-brake.md#fr-470), [FR-472](findings/G2-size-brake.md#fr-472).

**3.2 What the model-checking row must show.** *Today:* v0 needs model checking to show that "the safety promises hold under loss, reordering and partition" (`IN_FORCE.md:267`). Ruling 75 E1 settled which promises, not which faults. *What is new:* the models' own account says its bounded tables cannot be combined into that result (`experiments/012-model-checking/G5.md:336-339`). 79 of the 114 promises the models hold were checked with no network cut. The text of 34 to 49 of the 217 safety promises the row covers (`IN_FORCE.md:267`) has changed since. Every option reruns the models on the new text.
- (a) Amend the row. Each modelled promise holds under loss and reordering. Where its parts sit on two machines, it also holds under a cut and its repair; the coordinator classes which promises those are, for you to strike. Every other promise has a safety vector or a stated reason, and every model finding is fixed or stated as a limit. Cost: vectors, not yet counted. Gives up: a cut on promises that sit on one machine.
- (b) Keep the words and read them in full: a cut added to the five models that have none, and two faults at once where a promise rests on two models. About five more lane orders (an estimate).
- (c) As now: you judge at publication, and the reruns have no fixed target, so they may need to run again.

*Recommended: (a).* It matches the roadmap's own wording of step 6 (`ROADMAP.md:31`), and it fixes the target before the reruns are registered. *Changes:* `IN_FORCE.md:267`, ruling 47's row, ruling 75 E1, A40's done-when and `ROADMAP.md:129`. *Waits:* the reruns' registrations. *Findings:* [FR-454](findings/G1a-model-checking-register.md#fr-454), [FR-448](findings/G1a-model-checking-register.md#fr-448), [FR-190](findings/X03-model-checking.md#fr-190).

**3.3 v0's security claim.** *Today:* ruling 47 writes every route around the engine that A7 (the probe of what a native Agent can bypass) finds into v0's public claim, and A7 does not block (`IN_FORCE.md:269`). The claim's parts are scattered. A20 (your open ruling on v0's authorization scope, raised by the plan audit's finding D5) and G8 (the independent builds' gate: every open question closed or put outside their scope) still wait for A7 (`docs/AUDIT.md:166`; `docs/implementation/INDEPENDENT_BUILDS.md:132`). The review drafted the claim, with its verifier's corrections.
- (a) The corrected draft becomes the draft for publication, and it grows with each route found. A20 and the plan audit's finding D5 (not this review's decision D5) close on it once you confirm the coordinator's map of A20's three questions: cross-path to ACC-009, issuer-independence to ACC-030, and provider scope to its promises or stated as open. The native probes (A5, A6, A7, A10, A22 and A73, on the clients, secret stores and the engine's own runner) are put outside G8's scope.
- (b) A20 and G8 wait until A7's rows that can run now have run: about one order and a day.
- (c) No public claim until a runtime provider that confines its Agents ships. This amends rulings 47 and 70.

*Recommended: (a).* A7's rows still run, as observation, and what they find fits sentences already drafted. *Changes:* this review's ruling closes A20 and D5 and sets G8's exclusions. *Waits:* G8, and A75's text. *Findings:* [FR-480](findings/G3-native-probes.md#fr-480), [FR-482](findings/G3-native-probes.md#fr-482), [FR-194](findings/X04-stated-limits.md#fr-194) (asked with the language questions).

### 4. One amendment to the accepted plans of S2 to S10

**Today.** You accepted these nine plans on 2026-11-11 (ruling 75 D1). None has run, so they can still be amended (ruling 45). The coordinator makes the changes that only define a count or add an observation reported apart. They apply under every option, each dated and open to your strike. Proposals that ask again what you answered on 2026-11-11 and 2026-11-12, with nothing new since, are left out.

**What is new.** Ruling 77 turned off the second machine's interop and gave its run user no sudo, so S3's accepted way to cut, heal and lose the machine can no longer be followed. Ruling 77 also made `wsl --terminate` on the one distro your stop command. S3's accepted loss step (O4) named `wsl --shutdown`, which stops Docker Desktop's distro too. And no open-source Linear MCP server found on 2026-11-12 runs unchanged against S2's test stub.

**The rows that need you** (table below):
- **S2:** the coordinator chooses a pinned open-source plain server that meets the published requirements, and logs the choice for you to strike. Measure D20's list of places (the round-6 measure of what can reach a credential, not decision D20 of this review) also gains another Agent's home and client login.
- **S3, option (E)** (S3's plan offered (A) to (C); a finding added (D), a relay on the Mac): the Mac's own firewall cuts the network, run by the coordinator's locked-down script under one administrator rule you add once. You still stop the distro at each loss and make two test cuts: 12 actions over four runs, not 72. A last phase is added to each run: it declares the machine lost while a test run is alive. That lets S3 show the owed stop you agreed to on 2026-11-12 actually ending a run.
- **S7, split:** the outside author works against a scripted consumer, so it needs neither your presence nor the second accounts. Real Agents on the second accounts then use the same packages. The cost: the coordinator, not the outside author, sets up the real Agents.

**Options.**
- (a) Accept the table.
- (b) Accept it, except the rows you strike or the other letters you pick. For S3, option (A) is your hand at every barrier: 72 actions, 36 to 72 extra minutes. (A), (B) a Windows-side mechanism set up once, and (C) a helper in the distro are the options of S3-OF in S3's plan, still open to you, where the coordinator recommended (A). (E) is new and replaces that recommendation.
- (c) As now: S2's server and S3's method stay open, and their runs wait. S7 runs as accepted.

**Recommended: (a).** No evidence case or measure is dropped. S3's owed-stop condition becomes able to fail. S3's fault harness is unblocked at the cost of one administrator rule.

**Changes:** ruling 75 D1, for S3's fault method, O4's command, P4 and runs, and for S7's tasks and run. Dated amendments go to experiments 013, 014 and 018, the second-machine guide, and the roadmap's S3 and S7 rows. `IN_FORCE.md:273` cites the new ruling. `spec/` does not change.

**Waits on it:** S2's runs; S3's fault harness and runs; S7's harness and run.

**Findings:** [FR-384](findings/E01-s2-s3-s4.md#fr-384), [FR-379](findings/E01-s2-s3-s4.md#fr-379), [FR-381](findings/E01-s2-s3-s4.md#fr-381), [FR-385](findings/E01-s2-s3-s4.md#fr-385), [FR-386](findings/E01-s2-s3-s4.md#fr-386), [FR-319](findings/P05-second-machine.md#fr-319), [FR-320](findings/P05-second-machine.md#fr-320), [FR-394](findings/E02-s5-s6-s7.md#fr-394), [FR-395](findings/E02-s5-s6-s7.md#fr-395), [FR-425](findings/E04-signin-isolation.md#fr-425), [FR-502](findings/G5-round6-evidence.md#fr-502), [FR-509](findings/G5-round6-evidence.md#fr-509), [FR-513](findings/G5-round6-evidence.md#fr-513).

**Supporting table: one row per scenario.** E and D numbers here are round 6's evidence cases and design critiques, placed by ruling 44, not this review's decisions D1 to D88.

| Scenario | What changes | Who decides | Your minutes | Round-6 cases and measures dropped |
| --- | --- | --- | --- | --- |
| S2 | The plain server, chosen by the coordinator. D20's list of places gains another Agent's home and client login. The coordinator also adds: an expected result for each task step; D05's five cases in one shared file, and a lane order so that one non-plain row makes an onward request; P5 judged against a snapshot; D20's attempts by a scripted probe that reads only planted markers, never a real credential; E07's prediction written down first; each D01 row's cost in lines; M1's trials and prompts fixed; one E17 case added and reported apart | You: whether the coordinator picks the server (S2's plan has it chosen with you), and D20's added location. The coordinator: the rest, which you may strike | Unchanged: 45 a run proposed, 180 for four | None: E02, E07, E13, E17, D01, D05, D09 and D20 stay |
| S3 | **Method (E).** The script is owned by root, cannot be changed by your user, takes no free arguments, and touches only the second machine's address. Two test cuts decide whether it works. Any path it misses gets a relay on the Mac, with no second question. Whether the harness keeps its own path to the second machine during a cut is untested. **Loss.** `wsl --terminate`; root still declares the machine lost, and the provider reporting it gone stays a separate case. **Added last phase.** A test server that ignores the end of its input, witnessed by the stop's ledger entry. A call the old run makes to the engine after the declaration must be refused. Two runs heal before sshd's 120 seconds and two after. Windows sleep is an untested alternative. **The coordinator adds:** the diff of the documents from their one-machine form; seconds to "unavailable", against SSH's 30, sshd's 120 and the machine provider's stated bound once it is written; three ordering cells after the MC2 rerun, reported apart; the revocation axis kept and labelled as a code-path exercise; D20 at each phase; a start waiting for an owed stop, tested in an undeclared cut; the cost of adding a machine; a practice node that is never counted | You: the method, O4's command, P4 and the added phase. The coordinator: the rest | One administrator rule, once. Then 12 actions over four runs, beside the proposed hour a run for enrollment, sign-ins and recovery. Under (A): 72 actions, 36 to 72 minutes | None: E15 (both axes) and D20 under a partition stay |
| S4 | D02 as a twelve-row table. A rubric for M2, applied by a judge who has not seen the harness. The lifecycle model asked first whether a Resource can outlive its retired Pool. E20 labelled an observation of a stated limit. The count of T1's repair corrected. Beside S4, not part of it: two lost-response cases on a small stub like experiment 008's, whose harness is archived. They run on the primary accounts, and you are told first. D4's replay on a second account stays | The coordinator | None added | None: E01, E09, E18, E19, E20, D02 and D10 stay. E18's boundary half comes to you only if the model shows its state cannot be reached |
| S5 | The right next step written for each conflict class. The branches listed. A readback case added to the fixture's own contract check. A plan-and-apply branch, reported apart, only if batch 2 has merged | The coordinator | None | None: E03, E04, E10, E16 and D10 stay |
| S6 | M1's expected value written as a number (zero). M2's retained-data checks as a fixed list. The plan's stray T4 restart removed. One local machine, as the README says. A child process that outlives a stop goes to the language list and the provider suite; here it is a measure with no pass condition. The fixture's ledger kept outside the state directory | The coordinator | None | None (S6 holds only D22) |
| S7 | The split (described above). The coordinator also adds: "intervention" defined; the author's pack frozen and hashed; added measures (doubts, files read outside `spec/`, time); D01's middle row defined by the logged reading of "adapter"; a second server and a self-check, both reported apart; a line in Limits | You: the split. The coordinator: the rest | Unchanged: your sign-ins for the real-Agent part. The tree gives no estimate | None: D01 stays |
| S8 | The claim stated in two parts. An expectation for "small adapters", not a pass condition. Seeded leaks the suite must catch. A hand-written oracle. Scripted checks beside the real run. One observation for A7 | The coordinator | None | None (S8 holds only D22) |
| S9 | T4(a) and M2 follow OBS-004 once the engine fix lands after the pin. A rubric for M2. Dispatches at start counted. Every request names the caller that sent it | The coordinator (`spec/` binds) | None | None (S9 holds only D22). M2's expected count is corrected |
| S10 | A predicted set of edits, reported beside D16 and not used as a pass list. `lift.v1` requests and document classes added to D16's ledger. The six implementations named. A ledger of behavior that differs across a swap. Stage R's checks. `ROADMAP.md:131` aligned. "Or else" put on the strike list | The coordinator | None | None: E06, E14, D15, D16 and D09 stay |

No round-6 evidence case or measure leaves its scenario, and D22 still rides on every run ([FR-502](findings/G5-round6-evidence.md#fr-502)). The first pass's drops are not carried, because each would undo what ruling 44 placed or what you accepted in ruling 75 D1. Those drops were:
- D22's counts in S5 to S7;
- D01 in S7, and S2's adapter row;
- D05's onward case;
- D20 in S2's isolated runs, in S3's cells and in S3's isolated profile;
- E15's revocation axis;
- E17 out of S2;
- E09's replay off the second account;
- E18's boundary half;
- S5's four cases (E03, E04, E10 and E16) carried by vectors alone;
- E14's per-provider swaps.

E15's revocation axis stays only as a labelled code-path exercise, because its cells are decided on the Mac and cannot differ ([FR-380](findings/E01-s2-s3-s4.md#fr-380)). So the two combined losses the second pass found do not arise: no run measuring D20, and D01's three-row comparison lost ([FR-503](findings/G5-round6-evidence.md#fr-503)).

Findings by row: S2 [FR-379](findings/E01-s2-s3-s4.md#fr-379), [FR-386](findings/E01-s2-s3-s4.md#fr-386), [FR-425](findings/E04-signin-isolation.md#fr-425), [FR-508](findings/G5-round6-evidence.md#fr-508); S3 [FR-381](findings/E01-s2-s3-s4.md#fr-381), [FR-385](findings/E01-s2-s3-s4.md#fr-385), [FR-319](findings/P05-second-machine.md#fr-319), [FR-320](findings/P05-second-machine.md#fr-320), [FR-327](findings/P05-second-machine.md#fr-327), [FR-328](findings/P05-second-machine.md#fr-328), [FR-380](findings/E01-s2-s3-s4.md#fr-380), [FR-509](findings/G5-round6-evidence.md#fr-509); S4 [FR-383](findings/E01-s2-s3-s4.md#fr-383), [FR-510](findings/G5-round6-evidence.md#fr-510); S5 [FR-399](findings/E02-s5-s6-s7.md#fr-399), [FR-511](findings/G5-round6-evidence.md#fr-511); S6 [FR-397](findings/E02-s5-s6-s7.md#fr-397), [FR-512](findings/G5-round6-evidence.md#fr-512); S7 [FR-394](findings/E02-s5-s6-s7.md#fr-394), [FR-395](findings/E02-s5-s6-s7.md#fr-395), [FR-513](findings/G5-round6-evidence.md#fr-513); S8 [FR-413](findings/E03-s8-s9-s10.md#fr-413), [FR-514](findings/G5-round6-evidence.md#fr-514); S9 [FR-411](findings/E03-s8-s9-s10.md#fr-411), [FR-515](findings/G5-round6-evidence.md#fr-515); S10 [FR-410](findings/E03-s8-s9-s10.md#fr-410), [FR-444](findings/E05-falsifier.md#fr-444), [FR-315](findings/P04-providers-built.md#fr-315), [FR-516](findings/G5-round6-evidence.md#fr-516).

### Not asked about the approach

- **Group 5, separation and remote runs** ([FR-333](findings/P06-dogfooding.md#fr-333), [FR-325](findings/P05-second-machine.md#fr-325)): answered on 2026-11-12 by ruling 77 point 1 and ruling 75's later answers, with nothing new since. The coordinator's decisions are logged for you to strike: the remote-run gate as built until A74 reports, rootless Docker on the second machine, the shortcut table filled, and a "not shown" list on results taken under one user.
- **A statement that needs no answer:** at order 050's sitting you may also create the dedicated macOS user (ruling 75 B3). That would likely save a second round of sign-ins, but it pulls the dogfood harness's fix ahead of 050, and A22's keychain case too if a slot is Claude. Or leave it to B3's time, as you answered. Ruled on 2026-11-18: [ruling 84](../ruling-84/README.md), point 1. No dedicated user is planned; a sandbox method, Docker first, takes its place.
- **Group 6, the second accounts** ([FR-424](findings/E04-signin-isolation.md#fr-424)): answered by ruling 49's record (2026-11-10) and ruling 72's later answers (2026-11-12). A5 is rewritten in place as ruling 49's staged sign-in test and gates nothing ([FR-421](findings/E04-signin-isolation.md#fr-421)). The clash between ruling 61 B12 and ruling 75 B1 over A78's orders 4 and 5 comes to you with batch 2's plan ([FR-334](findings/P06-dogfooding.md#fr-334)).
- **E01 I6** ([FR-383](findings/E01-s2-s3-s4.md#fr-383)): "run S4 first" is moot before the pin, and its tightenings are question 4's S4 row. Its lost-response probe is the coordinator's, in 1(a) or after the pin.
- **P02 I11** ([FR-287](findings/P02-s1-bar.md#fr-287)): answered by ruling 71 points 2 and 4 on 2026-11-11. The coordinator logs the reading that your judgment of a failed rerun also releases batch 2.
- **P03 I3, as far as the bar holds finding 4-01** ([FR-292](findings/P03-plan-apply.md#fr-292)): answered on 2026-11-10 (ruling 55 A3: measured in the rerun). Its substance goes to the language questions.
- **Review item 9, the counting rules added after the baseline:** answered on 2026-11-10 ("Agree on all"). They stay reported apart.
- **P03 I12** ([FR-301](findings/P03-plan-apply.md#fr-301)), approving a plan by its digest: answered by ruling 55 A4 and ruling 76 point 2. It is kept as a dated note for the bar's judgment.
- **P07 I6, P08 I2 and P08 I16** ([FR-349](findings/P07-engine-build.md#fr-349), [FR-361](findings/P08-progress.md#fr-361), [FR-375](findings/P08-progress.md#fr-375)), what v0 holds: answered by ruling 47's list, 61 B1, 72 point 1 and 75 D1. P08 I2's order of work after the bar goes with the bar's judgment.
- **E02 I3** ([FR-395](findings/E02-s5-s6-s7.md#fr-395)): not a gate question. Its split is question 4's S7 row.
- **G1a I9** ([FR-454](findings/G1a-model-checking-register.md#fr-454)): asked once, as 3.2.
- **E02 I8** ([FR-400](findings/E02-s5-s6-s7.md#fr-400)), one fresh author for S5 and S6: answered by ruling 75 D3.
- **E03 I9** ([FR-414](findings/E03-s8-s9-s10.md#fr-414)), scripted clients for S8 and S10: answered by ruling 75 D2 and the later answers of 2026-11-12.
- **E03 I2** ([FR-407](findings/E03-s8-s9-s10.md#fr-407)), what a provider swap may edit: accepted D16 already lists it. Only the count is new, and it is reported beside D16.
- **P04 I6** ([FR-308](findings/P04-providers-built.md#fr-308)), no new provider code in a restarted lane: a rule for the coordinator, not a question for you.

## The language

### 5. May a reconciliation store a control's values or make an instance ready on an actor's word? (unsafe)

**Today.** A control sets a limit to 20. Its answer is lost, so its outcome is unknown. An administrator then sets 10. The control's sender then reconciles it: through its Conduit to the Pool, it records that the 20 took effect. The engine stores 20 over 10 at once, with no call to the implementation and no Conduit to the instance (REV-010, `spec/conventions/revisions.yaml:132-143`, `:156-157`). The server may then hold 10 while the engine sends 20. Recording that an unknown setup occurred likewise makes an Agent ready (LIF-028, `spec/conventions/lifecycle.yaml:597`). The coordinator's interim fix refuses that where a required part, such as the account check, never began. Still open: whether a reconciliation should store or ready anything at all.

**What is new.** Rulings 32 Q4, 33 F11, 41 item 15 and 66 point 4 made a reconciled control store its values, and on 2026-11-12 you kept reconcile.v1 as it is (finding 4-12). New since then: the overwrite above; a sender whose Conduit to the instance was closed can still store; and a setup recorded as having occurred although parts of it never began.

**Options.**
- **A. Record only.** A reconciliation records the outcome and what was checked. It stores nothing and makes nothing ready, and the settings stay reported unknown. The way on is to send again: the control, naming the current revision as HTTP's If-Match does, or a further setup, which leaves finished parts as they are (OPS-021). Confirming a retirement's shutdown stays. Cost: one order; the engine gets smaller.
- **B. Keep storing and readying, with two refusals for a control:** conflict where a change was stored after the control was sent, as under REV-002, and the reconciler must hold the control's Conduit at that moment. Cost: one order. Amends ruling 33 F11 and narrows ruling 66 point 4.
- **C. As now,** with a limit stated in `spec/README.md`: a record can store old values over newer ones. Cost: one sentence.

**Recommended:** A. It removes text and closes both paths, and a control and a setup can each already be sent again safely.

**Changes:** REV-010 keeps only the outcome, and REV-009 keeps the settings reported unknown. REV-013, OPS-031, reconcile.v1's `revision` input and the values kept for it go. LIF-028 keeps only confirming a shutdown, and ACC-044's provenance for a reconciled open goes. Amends rulings 32 Q4, 33 F11 and 41 item 15 where they reach reconciliation (`IN_FORCE.md:136-137`), and ruling 75 D1 for S5's accepted P5, T5 and V4. The vectors reconcile-revision-in-progress, reconcile-revision-succeeded and fleets/e3b/reconcile-control shrink.

**Waits on it:** the order for REV-010 and LIF-028; S5's P5, T5 and V4, accepted under ruling 75 D1, whose T5 expects the overwrite (`experiments/016-s5-control-contention/README.md:108-111`); S4's reconciliation runs (`ROADMAP.md:142`).

**Findings:** [FR-010](findings/L02-lifecycle.md#fr-010), with the two points its verifier added (a setup made ready; an open's provenance). The coordinator closes the same path for an open ([FR-011](findings/L02-lifecycle.md#fr-011), [FR-214](findings/X06-authority-access.md#fr-214)).

### 6. Keep the fence of ruling 75 A1, or drop the record "did not occur"?

**Today.** A write times out, so its outcome is unknown. Since order 077, an actor may record that it did not occur only once every attempt is fenced: no copy of the call is still on its way, or the implementation confirms it can no longer act. The same key is then sent again (OPS-014, OPS-024, `spec/conventions/operations.yaml:313-318`, `:515-528`). **Four unsafe faults sit in what counts as a fence**, and each can let the key's effect land twice: [FR-217](findings/X07-retries-fence.md#fr-217), [FR-218](findings/X07-retries-fence.md#fr-218), [FR-022](findings/L03-operations.md#fr-022) with [FR-180](findings/X03-model-checking.md#fr-180), and [FR-219](findings/X07-retries-fence.md#fr-219) with [FR-231](findings/X08-one-run.md#fr-231). A verifier adds a fifth route, with no number of its own. In the second pass, [FR-548](findings/GL2-vectors.md#fr-548) finds a test vector that expects the first fault. Elsewhere the fence fails closed: a hook on a `remote` item is never fenced, so later setups stay unknown until the hook is recorded as having occurred or the instance is replaced. The table sets out each. The coordinator narrows the unsafe evidence now.

**What is new.** You ruled 75 A1 on 2026-11-11 and its two limits on 2026-11-12. The faults, and the hooks' lack of a way on, came to light after both.

**Options.**
- **A. Keep the fence, narrowed.** What still escapes becomes a third limit of 75 A1: a child process that leaves its process group on macOS, a server that finishes after a restart a call it had taken, and hooks. Cost: S6-END and S4-LATE, the two planned engine orders that build the fence for a run's end and for late answers, stay (about 950 to 1,450 lines).
- **B. No fence.** "Did not occur" is taken on the actor's word. A request repeated under that key is answered failed, as Stripe replays a stored result; a hook so recorded is sent again at the next setup. Cost: a recorded failed can become false (against OPS-003), and a hook can run twice (against OPS-018).
- **C. No fence and no "did not occur".** An unknown stays unknown until recorded as having occurred. A hook that did not run has its work done by hand and recorded, or its instance is replaced, as Terraform replaces a resource whose provisioner failed. Cost: what never happened stays listed as unknown; and on a plain server a retry is a new request, so the Agent must itself make sure the first call can no longer land, as with a remote server today.

B and C each take one specification order, and under either S6-END goes and S4-LATE shrinks.

**Recommended:** C. It needs the least text and machinery and keeps OPS-003 and OPS-018 whole. With question 5's A, a reconciliation only records. B keeps the feature but drops the protection you ruled for.

**Also waiting for you, under every option.** Until you answer, the coordinator applies the recommended reading as an interim fix.
- A later attempt is answered failed while an earlier attempt of the invocation is unanswered (F13). Either the invocation stays unknown and no attempt number is sent (recommended), or an attempt number is sent under IFC-006 for a deduplicating implementation to check. [FR-220](findings/X07-retries-fence.md#fr-220)'s verifier holds this yours: it changes a reported outcome and chooses between two designs.
- Under A only, an actor records "did not occur" after an answer that said succeeded (F12). Either the record reports conflict (recommended), or it is accepted. [FR-023](findings/L03-operations.md#fr-023)'s verifier holds this yours, since it adds a refusal.

**Changes:** OPS-014 drops "did not occur" and the fence, and says how to move on after a hook. OPS-018, OPS-024, OPS-030, OPS-031, LIF-028, REV-010 and reconcile.v1 lose their fence text. S1-reconcile, the eight vectors that record "did not occur" and S5's `occurred: false` branches change; the engine's fence marks go. Amends ruling 75 A1 and the two-way reconciliation of rulings 20 OD2, 32 U4 and 41 item 15 (`IN_FORCE.md:136`, `:150`), and ruling 75 D1 for S5's accepted `occurred: false` branches. The coordinator's narrowing is withdrawn.

**Waits on it:** the MC2 rerun (A40), which checks the rule you choose; S6-END and S4-LATE; re-pinning the dogfood engine, held before order 077 until you answer; the statements [FR-195](findings/X04-stated-limits.md#fr-195), [FR-225](findings/X07-retries-fence.md#fr-225) and [FR-226](findings/X07-retries-fence.md#fr-226) ask for.

**Findings:** [FR-217](findings/X07-retries-fence.md#fr-217), [FR-218](findings/X07-retries-fence.md#fr-218), [FR-022](findings/L03-operations.md#fr-022), [FR-180](findings/X03-model-checking.md#fr-180), [FR-219](findings/X07-retries-fence.md#fr-219), [FR-231](findings/X08-one-run.md#fr-231), [FR-332](findings/P06-dogfooding.md#fr-332), [FR-346](findings/P07-engine-build.md#fr-346), [FR-024](findings/L03-operations.md#fr-024), [FR-025](findings/L03-operations.md#fr-025), [FR-195](findings/X04-stated-limits.md#fr-195), [FR-225](findings/X07-retries-fence.md#fr-225), [FR-226](findings/X07-retries-fence.md#fr-226), [FR-548](findings/GL2-vectors.md#fr-548), [FR-540](findings/GL1-protocol.md#fr-540).

#### Supporting table: what counts as a fence

| Event that fences today | Unsafe | First pass | Second pass |
| --- | --- | --- | --- |
| A late answer, even one saying the write succeeded | yes | [FR-217](findings/X07-retries-fence.md#fr-217) | [FR-548](findings/GL2-vectors.md#fr-548): the vector requests-restart-interrupted expects "did not occur" to be accepted after a late success |
| An error that a proxy or gate wrote in the server's place | yes | [FR-218](findings/X07-retries-fence.md#fr-218) | |
| A `stopped` report, while a call held before the run can still reach a later run | yes | [FR-022](findings/L03-operations.md#fr-022) with [FR-180](findings/X03-model-checking.md#fr-180) | |
| A process's end, or its answer, while processes it started still act; Codex's escaped the process-group kill in the smoke test (`docs/implementation/HANDOVER.md:149-150`) | yes | [FR-219](findings/X07-retries-fence.md#fr-219) with [FR-231](findings/X08-one-run.md#fr-231); [FR-332](findings/P06-dogfooding.md#fr-332) and [FR-346](findings/P07-engine-build.md#fr-346) in part | |
| A run's end, where the server finishes after its restart a call it had taken before | yes, says the verifier | a point added by the verifier of [FR-217](findings/X07-retries-fence.md#fr-217), with no number of its own | |
| A cancelled call: the protocol has the server send nothing more, so only a failed result already on its way, or the run's end, fences it | no, fails closed | [FR-024](findings/L03-operations.md#fr-024), [FR-025](findings/L03-operations.md#fr-025) | [FR-540](findings/GL1-protocol.md#fr-540): already known; its verifier folds it into [FR-024](findings/L03-operations.md#fr-024) and [FR-025](findings/L03-operations.md#fr-025) |
| A hook on a `remote` item: never fenced, and it has no new request | no, fails closed | [FR-195](findings/X04-stated-limits.md#fr-195), [FR-225](findings/X07-retries-fence.md#fr-225), [FR-226](findings/X07-retries-fence.md#fr-226) | |
| The fence's events stand only in an `observable` | counted once, as a fault of the form ([FR-140](findings/L13-framing.md#fr-140)) | [FR-224](findings/X07-retries-fence.md#fr-224) | [FR-549](findings/GL2-vectors.md#fr-549) |
| No model check has run on order 077's text | not a fault in the fence | [FR-183](findings/X03-model-checking.md#fr-183) | [FR-446](findings/G1a-model-checking-register.md#fr-446) plans the rerun |

The verifiers of [FR-491](findings/G4-who-verifies.md#fr-491) and [FR-496](findings/G4-who-verifies.md#fr-496) count six first-pass findings marked unsafe among those on retries and the fence. That counts findings, not faults in the fence. Three of them are [FR-217](findings/X07-retries-fence.md#fr-217), [FR-218](findings/X07-retries-fence.md#fr-218) and [FR-219](findings/X07-retries-fence.md#fr-219), above. [FR-220](findings/X07-retries-fence.md#fr-220) and [FR-221](findings/X07-retries-fence.md#fr-221) are unsafe faults outside the fence's evidence, which the coordinator fixes whatever you answer. [FR-223](findings/X07-retries-fence.md#fr-223)'s verifier dropped its unsafe mark.

### 7. What a call that changes nothing reports when no answer comes

**Today.** An Agent lists files through a plain server, and the call times out. Listing changes nothing, yet the call ends unknown (IFC-010, `spec/interface/implementation.yaml:228-230`). An error from the same call would end failed (`:225-227`). The call waits in the Resource's `unreconciled` list until an actor with reconcile.v1 on the Pool records that the read occurred. A repeat with the same key is never sent (OPS-012, `spec/conventions/operations.yaml:281-283`).

**What is new.** Ruling 41 item 27, ruling 49 (an unanswered input request ends unknown) and ruling 75's later answer of 2026-11-12 (a remote server that never answers stays unknown) were given for calls in general. None of them looked at a call that states no effect. For such a call, unknown protects nothing and only waits on a reconciliation that has nothing to check.

**Options.**
- **A. Failed.** This covers an operation that states no effect, is not a control and starts no model work. For such an operation, no answer, a cancellation or an unanswered input request ends failed, and the result says that no answer came. Nothing is listed, and a repeat is sent (OPS-024), as an HTTP client may resend a safe method (RFC 9110, section 9.2). Cost: one order, one vector, one branch in the engine.
- **B. Unknown, with two exceptions.** Such calls stay off `unreconciled`, and a repeat is sent. Cost: about the same as A, with more text.
- **C. As now,** with a stated limit that such reads wait for a reconciliation. Cost: one sentence.

**Recommended:** A. Failed is true where there is no effect (OPS-003). IFC-010, IFC-046 and OPS-024 already treat this class of call as safe to send again, and a reconciliation with nothing to check goes away.

**Changes:** OPS-007, IFC-010, IFC-016 and OPS-027 change for this class, and OPS-034's first case is dispatched as OPS-024 dispatches. Amends ruling 41 item 27 and ruling 49 (`IN_FORCE.md:196`) and, for this class, ruling 75's later answer. `IN_FORCE.md:22` ("An unknown outcome stays unknown") stands, since such a call is no longer unknown.

**Waits on it:** the order for OPS-007 and IFC-010 (SO-E in orders.md). The coordinator's IFC-046 fix (F46), which uses the same class of call, does not wait: it lands in SO-B before batch 1 builds on outcomes.

**Findings:** [FR-021](findings/L03-operations.md#fr-021), [FR-043](findings/L04-interface.md#fr-043).

### 8. What a setup, retirement or cascade reports after a restart in its middle

**Today.** A retirement is dispatched, and the engine restarts before the hook or the stop is sent. OPS-022 takes the outcome from the parts: none was dispatched, so the outcome is failed and the shutdown is pending (`spec/conventions/operations.yaml:465-474`; LIF-024). IMP-007 says that anything in progress across a restart is unknown, so the shutdown is unknown (`spec/conventions/implicit.yaml:75`). The first build follows IMP-007 (`engine/CONFORMANCE.md:268`). The same split holds for a setup and for the cascades that retire a Pool or Fleet with its members (LIF-021). So two engines can each keep every promise and still report differently. A cancellation of the same verbs already takes the outcome from the parts (OPS-027, `spec/conventions/operations.yaml:635-636`). Neither reading is unsafe, since pending and unknown both block removal.

**Options.**
- **A. From the parts as stored.** The verb has ended with the outcome its parts give. A part with a call outside the engine in flight ended unknown, and a part not begun was not dispatched. The engine stores that it is about to send a call before it sends it. Cost: the first build's recovery and its vectors change; they report failed and pending where they now report unknown.
- **B. Unknown, as the first build does.** OPS-022 and LIF-021 add "or unknown where the engine restarted while it was in progress". The coordinator writes this under ruling 37 if you do not take A. Cost: nothing to build, but each such restart leaves an entry to reconcile by hand.

**Recommended:** A. It reports what the engine knows, follows OPS-027, and differs from B only where no part with an effect was in flight. Under question 6's C, B's entries would stay listed as unknown, since no one could record that they did not occur.

**Changes:** OPS-022 and LIF-021 (the restart sentence); IMP-007 (an exception for these verbs); OPS-008 (store before sending); the first build's recovery and vectors. No ruling is amended. Ruling 30 forbids only sending anything again after a restart, which A keeps, and the multi-machine record behind ruling 43 repeats IMP-007 in a walk-through without deciding it.

**Waits on it:** the coordinator's fix on the engine's own verbs, which writes B unless you take A.

**Findings:** [FR-558](findings/GL3-restart-mid-act.md#fr-558).

### 9. What the engine's own panel is, and who may see it (unsafe)

**Today.** To admit a machine, the person types the fingerprint read on it. The engine shows its own copy, the join command and any device-login code "only in the engine's own panel" (PND-004 and PND-008, `spec/promises/pending.yaml:76-79`, `:343-344`). Nothing says what that panel is or how it keeps a model out. `spec/definitions.md:267` calls a control panel an Interface, and the first build has no panel. Suppose an engine prints the panel as a command's output, and root's model-driven console reads that output. The engine keeps every promise, and the model types the fingerprint itself (ENR-006, `spec/promises/enrollment.yaml:143`).

**What is new.** Ruling 56 point 4 (2026-11-10) put evidence "at the root Interface's control panel; never relayed through a model's output". Since then, order 038a has kept codes and fingerprints out of every message, tool result and record of a request, so they appear only in the engine's own panel and the recorded answer. Ruling 68 point 3 R2 (2026-11-11) made admission rest on a fingerprint the person types. No promise defines that separate panel.

**Options.**
- **A. A screen the person opens.** The panel is the engine's own rendering to a terminal or window. It is never returned to a request or written to a pipe, file or message, in the way ssh, sudo and GnuPG's pinentry ask for a password on the terminal. In v0 only root's person sees it, since root receives sign-ins in practice (ruling 75 C2), so a scoped Interface's device code shows there too. Cost: a definition, one interface promise, one implementation-defined item and one vector, built by order 051c. It narrows PND-004's "a person permitted to inspect the answer" and the device leg of PND-010's scoped sign-in. A model on the same user with a terminal tool still reads the panel; that becomes a stated limit.
- **B. Each engine's choice,** stated in its conformance document. Cost: one implementation-defined item. The printed-output path stays open.
- **C. No separate surface.** Root's tool results carry the values, and the term leaves seven promises. This contradicts ruling 56 point 4, reverses PND-004 and PND-008, and lets the model type the fingerprint.

**Recommended:** A. It keeps ruling 56 point 4 and the typed fingerprint, it closes the easy path, and it is what the batch-1 plan already says for order 051c (`docs/proposals/spec-batch-1/README.md:643`).

**Changes:** `spec/definitions.md` (a new row and the "control panel" row); one interface promise cited by PND-004, PND-008, ENR-003, ENR-004, ENR-005, IFC-049 and PRV-014; PND-004's audience sentence and the device leg of PND-010; one implementation-defined item; the limits "Pending actions, sign-in and the network"; `IN_FORCE.md` topic 9; the answer recorded with ruling 56; ruling 66 point 3, narrowed for the device leg: a scoped Interface's person no longer sees the device codes of its Fleets' Agents, which show only at root's panel (ruling 75 C2 already has root receive sign-ins in practice); one safety vector.

**Waits on it:** order 048a-2, on the path to the pin (question 1, part 2); order 051c; S3's enrollment of the second machine; A5's device-code leg; and the coordinator's fix that brings a device login's user code to the engine ([FR-066](findings/L06-secrets-pending.md#fr-066)).

**Findings:** [FR-141](findings/L13-framing.md#fr-141), [FR-193](findings/X04-stated-limits.md#fr-193), [FR-064](findings/L06-secrets-pending.md#fr-064).

### 10. What an actor other than root may do with install.v1 (unsafe)

**Today.** No ruling settles who may install. Root may open, for another actor, a Conduit to `management` that lists only install.v1. That actor names any file or URL. The engine fetches it as its own user, and the refusal for a wrong digest names the SHA-256 digest of what it found (PKG-026, `spec/promises/packages.yaml:329-331`). So an actor that names a secret's file in the shipped file store learns that secret's digest. The actor may also install its own archive as a new version at a trusted package's address. Ruling 75 C6 has root confirm a client ID that two installed Types claim, but the specification lets "Types at one address share freely" (CTR-013, `spec/promises/contracts.yaml:316-317`). So the actor's logins share the trusted package's client IDs with no confirmation, and the sign-in message names the trusted package (`spec/interface/implementation.yaml:1367-1368`).

**Options.**
- **A. Root alone installs in v0.** install.v1 is reserved to root, beside the five operations ACC-032 already reserves. Both paths close. Cost: one small order, and one more named v0 exception to delegation (preservation test 5). No broker, scoped Interface or peer's admitted Interface (ruling 68 R3) installs.
- **B. Others install under guards,** after Nix's allowed-uris. The conformance document lists the places a fetch may read, only root is told the fetched digest, and versions at one address are compared for client IDs. Cost: PKG-023, PKG-026, CTR-013 and DL7 change, with three vectors.
- **C. As now,** with both paths stated under "What access does not narrow". Cost: one sentence; both paths stay open.

**Recommended:** A. One mark on the contract closes an unsafe path that no scenario uses: only root's default Conduit to `management` lists install.v1 (`spec/core/templates.yaml:121`). Installing system packages is root's job under dpkg and rpm. Opening install later only adds, while narrowing it after v0 would break callers, so B can come in v1.

**Changes:** install.v1's contract (reserved to root); ACC-032; ACC-033 row 9; CTR-013, whose clause on a root-only input goes; `spec/README.md` ("What access does not narrow" and the named exceptions to test 5); `IN_FORCE.md` topic 1; one vector, in which another actor's install reports refused; the engine's reserved list.

**Waits on it:** nothing. Whatever you answer, order 038c's reading of "another address" goes on the review list. The coordinator also bounds a fetch's time and has the engine's own file fetch read only a regular file ([FR-106](findings/L09-packages.md#fr-106), [FR-258](findings/X10-queued-and-kept.md#fr-258)).

**Findings:** [FR-100](findings/L09-packages.md#fr-100), [FR-101](findings/L09-packages.md#fr-101).

### 11. Whether closing a secret's Conduit stops the engine's own use of the value

**Today.** Root closes the Conduit through which the engine read a token for a remote server, to cut that instance off. Inspection shows the Conduit closed. Yet the engine still sends the token on every request until it restarts or the instance is retired. A further setup whose read is refused keeps the old connection (SEC-013, `spec/promises/secrets.yaml:338-342`, `:355-358`). Only after a restart does the engine read again, get refused and report unavailable (`:584-587`).

**What is new.** Ruling 41 (2026-11-06) accepted "Closing the Conduit stops later reads and cannot recall a copy; to revoke a credential, rotate it in the store" (`docs/proposals/secrets/README.md:127-129`). That reason holds for a value handed to a run. No record considered a value that the engine itself keeps and goes on sending.

**Options.**
- **A. The close ends the engine's holding.** Closing that Conduit, or retiring the secret Resource, ends it. The next request reads again, is refused and reports unavailable. A run keeps its value, as ruled. Cost: one ending event in SEC-013, one vector and a small engine change. The instance stays ready while each request reports unavailable.
- **B. As now,** with the limit stated under "What the guarantee for secrets covers": to revoke, rotate in the store, which already stops the engine's use at the service. Cost: one sentence.

**Recommended:** A. A close should take effect when it is made, not at an unrelated restart, so that what inspection shows is what happens. A reuses SEC-023's existing read and follows OAuth token revocation (RFC 7009) and Vault's lease revocation.

**Changes:** SEC-013 and SEC-023; the limits on secrets in `spec/README.md`; `IN_FORCE.md` topic 12; one vector; the engine drops the header values it holds when the reading Conduit closes. Amends item 6 of ruling 41's secrets record, for the case where the engine holds the value.

**Waits on it:** nothing. S2's rotation step (E13) would exercise it.

**Findings:** [FR-065](findings/L06-secrets-pending.md#fr-065).

### Not asked now about these subjects

- Whether an error raised on the engine's own side fences a call ([FR-543](findings/GL1-protocol.md#fr-543)): its verifier refuted it, since OPS-014's list of fencing events is closed.
- The other restart findings ([FR-556](findings/GL3-restart-mid-act.md#fr-556), [FR-557](findings/GL3-restart-mid-act.md#fr-557), [FR-559](findings/GL3-restart-mid-act.md#fr-559) to [FR-563](findings/GL3-restart-mid-act.md#fr-563)): the coordinator's. One defect fix stores the engine's own verbs whole, so they never end unknown, and a separate sentence covers deleting a removed instance's directory ([FR-561](findings/GL3-restart-mid-act.md#fr-561)). Logged for you to strike.
- Striking the twelve `deduplicates` marks of the core contracts ([FR-556](findings/GL3-restart-mid-act.md#fr-556), [FR-559](findings/GL3-restart-mid-act.md#fr-559)): it removes a feature, so it is yours. It is not asked now, because its verifier found that a cancellation or a stopped part still leaves an engine verb unknown. It comes to you once the fix above has made the marks unreachable.
- Not passing on a dropped connection's cancellation, option (a) of [FR-024](findings/L03-operations.md#fr-024): moot under question 6's B or C, and a stated limit under A.
- Whether recording "did not occur" after an answer that said succeeded reports conflict ([FR-217](findings/X07-retries-fence.md#fr-217), [FR-023](findings/L03-operations.md#fr-023)): asked with question 6, since [FR-023](findings/L03-operations.md#fr-023)'s verifier holds it yours. It is applied meanwhile as an interim fix, on the review list. Moot under question 6's B or C.
- A later attempt answered failed while an earlier one is unanswered ([FR-181](findings/X03-model-checking.md#fr-181), [FR-220](findings/X07-retries-fence.md#fr-220)): asked with question 6, since [FR-220](findings/X07-retries-fence.md#fr-220)'s verifier holds it yours. Until you answer, the coordinator keeps the invocation unknown as an interim fix (F13) and sends no attempt number.
- A `started` output on run.v1 ([FR-185](findings/X03-model-checking.md#fr-185)): order 081 forbade new outputs, so the coordinator states the limit instead.
- Asking servers not to finish, after a restart, a call taken before it (a point added by [FR-217](findings/X07-retries-fence.md#fr-217)'s verifier), or cancelling an onward call that outlives its invocation (with [FR-223](findings/X07-retries-fence.md#fr-223)): both would be new behavior, and neither is recommended. The first is a limit inside question 6's A; the coordinator states the second.
- The sign-in fixes ([FR-062](findings/L06-secrets-pending.md#fr-062), [FR-063](findings/L06-secrets-pending.md#fr-063) with [FR-192](findings/X04-stated-limits.md#fr-192), [FR-066](findings/L06-secrets-pending.md#fr-066), [FR-067](findings/L06-secrets-pending.md#fr-067), [FR-068](findings/L06-secrets-pending.md#fr-068), [FR-433](findings/E04-signin-isolation.md#fr-433)): the coordinator's, logged for you to strike. The hold on a Resource whose sign-out failed also goes on the review list.
- SSH-R's contract keeps a retired Agent's remote home, which `IN_FORCE.md:191` says is removed with the Agent ([FR-431](findings/E04-signin-isolation.md#fr-431)): the coordinator brings the contract into line with the ruling, or logs the departure.
- A warning when an Agent with its own tools shares a machine with a Resource that keeps a login ([FR-197](findings/X04-stated-limits.md#fr-197)): the coordinator's under rulings 52 and 71 point 4. It rewords `IN_FORCE.md:194`, is logged for you to strike and goes on the review list.
- Moving sign-in out of the core ([FR-118](findings/L10-core-operations.md#fr-118)): ruled by ruling 49 points 1 and 3, ruling 73 points 1 to 3 and ruling 75 C6. Raised again only if A81, A82 or A5 fails.
- Moving sign-in's checks into a package ([FR-161](findings/X01-catalogue-layers.md#fr-161)), or the fingerprint check into the machine provider ([FR-162](findings/X01-catalogue-layers.md#fr-162)): ruled by rulings 49, 73 and 75 C1 to C7, and by ruling 56 points 2 to 4, 9 and 10, as ruling 68 point 3 R2 amends point 9. Only the cost is new.
- A restart verb in place of rotating a secret by replacing the instance ([FR-072](findings/L06-secrets-pending.md#fr-072)): waits on D09's counts in S2 and S10 (A67).
- A form for unpublished packages ([FR-112](findings/L09-packages.md#fr-112)): ruled by ruling 41 item 11. Raised only if the S1 rerun shows that the two regimes cost authors lines or edits.

### 12. A kept store under a changed built-in core package

**Today.** The core package is built into the engine. It changed in six commits on 2026-11-11 while its version stayed 0.1.0, as batch 1's reading 18 allows (edits in place, which you let stand in ruling 68 point 5). Nothing in spec/ says what a kept store follows when a newer engine starts over it, as the dogfood engine will at each re-pin. The first build keeps each stored Conduit's list of operations. Root's first-start and default Conduits, which no request may close or replace (ACC-024, LIF-033), keep their old lists. So once order 049a adds the enrollment verbs, root on an older store cannot use them. Its commands call `management.<verb>` (`engine/crates/shell/src/author.rs:151`), and that list does not grow.

**Options.**
- **A.** As now, with the limit stated: a core change that root needs means a new store, and every Agent signs in again. Text only.
- **B.** Write down what the first build does: stored objects read the built-in core, and no stored Conduit changes. Root reaches a new verb only through a further Conduit of its own (ACC-054), which no command calls. One sentence in NAM-020, one vector.
- **C.** B, and at each start the engine brings root's own Conduits made from core templates up to the built-in lists. The step is recorded and dispatches nothing, as the Kubernetes API server updates its built-in roles at start. Only root gains, and only what it may already open for itself. One promise, two vectors, one step at start.
- **D.** Refuse a store made under another core, as a database checks its schema version. Every core edit then needs a new version, which ends reading 18's edits in place (ruling 68 point 5).

**Recommended:** C. It is the existing paradigm. The dogfood store keeps working with no new store and no new sign-ins (ruling 70), and only root's own access widens.

**Changes:** NAM-020; one convention promise; an exception to FLEET-005's "never widen" (batch 1's text, not a ruling); OBS-004's observable names the step, which is no background reconciler (`IN_FORCE.md:22`); `IN_FORCE.md` topic 5. No ruling amended.

**Waits on it:** enrollment on the dogfood store once 049a reaches `main`, and the core's first release. Not the S1 rerun: no root list has changed since the dogfood pin, and the store is made again after order 050 (`docs/implementation/HANDOVER.md:157-158`).

**Findings:** [FR-073](findings/L06-secrets-pending.md#fr-073), [FR-081](findings/L07-objects-naming.md#fr-081), [FR-119](findings/L10-core-operations.md#fr-119), [FR-130](findings/L11-core-rest.md#fr-130).

### 13. Upgrading or swapping the machine provider

**Today.** The second machine joins through an SSH machine provider, which is a Resource (ruling 77 point 1). Each node is bound to that Resource (PLC-026, batch 1's reading 19). To upgrade the provider or rotate its key, you retire it and register another (finding 4-01: what exists is changed only by replacing it). Every node but `local` is then omitted, and nothing runs there. PRV-013 says this lasts "until its machine is enrolled again", but that never comes by itself. Root must first declare the node lost, or unregister every instance on it and retire the node. Then the machine is enrolled again, its instances and Conduits are registered again, and its Agents sign in again.

**What is new.** You saw reading 19 (a node bound to the provider's instance), with option B as its stated alternative, on 2026-11-11, and it stood (ruling 68 point 5). New: the "until" never comes; under 4-01 every upgrade strands the nodes, not only a swap; ruling 77 point 1 (2026-11-12) made the provider an SSH package.

**Options.**
- **A.** As now, with the limit stated in PRV-013 and in PRV-018, which today says a machine provider swaps without changing who may reach what. Text only.
- **B.** Bind a node to the provider's Type, as a Kubernetes Node names its cloud provider. A same-Type replacement that reports the same hostname and machine id finds the node again. The id comes from the machine's key (ruling 51 point 8). A provider of another Type still strands nodes, stated as a limit.
- **C.** Retiring a machine provider reports conflict while nodes are bound to it, as PRV-020 protects a runtime provider. Nothing is stranded, but every upgrade needs the whole teardown first.
- **D.** A new provider that admits a machine, after a typed fingerprint, rebinds the omitted node of that hostname. Four promises change.

**Recommended:** B. It removes the cost from the case that comes first and often, each upgrade of the SSH provider, and adds no request or state. A swap to another kind of provider is decided when one is taken up.

**Changes:** PLC-026, PRV-013's observable, ENR-006 check 3 (with the coordinator's one-live-node fix), `node.v1`'s description, `IN_FORCE.md:223`; one vector. Amends ruling 68 point 5 for reading 19.

**Waits on it:** order 049a1, which binds to the Resource, then SSH-M and order 049b. None is merged, so the change is cheap now. Not the S1 rerun.

**Findings:** [FR-053](findings/L05-providers.md#fr-053).

### 14. Telling the user that a consumer runs inside the engine's domain of trust

**Today.** An Agent that names no runtime runs on the engine's own runner, as the engine's operating-system user. So it can read the engine's keys and get past every gate (`spec/README.md:500-505`). Yet inspection reports every restriction on its Conduits as "mediated" (ACC-027). It reports the same for an Agent that will run on the second machine under a separate run user (ruling 77 point 2). "Mediated" is true by definition, since the Agent's requests pass through the engine. But it says nothing of what the Agent can do around the engine. To see the risk, a user must combine three facts: `directoryNamed`, an absent `runtime` in `declared`, and DL16 (where the engine states which operating-system user its own runner uses).

**Options.** Under every option the coordinator rewords PRV-029's "isolation is mediated" (`spec/promises/providers.yaml:977-979`), since nothing mediates that isolation.
- **A.** As now: the limit stays stated in `spec/README.md`. No cost.
- **B.** A, plus the status of each Agent and Resource that has a run says whether that run is inside the engine's domain of trust. Inspecting its Conduits shows the same, as Kubernetes shows `privileged` on a Pod. One status member and one vector. It reverses order 075's logged choice to add no runtime member before a confining provider exists (`docs/work-orders/075-spec-own-tools-reach.md:289-292`).
- **C.** B, plus restrictions on such a consumer are reported as neither enforced nor mediated. It changes ACC-027 and what preservation test 9 reports (ruling 17).

**Recommended:** B. Ruling 52 point 2 asks every result to say what holds. Agents on the first machine run this way and those on the second will not, so the member tells them apart. C would change a report you ruled.

**Changes:** LIF-013, the status in `spec/core/results.yaml`, ACC-027's observable; one vector. The engine sets the member from the runtime it chose.

**Waits on it:** the `.v2` Agent Types (orders 050a, 076 and 050).

**Findings:** [FR-194](findings/X04-stated-limits.md#fr-194).

### 15. The shared conventions: say what the layer holds, or move rules out

**Today.** The specification calls the conventions "the few behaviors every part must agree on, stated as invariants" (`spec/README.md:62`). The tracker calls the language "small" (`docs/AUDIT.md:381`). The convention layer holds 245 promises and 12,014 words of obligation, against 179 and 5,854 on 2026-11-06. Only 132 sit in the seven conventions' own files (127 then). Your rulings put 63 of the other 113 there, for providers, secrets, pending actions, sign-in and enrollment. The other 50 or so are core rules, nearly all there since 2026-11-06 (table below).

**What is new.** Round 6 raised this at 207 promises, and ruling 44 took only round 6's critique D21 (reading paths for each kind of reader). 38 more have come since, and the claim's own words were never put to you.

**Options.** Under every option, the coordinator does two things that need no ruling. It writes down that `observable` binds, and it adds a sentence at `spec/README.md:62` saying where convention promises sit. No option stops growth: the brake on growth is question 3.1.
- **A.** Correct the words. The seven conventions stay few and stated as invariants, beside the provider contracts and the core's own procedures. LD1, the test that assigns a rule to a layer, is restated for the three layers. Part of one spec order; no promise changes.
- **B.** Move about 30 provider and setup promises into capability contracts, as Kubernetes keeps its container runtime interface apart. First a ruling that contracts bind, then about four orders before v0. About 215 remain, and no obligation goes.
- **C.** B, and let a document's shape live in its schema alone ([FR-139](findings/L12-schemas.md#fr-139)). One more order, and a caller no longer learns which rule it broke.
- **D.** As now: note the mismatch. LD1 still has no outcome for the interface layer, and the next reader finds the mismatch again.

**Recommended:** A. Your rulings placed nearly all the growth. B and C cost a ruling and four or five orders and remove no obligation.

**Changes:** `docs/AUDIT.md:381-382` and `:406` (the words of rulings 13 and 14), `spec/README.md:62`, `IN_FORCE.md:37`, and the ledger's lines for rulings 13 and 14.

**Waits on it:** LD4's brief for the independent builds. Under B or C, also batch 2's text, order 049a1 and A18.

**Findings:** [FR-145](findings/L13-framing.md#fr-145), [FR-160](findings/X01-catalogue-layers.md#fr-160), [FR-142](findings/L13-framing.md#fr-142), [FR-139](findings/L12-schemas.md#fr-139). The coordinator's steps: [FR-140](findings/L13-framing.md#fr-140), [FR-153](findings/X01-catalogue-layers.md#fr-153), [FR-441](findings/E05-falsifier.md#fr-441), [FR-440](findings/E05-falsifier.md#fr-440). Question 3.1: [FR-472](findings/G2-size-brake.md#fr-472).

**Supporting table.** Convention-layer promises, recounted from each commit.

| | 2026-11-06 (`e271d1c7`) | Round 6's count (`c4d7d332`) | Now (`2eb8f258`) |
|---|---|---|---|
| Convention promises | 179 | 207 | 245 |
| Words of their `then` | 5,854 | 8,669 | 12,014 |
| In the seven conventions' files | 127 | 122 | 132 |
| In topic files | 52 | 85 | 113 |

The 113 in topic files now: providers 26 and secrets 15 (ruling 40); pending 12 and enrollment 10 (batch 1); and 50 others: placement 10, Conduits 9, Pools 6, templates 6, packages 6, naming 4, first start 3, objects 2, Fleets 2, instantiation 1, submissions 1.

### 16. Cutting surface that nothing uses

**Today.** Three ruled pieces say again what the language already says, and nothing shipped depends on them. A `handle` (an opaque ID an implementation issues and the engine passes back unchanged) is, to the engine, a plain string that may not state a default. `manages: {fleets: {s1: manage}}` makes the same Conduit as a `consumes` item naming the core's `fleet-manage` template. A machine provider must store the labels root names for a new machine and report them back. Yet the engine holds those labels and fixes the node's labels itself (ENR-010). Because each piece is ruled, the scope guard keeps it until you rule (`docs/AUDIT.md:316-321`).

**What is new.** No contract has used handles since ruling 41 P1 kept them, and IFC-011 contradicts IFC-010. The checks on `manages:` bind only that spelling, so they protect nothing. (Context, not new: in the same ruling, point 6, you said a separate grouping shorthand might not be needed. `manages:` itself is point 1, which you accepted.) [FR-050](findings/L05-providers.md#fr-050) found that `place.v1` asks the provider for facts only the engine holds.

**Options.**
- **A.** Cut all three. The labels go once the coordinator's fix for [FR-050](findings/L05-providers.md#fr-050) has the engine match them itself. Amends rulings 31 I5, 41 P1, 66 point 1 and 52 point 4. One spec change (table below).
- **B.** Cut handles and `manages:` only. Amends rulings 31 I5, 41 P1 and 66 point 1; ruling 52 point 4 stands.
- **C.** As now: cut none until S3 to S9 have run, the scope guard's default, and A18 (which puts to you each promise no scenario exercised) presents them at the roadmap's step 8. Meanwhile the coordinator fixes IFC-011 against IFC-010 and states that SCT-012's checks bind only its own spelling.

**Recommended:** A. Nothing is built on these pieces yet, so cutting them is cheapest now. `manages:` saves characters, not the lines S1 counts (ruling 63). Labels belong on the node the engine holds, as Kubernetes matches a node selector itself. A and B depart from the guard's order by your ruling. If you would rather not reopen enrollment's labels before S3, B leaves ruling 52 point 4 alone.

**Changes:** the table below.

**Waits on it:** the fix for [FR-050](findings/L05-providers.md#fr-050) (labels only); batch 2's Fleet-held Pools (`IN_FORCE.md:180`), which would then be written in `conduits:`; SSH-M and order 049a1's label check, both unmerged; and order 043b, on the path to the pin, which the batch-1 plan has build `manages:` (SCT-012, with the vector sectioned manages-agent-own-fleet).

**Findings:** [FR-045](findings/L04-interface.md#fr-045), [FR-204](findings/X05-short-form.md#fr-204), [FR-061](findings/L05-providers.md#fr-061). The labels part depends on [FR-050](findings/L05-providers.md#fr-050).

**Supporting table.** What option A removes and changes.

| Piece | Used today by | Ruled by | Removed | Changed |
|---|---|---|---|---|
| Handles | No contract in `spec/core/` or `packages/`. One schema example (`spec/schemas/examples/valid/files-package/contracts.yaml:142`, `:155`) and two fixtures | 31 D4 and I5; 41 P1 kept the name | IFC-011, `handle.v1` and its schema keyword | CTR-002, CTR-012, IFC-033. Fixtures e05-handle and e05-secret-omitted use a string. One engine citation (`engine/crates/core/src/schemas.rs:1147`) |
| `manages:` | The sectioned example (`spec/schemas/examples/valid/sectioned-project/s1.yaml:15`, `:23`). No translator. S1's count left it out | 66 point 1 | SCT-012, `manages.v1`, its reserved-key entries, SCT-012's two invalid examples | SCT-006, SCT-013, SCT-014, OBJ-019, OBJ-022. The sectioned example uses `consumes`. SCT-013's invalid example `SCT-013-sectioned-reserved-local.yaml` uses another reserved name |
| Labels echoed by the provider | Nothing built: SSH-M is not, and 049a1, which checks the echo, is unmerged | 52 point 4 | None | ENR-002, ENR-006 check 4, ENR-010, PRV-014 |

Also changed: `spec/definitions.md`, `spec/core/README.md`, and `IN_FORCE.md` lines 179-181, 221 and 239. IFC-011 and SCT-012 are registered as removed IDs.

### 17. Four of the coordinator's fixes that change something you accepted

**Today.** The coordinator fixes a defect without asking only where the fix adds no behavior and reverses no choice you accepted (rulings 37 and 46 point 4). Four fixes in [decisions.md](decisions.md) would each also reverse an accepted choice, so they come to you:
- **F15.** When the engine cannot write or read its store, every request is refused and reports unavailable. Today only reads of secrets fail closed (the secrets proposal, accepted with ruling 41), and what other requests do is left to each engine (DL10).
- **F39.** `conduit serve` stops setting up registered instances at each start; bringing a project back after a restart becomes one explicit command, recorded as root's (OBS-004). This reverses order 027's settlement of finding 4-07, which ruling 58 point 1 accepted.
- **F72.** For an instance placed on the engine's own machine, the engine asks no machine provider. This withdraws the provider's "or none" for `local`, which ruling 42 accepted.
- **F124 and F127.** Ruling 17 NS2's writing rules for promises change: rule 1 (one obligation per promise) is reworded to say what a promise may hold, a rule is added on what an `observable` may say, and the meaning of a promise's `vectors` field is reworded. No behavior changes.

**Options.**
- **(a)** Accept all four.
- **(b)** Accept all except those you name. Each one not accepted keeps only its part that changes nothing you accepted.
- **(c)** Accept none, with the same narrowing for each.

**Recommended: (a).** Each makes the engine do what the purpose already says: fail closed rather than guess (F15), act only when asked (F39, `IN_FORCE.md:22`), ask a provider only about other machines (F72), and say plainly which text binds (F124, F127).

**Changes:** the rows of those fixes in decisions.md; this review's ruling amends ruling 41's secrets record (F15), ruling 58 point 1 for finding 4-07 (F39), ruling 42 (F72) and ruling 17 NS2 (F124, F127).

**Waits on it:** the orders that carry them ([orders.md](orders.md)): F15 with SO-F, F39 after the pin, F72 with the staged patch 049b, F124 and F127 with the sweep of the observables.

**Findings:** [FR-236](findings/X09-records-loss.md#fr-236), [FR-409](findings/E03-s8-s9-s10.md#fr-409), [FR-049](findings/L05-providers.md#fr-049), [FR-050](findings/L05-providers.md#fr-050), [FR-140](findings/L13-framing.md#fr-140), [FR-157](findings/X01-catalogue-layers.md#fr-157).

### Asked later, not now

- What changing a running Agent may cost, finding 4-01 ([FR-292](findings/P03-plan-apply.md#fr-292), [FR-093](findings/L08-templates-conduits.md#fr-093), [FR-072](findings/L06-secrets-pending.md#fr-072)): after the S1 rerun, with the bar (`IN_FORCE.md:310`). Three questions go with it. May apply send a Type's control in place of a replacement ([FR-251](findings/X10-queued-and-kept.md#fr-251))? Must narrowing access cost a tombstone lift ([FR-018](findings/L02-lifecycle.md#fr-018))? May an approval name the plan instead of each object ([FR-301](findings/P03-plan-apply.md#fr-301))?
- Whether an Agent's own requests during a turn answer to the turn's caller ([FR-005](findings/L01-access.md#fr-005)): with ruling 62 C6 to C12, after the rerun. The limit is written now. Two of those points carry findings of this review: C10, who reads a record's kept content ([FR-027](findings/L03-operations.md#fr-027), [FR-240](findings/X09-records-loss.md#fr-240)), and C11, root's register tools that always refuse ([FR-216](findings/X06-authority-access.md#fr-216)).
- Whether a Resource may start model work ([FR-263](findings/X10-queued-and-kept.md#fr-263)): with batch 2's text for ruling 61 B13.
- Whether the engine refuses a store restored from an older copy ([FR-237](findings/X09-records-loss.md#fr-237), [FR-256](findings/X10-queued-and-kept.md#fr-256)): not recommended for v0. On one machine the engine would need a counter that a disk restore cannot bring back. The limit is written now. Revisit this when the store is remote or has several parts.
- Whether a purge may also drop the engine's kept invocation state ([FR-244](findings/X09-records-loss.md#fr-244)): not put for v0, since on one machine the growth is small. The limit is written now.
- Cutting ACC-033's rules for a member of a retired owner ([FR-008](findings/L01-access.md#fr-008), [FR-212](findings/X06-authority-access.md#fr-212), [FR-259](findings/X10-queued-and-kept.md#fr-259)): ruling 75 D answered it on 2026-11-11 (S4's Q4). Raise it again at A18, and only once a member's registration is ordered against its owner's retirement ([FR-250](findings/X10-queued-and-kept.md#fr-250)).
- Cutting `/operations` as an input's destination: at A18, after S3 to S9. Cutting a `remote` item that is not marked plain: only after S2 and S7.
- Whether contracts at one address keep two regimes ([FR-112](findings/L09-packages.md#fr-112)): only if the S1 rerun shows that they cost authors lines or edits.
