# G6-outside-the-tree: items

Not binding. Part of [the full review](../README.md) of 2026-11-14 (second pass, the approach and the experiments), at commit `2eb8f258`: what one analyst returned, with the adversarial verifier's verdict under each. Where the verifier corrected something, the correction governs. [Index of all parts](README.md).

**The analyst's summary.** At 2eb8f258 the specification, the engine, the provider packages, the model-checking harness with its dated tables and traces, and the S1 baseline's results are all in the tree or in main's history, and an outside reader can check them. What is not in the tree is the means of the work and the evidence of the activity the owner ranked first. The order runners are described in 13 lines of the handover and exist only beside a private note. The dogfood harness, the conduit-core Fleet's documents, A78's plan and every measurement of order 042's run through Conduit are outside it too, and the engine's own records of that run sit in a store the handover plans to initialize again. The round-2 verifier reports that orders 071 and 080 are to resume from are in no branch. No Record from 037 to 150 names the model or client version that did the work, and an exposure of keys on 2026-11-11 has no recorded close. This is a repeated fault, not a new one: the tree states the rule in four places and records the same loss at least four times before the handover. I would therefore make the fix a check at merge (a fresh reader who must dispatch from the tree alone; a register row that must link a results file), not another rule. Private details are not the real obstacle: every path, host and user name in a runner can be a variable, only the store and the event streams must never be committed, and the path checker needs three more patterns before redacted scripts go in. One docs order, about two hours of the coordinator's own redaction, and one question to the owner (were the printed keys replaced) close nearly all of it. The one step with a deadline is exporting the dogfood store's records with the pinned engine before that store is re-pinned or initialized again.

**What the analyst read.** Commit 2eb8f258 (2026-11-12), read only; nothing built, run or written; no network, so nothing on GitHub (branch protection, repository variables, CI runs, log retention) was checked. Read in full: docs/guides/full-review.md; docs/implementation/HANDOVER.md; AGENTS.md; docs/implementation/WORK_ORDERS.md lines 1-223; docs/work-orders/README.md, verify.md and verify.schema.json; docs/guides/second-machine.md; experiments/README.md; experiments/007-s1-baseline/README.md; tools/check_private_paths.py; the first pass's approach critic, its approach index, P06 I12 and I13 with the verifier's notes, P01 I9, I10 and I13, P05 I4 and I6, P07 I9, E01 I14. Read in part: the Records of orders 041b, 042, 045, 068, 069b, 070, 072, 074, 075, 077, 079, 081, 082, 104, 105, 116, 137 and 150, and with read-only git the branch-only Records of 144 and 049a1; experiments/012-model-checking/README.md (head, Run it, Results, all headings); ROADMAP.md rows A70 to A82 and the rules for proposing an experiment; docs/AUDIT.md (where things stand, purpose, preservation tests); IN_FORCE.md topic 15; the records of rulings 74 and 77; the review list; the SSH runtime provider's README (setup, D15, build); .github/workflows by grep; engine/README.md's lines on the records export; REC-001; CONTRIBUTING.md lines 96-112; docs/proposals/metrics/general.md's source list; the language critic's first missing area and X03's index lines. Counted by command: the '(log)' marks; order files and Records; register rows by state; history-pinned commits and their ancestry; the baseline's removed files and their bytes; private-path patterns in that history and in the first pass's files (counts only, no value printed). Not read, by rule: the private handoff note, any log, the runners, the harness, the dogfood store, any credential. Not read for lack of need: spec/ beyond REC-001, engine code beyond one function, about 54 of the 72 Records beyond locating them, the first pass's language files beyond the index. Claims about GitHub's log retention, pull-request refs and the Twelve-Factor and SLSA practices are general knowledge, not checked here.


## FR-525

**The inventory: fourteen things a third coordinator or an outside reader cannot reproduce from the tree, ranked by what a loss would cost**

G6 I1 · fact · verifier: partly · decides: none

Each row: what it is; where the documents say it lives; whether private details stop it being committed as it is; the least that should enter the repository. Rows 1 to 5 are the ones whose loss would stop work or erase a result. Rows 11 and 12 are safe and listed so the record says so.

| # | What | Where the documents say it lives | Private details in it | Least to enter the repository | What a loss costs |
| --- | --- | --- | --- | --- | --- |
| 1 | The order runners: the dispatch runner (named once, `run-order.sh`), the runner that continues from a verifier report, the one that starts a remote copy on the Windows computer | "A runner kept outside the repository"; listed with the private note | Yes as they are (local paths, a host, user names); none is needed, each can be a variable | The scripts as text with variables, an example variables file, a method-as-run page | No order can be dispatched; rebuilt from 13 lines of prose, with at least five recorded fixes relearned |
| 2 | A78: the harness, the conduit-core Fleet's documents, the plan, the smoke test's and order 042's measurements, the engine's own records of 042's turns | "A harness outside the repository"; "a store outside the repository"; results marked '(log)'; the plan is cited by ruling 74's record and has no file | Harness and Fleet documents: yes (slot paths). The store: never (it holds logins). Measurements: no | One experiment directory: method as run, dated results, the Fleet's documents with placeholder paths, a scrubbed records export; five log rows | The first-priority activity (ruling 70) rests on a 16-line Record; the store is due to be initialized again after order 050, which destroys the records |
| 3 | Worker reports, verifier reports, reviews before dispatch, event streams, the coordinator's own time | Files the dispatch command writes; "its logs" in the private note; spend only in session transcripts | Reports may quote worktree paths (scrub); event streams never | Now: a Record on the branches of 071 and 080 with the last verifier's findings verbatim. From the next order: the final worker report and each verifier report committed with the Record | 071's and 080's next step names a file that is outside; A77's question cannot be answered |
| 4 | Whether the keys an agent printed on 2026-11-11 were replaced | One sentence of the handover, marked '(log)' | None needed: labels only | One dated line | An exposure with no recorded close |
| 5 | This review's first pass: 263 findings and 182 items with verdicts, 40 files, 2.47 MB | A session's scratch space | 17 files match the path checker's patterns (31 occurrences) | The two indexes and the record now; the per-reader files scrubbed, leaving at the next condensation | The whole first pass |
| 6 | Eleven facts marked '(log)' | "found only in the coordinator's private notes" | None | A row or a sentence each (I8) | Small each; the A78 ones are row 2 |
| 7 | Checks run by hand on the Mac whose evidence is a sentence: 077's merge, the normal and differential vector runs, 070's timings | Records and the handover | None | One result line per merge: commit, command, last line, machine by label | A rerun of about 2.4 hours for the differential vectors |
| 8 | State held by GitHub: CI paused on 2026-11-11, red at times since #260, the protection of `main`, the variable `CONDUIT_PRIVATE_NAMES` | GitHub; cited by pull request, never by run | The variable's value: yes. Its existence and the rules: no | A paragraph naming the settings; run addresses in the CI fix's Record | Nobody can tell which merges had no CI, or whether names are checked |
| 9 | Which model, effort and client version carried out each order | Nowhere | None | Three fields the runner writes | Worker quality cannot be compared across dates |
| 10 | The second machine: the result of each setup step's check, capacity, the remote work-order copy | "They stay outside the repository"; capacity marked '(log)' | Name, address, fingerprint: yes. Outcomes: no | A dated table of steps done and checked; one capacity sentence | The owner or coordinator re-derives the machine's state |
| 11 | The S1 baseline's raw evidence: 33 files, 77,254 bytes | Main's history at f1cb4734, linked 36 times from the results; removed by order 066 under ruling 58 point 5 | None by the checker's patterns (0 matches) | One sentence giving the offline command | None while history is intact |
| 12 | Model-checking outputs | In the tree: tables and traces in the README, the harness in src/ | None | The commit beside each dated result; two stale index lines | None |
| 13 | Earlier sources kept only in session scratch: the metrics benchmark and probe store, the N1 to N4 papers, the condensation's skeptic items | Session scratchpads, by the documents' own words | Unknown | Mark the cited numbers "source not kept" | Already lost, most likely |
| 14 | The "last version" link at spec/README.md:279 | A commit reachable only through pull request 7's head ref | None | A tag on that commit | Two links break in any clone of `main` alone |

