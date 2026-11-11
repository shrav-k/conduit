# Work order 110 (MC5b): reading and confining secrets, model-checked

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

This order models how the engine reads a secret: for whom, through which Conduit, within which
invocation, and where the value may go. It covers 16 promises:

- in `spec/promises/secrets.yaml`: SEC-004, SEC-007, SEC-008, SEC-010, SEC-011, SEC-012,
  SEC-013, SEC-014, SEC-015, SEC-016, SEC-017, SEC-018, SEC-020, SEC-023 and SEC-024;
- in `spec/promises/providers.yaml`: PRV-026.

Its sister order 109 covers gates and identity, PRV-011 among them.

Read:

- experiment 012's README and `src/`, above all `operations.rs` (MC2) and `access.rs` (MC3).
  These show the form a model takes there, and the findings MC3 recorded against its own order:
  a property must not ask more than its promise does;
- the catalogue's rows for the 16 promises, and the promises themselves;
- the promises the properties lean on, outside the 16: PRV-002, PRV-007, PRV-010, REC-001,
  IFC-017, IFC-041, ACC-011, ACC-034 and IMP-007;
- `spec/README.md`'s section "What the guarantee for secrets covers".

## The decisions it applies

1. **Where it lives.** A new module, `experiments/012-model-checking/src/secrets.rs`, and its
   section in the README, placed last. Its runs take about 15 seconds of the crate's test time at
   most. **Do not change `src/harness.rs`:** other orders are changing it. Use the
   `fault_allowed` hook it has; where a fault cannot be restricted that way, guard the action in
   your own model, and say so.
