# Ruling 78: the full review's answers: one coordinator to the pin, S1's bar in two groups, no fence, a reconciliation only records, root alone installs, three cuts

Status: in force; amended by 80 (point 5: a setup's pending action stays open; point 6: F13 after a late success, and not for a call that changes nothing; point 17: the measures file is not the store) (ruled by the owner on 2026-11-14). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topics 1, 2, 4, 5, 7 to 13 and 15 (spec pending wherever `spec/` must change). This record says why.

At the handover of 2026-11-12 the owner asked for a full review of the repository and the progress
so far. The coordinator carried it out on 2026-11-14, on commit `2eb8f258`, and put
[17 questions](../full-review/questions.md) to the owner, each with lettered options, a
recommendation, what it changes and what waits on it ([the full review](../full-review/README.md)).
The coordinator's own decisions D1 to D88 and fixes F1 to F144 are logged in
[decisions.md](../full-review/decisions.md) for the owner to strike.

The owner: "Let's go through each of the decisions one by one and attempt to compress the
context". The coordinator then put the questions one at a time, and the owner answered each by
choosing an option. The options chosen, as the owner gave them:

- 1. Part 1, a second Claude session until the pin: "(b) None (Recommended)". Part 2, sign-in
  before the pin: "(a) Yes, as planned (Recommended)".
- 2. S1's bar: "(b) Two groups (Recommended)".
- 3.1 v0's feature list: "(b) Close it (Recommended)". 3.2 The model-checking row: "(a) Amend
  the row (Recommended)". 3.3 v0's security claim: "(a) Drafted claim (Recommended)".
- 4. S2: "Coordinator picks (Recommended)". S3: "(E) Mac firewall (Recommended)". S7: "Split
  (Recommended)".
- 5. Reconciliation: "(A) Record only (Recommended)".
- 6. The fence: "(C) Drop both (Recommended)". With it, F13: "Stays unknown (Recommended)".
- 7. A call that changes nothing: "(A) Failed (Recommended)".
- 8. A restart mid-act: "(A) From the parts (Recommended)".
- 9. The engine's own panel: "(A) A screen you open (Recommended)".
- 10. install.v1: "(A) Root only (Recommended)".
- 11. Closing a secret's Conduit: "(A) Yes, at once (Recommended)".
- 12. A kept store under a changed core: "(C) Bring root up to date (Recommended)".
- 13. The machine provider: "(B) Bind to Type (Recommended)".
- 14. The domain of trust: "(B) Status says so (Recommended)".
- 15. The convention layer: "(A) Correct the words (Recommended)".
- 16. Cuts: "(A) Cut all three (Recommended)".
- 17. The four fixes, all four accepted: "F15 Store fails closed", "F39 Nothing starts itself",
  "F72 Local not asked", "F124/F127 Writing rules".

The coordinator summarized the answers and said it would record them as ruling 78. The owner:
"Continue". Every answer is the recommended option. The answers change decisions in force, so they
are a new ruling and not later answers to older ones (ruling 67).

## The decisions

1. **The order of the work until the pin**
   ([question 1](../full-review/questions.md#1-the-order-of-the-work-until-the-s1-reruns-build-is-pinned)).
   - *Part 1.* Until the build that the S1 rerun measures is pinned, one coordinator, the admin
     session, runs the work, and no lane runs beside it; the lane restarts at the pin. While the
     Claude allowance is the limit, a second session would only split it. The lane's checks that
     option (a) would have run first, among them two provider suites, A74's systemd probe, A6's
     lost-response stub and the blind-verdict read, run after the pin. This amends ruling 71 point
     1 until the pin, and changes IN_FORCE.md topic 15 and the roadmap's "In parallel" row.
   - *Part 2.* Sign-in stays before the pin: the pin waits for the six sign-in orders, and the owner
     signs each dogfood Agent in once. This changes no decision; orders 050 and 053 are written to
     it, and IN_FORCE.md topic 13 says so.
2. **S1's bar: two groups** ([question 2](../full-review/questions.md#2-s1s-bar)). In each run,
   the coordinator's and the fresh Codex author's, Conduit must take fewer lines than the baseline
   (A70) in both of ruling 45's groups: setup (T1 and T2), and change and repair (T4 to T7); T3 and
   the limit of one and a half times per task drop out. This was chosen before any count from the
   rerun exists, knowing ruling 76's forecast, and it makes a pass likelier; block layout still
   fails setup on the paper count. The question attached four rules to any answer unless the owner
   struck them, and the owner struck none, so they hold: lines count as the author wrote them, with
   the other readings reported beside; T3's commands count as inspection effort; the fresh author's
   pack holds nothing of S1's design, so the one short-form example in `spec/`, which is S1 itself,
   is replaced; and measures added now never gate. This amends ruling 71 point 3 and ruling 47's bar
   row, clarifies ruling 63 point 1, and changes IN_FORCE.md topic 13 and that example in `spec/`.
3. **v0's gate and scope** ([question 3](../full-review/questions.md#3-v0s-gate-and-scope)).
   - *3.1, the feature list is closed.* v0 holds what is ruled up to and including this ruling;
     batch 2, metrics and every fix stay as ruled. A later addition, small ones included, lands
     after v0 unless the owner's words say "in v0", and it reaches the owner with its size. This
     adds one line beside the scope guard in IN_FORCE.md topic 1, and no ruling changes.
   - *3.2, what model checking must show.* Each modelled safety promise holds under loss and
     reordering, and, where its parts sit on two machines, also under a cut and its repair; the
     coordinator classes which promises those are, for the owner to strike. Every other safety
     promise has a safety vector or a stated reason, and every model finding is fixed or stated as a
     limit; a cut is no longer required where a promise sits on one machine. The reruns of the
     models are registered against this target. This amends ruling 47's model-checking row and
     ruling 75 E1, and changes IN_FORCE.md topics 10 and 13, A40's done-when in the tracker and the
     roadmap's model-checking row.
   - *3.3, the security claim.* The review's corrected draft of v0's security claim
     ([FR-480](../full-review/findings/G3-native-probes.md#fr-480), as its verifier corrected it)
     becomes the draft for publication, and it grows with each route around the engine that A7
     finds; A7's rows still run, as observation. A20 and the plan audit's finding D5 close on it,
     on the coordinator's map of A20's three questions: cross-path to ACC-009, issuer-independence
     to ACC-030, and provider scope to its promises or stated as open. The native probes (A5, A6,
     A7, A10, A22 and A73) are put outside the scope of G8, the independent builds' gate. This
     changes IN_FORCE.md topic 13 and closes A20; no ruling is amended.
4. **The accepted plans of S2, S3 and S7**
   ([question 4](../full-review/questions.md#4-one-amendment-to-the-accepted-plans-of-s2-to-s10)).
   The plans of S2 to S10 have not run, so they can still be amended (ruling 45).
   - *S2.* The coordinator chooses a pinned open-source plain server that meets the published
     requirements, and logs the choice for the owner to strike; S2's plan had it chosen with the
     owner. Measure D20's list of places that can reach a credential also gains another Agent's
     home and client login.
   - *S3, option (E).* The Mac's own firewall cuts the network, run by the coordinator's locked-down
     script under one administrator rule that the owner adds once; two test cuts decide whether it
     works, and a path it misses gets a relay on the Mac. The owner still stops the distro at each
     loss, with `wsl --terminate` in place of O4's `wsl --shutdown`, which also stopped Docker
     Desktop's distro: 12 actions over four runs, not 72. A last phase is added to each run: the
     machine is declared lost while a test run is alive, so that P4's owed stop is seen to end a
     run and can fail.
   - *S7, split.* The outside author works against a scripted consumer, so it needs neither the
     owner's presence nor the second accounts. Real Agents on the second accounts then use the
     same packages, and the coordinator, not the outside author, sets them up.
   - The coordinator's other changes in the review's table for S2 to S10 apply, each dated and open
     to the owner's strike, and no round-6 evidence case or measure leaves its scenario. This amends
     ruling 75 D1 for S2's server and D20's list, S3's fault method, O4's command, P4 and runs, and S7's tasks and
     run; dated amendments go to experiments 013, 014 and 018, the second-machine guide and the
     roadmap's S3 and S7 rows, and IN_FORCE.md topic 13 cites this ruling. `spec/` does not change.
5. **A reconciliation records only**
   ([question 5](../full-review/questions.md#5-may-a-reconciliation-store-a-controls-values-or-make-an-instance-ready-on-an-actors-word-unsafe)).
   A reconciliation records the outcome and what was checked; it stores no control's values and
   makes nothing ready, and the settings stay reported unknown. The way on is to send again: the
   control, naming the current revision as HTTP's If-Match does, or a further setup, which leaves
   finished parts as they are (OPS-021); confirming a retirement's shutdown stays. A later order
   keeps only the outcome in REV-010, has REV-009 keep the settings reported unknown, keeps only
   confirming a shutdown in LIF-028, and removes REV-013, OPS-031, `reconcile.v1`'s `revision`
   input, the values kept for it and ACC-044's provenance for a reconciled open; the vectors
   reconcile-revision-in-progress, reconcile-revision-succeeded and fleets/e3b/reconcile-control
   shrink. This amends rulings 32 Q4, 33 F11 and 41 item 15 where they reach reconciliation, and
   ruling 75 D1 for S5's accepted P5, T5 and V4, and changes IN_FORCE.md topic 7.
6. **No fence, and no "did not occur"**
   ([question 6](../full-review/questions.md#6-keep-the-fence-of-ruling-75-a1-or-drop-the-record-did-not-occur)).
   No actor records that an unknown effect did not occur, so no attempt needs fencing: an unknown
   outcome stays unknown until it is recorded as having occurred. A hook that did not run has its
   work done by hand and recorded, or its instance is replaced, as Terraform replaces a resource
   whose provisioner failed; on a plain server a retry is a new request, so the Agent itself makes
   sure the first call can no longer land, as with a remote server today. A later order drops "did
   not occur" and the fence from OPS-014 and says there how to move on after a hook, takes the
   fence's text out of OPS-018, OPS-024, OPS-030, OPS-031, LIF-028, REV-010 and `reconcile.v1`,
   and changes S1-reconcile, the eight vectors that record "did not occur", S5's `occurred: false`
   branches and the engine's fence marks. This amends ruling 75 A1, the two-way reconciliation of
   rulings 20 OD2, 32 U4 and 41 item 15, and ruling 75 D1 for S5's `occurred: false` branches, and
   changes IN_FORCE.md topic 7. The first of the two limits in ruling 75's later answer, that the
   fence covers Conduit's own dispatch and each hop handles its own idempotency, has no fence left
   to limit.
   - *F13.* Where a later attempt is answered failed while an earlier attempt of the same
     invocation has no answer, the invocation stays unknown, and no attempt number is sent. F13 is
     no longer an interim fix: it is ruled.
   - *F12*, the coordinator's narrowing of the fence's evidence, is withdrawn with the fence, and
     its conflict refusal, asked only under option A, falls away.
7. **A call that changes nothing ends failed when no answer comes**
   ([question 7](../full-review/questions.md#7-what-a-call-that-changes-nothing-reports-when-no-answer-comes)).
   This covers an operation that states no effect, is not a control and starts no model work. For
   such a call, no answer, a cancellation or an unanswered input request ends failed, and the
   result says that no answer came; nothing is listed as unreconciled, and a repeat is sent
   (OPS-024), as an HTTP client may resend a safe method (RFC 9110, section 9.2). A later order
   changes OPS-007, IFC-010, IFC-016 and OPS-027 for this class, and has OPS-034's first case
   dispatched as OPS-024 dispatches; IN_FORCE.md's "An unknown outcome stays unknown" stands, since
   such a call is no longer unknown. This amends ruling 41 item 27, ruling 49 (an unanswered input
   request ends unknown) and, for this class, ruling 75's later answer that a call to a remote
   server that never answers stays unknown, and changes IN_FORCE.md topics 7 and 9.
8. **A restart mid-act reports from the parts**
   ([question 8](../full-review/questions.md#8-what-a-setup-retirement-or-cascade-reports-after-a-restart-in-its-middle)).
   After a restart in the middle of a setup, a retirement or a cascade, the verb has ended with the
   outcome its parts give, as stored: a part with a call outside the engine in flight ended
   unknown, and a part not begun was not dispatched. The engine stores that it is about to send a
   call before it sends it, and a cancellation of the same verbs already reports this way
   (OPS-027). A later order changes OPS-022 and LIF-021 (the restart sentence), IMP-007 (an
   exception for these verbs), OPS-008 (store before sending), and the first build's recovery and
   vectors, which then report failed and pending where they now report unknown. No ruling is
   amended: ruling 30 forbids only sending anything again after a restart, which this keeps.
   IN_FORCE.md topic 8 changes.
9. **The engine's own panel is a screen the person opens**
   ([question 9](../full-review/questions.md#9-what-the-engines-own-panel-is-and-who-may-see-it-unsafe)).
   The panel is the engine's own rendering to a terminal or window; it is never returned to a
   request or written to a pipe, file or message, as ssh, sudo and GnuPG's pinentry ask for a
   password on the terminal. In v0 only root's person sees it, since root receives sign-ins in
   practice (ruling 75 C2), so a scoped Interface's device codes show there too; a model on the
   same user with a terminal tool can still read the panel, and that becomes a stated limit. A
   later order, with order 051c building it, adds a definition to `spec/definitions.md` and changes
   its "control panel" row, adds one interface promise that PND-004, PND-008, ENR-003, ENR-004,
   ENR-005, IFC-049 and PRV-014 cite, narrows PND-004's audience sentence and the device leg of
   PND-010, adds one implementation-defined item and one safety vector, and changes the limits
   "Pending actions, sign-in and the network" in `spec/README.md`. This keeps ruling 56 point 4 and
   records with it what the panel is, narrows ruling 66 point 3 for the device leg, and changes
   IN_FORCE.md topic 9.
10. **Root alone installs in v0**
    ([question 10](../full-review/questions.md#10-what-an-actor-other-than-root-may-do-with-installv1-unsafe)).
    `install.v1` is reserved to root, beside the five operations ACC-032 already reserves; no
    broker, scoped Interface or peer's admitted Interface installs. This closes both paths the
    review found: learning a secret's digest from a refused install, and installing an archive at a
    trusted package's address to share its client IDs. Opening install to others under guards can
    come in v1, since opening it later only adds. A later order changes `install.v1`'s contract,
    ACC-032, ACC-033 row 9, CTR-013 (its clause on a root-only input goes), `spec/README.md` ("What
    access does not narrow" and the named exceptions to preservation test 5), one vector and the
    engine's reserved list. No ruling is amended; IN_FORCE.md topics 1 and 5 change.
11. **Closing a secret's Conduit ends the engine's own use at once**
    ([question 11](../full-review/questions.md#11-whether-closing-a-secrets-conduit-stops-the-engines-own-use-of-the-value)).
    Closing the Conduit through which the engine reads a secret, or retiring the secret Resource,
    ends the engine's holding of the value: the next request reads again, is refused and reports
    unavailable, while the instance stays ready. A run keeps a value it was given, as ruled, since
    a close cannot recall a copy. A later order changes SEC-013, SEC-023 and the limits on secrets
    in `spec/README.md`, with one vector, and the engine drops the header values it holds when the
    reading Conduit closes. This amends item 6 of ruling 41's secrets record for a value that the
    engine itself holds, and changes IN_FORCE.md topic 12.
12. **A kept store under a changed core: root is brought up to date**
    ([question 12](../full-review/questions.md#12-a-kept-store-under-a-changed-built-in-core-package)).
    Stored objects read the built-in core package, and no stored Conduit changes, with one
    exception: at each start the engine brings root's own Conduits made from core templates up to
    the built-in lists. The step is recorded and dispatches nothing, as the Kubernetes API server
    updates its built-in roles at start; only root gains, and only what it may already open for
    itself. A later order changes NAM-020, adds one convention promise and two vectors, makes an
    exception to FLEET-005's "never widen" (batch 1's text, not a ruling), and has OBS-004's
    observable name the step, which is no background reconciler. No ruling is amended;
    IN_FORCE.md topic 5 changes.
13. **A node is bound to its machine provider's Type**
    ([question 13](../full-review/questions.md#13-upgrading-or-swapping-the-machine-provider)).
    A node is bound to the Type of its machine provider, not to the provider's instance, as a
    Kubernetes Node names its cloud provider. A replacement of the same Type that reports the same
    hostname and machine id finds the node again, the id coming from the machine's key (ruling 51
    point 8), so an upgrade of the SSH provider strands no node; a provider of another Type still
    strands nodes, a stated limit, decided when such a swap is taken up. A later order changes
    PLC-026, PRV-013's observable, ENR-006 check 3 (with the coordinator's one-live-node fix),
    `node.v1`'s description and one vector. This amends ruling 68 point 5 for batch 1's reading 19,
    and changes IN_FORCE.md topic 10.
14. **Status says when a run is inside the engine's domain of trust**
    ([question 14](../full-review/questions.md#14-telling-the-user-that-a-consumer-runs-inside-the-engines-domain-of-trust)).
    The status of each Agent and Resource that has a run says whether that run is inside the
    engine's domain of trust, as a run on the engine's own runner is, and inspecting its Conduits
    shows the same, as Kubernetes shows `privileged` on a Pod. Restrictions on such a consumer are
    still reported as mediated, as ruled, so the member is what tells Agents on the first machine
    from those on the second. A later order changes LIF-013, the status in `spec/core/results.yaml`
    and ACC-027's observable, with one vector, and the engine sets the member from the runtime it
    chose; the coordinator also rewords PRV-029's "isolation is mediated". This reverses order
    075's logged choice to add no runtime member before a confining provider exists, which was the
    coordinator's; no ruling is amended, and IN_FORCE.md topic 10 changes.
15. **The convention layer: the words are corrected**
    ([question 15](../full-review/questions.md#15-the-shared-conventions-say-what-the-layer-holds-or-move-rules-out)).
    The seven conventions stay few and stated as invariants, and the words that describe the
    convention layer say that it also holds the provider contracts and the core's own procedures.
    LD1, the test that assigns a rule to a layer, is restated for the three layers. No promise moves
    or changes, and the brake on growth is point 3.1. A later order corrects the sentence of
    `spec/README.md` that calls the conventions "the few behaviors every part must agree on", the
    tracker's [layered claim](../../AUDIT.md#layered-claim) and its LD1, and the ledger's lines for
    rulings 13 and 14 record the change. This amends the words of rulings 13 and 14, and changes
    IN_FORCE.md topic 2.
16. **Three cuts**
    ([question 16](../full-review/questions.md#16-cutting-surface-that-nothing-uses)). Handles, the
    `manages:` shorthand and the labels that a machine provider stores and reports back are cut:
    nothing shipped depends on them, and each says again what the language already says. A later
    order removes IFC-011, `handle.v1` and its schema keyword, SCT-012, `manages.v1`, its
    reserved-key entries and SCT-012's two invalid examples, registers IFC-011 and SCT-012 as
    removed IDs, and changes CTR-002, CTR-012, IFC-033, SCT-006, SCT-013, SCT-014, OBJ-019,
    OBJ-022, ENR-002, ENR-006 check 4, ENR-010, PRV-014, `spec/definitions.md`,
    `spec/core/README.md`, the sectioned example and two fixtures. The labels go once the
    coordinator's fix for [FR-050](../full-review/findings/L05-providers.md#fr-050) has the engine
    match them itself, as Kubernetes matches a node selector; order 043b, on the path to the pin,
    builds no `manages:`, and a scope is written as `consumes` items naming the core's scope
    templates. This amends rulings 31 I5, 41 P1, 66 point 1 and 52 point 4, and changes
    IN_FORCE.md topics 4, 9, 10 and 11.
17. **Four of the coordinator's fixes are accepted**
    ([question 17](../full-review/questions.md#17-four-of-the-coordinators-fixes-that-change-something-you-accepted)).
    Each reverses a choice the owner had accepted, so each came to the owner; each lands with the
    order that orders.md names.
    - *F15, the store fails closed.* When the engine cannot write or read its store, every request
      is refused and reports unavailable, and nothing is dispatched or changed. This amends the
      secrets record accepted with ruling 41, which failed closed for reads of secrets alone and
      left other requests to each engine (DL10), and changes IN_FORCE.md topic 7. The rest of F15
      stays the coordinator's: the answer names ACC-037 and is an exception to REC-001, and DL10
      says whether and how root can still purge then.
    - *F39, nothing starts itself.* `conduit serve` sets up no registered instance at start;
      bringing a project back after a restart becomes one explicit command, recorded as root's
      (OBS-004). It is made after the S1 rerun's pin, and the dogfood harness then sends the command
      after each restart. This reverses order 027's settlement of finding 4-07, amends ruling 58
      point 1 for that finding, and changes IN_FORCE.md topic 8.
    - *F72, `local` is not asked.* For an instance placed on the engine's own machine, the engine
      asks no machine provider. This withdraws the provider's "or none" for `local` that ruling 42
      accepted (item 10 of its machines log, on PRV-015), and changes IN_FORCE.md topic 10. The rest
      of F72 stays the coordinator's: elsewhere the engine passes the provider the machines it
      matched, and the provider names one or none.
    - *F124 and F127, the writing rules.* Rule 1 of ruling 17 NS2 (one obligation per promise) is
      reworded to say what a promise may hold, a rule is added on what an `observable` may say, and
      the meaning of a promise's `vectors` field is reworded: the manifest's case tags are the one
      record of which cases test a promise. No behavior changes. This amends ruling 17 NS2, and
      changes IN_FORCE.md topic 2.

## What it amends

- **Rulings 13 and 14**: their words on the convention layer, and LD1's test (point 15).
- **Ruling 17**: NS2's writing rules for promises (point 17, F124 and F127).
- **Ruling 20**: OD2, where it lets a reconciliation record that an effect did not occur
  (point 6).
- **Ruling 31**: I5, handles (point 16).
- **Ruling 32**: Q4, a reconciled control storing its values (point 5), and U4, where it lets a
  reconciliation record that an effect did not occur (point 6).
- **Ruling 33**: F11, a reconciled control storing its values (point 5).
- **Ruling 41**: item 15 where it reaches reconciliation (points 5 and 6); item 27 for a call that
  changes nothing (point 7); P1, which kept the name `handle.v1` (point 16); and its secrets
  record: item 6, for a value that the engine itself holds (point 11), and the choice to fail
  closed for reads of secrets alone (point 17, F15).
- **Ruling 42**: the machine provider's "or none" for `local` (point 17, F72).
- **Ruling 47**: the bar's row (point 2) and the model-checking row (point 3.2).
- **Ruling 49**: an unanswered input request ends unknown, except for a call that changes
  nothing (point 7).
- **Ruling 52**: point 4, the labels that the machine provider binds and reports back (point 16).
- **Ruling 58**: point 1, for finding 4-07, whose settlement by order 027 F39 reverses
  (point 17).
- **Ruling 63**: point 1, clarified: the primary count is the lines as the author wrote them, with
  the other readings beside (point 2).
- **Ruling 66**: point 1, the `manages:` shorthand (point 16); point 3, narrowed for the device leg:
  a scoped Interface's person no longer sees the device codes of its Fleets' Agents, which show
  only at root's panel (point 9).
- **Ruling 68**: point 5, for batch 1's reading 19, a node bound to the provider's instance
  (point 13).
- **Ruling 71**: point 1, until the S1 rerun's pin (point 1); point 3, the bar's rule (point 2).
- **Ruling 75**: A1, the fence (point 6); D1, for S2's server and D20's list, S3's fault method,
  O4's command, P4 and runs, S5's P5, T5, V4 and `occurred: false` branches, and S7's tasks and run
  (points 4 to 6); E1, what model checking must show (point 3.2); and the second limit of its later answer of
  2026-11-12: a call to a remote server that never answers stays unknown, except for a call that
  changes nothing (point 7).

Points 3.1, 3.3, 8, 10, 12 and 14 amend no ruling, and part 2 of point 1 changes nothing. Not a
ruling: the closure of finding 4-12 on 2026-11-12 kept `reconcile.v1` as it was, with the owner's
agreement ([increment 4's log](../first-build/log-increment-4.md)); point 5 now changes it.

## What it closes or answers

- **A20 and the plan audit's finding D5** (not the review's decision D5) close on point 3.3, on
  the coordinator's map of A20's three questions as question 3.3 states it: cross-path to ACC-009,
  issuer-independence to ACC-030, and provider scope to its promises or stated as open. A20 is done.
- **S3-OF**, how S3 cuts, heals and loses the machine, is answered by point 4's option (E), which
  replaces the coordinator's earlier recommendation, (A).
- **S2's plain server**, which waited for the owner before S2's runs, is the coordinator's choice
  (point 4).
- **A17**, "State what runs with no frontend attached and what starts it", closes on F39
  (point 17), as the coordinator's D79 says: nothing starts itself, and one explicit command brings
  a project back. Its promise is written after the pin.
- **G8** keeps its other conditions; the native probes are outside its scope (point 3.3).
- **Ruling 56 point 4** stands. Point 9 records with it what the engine's own panel is, as question
  9 says.
- **Ruling 75's later answer, its first limit**: moot with the fence (point 6).

The questions the review lists as asked later, and what it lists as not asked, stay as
[questions.md](../full-review/questions.md) states them.

## What follows

- The work orders of the review run as [orders.md](../full-review/orders.md) sets them out, under
  these answers: one coordinator to the pin, with the six sign-in orders on the path. SO-B no
  longer carries the narrowing of the fence's evidence, and the fence's rewrite goes to SO-E;
  under point 6, S6-END goes and S4-LATE shrinks. The hold on re-pinning the dogfood engine to
  `main`, kept until question 6 was answered, ends.
  Order 048a-2 and order 051c are written to point 9, and order 043b builds no `manages:`.
- Each change to `spec/` that this ruling decides is marked spec pending in IN_FORCE.md until the
  order that writes it lands. The dated amendments to the plans of S2, S3, S5 and S7, and to the
  second-machine guide, go with the review's orders for them; until then the guide, the handover
  and the independent builds' G8 assessment carry a dated line that points here.
- The coordinator's decisions D1 to D88 and fixes F1 to F144 stay logged in
  [decisions.md](../full-review/decisions.md) for the owner to strike (ruling 48). The exceptions
  are the parts ruled here: F13, and the parts of F15, F39, F72, F124 and F127 that question 17 put
  to the owner. F12 is withdrawn with the fence. Each of those rows in decisions.md ends with a
  dated note that says so.