**Evidence.** Row 1: docs/implementation/HANDOVER.md:8-10, :55-67; docs/guides/full-review.md:46-47; docs/proposals/first-build/to-review-with-the-owner.md:65. Row 2: HANDOVER.md:123, :130, :141, :157-158; docs/work-orders/050a-agents-v2-own-tools.md:26-27; docs/proposals/ruling-74/README.md:5; P06 I12 (verifier: supported). Row 3: docs/implementation/WORK_ORDERS.md:50-56; HANDOVER.md:99-101, :212-213; ROADMAP.md:70. Row 4: HANDOVER.md:282-284. Row 5: `ls approach language | wc -l` on the first pass's files = 40; grep count of the checker's patterns = 17 files, 31 occurrences. Row 6: `grep -n '(log)' docs/implementation/HANDOVER.md` = 12 lines, one the definition. Row 7: HANDOVER.md:208-210; docs/work-orders/070-build-state-copies.md:364-366. Row 8: docs/work-orders/104-mc1-model-checking-harness.md:202; 105-mc2-operation-identity.md:287; .github/workflows/checks.yml:65; `grep -rnoE 'actions/runs/[0-9]+|run [0-9]{9,}'` over the tree = 0 matches. Row 9: grep for model names and client versions over docs/work-orders, HANDOVER.md and PROVIDERS.md finds only order 050a's tested versions. Row 10: docs/guides/second-machine.md:114-118; HANDOVER.md:184-185. Row 11: `git ls-tree -r --long f1cb4734 experiments/007-s1-baseline/evidence-2026-11-09` = 33 files, 77,254 bytes; `git merge-base --is-ancestor f1cb4734 HEAD` succeeds; removal in f3a670a2. Row 12: experiments/012-model-checking/README.md:25-35, :108-124. Row 13: docs/proposals/metrics/general.md:1366-1368; docs/proposals/language-shape/short-form/critic.md:103; docs/proposals/condensation/critic.md:117-118. Row 14: `git merge-base --is-ancestor 0a9a3232 HEAD` fails; `git branch -r --contains 0a9a3232` gives origin/pull/7.

**Waits.** I2's work order is written from this table.

**Verifier.** partly. Most rows recount. `grep -n '(log)' docs/implementation/HANDOVER.md` gives 12 lines, and line 8 is the definition. The first pass is 40 files and 2,471,222 bytes; 17 files match the checker's patterns, 31 times. At f1cb4734 the baseline's evidence is 33 files and 77,254 bytes, the commit is an ancestor of HEAD, and the results link it 36 times. The tree pins eight link commits; only 0a9a3232 is not an ancestor, and only origin/pull/7 reaches it. `git grep -nE 'actions/runs/[0-9]+'` finds nothing. A78's state cell links no file (ROADMAP.md:71, parsed with /usr/bin/python3). START_ROUND, VERIFY_FIRST, ROUNDS, EXTRA, SCHEMA, JOBS and TT appear only in HANDOVER.md:53-65 and :300-305 (`git grep -nw` per name).

**Verifier's counterargument.** This is an inventory, not a ranking of the whole area by what a loss would cost. It leaves out items that matter as much as rows 6 to 14 (see missed). Several entries in the 'least' column already exist in part in the tree.

**Verifier's corrections.** Row 1, 'at least five recorded fixes': the Records attribute three fixes to the runner: the git identity (docs/work-orders/069b-spec-plan-apply-ruling-76.md:85-87), rustup's directory (075-spec-own-tools-reach.md:638-639) and `--no-track` (docs/implementation/WORK_ORDERS.md:45-47). Add one unexplained exit 127 (079-shell-https-address.md:355) and two dispatch failures marked (log) at HANDOVER.md:305. Row 2: 042's evidence is its Record (042-build-tombstones-and-inventory.md:47-62) and also the handover's A78 section (HANDOVER.md:119-158), about 40 lines in all. Row 7 cites HANDOVER.md:208-210, but the rows are at :209-210. Row 8: the variable is described at AGENTS.md:153 and .github/workflows/checks.yml:60-61; the unknown is only whether it is set. Row 10: the guide already holds a dated line on which steps were done (docs/guides/second-machine.md:21-25); only the result of each step's check is missing.


## FR-526

**One work order: the means and the evidence of the work enter the tree**

G6 I2 · recommendation · verifier: partly · decides: coordinator

A worker cannot and must not read the private note, so the coordinator prepares the inputs and the worker writes, checks and cross-links. I estimate two hours of the coordinator's own time for the redaction; it cannot be delegated.

**What the coordinator prepares.** (a) The three runners and the dogfood harness, every private value replaced by a variable. (b) Extracts of its log for the eleven '(log)' marks and for A78. (c) The last verifier report of 071 and of 080. (d) With the engine still pinned at 38c0b4f, a records export of the dogfood store. Do (d) before the re-pin and before the store is initialized again; it is the only step with a deadline.

**Parts.**
1. `docs/implementation/runners/`: the scripts as text files, an example variables file, and a method-as-run page. The page gives the worker's and verifier's commands in full, every variable, the guards, the checkpoint commit, what the verifier is shown, the load rule and the failure modes the Records already name. Correct WORK_ORDERS.md's dispatch section and verify.md where they differ from practice (I3).
2. An experiment directory for A78, numbered once by the coordinator: method as run, `results-2026-11-12.md`, the Fleet's documents with placeholder paths, the scrubbed export. Rows in increment 4's log for the four frictions (the first pass drafted them, P06 I8 to I11) and for the architecture names.
3. On the branches of 071 and 080, a Record of where each stands with the verifier's findings verbatim, as 144's branch already has. From the next order on, the runner commits the final worker report, each verifier report and a short header with the Record.
4. The '(log)' marks to zero (I8).
5. In the second-machine guide: a dated table of steps done and checked, the capacity sentence, and "ask the owner before part 2".
6. One result line in the Records of 077 and 042; the CI fix's Record names its three runs by address; a paragraph in CONTRIBUTING.md naming the repository's settings and variables, without values.
7. One sentence in the baseline's README giving the offline command; the two stale lines about experiment 012 in experiments/README.md; a tag on commit 0a9a3232.
8. Three more patterns and their unit tests in `tools/check_private_paths.py` (I10). This part is code and may be a separate small order.
9. The first pass of this review under `docs/proposals/full-review/` (I11).

**Done when.** The six checks of AGENTS.md pass. `grep -c '(log)' docs/implementation/HANDOVER.md` prints 0. A fresh read-only reader, given only the tree, writes the exact commands to dispatch order 147, to continue 071 from its last verifier report and to start an order on the second machine, and lists every value it had to guess. It passes if the only guesses are values the example file marks private.

**What I would not do.** No rewrite of the runners in Rust: A78 is meant to replace them, and scripts kept as text have precedent (six `.py.txt` files of increment 4, and the method-as-run pages of experiments 008 and 009). No event streams and nothing of the store. No backfill of reports for the 72 Records already written. No restoring of the baseline's files, which ruling 58 point 5 removed on purpose.

**Existing paradigms.** Configuration in the environment with a checked-in example file (the Twelve-Factor practice). The repository's own rule for raw records (committed with the result, removed at the next condensation, linked at their last version). A provenance header naming builder, source commit and parameters, as SLSA provenance does. A git tag to keep a commit reachable.

**Cost and what it buys.** One docs order, likely two rounds (docs orders 060 to 066 took 12 to 21 worker minutes and 2 to 6 verifier minutes each; this one is larger), plus the coordinator's two hours. It buys: a third coordinator can dispatch from the tree; A78 has a result a reader can check; A77 gets data; the private note shrinks to names, addresses and accounts, which the owner can restate.

One point to log for the owner to strike: scripts kept as text beside the rule that code is Rust. If the coordinator wants the tree's copy to be the copy that runs, that is an exception to AGENTS.md's rule and goes to the owner in the next batch; this order does not need it.

**Evidence.** AGENTS.md:79-80 ("Evidence: its own directory under `experiments/`, with dated results"), :86-89 ("Code is Rust ... The only exception is the Python checkers in `tools/`"). CONTRIBUTING.md:109-111 ("Dated results, the method as run, prompts and counted artifacts remain as v0 evidence. Raw run records leave the tree once their results are merged, with links to their last version in git"). experiments/README.md:78-83. `git ls-files | grep -E '\.(py|sh)\.txt$'` = 6 files under docs/proposals/first-build/increment-4/fresh-author-run/. experiments/009-native-launch/HARNESS.md:1-6 ("Historical method as run"). docs/implementation/WORK_ORDERS.md:216-222 (times of orders 060 to 066). HANDOVER.md:153-158 (pin at `38c0b4f`; "After 050, the store is initialized again"). `git show origin/wo/144-s4-srv2-hook:docs/work-orders/144-s4-srv2-hook.md` line 363 on (the branch Record that is the model). ROADMAP.md:98-99 (directory numbering).

**Waits.** The lane's restart and any dispatch by a session without the private note wait on part 1. A78's next order (046) waits on part 2's export. Rows 4 and 5 of the handover's queue wait on part 3.

