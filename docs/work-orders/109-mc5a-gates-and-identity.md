# Work order 109 (MC5a): gates and identity, model-checked

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. This order writes no conformance vector, so its `vectors` list is
empty. It has network access to fetch crates; use it for nothing else. Build with
`CARGO_BUILD_JOBS` as your environment sets it, run one cargo command at a time, and never print
a whole large file or log: use `head`, `tail`, `grep` or a count.

## What and why

An order of the proof lane under [ruling 71](../proposals/ruling-71/README.md). A40 asks that each
safety promise of the shared conventions hold under model checking, or be listed with the reason
it cannot be checked this way (G5). Ruling 75, E1, counts every convention-layer safety promise in
[the catalogue](../implementation/SAFETY_CATALOGUE.md), 217 in all.

This order models how a run's server is reached only through its gate, how an instance's
identity is established and delivered, and how a run's identity and secrets reach a second
machine. It covers nine promises of `spec/promises/providers.yaml`: PRV-003, PRV-011, PRV-018,
PRV-022, PRV-023, PRV-024, PRV-025, PRV-028 and PRV-029. Its sister order 110 takes the reading
of secrets.

Read:

- experiment 012's README and `src/`, above all `operations.rs` (MC2) and `access.rs` (MC3).
  These show the form a model takes there, and the findings MC3 recorded against its own order:
  a property must not ask more than its promise does;
- the catalogue's rows for the nine promises, and the promises themselves;
- the promises the properties lean on, outside the nine: IFC-017, IFC-022, IFC-041 and IFC-042;
- `spec/README.md`'s section "What the guarantee for secrets covers", for the domain of trust.

## The decisions it applies

1. **Where it lives.** A new module, `experiments/012-model-checking/src/gates.rs`, and its section
   in the README, placed after MC3's results. Its runs take about 15 seconds of the crate's test
   time at most. **Do not change `src/harness.rs`:** another order is changing it at the same
   time. Its `fault_allowed` hook already governs reordering, loss, duplication, cut, crash and a
   lost machine. Where a fault cannot be restricted that way, guard the action in your own model,
   and say so.
2. **Every promise accounted for.** The README lists each of the nine promises once, either as
   modelled, with the property that checks it, or as not modelled, with the reason.
   - **Parts left out:** PRV-003's "on no other" machine, its machine provider and its fetching;
     and PRV-025's bound on how long a gate stays open through silence, a matter of time that
     the harness cannot check.
   - **Supporting promises** are named and kept out of the counts.

   The list is MC8's input.
3. **The model.** It models the promises, not the engine's code.
   - **M1.** The engine runs here, with A, a consumer that sends requests to instance I. The
     server of the runtime provider P runs inside the engine node, on the engine's own runner,
     and ends when the engine crashes. The engine's own runner also runs two instances, L1 and
     L2, each with its own state directory (PRV-029). After the engine's recovery, a further setup
     of each runs it again, as a local action of the engine (PRV-026 says such a run is not run
     again otherwise).
   - **M2.** One node holds three things:
     - P's part on M2, which receives `run.v1`'s secrets and identity over P's channel (PRV-011);
     - I's run, whose server answers at URL `u` behind its gate (PRV-025);
     - X, another process on M2. X is not root, not the operating-system user of I's run, and
       not P's user: IFC-022 and PRV-025 let those present as I or pass the gate.

     X acts by local actions of the M2 node. It may send a request to I's gate without proof.
     It may take `u` after I's run has stopped. It may send a request to the engine from the
     address I's run used. And it may observe P's channel when that channel does not
     authenticate M2.
   - **Identity.** I's run is given identity files: a certificate naming I's ID, and a token
     (PRV-022). I's run sends one request to the engine, presenting its token.
   - **Connections.**
     - The engine reaches `u` by a connect step that carries nothing.
     - What answers at `u` then proves, or fails to prove, that it serves I (PRV-028).
     - A proof belongs to its connection and is held in the engine's volatile state.
     - Messages on a proven connection are delivered only to the process that proved.
     - When X takes `u`, M2 sends the engine a reset, which may be lost or reordered. The
       engine then connects anew for its next request, and that connect reaches X.
     - The cut breaks every connection between M1 and M2. After the heal, the engine connects
       anew and needs a new proof.
   - **Crashes and restarts.** The harness's crash is the engine's alone. I's run stopping, and
     P's restart of it, are guarded local actions of the M2 node, so that they do not draw on the
     engine's crash budget.
   - **The cut** is between the engine and the M2 node, and severs everything between M1 and M2.
4. **The ledger.** Every property reads the ledger, never a node's own state. It logs:
   - each connection to `u`, and whether, and by which process, it was proved;
   - each message the engine sent, to which process, and what it held, by class: request
     contents, a secret's value, identity material, or nothing secret;
   - each request that reached I's server, and the proof it carried;
   - each request the engine received, the instance it was attributed to, and what the
     attribution rested on;
   - each place identity material and secrets were delivered or appeared: a run, a gate, a
     provider, a record, an inspection result, an observation, a diagnostic, a result or `_meta`;
   - the variables, files and state directory each run of the engine's own runner was given;
   - whether P's channel to M2 authenticated M2, and what X observed on it.
