# Work order 108 (MC4): placement and a lost machine, model-checked across two machines

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. The machine is shared:
build with `CARGO_BUILD_JOBS=3`, run one cargo command at a time, and never print a whole large
file or log: use `head`, `tail`, `grep` or a count.

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). A40 names two
machines, with message loss, reordering, a partition and a lost machine, and ruling 47 makes the
safety promises holding under them a condition of v0. This order models what happens to
instances placed on a second machine: how they are placed, what an unreachable machine and a
gone machine do, and how a run is asked for, stopped and reported. It covers 22 safety promises
in [the catalogue](../implementation/SAFETY_CATALOGUE.md), all of the convention layer:

- in `spec/promises/placement.yaml`: PLC-002, PLC-012, PLC-017, PLC-019, PLC-020, PLC-023,
  PLC-024, PLC-026 and PLC-027;
- in `spec/promises/providers.yaml`: PRV-002, PRV-005, PRV-007, PRV-008, PRV-009, PRV-010,
  PRV-012, PRV-014, PRV-015, PRV-019, PRV-021 and PRV-027;
- in `spec/promises/pools.yaml`: POOL-016.

Read:

- experiment 012's README and `src/`, MC2's and MC3's modules above all, for the form a model
  takes there;
- the catalogue's rows for the 22 promises, and the promises themselves;
- the promises the properties lean on, outside the 22: PLC-009, LIF-025, LIF-036, TPL-018 and
  OBS-003;
