# Work order 172b: SO-B, part two: a reconciliation records only, and what a stop must end

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share: list in `vectors` every case you add or change, with the promises it
tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, case, fixture or log:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

This is the second half of SO-B, order 21 of the full review's
[orders](../proposals/full-review/orders.md). [Order 172a](172a-so-b-attempts-and-resends.md) says
what [ruling 78](../proposals/ruling-78/README.md#the-decisions) withdrew from SO-B and why it is
split; this order holds what SO-B confirms as done: a reconciliation of a setup or an open, and a
stop.

1. **A reconciled open or setup that bypasses the checks**
   ([FR-011](../proposals/full-review/findings/L02-lifecycle.md#fr-011) =
   [FR-214](../proposals/full-review/findings/X06-authority-access.md#fr-214)). LIF-028's `then`
   (`spec/conventions/lifecycle.yaml:597-598`): "Recording that a setup or an open occurred makes
   the instance ready or the Conduit open, unless the instance, or the Conduit's consumer, has been
   retired." So a reconciliation that records an unknown `open.v1` as having occurred opens the
   Conduit with none of the checks an open makes. A matching tombstone (LIF-041) is never
   consulted, so an administrator's close of a sibling Conduit is undone without a lift (FR-011),
   and a Conduit that only root may open, under ACC-054 or SEC-008, is opened on another actor's
   word (FR-214). The first build does the same for an open; for a setup it already makes nothing
   ready, since its reconciliation changes an object's state only for `open.v1` and `retire.v1`
   (`engine/crates/core/src/requests.rs:1303-1368`). The review's interim fix F11 held a reconciled
   open to the checks of an open until the owner answered on reconciliation. Ruling 78 point 5
   answered (option A): "A reconciliation records the outcome and what was checked; it stores no
   control's values and makes nothing ready, and the settings stay reported unknown. The way on is
   to send again: the control, ... or a further setup, which leaves finished parts as they are
   (OPS-021)", and "ACC-044's provenance for a reconciled open" goes. So a reconciled setup or open
   records the outcome alone: the open that later opens the Conduit makes every check, and a
   further setup that succeeds makes the instance ready. That is FR-011's own simpler fix, which
   its finding left to the owner, and it supersedes F11's interim. [IN_FORCE.md](../../IN_FORCE.md)
   holds point 5 as spec pending (topic 7, line 158). This order writes it for setups and opens.
   Its half for controls (REV-009, REV-010, REV-013, OPS-031, `reconcile.v1`'s `revision` input and
   the vectors that shrink with them) and the fence's text in LIF-028 and `reconcile.v1` (78 point
   6) are SO-E's (review order 52). What PND-006's setup action does after such a reconciliation
   was put to the owner, and [ruling 80](../proposals/ruling-80/README.md) answers it (point 4):
   the action stays open until a later setup answers it retried or a retirement answers it
   retired; ready is dropped as an answer to it, and its `when` stays true for an outcome a
   reconciliation set. IN_FORCE.md holds that as spec pending too (topic 7, line 159), and this
   order writes it.
2. **A stop that succeeds while the run's processes go on**
   ([FR-219](../proposals/full-review/findings/X07-retries-fence.md#fr-219) =
   [FR-231](../proposals/full-review/findings/X08-one-run.md#fr-231), with
   [FR-332](../proposals/full-review/findings/P06-dogfooding.md#fr-332) and
   [FR-346](../proposals/full-review/findings/P07-engine-build.md#fr-346), their stop half). The
   `run` row of `spec/definitions.md` (line 314) defines a run as "The running of an instance's
   item that is run", and nothing in `spec/` says how far it extends. PRV-030
   (`spec/promises/providers.yaml:992`) forbids a successful stop "while any run that it started
   for that ID is running", which a provider can read as the process it started alone. In the
   dogfood smoke test, Codex's terminal children outlived the stop of the run's process group
   (row 4-23 of [increment 4's log](../proposals/first-build/log-increment-4.md)). Under that
   reading the stop succeeds, LIF-024 confirms the shutdown, the instance is removed, and a child
   that holds the run's values goes on. The coordinator's fix F61
   ([the decisions](../proposals/full-review/decisions.md#providers-runs-and-machines)), as
   FR-231's verifier corrected the definition: a run is every process that its runtime provider, or
   the engine's runner, started for the ID, and every process those started; a stop succeeds only
   once none of the processes held by the provider's unit still runs; the provider's contract and
   DL16 name that unit and state what can escape it. F61's last sentence, whether a run's end
   fences an attempt, fell with the fence (78 point 6), and so did FR-219's other half. The builds
   already stop this way: [order 156](156-runner-stopped-after-end.md) has the engine's runner
   record a stop only once it has seen the run's process group empty, and `engine/CONFORMANCE.md`'s
   DL16 rows name that group and say that a process leaving it is outside the runner's confinement;
   `provider-local-process` stops a process group, which a descendant that calls `setsid` leaves,
   and SSH-R its control group, which `systemd-run --user` units leave.

The reconciliation's open case fails on the first build and is listed in
[conformance/expected-failures.yaml](../../conformance/README.md#expected-failures), as
[rule 8 of "Writing promises"](../../CONTRIBUTING.md#writing-promises) has a case that the build
does not pass yet listed; this order adds no promise ID. Its setup case passes on the first build
and is not listed: a listed case that passes fails the run. The case of the setup action fails on
the first build, which computes no pending action, and is listed too. The stop adds no case:
conformance cases drive the engine's core, which never sees a process, and order 156's tests show
the engine's own runner. The engine follows in review order 22, for the open alone; the setup
action's case passes once the engine computes setup actions (S9-CORE2 in
[experiment 020's plan](../../experiments/020-s9-start-and-repair/plan.md)).

Nothing here touches sign-in: write nothing on a sign-in action, on which keys a package's hooks
carry, or on a connection over which a person completes a sign-in. In PND-003 change only the
setup action's answers.

**Who decided what.** Point 5 is the owner's, and so is ruling 80's point 4. F61 is the
coordinator's, logged for the owner to strike. Where a decision below marks a reading as the
coordinator's, apply it as written and report it under `decisions` as the coordinator's; you write
no record.

**Before you start.** Ruling 80's record, which this order links, is on `main` since #318, with
the lines its commit wrote into `IN_FORCE.md`, and order 172a's text, which this order links, is
in `docs/work-orders/`. `ls docs/proposals/ruling-80/README.md
docs/work-orders/172a-so-b-attempts-and-resends.md` lists both, and this prints 1:
``grep -c 'PND-006 answers it `ready` today; order 172b writes the change' IN_FORCE.md``. If
either fails, stop and report that. Orders 171a and 171b (SO-A, review order 19) and 172a may land
before this one, and none of them changes a promise this order changes. 171a changes other text
of `spec/conventions/lifecycle.yaml`, `spec/conventions/access.yaml`,
`spec/core/operations.yaml` (the descriptions of `open.v1` and `close.v1`),
`spec/implementation-defined.yaml` (DL9), `spec/definitions.md` and `spec/README.md`; 171b other
text of `spec/promises/pending.yaml` (it adds PND-014 after PND-013, above PND-006),
`spec/definitions.md`, `spec/README.md` and `IN_FORCE.md`; 172a other rows of
`spec/definitions.md` and other bullets of topic 7 of `IN_FORCE.md`. Each also changes other
rows of the account, the list in
`conformance/expected-failures.yaml` and `engine/STANDING.md`. Whichever merges later rebases,
keeps both sides' entries in each list in byte order of `id`, and writes `engine/STANDING.md`
again with the tool (decision 13). Where a line has moved, find it by the words quoted.
The row `setup action` of `spec/definitions.md` (line 275) is the line after 171b's row `sign-in`
(line 274): where 171b lands between your start and your rebase, Git reports a conflict there;
keep 171b's row as it merged, and apply decision 5's change to `setup action` alone.

**Size: M.** Text in ten files under `spec/`, three cases and one changed expectation, two
bullets of `IN_FORCE.md`, one row of the account and the generated standing table.

## The decisions it applies

Line numbers are those of `main` at `d4c8cf8b`. Folded `>-` text keeps its four-space indent and
lines under 97 characters; rewrap only as far as a change needs. Everything a decision does not
name stays word for word ("Writing promises", rules 1 and 7).

1. **LIF-028: a reconciled setup or open records only** (78 point 5),
   `spec/conventions/lifecycle.yaml`, lines 590 to 619.
   - `then`: its first sentence (lines 597 and 598) becomes "Recording that a setup or an open
     occurred gives the outcome succeeded and changes nothing else: no instance is made ready and no
     Conduit is opened." The two sentences after it stay, the fenced "did not occur" included,
     which SO-E removes.
   - `observable`: its first two sentences (lines 603 to 607, from "After a reconciliation that
     records a setup" to "until CON-008 closes it.") become: "After a reconciliation that records a
     setup as having occurred, inspecting the instance reports the state it had, as LIF-004 leaves
     it after a setup whose outcome is unknown; an instance that was retired in the meantime stays
     retired, under LIF-006. The way on is a further setup, which leaves what is done already as it
     is under OPS-021: the instance is ready once one succeeds, under LIF-003. After one that
     records an open as having occurred, the Conduit's state does not change: a Conduit that is
     registered is opened only by a later `open.v1`, sent again or as a part of a further setup
     under CON-004, which makes every check that an open makes, LIF-041, ACC-032, ACC-054 and
     SEC-008 among them; one whose consumer was retired in the meantime stays registered, under
     CON-031, until CON-008 closes it." Its last sentence (lines 616 to 618), "Reconciliation of an
     open records its original invocation provenance under ACC-044; confirming a close or
     retirement preserves its tombstone under LIF-040.", becomes "A reconciliation of an open
     opens nothing, so it gives no provenance under ACC-044; confirming a close or retirement
     preserves its tombstone under LIF-040."
   - *Coordinator's reading.* CON-004 makes the opening of a declared Conduit a part of its
     consumer's setup, a request for `open.v1` dispatched by the Conduit's state, so a further
     setup also opens a Conduit that a reconciliation left registered (decision 6); the
     observable names both ways, each an open that makes its checks.
2. **LIF-003**, the same file, lines 45 to 57.
   - Its `when` (lines 49 and 50), "An instance's setup succeeds: the invocation of `set-up.v1`
     that names it or, for a Conduit, of `open.v1`.", becomes "An instance's setup succeeds: the
     invocation of `set-up.v1` that names it or, for a Conduit, of `open.v1`, other than one whose
     outcome a reconciliation set under LIF-028."
   - Its `observable` (lines 55 and 56): "A setup whose outcome was unknown, and that is reconciled
     as having occurred, makes the instance ready in the same way, under LIF-028." becomes "A setup
     whose outcome was unknown and that is reconciled as having occurred leaves the state as it was,
     under LIF-028: the instance is ready once a further setup succeeds."
   - *Coordinator's reading.* A reconciled setup's outcome becomes succeeded, so without the
     `when`'s exception LIF-003's `then` would still make the instance ready. The exception is
     worded as DL10 words the same case ("other than one whose outcome a reconciliation set").
3. **ACC-044**, `spec/conventions/access.yaml`, its `observable` (lines 816 and 817): "Reconciling
   an open as having occurred gives that invocation's provenance." becomes "A reconciliation that
   records an open as having occurred opens nothing, under LIF-028, so it gives no provenance: the
   request that later opens the Conduit gives its own."
4. **`reconcile.v1` and the core's summary.**
   - In `spec/core/operations.yaml`, the description of `reconcile.v1`: "Recording that a setup or
     an open occurred gives it the outcome succeeded and makes the instance ready or the Conduit
     open, unless the instance or the Conduit's consumer has been retired." (lines 507 to 510)
     becomes "Recording that a setup or an open occurred gives it the outcome succeeded and changes
     nothing else: a further setup makes the instance ready, and `open.v1` sent again opens the
     Conduit." "A reconciled opening keeps that invocation's provenance (ACC-044);" (lines 523 and
     524) becomes "A reconciled open gives no provenance, since it opens nothing (ACC-044);". In
     its `effect` (lines 544 to 546), the sentence "Where a setup or an open is recorded as having
     occurred, sets the state of the instance or the Conduit to ready, unless the instance or the
     Conduit's consumer is retired." goes, and "Where a retirement or a close is, confirms the
     shutdown" becomes "Where a retirement or a close is recorded as having occurred, confirms the
     shutdown". The contract's schema, its `revision` input and its fence text do not change. This
     file's descriptions hold lines longer than 100 characters; leave every line you do not change
     as it is.
   - In `spec/core/README.md`, in the item that begins "**Reconciling.**" (line 190), the sentence
     at lines 201 to 203: "Recording that an unknown setup or open occurred makes the instance
     ready or the Conduit open, unless the instance or the Conduit's consumer has been retired."
     becomes "Recording that an unknown setup or open occurred records the outcome alone: a further
     setup makes the instance ready, and `open.v1` sent again opens the Conduit."
   - In `spec/core/results.yaml`, the description of `provenance.v1` (lines 216 and 217):
     "Repeats and reconciliation preserve the original provenance; a purge never removes it."
     becomes "Repeats preserve the original provenance, and a reconciliation of an open gives none,
     since it opens nothing (ACC-044); a purge never removes it." Rewrap only those two lines.
     (Added by the coordinator on the Opus reader's finding.)
   - No case asserts the core's descriptions; if a case or a fixture changes because of this
     decision or of decision 7's change to `stop.v1`, stop and report.
5. **PND-006: a reconciled setup's action stays open** (ruling 80, point 4),
   `spec/promises/pending.yaml`, lines 296 to 311.
   - Its `when` (line 300), "An instance is not retired and its latest set-up.v1 invocation ended
     failed or unknown.", becomes "An instance is not retired and its latest set-up.v1 invocation
     ended failed or unknown, including one whose outcome a reconciliation has since set under
     LIF-028."
   - Its `observable` (lines 306 and 307): "Dispatch of a further set-up.v1 answers retried;
     reconciliation as having occurred answers ready; retirement answers retired; answer.v1 may
     answer dismissed." becomes "Dispatch of a further set-up.v1 answers retried; retirement
     answers retired; answer.v1 may answer dismissed. A reconciliation that records the setup as
     having occurred answers nothing and leaves the action open, since it makes nothing ready
     under LIF-028." The rest of the `observable`, and the `then`, stay.
   - PND-003's `then` (lines 53 and 54): "setup is answered retried, ready, retired or dismissed
     (PND-006)" becomes "setup is answered retried, retired or dismissed (PND-006)". Nothing else
     in PND-003 changes.
   - `spec/definitions.md`, the row `setup action` (line 275): "The computed pending kind for the
     latest failed or unknown setup of a live instance," becomes "The computed pending kind for
     the latest failed or unknown setup of a live instance, including one whose outcome a
     reconciliation has since set,". The rest of the row stays.
   - *Coordinator's reading.* That the `when` stays true for an outcome a reconciliation set is
     ruling 80's own: a reconciliation sets the outcome succeeded, so without the addition the
     action would no longer be computed and would vanish unanswered. The wording, and the changes
     to PND-003 and to the row, are the coordinator's: PND-003 lists the answers that PND-006
     gives, and the row repeats PND-006's `when`.
6. **What does not change here, and why.** ACC-055, which says who may record, and LIF-041, which
   the later open now meets. OPS-022: CON-004 and POOL-013 dispatch an open part or an entry's
   setup by the Conduit's or the Resource's state, so a further setup opens a Conduit that a
   reconciliation left registered; OPS-022's observable sentence on a part "reconciled as having
   occurred" stays for SO-E, which rewrites OPS-022 under 78 point 8. REV-014, under which a
   reconciliation of a setup counts as a setup, so that it is not dispatched while another setup
   of the instance is, stays. And the control half of point 5 and the fence's text, which are
   SO-E's.
7. **The run's extent and the stop** (F61).
   - `spec/definitions.md`, the row `run` (line 314): after its first sentence, which ends "no run
     is asked for (PRV-007, TPL-018).", add: "Its processes are every process that its runtime
     provider, or the engine's own runner, started for the instance's ID, and every process those
     started; a stop succeeds only once none of them still runs in the unit by which the provider
     stops the run, which the provider's contract, or DL16 for the engine's own runner, names with
     what can leave it (PRV-030)."
   - `spec/definitions.md`, the row `runtime provider` (line 312): "a stop succeeds only once no
     run of the ID is running or being started" becomes "a stop succeeds only once no process of a
     run of the ID still runs in the unit by which it stops that run and no run is being started".
   - PRV-030, `spec/promises/providers.yaml:986-1005`. Its `then` (lines 992 to 994) becomes: "It
     reports succeeded while any process of a run that it started for that ID still runs in the
     unit by which it stops that run, or while a run is still being started under an earlier
     `run.v1`; or it starts any run for that ID after the stop succeeded, by a restart under
     PRV-010 or otherwise, until `run.v1` next asks for one." Its `observable` gains, before its
     first sentence: "A run's processes are every process that the provider started for the ID and
     every process that those started. The provider stops a run by a unit that the operating
     system ends whole, such as a process group, a control group, as systemd stops a unit under
     `KillMode=control-group`, or a job object. Its contract names that unit and which processes of
     a run can leave it, which a stop that succeeded may leave running. Where the provider cannot
     tell whether its unit still holds a process, the stop ends unknown, under PRV-008."
   - PRV-008's `observable` (lines 294 and 295): "A stop succeeds only once no run of the ID is
     running, including one still being started:" becomes "A stop succeeds only once no process of
     a run of the ID still runs in the unit by which the provider stops it, and no run is still
     being started:". The rest of the sentence stays.
   - `stop.v1`'s description in `spec/core/operations.yaml` (lines 997 to 999): "A stop succeeds
     only once no run of the ID is running, including a run still being started under an earlier
     `run.v1`, which the provider ends or cancels." becomes "A stop succeeds only once no process of
     a run of the ID still runs in the unit by which the provider stops it, under PRV-030, and no
     run is still being started under an earlier `run.v1`, which the provider ends or cancels."
   - `spec/core/README.md`, lines 416 and 417: "A stop succeeds only once no run of the ID remains
     running or being started, and prevents any start or restart until `run.v1` next asks
     (PRV-030)." becomes "A stop succeeds only once no process of a run of the ID still runs in
     the unit by which the provider stops it and no run is being started, and prevents any start
     or restart until `run.v1` next asks (PRV-030)."
   - LIF-024's `observable` (`spec/conventions/lifecycle.yaml:515-516`): "so one that succeeds
     leaves no run of the instance running, under PRV-030." becomes "so one that succeeds leaves no
     process of the instance's run running in the unit by which its runtime provider stops the run,
     under PRV-030; a process that left that unit, as the provider's contract states, may still
     run."
   - DL16, `spec/implementation-defined.yaml`, line 182: after "the runner's restart policy, under
     PRV-010;" insert " the unit by which it stops a run, and which processes of a run can leave
     that unit, under PRV-030;".
   - `spec/README.md`, section "What the guarantee for secrets covers": after the bullet that begins
     "A run left on a machine whose node was declared lost" (lines 584 to 607), add the bullet: "A
     process that a run started and that left the unit by which its provider stops the run, as a
     process that starts a session of its own leaves a process group, goes on after a stop that
     succeeded, with the values the run was given, secrets included, and can act outside Conduit
     with them. The provider's contract, and DL16 for the engine's own runner, say what can leave
     that unit (PRV-030, LIF-024). Once the instance is removed, its credential reaches nothing
     (IFC-022, PRV-022)." Wrap it at the list's width.
   - *Coordinator's reading.* The definition is the one FR-231's verifier corrected, not the
     circular one the reader drafted ("that the provider's stop reaches"). The PRV-030 observable's
     examples of a unit, the core summary's sentence and the bullet of `spec/README.md` are the
     coordinator's, carrying F61 to every place that states when a stop succeeds. PRV-003 already
     binds the engine's own runner to PRV-030 and does not change; neither do
     `engine/CONFORMANCE.md` and the providers' READMEs, which already name their units and what
     leaves them.
8. **IN_FORCE.md**, topic 7, two bullets under "*Spec pending, rulings 78 and 80*". Find each by
   the words quoted; the bullet that heads them stays spec pending, as its other points are not
   written yet. Put no date in either line; `check_rulings.py` refuses one.
   - In the bullet that begins "A reconciliation records the outcome and what was checked" (line
     158), "REV-009, REV-010, REV-013, OPS-031, LIF-028, ACC-044 and `reconcile.v1`'s `revision`
     input say otherwise today (78 point 5, amending 32 Q4, 33 F11 and 41 item 15)." becomes "For a
     setup or an open this is in spec/: LIF-003, LIF-028, ACC-044 and `reconcile.v1`. For a
     control, REV-009, REV-010, REV-013, OPS-031 and `reconcile.v1`'s `revision` input say
     otherwise today (78 point 5, amending 32 Q4, 33 F11 and 41 item 15)." The rest of it stays.
   - The bullet that begins "Where a reconciliation records a setup as having occurred, the setup's
     pending action stays open" (line 159) becomes, whole:
     ``  - **In spec/**: where a reconciliation records a setup as having occurred, the setup's pending action stays open, and `ready` is not one of its answers: PND-006, PND-003 and spec/definitions.md (80 point 4, on 78 point 5).``
9. **Three cases and one changed expectation**, under `conformance/conventions/inventory/e4/`.
   Steps are zero-based here, as JSON Pointers; the runner and `tools/vector_delta.py` print step
   N + 1. Make copies with `cp` and edits with a script you run and do not commit. Each new step,
   and the new `call` of a fixture entry, carries evidence in the form the copied case's steps
   carry it (a `time`, 96 `ids`, 16 `revisions` and 16 five-character `suffixes`), with fresh IDs
   that the case uses nowhere else; a step copied unchanged keeps its evidence. A step's capture
   binds its symbol only after that step's expectations are checked, so a step cannot expect a
   value it captures; a later step can. Where a step's behavior does not change, the first build's
   output may settle what `spec/` leaves open.
   - **`e4-reconcile-open-records-only`**, in `reconcile-open-records-only/`, built from
     `reconcile-open-retired-consumer/` (its `case.yaml` and `requests.json`), with the title "A
     reconciled open records the outcome and opens nothing; open.v1 opens the Conduit" and the
     promises `LIF-028` and `ACC-044`.
     - Steps 0 to 2 as there: the project is registered, Agent `a` is set up and its run reported
       running, so the Conduit's consumer is ready and is never retired in this case.
     - `/steps/3`: the `lifecycle-fixture` of entry 0 of `requests.json`, as there, with no
       `capture`. In that entry the `invocations` stay as they are, the unknown `open.v1`
       invocation of `subject` among them, and the `call` becomes root's `access.reconcile.v1`
       with that invocation's ID, `"occurred": true` and `"checked": "opening externally
       confirmed"`, in place of the retirement. Expect `reply` `{"isError": false}`;
       `invocation_facts` with that invocation `succeeded`; and `object_facts` with `$subject`
       `"open": false` and `"state": "registered"`.
     - `/steps/4`, new: root's `access.open.v1` with `{"conduit": "subject"}`, as an `invoke`.
       Expect `reply` `{"isError": false}` and `object_facts` with `$subject` `"open": true`, and
       capture `reopening` from `/reply/_meta/io.github.shrav-k.conduit~1invocation`, as step 2
       of `e4-provenance-open` captures `opening`.
     - `/steps/5`, new: root's `management.inspect.v1` with
       `{"object": {"kind": "Conduit", "name": "subject"}}`, as an `invoke`, as step 3 of
       `e4-provenance-open` inspects. Expect `reply.isError` false and
       `reply.structuredContent.provenance.invocation` `"$reopening"`: the open of step 4 gave the
       provenance, and the reconciliation gave none.
   - **`e4-reconcile-setup-records-only`**, in `reconcile-setup-records-only/`, built from
     `reconcile-open-retired-consumer/`'s `case.yaml` alone, with the title "A reconciled setup
     records the outcome and makes nothing ready" and the promises `LIF-028` and `LIF-003`. It uses
     no fixture: drop `requests` from its `files`, and copy no `requests.json`. Its setup's outcome
     becomes unknown through the real lifecycle, as `setup-run-unknown-run`
     (`conformance/conventions/setup/run-unknown-run/`) makes one unknown at its steps 3 and 4.
     - Steps 0 and 1 as there: the project is registered, and root's `group.set-up.v1` of Agent
       `a` is pending, its invocation captured as `setup`.
     - `/steps/2`, new: a `setup-report` for `$setup` with
       `{"kind": "inspect-run", "state": "stopped"}`. Expect `state` `{"started": true}`, as step 3
       of `setup-run-unknown-run` does.
     - `/steps/3`, new: a `setup-report` for `$setup` with
       `{"kind": "run", "outcome": "unknown"}`. Expect `reply._meta` with
       `io.github.shrav-k.conduit/outcome` `unknown`, and `object_facts` with `$a`
       `"state": "registered"`.
     - `/steps/4`, new: root's `group.reconcile.v1` with `{"invocation": "$setup", "occurred":
       true, "checked": "setup externally confirmed"}`, as an `invoke`: an Agent's setup is
       reconciled through its owning Fleet, under ACC-033. Expect `reply` `{"isError": false}`,
       `invocation_facts` with `$setup` `succeeded`, and `object_facts` with `$a`
       `"state": "registered"`.
     - The case passes on the first build, whose reconciliation changes an object's state only
       for `open.v1` and `retire.v1`, and fails on an engine that makes the instance ready as
       LIF-028 said before this order. Do not list it; if it fails, report its error's first line
       and stop.
   - **`e4-reconcile-setup-action-open`**, in `reconcile-setup-action-open/`, built from
     `reconcile-setup-records-only/` (its `case.yaml`), with the title "A reconciled setup leaves
     its setup action open" and the promises `PND-006` and `PND-003`. Steps 0 to 4 as there.
     `/steps/5`, new: root's `group.inspect.v1` with `{"object": {"kind": "Agent", "name": "a"}}`,
     as an `invoke`, as `e4-retirement-lift-reconcile-cause` inspects Agent `a` at its step 7:
     `inspect.v1` inspects an Agent through its owning Fleet. Its evidence's time is that of step
     4's, so no expiry under DL19 can have passed. Expect `reply.isError` false and
     `reply.structuredContent.status.pending` a list of one item, with `"kind": "setup"` and
     `"id": "$setup"`: a setup action takes its invocation's ID (PND-001), and PND-002 raises an
     action, which lists it there for root, while it is unanswered and has not expired. So the
     reconciliation neither answered the action (PND-003) nor left it uncomputed (PND-006). The
     case fails on the first build, which computes no pending action; decision 10 lists it.
   - **`e4-reconcile-open-retired-consumer`**, changed: in `/steps/5/expect/object_facts/0`, the
     member `provenance` goes, since a reconciled open gives none under the new ACC-044. Nothing
     else in the case, its fixture or its coverage-index entry changes. It still passes on the
     first build; the new open case is the one that tests the change.
   - Add the three new entries to `conformance/conventions/inventory/e4/index.yaml`, in byte order
     of `id`, with their `case` paths written as the piece's other entries write theirs, the same
     `promises` as their cases, and `"safety_rows": []`. The manifest and the case schema do not
     change. Only the setup action's case tags promises that none tagged before, PND-006 and
     PND-003; `spec/README.md`'s figures of coverage by vectors are those recorded on 2026-11-10,
     as its Contents says, and do not change.
10. **The expected failures.** Add to `conformance/expected-failures.yaml`, in byte order of `id`
    and in the form "Expected failures" in `conformance/README.md` gives, two entries:
    `e4-reconcile-open-records-only`, with the reason "LIF-028 under ruling 78 point 5: the first
    build opens the Conduit when a reconciliation records an open as having occurred; review order
    22, the engine following SO-B, makes it pass and removes this entry"; and
    `e4-reconcile-setup-action-open`, with the reason "PND-006 under ruling 80: the first build
    computes no pending action, so inspecting the instance gives no status.pending; S9-CORE2 of
    experiment 020's plan, the engine's setup action, makes it pass and removes this entry".
11. **The account**, `experiments/012-model-checking/account.yaml`, by the rule that
    [order 153](153-model-account-checked.md) (decision 7) states: LIF-003, from
    `holds, not graded`, becomes `text changed, not rerun`, with `was` its old standing, `model`,
    `read` and `digest` unchanged, and this sentence appended to its `note`: "Work order 172b
    changed its when and observable, so that a reconciled setup leaves the state as it was (ruling
    78 point 5)." The note stays a plain scalar, as it is: the sentence holds no ": ", which would
    end the file's YAML there. Continue a long note on lines indented four spaces, as the file's
    other long notes are. LIF-024, LIF-028 and PRV-008 are `text changed, not rerun` already,
    ACC-044 and PRV-030 are `not modelled`, and PND-003 and PND-006 are `left to MC9`, which no
    model run gave: none of their rows changes. Keep every other byte of the file.
12. **The declaration**, `docs/work-orders/172b-declared.yaml`, in the form of
    `docs/work-orders/081-declared.yaml`: one `added` declaration for each new case, and one of
    kind `expectation`, with `cases: [e4-reconcile-open-retired-consumer]`,
    `paths: ['/steps/5/expect/object_facts/0/provenance/*']` and `operation: remove`. Nothing else.
13. **The standing table**, `engine/STANDING.md`, which `tools/promise_standing.py` generates
    ([order 185](185-promise-standing-generated.md)) from the coverage index and the expected
    failures among its inputs, so decisions 9 and 10 change it. Once they are done, run
    `python3 tools/promise_standing.py --write`; never edit the file by hand. From its state on
    your starting tree, these rows change: ACC-044 (Cases and Expected failures each up 1),
    LIF-003 (Cases up 1), LIF-028 (Cases up 2, Expected failures up 1), PND-003 and PND-006 (each
    Cases and Expected failures 0 to 1, Standing `no case, no reason` to `expected failure`, where
    no order that landed before has given either a case), and, in Totals, `expected failure` up 2
    and `no case, no reason` down 2, each in the columns convention and all. On `main` at
    `d4c8cf8b` that is ACC-044 17 to 18, LIF-003 3 to 4, LIF-028 2 to 4, `expected failure` 0 to 2
    and `no case, no reason` 89 to 87 and 153 to 151. Report any other change with its rows.

## Scope

Change only `spec/conventions/lifecycle.yaml` (LIF-003, LIF-024, LIF-028),
`spec/conventions/access.yaml` (ACC-044), `spec/core/operations.yaml` (`reconcile.v1`'s description
and effect, `stop.v1`'s description), `spec/core/results.yaml` (`provenance.v1`'s description),
`spec/core/README.md` (two sentences), `spec/promises/providers.yaml` (PRV-008, PRV-030),
`spec/promises/pending.yaml` (PND-003, PND-006), `spec/implementation-defined.yaml` (DL16),
`spec/definitions.md` (the rows `run`, `runtime provider` and `setup action`), `spec/README.md`
(one bullet), `IN_FORCE.md` (two bullets of topic 7),
`conformance/conventions/inventory/e4/reconcile-open-retired-consumer/case.yaml`,
`conformance/conventions/inventory/e4/index.yaml`, `conformance/expected-failures.yaml` and
`experiments/012-model-checking/account.yaml`, and create
`conformance/conventions/inventory/e4/reconcile-open-records-only/` (`case.yaml` and
`requests.json`), `conformance/conventions/inventory/e4/reconcile-setup-records-only/` and
`conformance/conventions/inventory/e4/reconcile-setup-action-open/` (each a `case.yaml` alone) and
`docs/work-orders/172b-declared.yaml`, and regenerate `engine/STANDING.md` with
`tools/promise_standing.py --write` alone. Nothing else under `engine/`, and not `packages/`,
`tools/`, the experiment's code or README, `conformance/manifest.yaml` or the case schema. Leave
`docs/implementation/SAFETY_CATALOGUE.md` as it is: it quotes an earlier commit, as its header
says, so its rows of LIF-028 and PRV-030 keep their old quotes, as order 171a leaves the rows of
the promises it changes.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 tools/check_private_paths.py
python3 tools/check_model_account.py
/usr/bin/python3 tools/check_model_account.py --history
python3 tools/promise_standing.py --check
python3 tools/vector_delta.py HEAD --declared docs/work-orders/172b-declared.yaml
git diff --check
```

`promise_standing.py --check` ends `Errors: 0`; with decisions 9 and 10 done and before
`--write`, it fails, naming `engine/STANDING.md`.

`vector_delta.py` reads the declaration with PyYAML, so where `python3` lacks it, run it with
`/usr/bin/python3`; it must end `Vector delta: 7 changes; 0 flips; 0 undeclared; 0 errors.` (three
added cases and the four members of the provenance removed). `check_model_account.py` ends
`Errors: 0`, and from the counts it printed on your starting tree `holds, not graded` falls by 1
and `text changed, not rerun` rises by 1, as does `holds, not graded` under "Text changed, not
rerun by was". Report any other count with its rows.

From `engine/`, one at a time:

```bash
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. e4-reconcile-open-
cargo run --release -p conduit-conformance -- .. e4-reconcile-setup-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The pieces tool prints `Nothing to move.` The first runner command prints `PASS` for
`e4-reconcile-open-replaced-subject` and `e4-reconcile-open-retired-consumer`, and
`EXPECTED FAILURE e4-reconcile-open-records-only (LIF-028, ACC-044): ` with its reason, and ends
`2 cases passed; 1 expected failures`. The expected failure's error must be
`object_facts/open: expected false, found true`. The runner names no step for an error in
`object_facts`; step 4 as it counts them, the reconciliation, is the only step of the case that
expects `"open": false`, so the error is there, on the Conduit the first build opened. Any other
error means the case is wrong: fix the case, never the list. The second command prints
`PASS e4-reconcile-setup-records-only (5 steps; LIF-028, LIF-003)`, and
`EXPECTED FAILURE e4-reconcile-setup-action-open (PND-006, PND-003): ` with its reason, and ends
`1 cases passed; 1 expected failures`. That expected failure's error must begin
`step 6: /reply/structuredContent/status/pending: missing required field`: the inspection, and
the `status.pending` that the first build does not give. An error at another step means the case
is wrong, as above. Report each error's first line, cut at 200 characters. The threads are 4
because at 24 the runner ran out of memory (order 071's Record).

The first build never reaches steps 4 and 5 of `e4-reconcile-open-records-only`, so check them
once by hand, between the runner commands and the tests. Keep copies of that case's `case.yaml`
and of `conformance/expected-failures.yaml` outside the repository; with a script, delete
`/steps/3` from the case and the case's entry from the list; from `engine/`, run
`cargo run --release -p conduit-conformance -- .. e4-reconcile-open-records-only`, which must
print `PASS e4-reconcile-open-records-only (5 steps; LIF-028, ACC-044)` and end `1 cases passed`;
then put both files back from the copies, check with `cmp` that each equals its copy, and run the
first runner command again, which prints what it printed before. Report the PASS line.

Each of these prints 0. They read the file with its line breaks as spaces, so a rewrap hides
nothing:

```bash
tr -s ' \n' '  ' < spec/conventions/lifecycle.yaml | grep -c 'makes the instance ready or the Conduit open'
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c 'makes the instance ready or the Conduit open'
tr -s ' \n' '  ' < spec/core/README.md | grep -c 'makes the instance ready or the Conduit open'
tr -s ' \n' '  ' < spec/conventions/access.yaml | grep -c "gives that invocation's provenance"
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c "keeps that invocation's provenance"
tr -s ' \n' '  ' < spec/core/results.yaml | grep -c 'Repeats and reconciliation preserve'
tr -s ' \n' '  ' < spec/conventions/lifecycle.yaml | grep -c 'makes the instance ready in the same way'
tr -s ' \n' '  ' < spec/conventions/lifecycle.yaml | grep -c 'leaves no run of the instance running'
tr -s ' \n' '  ' < spec/definitions.md | grep -c 'once no run of the ID is running or being started'
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c 'once no run of the ID is running, including'
tr -s ' \n' '  ' < spec/core/README.md | grep -c 'once no run of the ID remains running'
tr -s ' \n' '  ' < spec/promises/providers.yaml | grep -c 'once no run of the ID is running'
tr -s ' \n' '  ' < spec/promises/providers.yaml | grep -c 'while any run that it started for that ID is running'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'answers ready'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'retried, ready, retired'
grep -c 'PND-006 answers it `ready` today' IN_FORCE.md
```

Each of these prints 1:

```bash
tr -s ' \n' '  ' < spec/promises/providers.yaml | grep -c 'still runs in the unit by which it stops that run'
tr -s ' \n' '  ' < spec/implementation-defined.yaml | grep -c 'the unit by which it stops a run, and which processes of a run can leave that unit'
tr -s ' \n' '  ' < spec/conventions/lifecycle.yaml | grep -c 'occurred gives the outcome succeeded and changes nothing else: no instance is made ready and no Conduit is opened\.'
tr -s ' \n' '  ' < spec/conventions/lifecycle.yaml | grep -c 'other than one whose outcome a reconciliation set under LIF-028'
tr -s ' \n' '  ' < spec/conventions/lifecycle.yaml | grep -c 'sent again or as a part of a further setup under CON-004'
tr -s ' \n' '  ' < spec/README.md | grep -c 'goes on after a stop that succeeded'
grep -c 'For a setup or an open this is in spec/: LIF-003, LIF-028, ACC-044' IN_FORCE.md
grep -c '^  - \*\*In spec/\*\*: where a reconciliation records a setup as having occurred' IN_FORCE.md
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'ended failed or unknown, including one whose outcome a reconciliation has since set under LIF-028\.'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'answers nothing and leaves the action open'
grep -c 'latest failed or unknown setup of a live instance, including one whose outcome a reconciliation has since set' spec/definitions.md
```

`git status --short` lists only the files in scope. Do not stage anything. Report each choice you
made yourself and every check's last line. `git diff --check` and `check_private_paths.py` read
tracked files only, so the admin then stages the new files and runs
`python3 tools/check_private_paths.py` and `git diff --cached --check`.

Before it merges, a fresh Codex verifier and a fresh read-only Opus reader read the result
([ruling 79](../proposals/ruling-79/README.md), point 2). It merges on a pass of the local gate,
the coordinator's interim gate or `tools/gate.py` once order 193b has merged. CI's engine build,
its four engine shards and the doctests run on the pull request, on Linux, as a second opinion
that the merge does not wait for; a run that fails after the merge is fixed in a follow-up
([ruling 80](../proposals/ruling-80/README.md), point 10).

## Record

Run on 2026-11-16 on the Windows computer from `main` `a771b88b`: the worker (1,324 s) and a fresh
Codex verifier (168 s), which found no must or should in one round and did not rerun the cargo
tests. The worker changed the 20 files that Scope then named. LIF-028 now says that recording a
setup or an open as having occurred gives the outcome succeeded and changes nothing else, and its
observable names the two ways on: a further setup, under OPS-021 and LIF-003, and a later
`open.v1`, sent again or as a part of a further setup under CON-004, which makes every check that
an open makes. LIF-003's `when` excepts an outcome that a reconciliation set; ACC-044 and the
descriptions of `reconcile.v1` say that a reconciled open gives no provenance, since it opens
nothing; and `reconcile.v1`'s effect no longer makes anything ready. Under ruling 80, point 4,
PND-006's setup action stays open after a reconciliation of the setup, PND-003 drops `ready` from
its answers, and the row `setup action` of `spec/definitions.md` follows. Under F61, the rows `run`
and `runtime provider`, PRV-030, PRV-008, LIF-024, `stop.v1`, the core's summary and DL16 say that
a stop succeeds only once no process of the run still runs in the unit by which the provider stops
it, a unit that the provider's contract, or DL16 for the engine's own runner, names with what can
leave it; `spec/README.md` gains a bullet on a process that left that unit. The two bullets of
topic 7 of `IN_FORCE.md` say what is now in spec/. Three cases are new under
`conformance/conventions/inventory/e4/`: `e4-reconcile-open-records-only` and
`e4-reconcile-setup-action-open`, both listed as expected failures, and
`e4-reconcile-setup-records-only`, which passes; `e4-reconcile-open-retired-consumer` no longer
expects a provenance. The declaration gives 7 changes. The account moves LIF-003 from
`holds, not graded` to `text changed, not rerun`: the first count falls from 66 to 65, and the
second rises from 28 to 29. `engine/STANDING.md`, regenerated, changes the rows of ACC-044,
LIF-003, LIF-028, PND-003 and PND-006, and in Totals `expected failure` 0 to 2 and
`no case, no reason` 89 to 87 and 153 to 151. The worker's own choice was its evidence: UUIDv5 IDs
with distinct step tags, at 2026-11-12T12:00:00Z, the inspection at the reconciliation's time. It
reported every check of Done when with its stated value, and 241 tests passed.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on the branch and on a
scratch tree of the branch with `main`'s orders 178, 179 and 196. Under `env -i` it ran
`check_docs`, `check_rulings`, `set_names.py --check`, `check_schemas`, `check_roadmap`,
`check_model_account` (with and without `--history`), `promise_standing.py --check` and
`check_private_paths`, each Errors: 0 on both trees;
`vector_delta.py origin/main HEAD --declared-changed`, which gave 7 changes, 0 flips, 0 undeclared
and 0 errors; `git diff --check`, clean; and the 26 counts of Done when, 15 printing 0 and 11
printing 1. On the scratch tree `cargo fmt --check` and clippy were clean, the pieces tool printed
`Nothing to move.`, both runner commands printed what Done when states, the check by hand of steps 4
and 5 printed `PASS e4-reconcile-open-records-only (5 steps; LIF-028, ACC-044)`, and
`cargo test --release` passed 242 tests. With the core's reconciliation of an open switched off, as
review order 22 will build it, the open case passed all its steps; with a reconciled setup made
ready, the setup case failed with `object_facts/state: expected "registered", found "ready"`. So
both new cases can fail and can pass. It found each decision applied word for word, the
coordinator's readings marked as the coordinator's, nothing on sign-in, and the scope exactly the 20
files. It found no must and three shoulds:

- `spec/core/results.yaml`, outside Scope, still said in `provenance.v1`'s description that "Repeats
  and reconciliation preserve the original provenance", the twin of the ACC-044 sentence this order
  rewrites, which ruling 78, point 5, removes.
- The DL16 insertion left a line of 98 characters and a line holding only "again,", and the new
  bullet of `spec/README.md` was wrapped at 83 to 87 characters, not at its list's width of 96.
- A follow-up, not for this order: under ruling 80, point 4, a setup action stays open over an
  invocation whose outcome reads succeeded, and PND-006's evidence does not say that a
  reconciliation set that outcome (ruling 52).

A Claude Opus fixer then rebased the branch onto `main` `ecabe372`, which holds orders 178, 179 and
196 and touches none of this order's files; the rebase had no conflict. It applied the first two
shoulds as the reader gave them. Decision 4 and Scope now name `spec/core/results.yaml`, and its
sentence reads "Repeats preserve the original provenance, and a reconciliation of an open gives
none, since it opens nothing (ACC-044); a purge never removes it.", rewrapped over three lines
within the paragraph's width of 98. As its own choices, the fixer added to Done when a count of the
old words, which prints 0, made Size count ten files under `spec/`, and rewrapped the Scope
paragraph within 99. DL16's three lines are those the reader gave, and the bullet's six lines are
rewrapped within 96, with no word changed. The third should is left for the coordinator to log, for
SO-E or S9-CORE2: where a reconciliation set the outcome, PND-006's evidence would name that
reconciliation's invocation and its `checked` text.

The fixer then ran, on the Mac in the worktree after the rebase and the fixes, under `env -i`:
`check_docs`, `check_rulings`, `set_names.py --check`, `check_schemas`, `check_roadmap`,
`check_model_account` (with and without `--history`), `promise_standing.py --check` and
`check_private_paths`, each Errors: 0; `vector_delta.py origin/main HEAD --declared-changed`, which
gave 7 changes, 0 flips, 0 undeclared and 0 errors; `git diff --check origin/main...HEAD`, clean;
and the 27 counts of Done when, 16 printing 0 and 11 printing 1. From `engine/`, with
`CARGO_NET_OFFLINE=true`, `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` were
clean and the pieces tool printed `Nothing to move.` The runner printed, for `e4-reconcile-open-`,
`PASS` for the two older cases, the expected failure with
`object_facts/open: expected false, found true` and `2 cases passed; 1 expected failures`; for
`e4-reconcile-setup-`, `PASS e4-reconcile-setup-records-only (5 steps; LIF-028, LIF-003)`, the
expected failure with an error beginning
`step 6: /reply/structuredContent/status/pending: missing required field` and
`1 cases passed; 1 expected failures`. The check by hand printed
`PASS e4-reconcile-open-records-only (5 steps; LIF-028, ACC-044)` and `1 cases passed`; both files
then matched their copies under `cmp`, and the first runner command printed what it printed before.
`CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors` gave
`2 passed`, and `CONDUIT_VECTOR_THREADS=4 cargo test --release` exited 0, with 242 passed and 0
failed over its 28 results.
