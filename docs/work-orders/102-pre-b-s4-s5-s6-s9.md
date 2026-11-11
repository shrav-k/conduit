# Work order 102 (PRE-B): the pre-registrations of S4, S5, S6 and S9

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This is a paper order: it builds no Rust and writes no vector, so its
`vectors` list is empty. It has no network access. Never print a whole large file: use `head`,
`tail`, `grep`, `sed -n` or a count.

## What and why

An order of the scenarios lane under [ruling 71](../proposals/ruling-71/README.md). Ruling 47
makes "S1 to S10 run end to end, with their vectors passing" a condition of v0. Ruling 45 asks
every experiment to state, before it runs, what it must show and how each measure is counted.
IN_FORCE.md topic 13 adds that the success criteria are fixed before the run. This order writes
those pre-registrations for four scenarios:

- S4, a setup that fails partway, is retried and loses a response;
- S5, a control change under contention;
- S6, access revoked and a Resource retired while requests are in flight;
- S9, first initialization, later start and explicit repair.

Sister orders 101 (S2, S3) and 103 (S7, S8, S10) write the others in the same form, at the same
time. The lane coordinators put every proposed criterion to the owner before any run.

Read:

- [ROADMAP.md](../../ROADMAP.md): [the scenarios](../../ROADMAP.md#scenarios),
  [round 6's evidence and measurements](../../ROADMAP.md#round-6s-evidence-and-measurements),
  and the register's rows A6 and A71;
- the cases and measurements that table names for S4 and S5, in
  [the critique](../proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage):
  E01, E03, E04, E09, E10, E16, E18, E19 and E20; D02 and D10. S6 and S9 have no row there;
- [IN_FORCE.md](../../IN_FORCE.md), topics 7, 8, 9 and 13;
- rulings [41](../proposals/ruling-41/README.md) (items 1 and 15: the setup hook replaced
  graphs, and a control applies exactly what it was sent or fails),
  [47](../proposals/ruling-47/README.md#when-to-publish),
  [68](../proposals/ruling-68/README.md) (point 2), [71](../proposals/ruling-71/README.md) and
  [72](../proposals/ruling-72/README.md);
- [the first build](../implementation/FIRST_BUILD.md): its success criteria, tasks, measures
  and counting rules; and [increment 4's measurement plan](../proposals/first-build/increment-4/measurement-plan.md),
  the model of a counted run;
- [engine/CONFORMANCE.md](../../engine/CONFORMANCE.md), for what the engine leaves unfinished
  today, DL1 among it;
- [conformance/s1-safety-cases.md](../../conformance/s1-safety-cases.md). Its traces
  S1-unknown-write, S1-reconcile, S1-close-in-flight and S1-setup-sender border S4 and S6;
- [experiment 007](../../experiments/007-s1-baseline/README.md) and
  [experiment 008](../../experiments/008-real-client/README.md), as models of a pre-registered
  experiment, and A71's results for what real clients did after an unknown outcome;
- the promises each scenario exercises, under `spec/`. Start from
  `spec/conventions/operations.yaml`, `lifecycle.yaml`, `revisions.yaml`, `records.yaml` and
  `access.yaml`; `spec/promises/instantiation.yaml`, `templates.yaml`, `contracts.yaml`,
  `schemas.yaml`, `first-start.yaml`, `pools.yaml` and `pending.yaml`; and
  `spec/interface/implementation.yaml`.

## The decisions it applies

1. **Where they live.** S4's is `experiments/015-s4-partial-setup/README.md`, S5's is
   `experiments/016-s5-control-contention/README.md`, S6's is
   `experiments/017-s6-revoke-in-flight/README.md` and S9's is
   `experiments/020-s9-start-and-repair/README.md`. Each holds a placeholder now: replace it.
   The scenario's dated results go beside it later. The index rows in `experiments/README.md`
   and the links from ROADMAP.md's table of scenarios are already in place; change neither. A
   scenario's row in that table is its row in the register, so add no register row.
2. **The form.** Every pre-registration of orders 101 to 103 has the same sections, so that the
   owner can take them together.
   - It opens with a box, "For the owner", with one line for each item labelled *proposed* and
     one for each open question.
   - Keep each README under about 250 lines.
   - A question shared with another scenario, such as which clients run, is asked in the same
     words in each README; the coordinator gathers them.

   Then come these sections, in this order:
   1. *Status*: "Pre-registered on DATE by work order 102. Awaits the owner's acceptance of
      what it proposes; nothing here runs before that. Ruling 45 fixes what an experiment must
      show before it runs, and IN_FORCE.md topic 13 lists the criteria the owner has accepted.
      No result is recorded here."
   2. *Question*: what the scenario settles, from its row of the roadmap.
   3. *What it must show*, as numbered pass conditions, each an observation with the way it is
      judged.
   4. *What it covers*: the cases and measurements of round 6 that the roadmap names for it;
      the promise IDs it exercises, by file, as `spec/` states them today; and the register
      items it feeds or overlaps.
   5. *The tasks*: numbered `S4-T1`, `S4-T2` and so on, each saying what the author does, in
      the form of the first build's T1 to T7. A fault the run injects is preparation, said
      apart from the author's tasks.
   6. *Measures*: decision 4, then each measure of its own, with how it is counted.
   7. *Vectors expected*: each proposed vector by a proposed name, with the promises it cites
      and whether it is a safety vector. A safety vector is one whose failure would be unsafe
      as [the tracker](../AUDIT.md) defines it (search for "mark as unsafe"), and it names the
      safety promise it tests. Name any vector under `conformance/` that already covers a case.
   8. *Runs*: who authors; which Agents and clients; the machines, accounts, services and
      provider packages each run needs; the owner's presence and time; how many runs; and what
      is recorded.
   9. *What it needs first*: the engine, specification and package work that must exist, by
      function and promise, and what the owner must set up.
   10. *What `spec/` does not yet decide*: each behavior the scenario needs that no promise
       states. Cite the IN_FORCE.md line marked *spec pending* where there is one.
   11. *Open for the owner*: each question with its options and a recommendation, marked as a
       recommendation.
   12. *Limits*: what the scenario does not show.

   Every pass condition, measure, counting rule and vector carries one of three labels:
   *ruled*, with its source; *accepted by the owner*, with its source; or *proposed, for the
   owner's acceptance*.
3. **Pass conditions.** Each scenario has ruling 47's: it runs end to end on the reference
   engine with its vectors passing, and every safety vector it adds passes. The owner's
   accepted criterion "the user resolves uncertainty without guessing that an effect failed"
   (IN_FORCE.md, topic 13) is S1's. S4 restates it for its own unknown outcomes, labelled
   proposed, citing its source. Every other criterion is proposed.
4. **Measures (the lane coordinator's decision, logged for the owner to strike).** The first
   build's measures were written for S1. They apply to every scenario's author runs as
   follows.
   - **Carried over**, cited by link and not restated:
     - the table of measures;
     - three of its counting rules: provider work apart; a client's concepts count; requests
       and intervals without access;
     - ruling 63's count of lines, with the panel's actions listed apart (ruling 68, R1);
     - IN_FORCE.md topic 13's rules for each run.
   - **Not carried, because they are S1's alone**:
     - the comparison with A70 and its bar (ruling 71, point 3);
     - the rules for T3's reach around Conduit and for T4 and T5 against the better baseline;
     - S1's local name `project`.

   No scenario here has a numeric bar, and observed time and mistakes are recorded before any
   target is set (ruling 46, point 3). A scenario may propose a comparison, labelled proposed.
   D02 and D10 are defined with how each is counted:
   - **D10** serves S4 and S5. S4 holds what the actor can conclude, verify and do next after
     an unknown outcome, a conflict and a partial setup. S5 holds what `ruleId`, the
     invocation's identity and its status let a frontend tell apart in a conflict.
   - **D02**'s matrix is S4's, in full: clients that keep the key and clients that send none,
     against plain and deduplicating implementations, after a lost success, after an unknown
     outcome, and for an intended second action.
5. **Agents and clients.** Ruling 47 asks for real Agents in S1 and S2 only. These four
   scenarios run with the engine's own clients or scripted ones, unless a case needs what only
   a real client shows. Name each such case, such as E09's client that sends no key, with A71
   and A6 as what it overlaps. Then give the owner the options:
   - a real client on the owner's subscription;
   - a real client on a second subscription account signed in through Conduit's own sign-in
     flow, as ruling 72 settled for the S3 and S10 replays;
   - the case left to A6.
6. **Fault injection.** S4's faults are made by servers that the scenario packages:
   - a plain server that makes part of an effect and then answers `-32021`;
   - a task that fails;
   - a lost reply;
   - a hook's lost reply.

   Describe each server's behavior, and leave how it is built to its order. The faults belong
   to the run and are not the author's work.
7. **No new language.** A behavior a scenario needs that `spec/` does not state is a finding of
   class gap in your report, and a line of section 10. No promise is drafted, and no task
   assumes an answer. This covers S5's controls and their reconciliation, S6's order of
   retiring several Resources (DL1), and S9's repair, wherever `spec/` leaves them open.

## Scope

Change only the four READMEs named in decision 1.

Do not change any other file: not `spec/`, `conformance/`, `engine/`, `packages/`, ROADMAP.md,
`experiments/README.md`, IN_FORCE.md, docs/AUDIT.md, any other experiment, or any record or
ruling. Report anything else that needs changing.

## Done when

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" experiments/015-s4-partial-setup experiments/016-s5-control-contention experiments/017-s6-revoke-in-flight experiments/020-s9-start-and-repair
grep -rn -E "[[:space:]]+$" experiments/015-s4-partial-setup experiments/016-s5-control-contention experiments/017-s6-revoke-in-flight experiments/020-s9-start-and-repair
```

Report each gap as a finding, and each proposed criterion, measure and vector as one of your own
choices. Do not stage anything.

## Record

Run on 2026-11-11 beside orders 100, 101, 103 and 104: the worker in 15 minutes (920 s), then a
fresh verifier in 5 minutes (317 s). That verifier found one should: S9's repair task covered a
failed setup but not the recovery of an unknown one. A second round of 2 minutes (136 s) added
an unknown-hook branch. A fresh verifier in 4 minutes (246 s) then found one must: the new task
asked for no hook call at restart in both branches, but the engine's startup asks again for
outstanding setup, and a failed hook may be dispatched as a new attempt. Only an unknown hook is
held back until it is reconciled (OPS-021). Neither verifier could rerun the tools' unit tests in
its read-only sandbox.

The lane coordinator then finished the work, as the order's process allows after a second round:

- **S9.** In branch (a), the failed hook, S9-T4 now records each attempt at restart. In branch
  (b), the unknown hook, it keeps the requirement of no hook call before reconciliation. M2 and
  V4 say the same.
- **A6.** Under the admin's decision of 2026-11-11 (A6 option b, logged for the owner to
  strike), S4 records A6's lost-response trace. S5, S6 and S9 no longer offer A6 as where a
  real-client case goes; their option is now "none (scripted clients only)".

A fresh reader then checked these edits. It found one must: S4's Q3 had lost its option of no
real-client replay. That option is restored, and A6's trace comes from the replay if one is
chosen, and otherwise from S4's scripted runs. It also found smaller faults in S9's wording, all
fixed: the restart's attempt in branch (a) is now certain rather than possible, and V4 is split
by branch.

The worker's findings, all of class gap, are carried in section 10 of each pre-registration:

- G1: no general form records that only part of an effect was done. This overlaps finding 4-12
  of increment 4.
- G2: E18's Resource surviving under a retired Pool has no public way to be prepared.
- G3: the field that names a vector's safety promise is still spec pending.
- A gap in S9's unknown-hook recovery, fixed in round 2.

All four pre-registrations await the owner's acceptance. The coordinator ran check_docs,
check_rulings, set_names, check_schemas and check_roadmap, and the private-path grep, all clean.
