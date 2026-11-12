# Work order 081 (S-MC4): one run per instance ID, a replacement's overlap, two readings settled

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` each case you add or change, with the promises it
tests and whether it passes. This order has network access, to fetch crates and for tests that
bind a loopback port; use it for nothing else. Never print a whole large file, fixture or log: use
`head`, `tail`, `grep`, `sed -n` or a count. Run one cargo command at a time. Never run `env`,
`printenv`, `set` or `export -p`, never print an environment variable's value, and never read or
print a credential.

## What and why

Model checking found three defects in how the specification starts, stops and replaces an
instance's run. Work order 108 (MC4) found them, in
[experiment 012](../../experiments/012-model-checking/README.md), and its Record sums them up
(`docs/work-orders/108-mc4-placement-and-lost-machine.md`, lines 267 to 278). A fourth, a
contradiction in the text, was found while writing work order 111 (MC6), whose model has not yet
run: its decision 5 states it. AGENTS.md says where such findings go (lines 73 to 76): "A defect
in the language: fix it in the language, not around it", and a finding of the first build is fixed
"in the specification with a vector (ruling 46, point 4)". This order:

- fixes MC4-009, MC4-001 and order 111's contradiction in `spec/`;
- states in `spec/` the overlap that MC4-002 found, which ruling 42's D11 leaves, and writes no
  fence for it (see "Who decided what");
- adds a vector for a fix where the conformance runner can express it.

It changes no engine code. Where only the model can show a fix, your report says so.

1. **MC4-009: two runs of one instance, with no fault.** See the README, lines 1790 to 1802, and
   the 38-action trace of Correct/Base at lines 2356 to 2397.
   - I's run exits, and the retirement's inspection reports it `stopped`.
   - So the engine asks `run.v1` for a run to serve the retire hook (TPL-018, PRV-007).
   - Before that call arrives, the runtime provider restarts the run by its own policy (PRV-010).
   - The call then starts a second run.

   No promise says what a provider does with `run.v1` for an ID whose run it has just restarted.
2. **MC4-002: a later run beside the run it follows.** See the README, line 1333, and the
   33-action trace at lines 1614 to 1648.
   - During a cut, I is retired, and its hook is unavailable.
   - I is unregistered, and its stop ends unknown (LIF-036). The provider owes the stop (PRV-021).
   - After the heal, I′ is placed and its run started before the provider carries out I's stop.

   Root never declared anything, yet both run. In the model, I′ is a third instance that
   `Script(13)` places and sets up (`src/placement.rs`), not one that `register-replacement.v1`
   makes. MC4 classes the finding as a gap "between the requested aggregate invariant and the
   promises" (line 1333), and asks S3 to "observe the old and replacement run together in
   MC4-002's ordering, or demonstrate and specify the extra fence its implementation relies on"
   (lines 1678 to 1680).
3. **MC4-001: a node's own shutdown when it is declared lost** (README line 1332). PLC-017 gives
   every instance on a declared node the shutdown unknown. Its exception is an instance for which
   no setup was ever dispatched (LIF-025), and a node's setup dispatches nothing (PLC-026). Which
   rule applies to the node itself?
4. **A failed retire hook.** Order 111 states this one in its decision 5, lines 118 to 121.
   `retire-hook.v1` says that a hook that did not succeed "keeps the shutdown pending". OPS-022 and
   LIF-024 make a retirement whose hook failed and whose stop succeeded end unknown, and so its
   shutdown is unknown.
4b. **The Conduits a removal closes** (order 111, MC6-002, confirmed on the correct model in a
   147-action trace). LIF-019 says that no object that refers to an object is retired by it. But when
   an instance is removed, CON-008 closes the Conduits whose consumer it is, and closing a Conduit
   retires it (LIF-017, LIF-035). The model's control that keeps those Conduits keeps LIF-019 and
   breaks CON-008. Its trace and the control are in experiments/012-model-checking/README.md, under
   "MC6 findings, traces and limits", on order 111's branch, which merges with its follow-up 114.

Today's text, on `main` at 355e431:

- `run.v1`'s description already says: "A provider starts no run for an instance's ID while a run
  that it started for that ID may still be running" (`spec/core/operations.yaml` lines 816 and
  817). No promise holds that sentence. PRV-007 (`spec/promises/providers.yaml` lines 197 to 258)
  and PRV-010 (lines 308 to 322), which MC4 checked, do not, and nor does PRV-019 (lines 532 to
  551), which says what a provider does when asked with `run.v1`. Nothing says what the call
  answers when it starts no run. MC4's provider starts a run on every call (README line 1798).
- PRV-010 lets a provider restart a run that "stops without the engine asking for it to stop"
  (line 312). Take a run that exits on its own and is then asked to stop. The stop succeeds, since
  the run is not running (PRV-008), and the provider may still restart the run afterwards. Take
  also a `run.v1` that the provider received and is still carrying out, after the engine has
  stopped waiting for it: a stop that arrives meanwhile succeeds for the same reason, and the run
  then starts. Yet LIF-024's observable says that a retirement that succeeds "leaves no run of the
  instance running" (`spec/conventions/lifecycle.yaml` lines 484 and 485). MC4 assumed that the
  restart window ends once a stop is asked for (README lines 1696 and 1697), and its stop is one
  local action (`fn stop(` in `src/placement.rs`). No promise says either.
- LIF-036 says that the engine "asks for no further stop once the instance is removed, and keeps no
  state of the run" (lines 729 and 730). PRV-021 says that it "keeps no memory of a run once its
  instance is removed" (`providers.yaml` lines 595 and 596). So nothing ties I′ to I. That is the
  owner's choice in ruling 42's D11 (see "Who decided what"), but no text says what it means for
  a later instance, a replacement among them.
- PLC-017's `then` (`spec/promises/placement.yaml` lines 216 to 222) and LIF-025 (lifecycle, lines
  496 to 508) do not say whether a node's setup counts as dispatched. The engine makes that setup
  itself, and it "dispatches nothing and succeeds at once" (PLC-026, lines 391 and 392).
- Compare `retire-hook.v1` (`spec/core/operations.yaml` lines 799 to 801) with OPS-022
  (`spec/conventions/operations.yaml` lines 402 to 428) and LIF-024's `then` (lifecycle, lines 467
  to 474). The core already follows OPS-022: a failed hook and a stop that succeeded settle the
  retirement unknown (`engine/crates/core/src/retirement.rs`, `fn settle(`).

Steps of a case are zero-based in this order, as in a JSON Pointer `/steps/N`.
`tools/vector_delta.py` and the runner's errors print the same step as N + 1.

**Who decided what.** Each fix below is the coordinator's settlement of a defect. It is made under
ruling 46 point 4 and ruling 71, as [IN_FORCE.md](../../IN_FORCE.md) topic 15, "Who decides",
states (lines 303 to 305). The admin logs each one for the owner to strike. Apply each as written,
and report it under `decisions` as coordinator's. You write no record.

MC4-002 is settled here only as far as a coordinator may. Ruling 41 put D11 to the owner
(`docs/proposals/ruling-41/open-decisions.md` lines 250 to 283). One option was "The engine keeps
asking", whose cost it named: "Keeping asking adds engine state that outlives the instance". The
owner chose "one more stop, and the provider's contract" (`docs/proposals/ruling-42/README.md`
lines 47 to 50), which IN_FORCE.md line 156 holds under "42 D11". A fence that kept engine state
about a run once its instance is removed would reverse that choice, so it is the owner's, and the
admin puts it to the owner apart from this order. Step 2 writes no fence of any kind. It states
what D11 leaves, as ruling 42 had D11's limit "stated as a limit" (its log, line 181).

**What stops a fix.** A fix that would change what Conduit is for, or add or remove a feature, is
the owner's to rule on (IN_FORCE.md line 304). If you find that a fix as written here would do so,
do not write it. Leave its text as it was, finish the other fixes, and report under `findings` what
it would add or remove, and why. For this order, each of these counts as such a change:

- a new operation, input, output, key or message (ruling 40);
- an override for one instance;
- a change to who may declare a node lost, or to what a request to unregister asserts (LIF-017);
- engine state about a run that outlives its instance, or a stop that the engine asks for once
  the instance is removed (ruling 42, D11);
- a change to what S1's author writes.

The same line makes one exception: "a small finding that adds a feature, which a coordinator
settles and logs for the owner to strike". Step 1 adds two obligations of a runtime provider.
Should you read them as a feature, the admin settles them under that exception, so they are no
reason to stop. The admin has judged that steps 1 to 4, as written, are none of the changes
listed above.

**Orders in flight.** Line numbers in this order are those of `main` at 355e431. Where a line has
moved in your tree, find it by the words quoted.

- **Order 077** writes ruling 75 A1's fence into OPS-014, into a row for "fenced" in
  `spec/definitions.md`, and into the core. It defines fenced for an attempt. Do not use "fence"
  or "fenced" for a run; where you need the idea, write "known to have stopped". Both orders edit
  these files:
  - `spec/conventions/lifecycle.yaml` (077: LIF-028), `spec/core/operations.yaml` (077:
    `reconcile.v1`) and `spec/definitions.md`;
  - the count of portable vectors in `spec/README.md` line 7, and `conformance/manifest.yaml`,
    if you add a case;
  - the safety catalogue's quotes.

  077's search (its step 4) may also change the observables of PRV-008 and PRV-009. This order
  does not wait for 077. Keep each edit in those files to the lines it needs, so that whichever
  order merges second rebases easily.
- **Order 038b** writes enrollment. It amends PLC-017, PLC-023, PLC-024 and PLC-028, rewrites
  PLC-026, edits PRV-005 and PRV-014, and edits the rows of `spec/definitions.md` on nodes. Its
  ENR-010 fixes a node's labels at admission, so they stop being "last reported", and its PLC-017
  raises ENR-012's question. Change only the sentence of PLC-017 that step 3 needs, and in steps
  2 and 3 name a node's labels as your tree has them. If `docs/work-orders/038b-staged/` is in
  your tree, check at the end that its patches still apply (see "Done when").
- **Order 111** (MC6) changes only `experiments/012-model-checking/`. If the README in your tree
  has its section, cite its finding's ID and trace in step 4. If not, cite order 111's decision 5.
- **Orders 109 (MC5a), 110 (MC5b), 112 (MC7a) and 113 (MC7b)** have merged. Their models read
  promises that this order may change: 109 reads PRV-011, PRV-022 and PRV-028; 110 reads PRV-007,
  PRV-010, PRV-011 and `run.v1`; 112 reads PLC-017; 113 reads PRV-010. They are for step 7's
  report only.
- **The `s3/m2` lane** (orders 079 and 080) builds the shell's calls to a runtime provider, on a
  side branch, and 080 issues a certificate for each run. This order changes no shell code, but
  step 1 changes what such a provider must do.

**Size: S-M.** The specification's text for four findings, and at most a few cases.

Read first:

- AGENTS.md lines 73 to 76;
- in IN_FORCE.md:
  - topic 15's "Who decides" (lines 303 to 308);
  - line 22: "An unknown outcome stays unknown, and nothing runs a background reconciler or wakes
    on its own";
  - topic 8's lines 156 and 157: retirement, shutdown and removal, under rulings that include
    "42 D11", and "Replacement only by an explicit `register-replacement.v1`";
  - line 264: the 217 safety promises of 75 E1;
- D11 in ruling 41's open decisions (lines 250 to 283) and in ruling 42's record (lines 47 to
  50), and that record's log-integration items 1 and 4 (lines 178 and 181);
- in experiment 012's README, its model, and the two orders behind it:
  - MC4's model and pre-registration (lines 1071 to 1277);
  - its findings (lines 1323 to 1346);
  - "What MC4 establishes and what S3 must add" (lines 1652 to 1680);
  - the revision and MC4-009 (lines 1682 to 1817), and MC4-009's trace (lines 2356 to 2397);
  - "What the revision shows" (lines 2399 to 2427);
  - in `src/placement.rs`, `fn stop(` and the stages 12 and 13 of `Local::Script`;
  - order 108's Record, and order 111's decision 5 and its lines 129 and 130;
- the promises:
  - in `spec/promises/providers.yaml`: PRV-005, PRV-007 to PRV-011, PRV-019 to PRV-022, PRV-027
    and PRV-028;
  - in `spec/conventions/lifecycle.yaml`: LIF-009, LIF-017, LIF-024 to LIF-028, LIF-035 and
    LIF-036;
  - in `spec/promises/templates.yaml`: TPL-009 and TPL-018;
  - in `spec/promises/placement.yaml`: PLC-011, PLC-017, PLC-019, PLC-023, PLC-024, PLC-026 and
    PLC-027;
  - OPS-021, OPS-022 and OPS-023 (`spec/conventions/operations.yaml`), ACC-039, IFC-017 and
    IFC-022;
- these contracts in `spec/core/operations.yaml`:
  - `unregister.v1` and `register-replacement.v1`;
  - `retire-hook.v1`;
  - `run.v1`, `stop.v1` and `inspect-run.v1`, their descriptions and their `effect`s;
- in `spec/core/README.md`, the sentences on runtime providers (lines 383 to 427);
- in `spec/README.md`, the limit that begins "A run left on a machine whose node was declared lost"
  (lines 556 to 568);
- in `spec/definitions.md`, these rows:
  - runtime provider (line 308) and run (line 310);
  - shutdown (line 348), remove (line 349), declare lost (line 350) and unregister (line 351);
  - the runtime row of "Words the promises do not use" (line 417);
- [CONTRIBUTING.md](../../CONTRIBUTING.md), "Writing promises";
- in the core, `fn settle(` in `engine/crates/core/src/retirement.rs` and the tests in that file,
  `further_retirement_reuses_unknown_hook_after_successful_stop` among them;
- these cases: `change-retirement` (`conformance/conventions/changes/retirement/`), which uses
  `setup-report` to end a retirement's stop, and `setup-hook-failed` and `setup-hook-unknown`,
  for a hook's fixtures;
- in `conformance/README.md`: "Setup and Conduit lifecycle (I2-07)", "Assertions" and "Reviewing
  vector changes";
- in `engine/CONFORMANCE.md`: the row "DL16: run state and restart", and the paragraph that begins
  "H03 is bounded explicitly".

## What to do

1. **One run per instance ID** (MC4-009; coordinator's). Write both halves as obligations of a
   runtime provider. In both, a run that the provider is still starting under an earlier `run.v1`
   counts as a run that may still be running.
   - **(a) `run.v1` for an ID that may still have a run answers with that run.** A runtime provider
     may be asked with `run.v1` for an instance's ID while a run that it started for that ID may
     still be running. One that it restarted under PRV-010 after it last reported the run
     `stopped` is such a run. The provider then starts no other run.
     - Where that run is running, the call succeeds. Its result gives the `url` at which that
       run's server is reached now.
     - Where the provider cannot tell whether that run is running, it starts none. Write what the
       call then answers, in PRV-005's and PRV-007's terms.
     - The values of `secrets` and `identity` in a call that starts no run reach no run and are
       kept nowhere. Make PRV-011's "It delivers each value" and PRV-022's "It delivers what
       `identity` holds" agree. The run that answers presents the certificate it was given when it
       started, so make PRV-028's "the certificate that `identity` holds in `tls.crt` for that
       run" agree too, and check that IFC-017's check still admits it.

     This is what `systemctl start` does with a unit that is already active.
   - **(b) A stop that succeeded stays true.**
     - A runtime provider reports a stop succeeded only once no run that it started for the ID is
       running. It ends or cancels a run that it is still starting, as `systemctl stop` cancels a
       queued start job.
     - After a stop of an ID's run has succeeded, the provider starts no run for that ID, by a
       restart under PRV-010 or otherwise, until `run.v1` next asks it for one. This is
       `systemctl stop`, which cancels a unit's pending restart, and the deletion of a Pod, after
       which its kubelet restarts nothing.
     - A stop that the provider holds under PRV-021 ends, once carried out, whatever run of the ID
       is running then, one restarted meanwhile included. Do not oblige the provider to carry the
       stop out: PRV-021 leaves that to its contract (ruling 42's D11, and its log-integration
       item 1).
     - Like `run.v1`, a restart starts no run while another that the provider started for the ID
       may still be running.

   Why (b) speaks of a stop that succeeded, and not of a stop that was asked for:
   - A restart policy runs on the run's machine, as Docker's restart policy and systemd's
     `Restart=` do. Order 110 (MC5b) models it so: "P's restart of that run is a local action of
     this node" (its line 58).
   - During a cut, a provider that holds a stop under PRV-021 cannot cancel such a restart until
     it reaches the machine.
   - MC4's model assumed the stricter rule, "until a stop is asked for" (README lines 1696 and
     1697). Report that under `findings`.

   Why these two, and not a rule that the provider restarts nothing once the engine has begun to
   retire the instance:
   - The provider learns of a retirement only from its stop. A retirement asks for the run that
     serves its hook with the same `run.v1` that a setup uses (PRV-007), and telling the two apart
     would need a new input (ruling 40).
   - (a) alone stops MC4-009's trace at its last action.
   - (b) holds what LIF-024's observable claims, that a retirement that succeeds "leaves no run of
     the instance running", for a run that exited before its stop and for a `run.v1` still being
     carried out when the stop came. MC4 cannot show the second, since its stop is one local
     action.

   Write them where a promise's `when` is the provider being asked:
   - (a) in PRV-019's `then`, which says what a provider does when it is asked with `run.v1`;
   - (b) as one new promise in `spec/promises/providers.yaml`, at level `must not`, with the next
     number never used (CONTRIBUTING.md, rule 2).

   If (a) reads badly in PRV-019, put both halves in the new promise and say why. A new safety
   promise changes the catalogue's count of 217, which IN_FORCE.md line 264 cites. Report that for
   the admin, and change no line of IN_FORCE.md.

   Then make these texts agree, each only where (a) or (b) requires:
   - PRV-010's `then` and observable: its permission to restart is narrowed by (b), and it stays
     at level `may`;
   - the observables of PRV-007 and PRV-008: a run that is still being started, and a `url` from
     `run.v1` that may be that of a run already running;
   - PRV-011, PRV-022 and PRV-028, as (a) says;
   - LIF-024's observable, only to cite the promise that now holds "leaves no run of the instance
     running";
   - the descriptions of `run.v1`, `stop.v1` and `inspect-run.v1`, the last at "It reports
     `stopped` only where it knows that no run that it started for the ID is running";
   - the `effect` of `run.v1` ("Starts the run of the item for the instance") and of `stop.v1`
     ("Stops the run, where one is running");
   - the sentences on runtime providers in `spec/core/README.md`, and the rows of
     `spec/definitions.md` for run and for runtime provider.

   Keep `run.v1`'s existing sentence. Check TPL-018's observable ("A hook dispatched again in a
   further retirement may run against a new run") against (a). The hook may now be served by a run
   that the provider restarted, with the values it gave that run (PRV-010). Say whether any text
   must change.

   The engine's own runner is a runtime provider (PRV-003). Report under `findings` whether the
   shell's runner meets (a) and (b); the row "DL16: run state and restart" says "No automatic
   restart". Change no engine code.

   **Vectors: none.** The runner tests the engine, and the engine does nothing new here. It asks
   for a run only where the provider reports it `stopped`. Report under `findings` that only the
   model can show this fix. MC4 must be run again with a provider that follows (a) and (b), and
   S3's runtime provider must follow them.

2. **MC4-002 stated, with no fence** (coordinator's, within ruling 42's D11). Write what D11
   leaves, in the terms of the promises, and nothing more:
   - **After removal.** A request to unregister may remove an instance whose stop did not succeed:
     it ended unknown, failed, or was not dispatched (LIF-036). Its run may then go on until its
     runtime provider carries out the stop, if it ever does (PRV-021). Meanwhile any later
     instance may be set up and run beside it, a replacement under LIF-009 included: nothing ties
     the later run to the earlier one.
   - **Before removal.** The same holds while the earlier instance still exists, retired, with its
     shutdown pending or unknown. A replacement may be registered under LIF-009, and PRV-007 asks
     for its run as for any instance.
   - **What the earlier run can still do.** While its instance exists, every request it makes
     through that instance's Conduits but its retire hook's is refused (ACC-039). Once the instance
     is removed, its credential reaches nothing, since the engine checks on every request that the
     instance a credential names still exists (IFC-022, PRV-022). It keeps the values it was given,
     its secrets among them, and can act outside Conduit with them, as the limit in
     `spec/README.md` already says for a node declared lost.
   - **The precedent.** Unregistering asserts that the shutdown is settled in another way
     (LIF-017), as Kubernetes's force deletion of a StatefulSet Pod does. Kubernetes's
     documentation warns that force deletion can break a StatefulSet's at-most-one semantics.

   Write it in these places:
   - LIF-036's observable, beside "The engine asks for no further stop once the instance is
     removed, and keeps no state of the run" (lines 729 and 730);
   - PRV-021's observable, beside "The engine keeps no memory of a run once its instance is
     removed" (lines 595 and 596);
   - LIF-009's observable, for the replacement of a Pool's entry;
   - the limit in `spec/README.md` (lines 556 to 568). Today it names only a node declared lost or
     a machine reported gone. Extend it to an instance unregistered while its machine was cut off,
     and to a later instance that runs beside it.

   Then check the descriptions of `unregister.v1` and `register-replacement.v1`, and the rows of
   `spec/definitions.md` for unregister and for remove. Change each only where it says otherwise.
   Keep every sentence of LIF-017, LIF-036 and PRV-021 that this step does not name.

   **Vectors: none.** The text states what the engine already does. The admin has put the
   question of a fence to the owner, and a case that pinned today's behavior would flip if the
   owner adds one. Report this.

3. **A lost node's own shutdown** (MC4-001; coordinator's reading). LIF-025's condition is that no
   invocation of `set-up.v1` for the instance was ever dispatched, whatever its parts dispatched.
   Order 111 reads it the same way: "A setup that ended unknown was dispatched" (lines 129 and
   130). The engine sets a node up when it registers it (PLC-026), so LIF-025 does not cover a node
   that was set up.
   - When such a node is declared lost, its shutdown is unknown, as PLC-017 says of every instance
     on it.
   - The node stays, retired, and its status still holds its labels, as your tree has them. A
     request may unregister it, which PLC-011 permits once no instance placed on it exists.
   - A node declared lost before its setup was dispatched is under LIF-025. So is every instance on
     it, since PLC-019 keeps their setups from being dispatched.

   Why: under the other reading, the node's shutdown would be confirmed, and the node removed at
   once (LIF-035), while instances placed on it still exist. That breaks three things:
   - PLC-011's observable;
   - PRV-027's "The instance's node exists while the instance does" (`providers.yaml` line 873);
   - LIF-036's stop, which names the machine from the declared node's status (lifecycle lines 721
     to 723). Without the node, a runtime provider that keeps no state could no longer find a run
     left on a machine that comes back.

   Write the reading in PLC-017's `then`, in one sentence beside the one on LIF-025, and in
   LIF-025's observable if it must change. Check the row for shutdown in `spec/definitions.md`.
   Search `spec/` for "no setup was ever dispatched" and "setup was ever dispatched", and check
   that the reading changes no other case.

   **Vectors: none, unless your tree can express a declaration.** `declare-lost.v1` is still an
   unfinished public path (`engine/CONFORMANCE.md`, the paragraph "H03 is bounded explicitly").
   Neither the core nor the runner can declare a node lost. Report the case for the build order
   that finishes `declare-lost.v1`.

4. **A failed retire hook** (order 111's finding; the coordinator's settlement, for OPS-022 and
   LIF-024). Rewrite two clauses of `retire-hook.v1`'s description (lines 799 to 801):
   - "A hook that did not succeed keeps the shutdown pending". Make it say what TPL-009's
     observable says (templates lines 165 and 166): a hook that did not succeed keeps the shutdown
     from being confirmed. The shutdown then reports unknown or pending: OPS-022 gives the
     retirement's outcome, and LIF-024 gives the shutdown.
   - "and a further retirement dispatches it again". OPS-021 dispatches a hook whose outcome is
     unknown no more until it is reconciled. Make it say, as TPL-009's observable does, that a
     further retirement dispatches it again "as OPS-021 and OPS-028 allow".

   Keep the rest of the description.

   Why:
   - OPS-022 is the rule that counts a retirement's parts, and LIF-024 is the rule that reports a
     shutdown. TPL-009 cites both. A contract's description sums them up and does not override
     them.
   - The stop has an effect, and it is asked for whatever came of the hook. A failed hook beside a
     stop that succeeded leaves an effect, and calling that failed would let OPS-024 treat the
     retirement as having had none.
   - Pending is still right where no part with an effect succeeded, as for a hook served by a
     `remote` item with no run.

   Search `spec/` for any other text that says a failed hook leaves the shutdown pending. Check
   LIF-026's observable among them: "It is the way from a shutdown that is pending, after a
   retirement that failed" (lines 522 and 523). Change only what is wrong. If you find text that
   requires the other reading, stop this step and report it.

   **Vectors.** No portable case exercises a retire hook today. The core's own tests in
   `retirement.rs` do, `further_retirement_reuses_unknown_hook_after_successful_stop` among them,
   and so do the shell's tests. If the runner can express a retire hook with its existing inputs,
   add these cases:
   - one in which the hook fails and the stop succeeds, so the retirement and the shutdown end
     unknown;
   - if the runner can express it, one with no item that is run, so the retirement fails and the
     shutdown is pending.

   The core already settles both this way. If the runner cannot express them, say why. If a case
   that you wrote from the promises fails, report the failure and leave the case out: this order
   changes no engine code.

4b. **The Conduits a removal closes** (MC6-002; the coordinator's settlement, for CON-008 and
   LIF-017). LIF-019's rule is meant for objects that refer to the retired one from outside its
   Conduits, not for the Conduits of which it is an end, which CON-008 closes with its removal. Narrow
   LIF-019 so that it says so, citing CON-008, and check that LIF-017, LIF-035 and the definitions
   of "retire" and "close" agree with the narrowed text. Change nothing in CON-008. If a vector
   already expects LIF-019's literal wording, report it and declare the change.

5. **The vectors.** Write each expectation by hand from the promises, never by copying the
   engine's output, and declare every change.
   - **New cases**: step 4's, under `conformance/conventions/changes/`, built from the fixtures of
     `change-retirement`, with a template that selects a retire hook in the way that
     `setup-hook-failed`'s template selects its setup hook. Give the first the ID
     `change-retirement-hook-failed`, in `retirement-hook-failed/`.
   - **Changed cases.** Change no other case unless the new text requires it, and then only the
     steps it requires. Report each such case, with the promise.
   - **The coverage index.** Add each new case to `conformance/manifest.yaml`, or to its pieces if
     order 071 has landed in your tree, in the form of `change-retirement`'s entry. Add it to the
     `s1_coverage` list of each promise it tests. In `spec/README.md` line 7, set the count of
     portable vectors to the number of entries in the manifest's `cases` in your finished tree.
   - **The declaration.** If you add or change a case, write `docs/work-orders/081-declared.yaml`,
     in the form of `docs/work-orders/041a-declared.yaml`. Give one shape for each intended change,
     none broader than its change:
     - `added`, for each new case;
     - `input`, for a change under a step's `input`, or to a case's `files` or `initial`;
     - `flip`, for a change to a step's `expect/accepted` or `expect/stateUnchanged`, or to
       `expect/reply`'s `reason`, `outcome`, `accepted` or `tool`;
     - `expectation`, for any other change under a step's `expect`;
     - `metadata`, for a change to a case's `promises`, `title` or other top-level field;
     - `fixture`, for a change to a file that a case names. Such a change is reported once for
       every existing case that names the file, so give a new case its own `requests.json`
       rather than adding to a file that another case names.

     Run `python3 tools/vector_delta.py HEAD --declared docs/work-orders/081-declared.yaml`
     until it reports no undeclared change.

6. **The catalogue.** In `docs/implementation/SAFETY_CATALOGUE.md`, a promise you change may no
   longer hold the text that its row quotes. Then change that row's quote and keep its kind. Check
   these rows above all: PRV-007, PRV-008, PRV-010, PRV-011, PRV-019, PRV-021, PRV-022, PRV-028,
   LIF-009, LIF-024, LIF-025, LIF-026, LIF-036, PLC-017, TPL-009 and TPL-018. Add no row for a
   new promise: report it, and leave its classification and the counts to the admin.

7. **Before you finish, check**:
   - every request that is not dispatched names exactly one of refused, invalid, conflict and
     unavailable, in OPS-023's order (CONTRIBUTING.md, "Writing promises", rule 5);
   - MC4-009's trace, walked against the new text. The admin expects it to stop at action 38: the
     delivered `run.v1` starts no second run of I and answers with the restarted run's `url`.
     Action 37, the restart, stays allowed, since no stop has been asked for. Say what you find;
   - MC4-002's trace, walked against the new text. It does not stop, and no text may claim that
     it does: name the sentences that now state it. In the model, I′ is a third instance, so the
     text must cover any later instance, not only a replacement;
   - two more cases. Say where the new text stops each:
     - a run that exits, is asked to stop, and would be restarted after the stop succeeded;
     - a `run.v1` that ends unknown, then a stop that would succeed before the provider has
       started the run, which then starts;
   - no text you wrote uses "fence" or "fenced" for a run, and none calls a declaration of loss a
     fence;
   - nothing wakes on its own; nothing new is an operation, input, output, key or message; and no
     engine state about a run outlives its instance;
   - each promise you rewrote keeps everything that the change does not name, and every ID it
     cites exists;
   - what this order touches outside the scope, for your report only:
     - experiment 012. MC4's model has a provider that starts a run on every call, and a restart
       window that ends once a stop is asked for, stricter than (b). Its P1 counts a third
       instance, I′, which the text now says may run beside I. So P1 must be registered again;
     - the models of orders 109, 110, 112 and 113, where they read a promise you changed;
     - S3's T6 (`experiments/014-s3-two-machines/README.md` line 114);
     - experiment 012's call for S3 to "observe the old and replacement run together in MC4-002's
       ordering" (its README, lines 1678 to 1680);
     - the runtime provider of the `s3/m2` lane.

## Scope

Change:

- in `spec/`:
  - `spec/promises/providers.yaml`: PRV-007, PRV-008, PRV-010, PRV-011, PRV-019, PRV-021, PRV-022
    and PRV-028, each only where step 1 or step 2 requires, and one new promise as step 1 allows;
  - `spec/conventions/lifecycle.yaml`: LIF-009 and LIF-036 for step 2; LIF-024 for step 1, LIF-025
    for step 3 and LIF-026 for step 4, only where those steps require;
  - `spec/promises/templates.yaml`: TPL-018, only where step 1 requires;
  - `spec/promises/placement.yaml`: PLC-017, for step 3;
  - `spec/core/operations.yaml`: the descriptions of `run.v1`, `stop.v1` and `retire-hook.v1`; the
    `effect` of `run.v1` and of `stop.v1`; and the descriptions of `inspect-run.v1`,
    `unregister.v1` and `register-replacement.v1`, only where step 1 or step 2 requires;
  - `spec/core/README.md`: the sentences on runtime providers;
  - `spec/definitions.md`: the rows that steps 1 to 4 name;
  - `spec/README.md`: the limit at lines 556 to 568, and the count of portable vectors at line 7
    if you add a case;
  - elsewhere in `spec/`, only text that a step's search finds wrong, each place listed in your
    report;
- `conformance/conventions/changes/`: step 4's new cases and their fixtures;
- `conformance/manifest.yaml`, or its pieces, for those cases;
- `docs/work-orders/081-declared.yaml`, new, if you add or change a case;
- `docs/implementation/SAFETY_CATALOGUE.md`: the quotes that step 6 names.

Do not change:

- `engine/`: this order changes no engine code, and a case that the core fails is reported;
- `conformance/case.schema.json`: this order adds no runner input;
- `spec/names.yaml`, `tools/` or `packages/`;
- `experiments/`: experiment 012's dated results and S3's pre-registration stand;
- IN_FORCE.md: the admin logs each settlement;
- a record: the ledger, the rulings' records, `docs/AUDIT.md`, `ROADMAP.md` and `docs/proposals/`,
  `docs/proposals/first-build/` among them;
- the files of other work orders, `docs/work-orders/038b-staged/` among them.

Report anything else that needs changing.

## Done when

Every item of steps 1 to 7 is written, or stopped under "What stops a fix", or its absence is
reported with the reason.

From the repository root, these pass:

- `python3 tools/check_docs.py`
- `python3 tools/check_rulings.py`
- `python3 tools/set_names.py --check`
- `python3 tools/check_schemas.py`
- `python3 tools/check_roadmap.py`
- `python3 tools/check_private_paths.py`, if your tree has it (order 068 adds it)
- `python3 -m unittest discover -s tools/tests`, with the interpreter that has PyYAML
- `python3 tools/vector_delta.py HEAD --declared docs/work-orders/081-declared.yaml`, with that
  interpreter, if you added or changed a case, and `python3 tools/vector_delta.py HEAD` if not.
  Its last line reads `Vector delta: N changes; F flips; 0 undeclared; 0 errors.`, with N 0 where
  you changed no case, and it exits 0 where F is 0. A declared flip still exits 1: list each flip
  with its path, its before and after, and the promise that requires it.
- `git diff --check`

Run these with `bash`. Each prints nothing when it passes:

- the first shows that no line you wrote in a promise file has 97 characters or more;
- the second, that every promise ID that your changes to `spec/` cite exists;
- the last two cover the files that are not yet tracked, which `git diff --check` does not see.

The brackets keep this order free of the paths it looks for.

```bash
git diff -U0 -- spec/conventions spec/interface spec/promises | grep -E '^\+[^+]' | awk 'length > 97'
for id in $(git diff -U0 -- spec | grep -E '^\+[^+]' | grep -oE '\b[A-Z]{3,5}-[0-9]{3}\b' | grep -vx 'SHA-256' | sort -u); do grep -rqE "^- id: $id$" spec || echo "$id"; done
grep -rn -E '[[:space:]]+$' conformance/conventions/changes $(ls docs/work-orders/081-declared.yaml 2>/dev/null)
grep -rn -E '/[U]sers/|/[h]ome/|/private/[t]mp/|/tmp/[c]laude|\.codex/' conformance/conventions/changes $(ls docs/work-orders/081-declared.yaml 2>/dev/null)
```

`git status --porcelain` lists only paths in the scope; report any other line, and why.

If `docs/work-orders/038b-staged/` is in your tree, check that its patches still apply:

1. Copy your finished tree, without `engine/target/`, to a directory outside the repository.
2. There, run `git apply --check` for each patch, in each of the four sequences of order 038b's
   step 5.
3. Report each command's last line, then delete the copy. Write its location in no file.

Change no patch. Report any that no longer applies, with the hunk that conflicts.

In `engine/`:

- `cargo fmt --check`
- `cargo clippy --all-targets -- -D warnings`
- `cargo test --release`. The tests read `spec/`, so run it once more after your last change to
  `spec/`.

Report the totals passed, failed and ignored of each full run, and its wall time, as numbers only.
If a shell test fails, rerun it alone. Its timing-sensitive tests can fail under load (finding
4-18): rerun those with `-- --test-threads=2`. Tests that bind loopback ports may fail in your
sandbox: say so, name each, and the admin reruns them.

Report:

- in `decisions`: each promise, contract and definition row you changed, with the finding behind
  it (MC4-009, MC4-002, MC4-001, or order 111's), each marked coordinator's;
- under `own_choices`, every choice you made yourself, including:
  - where you wrote (a) and (b), and the new promise's ID if you added one;
  - what `run.v1` answers where the provider cannot tell;
  - the words of step 2's statement, and where each went;
  - the IDs of the new cases;
- under `findings`:
  - that only the model can show step 1, and what MC4 and S3 then need;
  - that MC4's model assumed a stricter restart window than (b);
  - whether the engine's own runner meets (a) and (b);
  - the declaration's case, for the build order that finishes `declare-lost.v1`;
  - each fix you stopped under "What stops a fix";
  - a new promise's effect on the count of 217;
  - any case that the core failed, and any 038b patch that no longer applies;
  - what step 7 found outside the scope;
  - anything else that needs changing;
- in `vectors`, each case added or changed;
- each check's last line.

Do not stage anything.

## Record

Run on 2026-11-11 and 12. The first worker, on the Windows computer, was stopped after about 20
minutes when the owner paused that computer; its partial work was checkpointed, rebased onto main
and continued on the Mac with the owner's answer on MC4-002 of 2026-11-12 (ruling 42's later
answer): the specification states as a limit that a run whose stop did not succeed may run beside
a later instance, and runtime providers Conduit ships carry out a stop they owe before starting
any run on that machine. The continuing worker finished every step in about 2.9 hours, and a fresh
verifier found no fault. MC4-009 is settled in PRV-019 and the new provider-side promise PRV-030;
MC4-001 and the retire hook's contradiction with OPS-022 and LIF-024 (MC6-001) are settled, and
LIF-019 is narrowed for removal's closing of a consumer's Conduits (MC6-002). Two vectors were
added; the totals are 519 promises (226 language, 245 convention, 48 interface) and 443 vectors.

The worker's findings, kept for later work: the MC4 model's P1 must be registered again, since it
counts the permitted overlap with any later instance, and the models of orders 109, 110, 112 and
113 need review against the changed promises; a declaration case for declare-lost.v1 waits for
the build to finish that path; the shell's own runner implements neither new provider guarantee;
PRV-030 is one more convention-layer safety promise for G5 (219 with order 082's PKG-032); and
PRV-027, LIF-036 and stop.v1 still speak of labels last reported, which 038b already logged.
