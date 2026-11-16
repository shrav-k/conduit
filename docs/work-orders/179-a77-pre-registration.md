# Work order 179: A77's pre-registration, under rulings 79 and 80, before the first paired read

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order changes three documents. It has no
network access and needs none. It needs no cargo command; if you run one, run one at a time.
Never print a whole large file: read `ROADMAP.md`, `docs/implementation/WORK_ORDERS.md` and
`docs/implementation/METHOD_AS_RUN.md` with `grep -n` and `sed -n`. Never run `env`, `printenv`,
`set` or `export -p`, never print an environment variable's value, and never read or print a
key, a token or any credential.

## What and why

The full review of 2026-11-14 (order 34 of [its orders](../proposals/full-review/orders.md))
found that A77, the register's row for ruling 48's question, "is the work as good, and does the
admin session spend less?", can neither pass nor fail. Its measure is "reviewed after five
orders", it has no pass condition, and "admin time was not recorded" (`ROADMAP.md` line 70).
Four findings say what it lacks:

- [FR-272](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-272), as its verifier
  corrected it: a pass condition over the orders dispatched, merged or not; a small record that
  a script can read; escapes and a red `main` as the measure of "as good"; and the coordinator's
  spend, which only its session's transcript holds;
- [FR-498](../proposals/full-review/findings/G4-who-verifies.md#fr-498), as its verifier
  corrected it: no two readers ever read one diff under one brief, so both read round 1, blind to
  each other, under the brief that will be used, judged with their names removed by a same-fault
  rule fixed first, on at least five orders per pooled kind;
- [FR-497](../proposals/full-review/findings/G4-who-verifies.md#fr-497): no Record names a must
  that the coordinator rejected, so each pass gains its musts accepted and rejected, each
  rejection with its reason;
- [FR-501](../proposals/full-review/findings/G4-who-verifies.md#fr-501): the Records do not give
  each reader's model and flags, the reads of an order before dispatch, the coordinator's spend,
  or the orders that did not merge.

The review merged FR-272 and FR-498 into A77's row of
[its changes to the experiments](../proposals/full-review/experiments.md#register-rows), and the
coordinator's D9 ([decisions](../proposals/full-review/decisions.md)) lists, from FR-272, FR-497
and FR-501, what each Record holds for each pass.

[Ruling 79](../proposals/ruling-79/README.md) then added a reader: beside the fresh Codex verifier
that reads each round, a fresh, read-only Opus reader reads each order once its checks pass,
before it merges. [Order 173](173-method-as-run.md) writes that method as run in
`docs/implementation/METHOD_AS_RUN.md`. The review planned the reader on the coordinator's model
for round 1, with ten extra Codex reads as the control. Under ruling 79 the Opus reader comes
last, so the pair on round 1 that the review's text asks for costs one more Opus read per paired
order instead, at most ten, on the Claude allowance. The owner keeps the pair on round 1
([ruling 80](../proposals/ruling-80/README.md), point 6); paired reads stop before dispatch does
when the allowance runs low. Both readers of the pair work under [verify.md](verify.md), the Codex
verifier's brief, which ruling 79 keeps in each round, and within that verifier's read-only
limits. The Codex verifier runs without the owner's configuration, so the Opus reader is started
with no memory or settings of the owner's, as D3 has the review's reader started. So only the
model differs. FR-498's verifier expected the wider brief of
[FR-492](../proposals/full-review/findings/G4-who-verifies.md#fr-492); verify.md has not taken
it, so the brief as it stands is the one that will be used. Until the pin one coordinator runs
the work ([ruling 78](../proposals/ruling-78/README.md), point 1), so it alone judges the paired
findings, on the paired reader's model: the registration hides the readers' names and says what
that cannot hide. The review would drop the Codex read where, over five orders, it adds no must
that the Opus reader missed. That would change ruling 79, point 2, so the result is a proposal
that the owner decides when it comes in (ruling 80), and the registration changes nothing by
itself.

Ruling 80, point 10, then changed what the review's "no merge leaves main's CI red" can mean.
Each merge now waits for a local gate on the coordinator's machine and no longer for GitHub's CI,
which keeps running and is not waited on; a run that fails after a merge is fixed in a
follow-up. The run of a merge's push to `main` therefore ends after the merge, and a red there
can no longer be prevented by waiting, only counted. The registration counts it: a merge turns a
job red when the job fails in that run and did not fail in the run of the push before it, so a
job left red by an earlier merge is not counted again against a later one.

This order writes the registration as a new section of
[work orders](../implementation/WORK_ORDERS.md), about 170 lines, which the review's done-when
finds by "A77's measure". It adds one paragraph to the method page, since D13 asks a merge that
changes how orders are checked to update that page in the same pull request. It gives A77's row
in `ROADMAP.md` the claim, the counting rule beside its done-when's old words, and, in its State,
the registration in place of ruling 80's note that this order rewrites the row. A77's question
comes from ruling 48, so the owner was told (FR-272's verifier): ruling 80 keeps the question's
second part and the counting rule over the 20 dispatched orders, and records the coordinator's
spend per order only as a baseline, compared with no work that the coordinator does by itself.
The order's other choices are the coordinator's under [ruling 45](../proposals/ruling-45/README.md),
each logged for the owner to strike ([ruling 48](../proposals/ruling-48/README.md)): as the
baseline, the 20 orders merged before A77's, recounted by the same rule, where the review's row
has "the rate so far"; two groups, code and text, of five paired orders each; verify.md as the
paired brief, with the paired reader started without the owner's memory or settings; the blind
judging and the same-fault rule; the seven days within which a fault counts as an escape; a red
counted against the merge that turned the job red; the record's keys, with each pass's musts
judged by the time the order's Record is written; and the spend counted over the sample's span,
without the readers' tokens. Where a measure cannot be had, the registration says that it is not
shown ([ruling 52](../proposals/ruling-52/README.md)).

The order changes three documents that bind nothing. It changes no promise and no vector, so it
needs no case or expected failure ([CONTRIBUTING.md](../../CONTRIBUTING.md), rule 8, from
[order 163](163-a24-at-entry.md)), no row of the model-checking account that
`tools/check_model_account.py` checks, and no declaration of vector changes
([order 174](174-vector-changes-declared.md)). It starts no read and writes no script: the script
that merges two reports blind, the records and the transcript script are the coordinator's.
It builds on ruling 80's record (#318), which this order and its new section link and which put
in A77's State the note that decision 4 replaces, and on orders 170 and 173, both on `main`: 173
wrote the method page that the new section links, and [order 170](170-a78-pre-registration.md)
changed A78's row, the line after A77's. Order 193b, which switches the merge to the local gate,
changes the table of variables and sections 5 and 7 of the same method page; this order changes
only the end of section 3, so either may merge first.

## The decisions it applies

Line numbers are those of `main` at `d4c8cf8b`, which holds orders 170 and 173 and ruling 80's
record. If `main` has moved since, find each place by its words. Each block below is written word
for word, with DATE replaced by the date you write it, as YYYY-MM-DD. Do not rewrap a block: its
lines are at most 100 characters, and each link sits whole on one line, because `check_docs.py`
checks a link only when its text and its target share a line. The blocks for
`docs/implementation/` are written as they serve from there.

1. **The base holds orders 170 and 173 and ruling 80's record.** Before anything else, from the
   repository root, run these five commands. Each prints 1. If one prints anything else, change
   nothing and report which: the coordinator dispatches this order only from a `main` on which
   all five print 1.

```sh
grep -c '^## 4\. Continue from a report$' docs/implementation/METHOD_AS_RUN.md
grep -c '^## Dispatch and checking$' docs/implementation/WORK_ORDERS.md
grep -c '^### The Claude verifier slot$' experiments/028-a78-dogfood/README.md
git ls-files docs/proposals/ruling-80/README.md | grep -c .
grep '^| A77 |' ROADMAP.md | grep -c 'with no comparison; order 179 rewrites this row under it |'
```

2. **The registration is a new section of `docs/implementation/WORK_ORDERS.md`.** Insert this
   block between the section "Running the orders through Conduit" (line 98), whose last
   paragraph ends "rather than going to any worker.", and the heading "## The record" (line 115),
   with one blank line before it and one after it. Nothing else in the file changes.

````markdown
## A77's measures, registered before the first paired read

Registered on DATE by work order 179, before the first paired read
([ruling 45](../proposals/ruling-45/README.md)). [A77](../../ROADMAP.md#experiments-and-probes)
keeps ruling 48's question: when Codex workers carry out the work under the admin session's
orders, is the work as good, and does the admin session spend less? It measures
[the method as run](METHOD_AS_RUN.md) under [ruling 79](../proposals/ruling-79/README.md), a
fresh Codex verifier in each round and a fresh, read-only Claude Opus reader once the checks
pass, and adds one step, the paired read below. It applies
[FR-498](../proposals/full-review/findings/G4-who-verifies.md#fr-498) and
[FR-272](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-272) as their verifiers
corrected them, [FR-497](../proposals/full-review/findings/G4-who-verifies.md#fr-497),
[FR-501](../proposals/full-review/findings/G4-who-verifies.md#fr-501), D3 and D9 of
[the review's decisions](../proposals/full-review/decisions.md). The owner's
[ruling 80](../proposals/ruling-80/README.md), point 6, pairs the readers at round 1, keeps A77's
question whole and the 20-order counting rule, records the coordinator's spend per order only as
a baseline, with no comparison, and leaves what the paired reads change to the owner. Its point
10 has each merge wait for a local gate and no longer for CI, so claim 1 counts the reds that
come after a merge. The other choices here are the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)). It changes only by a dated amendment, merged
before the first order it changes is dispatched.

### Claim

1. **The work is as good.** No merge of A77's orders turns a job of `main`'s CI red, and no more
   faults escape per merged order than from the baseline's. Merges wait for the local gate and
   not for CI (ruling 80, point 10), so a red is counted after the merge, not prevented.
2. **On one diff under one brief, the Opus reader finds what the Codex verifier finds, and
   more.** At round 1 of each group's paired orders, a fresh Opus reader finds every must-level
   fault that the Codex verifier finds, and at least one that the verifier misses.

The coordinator's spend per order is measured with no condition: it was never measured before,
so this is its first baseline.

### The orders

- *A77's orders* are the next 20 work orders dispatched after this section merges, merged or
  not. An order that the coordinator carries out itself, with no worker, is not dispatched. In
  an order run through Conduit (A78), the Codex verifier is `verifier-1`.
- *The baseline* is the 20 work orders merged last before this section merges. An order
  dispatched before this section merges and merged after it is in neither.
- An order is *code* if its Scope lets it change a `.rs` or `.py` file or a file under
  `.github/`, and *text* otherwise.
- *Paired orders.* Each of A77's orders whose round 1 is a Codex worker's is paired, in dispatch
  order, while its group has fewer than five. Each costs one more Opus read, at most ten in all.
  When the Claude allowance runs low (ruling 79, point 4, as ruling 80, point 6, amends it),
  paired reads stop before dispatch does: an order dispatched then is not paired, and its record
  says why.

### The paired read

A paired order is dispatched with `ROUNDS=1`, so that the runner stops after round 1's verifier.
Two fresh readers read its round-1 checkpoint, the same diff from the same base: the Codex
verifier, as [the method](METHOD_AS_RUN.md#3-the-codex-verifier-and-the-rounds) runs it, and a
Claude Opus reader, started as the method starts
[its Opus reader](METHOD_AS_RUN.md#6-the-opus-reader-and-fixes-by-hand) but given only what the
verifier is given: [the standing order](../work-orders/verify.md), the order, the worker's
report, the diff and the report's schema. The Codex verifier runs without the owner's
configuration, so the Opus reader is started with no memory or settings of the owner's (D3); a
pass that loaded any names them under `loaded`, and the result names each such paired order.
The Opus reader runs no command that writes, so no build and no test, as the verifier's
read-only sandbox allows none. Neither sees the other's report.

Before the coordinator reads either report, a script merges their findings into one list, each
finding whole but for its reader, sorted by file, line and problem, and numbered. The key from
number to reader, and each report's verdict, go into a file that the coordinator opens only once
it has judged every finding. It checks each finding against the files and records: accepted or
rejected, with the reason for a rejection; its own severity, by the standing order's words; for
a must, its class (order fault, specification gap, implementation, test weakness, out of scope
or environment); and for an accepted finding, the fault it belongs to. Two accepted findings
are one fault when one change to the files closes both; a finding that the change closes only
in part is a fault of its own. The accepted musts and shoulds of both readers start round 2, in
a report the coordinator writes for
[the continue runner](METHOD_AS_RUN.md#4-continue-from-a-report). The order then runs as the
method has it, and the Opus reader before its merge is another fresh one.

### The record each order keeps

For each of A77's orders, the coordinator commits `docs/work-orders/NNN-a77.yaml` in its first
pull request after the order is stopped, or after the checks run of its merge on `main` has
ended. It fills the record from the runner's files, the readers' reports, git and GitHub (D9),
and replaces each private value with a placeholder; the local gate and CI's path and secret
checks read it with the rest of that pull request. A pass whose findings the runner passed on to
the next round unjudged has its musts judged by the coordinator, as the paired read's are, when
it writes the order's Record, before the round's checkpoint is discarded. A record is complete
when no key is missing and none says `not recorded`:

```yaml
order: "<NNN>"
group: <code or text>
worker: <codex or opus, the model of round 1's worker (ruling 79, point 3)>
dispatched: <YYYY-MM-DDTHH:MMZ>
merged: <YYYY-MM-DDTHH:MMZ, or no and the reason>
base: <the commit round 1 started from>
lines: <+N -N, by git diff --shortstat of the squash, or of the last checkpoint>
order_reads: []    # each read of the order's file before dispatch: model, findings, changes made
rounds: []         # each worker round's seconds
passes:            # every read of the work, in the order they ran
  - reader: <codex, opus-paired, opus or opus-fix>
    round: <N>
    model: <as the client names it>
    client: <the client and its version>
    effort: <as passed, or default>
    loaded: []     # the memory, settings or instruction files it read beyond its brief
    seconds: <N>
    tokens: <N>    # input and output, as the client reports them
    marked: {must: <N>, should: <N>, nit: <N>}
    musts_accepted: <N>    # of the musts it marked, those not rejected
    musts_rejected: []     # each with its file, line and reason
    classes: {}    # its accepted musts, counted by class
fixes: []          # the coordinator's own fixes, each with the pass that read it
rebases: <N>
ci:                # from the checks run of the merge's push to main, once it has ended
  linux: <green; or red, each failed job, and whether it failed in the run before; or none>
  macos: <green; or red, each failed job, and whether it failed in the run before; or none>
paired: null       # for a paired order: findings, key and verdicts
escapes: []        # filled at the result
```

For a paired order, `paired` holds `findings`, the merged list, each finding with its number,
its fields as [the report's schema](../work-orders/verify.schema.json) gives them and the
coordinator's judgment; `key`, from number to reader; and `verdicts`, each reader's verdict.

### Verdicts

The result is a dated file linked from A77's row, written once A77's last order has merged or
stopped and seven days have passed since the last merge it counts.

- **Claim 1** holds when no merge of A77's orders turns a job of the `checks` workflow red on
  `main`, on Linux or in the macOS jobs it has run since order 187; every record is complete; and
  A77's orders have no more escapes per merged order than the baseline. A merge *turns a job
  red* when the job fails in the run of that merge's push to `main` and did not fail in the run
  of the push before it; a job left red by an earlier merge is that merge's, not a later one's.
  The red counts once its run has ended, after the merge, even when a follow-up fixes it; an
  order that did not merge turns nothing red. An *escape* is a fault in an order's merged
  change, found within seven days of the merge by a later Record, a commit that fixes it, CI on
  `main` or a finding in the first build's open log, and traced to that change. One found by a
  review on Claude Fable that the owner starts by hand (ruling 79, point 1) is reported apart and
  not counted. The baseline's escapes are recounted at the result by the same rule.
- **Claim 2** is judged for each group on its five paired orders. A fault is must-level when the
  coordinator judged it a must, and a reader found it when any of its findings is in it, at any
  severity. The claim holds for the group when the Opus reader found every must-level fault
  that the Codex verifier found, and at least one more. A group with fewer than five paired
  orders is not shown.
- **What follows is the owner's.** The review would drop the Codex read for a group in which,
  over five paired orders, it found no must-level fault that the Opus reader missed. Ruling 79,
  point 2, fixes who reads each order, so the result puts that to the owner with the counts and
  changes nothing by itself. No estimate of the faults both readers missed is made: from two
  readers it is unreliable, as FR-498's verifier found.

The result also gives, per paired order, each reader's findings, musts accepted and rejected,
the faults both found, minutes and tokens; and per order, the reads, the rounds and the
coordinator's fixes. The coordinator's spend is its own session's tokens from A77's first
dispatch to its last merge, divided by the orders merged; it counts all of its work in that
time, but not the readers it starts, whose tokens their passes count. A script beside the
runners reads the spend from the session's transcripts, which hold private text, so it stays
outside the tree. The result gives the command of each script it relies on, each private value
a variable. Until the transcript script exists, the spend is not shown.

### What it cannot show

The coordinator who judges the findings runs on Opus, the paired reader's model: removing the
reader's name hides the reader, not its style. Five orders in a group show only large
differences. The paired reader is held to the verifier's brief and limits, so it does less than
the Opus reader before the merge, which runs the checks. Each round's diff is discarded after
the merge (D63, as the method runs it), so a judgment can be checked later only against the
quotes its record keeps. A77's baseline is work by Codex workers too, so claim 1 shows whether
the work keeps its level, not how it compares with work the coordinator does by itself. Nor does
A77 answer yet whether the admin session spends less: the spend per order is a first baseline,
compared with nothing (ruling 80). The baseline's merges made before ruling 80 waited for CI, and
A77's wait only for the local gate: a fault that CI stopped before such a merge would be, in
A77, a red after the merge, which claim 1 counts, and also an escape.
````

3. **The method page points to it.** In `docs/implementation/METHOD_AS_RUN.md`, add this
   paragraph as the last of the section "## 3. The Codex verifier and the rounds": after the
   section's last paragraph and before the heading "## 4. Continue from a report", with one blank
   line before it and one after it. Nothing else in the page changes.

```markdown
For an order of A77's sample whose group still lacks five paired orders, a fresh Claude Opus
reader also reads round 1's checkpoint, given only what the Codex verifier is given and running
nothing that writes; neither sees the other's report. Such an order is dispatched with
`ROUNDS=1`. The coordinator judges both reports' findings with their readers' names removed, and
the accepted musts and shoulds of both start round 2 through the continue runner. Each of A77's
orders keeps a record in `docs/work-orders/NNN-a77.yaml`.
[A77's measures](WORK_ORDERS.md#a77s-measures-registered-before-the-first-paired-read) give the
rules.
```

4. **A77's row in `ROADMAP.md`** (line 70). Three cells change; the other cells, Source among
   them, do not.
   - *Question it settles.* After its last words, "does the admin session spend less?", add a
     space and this text. Ruling 80, point 6, gave the question this second part:

```markdown
Since [ruling 80](docs/proposals/ruling-80/README.md), point 6, also: on one diff under one brief, does a fresh Opus reader find the must-level faults that the Codex verifier finds, and more?
```

   - *Done when.* The cell becomes the text below. Its first sentence is the cell as it was:
     A77's question comes from ruling 48, so the counting rule is added beside the old words
     rather than in their place; the owner, told as FR-272's verifier asks, keeps both the rule
     and the question's second part (ruling 80, point 6). Since merges no longer wait for CI
     (ruling 80, point 10), the rule counts the reds after a merge.

```markdown
Each order's row in [work orders](docs/implementation/WORK_ORDERS.md#the-record): the order, verifier findings, admin corrections and recorded times; reviewed after five orders. From DATE, its verdict is the one [registered in work orders](docs/implementation/WORK_ORDERS.md#a77s-measures-registered-before-the-first-paired-read), over the next 20 orders dispatched, merged or not: the work is as good when every order's record is complete, no more faults escape per merged order than from the 20 orders merged before, and no merge turns a job of `main`'s CI red, counted once the merge's run on `main` has ended, since merges wait for a local gate and no longer for CI (ruling 80, point 10); and at round 1 of five paired orders in each group, code and text, a fresh Opus reader given the Codex verifier's diff and brief finds every must-level fault the verifier finds, and more. What the paired reads change is the owner's (ruling 79, point 2, and ruling 80, point 6)
```

   - *State.* Ruling 80's record ended the cell with a note that this order rewrites the row.
     Replace that note's last words, "order 179 rewrites this row under it", with the text
     below; the semicolon before them stays. The rest of the cell does not change.

```markdown
its measures, the coordinator's spend among them, were [registered](docs/implementation/WORK_ORDERS.md#a77s-measures-registered-before-the-first-paired-read) on DATE by work order 179, before the first paired read; not run
```

## Scope

Change only these three files, and in each only what the decisions name:

- `docs/implementation/WORK_ORDERS.md`, the new section (decision 2);
- `docs/implementation/METHOD_AS_RUN.md`, one paragraph (decision 3);
- `ROADMAP.md`, A77's row (decision 4).

Create no `docs/work-orders/NNN-a77.yaml` and no script. Do not change `docs/work-orders/verify.md`
or `verify.schema.json`, `AGENTS.md`, `IN_FORCE.md`, any ruling, the review's files, the handover,
experiment 028, `tools/`, any order's file, or the record in `WORK_ORDERS.md`.

## Done when

From the repository root, each passes:

```sh
python3 tools/check_roadmap.py          # Errors: 0
python3 tools/check_docs.py             # Errors: 0
python3 tools/check_private_paths.py    # Errors: 0
git diff --check                        # prints nothing
git diff --name-only    # exactly ROADMAP.md, docs/implementation/METHOD_AS_RUN.md and docs/implementation/WORK_ORDERS.md
git diff --numstat      # ROADMAP.md 1 added and 1 deleted; the other two files 0 deleted
```

Then each of these prints what its comment says. The commands that start with `tr` read the file
with its line breaks as spaces. Those marked "on main" print something else on a `main` that
holds orders 170 and 173 and ruling 80's record but not this change, so each fails with the
change removed.

```sh
W=docs/implementation/WORK_ORDERS.md
M=docs/implementation/METHOD_AS_RUN.md
S="/^## A77's measures, registered before the first paired read\$/,/^## The record\$/"
A='a77s-measures-registered-before-the-first-paired-read'
grep -c "A77's measure" $W                                                       # 1; on main, 0
grep '^## ' $W | grep -A1 '^## Running the orders through Conduit$' | tail -1   # ## A77's measures, registered before the first paired read; on main, ## The record
awk "$S" $W | grep -c '^### '                                                     # 6
awk "$S" $W | awk 'length > 100' | wc -l                                          # 0
awk "$S" $W | grep -c 'DATE'                                                      # 0
awk "$S" $W | sed -n '/^```yaml$/,/^```$/p' | sed '1d;$d' | /usr/bin/python3 -c 'import sys, yaml; print(len(yaml.safe_load(sys.stdin)))'   # 15
tr -s ' \n' '  ' < $W | grep -c "Neither sees the other's report"               # 1
tr -s ' \n' '  ' < $W | grep -c 'are one fault when one change to the files closes both'   # 1
tr -s ' \n' '  ' < $W | grep -c 'puts that to the owner with the counts and changes nothing by itself'   # 1
tr -s ' \n' '  ' < $W | grep -c 'is the work as good, and does the admin session spend less?'   # 1; on main, 0
tr -s ' \n' '  ' < $W | grep -c 'so a red is counted after the merge, not prevented'   # 1; on main, 0
tr -s ' \n' '  ' < $W | grep -c 'did not fail in the run of the push before it'   # 1; on main, 0
awk "$S" $W | grep -c '(../proposals/ruling-80/README.md)'                         # 1
awk '/^## 3\. /,/^## 4\. /' $M | grep -c "WORK_ORDERS.md#$A"                      # 1; on main, 0
grep -c "$A" $M                                                                   # 1; on main, 0
grep '^| A77 |' ROADMAP.md | grep -o "$A" | wc -l                                 # 2; on main, 0
grep '^| A77 |' ROADMAP.md | grep -o -e 'does the admin session spend less? Since' -e 'reviewed after five orders. From' -e 'with no comparison; its measures' -e 'no longer for CI (ruling 80, point 10)' -e 'What the paired reads change is the owner' | wc -l   # 5; on main, 0
grep '^| A77 |' ROADMAP.md | grep -c 'order 179 rewrites this row'                 # 0; on main, 1
grep '^| A77 |' ROADMAP.md | grep -c 'DATE'                                       # 0
```

`check_docs.py` checks that each new link and anchor resolves. `/usr/bin/python3` is used because
it has PyYAML. Do not stage anything. Report each choice you make as your own, and every check's
last line.

The admin then has a fresh, read-only Opus reader read the diff before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2), and the merge waits for the local gate
(ruling 80, point 10). A77's orders are those dispatched after
it merges, so an order dispatched before then is not one of them. Before the first paired read,
the admin has the script that merges two reports blind.
