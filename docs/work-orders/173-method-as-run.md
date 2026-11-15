# Work order 173: the method as run: one page for dispatch, rounds, reads and merge

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order writes one page and changes up to three
documents. It has no network access and needs none. It needs no cargo command; if you run one, run
one at a time. The inputs in `target/wo-173/` are text to read, never to run: run none of them, and
run no `codex`, `ssh`, `git fetch` or `git push`. Read each input in parts, with `sed -n`, and never
print a whole large file. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 27 of [its orders](../proposals/full-review/orders.md)) found
that the runners that dispatch every work order are kept outside the tree, beside the private
handoff note on one machine, and that the tree describes them in places that disagree
([FR-527](../proposals/full-review/findings/G6-outside-the-tree.md#fr-527), as its verifier
corrected it):

- [WORK_ORDERS.md](../implementation/WORK_ORDERS.md) says "One more round at most", after which the
  admin finishes the work itself. [The handover](../implementation/HANDOVER.md) runs up to
  `ROUNDS` rounds, then stacks finishing orders.
- WORK_ORDERS.md's command sets `CARGO_NET_OFFLINE`, while a `net` order fetches crates.
- [verify.md](verify.md) told the verifier that the changes were uncommitted. Ruling 79's change
  corrected that: it now says that the runner commits them as a checkpoint.

If the one copy of the runners were lost, no order could be dispatched until they were rebuilt
from prose.

[FR-526](../proposals/full-review/findings/G6-outside-the-tree.md#fr-526), part 1, asks for the
method as run: the worker's and the verifier's commands in full, every variable, the guards, the
checkpoint commit, what the verifier is shown, the load rule, and the failure modes the Records
name. Its verifier corrected the form. The coordinator's D62 and D13
([decisions](../proposals/full-review/decisions.md)) apply that correction:

- the tree gets a Markdown page with each command and every private value as a variable, not the
  scripts: a copy that does not run would drift, and the tree's code is Rust;
- the scripts stay outside the tree;
- a merge that changes how orders are dispatched or checked updates the page in the same pull
  request.

D63 has the page say what becomes of each round's checkpoint. Two '(log)' marks of
[FR-532](../proposals/full-review/findings/G6-outside-the-tree.md#fr-532) go into the page: the
load rule and the two failed dispatches of order 081. Moving the handover's step 7 also closes a
third mark, which FR-532's table says to drop because the Records of 075 and 072 already hold its
fact. [FR-270](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-270)'s verifier
corrected a citation in the handover. Ruling 71 point 4 lets a coordinator settle a small finding
that adds a feature. The leave to finish an order by hand is a different one: the admin's, under
ruling 48, as WORK_ORDERS.md stated it.

Since the review, [ruling 78](../proposals/ruling-78/README.md), point 1, left one coordinator
until the S1 rerun's pin. [Ruling 79](../proposals/ruling-79/README.md), point 2, added a fresh Opus
reader before each merge and for each of the coordinator's fixes. The Records of the orders merged
since, from 151 on, show the Opus reader as run.

This order writes the page and points the other descriptions at it. It changes no step of the
method. The choices it states are D63's and, if no input gives one, the command that reads the load
(decision 8). It does not commit the runners or any input, and it leaves the handover's other
'(log)' marks and other stale lines to the review's orders 37 and 36.

## The decisions it applies

Line numbers are those of `main` at `7346a280`. If a line has moved, find the phrase by its words.

1. **The inputs are the coordinator's.** Before the worker began, the coordinator placed five text
   files in `target/wo-173/` at the tree's root. Git ignores `target/`, so none is committed, and
   the coordinator deletes them after the merge. In each file, the coordinator replaced every
   private value (a path, a host, a user name or an account) with a placeholder: capitals between
   angle brackets, such as `<MAC_CHECKOUT>`. The five files:
   - `run-order.txt`: the runner that dispatches an order on the Mac. It covers the worktree and
     branch, the worker, the checkpoint, the Codex verifier and the rounds.
   - `continue-order.txt`: the runner that continues an order from a verifier report.
   - `remote-order.txt`: the runner that starts an order on the Windows computer over SSH, follows
     its log and fetches its branch back.
   - `opus-reader.txt`: the brief the coordinator gave to the fresh Opus reader of an order and of
     a coordinator's fix, as used for the orders merged since 151.
   - `variables.txt`: one line for each placeholder and for each environment variable that the
     runners read from their caller. There is no header line. Each line has four tab-separated
     fields: the name as the inputs write it; `private` or `not private`; what it stands for; and,
     for a variable that is not private, its default or `none`. The file holds no private value.

   Before anything else, run `shasum -a 256 target/wo-173/*` and keep the output for your report.
   Do not change, move, run or delete any input. If any of the input checks in Done when prints
   anything, stop: report the file, the line and the check, and write nothing. The coordinator
   corrects the inputs.
2. **The page is `docs/implementation/METHOD_AS_RUN.md`.**
   - It is about 280 lines long, and at most 340.
   - Prose lines are at most 100 characters; tables and code blocks are exempt.
   - Placeholders appear only in code spans and code blocks.
   - The title is `# The work-order method as run`. After a blank line comes a line that reads
     exactly "Not binding.".

   Then one paragraph says four things:
   - the page gives the steps by which the coordinator writes, dispatches, checks and merges a
     work order, as run on the day you write it, with each command;
   - the runners are scripts kept outside the tree, beside the private handoff note, which also
     holds every private value. The tree keeps this page instead, because a copy that does not run
     would drift and the tree's code is Rust (D62, logged for the owner to strike);
   - a merge that changes how orders are dispatched or checked updates this page in the same pull
     request (D13);
   - until the S1 rerun's pin, one coordinator runs the method (ruling 78, point 1). The lane,
     when it restarts at the pin, uses the same runners (the handover's section Restarting the lane
     coordinator).

   "The private handoff note" is a Markdown link to the handover. D62 and D13 link to the
   decisions file, ruling 78 links to its record, and the section's name links to that section
   of the handover.
3. **The page's sections**, in this order and with these headings:
   - `## Variables`. A table with the header `| Variable | Stands for | Private | Default |` and one
     row for each line of `variables.txt`, in the file's order:
     - Variable: the name as the inputs write it, in a code span;
     - Private: `yes` or `no`;
     - Default: the input's default, or `none`; for a private variable, `in the private note`.

     BASE, SCHEMA, EXTRA, JOBS, TT, ROUNDS, START_ROUND and VERIFY_FIRST each get a row. For one
     that `variables.txt` does not list, Stands for says "not read by the runners as run", Private
     is `no` and Default is `none`, and your report says so. Below the table, one sentence: a
     private value is never written in the tree, in an order, in a Record or in a pull request.
   - `## 1. Write the order`. The handover's step 1 (lines 54 to 60): the form that orders 162 to
     169 use (the preamble, What and why, The decisions it applies, Scope, Done when, and at merge
     the Record); the schema named at dispatch; and the order's file merged to `main` first, on a
     branch `orders/NNN`, because the runners read order files from `origin/main` whatever the
     base. Link WORK_ORDERS.md's section An order for the design.
   - `## 2. Dispatch on the Mac`. From `run-order.txt`, in its order:
     - how the runner is called, with `net`;
     - the checks it makes before the worker starts;
     - the worktree and the branch, made with `--no-track`, and what the runner does with a
       leftover branch;
     - the environment variables it removes, by name only;
     - the worker's command, whole, in a code block;
     - how the command differs for a `net` order, against `CARGO_NET_OFFLINE` otherwise;
     - what the worker reads on standard input, including `EXTRA`'s section "From the admin
       session";
     - where the report and the event stream are written;
     - the checkpoint commit, its message and its git identity;
     - `JOBS` and `TT`;
     - the model and the effort, as the runner passes them. If it passes no model, say so: the
       model is then the client's default on the day of the run (FR-527).

     Beside each flag, keep the reason WORK_ORDERS.md gives for it (lines 58 to 73). Where the
     command as run does not pass that flag, say so in one line and keep the reason.
   - `## 3. The Codex verifier and the rounds`. From `run-order.txt`:
     - the verifier's command, whole;
     - what the verifier is shown;
     - what starts another round, and `ROUNDS`;
     - what the runner does when a verifier fails or writes no valid report.
       [The harness of experiment 028](../../experiments/028-a78-dogfood/inputs/harness.md) says
       that `run-order.sh` then reads it as no findings. Say what the input does.

     Keep WORK_ORDERS.md's sentence on the reviewer reading cold (lines 80 to 82). Then give
     decision 5's rule.
   - `## 4. Continue from a report`. From `continue-order.txt`: `START_ROUND`, `VERIFY_FIRST`,
     which report the runner reads and from where, and the base it uses. Then the handover's step 4
     (lines 70 to 73): the coordinator may write that report itself, to continue a checkpoint (070,
     077 and 081 after the Windows pause) or to fix a merge (042 after its rebase).
   - `## 5. On the Windows computer`. From `remote-order.txt`:
     - the SSH target and the account the remote copy runs as, as placeholders;
     - the work-order clone there, its git identity and rustup's directory;
     - how the order reaches it;
     - how the runner follows its log;
     - how `wo/NNN-name` is fetched back to the Mac.

     Then give the handover's step 7 (lines 87 to 90). Its last clause cites the Records of 075 and
     072, each as a Markdown link, in place of its '(log)' mark. Then one sentence links the
     handover's section The second machine, for the owner's pause and resume.
   - `## 6. The Opus reader and fixes by hand`. Ruling 79, points 2 and 4:
     - once an order's checks pass, a fresh, read-only Claude Opus reader verifies it before it
       merges;
     - a fix of the coordinator's is read the same way;
     - when the Claude allowance runs low, dispatch stops first and nothing merges unread.

     Then say what the reader is given and asked, from `opus-reader.txt`, in at most 15 lines.
     Quote its words for what the reader must do and must not do. Then give decision 6.
   - `## 7. Merge and the Record`. The handover's steps 6 and 8 (lines 77 to 86 and 91 to 98), in
     this order:
     - the checks of AGENTS.md, and for a Rust directory its tests, fmt and clippy;
     - `tools/check_private_paths.py`, run with `CONDUIT_PRIVATE_NAMES` set once the files are
       staged, and the pull request's `secrets` job;
     - the guarded squash, in one command, exactly as the handover's bullet "The squash guard"
       gives it (lines 295 to 297), then the scope check;
     - the push, the pull request and CI. Give CI's jobs as `.github/workflows/checks.yml` defines
       them on your base, with the rest of step 8's facts. At `7346a280` the `docs` job runs seven
       Python checks (lines 120 to 146), where step 8 says six, and the `secrets` job scans the
       commits a pull request adds;
     - the squash-merge with a message written by hand, whose subject takes the form `git log`
       shows for the orders merged since 151: "Work order NNN: title (#PR)";
     - the Record at the order's foot. Its first sentence takes the form that most Records since
       151 use: "Run on DATE on the Mac from `main` COMMIT: the worker (N s) and a fresh Codex
       verifier (N s), ...". Then come the rounds, the findings, each Opus reader and what it found,
       the coordinator's changes and own decisions, and what is left for later.

     Then give decision 7.
   - `## Load`. Decision 8.
   - `## Failure modes`. Decision 9.

   The page ends with one sentence: an order run through Conduit (A78) uses the harness in
   experiment 028, which keeps `run-order.sh`'s interface and log lines. "Experiment 028" is a
   Markdown link to that experiment's README.
4. **The inputs govern what the page says the runners do.**
   - Every command, flag and variable in sections 2 to 6 comes from an input, from the text this
     order moves into the page, or from a decision here.
   - Give each command a step runs, in order, with its placeholders. Do not reproduce a script's
     loops, parsing or logging.
   - Where a command uses a script's own variable, write what it holds in the page's terms (a
     placeholder from `variables.txt`, `NNN`, `wo/NNN-name`), or say beside the command what it
     holds. No name in a command is left unexplained.
   - Where an input and the tree's text disagree, follow the input. List each difference in your
     report, with both places.
   - Where an input does not give what a section asks for, write "not recorded in the runner"
     there, and do not guess ([ruling 52](../proposals/ruling-52/README.md)).
5. **After the last round** (FR-270, as its verifier corrected it; D5; ruling 79, point 2). The
   page says what happens when the rounds end with a must left, or when an Opus reader finds one.
   The coordinator reads the findings and the branch, and the Record names one of three outcomes:
   - The fault is of the same kind as an earlier pass's. The mechanism is wrong, and a new order
     replaces it, as 150 replaced the handoff of 146 and 149.
   - The fault is new, local and small. The coordinator fixes it, and a fresh Opus reader reads
     the fix before the merge.
   - Otherwise, a finishing order names each finding and its kind, and is stacked on the branch.
     Use the examples of the handover's step 5 (lines 74 to 76).

   There is no fixed stop at a fourth pass. When the coordinator applies a reader's shoulds,
   another fresh Opus reader reads them too, as the Records of 153, 156 and 168 show. This rule
   replaces WORK_ORDERS.md's "One more round at most".
6. **A fix by hand cites its leave** (FR-270's correction). Section 6 says:
   - A coordinator's fix of a last finding rests on the admin's leave to finish an order itself:
     the admin's, under ruling 48, which WORK_ORDERS.md stated as "the admin finishes the work
     itself" and this page now holds under D5. A fresh Opus reader reads the fix.
   - Ruling 71 point 4 is a different leave. Under it, a coordinator settles a small finding that
     adds a feature and logs it for the owner to strike. Until the S1 rerun, it excludes anything
     that could move S1's author counts: CLI flags, short-form keys, templates and defaults
     (IN_FORCE.md, topic 15).
   - The handover's step 9 (lines 99 to 103) cited point 4 for both. Keep its examples as it lists
     them.
7. **Each round's checkpoint** (D63). Section 7 ends with this text:

   > Each round's checkpoint is a local commit on `wo/NNN-name`. For an order run on the Windows
   > computer, it is also on the `win/` ref fetched to the Mac. Only the squash is pushed at
   > merge, so the round-by-round diffs are discarded after the merge, and only the Records
   > describe the rounds (D63). An order stopped unmerged is pushed with its checkpoints, as 049a1,
   > 080 and 144 are.

   If `remote-order.txt` fetches the branch to another ref, name that ref instead and report it.
   If an input shows that a runner pushes a checkpoint, say what it pushes, and report it.
8. **Load** (FR-532, the mark on the handover's line 303). The section gives the handover's
   bullet (lines 300 to 303) without its '(log)' mark:
   - the lane coordinator runs one heavy, engine-building order on each machine;
   - the admin ran at most two engine orders on the Windows computer until 070 merged;
   - on the Mac, the coordinator dispatches only while the load is under 8, with `JOBS` and `TT`
     capped;
   - under load, timing-sensitive shell tests are rerun with `RUST_TEST_THREADS=2`.

   Give the command that reads the load, as an input gives it. If no input does, give
   `sysctl -n vm.loadavg` as the check the coordinator makes by hand before a dispatch, and say
   that its first number is the one-minute load.
9. **The failure modes the Records name**, one bullet each, each with its source as a Markdown
   link:
   - two orders started at once raced for Git's configuration, hence `--no-track`. Its source was
     WORK_ORDERS.md's section Dispatch, which decision 10 removes: cite
     [FR-525](../proposals/full-review/findings/G6-outside-the-tree.md#fr-525), whose verifier
     lists it among the runner's fixes;
   - on the Windows computer, the checkpoint commit failed for want of a git identity, which is now
     set in the work-order clone ([069b's Record](069b-spec-plan-apply-ruling-76.md#record));
   - rustup could not write its own directory in the Windows sandbox, which the runner now allows
     ([075's Record](075-spec-own-tools-reach.md#record));
   - on the Windows computer, the run script exited 127 after Codex had finished, for a command it
     could not find. The admin committed the work and ran the verifier on the Mac
     ([079's Record](079-shell-https-address.md#record)). The Record does not say which command;
   - two of 081's dispatches failed at once, one for a missing `SCHEMA` and one for a leftover
     branch. Its only source was the handover's line 307, which decision 11 removes; the page now
     records it in place of that '(log)' mark. Cite
     [FR-532](../proposals/full-review/findings/G6-outside-the-tree.md#fr-532), whose table lists
     it;
   - the Codex verifier's read-only sandbox does not rerun the build or the tests
     ([156's Record](156-runner-stopped-after-end.md#record),
     [167's Record](167-a5-probe-signals-tested.md#record)). The Opus readers of 156, 161 and 167
     reran the order's checks;
   - a verifier that fails or writes no valid report, as decision 3 has the page say.

   For each, say what the runner now does about it, as an input shows, or write "not recorded in
   the runner".
10. **WORK_ORDERS.md.** Replace lines 43 to 86, from the heading "## Dispatch" through item 3 of
    "Checking what comes back", with a section headed `## Dispatch and checking`. The section holds
    only this paragraph, wrapped at 100 characters:

    > How an order is dispatched, checked by a fresh Codex verifier in each round, read by a fresh
    > Opus reader before it merges (ruling 79) and merged, with each command and the fault each
    > part of the dispatch answers, is in the method as run.

    "The method as run" is a Markdown link to the page. "Ruling 79" links to that ruling's record.
    Every fact of the removed lines is in the page, as decisions 3, 5 and 9 place it. In the
    lesson "Close the input, pin the flags, write the report to a file." (line 109), "The dispatch
    above is" becomes "The dispatch's flags are"; rewrap that bullet only. Nothing else in
    WORK_ORDERS.md changes.
11. **HANDOVER.md.** Make only these changes:
    - **Lines 10 to 12.** "(paths, hosts, accounts, the runners)" becomes "(paths, hosts,
      accounts, the runners' scripts)". After "beside the main checkout.", add: "The method as run
      gives the runners' steps, with each private value a variable." "The method as run" is a
      Markdown link to the page. Rewrap lines 10 to 13 at 100 characters; line 9 does not change.
    - **The pipeline section.** The body of "## The work-order pipeline", lines 51 to 103, becomes
      the paragraph below, and the heading stays:

      > The method as run gives every step, from writing an order to its Record, with each
      > command and each runner variable. Work orders gives the design and the lessons.

      "The method as run" links to the page, "Work orders" links to WORK_ORDERS.md, and "the
      lessons" links to WORK_ORDERS.md's section What this round taught. Every fact of steps 1 to 9
      is in the page, in the section that decisions 3 to 6 name for it.
    - **Line 120.** "(item 7 above)" becomes a parenthesized Markdown link, with the text "the
      method as run", to the page's section 5. On the Windows computer.
    - **"The squash guard"** (lines 293 to 297). Keep its text up to "restored in #190.". The rest
      becomes "The method as run gives the one command and the check after it.", where "The method
      as run" links to the page's section 7. Merge and the Record.
    - **"Load"** (lines 300 to 303) becomes "**Load.** The method as run gives the rule.", where
      "The method as run" links to the page's section Load.
    - **"Don't start finishing orders blindly"** (lines 304 to 307) loses its last sentence, on
      081's two failed dispatches. The page's failure modes now hold it.

    Nothing else in the handover changes. The review's order 36 edits it next.
12. **verify.md.** It changes only where `run-order.txt` or `continue-order.txt` shows that it
    misstates what the verifier is given. Then only that sentence changes, and your report names
    it. Otherwise verify.md does not change. Its checkpoint sentence (lines 3 to 5) already matches
    FR-527's correction.

## Scope

Create `docs/implementation/METHOD_AS_RUN.md`. Change only:

- `docs/implementation/WORK_ORDERS.md`, in the lines decision 10 names (43 to 86, and the lesson on
  line 109);
- `docs/implementation/HANDOVER.md`, in the lines decision 11 names;
- `docs/work-orders/verify.md`, and only under decision 12.

Change nothing under `target/wo-173/`. Change nothing else, including AGENTS.md, CONTRIBUTING.md,
the review's files, the second-machine guide, experiment 028 and every order's file.

## Done when

First, the input checks. From the repository root, each of these prints nothing. If one prints
anything, stop as decision 1 says:

```sh
I=target/wo-173
for f in run-order continue-order remote-order opus-reader variables; do test -s $I/$f.txt || echo $f; done
grep -rnE '/(Users|home)/[A-Za-z0-9._-]+|[A-Za-z]:\\+Users\\+|\.ts\.net|SHA256:[A-Za-z0-9+/]{43}|100\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\.[0-9]{1,3}\.[0-9]{1,3}|/private/[t]mp/|/[t]mp/[c]laude|\.codex/[a]uth\.json' $I | cut -d: -f1,2
grep -rcE 'sk-[A-Za-z0-9_-]{20,}|gh[pousr]_[A-Za-z0-9]{20,}|github_pat_|xox[abpr]-|AKIA[0-9A-Z]{16}|lin_api_|BEGIN [A-Z ]*PRIVATE KEY' $I | grep -v ':0$'
grep -ohE '<[A-Z][A-Z0-9_]*>' $I/*.txt | sort -u | while read -r v; do cut -f1 $I/variables.txt | grep -qxF "$v" || echo "$v"; done
```

Then, once the work is done, each of these passes from the repository root:

```sh
P=docs/implementation/METHOD_AS_RUN.md
I=target/wo-173
shasum -a 256 $I/*                       # the same digests as before you began
grep -c '^Not binding\.$' $P             # 1
wc -l < $P                               # about 280, at most 340
grep -c '^| Variable | Stands for | Private | Default |$' $P   # 1
awk '/^ *```/ {c = !c; next} !c && !/^\|/ && length > 100 {print FNR}' $P   # prints nothing
for h in '## Variables' '## 1. Write the order' '## 2. Dispatch on the Mac' '## 3. The Codex verifier and the rounds' '## 4. Continue from a report' '## 5. On the Windows computer' '## 6. The Opus reader and fixes by hand' '## 7. Merge and the Record' '## Load' '## Failure modes'; do grep -qxF "$h" $P || echo "$h"; done   # prints nothing
cut -f1 $I/variables.txt | while read -r v; do grep -qF "| \`$v\` |" $P || echo "$v"; done   # prints nothing
awk -F'\t' '$2 == "private" {print $1}' $I/variables.txt | while read -r v; do grep -F "| \`$v\` |" $P | grep -qF '| yes | in the private note |' || echo "$v"; done   # prints nothing
grep -oE '<[A-Z][A-Z0-9_]*>' $P | sort -u | while read -r v; do grep -qF "| \`$v\` |" $P || echo "$v"; done   # prints nothing
for v in BASE SCHEMA EXTRA JOBS TT ROUNDS START_ROUND VERIFY_FIRST; do grep -qF "| \`$v\` |" $P || echo "$v"; done   # prints nothing
for s in "raced for Git's configuration" 'A70 lost a task' 'surprise 7' "order 010's second round" 'a contradiction across nine promises' 'orders/NNN' 'mknodat' '#207' '#249' 'RUST_TEST_THREADS=2' 'exited 127' 'git merge-base --is-ancestor origin/main HEAD && git reset --soft origin/main' 'discarded after the merge'; do tr -s ' \n' '  ' < $P | grep -qF -- "$s" || echo "$s"; done   # prints nothing
for f in 075-spec-own-tools-reach.md 072-shell-list-operation.md 069b-spec-plan-apply-ruling-76.md 079-shell-https-address.md; do grep -qF "$f" $P || echo "$f"; done   # prints nothing
grep -c '(log)' $P                       # 0
grep -nE '/(Users|home)/[A-Za-z0-9._-]+|[A-Za-z]:\\+Users\\+|\.ts\.net|SHA256:[A-Za-z0-9+/]{43}|100\.(6[4-9]|[7-9][0-9]|1[01][0-9]|12[0-7])\.[0-9]{1,3}\.[0-9]{1,3}|/private/[t]mp/|/[t]mp/[c]laude' $P | cut -d: -f1   # prints nothing
grep -cE 'sk-[A-Za-z0-9_-]{20,}|gh[pousr]_[A-Za-z0-9]{20,}|github_pat_|xox[abpr]-|AKIA[0-9A-Z]{16}|lin_api_|BEGIN [A-Z ]*PRIVATE KEY' $P   # 0
grep -c '(log)' docs/implementation/HANDOVER.md                    # 7; on main, 10
grep -c 'METHOD_AS_RUN.md' docs/implementation/HANDOVER.md         # 5
grep -c 'METHOD_AS_RUN.md' docs/implementation/WORK_ORDERS.md      # 1
grep -c '^## Dispatch and checking$' docs/implementation/WORK_ORDERS.md   # 1
grep -c -e 'One more round at most' -e 'CARGO_NET_OFFLINE' -e 'The dispatch above' docs/implementation/WORK_ORDERS.md   # 0
tr -s ' \n' '  ' < docs/implementation/WORK_ORDERS.md | grep -c "The dispatch's flags are the sum of the first round's harness faults"   # 1
grep -c -e 'START_ROUND' -e 'item 7 above' -e "Two of 081's dispatches" -e 'git reset --soft' docs/implementation/HANDOVER.md   # 0
grep -c 'are uncommitted' docs/work-orders/verify.md               # 0
python3 tools/check_docs.py              # Errors: 0; it reads the untracked page too
python3 tools/check_private_paths.py     # exit 0
git diff --check
git status --short                       # the changed files in scope and ?? docs/implementation/METHOD_AS_RUN.md, nothing else
```

On `main`, the page does not exist, WORK_ORDERS.md still says "The dispatch above" and the handover
has ten '(log)' lines, so these checks fail without the change. Do not stage anything. Report each
choice you make as your own, each difference decision 4 asks for, the digests from before you began
and from the end, and every check's last line.

The admin then has a fresh, read-only Opus reader read the diff, with the inputs still in place,
before it merges (ruling 79). The admin stages the files and runs `check_private_paths.py` with
`CONDUIT_PRIVATE_NAMES` set, and the pull request's `secrets` job must pass. Before the merge, the
coordinator also runs the review's own check for this order, which is not the worker's:

- A fresh, read-only Codex session (ruling 48, point 4) works in a clean clone of the pushed
  branch, with no private note, no memory, none of the owner's settings and no `target/wo-173/`.
- It writes the exact commands for three tasks: to dispatch this order on the Mac; to continue
  order 080 from its round-2 verifier report, since 071, which the review named, has merged; and
  to start this order on the Windows computer. It lists every value it had to guess.
- It passes when each guess is a variable the page marks private. An order's own content, such as
  the text of 080's `EXTRA`, is not a guess about the method.

A failure is fixed on the branch before the merge, and the fix is read as decision 5 says. The
coordinator records the result in this order's Record and deletes `target/wo-173/` after the merge.
