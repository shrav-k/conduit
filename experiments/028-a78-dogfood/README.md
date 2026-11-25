# 028 — A78: Conduit runs its own orders
Not binding.

This is A78's directory in the [roadmap's register](../../ROADMAP.md#experiments-and-probes).
Stage 1 is orders 1 to 3, which root sends through turns on Agents of the `conduit-core` Fleet
([ruling 70](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-70/README.md);
[ruling 61, B12](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-61/README.md)). A78's claim, pass conditions and
measures for orders 2 and 3 are
[registered below](#orders-2-and-3-registered-before-order-2-runs), before order 2 runs; those
for orders 4 and 5 are registered before order 4 runs.
Stage 2, on the store initialized again with the Agent packages 0.2.0, or 0.2.1 if it is first
initialized with them, is [registered below](#stage-2-registered-before-its-first-turn).
Stage 3, the multi-account Fleet on the Agent packages 0.2.4, is
[registered below](#stage-3-registered-before-its-first-turn).

## The setup as run

The engine was pinned at `38c0b4f`. It holds order 074, and neither 070 nor 077:

```sh
git merge-base --is-ancestor 6591a064 38c0b4f  # 074: succeeds (exit 0)
git merge-base --is-ancestor 1a34ccf9 38c0b4f  # 070: fails (exit 1)
git merge-base --is-ancestor 575a1bcd 38c0b4f   # 077: fails (exit 1)
```

The `.v2` Agent packages were installed from `wo/076-agents-v2-folder-grant`; they stay unmerged until order 050.
[Order 076's Record](../../docs/work-orders/076-agents-v2-folder-grant.md#record) says where that branch stands; the inputs give no commit for the installed archives.
The [Fleet templates](inputs/fleet/templates.yaml), [instances](inputs/fleet/instances.yaml) and [harness](inputs/harness.md) give `worker-1` a 4-hour deadline and `verifier-1` 1 hour.
Both are `codex.v2`, with effort `high`, account services off and their own tools on, working
in the same slot clone `../slot-1`; setup fills `@SLOT_1@` with the slot's absolute path.
The tree Pool is empty in stage 1, so no tree Resource is granted; in v0 nothing confines the
Agents' own tools ([harness](inputs/harness.md)).

The coordinator's placeholders (`<...>`) and the Fleet templates' own placeholders (`@...@`):

| Placeholder | Stands for | Class | Input files using it |
| --- | --- | --- | --- |
| `<dogfood>` | Harness directory outside the repository | path | `harness.md`, `plan.md` |
| `<main checkout>` | Main checkout, also a source for a locally held branch | path | `harness.md` |
| `<slot-1>` | Slot clone fetched by the admin | path | `harness.md` |
| `<github url>` | Repository clone URL | other | `plan.md` |
| `@SLOT_1@` | Slot's absolute path, filled by setup | path | `fleet/instances.yaml` |
| `@WORKER_DEADLINE@` | Worker deadline; harness says 4 hours | other | `fleet/templates.yaml` |
| `@VERIFIER_DEADLINE@` | Verifier deadline; harness says 1 hour | other | `fleet/templates.yaml` |
| `@CODEX_VERSION@` | Installed agent-codex version; value not stated in manifest | other | `fleet/conduit.yaml` |
| `@CODEX_DIGEST@` | Installed agent-codex archive digest; value not stated | other | `fleet/conduit.yaml` |

Other text in angle brackets in `plan.md`, such as `<schema>`, `<cause>`, `<turnDeadlineSeconds>`
and `<turnDeadlineSeconds×1000>`, is the plan's own notation, not a redaction.

## The harness as run

The [harness README](inputs/harness.md#what-conduit-ordersh-does) gives the exact commands;
its sequence is:

1. Lock the slot, check it is clean with pushes disabled, fetch the order, create its branch and clean the slot, retaining build output.
2. Before each fresh conversation, stop serve, export records, restart with its call timeout, wait for root's turn tools and check startup setup failures.
3. Send run-order.sh's prompt, plus a paragraph on how the turn runs, through root's turn tool for that Agent (`turn-worker-1` or `turn-verifier-1`), in the background, with the watchdog and a check that serve still runs.
4. Extract the report text, strip a code fence and validate against its schema; allow one repair turn in the same conversation, without restarting.
5. Run guards after every turn, including repairs: HEAD and branch unmoved, verifier tree unchanged, nothing pushed, main checkout untouched; checkpoint worker output, then run the verifier and further rounds as its findings require.
6. On exit, stop serve, export final records and append the run's line to `measures.jsonl`.

## The plan

[The plan](inputs/plan.md) is what [ruling 74's record](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-74/README.md)
cites as “The coordinator's plan for A78 under ruling 70”. It plans “one order at a time” on
fixed slots, with root dispatching orders 1 to 3 and a coordinator Agent dispatching orders
4 and 5 after appointment. It plans the Agent packages, the session idle-limit fix, smoke
checks, guards, reports, friction logging and later follow-ups; its earlier order numbers
and own-tools switch remain its words, rather than a description of the final setup.

## The export

The coordinator made [the export](inputs/records-2026-11-12.jsonl) on 2026-11-15 with the
dogfood's pinned engine `38c0b4f`, using `conduit records --store` with serve stopped, before
order 050 creates a new store. By the coordinator's account stderr was empty, the file is
byte-identical to the harness's final export, and it holds no absolute path, user name or
credential. Its filename takes the store's last-write date. From the repository root:

- 344 CloudEvents lines: `wc -l < experiments/028-a78-dogfood/inputs/records-2026-11-12.jsonl`.
- 60 access decisions, 60 dispatches, 60 invocation endings, 104 object changes and 60 request endings: `jq -r .type experiments/028-a78-dogfood/inputs/records-2026-11-12.jsonl | sort | uniq -c`.

## Files

Each input keeps the words it was written with before it was committed. The Fleet files are templates, not rendered copies.
The raw export leaves at the next condensation; the results then link its last version under [keeping evidence](../README.md#keeping-evidence).

| File | Holds |
| --- | --- |
| [README.md](README.md) | Method as run and its limits |
| [results-2026-11-12.md](results-2026-11-12.md) | Dated observation of order 042 |
| [results-2026-11-21-stage-2.md](results-2026-11-21-stage-2.md) | Stage 2's dated results, 2026-11-21: orders 217 to 239, from the records export and the coordinator's call results |
| [inputs/records-2026-11-12.jsonl](inputs/records-2026-11-12.jsonl) | Engine's raw records; IDs, operations, times, attempts, outcomes and access decisions, without request inputs |
| [inputs/measures.jsonl](inputs/measures.jsonl) | Harness's run measures |
| [inputs/fleet/conduit.yaml](inputs/fleet/conduit.yaml) | Fleet project's manifest template |
| [inputs/fleet/instances.yaml](inputs/fleet/instances.yaml) | Fleet, Agents, Pool and Conduit instances |
| [inputs/fleet/templates.yaml](inputs/fleet/templates.yaml) | Agent, Pool and Conduit templates |
| [inputs/harness.md](inputs/harness.md) | Harness README; code stays outside the tree |
| [inputs/plan.md](inputs/plan.md) | Coordinator's plan cited by ruling 74 |
| [stage-2/fleet/conduit.yaml](stage-2/fleet/conduit.yaml) | Stage 2's manifest template, written before 0.2.0 |
| [stage-2/fleet/instances.yaml](stage-2/fleet/instances.yaml) | Stage 2's Fleet, Agents, Pool and Conduit instances |
| [stage-2/fleet/templates.yaml](stage-2/fleet/templates.yaml) | Stage 2's Agent, Pool and Conduit templates |
| [stage-3/fleet/conduit.yaml](stage-3/fleet/conduit.yaml) | Stage 3's manifest template, for the Agent packages 0.2.4 |
| [stage-3/fleet/instances.yaml](stage-3/fleet/instances.yaml) | Stage 3's Pool, login Resources, Agents and Conduit instances |
| [stage-3/fleet/templates.yaml](stage-3/fleet/templates.yaml) | Stage 3's Pool, Conduit, login Resource and role Agent templates |
| [coordinator/README.md](coordinator/README.md) | The coordinator's Rust tooling, the binary `wo` (DEC-174), and its merge queue |
| `NNN-name/`, from order 2 on | One directory per order: its scrubbed records export, the harness's log and measures, its reports and its dated result, as the registration below says |

## What it cannot show

Every result here is obtained with every Agent under one operating-system user, the owner's
([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 2). No dedicated macOS user is
planned: ruling 84, point 1, amends [ruling 75 B3](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-75/README.md), and
isolation is to come later from a sandbox method, whose design states whether a result obtained
inside one shows Agents kept apart. Under one user, an Agent that runs code can read, use or
replace another Agent's login and act as root's Interface, so no result here shows Agents kept
apart ([the full review's D60](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md)). A guard that held is
not proof either: the guards read a slot that the Agents they check can write
([finding 4-22](../../docs/implementation/first-build/log-increment-4.md)).

The records say which requests were made, by whom, and how each ended. They say no more.
What an Agent did in its slot with its own tools is not a Conduit request, so it is not in
the records (CTR-014, PRV-006, ACC-027). In stage 1 every caller is root, so the records do
not test attribution among callers. An order run through Conduit is not evidence about
access unless the results count a refusal, an open or a close. The reports of 042's worker
and verifier are not in this directory.

## Orders 2 and 3: registered before order 2 runs

Registered on 2026-11-15 by work order 170, before order 2 runs through Conduit
([ruling 45](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-45/README.md)). It holds for orders 2 and 3, which root
sends. Orders 4 and 5, which a coordinator Agent sends, are registered before order 4 runs, once
the owner has answered what they test (D59). This text changes only by a dated amendment, made
before the first order it changes runs. Its choices, and the D-numbers it cites from
[the full review's decisions](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md), are the
coordinator's, each open to the owner's strike
([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)).

Amended on 2026-11-19, before order 2 runs through Conduit, by the coordinator's change for
[ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 1. Under "The Claude verifier slot",
the slot was to be added "only after D58's guards and A22's open case have run (D61)"; it is now
added once D58's guards are in place, since A22 no longer runs before any Claude Agent becomes a
dogfood slot. Above this registration, "What it cannot show" no longer names the dedicated macOS
user of ruling 75 B3 as what comes later. No step, measure or pass condition changes. Ruling 84
assigns this correction to an order; the coordinator made it directly, since it changes no step,
measure or pass condition, and the same holds for experiment 027's amendment of the same date.

### Claim

Running a work order through Conduit's Agents, as
[ruling 70](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-70/README.md) asks, costs a bounded set-up time per turn,
loses no turn that the plain runner would have kept, and leaves in the engine's own records which
turns were asked for, by whom and how each ended.

It claims nothing about access (D57), nothing about Agents kept apart (D60) and nothing about how
long an order takes: no condition is set on its speed (D54), and no order is paired with an order
on the plain runner
([FR-275's verifier](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P01-work-orders-lanes.md#fr-275)).

### Setup, recorded before order 2's first turn

The coordinator records each of these in this section, by a dated amendment merged before order
2's first turn, and order 2's result repeats each, with the command that shows it where one does:

- the commit of `main` that the dogfood engine is re-pinned to from `38c0b4f`, now that
  [ruling 78](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-78/README.md) has ended the hold on the re-pin, and that
  it holds orders 156 and 162, which
  [FR-336](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P06-dogfooding.md#fr-336) and the settlement
  of [finding 4-23](../../docs/implementation/first-build/log-increment-4.md) put before the re-pin:
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
  ([FR-531's verifier](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/G6-outside-the-tree.md#fr-531));
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
[FR-337's verifier](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P06-dogfooding.md#fr-337)). A guard
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
   [increment 4's](../../docs/implementation/first-build/log-increment-4.md), before the next order
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
[ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md), is added beside `verifier-1`, which stays
for A77's paired reads
([FR-499](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/G4-who-verifies.md#fr-499), as its verifier
corrected it). It is added only once D58's guards are in place (D61, as ruling 84, point 1,
amends it), and its reach over the worker's tree waits for the owner's answer on what orders 4
and 5 test (D59). Its turns draw on the owner's primary Claude allowance. They are measured as a
verifier turn is above; the registration of the first order it reads, made before that order runs, says how they
are judged, and each result says which verifier read each pass.

### Not registered here

- Orders 4 and 5 (D59), with the review's choice that their coordinator Agent runs on the
  coordinator's model (FR-499), and any order after order 3 that root sends if batch 2 is late,
  which is the owner's.
- The probes of A78 in
  [the review](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/experiments.md#a78-the-conditions-per-order): P1,
  cuts of the network under the `.v2` wrapper and under a plain launch, and P2, a lost caller,
  before order 5; each is registered before it runs. P3, the process tree, the review moved to
  experiment 022 and A7, and P4, upgrading a signed-in Agent's package, it did not register.

## Stage 2: the multi-slot Fleet, written before 0.2.0

Written on 2026-11-19 by work order 210, before order 050n merges the Agent packages 0.2.0. It
registers nothing: no order has run in stage 2, and the claim, the measures and the pass
conditions of stage 2's orders are registered by a dated amendment, merged before the first of
them runs ([ruling 45](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-45/README.md)). Its choices are the
coordinator's, each open to the owner's strike
([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)).

Stage 2 runs on the store that is initialized again once 0.2.0 merges. The `conduit-core` Fleet's
documents for it are [its manifest](stage-2/fleet/conduit.yaml),
[its templates](stage-2/fleet/templates.yaml) and [its instances](stage-2/fleet/instances.yaml).
They are templates, not rendered copies: the harness fills the placeholders below into a copy
outside the repository and submits that copy. It writes each directory and account as a quoted
YAML string, and each digest as one quoted string that includes its `sha256:` prefix; before it
submits the copy, it checks that the copy reads back to the values it was given.

| Agent | Type | Directory | Turn deadline | Root's Conduit |
| --- | --- | --- | --- | --- |
| `worker-1` | `codex.v2` | `<slot-1>` | 14400 s | `turn-worker-1` |
| `verifier-1` | `codex.v2` | `<slot-1>` | 3600 s | `turn-verifier-1` |
| `worker-2` | `codex.v2` | `<slot-2>` | 14400 s | `turn-worker-2` |
| `verifier-2` | `codex.v2` | `<slot-2>` | 3600 s | `turn-verifier-2` |
| `reader-1` | `claude-code.v2` | `<slot-r>` | 3600 s | `turn-reader-1` |

- Every Agent has effort `high` and its account services off
  ([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md), point 1), and `reader-1` has the model
  `opus` and peer messaging off. Each names a directory, so its client's own tools are on and
  nothing confines them in v0 ([ruling 70](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-70/README.md)).
- Each keeps its client's login in a home of its own inside its state directory
  ([order 050c](../../docs/work-orders/050c-agents-v2-client-home.md)), which keeps the accounts
  apart for the clients but is no wall against an Agent that runs code
  ([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 2). Each names in `account` the
  owner's primary account of its client, and no Agent names a second account until the owner says
  so (ruling 84, point 3).
- Two pairs of a worker and a verifier, each pair in a slot clone of its own: the coordinator
  dispatches at most two Codex runs at once on the Mac, and each Agent costs the owner one
  sign-in. A third pair is four more documents and two more sign-ins.
- `reader-1` is [the Claude verifier slot](#the-claude-verifier-slot) of stage 1's registration:
  the fresh read on Claude Opus before each merge
  ([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md)). It works in a clone of its own, into
  which the harness brings each order's branch, so no Conduit and no directory of its names a
  worker's slot. The coordinator proposes to read the reach over the worker's tree that waits for
  D59 as a Conduit or a directory naming that tree, which `reader-1` does not have; order 211's
  dated amendment records it, or the owner's answer, before `reader-1`'s first turn. Its own
  tools, like every unconfined Agent's, still reach the worker's slot.
- The Pool `trees` stays empty, as in stage 1: no tree is granted through Conduit before orders 4
  and 5.

Placeholders, filled only in the rendered copy and never committed:

| Placeholder | Stands for | Class |
| --- | --- | --- |
| `<codex-version>`, `<codex-digest>` | The agent-codex archive's version and SHA-256 | other |
| `<claude-version>`, `<claude-digest>` | The same for the agent-claude-code archive | other |
| `<slot-1>`, `<slot-2>` | The absolute paths of the two slot clones | path |
| `<slot-r>` | The absolute path of the reader's clone | path |
| `<codex-account>` | The owner's primary ChatGPT account, as Codex names it | account |
| `<claude-account>` | The owner's primary Claude account, as Claude Code names it | account |

Before stage 2's first turn:

1. Order 050n has merged the Agent packages 0.2.0, and the store is initialized again with their
   archives; by the coordinator's notes (private), stage 1's store holds archives that also carry
   the version 0.2.0.
2. Before the store is initialized with these documents, and so before `reader-1` becomes a
   dogfood slot or is signed in, D58's guard is in the harness: it runs nothing an Agent could
   have written, so nothing from a slot or from the reader's clone, and reads its own helpers
   only from its own directory and the main checkout; it never runs git inside a slot or the
   reader's clone, but reads each with the harness's own git directory and the slot or clone only
   as the working tree, or by plain file reads; and each of its guards answers held, broken or
   unknown, an unknown checked again before the next turn
   ([D58](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md);
   [IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed)). A22 no longer runs
   first ([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 1).
   Until then the harness may submit these documents without `reader-1` and `turn-reader-1`,
   and stage 2 begins with its Codex slots (ruling 89, point 5).
3. The owner has signed each of the five Agents in once, on the Mac, through `conduit sign-in`
   ([order 051b-2](../../docs/work-orders/051b2-cli-sign-in.md);
   [ruling 85](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-85/README.md), point 5), one Agent at a time: Codex's
   browser login listens on fixed ports, and a sign-in that may share one with another in
   progress reports unavailable
   ([order 048a-5c](../../docs/work-orders/048a5c-build-sign-in-fixed-port.md)).
   If `reader-1` joins later, as item 2 allows, this holds for the Agents submitted at the start,
   and `reader-1` is signed in so before its own first turn.
4. Until [order 203](../../docs/work-orders/203-new-conversation.md)'s `new-conversation.v1` is
   installed with the packages 0.2.1, a fresh conversation still restarts the engine, and a
   restart ends every Agent's conversation. Until then the harness sends one turn at a time,
   whatever the number of slots. Root's turn Conduits gain that operation by a later change, made
   with 0.2.1, and the slots then run at once.

Every Agent of stage 2 runs under the owner's operating-system user too, so
[What it cannot show](#what-it-cannot-show) holds for every result of stage 2.

## Stage 2: registered before its first turn

Registered on 2026-11-19 by work order 211, before stage 2's first turn
([ruling 45](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-45/README.md)), after the section above wrote stage 2's
documents. It holds for every order of stage 2: an order that root dispatches to go through
Conduit on the store initialized again with the Agent packages 0.2.0, or 0.2.1 if it is first
initialized with them, one turn at a time either way, fixed as stage 2's at its dispatch. Stage 2
ends when the owner ends it or when a dated amendment lets turns run at once; a turn sent while
another is in flight is not stage 2's. An upgrade of the store's Agent packages, or a new
initialization, ends stage 2 unless a dated amendment merged before it says otherwise. This text
changes only by a dated amendment, merged before the first turn it changes. Its choices are the
coordinator's, each open to the owner's strike
([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)).

### Stage 2's orders

Root sends each order through the turn Conduits of
[stage 2's instances](stage-2/fleet/instances.yaml): the worker and the verifier of one pair work
in that pair's slot clone, and `reader-1` gives the order its fresh read on Claude Opus before it
merges ([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md)), in a clone of its own. Each Agent
keeps the login that the owner gave it once, through `conduit sign-in`, in a home of its own.

- If order 2 or 3 has not run when stage 2 starts, stage 2's first orders are those orders. They
  each get two verdicts: A78's, under the seven conditions of
  [their registration](#orders-2-and-3-registered-before-order-2-runs), and stage 2's, under this
  section's nine. The setup recorded below serves as theirs, and their conditions are read as
  stage 2's conditions 1 to 7 are read below.
- [Ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 3, sets the first dogfood
  milestone: the Agents of the owner's primary accounts, signed in through Conduit on the Mac, run
  Conduit's work orders one turn at a time. The coordinator reads it as the owner's word for an
  order after order 3 to go through Conduit before
  [order 203](../../docs/work-orders/203-new-conversation.md), one turn at a time. Such an order
  is an order of stage 2, judged here, and does not count toward A78's five, whose orders 4 and 5
  a coordinator Agent sends.
- No order that changes this experiment's directory, or the harness, is sent through Conduit in
  stage 2. An order's text and its report schema are on `main` before its first turn.
- Each order of stage 2 keeps the directory, the redactions and the result that orders 2 and 3
  keep ([What counts](#what-counts-and-what-each-outcome-changes)). The account values that its
  sign-ins and account checks return are private values: the records export carries them, and
  they are replaced in the export by the placeholders of the section above before it is copied.
  Its result opens with "Not binding.", then the first paragraph of What it cannot show and the
  first paragraph of [What stage 2 cannot show](#what-stage-2-cannot-show), then its verdict.

### `reader-1`, D58, D59 and A77

- `reader-1` is signed in and sent turns only once D58's guard is in the harness, as the setup
  below records it ([ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 5), and after the
  Claude scratch-home sign-in of
  [027's amendment](../027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs)
  has passed; if that sign-in failed or ended a fault, only once the owner, told of it, has
  answered. Until then the four Codex Agents carry stage 2, and each order's fresh read runs
  outside Conduit as the method runs it. Stage 2's store may be initialized before then, as
  item 2 of the section above's list allows: it is then rendered without `reader-1` and
  `turn-reader-1`, and both are added from stage 2's documents once the guard holds. Item 3 of
  that list then holds for the four Codex Agents before stage 2's first turn, and for `reader-1`
  before its own first turn.
- `reader-1` is [the Claude verifier slot](#the-claude-verifier-slot). The coordinator reads its
  "reach over the worker's tree", which waits for the owner's answer on D59, as a Conduit or a
  directory of `reader-1` that names a worker's slot. `reader-1` has neither: the harness brings
  each order's branch into its own clone. Its own tools, like every unconfined Agent's, still
  reach the worker's slot. If the owner strikes this reading, `reader-1` is sent no turn until
  the owner has answered D59.
- A turn of `reader-1` is the fresh Opus read before the merge, as
  [the method](../../docs/implementation/METHOD_AS_RUN.md#6-the-opus-reader-and-fixes-by-hand)
  gives it, sent through Conduit. It is never A77's paired read: a paired reader is given only
  what the Codex verifier is given and runs no command that writes, and `reader-1` runs the
  order's checks, so the Opus reader of A77's paired read is started outside Conduit, as A77
  registers it. A77 pairs no order until order 050n merges (its amendment of 2026-11-19), and
  stage 2 starts after it: an order of stage 2 may then be paired, and its paired Opus read is
  still started outside Conduit. How an order's A77 record gives a turn of `reader-1`, and which
  verifier an order on the second pair has there, are A77's to say, by a dated amendment of its
  own section.

### Stage 2's claim

For each of its orders, stage 2 claims what [the claim](#claim) of orders 2 and 3 claims, on the
store initialized again with the Agent packages 0.2.0, or 0.2.1 if it is first initialized with
them, one turn at a time either way: the four Codex Agents and, once it joins, `reader-1`, each
signed in once by the owner and keeping its client's login in a home of its own. It adds two
claims:

1. the owner's one sign-in of each Agent carries the stage: no turn waits on a sign-in asked
   again;
2. the fresh read on Claude Opus before each merge can be a turn of `reader-1` through Conduit:
   fresh, valid against its schema, and leaving its clone unchanged. Only an order whose fresh
   read was a turn of `reader-1` shows it.

It claims nothing that orders 2 and 3 do not: nothing about access (D57), Agents kept apart (D60)
or speed (D54). It claims nothing about turns at once, which stage 2 does not send, and nothing
about how well `reader-1` reads beside the reader it replaces, since no read of `reader-1` is
paired with another reader's.

### Stage 2's setup, recorded before its first turn

The coordinator records each item below here by a dated amendment merged before stage 2's first
turn. Stage 2's first result repeats each item, with the command that shows it where one does.
If `reader-1` joins later, the items that concern it are recorded by a dated amendment merged
before its first turn, and the first result after it joins repeats them.

- the commit of `main` the dogfood engine runs, and that it holds the merges of orders 050n and
  051b-2, each shown by `git merge-base --is-ancestor`. Run the same check for `b1f6f8ca` and
  `60126251`, as in [orders 2 and 3's setup](#setup-recorded-before-order-2s-first-turn). Record
  whether the commit holds the order that removes the `reconcile.v1` caveat (ruling 78, point 6),
  or that the harness calls no `reconcile.v1`, and the call timeout the engine is served with.
- each Agent archive's version and digest, and the commit it was built from.
- stage 2's documents as rendered, giving each line that differs from the files in
  `stage-2/fleet/` beyond filling a placeholder, 0 if none, and the Agents' deadlines. If the
  store was first rendered without `reader-1`, record when it and `turn-reader-1` were added.
- that the store was initialized again with no Agent of stage 1, and when.
- the commit or digest of the harness with D58's guard, and when the guard held, before
  `reader-1` was added and signed in. The harness reads each order's text, report schema and
  refs from a git directory of its own, never through a slot's or the reader's clone's. It
  runs each clone's guards with that git directory and the clone only as the working tree,
  runs nothing from a clone, and runs a guard that answered unknown again before the next
  turn.
- the outcome of 027's scratch-home Claude sign-in, by a link to its result, or the owner's
  answer to its failure, before `reader-1` is signed in.
- each sign-in by the owner, one at a time. Record the kind of login `sign-in-login.v1` names,
  without a URL, code or credential, the seconds from `sign-in.v1` dispatch to its end in the
  records, and whether the account check answered the account the Agent names, without the
  account's value. Record that the login files in the clients' default homes kept their size,
  time and inode, checking only those. Record each client's everyday status before and after,
  through a filter that prints only whether a login is held and whether the expected account
  is named. Leave Keychain items unopened.
- the version each Agent runs, from `codex --version` or `claude --version` on the binary the
  Agent resolves, with the resolved path recorded as a placeholder.
- the commit or digest of the harness's scripts as run, and the lines its log and guard files
  write for each turn, as orders 2 and 3's setup lists them and for each measure below. Give
  the digests of the brief and of the report schema that `reader-1` is given. A measure that
  no line gives is not shown.
- three restarts of the engine on stage 2's store after the sign-ins, with no turn sent, and
  their median. Time each as set-up seconds are. After each restart, every account check
  answers the account its Agent names and no sign-in action is pending. If any of the three
  exceeds condition 2's bound, amend it before stage 2's first turn by the rule in orders 2
  and 3's setup. If `reader-1` joins later, make one more such restart after its sign-in and
  amend the bound by the same rule if exceeded, in a dated amendment merged before its first
  turn.

### Stage 2's measures

Every order of stage 2 takes each per-turn and per-order measure as
[orders 2 and 3's Measures](#measures) takes it, with each number beside the command that gives
it. Stage 2 adds the measures below. Set-up seconds, there and in the setup's restarts, run
until each of root's turn tools on the store answers, as condition 2 is read below.

Per turn:

- the pair's slot clone or the reader's clone in which the turn ran.
- the seconds it waited, once ready, for the turn in flight to end.
- the seconds the harness spent readying the clone before the turn, for its lock, fetch,
  branch and clean, counted apart from the restart's seconds.
- for a turn of `reader-1`, its verdict, its findings by level and its clone guard's answer.

Per order:

- the pair the order ran on.
- each read run outside Conduit in place of a turn of `reader-1`, and why.
- each sign-in action raised, by Agent, with the part that raised it (sign-in or setup), the
  restart it followed and the check's answer (another account, no account, failed or unknown).
- each setup's account check, by Agent, with the answer the records give (the named account,
  another, none, failed or unknown), or not shown.
- the sign-in actions pending after each restart and before any turn, by Agent, as the
  harness's check gives them.
- each client's version at the order's first turn, read as in the setup, beside its version
  from the setup.
- each stop and pause, with its rule, its evidence and when it was lifted.

Per stage, in each result, over stage 2's orders so far:

- the restarts and their seconds, the waits for a turn in flight and the turns sent, showing
  what a new conversation without a restart would remove, with no pass condition on this
  measure.
- the seconds of each turn of `reader-1` and its tokens where the client reports them, with no
  comparison.

### Stage 2's pass conditions

An order of stage 2 passes when all nine hold, and a condition not shown fails it, as for orders
2 and 3. Conditions 1 to 7 are [those of orders 2 and 3](#pass-conditions), read for stage 2 so:
a turn is a turn on any Agent of stage 2; in condition 1, the plain turn beside a turn of
`reader-1` is a plain Claude Code run on the same machine and account; in condition 2, set-up
seconds run until each of root's turn tools on the store answers; in condition 3, a report of
`reader-1` is valid against the reader's schema that the setup records; and in condition 4, the
guards include D58's guards on the harness's own git directory and files, and the guard of the
reader's clone, which holds when the turn left its HEAD and its branch unmoved, the harness's
`git status` of the clone is empty and no ignored path is new since the turn's start, except
these: a rewritten Cargo.lock, restored and noted; a new `target/` or a package's
`packages/NAME/dist/`, noted; and a new `__pycache__/` that holds only `.pyc` files, which the
harness removes after the turn, before the guard reads, and notes. Any other new ignored path
breaks the guard.

8. **No sign-in is asked again of an Agent the owner signed in.** No sign-in action is raised, for
   an Agent of stage 2 that the owner has signed in, from the order's first restart to its last
   end, the window counting from that Agent's last sign-in by the owner, unless the result shows,
   by a log line it cites, that the provider ended that login itself. An action raised by a
   setup's account check that failed or ended unknown counts, and the result names it apart.
9. **Every read is fresh.** Each turn of `reader-1` and each verifier's turn, but a repair turn,
   is the first of a new conversation of its Agent: it is preceded on that Agent by a restart of
   the engine or, where installed, by a `new-conversation.v1` that succeeded, as the harness's log
   shows it. A read that is not fresh is not ruling 79's read, and nothing merges on it.

Stage 2's claims hold when each of its first five orders passes. An order that fell back to the
plain runner before its first turn stays among the five, its turns counted as lost under
condition 1, or as not shown, with the reason. Each order has its result. Once
the fifth has its result, or stage 2 ends with fewer, a result for the stage gives the verdict,
and with fewer than five orders it says that the claims are not shown. With no turn of
`reader-1` among the five, it says that claim 2 is not shown.

### Stage 2's stop rules

A stop pauses stage 2: the harness sends no further turn through Conduit, the coordinator tells
the owner that day with the rule and its evidence, and the orders in flight go on on the plain
runner, where each lost turn runs again (D54). A stop does not take A78 off the core chain; that
stays the owner's (D54). The coordinator lifts a stop once its cause is a row of the first build's
open log and the fix, or why none is needed, is in the order's result, except where a rule leaves
the lift to the owner.

1. **A guard answers broken.** That includes D58's guards on the harness's own git directory and
   files, the guard of the reader's clone, and a write that the sweep sees by an Agent outside its
   own slot or clone and its own state directory. The lift is the owner's when the write reached
   another Agent's state directory, a login, the main checkout or a remote.
2. **An Agent acts under an account it does not name.** A setup's account check answers another
   account, as the records give it. The lift is the owner's.
3. **A sign-in is asked again.** When an Agent the owner signed in raises a sign-in action, that
   Agent is sent no turn until the owner signs it in again through `conduit sign-in`, and the
   other Agents go on. A second time on any Agent stops stage 2. Only the owner signs an Agent in.
4. **Two turns in a row are lost** to a fault of the engine, an Agent package or the harness, as
   condition 1 reads a lost turn, or a turn is recorded as not having occurred (condition 5).
5. **The engine does not come back.** A restart on the store does not bring back root's turn
   tools within twice condition 2's bound, or the store does not start.

An allowance's end is a pause, not a stop. When the ChatGPT Codex allowance runs out, the Codex
Agents' turns wait for it; when the Claude allowance runs low, A77's paired reads stop first and
dispatch next, and nothing merges unread
([IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed)). No API key is used, and
no Agent names one. A read of `reader-1` that is lost, or still invalid after its repair turn, is
run outside Conduit as the method runs the fresh read, so that nothing merges unread.

### What stage 2 cannot show

Each Agent keeps its client's login in a home of its own, which keeps the accounts apart for the
clients but is no wall against an Agent that runs code
([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 2): under one user, any Agent of
stage 2, `reader-1` among them, can read or use another's login, and each one's own tools reach
every slot. Each can also read what the owner's environment and shell profile hold, an API-key
variable among them, a reach the owner accepts for the dogfood
([ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 4), and stage 2 shows nothing about
whether an Agent used it. A sign-in that carried the stage shows that it did so on this store,
for this stage's length, not across a package's upgrade or the install of 0.2.1 (A78's P4, not
registered). An account check that answered the named account shows what the client said, not
which account the provider charged. One turn at a time shows nothing about turns at once: two
pairs show five Agents kept in one store and two slot clones used in turn, not two orders run
together. And no read of `reader-1` is paired with another reader's, so stage 2 shows nothing
about how well it reads.

### What stage 2 leaves unregistered

- Turns at once. Once order 203's `new-conversation.v1` is installed with the packages 0.2.1,
  root's turn Conduits gain it by a change of stage 2's documents, and a dated amendment, merged
  before the first turn sent while another is in flight, registers what that adds: the restarts
  and waits it removes, both pairs at once, and whether a store of 0.2.0 takes the Agents of
  0.2.1 without new sign-ins (A78's P4).
- A third pair, a second account, which comes after the S1 rerun's pin and only on the owner's
  word ([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 3; ruling 89, point 7), the
  Linux node as a node of the Fleet, and a tree granted through the Pool (orders 4 and 5, D59):
  each by its own registration or amendment, before the first turn it changes.

## Amendment of 2026-11-20: stage 2 directly through Conduit

Amended on 2026-11-20 (PST), after order 215's turns, which ran as a dogfood probe outside stage
2's registration (pull request 443). *Coordinator's* marks a line open to the owner's strike
(ruling 92, point 6).

**The owner's decision**, verbatim as the coordinator's notes (private) hold it. At about 01:00,
asked what should happen with the dogfood, the owner chose "One turn now as a stopgap", whose
option read: "Run order 215's first turn on the tested shell harness, with the record clearly
marking the harness as temporary. Codex builds its Rust replacement in parallel, and the dogfood
moves onto it as each part lands." At about 01:15, for the stopgap turn, the owner answered "Run
it as approved". At about 03:15 the owner chose "Directly through Conduit (Recommended)", whose
option read: "Every dogfood order goes by `conduit call` of root's turn tools, with Conduit's
records as evidence and each manual step a logged gap. The shell harness is retired. 211's
registration gets a dated amendment saying so."

*Coordinator's*: for each later turn of stage 2, this amendment replaces the harness and the
method order 211 registered [above](#stage-2-registered-before-its-first-turn); the rest stands.
Where stage 2 names the harness, read Conduit's records; a measure no record gives is not shown.

**The setup as it was done**, on 2026-11-19 and 2026-11-20 (PST), as the store's setup and
harness logs and the coordinator's notes (private) hold it, the paths as `<codex-path>` and
`<claude-path>`:

- The engine at `b897d316` (order 050n, pull request 441); `git merge-base --is-ancestor`
  exits 0 there for `83b74bf4` (051b-2), `b1f6f8ca` and `60126251`. Built at it: agent-codex
  0.2.0, `sha256:be01c2efe3d836fc4177fe5337ff489ac6f3bb2b4b89172e68629f9b79a6e887`, and
  agent-claude-code 0.2.0,
  `sha256:fea05dc9e880c06c608b84ef33a791b45f2f156d4d1436a6e5e818a213bb6bab`; Codex 0.167.1 at
  `<codex-path>`, Claude Code 2.1.318 at `<claude-path>`. The store's init: exit 0, one restart
  of 14 s, a sign-in action at root for each of the five Agents.
- The harness at commit `3efeab26`. Each of the seven starts of serve in the harness's serve log
  had a call timeout of 15000 s.
- The owner's five sign-ins, just after midnight on 2026-11-20, in one sitting, by
  `conduit sign-in` and a browser approval: `worker-1`, `verifier-1`, `worker-2`, `verifier-2`,
  then `reader-1`, after 027's Claude sign-in
  [passed](../027-a5-sign-in/results-2026-11-19-1-step-1.md#the-claude-sign-ins-result). Each
  succeeded; the unanswered actions counted 4, 3, 2, 1, then none; `conduit pending` listed none.
  The everyday Codex and Claude Code logins were unchanged by their statuses, and the Codex
  default home's login file had its stat equal before and after.
- The restarts after the sign-ins: one of 19 s, then three with no turn sent, 19, 23 and 24 s
  (median 23 s, under condition 2's 240 s), each with every setup passing and no action raised.
- Not recorded: the init's time and that it held no Agent of stage 1; the rendered lines that
  differ; each sign-in's login kind, seconds and account check; the Claude default home's login
  file; each Agent's resolved version; `reconcile.v1`'s caveat.

**The method as run for order 215** on 2026-11-20, the first order built through Conduit by its
own Agents (pull request 443). The coordinator started serve on the store by hand with a long
call timeout (7200 s). `worker-1` and `verifier-1` share one directory, `slot-1`, which the
coordinator brought to `main` before the first turn. Root called, by `conduit call`,
`turn-worker-1.turn.v1` (07:28 to 07:47 PST), `turn-verifier-1.turn.v1` (07:48 to 07:58, two
musts) and `turn-worker-1.turn.v1` again in the same conversation (07:59 to 08:01, both fixed).
All three succeeded. The coordinator copied the results back into a trusted worktree, and the
gate and the fresh read ran there.

*Coordinator's*: turns sent before this amendment merges, order 216's included, are outside stage
2's registration, as order 215's were.

**What this method does not have**:

- A guard on what an unconfined Agent touches: the harness's guards are retired with it, and
  Conduit has none. `verifier-1` has the worker's reach over `slot-1`. *Coordinator's*: so pass
  condition 4 is not shown, and no order of stage 2 can pass as registered until a guard exists
  in Conduit itself or the owner decides otherwise; stop rule 1 has no guard to answer it. Pass
  condition 9 is read from Conduit's records of the restart or `new-conversation.v1` before the
  turn, and is not shown where they do not show it.
- `reader-1`'s turns. [Ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 5, tied the
  Claude reader slot to D58's guard in the harness. *Coordinator's*: `reader-1`, though signed
  in, takes no turn until a guard exists in Conduit itself or the owner decides otherwise; each
  fresh read runs outside Conduit, and stage 2's claim 2 is not shown.
- The steps done by hand, each a gap in the coordinator's notes (private), to be ordered as
  engine and package work ([ruling 92](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-92/README.md), points 2, 9).

## Amendment of 2026-11-21: links after work order 235

The coordinator, outside every turn, pointed this directory's links into `docs/proposals/` at
`077fa47e`, and the open log's at its new place, since a stage-2 order may not change this
directory. No other text changed.

## Stage 3: registered before its first turn

Registered on 2026-11-22 by work order 277, before stage 3's first turn
([ruling 45](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-45/README.md)). It holds for every order that
root sends through stage 3's Fleet, its turns running at once, across pairs and accounts. An
order is stage 3's when root dispatches its first turn after step 9's restart of
[the setup](#before-the-sitting-and-at-it) to an Agent of stage 3's documents, which the records
name by its ID, since four of their names were stage 2's Agents' too. An order whose first turn
was dispatched before that restart is not stage 3's, and conditions 8 and 11 and stop rule 3
count nothing from before it. It changes only by a dated amendment, merged before the first turn
it changes. It applies the owner's decisions
[DEC-042](../../docs/decisions/log.md#dec-042-2026-11-21-model-logins-are-pool-owned-resources),
[DEC-043](../../docs/decisions/log.md#dec-043-2026-11-21-the-sign-in-freeze-ends-in-sequence),
[DEC-044](../../docs/decisions/log.md#dec-044-2026-11-21-records-fail-closed-and-research-before-deciding),
[DEC-045](../../docs/decisions/log.md#dec-045-2026-11-21-the-shared-agent-contracts-the-schedule-and-stage-2s-record),
[DEC-046](../../docs/decisions/log.md#dec-046-2026-11-21-the-fleet-grows-on-every-account-by-measurement),
[DEC-047](../../docs/decisions/log.md#dec-047-2026-11-21-usage-in-records-and-harnesses-in-role-templates),
[DEC-050](../../docs/decisions/log.md#dec-050-2026-11-22-both-reads-on-every-order),
[DEC-051](../../docs/decisions/log.md#dec-051-2026-11-22-the-linux-programs-built-in-docker),
[DEC-052](../../docs/decisions/log.md#dec-052-2026-11-22-a-release-reached-through-a-project-contract-must-be-installed),
[DEC-053](../../docs/decisions/log.md#dec-053-2026-11-22-an-opus-drafter-on-the-second-claude-account),
[DEC-054](../../docs/decisions/log.md#dec-054-2026-11-22-pair-4-by-measurement) and
[DEC-055](../../docs/decisions/log.md#dec-055-2026-11-22-stage-3-sets-condition-4-aside). Its
choices are the coordinator's, each open to the owner's strike
([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)).

The method of
[the amendment of 2026-11-20](#amendment-of-2026-11-20-stage-2-directly-through-conduit) holds:
root's tools are called by `conduit call`, Conduit's records are the evidence, and each manual
step is a logged gap. Its reading that `reader-1` waits for a guard does not: the owner struck it
([stage 2's results](results-2026-11-21-stage-2.md#owner-sittings)). No order that changes this
experiment's directory goes through Conduit.

### Stage 3's documents

The `conduit-core` Fleet's documents for stage 3 are [its manifest](stage-3/fleet/conduit.yaml),
[its templates](stage-3/fleet/templates.yaml) and [its instances](stage-3/fleet/instances.yaml).
They are templates, not rendered copies. The coordinator renders them by hand into a copy outside
the repository (a gap), reads the copy back against the values it was given and submits it. No
account value is printed in doing so.

| Agent | Type | Account | Directory | Deadline | Root's Conduit |
| --- | --- | --- | --- | --- | --- |
| `worker-1` | `codex.v4` | `primary-chatgpt` | `<slot-1>` | 14400 s | `root-worker-1` |
| `verifier-1` | `codex.v4` | `primary-chatgpt` | `<slot-1>` | 3600 s | `root-verifier-1` |
| `worker-2` | `codex.v4` | `primary-chatgpt` | `<slot-2>` | 14400 s | `root-worker-2` |
| `verifier-2` | `codex.v4` | `primary-chatgpt` | `<slot-2>` | 3600 s | `root-verifier-2` |
| `worker-3` | `codex.v4` | `second-chatgpt` | `<slot-3>` | 14400 s | `root-worker-3` |
| `verifier-3` | `codex.v4` | `second-chatgpt` | `<slot-3>` | 3600 s | `root-verifier-3` |
| `worker-4` | `codex.v4` | `second-chatgpt` | `<slot-4>` | 14400 s | `root-worker-4` |
| `verifier-4` | `codex.v4` | `second-chatgpt` | `<slot-4>` | 3600 s | `root-verifier-4` |
| `reader-a` | `claude-code.v4` | `second-claude` | `<slot-r>` | 3600 s | `root-reader-a` |
| `drafter` | `claude-code.v4` | `second-claude` | `<slot-d>` | 3600 s | `root-drafter` |

- Each Agent is made from its role's template, which fixes its text, its report schema, its model
  and its effort (DEC-047): `worker-role`, `verifier-role`, `reader-role` or `drafter-role`.
  Every Agent has effort `high` and its account services off; the two Claude Agents have the
  model `opus` and peer messaging off.
- Each Agent's `login` names its account's Resource, which the Pool `logins` owns:
  `primary-chatgpt` and `second-chatgpt` are `codex-login.v1` Resources, `second-claude` an
  `account-home.v1` Resource. Each account's one login serves its Agents and is never copied:
  each Codex Agent's home links to its Resource's login, and each Claude Agent's client runs in
  its Resource's account home.
  Root reaches the Pool through `root-logins`, and each Agent through `root-NAME`, from its
  package's `conversation` template, which grants `turn.v1` and `new-conversation.v1`.
- A pair shares a working copy of its own; `reader-a` and `drafter` have one each. `reader-a`'s,
  `<slot-r>`, is the clone that stage 2's `reader-1` reads in, and the two never read at once
  before step 10 retires `reader-1`.
- No account home is made for `primary-claude`: that account runs the coordinator alone
  (DEC-046, as DEC-050, DEC-053 and DEC-054 place the reader and the drafter).
- Stage 2's Fleet `conduit-core` owns the Agents, and stage 3's documents declare no Fleet.
  Stage 2's Pool `trees`, its Conduit `root-trees` and its templates stay on the store, unused.
  A new store takes that Fleet first.

Placeholders, filled only in the rendered copy and never committed:

| Placeholder | Stands for | Class |
| --- | --- | --- |
| `<codex-version>` | The agent-codex archive's version, 0.2.4 | other |
| `<codex-digest>` | That archive's SHA-256 | other |
| `<claude-version>`, `<claude-digest>` | The same for the agent-claude-code archive | other |
| `<primary-chatgpt-account>` | The primary ChatGPT account, as Codex names it | account |
| `<second-chatgpt-account>` | The second ChatGPT account, as Codex names it | account |
| `<second-claude-account>` | The second Claude account, as Claude Code names it | account |
| `<slot-1>` to `<slot-4>` | The absolute paths of the four pairs' working copies | path |
| `<slot-r>`, `<slot-d>` | The absolute paths of the reader's and the drafter's copies | path |
| `<codex-model>` | The Codex Agents' model, as `codex` names it; never empty | other |
| `<report-schema>` | The JSON text of [the orders' shared schema](../../docs/work-orders/004-i2-01-vectors-and-first-start.schema.json) at the engine's commit | other |
| `<verify-schema>` | The JSON text of [`verify.schema.json`](../../docs/work-orders/verify.schema.json) at the engine's commit | other |

Each directory, account and schema is written as one quoted YAML string, and the setup's
amendment gives each schema's SHA-256. `<codex-model>` is never empty, since the Codex package
omits `usage` for an empty model (condition 10).

### Before the sitting, and at it

The coordinator records steps 1 to 9 below, with their evidence, by a dated amendment merged
before stage 3's first turn, and step 10, which comes after that turn, by a later dated amendment
that names the read that freed `reader-1`. The documents are submitted in parts, in this order.

Before the sitting, while stage 2's pairs work on, the coordinator:

1. builds the dogfood engine from `main` holding orders 258, 260, 262 and 269, each shown by
   `git merge-base --is-ancestor`, and serves it with a call timeout above 14400 s, the workers'
   deadline;
2. installs the 0.2.4 archives into stage 2's store, recording each one's version, digest and
   the commit it was built from;
3. submits the documents that name no account and no Agent (the manifest, the templates, the
   Pool `logins` and `root-logins`) and `primary-chatgpt`'s Resource, and sets that Resource up;
4. has the owner type each second account, shown on the owner's screen and confirmed before it
   is kept.

At the owner's signal, the coordinator:

5. retires `worker-1`, `verifier-1`, `worker-2` and `verifier-2` one at a time, each one's retire
   hook logging out only its own login. After each, the stat of the everyday Codex login file,
   which is never opened, is unchanged; otherwise the coordinator stops and tells the owner. The
   everyday Claude login and a revocation by the vendor are not shown;
6. lifts their tombstones, and those of root's `turn-worker-1`, `turn-verifier-1`,
   `turn-worker-2` and `turn-verifier-2`, since a Conduit's tombstone also matches by consumer,
   target and capability (LIF-041);
7. has `primary-chatgpt` signed in; then, for each second account in turn, tells the owner before
   its first use, submits its Resource and sets it up, and has it signed in. Each sign-in is the
   owner's, one at a time, since Codex's browser login listens on fixed ports, and goes by
   `sign-in.v1` through the Pool;
8. submits and sets up the signed-in accounts' Agents and root's Conduits to them, with no
   sign-in;
9. restarts the engine, which shows every setup passing and no sign-in action pending;
10. retires `reader-1` last, once `reader-a`'s first read of an order of stage 3 has succeeded
    and is fresh, as condition 9 asks.

### Stage 3's claim

For each of its orders, stage 3 claims what [the claim](#claim) of orders 2 and 3 claims, with
turns at once across pairs and accounts on one store. It adds what conditions 8 to 12 show:

1. a login's one sign-in carries its Agents in parallel, and a lapse makes that login take one
   model call at a time until root answers its notice (PND-005; condition 8);
2. each turn's usage is in its record (IFC-051; condition 10);
3. a usage limit pauses only its account (condition 11);
4. both reads of an order run blind and at once (DEC-050; conditions 9 and 12).

It claims nothing about access (D57), Agents kept apart (D60), speed (D54) or the drafts.

### Stage 3's measures

Each number in a result has beside it the command that gives it. Results name accounts by label,
read the records by named fields, and are per stage, as
[stage 2's](results-2026-11-21-stage-2.md) are, until the owner answers their
[item 3](results-2026-11-21-stage-2.md#for-the-owner).

- Per turn, from the records: the Agent, its account and its pair; its span; its outcome; its
  `usage`; and the turns in flight at its dispatch, in all and on its login. Beyond the amendment
  of 2026-11-20's rule of records alone, since these Types keep no inputs or outputs in their
  records, from what the coordinator kept of each call, its message, its arguments and its
  printed result, matched by invocation ID as
  [stage 2's results](results-2026-11-21-stage-2.md#the-evidence-and-its-limit) match them: the
  failure class, `failedTurn.limit`, and why the turn has no `usage`; its report, and whether the
  report is valid against its schema (condition 3); and for a read, the checkpoint's commit that
  its message names and whether the message holds the other read's report (condition 12).
- Per order: both reads' spans, their overlap and their waits; the fix turns, one covering both
  reads (DEC-050); the tokens by role.
- Per account: each sign-in action and notice, with its call and the time to root's answer; each
  usage-limit failure, with its `resetsAt` and the turns it held.
- Per stage: the most turns at once, in all, per account and per login; the tokens per account
  per day.
- With no pass condition, each of the drafter's first five drafts: its order, its span, its
  tokens, and what the coordinator kept, changed or rewrote (DEC-053).

### Stage 3's pass conditions

An order of stage 3 passes when all its conditions hold, and a condition not shown fails it.
Conditions 1 to 7 are [stage 2's](#stage-2s-pass-conditions), as stage 2 reads them. Condition 4
is set aside: it is recorded as not shown and judges no order, since the Agents run unconfined and
no guard runs (DEC-055). A turn of `drafter` belongs to no order's verdict: no condition judges
it, and it is measured as any turn is and as DEC-053 asks.

8. **A login is signed in once.** From the order's first turn's dispatch to its last turn's end,
   no sign-in action is raised on a login Resource but by a lapse report, and no two model calls
   run at once on a login from its lapse report until root answers the notice (PND-005). An
   action raised by a setup's account check that failed or ended unknown counts, and the result
   names it apart.
9. **Every read is fresh.** Each read but a repair is the first turn on its Agent since a
   successful `new-conversation.v1` or a restart.
10. **Every turn's usage is kept.** Each succeeded turn has `usage` with its model and client
    version, or lacks it for a reason its package's README names, which the result names; no
    turn has `usageDropped`.
11. **A limit pauses its account.** No turn is dispatched on an account between its usage-limit
    failure and that failure's `resetsAt` or, if that is null, the coordinator's recorded read
    that the account reopened; and every turn runs on its Agent's account as the table gives it.
12. **Both reads are blind and at once.** An order's two reads read one checkpoint, whose commit
    both messages name, and overlap, and neither one's message holds the other's report.

The claims hold when stage 3's first five orders pass and, over them, turns ran at once on both
ChatGPT accounts, and two pairs at once on one login.

### Pair 4, by measurement

The owner's DEC-054 holds pair 4 back, since the Mac met timeouts of the
[S2](../../ROADMAP.md#scenarios) tests and a full disk at two pairs and the gate on 2026-11-22.
`worker-4` and `verifier-4` are set up, but sent no turn until a dated amendment shows, over five
consecutive orders on pairs 1 to 3, no `usage-limit` failure on `second-chatgpt`, no turn lost to
the machine (a timeout, a full disk, a killed process), and each gate run PASS or failing only on
the order's change. For the same reason one reader, `reader-a`, reads for every pair, and a second
reader comes by measurement, as pair 4 does (DEC-054). More Agents take new documents and an
amendment.

### Stage 3's stop rules

A stop pauses stage 3 as [stage 2's stop rules](#stage-2s-stop-rules) pause stage 2. Stage 2's
rules 1, 4 and 5 hold, and its rule 2 holds for a Resource's account check. Rule 1 has no guard
to answer it while none runs (DEC-055). Rule 3 becomes:

3. **A sign-in is asked again, or work leaves its account.** From step 9's restart, a sign-in
   action that is no lapse report, one raised by a setup's account check that failed or ended
   unknown among them, or a second lapse on one Resource in one calendar day, PST, stops that
   account until the owner signs it in again. A turn dispatched on a paused or stopped account,
   or work moved between accounts, stops stage 3, and the lift is the owner's.

A usage limit pauses its account: a read it holds waits, and nothing merges unread. No API key is
used, and no Agent names one.

### What ends stage 3

Stage 3 ends when the owner ends it, at an upgrade of the store's Agent packages, at a new
initialization or at a change of the account map, unless a dated amendment merged before says
otherwise. Pair 4's start does not end it. For stage 3, an upgrade is a change of the version or
digest that the manifest pins for either Agent package, or an instance of stage 3's documents
moving to another release; an archive installed with no instance moved is none. Step 2's install
comes before stage 3's first turn and ends nothing of it. What such an install means for stage 2
is open to the owner in [stage 2's results](results-2026-11-21-stage-2.md#for-the-owner), items 1
and 4, and this registration does not answer it.

### What stage 3 cannot show

[What it cannot show](#what-it-cannot-show) holds for every result of stage 3. Any Agent's tools
reach every working copy and every login, and can log a shared login out for all its Agents
([ruling 70](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-70/README.md)). The tokens are each client's report, not
what the account is charged or what is left of its week, and they leave out the coordinator and
the [S3, S7 and S10](../../ROADMAP.md#scenarios) tests. A refresh failure that fails no turn is not seen. Blind is only what
each message carries. One Mac shows nothing about another machine.

## Amendment of 2026-11-22: stage 3 under DEC-068 to DEC-071

Amended on 2026-11-22 (PST), before stage 3's first turn, by the owner's answers of that day:
[DEC-068](../../docs/decisions/log.md#dec-068-2026-11-22-the-second-account-sitting-runs-by-hand-tonight),
[DEC-069](../../docs/decisions/log.md#dec-069-2026-11-22-pairs-1-and-2-and-one-reader-take-turns-first),
[DEC-070](../../docs/decisions/log.md#dec-070-2026-11-22-stage-3-is-judged-on-access) and
[DEC-071](../../docs/decisions/log.md#dec-071-2026-11-22-the-claude-reader-reads-text-facing-orders),
with
[DEC-072](../../docs/decisions/log.md#dec-072-2026-11-22-the-s1-chain-merges-next) and
[DEC-073](../../docs/decisions/log.md#dec-073-2026-11-22-v0s-seven-items-full-v0-by-the-end-of-the-year)
where they bear on what stage 3 measures. The rest of stage 3's registration stands, but two
passages are out of date: the owner has answered items 1 and 4 of
[stage 2's results](results-2026-11-21-stage-2.md#for-the-owner) by DEC-056 and DEC-059, and
item 3 by DEC-058, so stage 3's results commit their scrubbed records export, split by order.
*Coordinator's* marks a line open to the owner's strike (ruling 92, point 6).

**The sitting, by hand** (DEC-068). Step 4, and step 7 for the second accounts, go so:

- The coordinator writes the login Resources of `second-chatgpt` and `second-claude` into copies
  outside the repository, every line but each one's `account:` line, which the owner types in
  their own editor. *Coordinator's*: the coordinator writes that line with its placeholder, and
  the owner types the account over it.
- The coordinator never opens a copy after. It checks each copy's mode and that no placeholder is
  left; step 7 then submits the copy. Unlike the other documents, these two copies are not read
  back against their values. *Coordinator's*: the mode must be 600, and counts that print no
  line show one `account:` line and no placeholder. If any check fails, the coordinator submits
  nothing and tells the owner.
- Before the sign-ins, the owner gives each account's plan and weekly reset day, by label.
  *Coordinator's*: the coordinator keeps them in its notes (private). The per-account measures
  add, for each account that runs Agents, its tokens per week counted from its reset day, and a
  result numbers the weeks and gives none of their dates.
- Each of these steps is a gap. `value.v1` (DEC-076) replaces the copies and their checks once
  it lands, by a dated amendment.

**Who takes turns** (DEC-069). The sitting still signs in every account and sets up every Agent
(steps 7 and 8). Pairs 1 and 2 and `reader-a` take turns first. Pair 3 and `drafter` are sent no
turn until orders 280 to 282 have merged and one day's engine gates average under 20 minutes.
*Coordinator's*: that day is any whole calendar day, PST, after the last of the three merges.
Its engine gates are the Mac's `tools/gate.py` runs whose `engine test` row passed, and the day
has at least one. Each one's seconds are its record's `finished` less its `started`. A dated
amendment shows the merges and that day before pair 3's or `drafter`'s first turn.

Pair 4 stays held by [DEC-054's measure](#pair-4-by-measurement), read as no window reached: no
five consecutive orders have shown it yet. *Coordinator's*: its five orders count from pair 3's
first turn, since before it `second-chatgpt` carries no turn.

While pair 3 waits, no turn runs on `second-chatgpt`. Where stage 3's first five orders hold no
turn of pair 3, the result says so, and that the turns at once on both ChatGPT accounts, which
the claims ask, are not shown. *Coordinator's*: stage 3's claims are then not shown, and the five
do not move.

**The verdict** (DEC-070). An order of stage 3 passes when conditions 1, 3, 5, 8, 11 and 12
hold, and one of them not shown fails it. Conditions 6 and 9 are measured, and each result
reports them for each order, but they judge none. Condition 10 judges nothing until each Agent
package names every reason a turn lacks `usage`. Conditions 2 and 7 judge no order, since
DEC-070 names neither, and condition 4 stays set aside (DEC-055). *Coordinator's*: the README
that counts is the installed release's, 0.2.4. A README that names every reason comes only in a
later release, whose install ends stage 3 (below), so condition 10 judges no order of stage 3
unless a dated amendment, merged before the first turn it judges, says otherwise. Each result
reports conditions 2, 7 and 10 as it reports 6 and 9.

So, while their conditions judge nothing, three things are not shown: claim 2 of
[stage 3's claim](#stage-3s-claim), which rests on condition 10; the bounded set-up time and the
records of every turn that stage 3 takes from [the claim](#claim) of orders 2 and 3, which rest
on conditions 2 and 7; and fresh reads, which condition 9 reports. Claim 4 rests on condition 12
alone. *Coordinator's*: the access that DEC-070 judges is what its listed conditions show;
stage 3 still claims nothing about access as D57 means it.

**What ends stage 3** (DEC-070). DEC-070 asks this amendment to say which store upgrade, if any,
does not end the stage. *Coordinator's*: none. Each upgrade that
[What ends stage 3](#what-ends-stage-3) names ends the stage. So does an install of agent-codex
or agent-claude-code 0.2.5 or later into the dogfood store after stage 3's first turn, even with
no instance moved: for those releases, this replaces the registration's "an archive installed
with no instance moved is none". Step 2's install, before the first turn, ends nothing, whatever
release `main` then carries. A later dated amendment, merged before the install, may say otherwise.

**The reads** (DEC-071, amending DEC-050). The verifier of the order's pair reads every order,
blind. `reader-a` also reads, blind and at the same time, an order that changes `spec/`, a
diagnostic or refusal, agent- or owner-facing package text, or a README, and one fix turn covers
both reads. An engine-internal order merges on its verifier's clean read and the gate. Claim 4 is
then a claim about the orders with both reads; with no such order among stage 3's first five
orders, claim 4 is not shown. *Coordinator's*:

- an engine-internal order is read as any order outside the four kinds above;
- before an order's first read is dispatched, the coordinator records whether `reader-a` reads
  it and by which part of DEC-071, and the result gives it;
- for an order with one read, condition 12 asks only that the verifier's message name the
  checkpoint's commit it reads, and the per-order measures give that read's span alone;
- each verifier's turn message says whether `reader-a` reads the order, since `verifier-role`'s
  text, written under DEC-050, says that the Claude reader reads the same change at the same
  time. The documents keep that text, so that they stay as order 277 checked them, each role
  text by its hash.

**What DEC-072 and DEC-073 bear on.** DEC-072 orders the merges, the S1 chain (orders 245 to 250)
ahead of every engine order not on the sitting's path; it judges nothing and adds no measure.
DEC-073 adds machines, accounts and usage wherever measurement shows they speed the work.
*Coordinator's*: so the per-stage measures add, with no pass condition, the orders merged per
day, PST, and each day's mean seconds of its engine gates, read as above. An Agent added takes
new documents and an amendment, as [the registration](#pair-4-by-measurement) says, and so does
an Agent placed on another machine. An account added changes the account map, which ends stage 3
unless a dated amendment merged before says otherwise. Stage 3 still claims nothing about speed
(D54).

## Amendment of 2026-11-23: stage 3's setup, steps 1 to 9

Amended on 2026-11-23 (PST), before stage 3's first turn, as
[the registration](#before-the-sitting-and-at-it) asks: steps 1 to 9 with their evidence, from the
store's setup and serve logs, its records and the coordinator's notes (private). The coordinator
wrote it outside every turn. The bar on orders that change this directory covers orders sent
through the Fleet
([DEC-057](../../docs/decisions/log.md#dec-057-2026-11-22-order-235s-change-was-no-breach)), so
this breaches nothing. Paths are written `<slot-N>`, `<codex-path>` and `<claude-path>`, and no
account value appears. Times are PST. *Coordinator's* marks a line open to the owner's strike
(ruling 92, point 6).

**The setup, in the order it was done**, on 2026-11-22:

- **Before the switch.** Serve on the engine at `b897d316` was stopped, and the records were
  exported at 13:18: 2,482 lines, nothing on the error output. It was the old engine's last export.
- **Step 1.** The dogfood engine was built at `M`, `933eae2210d640d6d9b34f9b2bf282da8673a279`.
  `git merge-base --is-ancestor` exits 0 at `M` for seven commits: orders 258 (`37c10930`), 260
  (`2194bb0b`), 262 (`97a4b721`), 269 (`dc172970`), 277 (`4f14b1db`) and 279 (`faddf9c6`), and
  [the amendment of 2026-11-22](#amendment-of-2026-11-22-stage-3-under-dec-068-to-dec-071)
  (`14d89630`). Serve started on it at 13:21:30, by hand under `env -i`, with a call timeout of
  15000 s, above the workers' 14400 s deadline. It printed its ready line, raised no action, and
  opened the store made at `b897d316` with stage 2's five Agents set up. Codex 0.167.1 at
  `<codex-path>` and Claude Code 2.1.318 at `<claude-path>`, read from their package files at
  13:35, and read again at 18:19, unchanged. They read the same again on 2026-11-23 at about 08:05, before the first sign-in.
- **The render.** 277's documents were rendered at `M` by a one-time Python script (a gap,
  below). It filled every placeholder but the accounts and wrote each account line with its
  placeholder. Each folder was read back against its values, with only the account placeholders
  left. The two schemas went in as they are at `M`: `<report-schema>` with SHA-256
  `a10a24a0bce7a102d9864a708ae6938277ef0cf2a76b3cf3c32dd9bfb3a11e5d` and `<verify-schema>` with
  `2dfe62ff7517965078ca9aedd587369a0c5143f2f2f69e4c02076029477ed36b`. For `<codex-model>`: stage
  2's four Codex Agents declare the model `""`, read from one field of the store's inspection, so
  stage 2 ran on the client's own default. The render took `gpt-6.1-sol` as a candidate, first in
  the model catalog compiled into the installed Codex client. `<slot-3>`, `<slot-4>` and `<slot-d>` were cloned at `M`, each with no change.
- **Step 2.** At about 14:2x, agent-codex 0.2.5,
  `sha256:143b69af6ddd6f8ae3d458513ad994df79b285c4b6d678ffa6093f02040f0008`, and
  agent-claude-code 0.2.5,
  `sha256:fbf4ed90d15998308e889f2c7391ab4ac41bd82a92f3d699e320e0ede89973c9`, both built at `M`,
  were installed into stage 2's store, each by a dry run and then the install. Both dry runs and both installs exited 0, between about 14:20 and 14:22; the coordinator's notes keep no closer time. The gate's builds of 0.2.5 in two other directories gave the same two
  digests. Stage 2 had ended at order 239's merge
  ([DEC-059](../../docs/decisions/log.md#dec-059-2026-11-22-stage-2-ended-at-order-239s-merge)),
  and stage 3 had taken no turn, so the install came at a stage boundary
  ([DEC-080](../../docs/decisions/log.md#dec-080-2026-11-22-the-per-node-chain-is-finished-now)).
  The amendment of 2026-11-22, merged before it, says step 2's install ends nothing
  ([DEC-070](../../docs/decisions/log.md#dec-070-2026-11-22-stage-3-is-judged-on-access)).
  *Coordinator's*: the registration's 0.2.4 is read as 0.2.5, which that amendment's "whatever
  release `main` then carries" allows. Condition 10 is read by the installed release's README.
  From `4f14b1db` to `933eae22` neither package README's "Usage in each turn's result" section
  changed, so condition 10 still judges no order of stage 3.
- **Step 3, `base/`.** Submitted at about 14:22, before any account was written. The
  registration, the open of `Conduit/root-logins` and the setup of `Pool/logins` each succeeded,
  with no rule ID. Root's tools then held seven `root-logins.*` and nine `logins.*`, and no action
  was pending.
- **Step 4 asked.** At 14:33 the coordinator asked the owner in chat to
  type the account line in each login Resource's copy, `primary-chatgpt`'s included (below), and
  to give each account's plan and weekly reset day by label.
- **16:14, a restart that is not step 9.** At 15:03 the coordinator cancelled a turn of stage 2's
  `verifier-2`. Its session then refused an implicit restart, and its next turn failed at once, at
  15:58. Stage 2's Conduits from root grant only `turn.v1`, so only a restart of serve freed it.
  With no turn in flight, the coordinator exported the records (2,904 lines) and restarted serve
  at 16:14:17 by step 1's command line in a shell wrapper. Turns went on at 16:15. After it all five turn tools answered, serve's error output was empty and no action was pending. It came after step 2 and `base/`, and step 9 follows step 8, so it is not
  step 9.
- **The model, about 18:25 to 18:38.** The owner struck `gpt-6.1-sol` for `gpt-6-astra`.
  `base/` was rendered again with it into a new folder and read back against its values, and the
  `worker-role` and `verifier-role` templates were replaced from that folder, each succeeding
  with no action pending. Both read `gpt-6-astra` at 18:38. No Agent of stage 3 existed yet, so
  no instance moved. In the same answers the owner gave each account's plan by label, kept in the
  coordinator's notes (private). No reset day was given then.
- **Steps 5 and 6.** *Coordinator's*: step 5's signal was the owner's message of about 17:50,
  which gave about 19:00 for the sitting. From 18:38, with no turn in flight, `worker-1`,
  `verifier-1`, `worker-2` and `verifier-2` were retired one at a time, from 18:39 to 18:40. Each
  exited 0 with outcome `succeeded`. The stat line of the everyday Codex login file, which was
  never opened, was equal before and after each. Their four tombstones were lifted, and with them
  those of root's `turn-worker-1`, `turn-verifier-1`, `turn-worker-2` and `turn-verifier-2`. None
  of the eight was then listed. `reader-1` and `turn-reader-1` stayed, and no action was pending.
  The everyday Claude login and a revocation by the vendor are not shown. From then on no Codex
  lane ran.
- **The postponement.** At about 18:56 the owner moved the sitting to the morning of 2026-11-23.
- **Step 3's Resource, `primary-chatgpt`.** It came after steps 5 and 6, not before them as the
  registration lists it. *Coordinator's*: neither depends on the other, and the coordinator had
  extended
  [DEC-068](../../docs/decisions/log.md#dec-068-2026-11-22-the-second-account-sitting-runs-by-hand-tonight)'s
  by-hand typing, which names only the two second accounts, to `primary-chatgpt`. The owner chose
  otherwise. At about 19:59 the owner gave the account in chat, the coordinator showed it back,
  the owner confirmed it, and the coordinator wrote it into the copy, as
  [the registration](#stage-3s-documents) has it write a value it is given. The line was not read back against that value; only the checks below ran. The coordinator has seen that one account value, only in
  that chat. The copy passed the checks: mode 600, one `account:` line holding one double-quoted
  value, no placeholder or angle bracket. At 20:00 its registration and
  `setup Resource/primary-chatgpt` succeeded with no rule ID. The setup found no login and raised
  one sign-in action at root, the only one pending.
- **No sign-in that night.** At about 23:54 the owner said that no account signs in that night.
  Its action was still pending at the first sign-in the next morning.

**At the sitting, 2026-11-23:**

- **Step 4, the second accounts.** At about 08:30 the owner gave the two second accounts' lines in
  chat, as for `primary-chatgpt`: the coordinator showed both back, the owner confirmed them, and
  the coordinator wrote each into its copy. Neither was read back against the value; both copies
  passed the checks. No weekly reset day was given; each account's week is measured from its first
  use.
- **The owner's ruling before the team accounts.** The owner ruled that a usage guard is a
  feature Conduit offers, declared on the login Resource with explicit flags, and that no Agent on
  a team seat (`second-chatgpt`'s pairs 3 and 4, `second-claude`'s `reader-a` and `drafter`) takes
  a turn until it is built and merged. *Coordinator's*: this holds those Agents back further than
  [DEC-069](../../docs/decisions/log.md#dec-069-2026-11-22-pairs-1-and-2-and-one-reader-take-turns-first)
  does; `reader-1` keeps the reads on the primary Claude account meanwhile, and step 10 waits.
- **Step 7.** `primary-chatgpt` signed in at 08:11: outcome `succeeded`, the account present and
  answered signed in, no action pending after. Before `second-chatgpt`'s first use the
  coordinator gave its notice; its submission and setup succeeded at 08:31 with its one sign-in
  action pending, and it signed in at 08:47, `succeeded`, nothing pending after. Then the same for
  `second-claude`: its notice, the owner's "go", its submission and setup at 08:49, and its
  sign-in at 08:51, `succeeded`, nothing pending after; the owner mentioned no Keychain dialog.
  The everyday Codex login's stat line was equal before and after each.
- **Step 8.** At 09:12 the ten Agents and root's ten Conduits to them were submitted in one
  registration: the registration, the ten opens and the ten setups each succeeded, with no rule
  ID and no action pending, and root's tools held the twenty `root-*` tools.
- *Coordinator's*: stage 3's first turn is read as the first turn of a stage-3 order, which
  [the registration](#stage-3-registered-before-its-first-turn) places after step 9's restart.
  The smoke turns, outside every order, are not it, and no install was made after the first of
  them.
- **The smoke turns.** *Coordinator's*: they are recorded here as setup evidence, outside every
  order of stage 3. `worker-1` alone (ending 09:12:53); then `verifier-1`, `worker-2` and
  `verifier-2` at once (09:14:28 to 09:15:04), the first parallel use of one shared login; then a
  second turn on `worker-1` in the same conversation (09:15:18). Each exited 0 with a structured
  report naming its own working copy and no change, the second naming the first. Each end record
  holds `usage`, with client 0.167.1 and the model `gpt-6-astra`; none holds `limits`. `reader-a`
  took none, by the owner's ruling above.
- **Step 9.** The records were exported before the TERM: 3,983 lines, nothing on the error
  output. The TERM went at 09:15:57; serve exited after 44 s and printed its ready line 183 s
  after the TERM (stage 2's restarts took 19 to 24 s). Every instance came back ready with its
  setup `succeeded`, no action was pending, and root's tools held the twenty, `turn-reader-1`,
  seven `root-logins.*` and nine `logins.*`.
- The owner stopped no account; the account map is as registered.

**The records exports** stay uncommitted until scrubbed. Each is then committed as
[DEC-058](../../docs/decisions/log.md#dec-058-2026-11-22-records-exports-committed-by-order)
says: every account name replaced by its label, checked by the private-paths guard, and split
into each order's results directory. They are 13:18 on 2026-11-22, 2,482 lines; 16:14, 2,904
lines; the smoke turns', 3,983 lines; and step 9's, 3,983 lines. *Coordinator's*: the parts that belong to no
order, the smoke turns among them, stay uncommitted unless the owner rules otherwise.

**Turns outside both stages.** Every turn on stage 2's Agents after order 239's merge, those on
the engine at `M` included, belongs to neither stage (DEC-059). The coordinator's notes list each order whose turns ran on stage 2's Agents after that merge. Orders 292 and 295 had their first turns before step 9, so they are not stage 3's (the
registration). *Coordinator's*: their reads and fixes go to stage 3's Agents only after this
amendment merges, and count toward neither stage 3's first five orders nor pair 4's five.

**Gaps**, each in the coordinator's notes (private), to be ordered as engine and package work
(ruling 92, points 2 and 9): the engine and archives built by the retired harness's setup step;
serve started and restarted by hand, by step 1's command line in a shell wrapper; the render, by
a one-time Python script, against
[HOW-WE-WORK's rule 2](../../HOW-WE-WORK.md#1-the-rules-of-the-work) (ruling 92, point 2); the
working copies cloned by hand; the account lines' copies and their checks, until `value.v1`
(DEC-076) replaces them (DEC-068); and a cancelled turn on a stage-2 Agent, which only a restart
of serve cleared.

## Amendment of 2026-11-24: a fifth pair on the Business seat

Amended on 2026-11-24 (PST), before the fifth pair's first turn, by the coordinator outside every
turn, as [DEC-203](../../docs/decisions/log.md#dec-203-2026-11-24-a-fifth-pair-on-the-mac-this-week-on-the-business-seat)
asks: more lanes come first
([DEC-182](../../docs/decisions/log.md#dec-182-2026-11-24-everything-stays-in-v0-more-accounts-and-machines-come-first)),
and one account's login serves several Agents (DEC-042, DEC-046), so a fifth pair needs no new
account and no sign-in. The bar on orders that change this directory covers orders sent through
the Fleet (DEC-057), so this breaches nothing.

**The documents.** [The fifth pair's document](stage-3/fleet/pair-5.yaml) adds `worker-5` and
`verifier-5` from the `worker-role` and `verifier-role` templates, sharing the working copy
`<slot-5>`, on stage 3's login Resource `second-chatgpt` with pairs 3 and 4; and root's Conduits
`root-worker-5` and `root-verifier-5` from the Codex package's `conversation` template. Its
manifest is stage 3's. One placeholder is added, filled only in the rendered copy and never
committed: `<slot-5>`, the absolute path of the fifth pair's working copy (class path).

| Agent | Type | Account | Directory | Deadline | Root's Conduit |
| --- | --- | --- | --- | --- | --- |
| `worker-5` | `codex.v4` | `second-chatgpt` | `<slot-5>` | 14400 s | `root-worker-5` |
| `verifier-5` | `codex.v4` | `second-chatgpt` | `<slot-5>` | 3600 s | `root-verifier-5` |

**The steps**, each recorded with its evidence by a later dated amendment: the coordinator clones
`<slot-5>` at `main` and renders the document as one part; after this amendment merges, submits
it and sets it up, with no sign-in, and root's tools then hold `root-worker-5.*` and
`root-verifier-5.*`; then sends the pair its first turn.

**What it changes.** The fifth pair works as pairs 1 to 4 do and is judged by the same
conditions; its turns count toward stage 3's measures from its first turn. The Business seat's
usage is now shared by six Agents; until the usage guard's chain is merged and the login
Resource is its `.v2`, the coordinator watches it by hand, as for the team seats (DEC-179), and a
new account is asked for only when the seat's limit binds. The pair stays on this machine: a node
never moves (PLC-009), so the first pair on another machine is a sixth
([DEC-202](../../docs/decisions/log.md#dec-202-2026-11-24-the-first-remote-lane-is-a-new-pair-on-the-windows-node)).
*Coordinator's*: the Mac's load with five building pairs is measured from the turns' durations
and reported with the stage's measures.