- E15 in [ROADMAP.md's table of round 6](../../ROADMAP.md#round-6s-evidence-and-measurements),
  and S3's pre-registration,
  [experiments/014-s3-two-machines](../../experiments/014-s3-two-machines/README.md), for the
  faults S3 will make on real machines.

## The decisions it applies

1. **Where it lives.** A new module under `experiments/012-model-checking/src/`, and its section in
   the README. The existing models stay unchanged and passing.
2. **Every promise accounted for.** The README lists each of the 22 promises once, either as
   modelled, with the property that checks it, or as not modelled, with the reason.
   - **Not modelled here:** POOL-016 needs a Pool and repeats PLC-002's placement, so it is
     listed as not modelled. A static rule with no race and no fault, such as PRV-012, may carry
     the reason "a static decision, to be shown by vectors".
   - **Enrollment** is *spec pending, needed before S3*, so its promises are not yet written; the
     README says so.
   - **Supporting promises.** The ones the properties lean on (PLC-009, LIF-025, LIF-036,
     TPL-018, OBS-003) are named as supporting, outside the counts.

   This list is MC8's input.
3. **The model.** It models the promises, not the engine's code.
   - **Machines.** M1 holds the engine and A. M2 holds the runs.
   - **Nodes:**
     - **the engine.** It also holds the state of the provider servers it runs. When the engine
       crashes, they end with it (PRV-002), and they come back with its recovery;
     - **M2**, one node, holding the runs and the runtime provider's part there. That part keeps
       what PRV-021 requires it to keep: a stop it could not carry out, in saved state that
       neither the engine's restart nor the provider's retirement loses;
     - **A**, a consumer that sends requests to an instance.
   - **Instances.** I and a second instance J are placed on M2's node by the machine provider
     (PRV-015). After I is retired, a replacement I′ may be registered and placed.
   - **The runtime provider** starts a run on every `run.v1` it receives, and does not
     deduplicate. A run may exit on its own, and the provider may then restart it (PRV-010).
   - **The machine provider's reports, and the runtime provider's answers to `inspect-run.v1`,**
     are local actions, enabled when what the provider sees changes. They are not timers.
   - **"Unreachable"** means the engine's latest report from the machine provider says M2 is
     unreachable, or omits it, or that no report has come since the engine started. It is not
     the cut itself.
   - **Script:**
     1. The machine provider reports M2, and the engine registers M2's node (PLC-026); its
        setup is a separate later step.
     2. The engine sets I and J up. I's template selects a retire hook, run under TPL-018.
     3. A sends requests to I.
     4. The engine retires I, and then unregisters it (LIF-036).
     5. The root Interface may declare M2's node lost, once.
   - **Links.** The engine's links to the provider servers it runs are inside the engine node
     and take no fault. Duplication has a budget of zero, because the engine's provider calls
     are sent once, over stdio; the README says so.
4. **The ledger.** Every property reads the ledger, never a node's own state. It logs:
   - each run that starts, exits or stops, of which instance, on which machine;
   - each `run.v1` and `stop.v1` asked for, whom it named, and each answer;
   - each stop carried out late;
   - each report the engine received from the machine provider or the runtime provider, with
     what was true when it was made;
   - each status the engine reports, with the report it rests on;
   - each request dispatched, and each answer: an outcome, unavailable or unknown;
   - each retirement and its reported shutdown, each hook dispatched, each secret read for a
     run, each node registered, and each declaration of a node lost.
5. **Properties.** Each is named with its promise IDs, and checked in every reachable state.
   - **At most one live run** (PRV-007, PRV-010, PRV-014): runs of I and I′ together never
     overlap. The engine asks for no run of an instance while its latest answer for that run is
     unknown, and nothing is dispatched only because a run restarted. Root's declaration of a
     lost node is excepted, since root takes that risk.
   - **Unreachable is not gone** (PLC-023): while M2 is unreachable, as decision 3 defines it,
     nothing is retired for that reason, though root may still declare the node lost. A request
     to an instance on M2 is not dispatched and is answered unavailable. No run is asked for and
     no secret is read.
   - **A lost node retires with its shutdown unknown** (PLC-017, PLC-024, PLC-027). Once M2's node
     is declared lost, by root or because M2 is reported gone, the node and every instance on it
     are retired. For an instance that was set up, whose template selects a retire hook, the
     status never reports the shutdown as confirmed. No retire hook is dispatched, and no run is
     asked for one. LIF-025's case, an instance never set up, and PLC-027's case of no hook, are
     excepted as those promises say. Report as an ambiguity whether LIF-025 applies to the node
     itself, whose setup dispatches nothing.
   - **Stops are kept and addressed** (PRV-008, PRV-021, PRV-027). A stop is "asked" when
     `stop.v1` is answered unknown. Two parts:
     - in every state, each stop names the instance it is for, and the machine of that
       instance's node, and no other run is stopped by it;
     - in every quiescent state, no stop that was asked is left uncarried. A quiescent state is
       one with no message in flight, no action enabled, and M2 reachable.

     Carrying out a kept stop is an untimed local action of M2's provider part.
   - **Runs only where placed** (PLC-002, PRV-015, PRV-019): every run of an instance executes on
     the machine its placement named, and on no other.
   - **Status rests on a true report** (PRV-009, with OBS-003 supporting): every status of
     Running True or False rests on a report that was true when it was made, and carries that
     report's time. While M2 is unreachable, or before any report since the engine started, the
     status is Unknown.
   - **Setup waits for a ready node** (PLC-019): a setup of an instance while its node is
     registered and not ready is not dispatched, and is answered unavailable.
   - **A returned machine gets a new node** (PLC-026): a machine reported again after its node was
     retired is given a new node, never the retired one.

   These reachability properties stand beside the harness's own:
   - M2 unreachable and then reachable again;
   - M2 gone, and root's declaration;
   - a stop carried out late;
   - a request answered unavailable during a cut;
   - a run that exits and restarts;
   - I′ placed;
   - M2 returning with a new node.
6. **Faults.**
   - Partition: one cut, between the engine and M2, and its heal.
   - A lost machine: M2, and nothing else.
   - The engine's crash and recovery, with its state saved.
   - Loss and reordering on the links between the engine and M2, and between A and the engine.

   Budgets are one each. Consult `fault_allowed` for every fault kind, adding to the harness
   what MC3 did not, so that each fault stays on the nodes and links it bears on. Say why for
   each restriction in the README.
7. **Configurations.** Name them in the README. One combined configuration has a cut, a heal,
   the engine's crash and its recovery together, since the volatile-stop variant needs all four
   in one run. Split the rest by fault, as MC2 did, where that keeps runs small.
8. **Broken variants**, each with the property it is expected to break:
   - an unreachable machine treated as gone, so I is retired and run again (unreachable is not
     gone; at most one live run);
   - a run asked for while the latest answer for it is unknown (at most one live run);
   - a stop kept only in volatile state (stops are kept and addressed);
   - a stop that names J's run for I (the same);
   - a lost node whose instances are reported stopped (a lost node retires with its shutdown
     unknown);
   - a retire hook dispatched on a lost node (the same);
   - a run started on a machine other than the one placed (runs only where placed);
   - the last Running kept while M2 is reported unreachable (status rests on a true report);
   - a setup dispatched while its node is registered (setup waits for a ready node);
   - a returned machine given back its retired node (a returned machine gets a new node).
9. **The table, before any run.**
   - **What it gives.** The README pre-registers one row per configuration and variant, with
     every property's expected verdict and every reachability property's expected reach. A
     variant may break more than its named property; the table says which. The correct model
     runs in every configuration, and each variant only in the configuration it needs.
   - **Checks.** A test checks every full run against the table. One refutation run per variant
     prints its first trace on its named property.
   - **When a run disagrees.** Keep the table unchanged and add the observed one beside it.
     Report each difference as a finding.
   - **When a property breaks on the model as written.** Report it as a finding of class
     contradiction or gap, with the shortest trace and the promise IDs. Do not change the model
     to avoid it. A labelled control may stand beside it.
10. **Bounds.** Each run stays well under 200,000 unique states, and under a 10-second timeout,
    which is asserted after the run. The crate's whole `cargo test --release` should stay near a
    minute on `ubuntu-latest`, with the other models included. If a configuration passes its
    bound, split it, and say which faults each part enables.
11. **The README.** The model's section comes before its results and states:
    - the machines, the nodes, the instances, the script, the configurations and every bound;
    - the list of decision 2;
    - the parts left out, with reasons;
    - the table.

    Then add dated results:
    - for every configuration and variant: the unique states, the depth, the seconds, and the
      observed verdict of every property;
    - the first trace of each refutation;
    - any difference from the table.

    Close with what the model shows and what it does not, and how S3's runs on real machines
    stand beside it.
12. **No change to `spec/`.** A promise that cannot hold, or that the model shows to be
    ambiguous, is a finding in your report with a counterexample.

## Scope

Change only `experiments/012-model-checking/`:

- the new module and `src/lib.rs`;
- additions to `src/harness.rs` that its existing tests survive, with tests;
- the README.

Do not change any other file: not the existing models, `engine/`, `spec/`, `conformance/`,
`packages/`, `docs/`, `experiments/README.md`, any other experiment, ROADMAP.md, IN_FORCE.md or
any record or ruling. Report anything else that needs changing.

## Done when

In `experiments/012-model-checking/`, these pass:

- `cargo fmt --check`;
- `cargo clippy --all-targets -- -D warnings`;
- `cargo test --release`, and its wall time;
- `cargo test --release -- --nocapture --test-threads=1`, for the counts.

In your report's summary, give:

- the test totals and the wall time;
- for every configuration and variant: the unique states, depth, seconds and verdicts;
- every difference from the table;
- the counts of decision 2's list: modelled, and not modelled by reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes; new files are untracked, and these see
them:

```bash
grep -rn -E "/Users/[A-Za-z]|/privat[e]/tmp|/home/[a-z]" --exclude-dir=target experiments/012-model-checking
grep -rn -E "[[:space:]]+$" --exclude-dir=target --include='*.md' --include='*.rs' --include='*.toml' experiments/012-model-checking
```

`target/` stays ignored and unstaged. Do not stage anything.

## Record

Run on 2026-11-11 on the Mac, from a fixed base: the worker in 32 minutes (1,897 s), then a fresh
verifier in 8 minutes (450 s), which found four musts. A second round of 18 minutes (1,087 s)
fixed them:
- a request recorded as unknown later given an outcome;
- the provider's restart racing the retirement;
- shutdown checked only at the declaration;
- a blocked setup's answer.

A fresh verifier in 7 minutes (419 s) then found two more musts, both defects of the correct
model. A further retirement after M2 was reported gone asked for no stop, and a report that
omitted M2 registered a new node. Two rounds were spent, so the lane coordinator fixed both:
- a retirement of an instance whose node was declared lost asks for its stop whatever M2's
  health, since PLC-023 covers only instances on a node that represents the machine, and
  PLC-027 asks for the stop all the same;
- only an explicit report of reachable or unreachable registers a node (PLC-026).

Three regression tests, each run without its fix and failing, cover them. A fresh reader checked
the fixes. Its one remaining finding, the unreachable and omitted cases of the first fix, is
fixed too. Five rows' counts were measured again; no verdict changed.

The order added `placement.rs`, and to the harness `fault_allowed` for every fault kind, with an
opt-in so earlier models are unchanged. Of the 22 promises, 17 are modelled, 4 are static
decisions and 1 needs a Pool. Twenty-six bounded runs finished within their limits, the largest
154,232 states.

**What it found in the promises, for the admin's lane:**

- **MC4-009, a gap.** On the correct model, with no fault at all, the provider's own restart of
  a run (PRV-010) races the engine's `run.v1` for the retire hook (PRV-007, TPL-018). Two runs of
  one instance become live. No promise says what a provider does with `run.v1` for an instance
  whose run it has itself just restarted.
- **MC4-002, a gap.** A stop left pending after an unknown unregistration (LIF-036, PRV-021) does
  not fence a replacement instance. After the heal the replacement can start before the old run
  is stopped.
- **MC4-001, an ambiguity.** Does LIF-025's "no setup ever dispatched" apply to a node, whose
  setup dispatches nothing, when root declares it lost?

"At most one live run" is therefore refuted on the correct model in most configurations, as
these findings explain. The other eight properties hold.

The lane coordinator reran the crate's 33 tests, fmt, clippy and the repository's checks before
the merge.
