# Work order 113 (MC7b): records, observation and installing packages, model-checked

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

This order models three things:
- the records kept of what happens, and their reading;
- observation, which must change nothing it observes;
- installing a package while sources and installs race.

It covers 20 promises:

- in `spec/conventions/records.yaml`: REC-001 to REC-006;
- in `spec/conventions/observation.yaml`: OBS-001, OBS-002, OBS-003, OBS-004 and OBS-006;
- in `spec/promises/packages.yaml`: PKG-023, PKG-024, PKG-025, PKG-027 and PKG-029;
- in `spec/promises/providers.yaml`: PRV-013 and PRV-017;
- in `spec/promises/naming.yaml`: NAM-023 and NAM-026.

The ten safety promises of pending actions, PND-001 to PND-012, are left to MC9, after batch 2's
specification; MC8 lists them so.

Read:

- experiment 012's README and `src/`, every model in it, and the findings recorded against
  earlier orders: MC2-001, MC3-001, MC3-002, and MC4's record of its model's own defects, in its
  section and in `docs/work-orders/108-mc4-placement-and-lost-machine.md`;
- the catalogue's rows for the 20 promises, and the promises themselves;
- the promises the properties lean on: PKG-026, PRV-010, PRV-016, PLC-023, ACC-032, ACC-033,
  OPS-008, OPS-012 and IFC-025.

## The decisions it applies

1. **Where it lives.** A new module, `experiments/012-model-checking/src/records.rs`, and its
   section in the README, placed after the others. Change `src/harness.rs` only by an addition
   that its existing tests survive.
2. **Every promise accounted for.** The README lists each of the 20 promises once, either as
   modelled, with the property that checks it, or as not modelled, with the reason.
   - **"A static decision, to be shown by vectors"** is enough for a rule with no race and no
     fault. Examples: OBS-002's fields, REC-002's fields, PKG-023, PKG-029, NAM-023 and
     NAM-026.
   - **Supporting promises** are named and kept out of the counts.

   The list is MC8's input.
3. **The design is the worker's, and it comes first.** Before any run, the README states the
   model: its nodes, its script, its configurations, its properties, its variants, and the
   table of expected verdicts. Each property quotes the `when` and `then` of the promise it
   checks, with every precondition. The script must reach each property's precondition in some
   configuration, and a reachability property shows that it does. Use up to three parts:
   1. records: events at two sources; a purge by root, and one asked for by another actor; a
      keyed repeat of a request made before the purge; and a reader that may meet records that
      cannot be read;
   2. observation: an instance observed while its machine is unreachable and while what is
      observed differs from what is declared;
   3. installing packages:
      - two installs of one address and version racing;
      - a package-source provider owned by `system` that claims a scheme, a second claimant, and
        a provider Resource not owned by `system`;
      - an archive that cannot be fetched, and one whose digest does not match.
4. **Faults that earlier reviews found, to be avoided.** The verifier checks each of these:
   - **A property that asks more than its promise.** Examples: a keyed repeat that MC3 required
     to dispatch, and a late reply that MC3 required to replace a recorded unknown.
   - **A precondition never reached.**
   - **A variant that breaks nothing.**
   - **A liveness claim.** Check quiescent states instead.
   - **A fault with no witness.** Reordering needs two messages on one link.
   - **Restricting faults.** Timers and recovery pass through `fault_allowed` once the protocol's
     `gate_remaining_faults` is true, and `fault_enabled` gives windows by state. Restrict timers
     with `timer_enabled`.
   - **Every fault with a budget must be reached in its run**, so split faults across
     configurations. Every budget of zero needs a reason.
5. **Readings to apply.** These are the coordinator's readings, logged for the owner to strike.
   - **Records** (REC-001, REC-003 to REC-006):
     - a record is kept of every event REC-001 lists;
     - nothing but a purge removes or alters a record;
     - only root may purge, and a purge removes only records made before its time. A purge
       leaves OPS-008's state of invocations alone, so a keyed repeat after it is answered as
       before, and no effect is applied a second time (REC-004);
     - records read from one source come in the order they were made there;
     - a gap is reported where records that should exist cannot be read.

     Records come with their sources' order and nothing more: no order is promised between
     sources. Keep records at their source in saved state; faults affect only the reading of
     them. Records removed by a purge are not a gap. REC-003 ("removed or altered by anything
     other than a purge") and REC-006 ("records that ... were lost") are in tension; report it
     as a finding of class ambiguity.
   - **Observation** (OBS-001, OBS-003, OBS-004, OBS-006):
     - observing never changes a declared definition, configuration or revision;
     - what cannot be observed now is reported unknown, with the time of the last successful
       observation, or none where it was never observed. Instances on other nodes report as
       before;
     - a difference between what is observed and what is declared makes the engine dispatch no
       setup, control, open or retirement. A runtime provider's own restart, or a machine coming
       back, may remove it (PRV-010);
     - inspecting or observing causes no effect and grants no access.
   - **Installing** (PKG-024, PKG-025, PKG-027, PRV-013, PRV-017):
     - an install of an address and version not installed, whose archive cannot be fetched now,
       is answered unavailable, and the installed packages are as they were;
     - an install at an address and version already installed with another digest is answered
       conflict, and fetches nothing;
     - one already installed with the same digest is decided like any other, so refused or
       invalid come first. Otherwise it succeeds, changes nothing and fetches nothing;
     - "already installed" is judged when the request is decided. No promise orders two first
       installs of one address and version: pre-register that race as a possible gap, and
       invent no rule for it;
     - at most one package-source provider is not retired for each scheme, and each is owned by
       `system` (PRV-013). The half about machine providers is outside this model;
     - an archive is checked against its digest before it is used (PKG-026), whether it was
       fetched through the provider that claims its scheme or by the engine's own `https` or
       `file` fetching (PRV-017).
6. **The ledger, the properties, the variants and the table.** As in MC2, MC3, MC4 and MC5a:
   - every property reads only the ledger, logged when each event is sent;
   - each safety property has at least one broken variant;
   - the table gives every property's expected verdict and every reachability property's
     expected reach, for each configuration and variant;
   - the correct model runs in every configuration, and each variant only where it is needed;
   - a test checks every full run against the table;
   - one refutation run per variant prints its first trace.

   When a run disagrees with the table, keep the table unchanged and add the observed one beside
   it. When a property breaks on the model as written, first check decision 4's first fault. If
   the property asks more than its promise, report a finding of class ambiguity against this
   order's wording, and add a control, registered before its first run. Otherwise report a
   finding of class contradiction or gap, with the shortest trace. Do not change the model to
   avoid it.
7. **Faults.**
   - Loss and reordering on the links that carry requests and records.
   - The engine's crash and recovery, with its state saved.
   - A record that cannot be read, as a model choice.
   - A machine that cannot be observed, as a model choice.
   - A fetch that fails, as a model choice.

   Budgets are one each, split across configurations.
8. **Bounds.** Each run stays well under 200,000 unique states and under a 10-second timeout, with
   a target near half a second. Together, this order's runs should take about 15 seconds of the
   crate's test time.
9. **No change to `spec/`.** A promise that cannot hold, or that the model shows to be ambiguous,
   is a finding in your report with a counterexample.

## Scope

Change only `experiments/012-model-checking/`: the new module, `src/lib.rs`, the README, and an
addition to `src/harness.rs` that its existing tests survive, if one is needed.

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