5. **Properties.** Each is named with its promise IDs and checked in every reachable state. Each
   asks only what its promise asks. Where a promise has a precondition, the property has it too.
   - **the gate admits only proof** (PRV-025): every request that reaches I's server carries
     proof made by the engine. No request X sends reaches it;
   - **proof before sending** (PRV-028): on a connection to `u`, the engine sends nothing but the
     connect step until the process that answers has proved, with the certificate for I's run,
     that it serves I. X receives no request contents, no secret and no identity material;
   - **identity rests on what is presented** (PRV-018): no request that X sends is attributed to
     I, whatever address it came from, and I's own request is attributed to I by its token;
   - **identity material stays with its holders** (PRV-022, PRV-024): I's identity files reach
     only I's run, its gate and P, the provider delivering them. No identity material appears in
     a record, an inspection result, an observation, a diagnostic, a result or `_meta`;
   - **the engine's own runner keeps its promises** (PRV-003, PRV-023, PRV-029):
     - L1 and L2 are given their identity and secrets as PRV-022 and PRV-011 have a provider give
       them;
     - each process is given its listed variables, and of the engine's own environment only
       those PRV-023 names. The engine's environment also holds `GITHUB_TOKEN`, which is not
       among those names, and the runner's pass list is empty; no process is given it;
     - each instance keeps the same state directory across the engine's restart, and never
       receives the other's;
   - **a channel to another machine authenticates it** (PRV-011, PRV-022). The model sets a flag
     for whether P's channel authenticates M2, and the README says this property checks that
     flag: `run.v1`'s secrets and identity
     reach M2 only over a channel that authenticates M2 and keeps them confidential. X observes
     no value.

   These reachability properties stand beside the harness's own, one for each property's
   precondition:
   - a proven request reaching I's server;
   - X's request refused at the gate;
   - X taking `u`, and a connect that reaches X;
   - X's request to the engine attributed to no instance;
   - I's own request attributed to I;
   - the engine's restart, with L1 and L2 run again;
   - P's channel carrying values to M2.
6. **Faults.**
   - Loss and reordering on the links between M1 and M2, and between A and the engine.
   - The cut of decision 3, and its heal.
   - The engine's crash and recovery, with its state saved.

   Budgets are one each. Restrict each fault with `fault_allowed`, and say why in the README.
7. **Broken variants**, each with the property it is expected to break:
   - a gate that admits a request without proof (the gate admits only proof);
   - the engine sends request contents on a connection before the proof (proof before sending);
   - a proof kept across a broken connection, so that the engine sends to X (the same);
   - a request attributed to I by the address it came from (identity rests on what is
     presented);
   - an identity file copied into a diagnostic (identity material stays with its holders);
   - `GITHUB_TOKEN` passed to L1 (the engine's own runner keeps its promises);
   - L2 given L1's identity directory or token (the same; PRV-003's delivery);
   - L2 given L1's state directory after the restart (the same);
   - P's channel that does not authenticate M2 (a channel to another machine authenticates it).
8. **The table, before any run.**
   - **What it gives.** The README pre-registers one row per configuration and variant, with
     every property's expected verdict and every reachability property's expected reach. A
     variant may break more than its named property; the table says which. The correct model
     runs in every configuration, and each variant only in the configuration it needs.
   - **Checks.** A test checks every full run against the table. One refutation run per variant
     prints its first trace on its named property.
   - **When a run disagrees.** Keep the table unchanged and add the observed one beside it.
     Report each difference as a finding.
   - **When a property breaks on the model as written.** First check whether the property asks
     more than its promise does, as MC3's did twice. If it does, report a finding of class
     ambiguity against this order's wording, and add a control with the promise's own condition,
     registered before its first run. If it does not, report a finding of class contradiction or
     gap with the shortest trace and the promise IDs. Do not change the model to avoid it.
9. **Bounds.** Each run stays well under 200,000 unique states, and under a 10-second timeout,
   which is asserted after the run. The crate's whole `cargo test --release`, with the other
   models, should stay near a minute on `ubuntu-latest`. If a configuration passes its bound,
   split it, and say which faults each part enables.
10. **The README.** The model's section comes before its results and states:
    - the nodes, the script, the configurations and every bound;
    - the list of decision 2;
    - the parts left out, with reasons;
    - the table.

    Then add dated results:
    - for every configuration and variant: the unique states, the depth, the seconds, and the
      observed verdict of every property;
    - the first trace of each refutation;
    - any difference from the table.

    Close with what the model shows and what it does not. The domain of trust's limits are
    stated as limits.
11. **No change to `spec/`.** A promise that cannot hold, or that the model shows to be ambiguous,
    is a finding in your report with a counterexample.

## Scope

Change only `experiments/012-model-checking/`: the new module, `src/lib.rs` and the README.

Do not change any other file: not `src/harness.rs` or the existing models, `engine/`, `spec/`,
`conformance/`, `packages/`, `docs/`, `experiments/README.md`, any other experiment, ROADMAP.md,
IN_FORCE.md or any record or ruling. Report anything else that needs changing.

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

Run on 2026-11-11 on the Windows computer (ruling 76, point 6), from a fixed base: the worker in
14 minutes (842 s), then a fresh verifier in 5 minutes (285 s); one round. The verifier found no
fault and reproduced every count and trace. The worker built with Rust 1.99.1, since 1.97.1 was
not installed there; the lane coordinator reran the crate under the pinned 1.97.1 on the Mac.

The order added `gates.rs`. All nine promises are modelled. The parts left out are PRV-003's
"on no other" machine, its machine provider and fetching, and PRV-025's bound on silence. Sixteen
bounded runs finished within their limits, the largest 5,110 states.

**No property of the current promises was found broken.** The six properties hold in all seven
correct configurations:

- the gate admits only proof;
- proof before sending;
- identity rests on what is presented;
- identity material stays with its holders;
- the engine's own runner keeps its promises;
- a channel to another machine authenticates it.

Each of the nine broken variants broke its named property, as the table registered. Every
reachability property was reached where the table expected. The worker's nearest reading of
decision 6: the harness does not consult `fault_allowed` for recovery or healing, so restricting
crashes to the engine and cuts to the engine and M2 limits those too.

The lane coordinator reran the crate's 21 tests, fmt and clippy under the pinned toolchain, and
the repository's checks, before the merge.
