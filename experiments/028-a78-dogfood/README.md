# 028 — A78: Conduit runs its own orders
Not binding.

This is A78's directory in the [roadmap's register](../../ROADMAP.md#experiments-and-probes).
Stage 1 is orders 1 to 3, which root sends through turns on Agents of the `conduit-core` Fleet
([ruling 70](../../docs/proposals/ruling-70/README.md);
[ruling 61, B12](../../docs/proposals/ruling-61/README.md)). A78's claim, pass conditions and
measures for orders 2 and 3 are
[registered below](#orders-2-and-3-registered-before-order-2-runs), before order 2 runs; those
for orders 4 and 5 are registered before order 4 runs.

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

[The plan](inputs/plan.md) is what [ruling 74's record](../../docs/proposals/ruling-74/README.md)
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
| `NNN-name/`, from order 2 on | One directory per order: its scrubbed records export, the harness's log and measures, its reports and its dated result, as the registration below says |

## What it cannot show

Every result here is obtained with every Agent under one operating-system user, the owner's
([ruling 84](../../docs/proposals/ruling-84/README.md), point 2). No dedicated macOS user is
planned: ruling 84, point 1, amends [ruling 75 B3](../../docs/proposals/ruling-75/README.md), and
isolation is to come later from a sandbox method, whose design states whether a result obtained
inside one shows Agents kept apart. Under one user, an Agent that runs code can read, use or
replace another Agent's login and act as root's Interface, so no result here shows Agents kept
apart ([the full review's D60](../../docs/proposals/full-review/decisions.md)). A guard that held is
not proof either: the guards read a slot that the Agents they check can write
([finding 4-22](../../docs/proposals/first-build/log-increment-4.md)).

The records say which requests were made, by whom, and how each ended. They say no more.
What an Agent did in its slot with its own tools is not a Conduit request, so it is not in
the records (CTR-014, PRV-006, ACC-027). In stage 1 every caller is root, so the records do
not test attribution among callers. An order run through Conduit is not evidence about
access unless the results count a refusal, an open or a close. The reports of 042's worker
and verifier are not in this directory.

## Orders 2 and 3: registered before order 2 runs

Registered on 2026-11-15 by work order 170, before order 2 runs through Conduit
([ruling 45](../../docs/proposals/ruling-45/README.md)). It holds for orders 2 and 3, which root
sends. Orders 4 and 5, which a coordinator Agent sends, are registered before order 4 runs, once
the owner has answered what they test (D59). This text changes only by a dated amendment, made
before the first order it changes runs. Its choices, and the D-numbers it cites from
[the full review's decisions](../../docs/proposals/full-review/decisions.md), are the
coordinator's, each open to the owner's strike
([ruling 48](../../docs/proposals/ruling-48/README.md)).

Amended on 2026-11-19, before order 2 runs through Conduit, by the coordinator's change for
[ruling 84](../../docs/proposals/ruling-84/README.md), point 1. Under "The Claude verifier slot",
the slot was to be added "only after D58's guards and A22's open case have run (D61)"; it is now
added once D58's guards are in place, since A22 no longer runs before any Claude Agent becomes a
dogfood slot. Above this registration, "What it cannot show" no longer names the dedicated macOS
user of ruling 75 B3 as what comes later. No step, measure or pass condition changes. Ruling 84
assigns this correction to an order; the coordinator made it directly, since it changes no step,
measure or pass condition, and the same holds for experiment 027's amendment of the same date.

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
  [the review](../../docs/proposals/full-review/experiments.md#a78-the-conditions-per-order): P1,
  cuts of the network under the `.v2` wrapper and under a plain launch, and P2, a lost caller,
  before order 5; each is registered before it runs. P3, the process tree, the review moved to
  experiment 022 and A7, and P4, upgrading a signed-in Agent's package, it did not register.

## Stage 2: the multi-slot Fleet, written before 0.2.0

Written on 2026-11-19 by work order 210, before order 050n merges the Agent packages 0.2.0. It
registers nothing: no order has run in stage 2, and the claim, the measures and the pass
conditions of stage 2's orders are registered by a dated amendment, merged before the first of
them runs ([ruling 45](../../docs/proposals/ruling-45/README.md)). Its choices are the
coordinator's, each open to the owner's strike
([ruling 48](../../docs/proposals/ruling-48/README.md)).

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
  ([ruling 48](../../docs/proposals/ruling-48/README.md), point 1), and `reader-1` has the model
  `opus` and peer messaging off. Each names a directory, so its client's own tools are on and
  nothing confines them in v0 ([ruling 70](../../docs/proposals/ruling-70/README.md)).
- Each keeps its client's login in a home of its own inside its state directory
  ([order 050c](../../docs/work-orders/050c-agents-v2-client-home.md)), which keeps the accounts
  apart for the clients but is no wall against an Agent that runs code
  ([ruling 84](../../docs/proposals/ruling-84/README.md), point 2). Each names in `account` the
  owner's primary account of its client, and no Agent names a second account until the owner says
  so (ruling 84, point 3).
- Two pairs of a worker and a verifier, each pair in a slot clone of its own: the coordinator
  dispatches at most two Codex runs at once on the Mac, and each Agent costs the owner one
  sign-in. A third pair is four more documents and two more sign-ins.
- `reader-1` is [the Claude verifier slot](#the-claude-verifier-slot) of stage 1's registration:
  the fresh read on Claude Opus before each merge
  ([ruling 79](../../docs/proposals/ruling-79/README.md)). It works in a clone of its own, into
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
   ([D58](../../docs/proposals/full-review/decisions.md);
   [IN_FORCE.md, topic 15](../../IN_FORCE.md#15-how-the-work-is-governed)). A22 no longer runs
   first ([ruling 84](../../docs/proposals/ruling-84/README.md), point 1).
   Until then the harness may submit these documents without `reader-1` and `turn-reader-1`,
   and stage 2 begins with its Codex slots (ruling 89, point 5).
3. The owner has signed each of the five Agents in once, on the Mac, through `conduit sign-in`
   ([order 051b-2](../../docs/work-orders/051b2-cli-sign-in.md);
   [ruling 85](../../docs/proposals/ruling-85/README.md), point 5), one Agent at a time: Codex's
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
