# P02-s1-bar: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (Focus 2 and 3, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** S1's bar did one good thing and is now doing harm. It caught a real fault, a canonical form that cost 5 and 10 times the baseline's lines, and it produced the short form, which brings S1's final state to the size of hand configuration. But ruling 71 narrowed the owner's "an author's work" to lines alone, a measure that round 6, the source of both the baseline and D22, never proposed, and on which the baseline is not a like good: A70 has no inspectable or revocable relationship, keeps no record, and its upgrade silently widened access. As ruled, the bar is passed or failed by YAML layout (identical content is 26 lines in flow style and 61 in block, against the baseline's 41), by whether the author runs `plan` before `apply`, and by one-line margins; exactly four tasks are forecast fewer, the minimum. It has steered three rulings in about sixteen hours, each made beside a paper count, and the specification's only short-form project example is S1's own state at T2, which the rerun's fresh author is handed. Neither increment-4 author struggled with lines: none of their 13 mistakes concerned how much was written, the fresh author spent 53% of its time on two tasks holding 27% of its lines, repairing unknown outcomes, and "concepts met", though pre-registered, was never reported. The bar's judgment now holds batch 2, and with it the runs of S2, S3 and S10 and Conduit's own coordinator Agent, findings 4-01 and 4-11, and a class of small CLI decisions, while the rerun waits on twelve unwritten orders. I recommend amending it (I1, option b): keep the line count as a ceiling on the coordinator's run, add pass-or-fail checks of what the relationship delivers and the baseline cannot, and judge the fresh author on finishing from its pack with errors that name their repair. The rerun should also report D22's five measures, time to a first success, an error-and-repair ledger, windows without access and a layout-free size; S1 should leave the specification's example; and three held-out tasks should be added that no paper count has seen.

**What the analyst read.** Tree: the repository root at 2eb8f258 (2026-11-12); read only, nothing built or run except read-only git, grep and two small /usr/bin/python3 counts.

Read in full: docs/guides/full-review.md; docs/implementation/HANDOVER.md; AGENTS.md "What holds"; the records of rulings 71, 63, 44, 45, 47 and 76; docs/proposals/first-build/to-review-with-the-owner.md; docs/implementation/FIRST_BUILD.md; experiments/007-s1-baseline/README.md and results-2026-11-09.md, with its seven setup files; docs/proposals/first-build/log-increment-4.md (each row to 2,500 characters); the increment-4 measurement plan, the coordinator's run report and the fresh author's run report; the fresh author's report.json (all 10 mistakes, 14 unclear points, 2 could-nots) and both T3 answers; experiments/023-paper-count/README.md; docs/AUDIT.md's Purpose, preservation tests, scope guard, layered claim, waiting-for-the-owner list and handover state; IN_FORCE.md topic 13.

Read in part: experiments/023-paper-count/results-2026-11-11.md (lines 1-230, 299-330, 365-404, 452-490; not the request routes, the comparison with K3's cases or the checks); docs/implementation/plan-and-apply/README.md (lines 1-121, 162-200, 260-284, 460-585; not ordering and access, kept state, the text grammar or the case format) and case 09 only among its cases; ROADMAP.md (step 3, the register rows for A70, A79 and experiment 023, proposing an experiment, scenarios, round 6's table); round 6's design critique (D22 and the baseline paragraph only); spec/README.md's section on the short form and the author's reading path; the spec's sectioned-project example; the ledger rows for rulings 47, 53, 63, 71 and 76; single rows or points of rulings 48, 53, 55, 61, 62 and 68; the first 60 lines of the language-shape summary and the passages of its critic and synthesis that mention the bar; the batch 1 plan's order list and critical path; the fresh author's log (first 45 lines and by grep) and the coordinator's log (by grep); S2, S3 and S10's plans by grep for their gates.

Not read: experiment 011; the full records of rulings 53, 55, 59, 61 and 62; the engine's code beyond two greps for the short form and for plan and apply; the baseline's raw evidence, which is in history at f1cb4734 and not in the tree; S4 to S9's plans beyond a grep; ruling 46's record. The recount of T5 under ruling 76 with a `plan` command, and the classing of the 13 mistakes, are mine and are marked so. Times in the runs are single observations by AI authors of different models.


## FR-277

**S1's bar: keep the line count, amend it, or replace it**

P02 I1 · question-for-owner · verifier: partly · decides: owner

Today's rule (ruling 71, point 3): in each of two runs, the author must write fewer lines than the hand-made baseline on at least four of S1's seven tasks, and on no task more than 1.5 times the baseline's lines. Nothing else decides.

(a) Keep it. The rerun reports other measures, but only lines decide.

(b) Amend it to three conditions, fixed before the rerun.
1. A cost ceiling. In the coordinator's run, no task takes more than 1.5 times the baseline's lines, counted as ruling 63 says. T3 leaves the line rule. "Fewer on most tasks" is reported, with the block, strict and layout-free readings, and no longer decides.
2. What the relationship delivers, pass or fail in both runs, with the baseline scored on the same list: T3 answered in full from the engine's inspection; T4's change reaches each running session at its next turn with no restart and no session lost; T5's upgrade gives no Agent an operation its Conduit does not list; T7 closes A's access by one act, leaves B working, and restores it; every request has a record naming its Agent and Conduit; every restriction is reported as mediated.
3. The fresh author finishes T1 to T7 from its pack alone, needs no script to drive the commands, discards no state, recreates no Agent, leaves no unknown outcome unresolved, and every error it meets names its cause and its fix (ruling 52). Its lines, time to a first success and errors are reported, not gated.
The owner still judges from the whole evidence (ruling 47).

(c) Replace it: no line rule; conditions 2 and 3 alone; the baseline reported as context.

Recommended: (b). The owner's words in ruling 47 are "an author's work", and round 6, which proposed both the baseline and D22, named no line count and said a language that adds setup work can still win. The line rule turns on YAML layout and on one-line margins (I2), has already steered three rulings (I5), and misses what both authors struggled with (I6). The ceiling keeps the tripwire that caught finding 4-11. On paper the present rule already passes in flow style, so (b) rescues nothing: it is stricter than (a) everywhere except "fewer on most".

It changes IN_FORCE.md topic 13 (line 266), ruling 71's status, FIRST_BUILD's success criteria and order 053's measurement plan.

**Evidence.** docs/proposals/ruling-71/README.md:34-41 ("the measure is the author's lines as ruling 63 counts them"; "both runs must pass"; "more than one and a half times"). docs/proposals/ruling-47/README.md:41 ("less of an author's work than the baseline on most of S1's tasks and much more on none"). docs/proposals/reaudit/round-6/design-critique.md:692-697 ("Measure duplicated credentials/configuration, per-agent access changes, inspection effort, provider-specific code and recovery interventions. A language that adds setup work can still win if changing and auditing relationships becomes materially easier"). IN_FORCE.md:266. docs/proposals/ruling-76/README.md:39 (forecast 21, 5, 0, 3, 2, 2, 2).

**Waits.** Order 053's measurement plan, which ruling 45 requires fixed before the rerun; then the judgment itself, and what hangs on it: batch 2's merge, findings 4-01 and 4-11, ruling 62's C6 to C12, and ruling 71 point 4's exclusion (I7).

**Verifier.** partly. Reopened every source. The rule is as quoted (docs/proposals/ruling-71/README.md:34-41; IN_FORCE.md:266). Round 6 names no line count and says a language that adds setup work can still win (docs/proposals/reaudit/round-6/design-critique.md:620-624, :692-697). Forecast 21, 5, 0, 3, 2, 2, 2 against 33, 16, 0, 8, 3, 2, 2 (docs/proposals/ruling-76/README.md:9-11, :39-40). The owner is the right decider: it changes a v0 pass condition. Three pieces do not hold. (1) 'The owner's words in ruling 47': the row is 'a list the coordinator drafted and the owner accepted' (docs/proposals/ruling-47/README.md:33-34, :41); the ledger's owner words for 47 are different (docs/proposals/ruling-log.md:55); `git grep "author's work"` over the ledger and owner-direction finds only that table. (2) '(b) rescues nothing: it is stricter than (a) everywhere except fewer on most' is false. Ruling 71 requires both runs to pass on lines (:37); (b).1 applies the ceiling to the coordinator's run only and (b).3 reports the fresh author's lines 'not gated'. The fresh author is the run most likely to fail: 646 lines against the coordinator's 332 (docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md:34), and the project's own synthesis warned that at the fresh author's factor of 1.7 T1 passes the baseline (docs/proposals/language-shape/short-form/synthesis.md:494-497). (3) 'Nothing else decides' is true of the bar's row only. v0 must also give a record for each request (IN_FORCE.md:263), and the success criteria already fixed before the run are: a competent reader completes the path, closing one Agent's access leaves the other working, the user resolves uncertainty without guessing (IN_FORCE.md:275). Most of condition 2 and part of condition 3 restate these.

