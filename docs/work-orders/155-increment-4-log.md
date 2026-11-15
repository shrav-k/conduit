# Work order 155: increment 4's log: the dogfood frictions and the Records' findings

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty. This order needs no network access: use none, and
reach no other machine. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never print a key, a token, a password or any credential.

## What and why

Ruling 70 asks that each friction of Conduit running its own orders be logged as a finding of the
first build, and ruling 46, point 4, asks the same of what the build meets in the promises. The
full review of 2026-11-14 (order 11 of [its orders](../proposals/full-review/orders.md)) found
that none of order 042's frictions is in
[increment 4's log](../proposals/first-build/log-increment-4.md), nor several findings that the
review and the scenario orders' Records made, and that rows 4-01 and 4-11 still read "being
settled with the owner" though later rulings decided most of them
([FR-288](../proposals/full-review/findings/P02-s1-bar.md#fr-288), item 3). This order writes those
rows as the review's verifiers corrected them and as [ruling 78](../proposals/ruling-78/README.md)
decided, and points the handover's list of frictions at them, before the next order runs through
Conduit.

## The decisions it applies

Read each finding a row carries. Where this order is silent, its verifier's corrections govern;
where this order and a finding differ, this order governs.

1. **Form.** Append rows 4-21 to 4-34, in that order, after 4-20: one table line each, with the
   log's six cells. Each Finding cell ends with a link to what it carries, written from the log as
   `../full-review/findings/<part>.md#fr-NNN`; 4-27's link is
   `../full-review/findings/P06-dogfooding.md#what-the-verifier-says-the-analyst-missed`. A class is
   one the log already uses (gap, ambiguity, contradiction, needless complexity, defect of the
   build), or "defect of the harness" for a fault of the dogfood harness, or "limit of the build"
   for a limit of a library the build uses. A row of either of those two classes says in its
   Promises cell that it is not a finding against the promises, as
   [the first build's log](../proposals/first-build/README.md#what-a-finding-is) marks a tool's
   defect. Where a draft row marks a fact "(log)", write the fact without the mark, as the
   coordinator's account, citing order 042's Record where the Record holds the fact. Cite the
   review's orders as "the full review's order N", linked to `../full-review/orders.md`, and its
   decisions and fixes by number (D58, F61), linked to `../full-review/decisions.md`. In the text
   this order adds, put no line count, forecast, code line number or path outside the tree. Change
   no row but 4-01, 4-06, 4-11 and 4-12, and renumber nothing.
2. **Rows 4-21 to 4-26** start from the draft rows of their findings, changed as follows. Where a
   settlement is given here, it replaces the draft row's settlement whole.
   - **4-21** ([FR-338](../proposals/full-review/findings/P06-dogfooding.md#fr-338)): the incident
     is "in round 3 of order 042, which ran on 2026-11-11 and 12". The Build cell says that the
     round was run again, that the Record does not say whether the unknown turn was reconciled, and
     that a guard tripped (4-25). Settlement: the full review's order 17 gives the `.v2` wrappers,
     before 0.2.0 publishes, a `network` class, found by fixed substrings as the other classes are,
     and a failed result whose class, turn number and whether the client started are fields. No
     client text reaches a result, an error or stderr, as order 050a ruled, so no tail of error
     events is kept. The refusal of later turns of a failed conversation stays until the S1 rerun.
     It shaped the fresh author's recovery (4-12), and ruling 71, point 4, keeps from the
     coordinator what could move S1's counts (D55). It is also what now stops a new turn in the
     same run from running beside a failed turn's leftover processes, so it ends no sooner than the
     full review's order 16 (4-23). In the Claude wrapper, which runs with
     `--no-session-persistence`, the refusal is the explicit answer. Whether Codex's own retry
     limits caused the failure is for the probe of network loss in
     [FR-337](../proposals/full-review/findings/P06-dogfooding.md#fr-337). No change to `spec/`.
   - **4-22** ([FR-340](../proposals/full-review/findings/P06-dogfooding.md#fr-340)): defect of the
     harness; Promises: a stated limit observed (CTR-014, PRV-006, ACC-027). "With no prompt" means
     the client asked for no permission. "The gap is in what stands guard meanwhile" becomes "The
     fault is in what stands guard meanwhile". Settlement, D58: the harness's guards read a slot
     with the harness's own git directory and the slot only as its working tree, or by plain file
     reads, and never run anything an Agent could have written; fetching from the slot cannot read
     uncommitted work. This is in place before the engine moves to another user, and, under D61,
     before any Claude Agent becomes a dogfood slot and before batch 2. Ruling 75's B2 and B3
     stand; B2's provider also waits for a change to `spec/`, which is the owner's (D55).
   - **4-23** ([FR-332](../proposals/full-review/findings/P06-dogfooding.md#fr-332)): defect of the
     build, one class. Write "terminal (PTY) children", not "each in a session of its own". The
     runner submits `builtin-run-stopped` only on Runner drop, at engine shutdown, right after
     signalling SIGKILL. The dogfood engine's pin predates order 077, so its fence did not run in
     042, and the smoke test showed children outliving the kill, not a fence recorded wrongly.
     Add to the Finding that, with no reconciliation at all, the next turn, on any key, runs beside
     what the earlier one left. Settlement: ruling 78, point 6, drops the fence from OPS-014 and
     the engine's fence marks (spec pending), so no sentence on the fence's limits goes to the
     review list. The stop must still mean what it says:
     - the full review's order 13 has the runner record a stop only once it has seen the run's
       process group gone, after TERM, a grace period and KILL, and otherwise submit nothing and
       say so; the dogfood engine's re-pin waits for it;
     - its order 16 has the `.v2` wrappers stop their client's whole process tree, found by a walk
       of parent links that reads no process's environment, and name what they could not stop,
       since a child that left the group outlives a stop of the group;
     - until then the harness's sweep stands in;
     - what a run is, and when its stop succeeds, goes to `spec/` as fix F61, with the full
       review's order 21.
   - **4-24** ([FR-339](../proposals/full-review/findings/P06-dogfooding.md#fr-339)): add that the
     restart works because a further setup starts a run that has stopped (CONFORMANCE.md, DL16),
     and that no operation stops a run short of retirement. Settlement: the full review's order 62,
     after the pin and once order 050 has published 0.2.0, gives the capability that holds
     `turn.v1` in each `.v2` package a separate operation, not marked as model work. It starts a
     new conversation and returns its ID, so that a Conduit can grant turns without the right to
     discard an Agent's context. Continuing a named conversation works for Codex (`resume`), not
     for the Claude wrapper as built. An input on `turn.v1`, the handover's earlier plan, is
     dropped: it would fail 050a's byte-for-byte check of `.v1`. Until order 62, the harness's
     restart stands, and its cost is measured again after the re-pin.
   - **4-25** ([FR-341](../proposals/full-review/findings/P06-dogfooding.md#fr-341)): defect of the
     harness; Promises "none; rulings 52 and 74, point 1". Settlement, D58: each guard answers
     held, broken or unknown, and an unknown is checked again before the next turn is sent. The
     guard that nothing was pushed stays (ruling 74, point 1), under a dedicated user too; dropping
     it is the owner's.
   - **4-26** ([FR-335](../proposals/full-review/findings/P06-dogfooding.md#fr-335)): write that
     `spec/` leaves the time allowed for a call to the engine (DL3, DL10), not that it "names no
     deadline". The deadline is set in three places: the Type's input, the client's tool timeout,
     and the engine's call timeout with its session idle limit (4-20). The engine relays no
     progress notification during a call, and neither `spec/` nor CONFORMANCE.md says whether it
     does. "This reopens part of 4-12" becomes "It bears on 4-12, closed with the owner's
     agreement; reopening it is the owner's (D57)". Settlement: the full review's order 14 has a
     dispatch deadline cancel its provider call before reporting it missing. A deadline per turn
     set by the caller stays the owner's, with 4-12. No deadline is raised again as the answer to
     a long turn (D57). Before A78's order 5, FR-337's probe of a lost caller measures the blocking
     route, the only one the engine serves, since the engine has no code for the protocol's tasks
     (IFC-016). Whether the engine relays progress notifications stays open.
3. **4-27**, gap, ACC-027, CTR-014, PRV-006, carrying the P06 verifier's item on 4-06
   ([its list](../proposals/full-review/findings/P06-dogfooding.md#what-the-verifier-says-the-analyst-missed)).
   4-06's settlement rests on a trial in which a sub-agent had no shell and a read-only sandbox.
   With `codex.v2` and a directory, the client's sandbox is off. The package's README on order
   076's branch (`git show origin/wo/076-agents-v2-folder-grant:packages/agent-codex/README.md`)
   says that Codex 0.167.0 still offers its collaboration tools and "their subprocesses inherit
   this permissive reach". So a dogfood worker can run parallel, unconfined model work that no turn
   names. Build: that README states it. Settlement: stated and unconfined, as ruling 70 allows. A
   sub-agent's requests of the engine carry its parent's credential (4-06). Confinement waits for
   ruling 75 B2's provider and its change to `spec/` (D55), and for the engine to call a runtime
   provider, which order 080 builds (CONFORMANCE.md).
4. **4-28**, defect of the build, PRV-014, ENR-010, PLC-007
   ([FR-252](../proposals/full-review/findings/X10-queued-and-kept.md#fr-252)). The first-start
   report maps five Rust architectures to Go's names and passes the rest through; give the
   finding's examples. ENR-010 fixes `local`'s labels at first start, so a wrong name stays, and a
   `nodeSelector` with Go's value never matches `local`. Build: nothing yet; no affected host is in
   use. Settlement: the full review's order 66, after the pin (F78), uses the local machine
   provider's table, with a unit test, since no portable vector can test a host's value. Repairing
   a store first started on such a host would be DL6's "how repair works", stated in
   CONFORMANCE.md. No change to `spec/`.
5. **4-29 to 4-31**, from the scenario orders' Records
   ([FR-391](../proposals/full-review/findings/E01-s2-s3-s4.md#fr-391)):
   - **4-29**, ambiguity, IFC-006, OPS-008. IFC-006 gives an implementation the target, the
     caller, the invocation and any key, but not the instance that sent the request, while OPS-008
     scopes a key by that sender, so a deduplicating server (order 143's) cannot rebuild a key's
     scope. Build: S4-SRV1 keys on the invocation. Settlement: open, a wording fix in `spec/`
     (rulings 37 and 46, point 4) that says which identity a deduplicating implementation keys on.
     The nearest reading is order 143's, the invocation ID, which the engine keeps one to one with
     caller, sender, target and key.
   - **4-30**, ambiguity, CTR-002, IMP-003. Order 143's fixture gives a Resource an operation
     marked `startsModelWork: true`, and `install` and `submit` accepted it. The definitions make
     model work a turn of an Agent's model, no rule refuses the mark elsewhere, and round 6's E01,
     which S4 carries, asks for a plain server's operation so marked. Build: accepted. Settlement:
     open, the coordinator's (rulings 37 and 46, point 4), decided before S4's runs: either E01's
     case is written for an Agent's operation, or a rule refuses the mark outside an Agent's Type.
   - **4-31**, limit of the build, OPS-027, OPS-013. rmcp 3.9.0, the MCP library of the engine and
     of S4's servers, drops the answer to a request whose peer cancelled it, as OPS-027 says a
     receiver that stops does. A cancellation that arrives after the server's check can still
     discard an answer the server released (orders 143 and 144). S4-LATE and round 6's E20 rely on
     a late answer, and once the full review's order 14 cancels a call at its deadline, a late
     answer to that call never leaves such a server. Build: S4-SRV1's README
     (`packages/scenario-s4-effect`) states it; S4's plan keeps it. Settlement: no change to
     `spec/`. S4-LATE and E20 rely on no answer to a call the engine cancelled, and ruling 78,
     point 6, shrinks S4-LATE.

   Three items are not logged; name each in your report: the hook's `unavailable` to any filter,
   which IFC-019 requires; D05's onward call, unexercised in orders 133, 136 and 139, a coverage
   fact of S2; and PRV-021's owed stop (order 141), queued in the handover for the specification.
6. **4-32**, defect of the build, OBS-004, PRV-026, DL16
   ([FR-408](../proposals/full-review/findings/E03-s8-s9-s10.md#fr-408)). At each start, before it
   listens, `serve` sends `set-up.v1` under root's identity for every `registered` instance and
   every `ready` one with a runnable item, which no actor asked for. CONFORMANCE.md says both "No
   automatic restart" and that serve restarts stopped runs. DL16 can cover restarting a ready
   instance's run, but not a setup, under root's identity, of an instance never set up. S9's
   second-round draft expected no hook call at restart; order 102 rewrote its T4a and M2 to match
   the engine, and no finding was logged. S9 can no longer fail on a dispatch at start for a failed
   or never-set-up instance, though it can still fail on an implicit redispatch of unknown work.
   Build: as order 027 settled 4-07, and order 102's rewrite. Settlement: ruling 78, point 17
   (F39). `serve` sets up no registered instance at start, and one explicit command, recorded as
   root's, brings a project back (OBS-004). The full review's order 66 builds it after the pin and
   notes the reversal in 4-07; S9's T4a and M2 then follow OBS-004. Leave 4-07 unchanged.
7. **4-33**, gap, PRV-029, LIF-035, PND-011, CTR-013
   ([FR-431](../proposals/full-review/findings/E04-signin-isolation.md#fr-431)). No operation signs
   one Agent out: `sign-out-hook.v1` runs only within sign-in or setup (PND-011). Removal of a home
   with its instance is promised on the engine's own runner only. SSH-R's contract keeps its
   remote state directory across starts and never removes it, and no runtime call tells a provider
   that an instance is gone. On macOS a Claude login is a keychain entry outside the directory. So
   a retired Agent can leave a live login on the Windows computer and in the keychain. Build:
   nothing yet. Settlement, D53 and F98, with order 050: each Agent Type's retire hook signs its
   client out and empties its home, and its contract says so. Where no retire hook can run, as on
   a lost machine, the login stays until it is revoked at the vendor, as the limits and the
   accounts guide are to say. Until SSH-R's contract empties or removes the directory at
   retirement, this row records that it departs from [IN_FORCE.md](../../IN_FORCE.md)'s "made and
   removed with it" (ruling 49). No operation for a person to sign an Agent out is asked for.
8. **4-34**, gap, CTR-013, SEC-020, PRV-029
   ([FR-434](../proposals/full-review/findings/E04-signin-isolation.md#fr-434), its layer point).
   CTR-013, a language-layer promise on what a Type contract states, holds in its observable an
   obligation of the engine: that it starts no login and holds no token a login yields. It cites
   SEC-020 for this, and SEC-020 says neither. No convention-layer promise states the obligation,
   so the safety catalogue has none to hold. Build: nothing. Settlement: the full review's order 20
   corrects the citation to PRV-029 and the limits (F99). Whether the obligation gets a
   convention-layer promise of its own is open, and the coordinator's, since such a promise would
   restate an obligation and add none (rulings 37 and 46, point 4).
9. **Rows 4-01 and 4-11** (D24). Keep each row's finding, its Build cell and the owner's choice of
   2026-11-10. Replace the text from "Under consideration" in 4-01, and from "Being settled" in
   4-11, to the end of the settlement, with what has been decided and what is left:
   - 4-01, decided:
     - plan and apply, composed of today's requests (ruling 53, point 4; ruling 76);
     - `as`, which lets a standalone Conduit keep the declared names of its tools (ruling 53,
       point 3);
     - replacement under a stable name, retiring the old object first and accepting a longer
       window without access, to be measured in the rerun (ruling 55, A3);
     - bound items keep the ID they resolved to, and a replacement's plan prints each bound it
       stops covering (ruling 59, A7);
     - narrowing needs no flag (ruling 76, point 1);
     - a node is bound to its machine provider's Type, so a replacement of the same Type that
       reports the same hostname and machine id finds its node again (ruling 78, point 13).

     Then add: "Expected, not observed (D23): replacing an Agent removes its home and its sign-in
     (PRV-029), so each replaced Agent is signed in again, where upgrading a client in A70's
     baseline keeps its login." Left: one choice, weighed on the rerun's measures of windows
     without access and sessions lost, and asked after the rerun: replacement as ruled, a change in
     place, or durable state kept by name outside the instance
     ([FR-292](../proposals/full-review/findings/P03-plan-apply.md#fr-292) and its verifier). Open
     to the owner until S1's bar is judged.
   - 4-11, decided:
     - the short form is language, its frame optional, and an inline definition makes a template
       and its instance together (ruling 55, A1 and A6);
     - grants refer to Conduit templates with `from:` (A2), which packages ship (ruling 63,
       point 3);
     - `as` (ruling 53, point 3);
     - root's standing Conduits to every Pool and Fleet (ruling 59, A3);
     - the rerun's counting rules (ruling 63, point 1);
     - S1's bar in two groups, setup (T1 and T2) and change and repair (T4 to T7), with lines
       counted as the author wrote them (ruling 78, point 2).

     Left: the owner's judgment of the bar after the rerun.
10. **Two notes.** 4-06's settlement ends: "This holds for `.v1` and for `.v2` with no directory;
    for `.v2` with a directory, see 4-27." 4-12's settlement ends: "Ruling 78, points 5 and 6,
    changes its last sentence: a reconciliation records the outcome and what was checked, stores
    no control's values and makes nothing ready, no actor records that an unknown effect has not
    occurred, and `reconcile.v1` loses its `revision` input (spec pending)."
11. **The handover.** In `docs/implementation/HANDOVER.md`, section "Dogfooding: Conduit runs its
    own orders (A78)", remove the line "What failed or cost (log):", its four bullets, the sentence
    "None is yet a finding ... that each be." and the sentence "The plan's follow-ups: ... before
    the rerun.". Put this one sentence in their place: "What failed or cost is logged in
    [increment 4's log](../proposals/first-build/log-increment-4.md) as findings 4-21 to 4-27, each
    with its settlement: the network outage (4-21), the write inside a slot's `.git` (4-22), the
    leftover processes (4-23), the restart for each fresh conversation (4-24), the guard that
    tripped (4-25), the turn of hours (4-26) and the sub-agents' reach (4-27)." Keep the sentences
    that begin "Next: re-pin" and "After 050" as they stand.

## Scope

Change only `docs/proposals/first-build/log-increment-4.md`, and in
`docs/implementation/HANDOVER.md` only the passage decision 11 names.

## Done when

From the repository root:

```sh
F=docs/proposals/first-build/log-increment-4.md
grep -c '^| 4-' $F                                                    # prints 34
awk -F'|' '/^\| 4-/ && NF != 8 {print $2}' $F                         # prints nothing
git diff -U0 -- $F | grep -o '^-| 4-[0-9][0-9]' | paste -sd' ' -      # prints -| 4-01 -| 4-06 -| 4-11 -| 4-12
grep -n -e '(log)' -e 'Being settled with the owner' -e 'did not occur' -e 'marker variable' \
  -e 'bounded tail' $F                                                # prints nothing
for n in $(seq 21 34); do grep "^| 4-$n |" $F | grep -q 'full-review/findings/' \
  || echo "4-$n"; done                                                # prints nothing
grep '^| 4-06 |' $F | grep -c 'see 4-27'                              # prints 1
grep '^| 4-12 |' $F | grep -c 'points 5 and 6, changes its last sentence'   # prints 1
grep -c 'What failed or cost (log)' docs/implementation/HANDOVER.md   # prints 0
grep -c 'findings 4-21 to 4-27' docs/implementation/HANDOVER.md       # prints 1
grep -c 'newConversation' docs/implementation/HANDOVER.md             # prints 0
python3 tools/check_docs.py
python3 tools/check_private_paths.py
git diff --check
git status --short                                                    # the two files only
```

Do not stage anything. In your report, give each choice you made yourself, every row you could not
write as this order states and why, the three items of decision 5 that are not logged, and every
check's last line.