2. **Every promise accounted for.** The README lists each of the 16 promises once, either as
   modelled, with the property that checks it, or as not modelled, with the reason.
   - **Not modelled here:** SEC-008 (who may open a Conduit listing `get-secret.v1`), SEC-024 (a
     template's rejection) and SEC-004 (the resolved form of a reference in a record) are static
     decisions "to be shown by vectors".
   - **Supporting promises** are named and kept out of the counts.

   The list is MC8's input.
3. **The model.** It models the promises, not the engine's code.
   - **Nodes:**
     - **the engine.** Inside it runs the server of a runtime provider P, on the engine's own
       runner. P's server ends when the engine crashes (PRV-002), and P's item names a secret.
     - **K**, a secret Resource that offers `get-secret.v1`.
     - **I's node**, holding instance I's run and P's part there. P's part keeps the value it
       was given for I's run, and P's restart of that run is a local action of this node
       (PRV-010; SEC-014).
     - **R**, an instance whose implementation item is reached by `remote` at URL `h`. Its header
       refers to a secret, and it is not `plain`, so the engine follows no redirect from it
       (IFC-017). The server at `h` may answer one request with a redirect to a second URL,
       `h2`.
     - **O**, an instance that, while serving a request from I, may send `get-secret.v1`
       onward.
   - **Conduits:**
     - I, P, R and O each have their own open Conduit to K listing `get-secret.v1`;
     - I has a Conduit to R and a Conduit to O;
     - P's Conduit to K may be closed once, and so may R's.
   - **Callers.** Each setup has its own caller, so that a read's caller can be checked
     (SEC-011).
   - **The script.** Every run, connection and HTTP request has an ID.
     0. P's setup reads P's secret through P's Conduit to K, and the engine's own runner runs P.
     1. I's setup reads I's secret through I's Conduit to K, just before the engine asks P for
        I's run (SEC-010).
     2. P restarts I's run once, by its own policy.
     3. The engine crashes once and recovers. After recovery, the engine makes one provider
        call, `inspect-run.v1` of I's run, as a local action. Before that call, P is run again,
        with its secret read again through P's Conduit as it stands then (PRV-026). I's run goes
        on across the restart, with no read for it (SEC-014).
     4. R's setup reads R's header secret through R's Conduit to K, then connects to `h`
        (SEC-013). It holds the value while the connection lasts, and sends it on each HTTP
        request.
     5. I sends two requests through its Conduit to R, and each becomes one HTTP request on the
        connection. After the engine's restart it no longer holds the value, so it reads again
        through R's Conduit before the next HTTP request (SEC-023).
     6. I sends `get-secret.v1` on its own behalf once. O sends it once as an onward request,
        while serving I's request through I's Conduit to O (SEC-007).
   - **Faults on reads.** Each of these is a local action, with a budget of one:
     - a read ends with a reason, or failed, or unknown;
     - the record of a read's dispatch cannot be kept (SEC-016).
   - **An answer that comes too late.** Once a read has ended, by a fault or by a restart, the
     engine discards any answer to it. A read dispatched before the restart belonged to an
     invocation that is now unknown (IMP-007).
   - **IDs.** P's run after the restart has its own ID, and I's run keeps its ID across P's
     restart. A read under SEC-023 is for its HTTP request on R's connection.
   - **Provider calls after a restart.** Any call to P after the engine's restart runs P again
     first, and that includes I's `run.v1` if the crash came before it. P is run again only
     where its run was asked for before the crash (PRV-026).
   - **P's hand-off.** P carrying I's value to its part on I's node, and keeping it there, count
     as the run's `secrets` (SEC-018 leaves that hand-off to PRV-011).
4. **The ledger.** Every property reads the ledger, never a node's own state. Each entry is
   logged when its event is sent, with the engine's epoch, the number of its restarts so far.
   The list below is a minimum; add what a property needs. It logs:
   - each read, with:
     - the instance and caller it is for, and the ID of the run, connection or HTTP request it
       is for;
     - the Conduit it went through, and whether that Conduit was open when it was dispatched;
     - the invocation it belongs to, its outcome, and who answered it: K or the engine;
   - each `get-secret.v1` sent: by whom, through which Conduit, and whether it was dispatched;
   - each run asked for, each connection made and each HTTP request sent, with its ID and the
     read whose value it used;
   - each place a value went: a run's `secrets`, the variable or argument of the engine's own
     runner, a header on a connection, a record, or anything else;
   - each HTTP request to `h` or `h2`;
   - each provider call to P, and whether P had been run again before it in this epoch;
   - each Conduit closed, P's restart of I's run, `h`'s redirect, and each record that could not
     be kept.
5. **Properties.** Each is named with its promise IDs and checked in every reachable state. Each
   asks only what its promise asks, with its preconditions.
   - **One read path** (SEC-007, SEC-010, SEC-011, SEC-020):
     - every read dispatched goes through the open Conduit of the instance it is read for, and
       belongs to the invocation SEC-011 names, with that invocation's caller;
     - `get-secret.v1` sent by I, or by O as an onward request, is never dispatched, whatever
       Conduit lists it;
     - the engine never serves `get-secret.v1` itself, and never puts a value from its own
       environment in a run's `secrets`.
   - **A failed read stops what it was for** (SEC-012, SEC-016):
     - after a read that ends with a reason, failed or unknown, the run, connection or HTTP
       request with that read's ID is not asked for, made or sent. A later read, for another
       ID, may succeed;
     - a read whose record of dispatch cannot be kept is not dispatched, and is answered
       unavailable;
     - no read is dispatched through a closed Conduit.
   - **Values stay where they were sent** (SEC-015, SEC-017, SEC-018):
     - a value read goes only into the `secrets` of the run it was read for, the variable or
       argument of the engine's own runner for that run, or the header on the connection it was
       read for;
     - no record holds a value read;
     - no value is kept to stand in for a later read: for P's read after the restart, no
       value read before it is used, and no HTTP request sent after a restart carries a value
       read before that restart.
   - **Fixed for the run, read again after a restart** (SEC-014, PRV-026):
     - while I's run lasts, across P's restart and the engine's, nothing more is read for it;
     - after the engine restarts, P is run again only with a value from a read dispatched after
       the restart while P's Conduit was open, and before the provider call it precedes.
   - **A remote header's secret** (SEC-013, SEC-023):
     - each HTTP request carries either the value of R's setup read, held with no restart of the
       engine since, or the value of a read through R's Conduit dispatched after the latest
       restart, while that Conduit was open, and before that request;
     - the value is sent only to `h`, never to `h2`.

   These reachability properties stand beside the harness's own, one for each property's
   precondition:
   - a read that ends unknown, and a record that cannot be kept;
   - `get-secret.v1` sent by I, and by O;
   - P's restart of I's run, and the engine's restart;
   - P run again after the restart, and P not run because its Conduit closed;
   - a re-read for R refused after R's Conduit closed;
   - an HTTP request sent after the restart on a connection made before it, carrying the value
     of a read under SEC-023;
   - `h` answering with a redirect.
6. **Faults.**
   - Loss and reordering on the links between the engine and K, and between the engine and `h`.
   - The engine's crash and recovery, with its state saved.

   Budgets are one each. Restrict each fault with `fault_allowed`, and say why in the README.
7. **Broken variants**, each with the property it is expected to break:
   - `get-secret.v1` dispatched for O's onward request (one read path);
   - a read made through P's Conduit for I's run (the same);
   - P's read after the restart carrying the wrong caller (the same);
   - the engine putting a value from its own environment in a run's `secrets` (the same);
   - the engine answering P's read after the restart itself (the same);
   - a run asked for after its read failed (a failed read stops what it was for);
   - a read dispatched although its record could not be kept (the same);
   - a value kept and used for P's read after the restart (values stay where they were sent);
   - a value written into the record of its read (the same);
   - a read for I's run while it lasts, after P's restart (fixed for the run, read again after a
     restart);
   - P's read after the restart dispatched although P's Conduit is closed (a failed read stops
     what it was for);
   - the header's value kept across the engine's restart and sent without a read (a remote
     header's secret);
   - the redirect followed, and the header sent to `h2` (the same).
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
   which is asserted after the run. If a configuration passes its bound, split it, and say which
   faults each part enables.
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