**Verifier's counterargument.** Strongest case against (b): the pre-registered thesis is 'Declaring relationships costs an author less than configuring each agent' (docs/implementation/FIRST_BUILD.md:32). It failed in increment 4. Dropping 'fewer on most' before the rerun, with a paper forecast at the four-task minimum, rewrites that thesis after the data; the owner should be told so in those words. The line rule is also the only measure with an outside yardstick that has ever failed the design, and by the analyst's own account (I11) condition 2 was already met by the 332-line run, so it cannot fail: it is a regression check, not a test. Second, (b).1 keeps the defects I2 names: the ceiling is 3 lines on T6 and T7 and 4 on T5, and T7 in block style is 7 (experiments/023-paper-count/results-2026-11-11.md:27). A smaller amendment does the same job with one sentence and no list of twelve checks: the line ceiling at 1.5 times is judged on the coordinator's run over two groups, setup (T1 and T2, baseline 49) and change and repair (T4 to T7, baseline 15); 'fewer on most' is reported; the fresh author is judged on ruling 46 point 3's criteria already in force, which is the job ruling 48 point 4 gave it ('Who tests the success criterion ... its measures are recorded beside the coordinator's', docs/proposals/ruling-48/README.md:31). 'No script' and 'every error names its fix' should be reported, not gated: a Codex author scripts by habit, and a client's or npm's error is not Conduit's to word. 'Leaves no unknown outcome unresolved' leans on finding 4-12, closed on 2026-11-12 with reconcile.v1 unchanged (docs/proposals/first-build/log-increment-4.md:23).

**Verifier's corrections.** Write 'the criterion the coordinator drafted and the owner accepted', not 'the owner's words'. Replace '(b) rescues nothing' with '(b) removes the line gate from the fresh author's run and keeps a ceiling on the coordinator's'. The list of what it changes must add ruling 47's own row (line 41, and its status in the ledger) and thesis 1 at FIRST_BUILD.md:32.


## FR-278

**The line count is a useful ceiling and an unfair gate**

P02 I2 · assessment · verifier: partly · decides: none

As a tripwire the count worked: it caught a canonical form that cost 5 and 10 times the baseline. As a pass-or-fail rule it now measures layout and command spelling.

- Layout decides. S1's final state in the short form parses to the same 26 values and 50 keys in either style, and is 26 lines in flow style and 61 in block, against the baseline's 41. By non-whitespace characters it is 1,268 against 1,234. The paper count agrees: flow passes, block fails.
- The margins are one line. Under ruling 76's forecast exactly four tasks are fewer (T1, T2, T4, T5), the minimum for "most". T5 wins 2 to 3. T6 and T7 sit at 2 against a limit of 3. T3 is 0 against 0: it can never be fewer, and any counted line on it is "much more".
- It penalizes care. An author who runs `conduit plan --replace shared` before applying T5 adds one counted line; T5 becomes equal and the run fails "most". The paper count's row for a plan before each apply failed both parts.
- It rewards not using the product. On paper T3 is answered by reading ten files and running no command, the baseline's own method, because an inspection command that names an Agent would count.
- It compares unlike authors. The fresh Codex author is set against a baseline written by an author who had read the server's README first; no fresh author did the baseline.
- It compares unlike goods (I4).

Existing practice does not gate a configuration language on lines. Usability is measured by task completion, time and errors (ISO 9241-11), and by time to a first working result; access systems are judged by whether one query answers "what can this subject do" (`kubectl auth can-i --list --as`).

**Evidence.** Command (read-only, /usr/bin/python3 with PyYAML over experiments/023-paper-count/states/{flow,block}/t7 and experiments/007-s1-baseline/setup): flow 26 non-blank lines, 1,268 non-whitespace characters; block 61 lines, 1,207; both 26 leaves and 50 keys in s1.yaml; baseline 7 files, 41 lines, 1,234 characters (5 configuration files alone: 35 lines, 726; s1.yaml alone in flow: 21 lines, 860). experiments/023-paper-count/results-2026-11-11.md:115-116 ("Most means at least four of seven. T3's equality to zero cannot count as fewer"), :130 ("Block has fewer on only two tasks and much more on T7, forecasting failure"), :153 (plan before each apply: "only three fewer; fails both parts"), :159 ("Any later required command makes T3 much more than zero"), :93-96 (T3: ten files, no command). docs/proposals/ruling-76/README.md:39. experiments/007-s1-baseline/results-2026-11-09.md:525-527 (the author read the server's README before T1). The T5 recount under ruling 76 is mine, from ruling 63 point 1's command rule.

**Verifier.** partly. Recounted with grep, tr and /usr/bin/python3 (PyYAML). experiments/023-paper-count/states/flow/t7: 5 + 21 = 26 non-blank lines, 408 + 860 = 1,268 non-whitespace characters. states/block/t7: 11 + 50 = 61 lines, 1,207 characters. s1.yaml parses to 26 leaves and 50 keys in both styles and the two parse equal. experiments/007-s1-baseline/setup: 7 files, 41 lines, 1,234 characters; the five configuration files alone 35 lines, 726. All as stated. The paper count's rows hold: flow passes, block fails (results-2026-11-11.md:130); T3's zero cannot be fewer and any line is much more (:115-116, :159); plan before each apply fails (:153). Four fewer, three equal under ruling 76 (docs/proposals/ruling-76/README.md:39-40). The baseline author had read the server's README (experiments/007-s1-baseline/results-2026-11-09.md:527-529). Two bullets are overstated. 'It penalizes care': only a plan that carries a flag counts. A bare `conduit plan` or `conduit apply` carries no chosen value (docs/proposals/ruling-63/README.md:19-20), and the reference's stopped plan prints the flag and operand needed: 'requires=--reopen agent-a--project' (docs/implementation/plan-and-apply/cases/09-t7-restore-unflagged.yaml:244). So the approval needed is free to learn; only the full preview costs a line. Row :153 is also under the store-required reading that ruling 76 point 3 removed. 'It rewards not using the product ... because an inspection command would count': the 'because' is not in the source. Order 073 fixed 'T3 primary is zero: list files/commands as inspection effort' before counting (experiments/023-paper-count/README.md:38), and increment 4 treated `tools --as` as a check, not a line (docs/proposals/first-build/increment-4/measurement-plan.md:38-40; coordinator-run-2026-11-10.md:55-56). The risk exists only under the alternative reading at :159.

**Verifier's counterargument.** Both authors are models, for whom writing is nearly free: 'the author is an AI agent, so the time to write a file is not a person's' (007 results:455-456); 'Times are a model's' (coordinator-run-2026-11-10.md:130). Time therefore understates volume, and lines are the only proxy in this design for what a person must type and read. That is a reason to keep a size test, and a reason not to lean on the time figures in I6. The layout-free size the analyst offers is not clean either: flow s1.yaml is 860 characters against 726 for the baseline's five configuration files and 1,234 for all seven, so the verdict flips with what is included. Kubernetes' `kubectl auth can-i --list --as` is correctly named. ISO 9241-11 defines effectiveness, efficiency and satisfaction; 'errors' is the analyst's gloss and no repository source backs the sentence on existing practice.

**Verifier's corrections.** The README limit is at 007 results:527-529, not 525-527. Say 'a plan that names a flag adds a counted line; a bare plan or apply does not and prints the flag needed'. Drop 'because an inspection command that names an Agent would count' or mark it as the alternative reading of 023 results:159.


## FR-279

**What the bar measures, on which tasks, counted how, and by whom**

P02 I3 · fact · verifier: partly · decides: none

- Tasks: T1 to T7 of the first build: set up a shared directory (A reads, B reads and writes) and a shared history; say what each agent can reach; swap the two agents' access live; upgrade the server; replace the directory; revoke and restore A.
- Measure: lines only. The owner's bar in ruling 47 says "an author's work". Ruling 71 point 3, recommended by the coordinator and accepted with the rest ("I accept your recommendations"), narrowed it to lines. No record gives a reason for 1.5.
- Counting (ruling 63): non-blank authored lines added, changed or removed; a command counts only if it carries a value the author chose, so a bare `conduit apply` is free; pins that `install` writes and packages' standard templates are tool or provider work; a line removed and restored counts once; panel sign-ins and root's Conduits are listed apart; block and strict readings are reported beside.
- Who writes: two AI authors, the coordinator and a fresh Codex session given the specification, the tasks and the CLI's format reference. Both must pass.
- Who counts: the coordinator, under a plan fixed beforehand. The baseline had an independent audit, which corrected its lines from 67 to 64 and three other counts. Neither increment-4 report records an audit or recount.
- The baseline's stated premise is one author doing both setups. That author was the admin session, which handed over on 2026-11-12; the rerun's coordinator is a different session.
- Round 6, the source of both the baseline and D22, proposed no line count. Lines entered through FIRST_BUILD's measures table.

**Evidence.** docs/implementation/FIRST_BUILD.md:123-136 (tasks), :141-158 (measures; :149 lines). docs/proposals/ruling-71/README.md:16-17, :34-41. docs/proposals/ruling-63/README.md:9-27. docs/proposals/ruling-log.md:71, :79 (owner's words "All as recommended"; "I accept your recommendations..."). `git grep -n -i -E "one and a half|1\.5 ?(x|times)"` finds the ruling and its echoes only. experiments/007-s1-baseline/results-2026-11-09.md:597-609 (audit: "64 (was 67)"), experiments/007-s1-baseline/README.md:57-59 ("The same author does the build's tasks"). `grep -n -i 'audit\|recount\|verifier\|independent'` over the two increment-4 reports: no match. docs/implementation/HANDOVER.md:3-5. docs/proposals/reaudit/round-6/design-critique.md:620-624, :692-697. IN_FORCE.md:266, :276-280.

**Verifier.** partly. Holds: tasks (docs/implementation/FIRST_BUILD.md:128-136); lines as the measure and both runs (ruling-71:35-41); 'I accept your recommendations' (ruling-71:16-17; ruling-log.md:79) and 'All as recommended' (ruling-log.md:71); ruling 63's rules (docs/proposals/ruling-63/README.md:16-29); no reason for 1.5 anywhere (`git grep -n -i -E 'one and a half|1\.5 ?(x|times)'` returns the ruling, IN_FORCE.md:266, ruling 76, order 073 and experiment 023 only); the audit's 64 (was 67) and three other counts, concepts 20 (was 22), recovery 6 (was 7), mistakes 7 (007 results:605-617); no audit or recount in either increment-4 report (grep finds nothing); round 6 names no line count; lines enter at FIRST_BUILD.md:149. Not established: 'That author was the admin session.' The results say the home additions were counted 'as the instructions for this run asked' (007 results:112) and 'the author reports no edit of it' (:589), which read as a delegated agent; the README states an intent, 'The same author does the build's tasks' (experiments/007-s1-baseline/README.md:57-58). The tree does not say that the baseline's author and the coordinator who ran increment 4 were one session. Imprecise: 'root's Conduits are listed apart' beside panel sign-ins. Root's Conduit lines are counted in the primary reading and itemized (experiments/023-paper-count/README.md:37; results:34); panel sign-ins are outside it.

**Verifier's corrections.** State that the one-author premise may never have held, and does not hold for the rerun either way. 'Narrowed the owner's an author's work' should read 'narrowed the accepted criterion' (ruling-47:33-34). Add ruling 48 point 4's wording (ruling-48:31): the fresh author was given the first success criterion to test, with its measures 'recorded beside the coordinator's'; that both runs must pass on lines is ruling 71's addition.


## FR-280

**The baseline does not deliver what Conduit does; on round 6's own measures the 332-line run had already won**

P02 I4 · fact · verifier: partly · decides: none

A70 is two agents' own settings, each in its vendor's syntax. Against the preservation tests of the tracker's Purpose:

- Test 2 (every access a relationship that can be inspected and revoked). The baseline's T3 answer took 10 items read and still missed two things an audit found (Codex's account servers, Claude Code's web tools). Its revocation is a rule in the client: "the server it hides still runs with access to the directory". Its T4 change did not reach the running session.
- Test 3 (owned once). Eight values are written in more than one place.
- Test 7 (attributable). No record exists.
- Test 9 (reports enforced or mediated). Nothing reports; the T5 upgrade silently gave the read-only agent two write tools, and it wrote with them.
- Test 4 (no vendor assumed). Each agent is configured in its own vendor's terms.

The coordinator's increment-4 run, which failed the bar at 332 lines, did better on each measure round 6 named: 0 provider-specific lines against 8; 1 recovery step against 6; T3 from 2 commands; one Conduit per change of access; no restart, no session lost, no new tool on upgrade; 93 access decisions recorded for 93 requests.

S1 does not exercise tests 5, 6 or 8, and has no credential, so it cannot speak to "establish once, share under restriction". That is S2's.

So lines against A70 compares the price of two different goods. A lower price is not required for the purpose's claim; a price many times higher would still stop the first user, which is why a ceiling is worth keeping.

**Evidence.** docs/AUDIT.md:247-286 (Purpose; :274 "The Conduit is an object"), :299-306 (tests 2, 3, 7, 9). experiments/007-s1-baseline/results-2026-11-09.md:276-306 (T3 omissions; "10 items read and no command run. A complete answer needs at least one command more"), :329 ("did not reach the running session"), :412-413, :429-437 (8 duplicated values), :43-45 (upgrade), :26 (provider-specific 8), :28 (recovery 6 and 1 expected). docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md:23-27 ("on four points the build did better"), :44-47, :66, :117. docs/proposals/reaudit/round-6/design-critique.md:694-697.

**Verifier.** partly. Holds: T3 from 10 items with two omissions found by the audit (007 results:276-293, :306-307); the revocation lives in the client and the server still runs (:411-413); 8 duplicated values (:429-439); no credential in S1 (:444-445); the upgrade gave B append_file and B wrote (:360-362); the coordinator's 0 provider-specific lines, 1 recovery step, 2 commands for T3, 93 decisions for 93 requests (docs/proposals/first-build/increment-4/coordinator-run-2026-11-10.md:45, :47, :66, :117). Does not hold as worded: (a) 'did better on each measure round 6 named'. Duplicated configuration, round 6's first measure, is in neither increment-4 report (`git grep -i duplicat` over increment-4 finds the plan only). Per-agent changes were not better: 'T4 1 file shared, 3 / 3 requests' against the baseline's 1 / 1 files (:44; 007 results:24). The 0 provider-specific lines sit beside 45 contract-address lines listed apart (:46). T3's reach around the engine came from two READMEs and increment 3's closeout, not from inspection (coordinator-run/t3-answer.md:11-12, :27-28), and inspection was incomplete (finding 4-09). That is three of five measures clearly better. (b) 'Its T4 change did not reach the running session' is confounded with the --settings stand-in, by the audit's own words (007 results:335-345, :618-621), and FIRST_BUILD.md:182-183 counts that restart 'only if a run in a trusted workspace repeats it'. (c) The T5 widening was 'made deliberately' by an author who 'knew at this step ... that 2.2.0 adds two write tools' and chose the shorter deny list (007 results:62, :384-385, :527-529, :555-556); against the better baseline the coordinator's report says the safety advantage 'is smaller' (coordinator-run:72-75). (d) The baseline's T7 revocation was one rule, reached the running session at its next turn, left B working and was restored (007 results:405-417). (e) The coordinator's run had 29 to 30 s without access at T4 and about 80 s at T6 (:48), which the item omits.

**Verifier's counterargument.** The conclusion that the two are different goods stands, in a narrower form. A70 does have per-agent access that can be read and revoked; at T7 the two were equal. What it lacks is an object independent of both ends, one place to ask, a record of each request, and single ownership of shared values. Conduit's answer about reach around the engine was documentary too, as the baseline's was. The preservation tests are checks on rework decisions ('Each rework decision is checked against them', docs/AUDIT.md:291), not a scoreboard for rival setups; using them so is the brief's suggestion and should be labelled a judgment, not a fact. S1 arguably does exercise test 6: the fresh author kept two turns unknown and replayed no write.

**Verifier's corrections.** Title: 'on three of round 6's five baseline measures the 332-line run did better; one was equal and one was never reported'. Line cites: recovery is 007 results:27; the duplicated list is :429-439; 'No credential exists in S1' is :444-445. The summary's 'A70 has no inspectable or revocable relationship' should read 'no relationship object, no record and no single place to inspect'.


## FR-281

**The margin has been fitted to S1's seven tasks, and the fresh author is handed the answer**

P02 I5 · assessment · verifier: partly · decides: none

The short form itself is general and reads like Compose. What has been fitted is the margin by which it passes.

- Three rulings in about sixteen hours, each beside a paper count. Ruling 63 followed a count of 55 lines in flow and 97 in block, and made flow the documented style, excluded tool-written pins and bare commands, and moved the `read` and `read-write` templates into the package, which took 8 lines off T1. Ruling 71 set 1.5. Ruling 76 chose five CLI readings; its record gives the forecast before (T7 "exactly at one and a half times") and after ("none over the limit"). One of them makes a dropped Conduit close with no flag.
- Experiment 023's own README says it informs the CLI's design and "never the bar". Ruling 76 cites it for the bar's forecast. Ruling 45 had dropped paper counts of an author's effort.
- The shipped templates are exactly S1's two grants. A grant outside them costs a local template, which no task prices.
- The specification's only valid short-form project is S1 itself: its first line is "# S1 at T2", and the author's reading path points to it. It differs from the paper count's T2 state in four hunks. The rerun's fresh author gets the specification, so its T1 and T2, two of the four tasks forecast fewer, can be copied. In increment 4 the author's pack held "nothing of S1's design".
- The form the bar produced is unproven: 15 promises in sections.yaml with no vector, and no translator, plan or apply in the engine at this commit.
- Ruling 71 point 4 sends every choice that could move the counts to the owner. Ruling 76's five points went up for that reason: the bar makes rulings.

**Evidence.** docs/proposals/ruling-63/README.md:9-11 ("counted 55 lines in flow style and 97 in block style against the baseline's 64 ... the bar turns on how lines are counted and written"), :24-27. experiments/023-paper-count/results-2026-11-11.md:308 (T1 34 to 23: "remove 8 lines of local read/write templates"). docs/proposals/ruling-76/README.md:5-11, :20-39. Commit times: `git log --date=iso` gives ruling 63 at 2026-11-10 22:33, ruling 71 at 2026-11-11 11:48, experiment 023 at 14:23, ruling 76 at 14:26. experiments/023-paper-count/README.md:14 ("never the bar"). docs/proposals/ruling-45/README.md:76. spec/schemas/examples/valid/sectioned-project/s1.yaml:1; spec/README.md:109-110; `diff` of that file against experiments/023-paper-count/states/flow/t2/s1.yaml: 4 hunks. IN_FORCE.md:278. docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md:13-14. spec/promises/sections.yaml: 15 ids (PyYAML count); `git grep -c 'SCT-0' -- conformance/manifest.yaml`: none; `git grep -n -E 'fn (plan|apply)' -- engine`: none. docs/proposals/language-shape/short-form/critic.md:31-34 ("the bar depends on writing each grant as one long line").

**Verifier.** partly. Holds: commit times (`git log --diff-filter=A --date=iso`): ruling 63 at 2026-11-10 22:33:32, ruling 71 at 11-11 11:48:36, experiment 023 at 14:23:10, ruling 76 at 14:26:12, so 15 h 53 min. Ruling 63 followed the 55 and 97 count (ruling-63:8-12); the templates took 8 lines off T1 (023 results:308); ruling 76 gives the forecast before and after (:9-11, :39-40); 'never the bar' (experiments/023-paper-count/README.md:14-15); X2 dropped (ruling-45:76). The shipped Conduit templates are exactly `read` and `read-write` (conformance/fixtures/s1/packages/filesystem-0.1.1/conduit.json:4-11; contracts.json holds two Conduit documents). The only valid sectioned project begins '# S1 at T2' and the author's path points to it (spec/schemas/examples/valid/sectioned-project/s1.yaml:1; spec/README.md:109-110); the canonical example is an unrelated 'review-desk' project, so increment 4's pack held no S1. 15 ids in spec/promises/sections.yaml (SCT-001 to 009, 011 to 016); no 'SCT-0' under conformance/; no `fn plan` or `fn apply` under engine/. The project's own record says the pass 'is thin and depends on style' and that template reuse was 'the remedy' (docs/proposals/language-shape/README.md:50-54). Corrections needed, below.

**Verifier's counterargument.** 'Has been fitted' is not shown; 'tuned against S1 with no held-out check' is. Each choice also follows a named system, which is ruling 40's own test: closing what the file drops is Terraform's; a current engine context is kubectl's and Docker's (ruling-76:20-30); pins written by the installer are every lockfile; shipped `read` and `read-write` templates are Kubernetes' default ClusterRoles view and edit. Ruling 76 also says 'It remains a forecast; the bar is judged on the real runs' (:40), which the item omits. Experiment 023 was used as its README says, to inform the CLI's design; the fair complaint is that a forecast sat beside each design decision, not that a rule was broken.

**Verifier's corrections.** The diff against the paper count's T2 state has 5 hunks (`git diff --no-index --minimal -U0 ... | grep -c '^@@'` gives 5), 4 without the comment line. 'Three rulings' undercounts: S1's count also shaped ruling 53 (decision 1, 'Whether an authoring layer counts for the bar'; decision 2, 'how it counts', ruling-53:17-18), 55 A4, 60, 68 R1 (asked because the `account:` lines 'put T1 at about 34 against the baseline's 33', docs/proposals/spec-batch-1/README.md:123), 74 point 2 and 75 B1 (IN_FORCE.md:271-281). A spike translator does exist outside the engine and passed 14 of 14 states on 2026-11-11 (experiments/011-short-form-spike/README.md:94), on S1's states only.


## FR-282

**Neither increment-4 author struggled with lines; both struggled with concepts, errors and repair**

P02 I6 · fact · verifier: partly · decides: none

The two runs logged 13 mistakes. None concerns how much was written.

- The coordinator's three: root needed a Conduit to its own Pool, and the failed store could not be repaired and was discarded; a refusal that named no reason; a diagnostic that read as an error after a close.
- The fresh author's ten: three of discovery (file names, command syntax), one of environment, two of concept (the same root Conduit; a tool's name prefix), four of errors and repair (failures that exited zero, timeouts with no cause, turns that ended unknown).
- The fresh author wrote JSON, which doubled its lines, because `python3` lacked PyYAML.
- Lines did not track cost. The fresh author's T1 was 263 lines in 5 min 1 s, the fastest T1 of the three authors, with 17 documents written by a script in one second. Its T6 and T7 took 53% of its time for 27% of its lines, spent on two unknown turns and two recreated Agents.
- Overall the coordinator wrote 5.2 times the baseline's lines in 1.19 times its time.
- Of the 12 findings the runs produced (4-06 to 4-17), one, 4-11, is about lines. The rest are refusals, repair, inspection, exit codes, discovery, timeouts and unknown outcomes.
- "Concepts met", one of D22's five measures and pre-registered in the measurement plan, appears in neither report.

The short form will meet the same kind of cost. In the reference's own case, an author who restores A's grant and runs `conduit apply` is stopped with exit 2 and must learn `--reopen` and a generated name. The line count scores that task 2 against 2.

**Evidence.** docs/proposals/first-build/increment-4/coordinator-run/log.txt:4, :5, :12, :39. docs/proposals/first-build/increment-4/fresh-author-run/report.json:85-131 (10 mistakes; :95 "Assumed python3 had PyYAML"; :100; :120; :130), :134-187 (14 unclear points). docs/proposals/first-build/increment-4/fresh-author-run/log.txt:1, :14-30 (17 files at 11:53:48), :71. docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md:32-50 (64, 332, 646 lines; 25 min 25 s, 30 min 13 s, 45 min 48 s; per task). experiments/007-s1-baseline/results-2026-11-09.md:217 (T1 10 min 53 s, 5 min 20 s of it a hung run). docs/proposals/first-build/log-increment-4.md:17-28. `git grep -n -i -c 'concept' -- docs/proposals/first-build/increment-4/ docs/proposals/first-build/README.md docs/proposals/first-build/log-increment-4.md`: measurement-plan.md 2, nothing else. docs/implementation/plan-and-apply/cases/09-t7-restore-unflagged.yaml:20, :244, :366.

**Verifier.** partly. Holds: three mistakes (docs/proposals/first-build/increment-4/coordinator-run/log.txt:4-5, :12, :39) and ten (fresh-author-run/report.json:82-133), which sort as the item says (three discovery, one environment, two concept, four errors and repair); 14 unclear points (:134-190); 17 documents at 11:53:48 (fresh-author-run/log.txt:14-30); T1 in 5 min 1 s (:1, :71) against 8 min 33 s and 10 min 53 s; T6 and T7 are 1,449 of 2,748 s, 52.7 per cent, for 172 of 646 lines, 26.6 per cent (fresh-author-run-2026-11-10.md:44-50); 332/64 = 5.19 and 1,813/1,525 s = 1.19; findings 4-06 to 4-17 are twelve and only 4-11 is about lines (log-increment-4.md:17-28); 'concept' appears in neither report (`git grep -c -i concept` finds measurement-plan.md only). Does not hold: 'JSON, which doubled its lines'. The source says only that JSON 'takes more lines ... for the same document' and gives 46 documents against 33 (fresh-author-run-2026-11-10.md:52-53); the fresh author's T2 in JSON was 67 lines against the coordinator's 73 in YAML (:45). Overstated: the title. Both reports name volume as the main failure ('the author wrote far more', coordinator-run-2026-11-10.md:26-30; finding 4-11), and the fresh author 'created and ran project/build_t1.py to author the initial project' (log.txt:32): a generator script is how an author answers volume.

**Verifier's counterargument.** 'Lines did not track cost' rests on a model's clock (coordinator-run-2026-11-10.md:130), on which agent turns and timeouts dominate and writing costs a second. For a person, 263 lines of JSON is a real cost. The honest reading is that mistakes and repair were about concepts and errors, while volume was the cost both reports ranked first. The closing example cuts the other way from how it is used: case 09's stop names its own repair, flag and operand ('requires=--reopen agent-a--project', cases/09-t7-restore-unflagged.yaml:244), which is what ruling 52 asks; the author need not learn it elsewhere.

**Verifier's corrections.** Replace 'which doubled its lines' with 'in JSON, for lack of PyYAML on `python3` (report.json:95-96); the format an author happens to choose moves the count'. Retitle: 'Neither author's mistakes concerned lines; both met volume as the first cost and errors and repair as the second'. 'The package maps only read/list/write' is report.json:181, not 170-171.


## FR-283

**What waits on the bar, and what the bar waits on**

P02 I7 · fact · verifier: partly · decides: none

Waiting on the owner's judgment of the bar:

- Batch 2's merge: Fleet authority, appointment and with it orders 4 and 5 of Conduit running its own work through a coordinator Agent (ruling 70's first priority), MC9, and apply calling a control, which S4's repair needs. Ruling 71 says batch 2 "begins now"; the handover says "Not begun", since its branch starts from a pin that does not exist.
- Batch 2's rule on secrets for an Agent that others can drive (orders B2-S, B2-C, B2-H; ruling 61 B13), on which the runs of S2, S3 and S10 are gated. S2 is the first test of "establish once, share under restriction"; S10 tests the layered claim.
- Findings 4-01 and 4-11.
- Ruling 62's C6 to C12.
- The coordinator's power to settle any CLI flag, short-form key, template or default.
- The next cold read, and the later reductions of the core.

