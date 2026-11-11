# Work order 101 (PRE-A): the pre-registrations of S2 and S3

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
those pre-registrations for S2, one service login shared by two Agents with different
restrictions, and for S3, S1 and S2 across two machines with a partition and a lost machine.
Sister orders 102 (S4, S5, S6, S9) and 103 (S7, S8, S10) write the others in the same form, at
the same time. The lane coordinators put every proposed criterion to the owner before any run.

Read:

- [ROADMAP.md](../../ROADMAP.md): [the scenarios](../../ROADMAP.md#scenarios),
  [round 6's evidence and measurements](../../ROADMAP.md#round-6s-evidence-and-measurements),
  and the register's rows A6, A73 and A74; and A63's row in [docs/AUDIT.md](../AUDIT.md);
- the cases and measurements that table names for S2 and S3, in
  [the critique](../proposals/reaudit/round-6/design-critique.md#proposed-evidence-for-the-next-stage):
  E02, E07, E13, E15 and E17; D01, D05, D09 and D20;
- [IN_FORCE.md](../../IN_FORCE.md), topics 9, 10, 12 and 13;
- rulings [46](../proposals/ruling-46/README.md) (points 3 and 5),
  [47](../proposals/ruling-47/README.md#when-to-publish),
  [49](../proposals/ruling-49/README.md), [68](../proposals/ruling-68/README.md) (points 2 to
  4), [71](../proposals/ruling-71/README.md) and [72](../proposals/ruling-72/README.md);
- [the first build](../implementation/FIRST_BUILD.md): its success criteria, tasks, measures
  and counting rules; and [increment 4's measurement plan](../proposals/first-build/increment-4/measurement-plan.md),
  the model of a counted run;
- [the multi-machine proposal](../proposals/multi-machine/README.md), sections 2 and 11. Its
  section 2 numbers its own walk-throughs "S1." to "S9."; those are not the program's
  scenarios, so do not cite them as such. Read also the
  [secrets proposal](../proposals/secrets/README.md);
- the guides [1Password and Linear](../guides/1password-and-linear.md),
  [a second machine](../guides/second-machine.md), [accounts](../guides/accounts.md) and
  [Docker](../guides/docker.md);
- [experiment 005](../../experiments/005-macos-keychain/README.md), for the keychain;
  [experiment 007](../../experiments/007-s1-baseline/README.md) and
  [experiment 008](../../experiments/008-real-client/README.md), as models of a pre-registered
  experiment;
- the promises each scenario exercises, under `spec/`. Start from `spec/promises/secrets.yaml`,
  `conduits.yaml`, `placement.yaml` and `providers.yaml`; `spec/conventions/access.yaml` and
  `operations.yaml`; `spec/core/requirements.yaml`; and `spec/interface/implementation.yaml`.

## The decisions it applies

1. **Where they live.** S2's is `experiments/013-s2-shared-login/README.md` and S3's is
   `experiments/014-s3-two-machines/README.md`. Each holds a placeholder now: replace it. The
   scenario's dated results go beside it later. The index row in `experiments/README.md` and
   the link from ROADMAP.md's table of scenarios are already in place; change neither. A
   scenario's row in that table is its row in the register, so add no register row.
2. **The form.** Every pre-registration of orders 101 to 103 has the same sections, so that the
   owner can take them together.
   - It opens with a box, "For the owner", with one line for each item labelled *proposed* and
     one for each open question.
   - Keep each README under about 250 lines.
   - A question shared with another scenario, such as which accounts sign in, is asked in the
     same words in each README; the coordinator gathers them.

   Then come these sections, in this order:
   1. *Status*: "Pre-registered on DATE by work order 101. Awaits the owner's acceptance of
      what it proposes; nothing here runs before that. Ruling 45 fixes what an experiment must
      show before it runs, and IN_FORCE.md topic 13 lists the criteria the owner has accepted.
      No result is recorded here."
   2. *Question*: what the scenario settles, from its row of the roadmap.
   3. *What it must show*, as numbered pass conditions, each an observation with the way it is
      judged.
   4. *What it covers*: the cases and measurements of round 6 that the roadmap names for it;
      the promise IDs it exercises, by file, as `spec/` states them today; and the register
      items it feeds or overlaps.
   5. *The tasks*: numbered `S2-T1`, `S2-T2` and so on, each saying what the author does, in
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
   engine with its vectors passing, and every safety vector it adds passes. These are ruled
   too:
   - S2: Claude Code and Codex, run by the engine as Agents, carry out S2's tasks through it,
     with a record for each request. Simulated clients alone do not pass (ruling 47), and the
     fallback of two Claude Code instances is void (ruling 68, point 4);
   - S2, accepted by the owner (IN_FORCE.md topic 13; ruling 46, point 3): its restrictions
     differ as intended without copying the shared service credential to the Agents;
   - S3: it runs on real machines (IN_FORCE.md topic 10; rulings 47 and 68, point 4).

   Every other criterion is proposed.
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
   This order owns D01, D05, D09 and D20, each defined with how it is counted:
   - D01's three implementations, with the adapter work counted, are S2's; S7 and A63 reuse
     its definition;
   - D09 is defined here on S2's store, and S10 repeats it on the swapped store.
5. **Agents.** S2 needs Claude Code and Codex as above, signed in as A4 and increment 3 did.
   S3's replays of S1 and S2 use second subscription accounts that the owner provides. Each is
   signed in through Conduit's own sign-in flow (ruling 49), so the runs also test Conduit's
   authentication. No API key is used (ruling 72, point 2). Say which run needs which account,
   and which steps of the sign-in flow each run exercises. Where the flow is not yet in `spec/`
   or the engine, that goes in section 9 or 10. Note
   [A8's finding](../../experiments/010-subscription-terms/results-2026-11-10.md) that a Codex
   login cannot be shared between Agents.
6. **S2's service and store.**
   - **The service.** S2's shared login is the test workspace of Linear that
     [the guide](../guides/1password-and-linear.md) sets up, reached with one key that no Agent
     is given. Its vectors use a local stub of the service instead, so that they run with no
     account and no network.
   - **The store.** S2's secret store is the macOS keychain, the first packaged secret
     provider, which starts with S2 (ruling 46, point 5; IN_FORCE.md topic 12). The file store
     ships with the engine.
   - **A22's open case.** A22's daemon case is open: experiment 005 read the keychain only in
     the owner's logged-in session. Say what S2 needs of it.
   - **1Password.** The guide's route through 1Password for the key would add a second store
     in S2, and waits for A73. Give it as an option for the owner, not as the default.
7. **S3's machines.**
   - **The second machine** is the owner's Windows computer, set up as
     [the guide](../guides/second-machine.md) says. It is available whenever the work needs
     it, and no rented machine stands in (ruling 72, point 3). A partition and a lost machine
     are made as the guide's section "How S3 cuts the network" says.
   - **Enrollment** is *spec pending, needed before S3* (IN_FORCE.md topic 10). List it in
     sections 9 and 10. Note that the guide's manual setup serves only until enrollment exists.
   - **The machines contract.** The second implementation of the machines provider contract
     (A63: the local machine and a second host) is exercised in S3, not in S10.
   - **A74.** S3 uses A74's results once they exist, and A74 runs first. Say which of A74's
     hypotheses H1 to H10 each S3 task depends on.
8. **No new language.** A behavior a scenario needs that `spec/` does not state is a finding of
   class gap in your report, and a line of section 10. No promise is drafted, and no task
   assumes an answer.

## Scope

Change only `experiments/013-s2-shared-login/README.md` and
`experiments/014-s3-two-machines/README.md`.

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
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" experiments/013-s2-shared-login experiments/014-s3-two-machines
grep -rn -E "[[:space:]]+$" experiments/013-s2-shared-login experiments/014-s3-two-machines
```

Report each gap as a finding, and each proposed criterion, measure and vector as one of your own
choices. Do not stage anything.

## Record

Run on 2026-11-11 beside orders 100 and 102 to 104: the worker in 12 minutes (721 s), then a
fresh verifier in 4 minutes (253 s). The verifier found one should: S3's tasks could finish
without making a pending stop, which left the recovery of an owed stop untested. A second round
of 2 minutes (134 s) added a step for it, and a fresh verifier in 4 minutes (252 s) found
nothing. Neither verifier could rerun the tools' unit tests in its read-only sandbox.

The worker replaced the two placeholders with pre-registrations in the form of decision 2,
every proposed item labelled. S3's proposed enrollment vector names no safety promise, because
enrollment's promises are not yet written; that was the worker's nearest reading of decision 2.
Its findings, all of class gap, are carried in section 10 of each:

- G1: SEC-008 lets an Agent read a secret even when others drive it. The rule against that is
  spec pending, batch 2.
- G2: the issue-scope and deadline restriction, the keychain store's locator and session, and
  the clients' own-tool controls belong to package contracts.
- G3: the field that names a vector's safety promise is still spec pending.
- G4: managed enrollment has no promises yet. IN_FORCE.md topic 10 says they are needed before
  S3.
- G5: PLC-024 asks for the cause of a machine's loss in records, with no portable field for it.

After the review, the lane coordinator made one change. Under the admin's decision of
2026-11-11 (A6 option b, logged for the owner to strike), S2's pre-registration now says which
of its tasks yield A6's traces: discovery and invocation, and the refresh, read as S2-T5's
rotation. The lost response comes from S4. Both pre-registrations await the owner's acceptance.
The coordinator ran check_docs, check_rulings, set_names, check_schemas and check_roadmap, and
the private-path grep, all clean.