**Verifier.** partly. The factual basis holds: AGENTS.md:79-80 and :86-89; CONTRIBUTING.md:109-111; experiments/README.md:80-83 ('a harness's method and flags as run ... stay with them'). There are six `.py.txt` files under docs/proposals/first-build/increment-4/fresh-author-run/. Orders 060 to 066 took 12 to 21 worker minutes and 2 to 6 verifier minutes (WORK_ORDERS.md:216-222). The pinned engine already has the export: `git show 38c0b4f:engine/README.md` has `records --store` at line 187 and the export's description at :200. 144's branch Record begins at line 363.

**Verifier's counterargument.** (1) The done-when cannot pass: `grep -c '(log)'` counts the definition at HANDOVER.md:8. (2) The `.py.txt` files are frozen evidence of one fresh author's run, not tools in use. A text copy of a runner that is not the copy that runs will drift, as the documents' numbers do. A method-as-run Markdown page with the exact commands in code blocks gives the same reproducibility with no exception to the Rust rule; that is the form of WORK_ORDERS.md:49-56 and experiments/009-native-launch/HARNESS.md. Only making the tree's copy the copy that runs needs an exception, and that is the owner's. (3) Committing reports, records exports and scripts needs a scan for secrets as well as paths. The checker detects no key (tools/check_private_paths.py:19-23), and HANDOVER.md:282-283 records that a session printed keys. The existing paradigm is gitleaks in CI or GitHub's secret scanning with push protection. (4) The step that destroys the records is order 050's new store (HANDOVER.md:157-158), not the re-pin: the store's format is still version 1 (engine/CONFORMANCE.md:18). Exporting first is still prudent. (5) The 'two hours' has no evidence behind it.

**Verifier's corrections.** Done-when: '(log)' appears only in its definition, or the definition is removed. Part 2: the handover's four frictions (HANDOVER.md:143-150) were drafted in the first pass as P06 I8 (4-21), I10 (4-22), I9 (4-24) and I2 (4-23, the escaped children). P06 I11, the guard that tripped, comes from :137 and is not one of the four; P06 drafted six rows in all, 4-21 to 4-26. The coordinator decides. No ruling is contradicted, provided the choice to keep scripts as text is logged for the owner to strike.


## FR-527

**The runners: 13 lines of prose, one command and one name are all the tree holds, and the three descriptions disagree**

G6 I3 · fact · verifier: supported · decides: coordinator

**What the tree holds.** The handover's pipeline steps 2 to 4 (13 lines), one `codex exec` command in WORK_ORDERS.md, the standing verifier order, and the name `run-order.sh`, once, in the review list. Eight variables are named, only in the handover: `BASE`, `SCHEMA`, `EXTRA`, `JOBS`, `TT`, `ROUNDS`, `START_ROUND`, `VERIFY_FIRST`.

**What it does not hold.** Which environment variables are removed "by name". How a worktree and branch are made and cleaned (a leftover branch failed a dispatch of 081). The checkpoint commit and its git identity (069b's failed for want of one). How the verifier is started and what it is shown. The loop over rounds and the continuation from a report. The remote copy over SSH, the following of its log and the fetch back. The allowance for rustup's directory (075). The load test before a dispatch. The Records attribute at least five fixes to the runner, each in a different place.

**Where the descriptions disagree.**
- WORK_ORDERS.md says "One more round at most", after which the admin finishes the work itself. The handover says up to `ROUNDS` (default 2), then finishing orders, and 049a1 ran five rounds.
- verify.md tells the verifier that the worker's changes "are uncommitted". The handover says the runner commits them as a checkpoint.
- The command in WORK_ORDERS.md sets `CARGO_NET_OFFLINE=true`, `--add-dir "$HOME/.cargo"` and `model_reasoning_effort="high"`. The handover's command names none of the three and turns the network on for `net` orders.

A reader cannot tell which is the command as run.

**Model and version.** No Record from 037 to 150, and neither pipeline description, names the model or the client version. With `--ignore-user-config` the model is whatever the client defaulted to that day.

**Private details.** Yes as the scripts are (paths of the checkout and the worktrees, the SSH host, user names). None is needed in the text.

**Loss.** The runners sit beside the private note on one machine. If that copy goes, every order stops until they are rebuilt from the 13 lines, and the lessons above are learned again.

**Evidence.** docs/implementation/HANDOVER.md:55-67 (steps 2 to 4), :53-54 ("the runners read order files from `origin/main`"), :59 ("It checkpoints the worker's output as a local commit"), :61-63, :68-70, :211, :300-301, :305. docs/implementation/WORK_ORDERS.md:50-56 (the command), :83-85 ("One more round at most"). docs/work-orders/verify.md:3-4 ("its changes are uncommitted: `git diff` and `git status` show them"). docs/proposals/first-build/to-review-with-the-owner.md:65 ("as `run-order.sh` does"). docs/work-orders/069b-spec-plan-apply-ruling-76.md:85-87 ("The runner's checkpoint commit failed there for want of a git identity"); 075-spec-own-tools-reach.md:638-639 ("The runner now lets Windows workers write rustup's directory"); 079-shell-https-address.md:355 ("the run script exited 127"). `grep -rnE 'START_ROUND|VERIFY_FIRST|ROUNDS=|EXTRA=|SCHEMA=' docs --include='*.md'` finds only HANDOVER.md. `git ls-files | grep -iE '\.sh$|runner|dispatch'` finds no runner script.

**Waits.** I2 part 1. The model and version fields also serve the first pass's open question on who verifies and with which model.

**Verifier.** supported. HANDOVER.md:55-67 is 13 lines. The runner's variables appear only in the handover (`git grep -nw`). `git ls-files` tracks no runner script, and `run-order.sh` appears once (docs/proposals/first-build/to-review-with-the-owner.md:65). WORK_ORDERS.md:84-85 says 'One more round at most'. verify.md:3-4 says the changes are 'uncommitted'. 069b's Record confirms that practice differs: the checkpoint commit failed and 'the verifier reviewed the staged changes against the base all the same' (069b:85-87). `git grep` finds no worker's model or client version in a Record from 037 to 150; the only versions named are the clients the admin checked for 050a (050a:84).

**Verifier's counterargument.** Some of the 'disagreements' are summaries, not contradictions. ROUNDS defaulting to 2 equals 'one more round at most'. The handover's command leaves flags out rather than contradicting them. WORK_ORDERS.md:66-68 also turns the network on for orders that bind loopback ports. High effort is confirmed for the runner at to-review-with-the-owner.md:65.

**Verifier's corrections.** There are three real contradictions. First, what follows round 2: WORK_ORDERS.md:84-85 has the admin finish the work, while the handover stacks finishing orders and raises ROUNDS (HANDOVER.md:63, :68-70; 049a1 ran five rounds, :211). Second, the verifier is told the changes are uncommitted (verify.md:3-4), while the runner commits them as a checkpoint (HANDOVER.md:59). Third, `net` orders fetch crates (HANDOVER.md:57-58) against CARGO_NET_OFFLINE (WORK_ORDERS.md:50, :65-66). 'At least five fixes' should read three in the Records, plus 079's exit 127 and 081's two failures marked (log).


## FR-528

**A78: the plan has no file, the engine's own records of order 042 are in a store due to be wiped, and an existing command could export them**

G6 I4 · fact · verifier: partly · decides: coordinator

The first pass established that the harness, the `.v2` Agent packages, the Fleet's documents and every dogfood measurement are outside the reviewed tree (P06 I12, supported). Four additions:

- **The plan.** Ruling 74's record opens "The coordinator's plan for A78 under ruling 70 found two things for the owner", and the handover speaks of "the plan's follow-ups". No file holds that plan. A ruling's record cites reasoning a reader cannot open.
- **The engine's records.** The engine kept a record of each of 042's eight turns in the dogfood store. That is Conduit's own account of who did what. The handover plans to re-pin that engine and, after order 050, to initialize the store again. Unless the records are exported first, the only Conduit-native evidence of the first order run through Conduit is destroyed by the plan itself.
- **The means to export exists.** `conduit records --store &lt;dir>` writes the records as ordered JSON lines, with `serve` stopped. By the engine's own rule no credential is in a record.
- **Of the eight register rows whose state is done or in progress, A78 alone cites no file of results.** A77 cites the work-order record; the other six link a dated results file under `experiments/`.

**Private details.** The harness and the Fleet's documents carry the slots' paths: placeholders fix that. The store holds logins and must never be committed. The export may carry directory paths and the text of orders and reports: scrub the paths; the rest is already public in the order files. The `.v2` packages are safe: their three branches are pushed.

**Least.** One experiment directory with the method as run, a dated results file, the Fleet's documents with placeholder paths, and the scrubbed export. If the export cannot be made, the results file says so in one line.

**Evidence.** docs/proposals/ruling-74/README.md:5-6; docs/implementation/HANDOVER.md:155 ("The plan's follow-ups"), :153-154 ("still pinned at `38c0b4f`, before 070"), :157-158 ("After 050, the store is initialized again"). docs/work-orders/042-build-tombstones-and-inventory.md:49-51 ("four worker turns and four verifier turns ... the real store, the dogfood harness"). engine/README.md:186-187, :200-201 ("Records exports ordered CloudEvents as JSON lines ... stop `serve` before exporting records"), :132 ("Keep credentials out of Decisions/records"). spec/conventions/records.yaml:13-23 (REC-001). ROADMAP.md:59-74: states parsed by /usr/bin/python3; done or in progress are A70, A71, A4, A8, A22, A77, A78, A79, and A78's row links no file. `git rev-list --count 38c0b4f..HEAD` = 57. `git branch -r | grep -E 'wo/(050a|076|078)'` lists three branches.

**Waits.** The export must precede the re-pin of the dogfood engine and order 050's new store. I7 fixes the measure before order 046.

**Verifier.** partly. docs/proposals/ruling-74/README.md:5 cites 'The coordinator's plan for A78'. No file holds that plan; WORK_ORDERS.md:135-150 gives the design, not the plan or its follow-ups. The plan to initialize the store again is at HANDOVER.md:153-158. `records` exists at the pin (engine/README.md at 38c0b4f: lines 187 and 200), and `git rev-list --count 38c0b4f..HEAD` = 57. Register rows that are done or in progress: A70, A71, A4, A8, A22, A77, A78 and A79; A78's cell links nothing. The three `.v2` branches are pushed.

**Verifier's counterargument.** The records are destroyed by 050's new store, not by the re-pin (engine/CONFORMANCE.md:18: format version 1 unchanged). 'No credential in a record' rests on a note to implementers (engine/README.md:132) and on records.yaml:84, which keeps no secret value in run.v1's record. The inputs a Type marks for recording are kept, though (spec/conventions/records.yaml:50-53), so the export can carry order text and slot paths and needs the secret scan as well as a path scrub. 'The .v2 packages are safe' holds for the code. But 050a, 076 and 078 have no Record on any branch: each branch holds only wip checkpoint commits (bb75e294, 228b5dea, 5f97ca38). So 042 ran on packages whose rounds and verifier verdicts are recorded nowhere.

**Verifier's corrections.** 'The other six link a dated results file': five do; A22 links experiments/005-macos-keychain/README.md. 'Eight turns': eight turns completed, and the failed round-3 turn was run again (042:49-54), so the store holds at least nine turn invocations.


## FR-529

**Reports of workers and verifiers were never committed, so the rule that raw records leave the tree with a link has nothing to link to; 071 and 080 resume from files that are outside**

G6 I5 · fact · verifier: partly · decides: coordinator

- **What they are.** Each worker answers in a JSON report by the order's schema (what changed, its own choices, each check's last line). Each verifier answers in `verify.schema.json`: a verdict and findings with severity, file, line, problem, evidence and fix. The dispatch also writes an event stream.
- **Where they live.** In files the dispatch command names, outside the tree; the lane's are in "its logs" in the private note.
- **What the tree holds instead.** Prose Records: 72 of the 80 order files have one. Eight do not: 049a1, 050a, 071, 076, 078, 080, 144, 147. For 144 and 049a1 a Record exists only on the unmerged branch. For 071 and 080 there is none on any branch. Their branches hold two checkpoint commits each and no report.
- **What waits on them now.** The handover's next step for both 071 and 080 is "Round 3 from round 2's verifier report". The handover gives each must in one clause, which is enough to write a new report by hand, as it says the admin may. Shoulds, nits and the verifier's quoted evidence are not there.
- **The handover points at a Record the tree lacks.** It says 144's two musts are "named in its Record", and PROVIDERS.md says the same. On `main` the order file has no Record.
- **What cannot be measured without them.** A77 asks whether the work is as good and whether the admin spends less. The Records hold no rejected findings, no severity for about half the older orders, and nothing a reviewer before dispatch found (P01 I10, supported). The admin's time "was not recorded"; the first pass's verifier notes that the coordinator's spend exists only in session transcripts.

**Private details.** A report may quote a worktree's path: a scrub of the prefix is enough, and the path checker catches a home path. Event streams may hold command output and must stay out.

**Least.** For 071 and 080 now: a Record on each branch in the form 144's branch uses, with the verifier's findings verbatim. From the next order: the runner commits the final worker report and each verifier report beside the Record, to be removed when A80 carries the Record into the table, linked at their last version. That is ruling 58 point 5's rule applied to orders as it already is to experiments.

**Evidence.** docs/implementation/WORK_ORDERS.md:54-55 (`--output-schema &lt;order>.schema.json -o &lt;report>.json --json`, `&lt;events>.jsonl`), :156-157 ("The admin's own time was not recorded"). docs/work-orders/verify.schema.json (properties verdict, findings; finding fields severity, decision, file, line, problem, evidence, fix). `ls docs/work-orders/[0-9]*.md | wc -l` = 80; `grep -l '^## Record'` = 72; `grep -L` lists the eight. `git show origin/wo/144-s4-srv2-hook:docs/work-orders/144-s4-srv2-hook.md | grep -n '^## Record'` = 363; the same for origin/wo/049a1-build-enrollment-core = 649; for the branches of 071 and 080 no match. `git log --format='%h %s' HEAD..origin/wo/071-conformance-manifest-pieces` = two wip commits. docs/implementation/HANDOVER.md:212-213 ("Round 3 from round 2's verifier report"), :214 ("with two musts named in its Record"), :64-67 ("The admin may write that report itself"), :99-101. docs/implementation/PROVIDERS.md:44-45. ROADMAP.md:70. P01 I9, I10 and I13 with their verifier's corrections.

**Waits.** Rows 4 and 5 of the handover's queue (080 and 071). A77's done-when. The first pass's open question on who verifies.

**Verifier.** partly. Recounted: 80 order files, 72 with '## Record', and the eight without as listed. On the branches, 144's Record is at line 363 and 049a1's at 649. The branches of 071 and 080 hold two wip commits each and no Record (`git log HEAD..origin/wo/...`). The fields of verify.schema.json are as stated. HANDOVER.md:64-67 and :212-213; WORK_ORDERS.md:54-55 and :156-157.

**Verifier's counterargument.** 'Never committed' overstates it: increment 4's fresh Codex author's report and log are committed (docs/proposals/first-build/increment-4/fresh-author-run/report.json and log.txt). That is a precedent for I2, though not a dispatch report. 'The handover points at a Record the tree lacks' also misleads: HANDOVER.md:214 and docs/implementation/PROVIDERS.md:44-48 name the pushed branch in the same clause, and the Record is on that branch. It is absent from main, not lost. For 071 and 080 the loss is bounded: the handover gives each must (:212-213) and lets the admin write the report itself (:64-67).

**Verifier's corrections.** Add 050a, 076 and 078 (and 147, never dispatched) to the orders with no Record on any branch, as the first pass's verifier of P01 I13 already noted. 144's branch Record paraphrases its verifier's findings; it does not quote them.


## FR-530

**Were the keys that an agent printed on 2026-11-11 replaced?**

G6 I6 · question-for-owner · verifier: partly · decides: owner

The handover says: "On 2026-11-11 an agent printed a session's environment, which held keys". It gives the rule that followed (no environment dumps; probes start under `env -i` with fake keys). Nothing in the tree says what became of the keys.

Options:
- **(a)** They were all replaced. The coordinator records one dated line: which kinds of key, by label only, and when they were replaced.
- **(b)** Not yet. Replace them now, then record as in (a).
- **(c)** They need no replacing, for a reason you give (test keys, or already expired). Recorded the same way.

Recommendation: (b) unless (a) is already true. Replacing a key costs minutes, and until it is done the incident is open whatever the new rule says.

What it changes: one dated line beside the standing rule (in the handover). No key, name or value enters the repository. Nothing in the build waits on it.

**Evidence.** docs/implementation/HANDOVER.md:282-284 ("On 2026-11-11 an agent printed a session's environment, which held keys (log). Every brief that may run a command says: never run `env`, `printenv`, `set` or `export -p`"). `grep -rn -i 'printed a session|held keys|environment dump'` over docs, IN_FORCE.md, ROADMAP.md, AGENTS.md and CONTRIBUTING.md finds only that passage. AGENTS.md:120-121 (workers never read, print or copy a credential): the coordinator cannot check this itself.

**Waits.** Only the incident's closing line. It is cheap to answer with the next batch.

**Verifier.** partly. Only HANDOVER.md:282-284 mentions the incident. `git grep` for env -i, printenv and similar finds otherwise only the orders' preambles. AGENTS.md:111-112 forbids workers to read credentials.

**Verifier's counterargument.** The fact is marked '(log)', so the coordinator's private note holds the incident and may already say whether the keys were replaced: read it first. Replacing keys is an action on the owner's own credentials, not a ruling. Under docs/guides/full-review.md:209 the numbered answers become a ruling, so send this as a direct request outside the batch. Nothing of that session is ever committed, which ties this to the secret scan in I2.

**Verifier's corrections.** The owner acts, rightly. Ask only if the private note is silent. Close the incident with one dated line naming key kinds by label.


## FR-531

**A78: the evidence of an order run through Conduit is the engine's own records, committed, and a fresh reader must rebuild the run from them**

G6 I7 · experiment-change · verifier: partly · decides: coordinator

**Row.** A78, ROADMAP.md:71. This is one clause for the single merged row the first pass's critic asks for (four other changes to A78 already exist); it is not a fifth version. It leaves the owner's done-when as it is.

**Claim it tests.** Preservation test 7, "Everything an agent does through Conduit is attributable", and REC-001, in real use: for an order run through Conduit, the engine's records alone say who did what and how each turn ended. Stage 1 tested how a turn is carried (P06 I1); this makes the same runs test something Conduit is for, at the cost of one command.

**Fixed before order 046 runs.**
- Each order run through Conduit gets a directory under A78's experiment directory. It holds the scrubbed output of `conduit records` for the order's interval, the harness's log lines, and the reports.
- An order does not count toward A78's five unless that directory exists.

**Pass condition, per order.** A fresh reader, given only the export, lists every turn: the Agent, the caller, start, end, outcome, and for each turn that ended unknown, the later request that settled or replaced it. The order passes if that list equals the harness's log turn for turn. A turn or an outcome the records lack is a finding of the first build against REC-001, or against the promise that should have recorded it, in the open increment's log.

**Measures.**
- Turns in the harness's log; turns found in the records.
- Fields the reader could not get from the records, each named.
- Turns that ended unknown, and how each was settled: reconcile, a repeat with the same key, or a new request. Order 042's unknown turn "was run again" and nothing records which.
- Redactions made before committing, counted by class (path, name, other). This measures how much private detail Conduit's own records carry, which decides whether an export can ever be shared.

**For order 042, retroactively.** Export the dogfood store with the pinned engine before the re-pin and report the same measures as an observation, not a pass or a fail: the measure was not fixed before that run (ruling 45).

**Cost.** One command and a scrub per order. **Buys.** A78's evidence becomes checkable from the tree and comes from Conduit, not from a harness outside it.

**Evidence.** ROADMAP.md:71 (A78's row: "Five orders run through Conduit, each friction logged as a finding of the first build"), :96-97 ("Each experiment states, before it runs, what it must show and how each measure is counted (ruling 45)"). docs/AUDIT.md:289-310 (preservation test 7). spec/conventions/records.yaml:13-23 (REC-001: "Every request can be found with how it ended"). engine/README.md:186-187, :200-201. docs/work-orders/042-build-tombstones-and-inventory.md:52-54 ("One turn of round 3 failed ... and was run again"). P06 I1 and I12; P06 I13's verifier ("No reconciliation is recorded for 042's unknown turn"). The approach critic's register item (A78 has four differently worded changes).

**Waits.** Order 046 through Conduit; the re-pin of the dogfood engine (export first).

**Verifier.** partly. ROADMAP.md:71 (A78's row) and :96-97 (ruling 45); docs/AUDIT.md:308 (preservation test 7); REC-001 at spec/conventions/records.yaml:13-23; engine/README.md:186-187 and :200-201; 042:52-54 ('was run again').

**Verifier's counterargument.** (1) 'The records alone say who did what' overclaims. An Agent's own tools are neither enforced nor mediated (CTR-014, PRV-006 and ACC-027, as P06's verifier notes), so what the Agent did inside its slot is no Conduit request and is not in the records. In stage 1 every caller is root, so attribution among callers is trivial. The records can say which turns were requested, by whom, and how each ended, and no more. (2) 'The later request that settled or replaced it' is not something REC-001 promises. A new request under a new key, as 042's was, carries no link by design. A gap there measures the harness's way of recovering; it is not a finding against REC-001. Keep the two pass conditions apart. (3) The item says it leaves the done-when as it is, yet 'does not count unless the directory exists' changes what counts. The coordinator may do that under ruling 45 but should say so. (4) P06 I7 already measures unknown outcomes; merge both into the one A78 row.

**Verifier's corrections.** Pass condition: every turn the harness's log lists is found in the export with its caller, Agent, start, end and outcome. Report the linkage of replacement requests as a measure, not as a pass condition.


## FR-532

**The eleven '(log)' marks: two are already recorded elsewhere, one is checkable from the code, and none needs a private detail**

G6 I8 · fact · verifier: partly · decides: coordinator

The handover defines the mark as "a fact found only in the coordinator's private notes ... still to be recorded". There are eleven. Two head whole lists (two bullets and four).

| Line | Fact | Already in the tree elsewhere | Least |
| --- | --- | --- | --- |
| :84 | Windows' sandbox could not use the pinned toolchain for 075 and timed out 072's S1 HTTP test | Yes: the Records of 075 and 072 say it | Drop the mark |
| :130 | What worked: the smoke test's six checks; 042's turns and times | Only 042's Record, in outline | The dated results file of A78's directory |
| :141 | What failed or cost: the network loss, the write in `.git`, a restart per conversation, children that escaped the stop | No. Increment 4's log has 20 rows and none of these | Four rows in increment 4's log (drafted in the first pass, P06 I8 to I11) |
| :183 | Ask the owner before part 2 of ruling 77's setup | The guide says only that part 2 can run now | One clause in the guide |
| :185 | WSL's 15 GB; five orders reached 12 GB; test binaries of 5 to 8 GB; out of memory once | Ruling 77 says only that WSL keeps its default | One sentence in the guide |
| :216 | The first-start report maps some Rust architecture names to the wrong Go names | Checkable from the code: the engine maps six names and passes the rest through, while the local-machine provider maps the full set | One row in increment 4's log |
| :267 | 049a1: one live node per machine, acceptances one at a time | Only in the Record on the unmerged branch | One row on the review list |
| :277 | The side branch `s3/m2`, merged after the rerun's pin | Its use shows in orders 079 and 120; the decision has no row | One row on the review list |
| :283 | An agent printed a session's environment, which held keys | No | One dated line after the owner's answer (I6) |
| :301 | The load rules; two test threads under load | In part: 045's Record and finding 4-18 | One sentence in the runners' page |
| :305 | Two of 081's dispatches failed, for a missing `SCHEMA` and a leftover branch | No | One line among the runners' failure modes |

So the marks are not eleven losses. Seven need a row or a sentence that the handover's own words can supply; two are already recorded; the two that matter are the A78 lists, whose evidence is row 2 of I1.

**Evidence.** `grep -n '(log)' docs/implementation/HANDOVER.md`: lines 8 (definition), 84, 130, 141, 183, 185, 216, 267, 277, 283, 301, 305. docs/work-orders/075-spec-own-tools-reach.md:635-639; 072-shell-list-operation.md:295-297. `grep -c '^| 4-' docs/proposals/first-build/log-increment-4.md` = 20. docs/guides/second-machine.md:21-26. docs/proposals/ruling-77/README.md:59 ("WSL keeps its default memory limit"). engine/crates/shell/src/lib.rs:717-725 against packages/provider-local-machine/src/lib.rs:27-40. `git show origin/wo/049a1-build-enrollment-core:docs/work-orders/049a1-build-enrollment-core.md` lines 649-661. docs/work-orders/079-shell-https-address.md:14, :359; 120-plan-providers.md:114. docs/work-orders/045-build-state-directory.md:55; docs/proposals/first-build/log-increment-4.md:29.

**Waits.** I2 part 4. The review-list rows overlap the first pass's item on decisions the owner may strike; add them once.

**Verifier.** partly. Eleven marks, at lines 84, 130, 141, 183, 185, 216, 267, 277, 283, 301 and 305. Line :84 is recorded elsewhere (075:635-639; 072:293-297). Increment 4's log has 20 rows (`grep -c '^| 4-'`). For :216, engine/crates/shell/src/lib.rs:717-725 is checked against packages/provider-local-machine/src/lib.rs:24-41. For :267, the 049a1 branch Record quotes the decision verbatim (lines 651-661). For :301, 045:55.

**Verifier's counterargument.** Line :183 is already recorded in the tree: HANDOVER.md:227-230 lists part 2 among the owner's open items, and docs/guides/full-review.md:34-36 puts it under 'Waiting for the owner'. So three marks are recorded elsewhere, not two. Line :277 has authority, ruling 76's later answer (docs/work-orders/120-plan-providers.md:114; docs/proposals/ruling-76/README.md:48-52), though it still lacks a row on the review list.

**Verifier's corrections.** For :141, the first pass's drafts are P06 I8, I9, I10 and I2, not I8 to I11. On :216, the engine has six match arms over five Rust architecture names.


## FR-533

**Checks and CI: results are cited by pull request and by sentence, never by run; one merge had no CI engine run and the differential run is in no workflow**

G6 I9 · fact · verifier: partly · decides: coordinator

- **No CI run is cited by its ID or address anywhere in the tree.** CI is cited by pull request number (the handover cites 13) or by a phrase such as "passed on 042's run". The runs and their logs are on GitHub. Its default keeps logs for 90 days (general knowledge, not checked here), so the failing runs behind "red at times since #260" are not kept for good.
- **CI was paused for part of 2026-11-11, by a setting.** Two lane Records say so. No workflow file changed between the commit that created CI and order 068, and 103 commits landed on `main` in that interval. The tree does not say when the pause began or ended, or which merges had no CI.
- **077 merged on local evidence**, with CI's engine shards skipped. The evidence is one sentence: the engine's suite and 446 vectors passed on the Mac. 042 merged next; its CI run included the shards (the first pass's verifier corrected P07 I9 on this).
- **The differential build is in no workflow.** It is the check on 070's lazy publication. The handover puts its vector run at about 2.4 hours on the Mac, and each statement that vectors pass "on the normal and differential builds" rests on a run by hand.
- **The first fix asks to "show it stable over three CI runs"** and names no place to show it.
- **Settings on GitHub that the tree relies on and does not describe:** the protection of `main` (ruling 74 point 1) and the variable `CONDUIT_PRIVATE_NAMES` that CI passes to the path checker.

**Private details.** None, except the variable's value.

**Least.** In each Record whose merge rested on a local run: the commit tested, the command, its last line, and the machine by label. In the CI fix's Record: the three runs' addresses. A paragraph in CONTRIBUTING.md naming the settings and variables, without values. The first pass already recommends the nightly differential run and required checks (P07 I9, P01 I8); I add nothing to those.

**Evidence.** `grep -rnoE 'actions/runs/[0-9]+|run [0-9]{9,}'` over *.md, *.yaml, *.yml, *.json = 0 matches. `grep -o '#[0-9]\{3\}' docs/implementation/HANDOVER.md | sort -u | wc -l` = 13. docs/implementation/HANDOVER.md:197-205 ("passed on 042's run"; "show it stable over three CI runs"), :208 ("the differential vector run takes about 2.4 hours on the Mac"), :210 ("Merged (#263, `575a1bcd`) on local evidence"). docs/work-orders/104-mc1-model-checking-harness.md:202 ("CI was paused on 2026-11-11"); 105-mc2-operation-identity.md:286-287. `git log --format='%h %cd %s' -- .github/workflows` = four commits (33eef301, d981fa46, 0bae3d4f, defee9ba); `git log --first-parent 0bae3d4f..defee9ba | wc -l` = 103. `grep -n differential .github/workflows/*.yml` = nothing. .github/workflows/checks.yml:65. IN_FORCE.md:299. P07 I9's verifier.

**Waits.** The CI fix at the head of the handover's queue should record its three runs this way.

**Verifier.** partly. No CI run ID appears anywhere in the tree. HANDOVER.md cites 13 distinct pull-request numbers. 'differential' appears in no workflow; the feature is defined at engine/crates/core/Cargo.toml:28 and engine/crates/conformance/Cargo.toml:22. 104:202 and 105:287 say CI was paused. HANDOVER.md:198-210.

**Verifier's counterargument.** The sharper gap is a ruling's precondition with no record of being met. IN_FORCE.md:299 requires main to be protected before Conduit runs its first order (ruling 74 point 1). No line in the tree records that the protection was applied, or when, before 042 ran (`git grep` for 'protect' finds only the rule). The '90 days' of log retention is general knowledge.

**Verifier's corrections.** 'No workflow file changed between the commit that created CI and order 068' is false: orders 033 (d981fa46) and 034 (0bae3d4f) changed workflows after 33eef301 created CI. The claim holds from 034 to 068: `git log --first-parent 0bae3d4f..defee9ba` gives 103 commits, 068 included. CONDUIT_PRIVATE_NAMES is described at AGENTS.md:153 and checks.yml:60-61. Two lane crates sit outside CI's glob (`git ls-files '*Cargo.toml'`: experiments/013-s2-shared-login/linear-stub and experiments/015-s4-partial-setup/scripted-client), so 'CI green' told nothing about them (first noted by P01's verifier).


## FR-534

**The private-path checker checks three kinds of path; names, addresses and fingerprints are checked only if a variable outside the tree lists them**

G6 I10 · fact · verifier: supported · decides: coordinator

The handover and the brief both say that private details never enter the repository ("no local path, user name, address, host name or account") and that `tools/check_private_paths.py` checks the tracked files.

What the checker has built in is three patterns: a home-directory path, a session scratch path, and the Codex login file. Anything else is checked only against the names in `CONDUIT_PRIVATE_NAMES`. CI takes that from a repository variable, and a local run takes it from the caller's environment. The tree cannot say whether the variable is set or what it holds, and a third coordinator would not know what to put in it.

So a tailnet name, a tailnet address or a host key's fingerprint in a committed file would pass the check as built. That matters now: I2 asks for redacted runner scripts, a harness and a records export to be committed, the kind of file most likely to carry one.

The tree is clean today. A search of the tracked files for names ending `.ts.net`, for addresses in the 100.64/10 range and for `SHA256:` fingerprints finds nothing private. The one match of the fingerprint pattern is a hexadecimal digest in a conformance fixture, which a new pattern must not flag.

**Least.** Three more built-in patterns with a unit test each: a name ending `.ts.net`; an IPv4 address in the 100.64/10 range; `SHA256:` followed by exactly 43 base64 characters that are not all hexadecimal. One sentence in CONTRIBUTING.md saying the variable exists and what kind of name belongs in it. It is the Python checker, so it is inside AGENTS.md's exception.

**Evidence.** tools/check_private_paths.py:16-23 (the three patterns), :32-33 (`CONDUIT_PRIVATE_NAMES` read from the environment). .github/workflows/checks.yml:64-66 (`CONDUIT_PRIVATE_NAMES: ${{ vars.CONDUIT_PRIVATE_NAMES }}`). docs/implementation/HANDOVER.md:287-290; docs/guides/full-review.md:222-224. `git grep -n -I -E '[a-z0-9-]+\.ts\.net'` = 0; `git grep -n -I -E '\b100\.(6[4-9]|[7-9]\[0-9]|1[01]\[0-9]|12[0-7])\.[0-9]{1,3}\.[0-9]{1,3}\b'` = 0; `git grep -n -I -E 'SHA256:[A-Za-z0-9+/]{43}'` = 1, conformance/language/archives/digest-format/install.json:5.

**Waits.** Should merge before, or with, I2's parts 1 and 2.

**Verifier.** supported. tools/check_private_paths.py:16-23 (three patterns) and :32-33 (the variable); checks.yml:64-66. My `git grep` counts: 0 names ending .ts.net, 0 addresses in the 100.64/10 range, and 1 `SHA256:` match, a hexadecimal digest at conformance/language/archives/digest-format/install.json:5.

**Verifier's counterargument.** AGENTS.md:153 already says the variable exists. HOME_PATH (:16) matches only forward-slash /Users and /home, so a Windows path such as `C:\Users\&lt;name>` passes. Material for the second machine, which uses PowerShell paths (docs/guides/second-machine.md:59), is exactly what I2 would commit. No pattern can find a key.

**Verifier's corrections.** Add a Windows home-path pattern as a fourth, and pair the checker with a secret scanner.


## FR-535

**This review's first pass is itself in session scratch space, and 17 of its 40 files would fail the path check as they are**

G6 I11 · fact · verifier: supported · decides: coordinator

The first pass (263 findings and 182 items, each with a verifier's verdict, two indexes and two critics' reports) is 40 files and 2.47 MB in a session's scratch directory. The handover's own standing rule is "Keep work out of session scratch space (order 019's run was lost)". The brief asks for a dated record under `docs/proposals/full-review/`, with findings in a table.

Seventeen of the 40 files match the path checker's patterns, 31 times in all: 27 are this machine's home path, mostly in each reader's coverage line, and 4 are the scratch path. A mechanical replacement of the tree's prefix clears them.

**Least.** Now, before the second pass ends: commit the two index files (one line per finding or item) and the two critics' reports on the review's branch, scrubbed. With the record: the per-reader files, scrubbed, to leave the tree at the next condensation with a link to their last version, as the re-audit rounds did.

**Evidence.** `ls approach language | wc -l` on the first pass's files = 40; `cat approach/* language/* | wc -c` = 2,471,222. `grep -lE` of the checker's three patterns = 17 files; `grep -oE ... | wc -l` = 31 (27 home path, 4 scratch path; no value printed). docs/implementation/HANDOVER.md:306-307. docs/implementation/WORK_ORDERS.md:122-123 ("Order 019's first run was lost before it was committed"). docs/guides/full-review.md:197-200.

**Waits.** The review's record. It costs minutes and should not wait for I2.

**Verifier.** supported. Recounted with the checker's own expressions in /usr/bin/python3: 40 files and 2,471,222 bytes; 17 files with matches, 31 in all (27 home paths, 4 scratch paths, 0 Codex login files). The rule is at HANDOVER.md:306-307 and WORK_ORDERS.md:122-123; the record the brief asks for at docs/guides/full-review.md:197-200.

**Verifier's counterargument.** The item understates the urgency. The first pass sits in the system's temporary directory, which macOS empties at restart and prunes over time (general knowledge, not checked). That deadline is in no one's control and comes sooner than the store's. Committing 2.47 MB of raw verdicts strains the relevance standard (rulings 57, 58). The proposed split is the right size: the indexes and critics now, and the per-reader files later, linked at their last version.


## FR-536

**The fault is a repeated one, so the fix is a check at merge and not another rule**

G6 I12 · assessment · verifier: partly · decides: coordinator

The rule already exists in four places. AGENTS.md: evidence goes in its own directory under `experiments/`, with dated results. CONTRIBUTING.md: dated results and the method as run remain as evidence. WORK_ORDERS.md's lessons and the handover's standing rules: keep durable work out of session scratch space.

The tree also records the fault at least four times before the handover:
- order 019's first run, lost with a session's scratch space;
- the short form's critic: "The N1-N4 working papers and the skeptics' scans are not in the repository";
- the condensation's critic: the skeptic items "are not written anywhere the cold reader can see", and the admin's memory outside the repository cites paths about to be removed;
- the metrics proposal, accepted by rulings 68 and 69, whose own source list is headed "the coordinator's session notes, not in the repository" and names a benchmark's source and output in a scratchpad.

Then at the handover the runners, the dogfood harness and all of A78's results were outside.

My judgment: this happens because the pipeline's merge gate checks code and documents and never asks where a cited number came from. A coordinator under load records the result and keeps the means where it is working. Stating the rule again will not change that. Three small checks would:
1. In `tools/check_roadmap.py`: a register row whose state begins "done" or "in progress" must link a file that exists. Today that flags A78 alone.
2. In the verifier's standing order: a number or a result in a Record or a results file with no path beside it is a should.
3. For a handover: the outgoing coordinator's last act is the fresh-reader test of I2 (dispatch from the tree alone).

What this costs is small: one function in a Python checker and two sentences. What it does not need is a perfect record. Rulings 57 and 58 set relevance as the standard, and nothing here asks for event streams, transcripts or every report ever written.

**Evidence.** AGENTS.md:79-80; CONTRIBUTING.md:109-111; docs/implementation/WORK_ORDERS.md:122-123, :695-700; docs/implementation/HANDOVER.md:306-307. docs/proposals/language-shape/short-form/critic.md:103. docs/proposals/condensation/critic.md:117-118. docs/proposals/metrics/general.md:1366-1368 ("MEASUREMENTS AND SAVED SOURCES (the coordinator's session notes, not in the repository)"); docs/proposals/metrics/README.md:3-5 (accepted by rulings 68 and 69). ROADMAP.md:70-71 (A77 links the record; A78 links nothing). IN_FORCE.md topic 15 ("The standard for records is relevance, not a perfect record", rulings 57 and 58).

**Waits.** Check 1 can ride with I10's change to the tools; checks 2 and 3 are sentences in verify.md and in the handover.

**Verifier.** partly. The rule is stated at AGENTS.md:79-80, CONTRIBUTING.md:109-111, WORK_ORDERS.md:122-123 and HANDOVER.md:306-307, and a fifth time at experiments/README.md:80-83. The earlier losses are recorded at WORK_ORDERS.md:699-700, docs/proposals/language-shape/short-form/critic.md:103, docs/proposals/condensation/critic.md:117-118 and docs/proposals/metrics/general.md:1366-1368. Check 1 flags A78 alone (parsed register).

**Verifier's counterargument.** The runners are outside by a stated choice (HANDOVER.md:8-10; docs/guides/full-review.md:46-47), not by a lapse. The cause there is private values inside the scripts, together with the Rust rule. I2 addresses it, and the choice belongs on the review list for the owner to strike. Check 1 can be met by linking the order file: require a dated results file under experiments/ for 'done'. Check 2 would turn most Records' test counts and timings into shoulds and add rounds, which P01 I1 finds do not converge; limit it to results files and claims in the register. Check 3 fails in exactly the case that occurred, a handover forced by a usage limit (HANDOVER.md:4-5). Make it continuous instead: the method-as-run page kept current at each merge that changes practice.


## FR-537

**What is working: model checking, the baseline, the lane's Records and the second-machine guide can be reproduced or checked from the tree and its history**

G6 I13 · assessment · verifier: supported · decides: none

- **Model checking.** The harness is in the tree with a pinned toolchain and lock file. No test is ignored or gated, and CI runs the crate's tests on every pull request, so every verdict is re-derived continuously. The outputs are in the README as dated tables and first traces, each beside the table registered before the run. Three small gaps: a dated table names no commit of the model, and several models were revised after their first tables; the state counts and times are copied from printed output, not asserted by a test; and the experiments index still says of 012 "No Conduit promise is checked yet" in two places. None loses anything.
- **The S1 baseline.** Its results file is in the tree. Its 33 raw files left under ruling 58 point 5 and are in `main`'s history at a commit the results link 36 times. That history holds no home or scratch path in those files, and `main` cannot be force-pushed. The bar's fixed input is safe. One sentence giving the offline command would spare a reader the dependence on GitHub's pages.
- **History-pinned links in general.** The tree's links pin eight distinct commits; seven are ancestors of the reviewed commit. The eighth is row 14 of I1.
- **The lane's Records.** They give seconds per round, the base commit, the machine, and each must by round. They are the model the admin's Records should follow, as the first pass also found.
- **Checkpointed work.** Every unfinished branch is pushed. 144's branch carries a Record written at the checkpoint that says where it stands and what the finishing order must do; that is exactly what 071 and 080 lack.
- **The second machine.** The guide and the SSH runtime provider's README together give the setup step by step with the reason for each setting, so the setup can be repeated on another computer from the tree. What is missing is only a dated note of which steps were done and checked, and the capacity facts.
- **The owner's words.** Confirmations of coordinator decisions are quoted in the rulings' later answers, not left in chat.

**Evidence.** experiments/012-model-checking/README.md:21-23 (CI runs `cargo test --release` in each experiment), :25-35 (Run it), :108-124 (dated table); `grep -rn '#\[ignore' experiments/012-model-checking/src | wc -l` = 0; headings at :1279 ("MC4 earlier-round results") and :1819 ("MC4 revised results"). experiments/README.md:31, :57 against experiments/012-model-checking/README.md:3. `grep -c f1cb473 experiments/007-s1-baseline/results-2026-11-09.md` = 36; checker patterns over the 33 files at f1cb4734 = 0 matches; IN_FORCE.md:299. Eight pinned commits by `grep -rnoh 'github.com/shrav-k/conduit/(blob|tree|commit)/&lt;hash>' | sort -u`, each tested with `git merge-base --is-ancestor`. docs/work-orders/137-ssh-r-runtime.md:231-236; 150-ssh-r-one-drainer.md:95-108. docs/guides/second-machine.md:55-112; packages/provider-ssh-runtime/README.md:292-312. docs/proposals/first-build/to-review-with-the-owner.md:75-78.

**Verifier.** supported. experiments/012-model-checking/README.md:21-23. CI's glob covers experiments/*/Cargo.toml (checks.yml:94). The harness has 0 `#[ignore]`, and the README has headings at :1279 and :1819. Stale lines at experiments/README.md:31 and :57. The baseline has 36 links and 0 checker matches across its 33 files. Of eight pinned commits, seven are ancestors. 144's branch Record. docs/guides/second-machine.md:55-112 and packages/provider-ssh-runtime/README.md:292-312.

**Verifier's counterargument.** 'Re-derived continuously' holds only while CI runs: it was paused on 2026-11-11 and has been red at times since #260 (HANDOVER.md:198-205).


## FR-538

**Smaller items: a link in spec/README.md that only a pull-request ref keeps alive, numbers whose sources were in scratch space, and the second machine's unrecorded state**

G6 I14 · fact · verifier: supported · decides: coordinator

- **spec/README.md:279** links the "last version" of `promises/graphs.yaml` at commit 0a9a3232. That commit is not in `main`'s history; only pull request 7's head ref reaches it. Its file differs from the last version on `main` by 9 added and 6 removed lines. A clone of `main` alone does not have it. One other document links the same commit. Least: a tag on the commit, the ordinary way to keep a commit reachable. Changing the link's target is not a substitute, because the two versions differ.
- **The metrics proposal's cost numbers** (a whole-state rewrite of about 2.5 seconds at 100,000 records; invocation entries of 129.6 KB) cite a benchmark source and a probe store in a scratchpad. If they are gone, mark the numbers "source not kept" where they are cited. The first pass's store-format recommendation (P07 I8) should not lean on them unmeasured.
- **The second machine.** The guide says of ruling 77's setup that "steps 1 to 5 were done, but for three parts" and asks the owner to "Tell the coordinator where a step differs"; the results of each step's check "stay outside the repository". No dated line says which checks passed. The address, name and fingerprint must stay out; a table of step, done, checked, by date, carries none of them.
- **The lane coordinator's own context** is gone with the session that held it, and its logs are in the private note. I found nothing of the lane's that a loss of those logs would take: its decisions are logged in its Records and its queue is in PROVIDERS.md.
- **The adopted standards.** The language critic notes that no first-pass reader could open the protocol revision the interface promises cite. That text is public and outside this area; I note only that the tree holds no pinned copy or digest of it.

**Evidence.** spec/README.md:279; docs/proposals/graphs-and-schemas/README.md:6. `git merge-base --is-ancestor 0a9a3232 HEAD` fails; `git branch -r --contains 0a9a3232` = origin/pull/7; `git diff --stat 0a9a3232:spec/promises/graphs.yaml c4d7d332^:spec/promises/graphs.yaml` = 9 insertions, 6 deletions. docs/proposals/metrics/general.md:114, :386, :403, :1366-1368. docs/guides/second-machine.md:21-26, :114-118. docs/implementation/HANDOVER.md:99-101; docs/implementation/PROVIDERS.md:36-66; docs/work-orders/104-mc1-model-checking-harness.md:197. The language critic's first missing area; spec/interface/implementation.yaml:6, :62.

**Waits.** I2 parts 5 and 7.

**Verifier.** supported. spec/README.md:279 and docs/proposals/graphs-and-schemas/README.md:6 link 0a9a3232. `git merge-base --is-ancestor` fails, and only origin/pull/7 contains the commit. `git diff --stat 0a9a3232:spec/promises/graphs.yaml c4d7d332^:spec/promises/graphs.yaml` gives 9 insertions and 6 deletions; c4d7d332 (#7) removed the file. The metrics proposal's numbers are at docs/proposals/metrics/general.md:114, :386, :403 and :1366-1368.

**Verifier's counterargument.** GitHub keeps pull-request refs while the repository exists, so the links work on GitHub; the tag matters for clones and mirrors. The second machine already has a dated line on which steps were done (docs/guides/second-machine.md:21-25); only the results of each step's check are kept outside (:114-117).


## What the verifier says the analyst missed

- **050a, 076 and 078 have no Record anywhere, and 042 ran on them** Each branch holds only wip checkpoint commits (`git log HEAD..origin/wo/050a-agents-v2-own-tools`, 076 and 078: bb75e294, 228b5dea, 5f97ca38), and none has a '## Record'. These are the `.v2` packages that ran order 042 (HANDOVER.md:126-128), so 042 ran on packages whose rounds and verifier verdicts are recorded nowhere. HANDOVER.md:223 lists 050a among the Records owed but not 076 or 078. The coordinator decides: write a branch Record for each, in the form 144's branch uses.
- **Nothing records that ruling 74 point 1 was carried out before Conduit ran its first order** IN_FORCE.md:299 requires `main` to be protected before Conduit runs its first work order. No line in the tree says when the protection was applied, or with which rules (`git grep -i protect` over docs/proposals/ruling-74, docs/work-orders and docs/implementation). Least: one dated line in ruling 74's record naming the settings, without account details. The coordinator records it, after checking with the owner if the coordinator did not apply it itself.
- **Committing reports, exports and scripts needs a scan for secrets, not only for paths** I2 and I7 commit worker and verifier reports, a records export and runner scripts. The checker detects only paths and the names in a variable (tools/check_private_paths.py:16-23, :32-33), and HANDOVER.md:282-283 records that a session printed keys. The existing paradigm is gitleaks or trufflehog as a CI step, or GitHub's secret scanning with push protection. A CI step adds no non-Rust code to the tree. The coordinator decides.
- **The path checker misses Windows-form home paths** HOME_PATH matches only `/(Users|home)/&lt;name>` (tools/check_private_paths.py:16). `C:\Users\&lt;name>` and expanded `%UserProfile%` paths, which the second machine's runner and notes may carry (docs/guides/second-machine.md:59), pass. Add the pattern to I10's three.
- **The first pass has an uncontrolled deadline** The first pass's 40 files are in the system's temporary directory. macOS empties that directory at restart (general knowledge, not checked), so they can vanish before the store does. Committing them (I11) is the most time-critical step in the area, ahead of the store's export.
- **The per-round checkpoints of orders built on Windows are not on GitHub** In the review checkout, `git for-each-ref refs/remotes/win` lists 27 refs fetched from the second machine's clone. Nine have no origin branch (111, 113, 132, 137, 140, 145, 146, 149, 150), and fifteen are 1 to 3 wip commits ahead of origin. Example: 0cb037d0, 070's partial output when it was stopped on 2026-11-11. The merged squashes keep the outcomes; the round-by-round diffs that A77 would read exist only on the two machines. Low cost each. Push them, or say in the method-as-run page that they are discarded.
- **A conflicting count that a results line would settle** 042's Record says 491 vectors passed (042-build-tombstones-and-inventory.md:59); the handover says 494 (HANDOVER.md:209). The tree at 042's merge has 494 (`git ls-tree -r --name-only 57813f06 conformance | grep -c '/case.yaml$'` = 494; at 077's merge, 575a1bcd, 446). This supports I1 row 7: one result line per merge naming the commit and the command.

## The verifier's notes

The analyst's inventory is accurate on the counts that carry it. These all reproduce at 2eb8f258: the 11 '(log)' marks, 80/72/8 Records, 40 files and 31 private-path matches in the first pass, 33 baseline files linked 36 times, 8 pinned link commits with one unreachable, zero CI run IDs, the runner's variables found only in the handover, A78 alone of the eight active register rows with no linked file, and the checker's three built-in patterns. The diagnosis stands: the means of the work (runners, harness, reports) and A78's evidence are outside the tree, and the export of the dogfood store with the pinned engine is the one step with a deadline set by the plan itself (order 050's new store, more than the re-pin, since the store's format is unchanged). The weaknesses are in a few overstated facts and in the fix. 'Never committed' ignores the committed fresh-author report. The 'missing' Record of 144 is on the branch the handover names. Workflow files did change after CI's creation, and the private-names variable is described in AGENTS.md. In the fix, the done-when's `grep -c '(log)'` can never print 0, and the `.py.txt` precedent is frozen evidence, not live tooling: a method-as-run page avoids the Rust-rule exception. The fix also lacks a secret scan, although it commits reports and exports in the wake of a key exposure. I7's pass condition claims more attribution than the records can give while own tools are unmediated, and it mixes the harness's recovery practice with REC-001. I12's checks 2 and 3 cost more than stated, and check 3 fails in the very situation that produced this handover. The who-decides assignments are right, except that I6 is an owner's action rather than a ruling question, and the coordinator's own private note should be read first. Coverage: I read in full HANDOVER.md, the dispatch and checking sections of WORK_ORDERS.md (lines 40-160), verify.md, the review brief's relevant parts, AGENTS.md:60-153, tools/check_private_paths.py, and the Records of 042, 069b, 072, 075 and 079 at the cited lines. From the first pass I read P06 I7, I12 and the verifier's notes, P01 I13, P07 I9's verdict, and the approach critic's register and missing-area paragraphs. I did not read the private note (outside the repository), the full ROADMAP or AUDIT, or every Record; I built and ran nothing.
