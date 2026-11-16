# Work order 170: A78's pre-registration: the claim, the pass conditions and the measures for orders 2 and 3

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order changes three documents. It has no
network access and needs none. It needs no cargo command; if you run one, run one at a time. Do
not run `conduit`, the dogfood harness or anything else against a store. Never print a whole large
file: read `ROADMAP.md` and `experiments/README.md` with `grep -n` and `sed -n`. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a key, a token or any credential.

## What and why

[Order 161](161-a78-stage-1-evidence.md), merged as pull request 287, brought A78's stage 1 into
`experiments/028-a78-dogfood/`: order 042's records, the method as run, and a dated observation
that is "not a pass or a fail: no measure was fixed before order 042 ran (ruling 45)". The
README still says: "A78's claim, pass conditions and measures for later orders are not
registered here yet." The full review of 2026-11-14 (order 12 of
[its orders](../proposals/full-review/orders.md)) asks for them before the next order runs through
Conduit:

- [FR-337](../proposals/full-review/findings/P06-dogfooding.md#fr-337) and
  [FR-275](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-275), as their verifiers
  corrected them: the overhead per turn is held to a bound stated beforehand, no order is paired
  with a plain-runner order, and what a harness checks under one user is observed, never counted
  as shown;
- [FR-531](../proposals/full-review/findings/G6-outside-the-tree.md#fr-531)'s counting rule, as
  its verifier corrected it: every turn of the harness's log found in the records is the pass
  condition, and the link from a request to the one that replaced it is a measure;
- [FR-499](../proposals/full-review/findings/G4-who-verifies.md#fr-499)'s verifier slot, added at
  order 050's re-initialization, as its verifier corrected it.

The review merged these into one text,
[A78: the conditions per order](../proposals/full-review/experiments.md#a78-the-conditions-per-order).
The coordinator's [decisions](../proposals/full-review/decisions.md) bound it: frictions logged
before the next order, and no speed condition (D54); no claim about access (D57); guards that
answer held, broken or unknown (D58); orders 4 and 5 left to the owner (D59); every result under
one operating-system user opening with what it cannot show (D60); no Claude slot before D58's
guards and A22's open case (D61).

Two things changed after the review, and this order applies both:

- [Ruling 78](../proposals/ruling-78/README.md), point 6: no actor records that an unknown effect
  did not occur, and the way on is a new request. The review's fifth condition, "Every unknown
  outcome is reconciled, with what was checked", can then no longer be met for a turn whose work
  did not happen. It becomes "no turn is recorded as not having occurred", checked on the
  `invocation-ended` record that a reconciliation writes (`engine/crates/core/src/requests.rs`,
  lines 1313 to 1319). What followed each unknown turn becomes a measure.
- Ruling 78 ended the hold on re-pinning the dogfood engine to `main`. The engine is still at
  `38c0b4f`. Order 2 runs on the re-pinned engine, so the registration records that setup and
  times the restart again before order 2, as FR-275's verifier asks. The bound is fixed now, from
  order 042's restarts. Orders 156 and 162, which the re-pin was to wait for, are both merged.

028's README, its index row and [the handover](../implementation/HANDOVER.md) call orders 1 to 3,
which root sends, stage 1. This order keeps that name and registers orders 2 and 3 by number. By
the handover's plan they are 046 and 047. Order 042's harness logged only each run's sum of
restart seconds, and its guards tripped or noted rather than answering held, broken or unknown.
So the registration also records, before order 2, which of the harness's log lines give each
measure; a measure that none gives is not shown, and the order does not pass
([ruling 52](../proposals/ruling-52/README.md)). The registration makes these choices: the bound
of 240 seconds; the seven conditions, among them two strict readings (a lost turn counts unless
shown to be the account's or the plain runner's too, and `client failure: other` names no
cause); the counting rule; and order 042 kept as order 1. They are the coordinator's under
[ruling 45](../proposals/ruling-45/README.md), logged for the owner to strike (ruling 48). The
order changes three documents that bind nothing. It does not re-pin the engine, change the harness
or log a finding. It does not register orders 4 and 5 or the review's probes.

## The decisions it applies

Line numbers are those of `main` at `60126251`. If a line has moved, find it by its words. Each
block below is written word for word, with DATE replaced by the date you write it, as
YYYY-MM-DD. Do not rewrap a block: its lines are at most 100 characters, and each link sits
whole on one line, because `check_docs.py` checks a link only when its text and its target share
a line. The README is two directories below the root, as this order is, so its links are written
as they serve from the README.

1. **The registration is a new last section of the README.** At the end of
   `experiments/028-a78-dogfood/README.md`, after the section "What it cannot show", add one blank
   line and then this block.

```markdown
## Orders 2 and 3: registered before order 2 runs

Registered on DATE by work order 170, before order 2 runs through Conduit
([ruling 45](../../docs/proposals/ruling-45/README.md)). It holds for orders 2 and 3, which root
sends. Orders 4 and 5, which a coordinator Agent sends, are registered before order 4 runs, once
the owner has answered what they test (D59). This text changes only by a dated amendment, made
before the first order it changes runs. Its choices, and the D-numbers it cites from
[the full review's decisions](../../docs/proposals/full-review/decisions.md), are the
coordinator's, each open to the owner's strike
([ruling 48](../../docs/proposals/ruling-48/README.md)).

### Claim

Running a work order through Conduit's Agents, as
[ruling 70](../../docs/proposals/ruling-70/README.md) asks, costs a bounded set-up time per turn,
loses no turn that the plain runner would have kept, and leaves in the engine's own records which
turns were asked for, by whom and how each ended.

It claims nothing about access (D57), nothing about Agents kept apart (D60) and nothing about how
long an order takes: no condition is set on its speed (D54), and no order is paired with an order
on the plain runner
([FR-275's verifier](../../docs/proposals/full-review/findings/P01-work-orders-lanes.md#fr-275)).

### Setup, recorded before order 2's first turn

The coordinator records each of these in this section, by a dated amendment merged before order
2's first turn, and order 2's result repeats each, with the command that shows it where one does:

- the commit of `main` that the dogfood engine is re-pinned to from `38c0b4f`, now that
  [ruling 78](../../docs/proposals/ruling-78/README.md) has ended the hold on the re-pin, and that
  it holds orders 156 and 162, which
  [FR-336](../../docs/proposals/full-review/findings/P06-dogfooding.md#fr-336) and the settlement
  of [finding 4-23](../../docs/proposals/first-build/log-increment-4.md) put before the re-pin:
  `git merge-base --is-ancestor b1f6f8ca <commit>` and the same for `60126251` each exit 0. Until
  the later order of ruling 78, point 6, removes it, that commit still lets `reconcile.v1` record
  a fenced turn as not having occurred; condition 5 forbids it;
- the branch and commit that the `.v2` Agent archives were built from, and each archive's
  version and digest;
- the Fleet's documents, as `inputs/fleet/` holds them or as changed, and the Agents' deadlines;
- whether the store was initialized again at the re-pin;
- the harness as run: the commit or digest of its scripts, and the lines its log and its guard
  files write for each turn it sends through Conduit, repair turns included, with its role and
  round; for each restart's seconds; for each guard's answer; and for the cause of each turn that
  does not succeed. A measure that no line gives is not shown;
- three restarts of the re-pinned engine on that store, with no turn sent, each timed as set-up
  seconds are below, and their median. If any of the three is over 240 seconds, condition 2's
  bound is amended here before order 2 runs, with the three times, to the longest of them plus a
  quarter of it, rounded up to a multiple of 10 seconds.

### Measures

Each is counted from the order's directory (below) unless it names another source, and each
number in a result has beside it the command that gives it, as in
[the results of 2026-11-12](results-2026-11-12.md) (D13).

A turn of the order is an `invocation-dispatched` record in `records.jsonl` whose operation ends
in `#turn.v1` and that is not in the export the harness made at the order's first restart; the
result gives that export's last record's time, with its command. A turn of the harness's log is
one that the harness sent through Conduit, not one run by `VIA=exec` or `ON_LIMIT=exec`. A turn
of the log is found in the export when it pairs with a turn of the order: on each Agent, the
log's turns in log order pair one to one with the order's turns in dispatch order, the first
with the first.

Per turn, one row for each turn of the order:

- the Agent, named from the access decision's `tool` and the Fleet's instances as the results of
  2026-11-12 name it, and the turn's role and round, from the harness's log;
- set-up seconds: the seconds of the engine restart before the turn, from the harness's stop of
  `serve` until root's two turn tools answer
  ([step 2 of the harness](inputs/harness.md#what-conduit-ordersh-does)), as the harness's log
  gives them for that restart. Order 042's harness gave only each run's sum, in `restartSeconds`.
  A repair turn is sent with no restart, and its set-up seconds are 0;
- work seconds: from the turn's `invocation-dispatched` record to its end, the first
  `invocation-ended` record of that attempt whose `reconciled` is not true, or "no end recorded";
- the outcome, from that end, and for a turn not `succeeded`, the cause that the harness's log
  gives from the Agent package's error or an engine diagnostic;
- whether the report was valid against its schema (the order's for a worker's report,
  `verify.schema.json` for a verifier's) at the first try, whether a repair turn was sent, and
  whether the report was valid after it;
- each guard's answer after the turn, held, broken or unknown, and its notes.

Per order:

- the turns of the harness's log and the turns of the order; each turn of the log not found in
  the export; and each field of a found turn that the records do not give;
- the rounds; the seconds of each round, from its first dispatch to its last end; and for each
  verifier pass, its musts, shoulds and nits, and the musts the coordinator accepted;
- each turn that ended `unknown`, and what followed it on its Agent: a `reconcile.v1` that
  recorded it as having occurred, with what was checked; a repeat under the same key (a later
  attempt of the same invocation); a new turn request; or nothing. Whether any field of the
  records links a new request to the turn it replaced is reported, not judged
  ([FR-531's verifier](../../docs/proposals/full-review/findings/G6-outside-the-tree.md#fr-531));
- the harness's interventions, each kind counted: repair turns; restarts, one for each fresh
  conversation and any other apart, with its reason; the processes the harness's sweep listed,
  and those it stopped; watchdog fires; turns run again on the plain runner (D54); and the
  coordinator's own steps in the slot or the harness during the order;
- what Conduit refused, opened and closed from the order's first dispatch to its last end:
  access decisions whose `permitted` is false, and the core's `open.v1` and `close.v1`
  invocations that succeeded. A zero says that the order tested no access (D57);
- the lines of the Fleet's documents added or removed for the order, 0 if none changed;
- the redactions made before committing, by class: path, name, other;
- the store's size, by `du -sk`, before the order's first restart and after its last turn, and
  its change divided by the order's invocations in the export, with the command; the order's
  invocations are those dispatched in `records.jsonl` and not in the export of its first restart.
  The time each commit takes is not measured: the engine does not write it yet.

Observed, and never counted as shown, while every Agent runs as one operating-system user (D60;
[FR-337's verifier](../../docs/proposals/full-review/findings/P06-dogfooding.md#fr-337)). A guard
that held shows nothing; a guard that answers broken still fails condition 4:

- the writes outside the slot, and the writes by the verifier, that the guards and the sweep
  saw; nothing else is looked for;
- the processes of a turn still alive after its failed end or after the run's stop, as the
  sweep lists them (finding 4-23);
- what the verifier could read of the worker's conversation: where the worker's client keeps it,
  and whether the verifier runs as the same user with the same home, by the Fleet's documents and
  the Agent package's README. No conversation file is opened.

### Pass conditions

An order passes when all seven hold. A condition that the order's directory, and the sources the
condition names, cannot show is "not shown", and an order with a condition not shown does not
pass. In conditions 1, 2, 3 and 7, a turn is a turn of the order and a turn of the log is a turn
of the harness's log, as Measures defines both. The claim holds for orders 2 and 3 when both
pass.

1. **No turn is lost to a fault of the engine, the Agent package or the harness.** A turn is
   lost when its call ends `failed` or `unknown`, the harness's watchdog ends it, or its report
   is still invalid after its one repair turn. A lost turn counts against this condition unless
   the Agent package names its cause `usage-limit` or `rate-limit`, limits that the account sets
   on a plain `codex exec` turn as well, or a plain `codex exec` turn that ran at the same time,
   on the same machine and account, ended with the same cause, by a log line that the result
   cites. A loss of the network is judged like any other cause (finding 4-21).
2. **Every turn's set-up seconds are at most 240.** The bound is about a quarter above order
   042's round-4 restarts, the only ones timed one by one: 171 to 191 seconds, by the
   coordinator's account (finding 4-24), and 181 seconds on average over that run's two
   restarts, by `inputs/measures.jsonl`.
3. **Every report is valid against its schema** (the order's for a worker's report,
   `verify.schema.json` for a verifier's), at the first try or after its one repair turn, and
   every turn that ends `failed` or `unknown` has its cause named in the harness's log, by the
   Agent package's error or an engine diagnostic. `client failure: other`, or no cause, names
   none.
4. **No guard answers broken, and none is still unknown when the order merges.** A trip whose
   cause is that the guard could not read what it checks, as when GitHub's refs could not be read
   in order 042 (finding 4-25), is unknown, and that guard runs again before the next turn (D58).
   Any other trip is broken; after a turn, a guard that neither trips nor answers unknown held. A
   note is not a trip.
5. **No turn is recorded as not having occurred** (ruling 78, point 6): no `invocation-ended`
   record whose `reconciled` is true and whose `outcome` is `failed` names a turn of the order.
6. **Each friction is in the first build's open log, now
   [increment 4's](../../docs/proposals/first-build/log-increment-4.md), before the next order
   goes out through Conduit** (D54), as a new row or in the row of the same finding, and the
   result names the rows. A friction is each lost turn, each repair turn, each guard broken or
   unknown, each intervention but the one restart for each fresh conversation, and each fault
   found during the order in the engine, the Agent packages or the harness.
7. **Every turn of the harness's log is found in the export** with its caller, its Agent (the
   target's ID, named as the per-turn measures name it), its dispatch, its end and its outcome,
   as the per-turn measures take them. A turn not found, or found without one of these, is logged
   as a finding of the first build against REC-001, or against the promise that should have
   recorded it (FR-531's verifier).

### What counts, and what each outcome changes

- From order 2 on, an order counts toward A78's five only once its directory holds its scrubbed
  records export, the harness's log, its measures, its reports and its dated result, whatever
  its verdict. This changes what counts, and is said here, as ruling 45 asks (FR-531's
  verifier). Order 042 counts as order 1 under the register row's own words: it ran through
  Conduit, and its frictions are findings 4-21 to 4-27. This directory lacks its reports and the
  harness's log because the rule was fixed after it ran.
- An order's directory is a directory here named as the order's file, without `.md`. It holds
  `records.jsonl`, the whole export that the harness makes after the order's last run;
  `harness.log`, the harness's log of each run of the order; `measures.jsonl`, the harness's
  lines for the order; `reports/`, each report the harness read, named by role and round;
  `guard/`, the harness's guard files, notes included; the Fleet's documents, if the order
  changed them; and `results-YYYY-MM-DD.md`, dated by the day of the order's last turn, in UTC.
- Before anything is committed, each private value is replaced by a placeholder that the result
  lists, as this README's table does, and `python3 tools/check_private_paths.py` and the pull
  request's secret scan pass.
- Each result opens with "Not binding.", then the first paragraph of
  [What it cannot show](#what-it-cannot-show), then its verdict: passed, or each condition failed
  or not shown.
- A failed condition takes no order off Conduit: A78 stays on the core chain (D54), and taking
  it off is the owner's. Each lost turn is run again on the plain runner (D54).

### The Claude verifier slot

At order 050's re-initialization of the store, a verifier slot that is a Claude Code Agent on
Claude Opus, the coordinator's model under
[ruling 79](../../docs/proposals/ruling-79/README.md), is added beside `verifier-1`, which stays
for A77's paired reads
([FR-499](../../docs/proposals/full-review/findings/G4-who-verifies.md#fr-499), as its verifier
corrected it). It is added only after D58's guards and A22's open case have run (D61), and its
reach over the worker's tree waits for the owner's answer on what orders 4 and 5 test (D59). Its
turns draw on the owner's primary Claude allowance. They are measured as a verifier turn is
above; the registration of the first order it reads, made before that order runs, says how they
are judged, and each result says which verifier read each pass.

### Not registered here

- Orders 4 and 5 (D59), with the review's choice that their coordinator Agent runs on the
  coordinator's model (FR-499), and any order after order 3 that root sends if batch 2 is late,
  which is the owner's.
- The probes of A78 in
  [the review](../../docs/proposals/full-review/experiments.md#a78-the-conditions-per-order): P1,
  cuts of the network under the `.v2` wrapper and under a plain launch, and P2, a lost caller,
  before order 5; each is registered before it runs. P3, the process tree, the review moved to
  experiment 022 and A7, and P4, upgrading a signed-in Agent's package, it did not register.
```

2. **The README's opening points to it.** The paragraph on lines 4 to 8, which ends "are not
   registered here yet.", becomes this block. Only its last sentence changes.

```markdown
This is A78's directory in the [roadmap's register](../../ROADMAP.md#experiments-and-probes).
Stage 1 is orders 1 to 3, which root sends through turns on Agents of the `conduit-core` Fleet
([ruling 70](../../docs/proposals/ruling-70/README.md);
[ruling 61, B12](../../docs/proposals/ruling-61/README.md)). A78's claim, pass conditions and
measures for orders 2 and 3 are
[registered below](#orders-2-and-3-registered-before-order-2-runs), before order 2 runs; those
for orders 4 and 5 are registered before order 4 runs.
```

3. **What it cannot show opens with D60's statement.** In that section, before its paragraph on
   line 96, which begins "The records say which requests were made", insert this block and one
   blank line. The paragraph on line 96 does not change.

```markdown
Every result here is obtained with every Agent under one operating-system user: the owner's now,
and the dedicated macOS user of [ruling 75 B3](../../docs/proposals/ruling-75/README.md) later.
Under one user, an Agent that runs code can read, use or replace another Agent's login and act as
root's Interface, so no result here shows Agents kept apart
([the full review's D60](../../docs/proposals/full-review/decisions.md)). A guard that held is
not proof either: the guards read a slot that the Agents they check can write
([finding 4-22](../../docs/proposals/first-build/log-increment-4.md)).
```

4. **The Files table names the directories to come.** After its last row, `inputs/plan.md`'s
   (line 92), add this row:

```markdown
| `NNN-name/`, from order 2 on | One directory per order: its scrubbed records export, the harness's log and measures, its reports and its dated result, as the registration below says |
```

5. **A78's row in `ROADMAP.md` names the claim, the pass conditions and the measures** (line 71).
   Three cells change; the other cells do not.
   - *Question it settles.* After its last words, "no slot retires", add a full stop, a space and
     this text:

```markdown
Claim, for orders 2 and 3: running an order through Conduit's Agents costs a bounded set-up time per turn, loses no turn the plain runner would have kept, and leaves in the engine's records which turns were asked for, by whom and how each ended
```

   - *Done when.* The cell becomes the text below. Its first sentence is the cell as it was:
     changing the done-when's own terms is the owner's
     ([FR-374](../proposals/full-review/findings/P08-progress.md#fr-374)'s verifier), so the
     counting rule is added beside it, and the conditions are said to decide no count.

```markdown
Five orders run through Conduit, each friction logged as a finding of the first build. From order 2 on, an order counts only once its directory in experiment 028 holds its scrubbed records export, the harness's log, its measures, its reports and a dated result, whatever its verdict. An order's verdict decides no count: under [its registration](experiments/028-a78-dogfood/README.md#orders-2-and-3-registered-before-order-2-runs) it passes when no turn is lost to a fault of the engine, the Agent package or the harness; every turn's set-up is at most 240 s; every report is schema-valid, with at most one repair turn, and the cause of every failed or unknown turn is named; no guard is broken, or still unknown at merge; no turn is recorded as not having occurred; each friction is logged before the next order; and every turn of the harness's log is found in the records. Measures: per turn, set-up and work seconds, outcome and cause, the report's validity and repair, and the guards' answers; per order, rounds and musts, each unknown turn and what followed it, the harness's interventions, what Conduit refused, opened and closed, the Fleet's lines changed, redactions and the store's size
```

   - *State.* The words "the next waits for the dogfood engine's re-pin to `main`" become the
     text below. The rest of the cell, its opening "in progress:" included, does not change.

```markdown
orders 2 and 3 [pre-registered there](experiments/028-a78-dogfood/README.md#orders-2-and-3-registered-before-order-2-runs) on DATE by work order 170, the next waiting for the dogfood engine's re-pin to `main`
```

6. **The experiments index says so.** In `experiments/README.md`, in 028's row (line 45), add a
   space and this sentence at the end of the cell Current evidence, after its last sentence,
   which ends with the link to the results.

```markdown
Orders 2 and 3 [pre-registered](028-a78-dogfood/README.md#orders-2-and-3-registered-before-order-2-runs) on DATE by work order 170; not run.
```

## Scope

Change only these three files, and in each only what the decisions name:

- `experiments/028-a78-dogfood/README.md` (decisions 1 to 4);
- `ROADMAP.md`, A78's row (decision 5);
- `experiments/README.md`, 028's row (decision 6).

Change nothing under `experiments/028-a78-dogfood/inputs/`, and do not change
`experiments/028-a78-dogfood/results-2026-11-12.md`, the handover, increment 4's log, the review's
files, `tools/` or any order's file.

## Done when

From the repository root, each passes:

```sh
python3 tools/check_roadmap.py          # Errors: 0
python3 tools/check_docs.py             # Errors: 0
python3 tools/check_private_paths.py    # Errors: 0
git diff --check                        # prints nothing
git diff --name-only    # exactly ROADMAP.md, experiments/028-a78-dogfood/README.md and experiments/README.md
```

Then each of these prints the number beside it. The commands that start with `tr` read the file
with its line breaks as spaces. The five marked "on main" print another number with the change
removed.

```sh
R=experiments/028-a78-dogfood/README.md
S='/^## Orders 2 and 3: registered before order 2 runs$/,0'
A='orders-2-and-3-registered-before-order-2-runs'
grep -c '^## Orders 2 and 3: registered before order 2 runs$' $R      # 1; on main, 0
awk "$S" $R | grep -c '^### '                                          # 7
awk "$S" $R | grep -c '^[1-7]\. \*\*'                                  # 7
awk "$S" $R | awk 'length > 100' | wc -l                                # 0
tr -s ' \n' '  ' < $R | grep -c 'Every turn.s set-up seconds are at most 240\.'          # 1
tr -s ' \n' '  ' < $R | grep -c 'whose .reconciled. is true and whose .outcome. is .failed. names a turn'   # 1
grep -A2 '^## What it cannot show$' $R | tail -1 | grep -c '^Every result here is obtained'   # 1; on main, 0
tr -s ' \n' '  ' < $R | grep -c 'measures for orders 2 and 3 are .registered below'      # 1; on main, 0
grep -c -e 'not registered here yet' -e 'did not occur' $R            # 0; on main, 1
grep '^| A78 |' ROADMAP.md | grep -o "$A" | wc -l                     # 2; on main, 0
grep '^| A78 |' ROADMAP.md | grep -o -e 'Claim, for orders 2 and 3:' -e 'An order.s verdict decides no count' -e 'Measures: per turn' | wc -l   # 3
grep -c "Orders 2 and 3 \[pre-registered\](028-a78-dogfood/README.md#$A) on" experiments/README.md   # 1
```

`check_docs.py` checks that each new link and anchor resolves. The admin then has a fresh,
read-only Opus reader read the diff before it merges (ruling 79), and merges it before the next
order runs through Conduit. Do not stage anything. Report each choice you make as your own, and
every check's last line.

## Record

Run on 2026-11-15 on the Mac from `main` `5f9fcbe8`: the worker (131 s) applied all six decisions
to the three files, and a fresh Codex verifier (159 s) found nothing to change in one round.
028's README now ends with the registration of orders 2 and 3: the claim, the setup recorded
before order 2, the measures, the seven pass conditions, what counts, the Claude verifier slot
and what is not registered. Its opening points to it, What it cannot show opens with D60's
statement, and its Files table names the order directories to come. A78's row in `ROADMAP.md`
gains the claim, the counting rule beside the unchanged done-when and the pre-registration in
its State, and 028's row in the experiments index says so.

A fresh read-only Opus reader then verified the order (ruling 79, point 2). It checked the
order's eight blocks against the branch byte for byte by script, ran every done-when command
from the root (the five "on main" counts give 0, 0, 0, 1, 0 at `5f9fcbe8`), checked with
`git merge-tree` that the branch merges cleanly with the newer `main`, and read the cited
findings, rulings, decisions and lines of `requests.rs`. It found no must, and ten shoulds
against the registration's own text, which the coordinator wrote. All ten were applied before
merge, to the README and to decision 1's block alike:

- a turn of the order is a `#turn.v1` dispatch in `records.jsonl` that is not in the export of
  the order's first restart, and a turn of the log is one the harness sent through Conduit, not
  by `VIA=exec` or `ON_LIMIT=exec`; the per-turn rows, the per-order count and conditions 1, 2,
  3 and 7 are read of the order;
- the order's directory holds `guard/`, the setup records the guard files' lines as well as the
  log's, and condition 4 says that any other trip is broken and that a guard that neither trips
  nor answers unknown held;
- condition 2 is amended if any of the three re-timed restarts, not their median, is over 240 s;
- the setup is recorded in the registration by a dated amendment merged before order 2;
- a verifier's report is checked against `verify.schema.json`, in the measure and condition 3;
- a condition is "not shown" only when neither the directory nor the sources it names show it;
- condition 1's exception for the plain runner needs a plain `codex exec` turn at the same time,
  on the same machine and account, that ended with the same cause, by a log line the result
  cites;
- a turn's end is the first `invocation-ended` record of its attempt whose `reconciled` is not
  true, and a repeat under the same key is among what can follow an unknown turn;
- the store's change per invocation of the order is measured;
- condition 2's rationale says that only round 4's restarts were timed one by one.

The coordinator's choices in these fixes, each open to the owner's strike (ruling 48), fix each
rule before any of order 2's data is seen: a new bound, if one is needed, is the longest
re-timed restart plus a quarter of it, rounded up to a multiple of 10 s; on each Agent, the
log's turns and the order's turns pair first with first; the order's invocations are those
dispatched in `records.jsonl` and not in the export of its first restart; and a turn's outcome,
and its end and outcome in condition 7, are those of its first unreconciled end. The definitions
stand before the per-turn list rather than inside its lead-in, and conditions 1, 2, 3 and 7 are
read of the order by one sentence before them, so that condition 2 keeps the words the done-when
counts. Every done-when count is unchanged after the fixes.

A second fresh Opus reader read the coordinator's fixes (ruling 79, point 2) and found no must. Its
one should was applied in both copies: the setup records the log line that names each turn sent
through Conduit, repair turns included, with its role and round.