Waiting on the pin only: the `s3/m2` branch.

The rerun itself waits on twelve orders, none written: 046, 047, 048a, 048b, 050, 053, 043a, 043b, 052a, 052b, 051a, 051b. They include the sign-in chain, which S1's claims do not need but ruling 68 R1 put into the rerun.

Findings 4-01 and 4-11 are in substance already decided by rulings 53, 55, 59 and 76 (the short form, inline definitions, root's default Conduits, apply with stable names). What is left of 4-01 is one choice: retire-then-recreate against an in-place update. That needs the windows without access and the sessions lost, which ruling 55 A3 says are "to be measured in the rerun" and which the line rule does not see.

**Evidence.** docs/proposals/ruling-71/README.md:30-33, :42-44. docs/implementation/HANDOVER.md:217, :219, :221 ("Batch 2 | Not begun"), :238, :277. IN_FORCE.md:171, :178, :307, :310. experiments/013-s2-shared-login/plan.md:204, :259 ("B2-S/C/H supply G1 only after the rerun's judgment"), :294; experiments/014-s3-two-machines/plan.md:187, :230; experiments/021-s10-swapped-providers/plan.md:197, :207. docs/implementation/FIRST_BUILD.md:50, :52. docs/proposals/language-shape/README.md:14. docs/proposals/spec-batch-1/README.md:598-672 (critical path). `ls docs/work-orders | grep -E '^0(4[0-9]|5[0-9])'`: only 040 to 042, 044, 045, 049a1, 050a. docs/proposals/ruling-55/README.md:18. docs/proposals/first-build/log-increment-4.md:12, :22. spec/README.md:339-349 (FLEET-005's default Conduit).

**Verifier.** partly. Holds: batch 2's contents and 'Not begun' (docs/implementation/HANDOVER.md:221; IN_FORCE.md:171, :200, :255); 'begins now ... from the build that the S1 rerun pins' (ruling-71:30-33), so no branch can start; S2, S3 and S10 gated on B2-S/C/H after the judgment (experiments/013-s2-shared-login/plan.md:204, :259, :294; 014 plan.md:187, :230; 021 plan.md:197, :207); findings 4-01 and 4-11 (IN_FORCE.md:310); C6 to C12 (IN_FORCE.md:178); twelve unwritten orders (`ls docs/work-orders | grep -E '^0(4[0-9]|5[0-9])'` shows 040 to 042, 044, 045, 049a1, 050a; HANDOVER.md:217). Wrong on three points. (1) 'ruling 68 R1 put [the sign-in chain] into the rerun': R1 only ruled how sign-ins are counted; they are there under ruling 49, and R1's option (c), API keys, was not taken (docs/proposals/spec-batch-1/README.md:122-128; ruling-68:41-43). (2) Ruling 71 point 4's exclusion ends 'Until the S1 rerun' (ruling-71:43; IN_FORCE.md:307), not at the owner's judgment. (3) 'Waiting on the pin only: the s3/m2 branch' is far too narrow. Twenty order rows in the plans of S2 to S9 name PIN as a dependency: SEC-D1, FLT-D0, S3-CS, S3-L1, S3-RET1, S3-EV, S4-TASK1, S4-LATE, S4-SET, S4-REC1, S5-C1, S5-C3a, S6-ACCESS, S6-END, S7-V, S7-R, S8-V, S8-R1, S9-CORE1, S9-CORE2 (grep of the plan tables under experiments/013 to 020). S5's plan says its core work 'waits batch 1 through 053 → PIN ... absent an assigned side branch' (experiments/016-s5-control-contention/plan.md:163).

**Verifier's counterargument.** On 4-01: ruling 55 A3 already chose retire-then-recreate, 'accepting a longer window without access, to be measured in the rerun' (docs/proposals/ruling-55/README.md:18). The choice is open only in that the measurement may reopen it; A3 says nothing of sessions lost. Plans bind nothing, so the PIN dependencies are how the queue is drawn, not a ruling; a coordinator can redraw them.

**Verifier's corrections.** Replace 'ruling 68 R1 put into the rerun' with 'ruling 49 requires; ruling 68 R1 only says how they are counted'. Move the coordinator's power over flags, keys, templates and defaults to 'waits on the rerun'. Replace the pin line with the twenty rows above: the rerun's pin, more than the bar's judgment, is what holds the engine work of every later scenario.


## FR-284

**The S1 rerun: what it must also report, fixed in order 053's plan**

P02 I8 · experiment-change · verifier: partly · decides: coordinator

Row: the rerun has no row in the register; it lives in step 3 of the roadmap and in A68. Give it one.

Claim tested: declaring relationships costs an author no more than configuring each agent, and delivers what configuring each agent cannot.

Pass condition: as the owner answers I1.

Measures to add, each defined before the run, for both runs and beside the baseline:

1. D22's five, per task: files edited, concepts met (a numbered list: terms of spec/definitions.md, commands and flags, client concepts), grants made by hand, scripts written, recovery steps that need a person. "Concepts met" was dropped last time.
2. The checks of I1's condition 2, each with its evidence, and the baseline's score on the same list. They are a regression test: the short form must not buy lines by losing what the 332-line run had.
3. The fresh author's time to a first success: from opening the pack to the first check that shows T1's state with the real clients.
4. An error ledger: each error or stop met, its text, whether it named the cause and the fix, how it was repaired (an edit, a command, state discarded, an Agent recreated), and the minutes it took. Unknown outcomes and how each was resolved.
5. Per task: commands typed, requests apply sent, applies that stopped partway, sessions lost, and each window without access in seconds (ruling 55 A3). These are what finding 4-01 needs.
6. A layout-free size beside the lines: non-whitespace characters on the counted lines.
7. What the fresh author read from its pack, the format it chose and why.

Cost: a longer plan and report; the baseline's per-task states rebuilt from its log, which is in history at f1cb4734. Buys: a judgment that rests on what the authors actually met.

**Evidence.** ROADMAP.md:27 (step 3), :45-46 ("one row for every experiment"); `grep -n '^| A68' ROADMAP.md`: none; docs/AUDIT.md:462. docs/proposals/first-build/increment-4/measurement-plan.md:38-56 (measures planned), :76-91 (the rerun's rules). docs/proposals/ruling-44/README.md:22-24 (D22). docs/proposals/ruling-55/README.md:18. docs/implementation/plan-and-apply/README.md:73 ("No rollback"), :570-578 (requests per case: 15, 7, 0, 10, 11, 11, 2, 0, 4). `git cat-file -e f1cb4734...:experiments/007-s1-baseline/evidence-2026-11-09/log.txt` succeeds; the tree keeps only prompts.txt. IN_FORCE.md:275 ("Observed time and mistakes are recorded before any numeric target is set").

**Waits.** Order 053; the pass conditions among these wait on I1.

**Verifier.** partly. Basis holds: A68 has no row in the register (`grep -n '^| A68' ROADMAP.md` finds nothing; it is in step 3, ROADMAP.md:27, and docs/AUDIT.md:462); the baseline's log is in history (`git show f1cb4734:experiments/007-s1-baseline/evidence-2026-11-09/log.txt` prints it); 'Concepts met' was planned and not reported. The coordinator is the right decider for the measures. But items 1 and 5 are not additions. D22's five are in force for every run: 'It counts files edited, concepts met, grants made by hand, adapters written, and recovery steps that need a person' (IN_FORCE.md:274). 'Every interval without access is listed' (IN_FORCE.md:276), and order 053's outline already holds '55 A3's windows without access, measured' (docs/proposals/spec-batch-1/README.md:659). The change is to make the report carry what is already owed. 'Duplicated configuration' also went unreported in increment 4 and is not named here; it is the measure of preservation test 3.

**Verifier's counterargument.** Every added measure is counted by a coordinator that also writes one of the two runs; an error ledger adds judgment calls ('named its cause and its fix') that need the fresh verifier of I12 to be worth anything. A leaner change: report all eleven measures of FIRST_BUILD.md:146-158 this time (two were dropped), and add only time to a first success and the error ledger. The layout-free size is not a clean measure: by characters the flow project is smaller or larger than the baseline depending on whether the baseline's two instruction files (508 of its 1,234 characters) are counted.

**Verifier's corrections.** D22 says 'the adapters written' (docs/proposals/ruling-44/README.md:22-24), not 'scripts written'. The restated claim, 'costs an author no more ... and delivers what configuring each agent cannot', replaces thesis 1 ('costs an author less', FIRST_BUILD.md:32); that sentence is the owner's, with I1, not the coordinator's. Add duplicated configuration, per-agent changes of access and provider-specific code to the list of what must be reported.


## FR-285

**Take S1 out of the specification's example, and add three held-out tasks**

P02 I9 · experiment-change · verifier: partly · decides: coordinator

Rows: A70 (ROADMAP.md:59) and the rerun (step 3, A68).

Claim tested: that the short form's economy is the language's and not S1's.

1. Replace `spec/schemas/examples/valid/sectioned-project`, which is "S1 at T2", with a project unlike S1 (for instance one Agent, a Resource with a control and a secret-backed Resource). The specification is the fresh author's format reference, so the example cannot simply be left out of the pack. The change adds no behavior.
2. Add three tasks, written now, done by the baseline and by both rerun authors, reported apart and not gating. No paper count of them is made first.
   - T8: add a third agent, a second Claude Code, that may read the directory and read the history but not post. It prices one more participant, where "declaring relationships costs less than configuring each agent" should show.
   - T9: from the engine alone, say which agents can write to the directory now, and list what agent A wrote there during the run. It tests preservation tests 2 and 7; the baseline keeps no record.
   - T10: give agent B a grant no shipped template covers: it may list the directory's files and not read them. It prices a grant outside `read` and `read-write`.

Measures: those of I8. Must show: each task's counts, time, errors and repairs, on both sides.

Cost: three short baseline tasks with no engine, three more tasks in each rerun, one example rewritten with its schema check. Buys: evidence on fitting that a count of T1 to T7 cannot give, and a first test of the scope guard's worry that a design fitted to one scenario is caught late.

**Evidence.** spec/schemas/examples/valid/sectioned-project/s1.yaml:1 ("# S1 at T2. Pins are illustrative; this example checks authored syntax."); spec/README.md:109-110. `git grep -l 'example.org/s1/' -- spec`: the two sectioned examples only. IN_FORCE.md:278. docs/proposals/first-build/increment-4/measurement-plan.md:24 ("the author does not copy reference-model/examples/s1/"). docs/proposals/ruling-63/README.md:27-29 (shipped templates). docs/AUDIT.md:312-322 (scope guard: "a design fitted to one scenario is caught early"). docs/proposals/first-build/increment-4/fresh-author-run/report.json:170-171 ("The package maps only read/list/write").

**Waits.** Before order 053 writes the fresh author's pack. The example's rewrite is a spec edit for the merge authority (ruling 37); the tasks are logged for the owner to strike.

**Verifier.** partly. The example's facts hold (spec/schemas/examples/valid/sectioned-project/s1.yaml:1; spec/README.md:109-110; IN_FORCE.md:278; `git grep -l 'example.org/s1/' -- spec` lists the two sectioned examples only). The fix is too narrow. Order 053's outline puts 'engine/examples/s1 in the short form' and flow-style guides into the rerun's preparation (docs/proposals/spec-batch-1/README.md:658); the plan-and-apply reference's nine cases are S1's T1 to T7 with their files (docs/implementation/plan-and-apply/cases/09-t7-restore-unflagged.yaml:20-25), and that reference says what ships to the fresh author is 053's decision (README.md:10-11). What is needed is a rule on the pack, 'nothing of S1's design' as in increment 4 (fresh-author-run-2026-11-10.md:13-14), and the example's rewrite as one part of it. The three tasks: T8 adds a third Agent, which needs an account and a sign-in at the panel under ruling 49; that is the owner's resource (IN_FORCE.md:264 reserves the second accounts for use through Conduit), so T8 is not the coordinator's alone. T9 is feasible: `conduit records` exists, with serve stopped (engine/README.md:186-201).

**Verifier's counterargument.** Held-out tasks written by someone who has read the paper counts are not neutral: T9 cannot be done by the baseline by construction, T10 aims at the known template gap, T8 at the place Conduit should win. Extending the baseline after Conduit's design is known repeats review item 9's problem from the other side. The project already has its guard against fitting: scenarios unlike S1 (docs/AUDIT.md:320-321), four of which, S4, S5, S6 and S9, are to be written by a fresh Codex author (IN_FORCE.md:273). Bringing one of those forward tests the short form's generality with no new tasks. For a baseline comparison, round 6 proposed the baseline for 'the same S1/S2 tasks' with 'duplicated credentials' first among its measures (design-critique.md:692-695); S2's was never registered and S2's README disclaims it (experiments/013-s2-shared-login/README.md:136, :258). An S2 baseline is the existing proposal and tests 'establish once, share under restriction', which S1 cannot.

**Verifier's corrections.** State the pack rule and list all three leaks (the spec example, 053's engine/examples/s1, the plan-and-apply cases). Mark T8's account as the owner's. 'The package maps only read/list/write' is report.json:181.


## FR-286

**A70: a second run of the baseline, by a fresh Codex author**

P02 I10 · experiment-change · verifier: partly · decides: coordinator

Row: A70 (ROADMAP.md:59), experiment 007, as a second dated result.

Claim tested: what today's tools cost an author who does not already know them, so that the rerun's fresh author is set beside a like author.

Method: a Codex session that has never seen the project gets the tasks (T1 to T7 and I9's three), the two clients' own help and public documentation and the filesystem server's README, under A70's isolation flags. No engine.

Must show: each task's counts by A70's definitions; time to a first success; the error ledger of I8; whether its T3 answer is complete against the audit's list (the first author missed two routes); whether it catches T5's widened access.

Pass condition: none; it is a baseline.

Why: ruling 71 sets the fresh author's lines against a baseline whose author knew the tools and had read the server's README. The measurement plan recorded this as an open choice, and a critic recommended judging the bar on the coordinator's run and using the fresh run for learnability. The first baseline author made 7 mistakes in 25 minutes; a fresh one will show whether hand configuration is as cheap as 64 lines suggests.

Cost: one Codex session of about an hour and an audit of its counts. Buys: the only like-for-like figure for time to a first success and for errors.

**Evidence.** experiments/007-s1-baseline/README.md:55-59 ("Neither result stands for a newcomer's effort"); experiments/007-s1-baseline/results-2026-11-09.md:459-467 (seven mistakes), :525-527. docs/proposals/first-build/increment-4/measurement-plan.md:110-113 ("asks which run judges the bar: it recommends coordinator versus the experienced baseline author, with the fresh run measuring learnability. That comparison choice remains to be stated explicitly"). docs/proposals/ruling-71/README.md:37.

**Waits.** Nothing; it needs no engine and can run now.

**Verifier.** partly. Basis holds: 'Neither result stands for a newcomer's effort' (experiments/007-s1-baseline/README.md:57-59); seven mistakes in 25 min 25 s (007 results:458-468, :28); the author had read the server's README (:527-529); the critic's recommendation and 'That comparison choice remains to be stated explicitly' (docs/proposals/first-build/increment-4/measurement-plan.md:113-115, not 110-113). One thing changed since that page: ruling 71 stated the choice, 'both runs must pass' (ruling-71:37), so it is ruled, not open. A second dated result under A70 is the coordinator's to add; it uses the owner's subscriptions for the real clients.

**Verifier's counterargument.** No model is fresh to Claude Code, Codex and MCP configuration in the way it is fresh to Conduit, which is in no training data. A fresh session removes project knowledge, not that asymmetry, so 'the only like-for-like figure' overstates what it buys; it gives a lower bound on a newcomer's cost with today's tools. Its value also depends on I1: under (b) or (c) the fresh author's lines no longer gate and the second baseline is context; under (a) there are two baselines and the owner must say which governs. So 'waits: nothing' is wrong: it should follow I1's answer, and I9's tasks if they are kept.

**Verifier's corrections.** Cite measurement-plan.md:113-115 and note ruling 71 point 3 closed that choice. Say 'a nearer author', not 'a like author'.


## FR-287

**Let three things stop waiting on the judgment**

P02 I11 · recommendation · verifier: partly · decides: owner

1. Batch 2's rule on secrets for an Agent that others can drive (B2-S, B2-C, B2-H; ruling 61 B13) is written now and merges when ready. S1 declares no secret Resource, so the rule cannot change what the rerun measures, and the runs of S2, S3 and S10 wait on it.
2. The rest of batch 2 merges at the rerun's pin, not at the judgment. Ruling 71's stated reason for the hold is that the rerun measure a build without Fleet authority; the pin secures that, and a second rerun runs from the same pin.
3. If the owner takes (b) or (c) in I1, ruling 71 point 4's exclusion ends. With "fewer on most" no longer deciding, a flag or a default cannot tip the bar, and choices like ruling 76's five return to the coordinator, logged to strike.

Cost: if the judgment reshapes the language again, merged batch 2 work may need rework, which is why it was held. Under (b) that risk is small: the ceiling's paper margins are wide on T1, T2 and T4, and the delivery checks were already met by the first run.

Buys: the scenarios that test the purpose's own claims, and Conduit's coordinator Agent, stop queuing behind a count of S1's lines and behind twelve unwritten orders; and fewer rulings.

This is the order of the work, so it is the owner's. It can ride in the same answer as I1.

**Evidence.** docs/proposals/ruling-71/README.md:30-33 ("The rerun therefore still measures a build without Fleet authority ... A second rerun, if one is needed, runs on a branch from the same pin"), :42-44. IN_FORCE.md:255 ("Spec pending, batch 2: the rule on secrets for an Agent that others can drive"). experiments/013-s2-shared-login/plan.md:50, :204, :259. experiments/007-s1-baseline/results-2026-11-09.md:437 ("No credential exists in S1"). docs/proposals/ruling-76/README.md:5-6 ("Every reading that could move S1's author counts it reserved for the owner"). docs/implementation/HANDOVER.md:221.

**Waits.** S2's, S3's and S10's runs; A78's orders 4 and 5; MC9; S4's T1 repair.

**Verifier.** partly. The owner is the right decider and the item says it amends ruling 71 point 2. Basis holds for points 1 and 2: the hold's stated reason is a rerun 'without Fleet authority' (ruling-71:30-33); B13 is batch 2 (IN_FORCE.md:171, :200, :255); S1 has no credential (007 results:444-445), and B13's one-hop display covers turn-holders 'other than root' (IN_FORCE.md:200), which S1 has none of. Two claims fail. 'A flag or a default cannot tip the bar' under (b): the ceiling keeps one-line margins on T6 and T7 (forecast 2 against a limit of 3) and any line on T3; the item's 'wide on T1, T2 and T4' leaves out T5 to T7. And the exclusion already ends at the rerun (ruling-71:43), so point 3 buys only the weeks before it. 'Stop queuing ... behind twelve unwritten orders': S2's run gate is 'PIN, B2-S/C/H, 050' (experiments/013-s2-shared-login/plan.md:204) and twenty scenario order rows wait on PIN (see I7). I11 leaves all of that queued; it saves the interval between the pin and the judgment, and the loop if a rerun fails.

**Verifier's counterargument.** The larger lever needs no ruling: scenario core work waits on PIN only 'absent an assigned side branch' (experiments/016-s5-control-contention/plan.md:163), and the coordinator already made one, s3/m2 (HANDOVER.md:277). Assigning side branches for the scenario core, or cutting the pin as soon as the rerun's build inputs are frozen, is a build decision. Risk in point 1: B2-C adds checks at open and register and B2-H changes what plans and inspection print, in the build the rerun measures; it must be shown that S1's printed plans and tool lists are unchanged, or the rule merges after the pin like the rest.

**Verifier's corrections.** Replace 'cannot tip the bar' with 'cannot tip fewer-on-most; can still tip the ceiling on T5 to T7'. Replace the 'Buys' sentence with what is saved: the pin-to-judgment interval and a failed rerun's loop.


## FR-288

**Five small fixes the coordinator can make now**

P02 I12 · recommendation · verifier: partly · decides: coordinator

1. Count by script, checked by a fresh reader. Use experiment 023's method in the rerun (`git diff --no-index --minimal -U0`, each hunk costing the larger of its removed and added lines), keep each task's state, and have a fresh read-only verifier recount, as the baseline's audit did. The increment-4 counts were never rechecked.
2. Stop paper forecasts. Until the rerun, no line count is set beside a design decision; a CLI or short-form choice is justified by the system it follows (Terraform, kubectl) and by safety.
3. Bring the log up to date. Rows 4-01 and 4-11 still read "being settled with the owner". State what rulings 53, 55, 59 and 76 decided, and that one choice is left: retire-then-recreate against an in-place update, to be judged on measured windows without access.
4. Remove the experiment's word from the specification. spec/README.md says a dangerous act's flag is "counted when used", a cascade is "counted under one flag", and a plan "counts every widening". That is ruling 55 A4's counting rule for the rerun, in binding text; a builder cannot tell what must be counted or by whom. Write "recorded with the plan".
5. Say in order 053 that T3's inspection commands are inspection effort and not lines. Ruling 63's rule does not exempt a command that names an Agent, and one such line fails T3 under ruling 71.

Cost: one small docs order and one clause in 053. Buys: counts nobody has to trust, and a specification free of a measurement rule.

**Evidence.** experiments/023-paper-count/README.md:23-27 (the diff method). experiments/007-s1-baseline/results-2026-11-09.md:597-609. docs/proposals/first-build/log-increment-4.md:12, :22 ("Being settled with the owner since 2026-11-10"). spec/README.md:352-357 ("counted when used"; "printed and counted under one flag"; "A plan prints and counts every widening"); docs/proposals/ruling-55/README.md:19. docs/proposals/ruling-63/README.md:19-20; experiments/023-paper-count/results-2026-11-11.md:159; docs/proposals/first-build/increment-4/measurement-plan.md:35-37 (checks not counted).

**Waits.** Item 4 is a wording fix under ruling 37; item 5 is moot if the owner takes (b) or (c) in I1.

**Verifier.** partly. 1 holds: the diff method is at experiments/023-paper-count/README.md:26-29; the baseline's audit at 007 results:597-617; no recount in increment 4. 2 holds and agrees with ruling 45 (:76). 3 holds: rows 4-01 and 4-11 still read 'Being settled with the owner since 2026-11-10' (docs/proposals/first-build/log-increment-4.md:12, :22). 4: the words are there (spec/README.md:352-358), but the section opens 'They are statements, not promises, and nothing tests them' (:333-334), so 'in binding text' overstates; and 'counted when used' is ruling 55 A4's ruled wording (docs/proposals/ruling-55/README.md:19), repeated as 'In spec/' at IN_FORCE.md:77. 5 holds and matches precedent: order 073 fixed T3's primary at zero with commands as inspection effort (023 README.md:38), and increment 4 treated `tools --as` as a check (measurement-plan.md:38-40).

**Verifier's counterargument.** On 1: a diff of states counts net transitions; the baseline counted from a log, including a line added for a mistake inside a task. Use one script on both sides, with the baseline's per-task states rebuilt from its log at f1cb4734, or the two counts are not the same measure. On 4: the wording fix is the coordinator's (ruling 37), but it changes words the owner accepted, so IN_FORCE.md:77 changes in the same edit and the change is logged to strike.

**Verifier's corrections.** Item 4: say 'in a section of spec/README.md that states limits, not promises'.


## FR-289

**What is working**

P02 I13 · assessment · verifier: supported · decides: none

- The baseline experiment is good work. It was pre-registered, run with the real clients, audited by a second reader who corrected four counts, and candid about its limits. It found things worth knowing whatever happens to the bar: trust lives in the home directory, an upgrade widened access, a revocation left the server running, and an agent's reach included servers no file declared.
- The bar earned its keep once. Two runs at 332 and 646 lines against 64 showed a real fault, and the reply, the short form with inline definitions, default Conduits for root and apply, removed it on paper: 35 lines against 64, and a final state the size of hand configuration.
- The first success criterion already held in increment 4: a fresh author finished every task from the specification and the commands alone, reading no ruling history and editing no provider.
- The project saw the weakness itself. Its own critic wrote that the pass depends on writing each grant as one long line and that the bar is "work" while only lines are projected; the synthesis called the bar "thin and on paper". The remedy chosen was to rule the layout. The better remedy is to stop letting layout decide.

**Evidence.** experiments/007-s1-baseline/README.md:1-9, :55-66; experiments/007-s1-baseline/results-2026-11-09.md:478-504, :597-628. docs/proposals/first-build/increment-4/fresh-author-run-2026-11-10.md:22-24, :82-83. docs/implementation/plan-and-apply/README.md:537-547 (total primary 35). docs/proposals/language-shape/short-form/critic.md:31-39, :117; docs/proposals/language-shape/short-form/synthesis.md:496-498.

**Verifier.** supported. Pre-registration, real clients, audit correcting four counts (lines, concepts, recovery steps, mistakes) and candour about limits (experiments/007-s1-baseline/README.md:3-9, :55-64; results:506-537, :597-628). 332 and 646 against 64 (fresh-author-run-2026-11-10.md:32-34). 35 on paper (docs/implementation/plan-and-apply/README.md:538-547). The first criterion held: 'read no ruling history and edited no provider' (fresh-author-run-2026-11-10.md:22-24, :82-83). The critic's two sentences and the synthesis's 'THE BAR IS THIN AND ON PAPER' are as quoted (docs/proposals/language-shape/short-form/critic.md:31-36, :117; synthesis.md:494). Ruling 63 point 2 is the remedy that ruled the layout.

**Verifier's counterargument.** One caveat on 'what is working': the accepted criterion 'The user can identify and resolve uncertainty without guessing that an effect failed' (FIRST_BUILD.md:113) was not met by the fresh author, who 'could not confidently attest whole-turn completion', kept two turns unknown and recreated both Agents (report.json:188-194); finding 4-12 was then closed with reconcile.v1 unchanged (log-increment-4.md:23). Only the first criterion is shown to hold.


## What the verifier says the analyst missed

- **The rerun's pin, not only the judgment, holds the engine work of S2 to S9** The assignment asked what waits on the bar. Twenty order rows across the plans of S2 to S9 list PIN as a direct dependency (SEC-D1, FLT-D0, S3-CS, S3-L1, S3-RET1, S3-EV, S4-TASK1, S4-LATE, S4-SET, S4-REC1, S5-C1, S5-C3a, S6-ACCESS, S6-END, S7-V, S7-R, S8-V, S8-R1, S9-CORE1, S9-CORE2), and experiments/016-s5-control-contention/plan.md:163 says the core work 'waits batch 1 through 053 → PIN ... absent an assigned side branch'. The pin exists so that the rerun measures a frozen build. The analyst named only s3/m2. This is the widest hold the bar causes, and the coordinator can lift it with side branches, as it did for s3/m2 (HANDOVER.md:277), without a ruling.
- **Review item 9 and the unequal treatment of a person's clicks** The assignment named item 9 of to-review-with-the-owner.md; no item discusses it. Two things follow from it. First, the rules added after the baseline (FIRST_BUILD.md:163-188) qualify the baseline's T4 and T5 failures that I4 leans on (see I4). Second, the two sides count a person's dialog answers differently: the baseline's two trust dialogs are counted as 9 authored lines in T1, 'a reader who prefers to count it so can subtract 9 lines and 2 files from T1' (007 results:112-115), while Conduit's sign-ins at the panel are listed apart (ruling 68 R1; IN_FORCE.md:277). Likewise the baseline's 6 lines of history convention count in T2 (007 results:222-224) while Conduit ships the history as a package, and counting that convention as provider work is still 'a proposal' (measurement-plan.md:118-120). Read alike, T1 is 21 against 24, a margin of 3, not 12. Ruling 47's row says rules set after the baseline 'are reported apart, so the comparison can be read by either set'; order 053 should report this reading.
- **Round 6 proposed the baseline for S1 and S2; S2's was never run** design-critique.md:692-695 says 'compare the same S1/S2 tasks' and lists 'duplicated credentials' first. Ruling 45 took 'the baseline for S1' only (ruling-45:61-66), and S2's pre-registration carries no comparison (experiments/013-s2-shared-login/README.md:136, :258). The analyst notes that S1 has no credential and cannot test 'establish once, share under restriction', then proposes three new S1 tasks instead of the experiment round 6 already named. An S2 baseline (two agents, one service login, each configured by hand) is where the baseline must copy the credential and Conduit must not; it answers the unlike-goods objection directly. It would be a new register row; the coordinator can propose it, and it needs no engine.
- **The bar is what put the short form into the language, at a stated cost in promises** Ruling 53's first decision is 'Whether an authoring layer counts for the bar, or the short form must be the language' (ruling-53:17). The record that led to ruling 55 A1 prices the choice: 'about 38 to 45 promises in all, against 13 to 18' for a CLI layer, and gives as a reason that 'spec/ itself is the format reference a fresh author reads' (docs/proposals/language-shape/README.md:43-46). spec/promises/sections.yaml now holds 15 promises with no vector. For an owner who wants a concise, provable language this is the bar's largest effect, and the analyst's I5 does not name it. Whether the short form should stay language or return to tooling is the owner's question, for after the rerun.
- **The 1.5 threshold had no slack at T7 on the day it was set** When ruling 71 was recorded (2026-11-11 11:48), the only count in the tree was experiment 011's: T7 at 4 lines against 2 (experiments/011-short-form-spike/results-2026-11-10.md:103), which ruling 63's remove-and-restore-once rule makes 3 (023 results:314). 'More than one and a half times' passes 3 against 2 exactly, and ruling 76's record notes T7 'sits exactly at one and a half times' (:10-11). This does not show the figure was chosen to fit; the record gives no reason either way. It does show the per-task ratio is a one-line test on T5 to T7, which argues for a ceiling on grouped totals.
- **The proposed delivery checks largely restate gates already in force** v0 already requires 'a record for each request' with real Agents (IN_FORCE.md:263), and the criteria fixed before the run already include 'closing one Agent's access leaves the other working' and 'the user resolves uncertainty without guessing that an effect failed' (IN_FORCE.md:275; FIRST_BUILD.md:110-116). The analyst's summary treats the line count as the only gate on S1. The real defect is narrower: ruling 71 points 2 and 4 hang batch 2 and the coordinator's delegation on the bar's row alone, and 'both runs must pass' on lines went beyond ruling 48 point 4, which gave the fresh author the first success criterion to test (ruling-48:31).
- **The fresh author's format decides its count, and last time an accident chose it** Under ruling 71 the measure is the lines the author actually writes; the block reading is only 'reported beside' (ruling-63:22-23). In increment 4 the fresh author wrote JSON because `python3` had no PyYAML (report.json:93-97). In the rerun, whether it writes flow, block or JSON turns on the example it copies and on `conduit init`'s skeleton, which is why ruling 63 point 2 ruled the documented style. A pass by the fresh author would therefore measure whether it copied the layout, not the language. This supports taking the line gate off the fresh author's run, and it is a reason the analyst could have given in place of '(b) rescues nothing'.

## The verifier's notes

Reviewed at commit 2eb8f258 (2026-11-12), read-only, in the review checkout; nothing was built, run or changed.

Overall judgment. The analyst's facts are mostly right and its direction is sound: the line count worked as a tripwire, the per-task rule now turns on layout and one-line margins, the baseline is not a like good, and neither author's mistakes were about lines. Twelve of thirteen items need a correction, though, and four matter to the owner's decision.

1. The recommendation's key reassurance is wrong. Option (b) does not 'rescue nothing': it removes the line gate from the fresh author's run, the run most likely to fail it. That may be right (ruling 48 point 4 gave the fresh author the first success criterion to test, not a line count), but the owner must be told plainly, and told that it re-words thesis 1 of the pre-registration after two failed runs.
2. Option (b) keeps a per-task ratio on baselines of 2 and 3 lines, which is the defect the analyst's own I2 describes. A ceiling on two grouped totals (setup; change and repair) avoids it, and one sentence amending ruling 71 point 3 would do, since most of the proposed delivery checks are already in force (IN_FORCE.md:263, :275).
3. The case against the baseline is overstated in places. The T5 widening was deliberate, the T4 result is confounded by the audit's own account, the baseline's T7 revocation worked, and the 332-line run was better on three of round 6's five measures, not all.
4. The largest thing the bar holds was missed. The rerun's pin gates twenty engine orders across S2 to S9; the coordinator can release them with side branches and needs no ruling.

Also missed: review item 9 and the unequal counting of a person's clicks, round 6's unrun S2 baseline, and the bar's role in making the short form part of the language.

Smaller errors: 5 diff hunks, not 4; ruling 49, not ruling 68 R1, puts sign-in into the rerun; ruling 71 point 4's exclusion ends at the rerun, not at the judgment; 'the owner's words' in ruling 47 are a coordinator's draft the owner accepted; several line cites are off by two to ten lines.

Coverage. Read in full:
- docs/guides/full-review.md, docs/implementation/HANDOVER.md, AGENTS.md 'What holds'
- rulings 44 (to line 80), 45, 53, 55, 63, 71 and 76; ruling 47 to line 80
- docs/implementation/FIRST_BUILD.md and to-review-with-the-owner.md
- experiments/007-s1-baseline: README and results
- docs/proposals/first-build/log-increment-4.md, both increment-4 run reports, the measurement plan, the coordinator's log and T3 answer, and the fresh author's report.json
- experiments/023-paper-count: README, and results lines 1 to 330
- IN_FORCE.md topics 13 and 15; docs/AUDIT.md Purpose, preservation tests, scope guard, layered claim and the waiting list

Read in part or by grep:
- docs/implementation/plan-and-apply/README.md (lines 1-125, 160-262, 460-585) and case 09 only
- round 6's design critique (D22 and the baseline passage)
- docs/proposals/language-shape/README.md (to line 80), with the critic's and synthesis's cited passages
- docs/proposals/spec-batch-1/README.md (R1 and the order list)
- rulings 48, 49, 61, 68, 72, 74 and 75
- the scenario plans under experiments/013 to 021 (dependency rows only)
- experiment 011, the fresh author's log beyond line 80, and ROADMAP.md

Not read: work order 073; the other eight plan-and-apply cases; experiment 023's results after line 330; the ledger beyond the rows cited.

Recounts were made with grep, tr, git diff --no-index and /usr/bin/python3 with PyYAML. The claims about ISO 9241-11 and about general practice in gating configuration languages were not checked against any source in the repository. Ruling 71's later answers concern the Codex allowance only and do not touch the bar.
