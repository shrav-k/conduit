# Work order 048a5gh (E7a-5gh): another account found while `sign-in.v1` polls is undone, and a sign-in that no retry completes ends unknown at DL18

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the case you add and the case you change, with the
promises each tests and whether it passes, and put the shell test under `checks`. This order has
network access only because the engine's tests bind loopback ports, which the sandbox refuses
without it; use it for nothing else, add no crate and no crate feature, and reach no other machine.
Run one cargo command at a time. Never print a whole large file, case, fixture or log: use `head`,
`tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print
an environment variable's value, and never read or print a key, a token or any credential. Do not
run `claude`, `codex` or any client, sign in nowhere, and open no URL: the URLs in the cases are
test values that no program fetches.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry for 048a-2 cuts 048a-5 into parts, each
after the one before, since an order holds one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)); order 048a-5d's
refresh cut its own part in two, so that the entry listed nine: 048a-5, 048a-5b, 048a-5c, 048a-5d,
048a-5d2, 048a-5e, 048a-5f, 048a-5g and 048a-5h; it lists seven since ruling 88. On 2026-11-19
the owner chose, among options the coordinator put with a recommendation, two things for the path
to the Mac dogfood, which ruling 88 records with the owner's words:

- "Pairs: 5e+5f, 5g+5h (Recommended)", whose text read: "Two orders instead of four, each a little
  larger; the fresh read and fixer stay, so review depth is unchanged per line. Saves about 5
  hours. Waives the one-behavior-per-order rule for these two, as 048a-5b did." So 048a-5e and
  048a-5f are one order, 048a-5ef, and 048a-5g and 048a-5h are this one, which holds two behaviors:
  - **048a-5g:** another account found while polling, undone by a `sign-out-hook.v1` part, ending
    failed naming both accounts, the action staying pending with the attempt as evidence; a failed
    undo sets `SignInUndone=False`;
  - **048a-5h:** DL18's end: a sign-in with no retry ends unknown, and no hook runs later, on the
    input the shell submits at the output's expiry.
- "Defer 7 and 8 (Recommended)", whose text read: "048b-join comes right after 048a-5h and joins
  the URL relay and polling only; 048a-7, 048a-8 and 051b-3 (the return through Conduit, used by
  Codex from other machines once keyed) move after dogfood, beside the keyed release." Under
  [ruling 86](../proposals/ruling-86/README.md) every 0.2.0 Agent package's sign-in hook is plain,
  so PND-013's first check fails for each of them and the carried return has no 0.2.0 user.

The owner's answer, not the coordinator, waives FR-264's one-behavior rule for this order. So the
path to the Mac dogfood is order 209, 048a-5d, 048a-5d2, 048a-5ef, **048a-5gh, this order**,
048b-join (the URL relay and polling only), then 051b-2 (`conduit sign-in`). This order is the
last engine order of the sign-in chain on it. It is on the sign-in chain, so A77's amendment of
2026-11-19 does not pair it, and [ruling 89](../proposals/ruling-89/README.md), point 6, pauses
pairing until 050n merges ([work orders](../implementation/WORK_ORDERS.md)), and under ruling 87
it runs one Codex round, a worker and a fresh Codex verifier, whose musts go to the fresh Opus
reader and the Opus fixer.

**Written ahead.** The coordinator wrote this order on 2026-11-19 against `main` at `c097ec81` by
joining its two held drafts, of 048a-5g and of 048a-5h (each written against `3c761dd9`, before
048a-5b merged), and refreshing the result against what has merged since (orders 048a-5b, 048a-5c,
048a-6 and order 207's text), against the texts of orders 207 and 209, of 048a-5d (refreshed that
day), of 048a-5d2 and of 048a-5ef, written beside this one, none of them merged as built at that
commit, and against ruling 86. It names what those orders build by the names their texts give.
Every change from the two drafts is the coordinator's, open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- the two drafts' behaviors are one order, with one start check, one scope, one declaration and one
  Done-when, as ruling 88 has it;
- the undo replaces 048a-5ef's `Poll::Other` branch, the interim ending for another account, so no
  function of its own tests another account (the 048a-5g draft's `polled_account_is_other`); a poll
  that failed or ended unknown counts as no account yet, as 048a-5ef's `poll_check` reads order
  207's text (ruling 86, point 3 (d));
- the undo has a phase of its own, `undo-pending`, so that the sign-in part arm, 048a-5ef's retry
  and DL18's end each tell an undo in flight by the phase alone (the 048a-5g draft kept the phase
  `poll-pending` and tested a member; the 048a-5h draft named "048a-5g's phase" without one);
- the waiting `sign-in-hook.v1` part ends inside `end_sign_in` (048a-5ef), so neither the undo's end
  nor DL18's end ends it on its own; the 048a-5h draft's loop of its own is dropped;
- one function ends an undo, `undo_ended`, called by the sign-out part's end, by a restart's
  recovery and by a cancel, in place of the 048a-5g draft's three copies; recovery extends the test
  that order 048a-4 wrote for setup's undo, in place of the draft's two passes;
- a cancel of the `sign-in.v1` while its undo is in flight reports `SignInUndone=False` as setup's
  cancel does, and a cancel of the sign-out part ends the sign-in as the part's end would; the
  draft left the first a finding and the second open;
- the new case shows a cancel and a restart during the undo, so that neither is left an untested
  choice;
- 048a-5ef's polling case, whose step 32 meets the interim ending, gains the undo there, so that its
  next sign-in still finds the earlier one ended;
- DL18's end tests a waiting sign-in as 048a-5ef's retry finds one, by its phase `retry-pending` and
  the attempt of its output, and lives in `sign_in.rs`, beside `end_sign_in`;
- `WAITING_ON_PERSON` names no phase of 048a-7 (dropped at the dispatch refresh: decision 5 calls
  048a-5ef's `waiting_for_retry`);
- every name, line and count is checked against `main` at `c097ec81`: `set_sign_in_undone_false`,
  `clear_sign_in_undone`, `lifecycle::report_sign_in_undone`, `pending::unanswered_sign_in`,
  `part_answered`, the part function `setup_sign_in_hook` and the case `requests-sign-in-hold` of
  17 steps.

**Its dispatch refresh.** The coordinator refreshed it on 2026-11-19, for its stacked dispatch,
against `main` at `92632057`, where order 207 has merged, order 209 has merged as built (#412), and
the texts of 048a-5d, 048a-5d2 and 048a-5ef have merged refreshed (#408, #410, #411), none of the
three yet merged as built; 048a-5ef's round 1 had started on 048a-5d2's round-1 branch and was not
yet there to read. Every change below is the coordinator's, open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- under the owner's choice on 2026-11-19 of the lever "Pipeline the chain", as the coordinator
  applies it ([ruling 89](../proposals/ruling-89/README.md), point 2, open to the owner's strike),
  this order may start from 048a-5ef's round-1 branch, before 048a-5ef merges, with orders 209,
  048a-5d and 048a-5d2 each merged or on its own round-1 branch below it: the start check accepts
  any of these bases, says how to tell them apart and that the texts on `main` govern where the
  base holds older copies, as 048a-5ef's text does, in place of the earlier dispatch note that
  relaxed one line; and the admin paragraph at the foot lists what the coordinator rebases and
  rereads when 048a-5ef merges (a readiness finding in the coordinator's notes (private): the
  texts written ahead refused or only half allowed the stacked dispatch);
- under the owner's choice on 2026-11-19 of the lever "Less prescriptive orders", as the
  coordinator applies it (the same point), the start check keeps only what the work needs: the
  checks of ruling 88's record and of the Records of 048a-5b, 048a-4 and 048b-relay1 are dropped
  (the Records now only tell the bases apart); F's roles, where 048a-5ef's build put one at
  another step, are found by what assumption (e) says each does and reported, with a stop only
  where one is missing; and the list "At dispatch the coordinator rechecks" becomes "What to
  recheck at dispatch", after the start check, as in 048a-5ef's text;
- in "Done when", the vectors run is folded into the whole engine suite, which runs once, last,
  after the mutations (a readiness finding in the coordinator's notes (private)), and the rebase
  list at the foot leaves it and the `vector_delta.py` check to `tools/gate.py`, which runs both;
  the spike's test count is no longer pinned, F's length is given against the start, the count of
  the name 048a-5gh in `engine/CONFORMANCE.md` becomes a check of each of decision 12's two
  sentences, and the greps that nothing names 048a-5g or 048a-5h no longer ask for a count at the
  start;
- it was checked against 048a-5ef's refreshed text, whole: that order adds
  `waiting_for_retry(v, attempt)` in `sign_in.rs`, the one test of SI's phase and its output's
  attempt that `respond` reads, so decision 5's `waits_on_person` calls it in place of a list of
  phases of its own (`WAITING_ON_PERSON` is dropped), and a later phase in which a sign-in waits on
  the person changes that one function for the retry and for DL18's end alike; assumption (e)
  takes `retried`, `respond`'s new signature and F's last step, a cancel, as that text writes them;
- the readiness review's other findings for this order were weighed: a restart while a sign-in
  waits on the person is now shown by 048a-5ef's relay case, which restarts the engine and refuses
  a retry after it, and this order's shell test drives the timer with a short expiry, so neither
  needs a case here; that an Interface whose session drops before its retry leaves the sign-in
  waiting, as a person who leaves the page does, is said under "What it builds", since this
  order's end at DL18 is what frees the instance then; a reconnect of the command line belongs to
  051b-2 or a later order, not here;
- order 209's merged `recover` was read: after the branch for a `sign-out-hook.v1` part it now
  calls `crate::lifecycle::record_setup_sign_in(t, i)`, which reads only setup's `raised`, so
  decision 4's restart branch and "Change nothing else in `recover`" still hold, and the item is
  no longer rechecked at dispatch; the line numbers it moved are found by the text quoted with
  them.

**What it builds.** Two behaviors. The quotes below write the `_meta` prefix that
`spec/names.yaml` sets as `META/`, and give PND-010 and PND-011 as order 207 writes them.

*The undo.* PND-011's `then`: "During polling after its URL elicitation, the expected account
succeeds with output account and answers the action signed-in; the account that the check before
sign-in-hook.v1 found counts as no account yet; any other account is undone by a sign-out-hook.v1
part, sign-in.v1 ends failed naming both accounts, and the action stays pending with the attempt as
evidence." Its `observable`: "If sign-out does not succeed, the engine reports SignInUndone=False
with its cause and, until that condition clears, dispatches to that instance no request other than
the parts of sign-in.v1, of a setup and of its retirement ... Only a later account-hook.v1 part
within sign-in.v1 or setup that reports the expected account or none clears that condition".
PND-010's `observable`: "no account yet is not failed, and a poll that failed or ended unknown
counts as no account yet, so that only a later check that reports the expected account completes
sign-in.v1." PND-005's `then`: "Evidence names the expected account, the login started by name and
return, device user code where relevant, triggering request and attempts with their results". The
condition, its hold and its clearing are order 048a-4's; this order reaches them from `sign-in.v1`
too.

*The end of the wait.* PND-010's `observable`: "Each retry polls account-hook.v1 to check the
account after the person's sign-in, under PND-011, until an account appears or DL18 ends" and
"DL18 ending without completion ends unknown and no hook runs later". DL18
(`spec/implementation-defined.yaml`): "The wait is finite; on its end the invocation ends unknown
and no later hook runs." IFC-049's `observable`: "A client need never retry; no later hook runs on
its behalf." `sign-in.v1`'s description: "No held request resumes and no later hook runs after
DL18." Until this order nothing ends a `sign-in.v1` that waits on the person: the shell refuses a
late retry as expired, but the invocation stays in progress until a restart, and while it does,
PND-010's conflict refuses every new `sign-in.v1` of that instance, so a person who leaves the
browser page, or whose Interface loses its session before it sends the retry, cannot sign in again
until the engine restarts.

No promise changes: the texts state all of it, and this order builds it. Nothing here reads
`plain`: the cases run on order 048a-3's fixture, whose `signed-server` carries Conduit's keys, and
a plain sign-out hook of a 0.2.0 Agent package ends succeeded on a complete result that is not an
error and unknown otherwise, by IFC-010's table (its contract states an effect), an outcome this
order reads as it reads a keyed one's.

**How DL18's end is built, by the coordinator's decision (a).** The shell issues, keeps and checks
the requestState it relays, and the core holds none (order 048b-relay1); an `invoke` of
`sign-in.v1` is a first request, and a `respond` is a retry. So the end of the wait is the shell's
to notice and the core's to decide. At the expiry of each output it relays, the shell submits one
input naming that output's invocation and attempt (decision 8). The core ends that `sign-in.v1`
unknown only where it still waits on that attempt for the person, and dispatches nothing
(decisions 5 to 7). A later retry is refused by the shell, whose state expired at the same instant,
and a `respond` that reaches the core for the ended invocation dispatches nothing (048a-5ef's
refusal). A restart needs nothing new: the engine's recovery ends every invocation in progress
unknown (DL6, IMP-007), and the shell's timers end with it.

**MC9a's K11, which the undo follows.** [Order 201](201-mc9a-pending-actions-sign-in-registered.md)
registers MC9a, whose property K11 (`experiments/012-model-checking/README.md`) reads, in its parts
(b) to (d): "within a `sign-in.v1`, a polling result naming an account that is neither `E` ignoring
case nor the account that its check before `sign-in-hook.v1` found requires a `sign-out-hook.v1`
part on ordinary completion, followed by a failed ending that names both accounts and answers no
sign-in action of A. No sign-out part is dispatched for `e` or for the initial check's account; (c)
if E incorporates a sign-out result that did not succeed while the invocation is open, it reports
`SignInUndone=False` with its cause before the invocation ends; (d) recovery may interrupt before
dispatch or while the sign-out result is outstanding. The interrupted invocation ends unknown and
answers no sign-in action." Its witnesses 8 (`wrong-account-undone`) and 9 (`undo-failed`) are the
undo's two paths; the new case shows both, and part (d) (decision 10).

**What the orders before it bring, which this order assumes.** Orders 048a-2 to 048a-5c, 048a-6,
048b-relay1, 048b-relay2, 207 and 209 have merged to `main`, each with its Record, and the work of
orders 209, 048a-5d, 048a-5d2 and 048a-5ef must be on the base this order starts from, each merged
or on its round-1 branch ("When this order starts"). It assumes, as their decisions and the
amendments their Records accept state them, and as they are built on your base; where a Record, or
the build on your base, names a function, member, phase or case otherwise, use that name, and say
so:

- **(a) From 048a-2 to 048a-4 (merged).** In `engine/crates/core/src/pending.rs`,
  `unanswered_sign_in(state, instance, time)`, giving the instance's sign-in action with no answer
  at that time, the instance given as an object with its `id`; the stored entry
  `{"action": A, "dispatched": null, "raisedBy": I}` keyed by the action's ID, `A`'s `evidence` a
  list of `evidence-item.v1` items written by the private `fact(value, place, proves)`; and the
  test `sign_in_action_raised_by_setup`. In `sign_in.rs`, `set_sign_in_undone_false(object,
  outcome, hook, time)`, whose message is
  `The sign-out hook ended {outcome} after setup's account check found an account other than the expected one, so that account may still be signed in. Requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable under PND-011 until an account check finds the expected account or none.`,
  with the reasons `SignOutFailed` (for `failed`) and `SignOutUnknown` (otherwise);
  `clear_sign_in_undone(object, time, within)`; `request_is_held_for_sign_in_undone`, which
  `invoke_inner`'s hold block reads; and the test `sign_in_undone_condition_and_hold`. In
  `lifecycle.rs`, `report_sign_in_undone(t, instance, outcome, hook, time)`, called "the undone
  helper" below, the only code that writes the decision `report-sign-in-undone`, called by setup's
  part arm (`sign_in_part_answered`) and by `lifecycle::cancel` where a setup's phase is
  `undo-pending`. In `requests.rs`, `recover`'s branch for a `sign-out-hook.v1` part whose parent
  setup's `setup.phase` is `undo-pending`, which calls the undone helper with `unknown`. The case
  `requests-sign-in-hold` (17 steps), its three files (048a-2's Type and logins; `signed-server`,
  not marked `plain`, implementing the core's `sign-in-hook.v1`, `account-hook.v1` and
  `sign-out-hook.v1`; the Agent `signed`, account `primary-account`), its definitions ROOT,
  `META/x`, `U`, `I`, `P`, "Root asks T with A", "a completion", SUCCESS and HELD(A) (048a-3,
  decision 6), and ACCOUNT(X), SIGN-OUT-FAILED, PART(H) and `K(x)` (048a-4, decision 8).
- **(b) From 048a-5b (merged).** `sign_in::begin`, which sets the `sign-in.v1` invocation's `object`
  to the instance's ID and its member `sign_in`, called SI below, to
  `{"phase": "account-pending", "parts": []}`; `part_answered(t, a, c, i, part, outcome)`, called
  "the part arm" below, which `lifecycle::child_answered` calls for a part whose parent is a
  `sign-in.v1` in progress; `expected_account`, called "the account function" below; `signed_in`;
  `link_action`, which links SI's `raised` to the `invocation-ended` record as `pendingAction`;
  `RETIRED_DURING_CHECK`; and `requests::cancel`, which ends a `sign-in.v1` as an external call,
  through its last branch, leaving its parts in progress (048a-5b's Record, V7).
- **(c) From 048a-5d and 048a-5d2.** The part function `setup_sign_in_hook(t, a, c, parent, object,
  name, promises, login)`, its last parameter `login: Option<&str>`, which names its decision
  `dispatch-sign-out` for `sign-out-hook.v1`, replies
  `{"pending":true,"invocation":<parent>,"hook":<part>}`, and gives `false`, dispatching nothing,
  where no implementation is resolved; `end_sign_in(t, a, c, i, outcome, rule, cause)`, called "the
  end" below, which ends the invocation through `finish` with one diagnostic from
  `explain::outcome_diagnostic`, sets SI's `phase` to `ended` and links SI's `raised`; and the hook
  part's member `hook_input`, which `finish` removes.
- **(d) From orders 207 and 209.** The texts of PND-010 and PND-011 quoted above.
- **(e) From 048a-5ef**, its file `048a5ef-build-sign-in-relay-and-polling.md`, as its build on your
  base has it. `relay_output(t, sign_in, time, c)` in `sign_in.rs` pushes S's input-required output
  with `invocation` S,
  `attempt` S's `attempt` and `expires` DL18's instant, kept with the URL, the login and its `_meta`
  in S's member `sign_in_wait`, whose `attempt` and `expires` it sets; every later output of S
  carries the same attempt and the first output's instant; and it sets SI's `phase` to
  `retry-pending`, the phase in which S waits for the person's retry.
  `waiting_for_retry(v, attempt)`, `pub(crate)` in `sign_in.rs`, is true exactly where SI's
  `phase` is `retry-pending` and `sign_in_wait.attempt` equals `attempt`.
  `requests::respond(t, evidence, invocation, attempt, identity, responses, c)` finds S only where
  it is a `sign-in.v1` in progress at that attempt for which `waiting_for_retry` is true and whose
  `sender` is the instance the identity establishes, and otherwise refuses the retry, pushing
  `refuse-input-response`, answering `-32602` and recording the refusal through `end`; an
  accepted retry reaches `sign_in::retried`, which dispatches the poll through the part function,
  sets SI's `phase` to `poll-pending` and its `poll` to the poll's ID. The part arm's arm for
  `poll-pending`, called "the poll arm" below, for the part whose ID is SI's `poll`, pushes the part
  as `poll`, clears `SignInUndone` where the poll reports the expected account or none, ends S under
  PND-003 where the instance is gone or retired, and then answers by
  `poll_check(outcome, outputs, expected, first) -> Poll`, `first` being SI's `first_account`:
  `Expected` signs in through `signed_in`; `NoneYet` (no account, the first check's account compared
  ignoring ASCII case, or a poll that failed or ended unknown) pushes `ask-sign-in-again` and gives
  the output again through `relay_output`; `Other` takes the interim ending, which pushes one
  decision named `end-sign-in-other-account`, citing `PND-011`, with the facts
  `{"invocation": S, "hook": H}`, and ends S failed under `PND-011` with a cause that ends "is not
  built yet (order 048a-5gh), so sign-in.v1 ends failed and nothing was signed in."; and `NotKnown`
  ends S failed under `PND-011`. A part's answer while SI's phase is one the part arm does not
  handle returns, changing nothing. `end_hook_wait(t, a, c, i, why)`, `pub(crate)`, called first in
  `end_sign_in` and in `signed_in`, ends each `sign-in-hook.v1` part of S in progress that holds
  `hook_input` unknown under PND-010, with the cause
  `The sign-in.v1 invocation it serves ended {why}, so its retry is never sent.`, and pushes it onto
  SI's `parts`, so every ending of S through them ends its waiting hook part first; a cancel's or a
  restart's `finish` does not call it.
  `finish` removes `sign_in_wait.url` and gives a relayed sign-in's final reply its `signInLogin`.
  `reply_context` lists `Respond` beside `Complete`. The portable runner reads the `respond` input.
  The case `requests-sign-in-polling`, called F below, on `requests-sign-in-hold`'s three files, 36
  steps, whose steps are named below by their roles: F-START, its step 10, root asks
  `group.sign-in.v1` with S and URL, capturing `signin1` and `check1`; F-CHECK, its step 11, a
  completion of `$check1` with another account, which dispatches the hook, capturing `login1`;
  F-HOOK, its step 12, a completion of `$login1` with a URL that passes, which gives the output;
  F-RETRY, its step 13, root answers `$signin1` with `accept`, which dispatches the poll, capturing
  `poll1`; F-AGAIN, its step 16, a completion of `$poll2`, captured at its step 15, with the first
  check's account, which gives the output again. Its step 32 completes `$poll5` with
  ACCOUNT(`"third-account"`) and expects ENDED(`signin3`, `login3`, `PND-011`), the interim ending,
  and its steps 33 to 36 start a fourth sign-in and cancel it on its retry. In
  `engine/CONFORMANCE.md`'s DL18 row, the sentence "Until order 048a-5gh, another account found
  while polling ends `sign-in.v1` failed with no sign-out, and nothing ends the invocation at its
  expiry: a sign-in that gets no retry stays in progress until a restart ends it unknown (DL6), and
  a further `sign-in.v1` of the instance reports conflict meanwhile."
- **(f) From 048b-relay1 (merged).** `Input::Respond`; `InputRequired`, whose `expires` is RFC 3339,
  "the end of the URL action's wait (DL18), from which its requestState is refused"; in
  `engine/crates/shell/src/server.rs`, `Handler`'s members `shell`, `executor` and
  `shutdown: CancellationToken`, and `call_tool`'s branch that passes a reply's member
  `inputRequired` to `relay::relay` and on its success returns
  `Ok(CallToolResponse::InputRequired(result))`, `relay` having checked that the member's
  `invocation` is a string, its `attempt` a `u64` and its `expires` RFC 3339; in `relay.rs`,
  `States::take` giving `Expired` for an issued state whose expiry is at or before `now`; and
  `serve`, which submits `{"kind":"recover"}` after the server stops.

Where one of them merged, or was built on your base, otherwise than this says, apply the nearest
sound reading of each decision below to what you find, and say so; but where the start check below
prints otherwise and does not say that a difference may be used and reported, stop.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`c097ec81`, before orders 209, 048a-5d, 048a-5d2 and 048a-5ef, which move them; find each place by
the text quoted with it.

- `recover` (`engine/crates/core/src/requests.rs:2333`) ends every invocation in progress unknown
  under IMP-007 through `finish`, in the order of `state.invocations`, calling no `child_ended`; for
  a `sign-out-hook.v1` part it then tests that its parent setup's `setup.phase` is `undo-pending`
  (line 2361) and calls the undone helper with `unknown` (line 2364). `finish` changes no member
  `sign_in`, so a parent `sign-in.v1` ended earlier in the same loop keeps SI's phase.
- `requests::cancel` (line 2232) sends a `setup` or `retirement` invocation to `lifecycle::cancel`,
  and ends any other through its last branch: it pushes a `cancel` intent and the decision
  `forward-cancellation`, and ends the invocation through `finish`, unknown under OPS-027 unless the
  cancellation is confirmed failed, as its tail expression. It calls no `child_ended`.
- `lifecycle::cancel`'s branch for a setup in `undo-pending` (`lifecycle.rs:1425`) ends its part and
  calls the undone helper with `unknown`.
- `lifecycle::child_answered` (`lifecycle.rs:1201`) returns at once where the parent is not in
  progress, and calls `sign_in::part_answered` for a `sign-in.v1` parent (line 1221).
- `explain::outcome_diagnostic` (`explain.rs:875`) writes
  `{operation}{ for subject} ended {outcome} under {rule}: {cause}`.
- `group.inspect.v1` shows root each pending action whole, its `evidence` included
  (`pending.rs`'s `action_view`).
- `Shell::transition` (`engine/crates/shell/src/lib.rs:362`) adds evidence to the input kinds in its
  `needs_evidence` list (line 377); under `#[cfg(test)]` it takes scripted evidence where a test
  pushed one, and generates it otherwise. `Shell::submit` commits an input, drives its intents and
  waits for its reply. Under `#[cfg(test)]`, `Shell::commit` pushes every transition to
  `shell.trace` while a test has set it to `Some`; `tests.rs`'s `established` sets it while it
  builds the shell and sets it back to `None` before it returns.
- `impl Drop for Pending` (`server.rs:261`) calls `shell.begin` with a cancel inside
  `tokio::task::spawn_blocking`, does not await it, and discards its result.
- The portable runner compares maps as projections and arrays with exact cardinality and order, and
  `record_facts` requires matching records in the stated causal order (`conformance/README.md`,
  "Assertions"); `cancel` and `restore-state`, which reaches `recover`, are portable input kinds.

**When this order starts.** Order 207 must have merged to `main`. The work of orders 209, 048a-5d,
048a-5d2 and 048a-5ef must be on your base, the commit your working tree starts from (`HEAD`), in
one of two ways, either of which serves: each has merged to `main`; or the coordinator dispatched
this order stacked on 048a-5ef's round-1 branch, whose tip holds 048a-5ef's build, committed,
before its read and merge, on top of the builds of 048a-5d2, 048a-5d and 209, each merged or on its
own round-1 branch below it. You tell them apart by the Records, which the admin writes at each
merge: until then the files of orders 209, 048a-5d2 and 048a-5ef end "Not yet run.", and
048a-5d's has no `## Record` heading (the first check's two lines). On any base, 048a-5ef's two
cases are present and pass (item 2 below and the start's runner runs). Report
which of the four have merged on your base, and `HEAD`'s commit. Nothing below reads which it is.
On a round-1 branch, the copies of this order's file and of its predecessors' files in your tree
may be older than the texts that merged to `main`; where they differ, the texts on `main` govern:
read them with `git show origin/main:docs/work-orders/FILE`, `FILE` being the file's name. Before
you change anything, check from the repository root that you find what they give, and stop,
change nothing and report if any check prints otherwise, except where its item below says that you
use what you find and report it:

```bash
grep -c '^Not yet run\.$' docs/work-orders/207-keyed-hooks-and-sign-in-spec-fixes.md docs/work-orders/209-build-keyed-hooks-and-sign-in-fixes.md docs/work-orders/048a5d2-build-sign-in-url-match.md docs/work-orders/048a5ef-build-sign-in-relay-and-polling.md
grep -c '^## Record' docs/work-orders/048a5d-build-sign-in-hook-part.md
ls -d conformance/conventions/requests/sign-in-url-relayed conformance/conventions/requests/sign-in-polling 2>/dev/null | wc -l | tr -d ' '
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'a poll that failed or ended unknown counts as no account yet'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'any other account is undone by a sign-out-hook.v1 part, sign-in.v1 ends failed naming both accounts, and the action stays pending with the attempt as evidence'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'If sign-out does not succeed, the engine reports SignInUndone=False with its cause'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'DL18 ending without completion ends unknown and no hook runs later'
python3 -c "import json; s = json.load(open('conformance/conventions/requests/sign-in-hold/case.yaml'))['steps']; print(len(s), s[6]['input']['request']['tool'], s[7]['input']['request']['tool'], s[9]['input']['request']['tool'], json.dumps(s[9]['capture'], sort_keys=True))"
python3 -c "import json; c = json.load(open('conformance/conventions/requests/sign-in-polling/case.yaml')); s = c['steps']; print(len(s), json.dumps(c['files'], sort_keys=True)); print([s[k]['input'].get('invocation', s[k]['input'].get('request', {}).get('tool')) for k in (9, 10, 11, 12, 14, 15, 30, 31, 32)], [d['ruleId'] for d in s[31]['expect']['reply']['structuredContent']['diagnostics']], json.dumps(s[9]['capture'], sort_keys=True), json.dumps(s[10]['capture']), json.dumps(s[12]['capture']))"
python3 tools/promise_standing.py --check | tail -1
grep -rlE '"sign-in-wait-ended"|"keep-sign-in-attempt"|fn undo_ended|fn end_sign_in_wait_at' engine/crates | wc -l | tr -d ' '
ls -d conformance/conventions/requests/sign-in-other-account-undone 2>/dev/null | wc -l | tr -d ' '
```

They print, in order:

1. four lines and a number: `207-keyed-hooks-and-sign-in-spec-fixes.md:0`, order 207's Record;
   `209-build-keyed-hooks-and-sign-in-fixes.md:0` where order 209 has merged on your base, or `:1`
   on its round-1 branch; `048a5d2-build-sign-in-url-match.md:0` and
   `048a5ef-build-sign-in-relay-and-polling.md:0` where each has merged, or `:1` on its round-1
   branch; and `1` where 048a-5d has merged, or `0` on its round-1 branch; report which; where a
   file is missing, or an order shows as merged while one before it in the chain (209, 048a-5d,
   048a-5d2, 048a-5ef) does not, stop;
2. `2`: 048a-5ef's two cases, on your base either way;
3. to 6. `1` four times: order 207's text of a failed or unknown poll, and the texts of PND-011 and
   PND-010 that this order builds;
7. `17 access.open.v1 to-signed.work.v1 group.inspect.v1 {"signedId": "/reply/structuredContent/id", "signin": "/reply/structuredContent/status/pending/0/id"}`:
   `requests-sign-in-hold`'s length and its steps 7, 8 and 10, the last capturing the sign-in
   action as `$signin`;
8. two lines: `36` and F's three files, each by a path that ends `sign-in-hold/fixture.json`,
   `sign-in-hold/manifest.json` and `sign-in-hold/documents.json`; then
   `['group.sign-in.v1', '$check1', '$login1', '$signin1', '$signin1', '$poll2', '$signin3', '$poll5', 'group.sign-in.v1'] ['PND-011'] {"check1": "/reply/hook", "signin1": "/reply/invocation"} {"login1": "/reply/hook"} {"poll1": "/reply/hook"}`:
   F's roles at its steps 10, 11, 12, 13 and 16, its step 15, its steps 31 to 33, the interim
   ending at its step 32, and the roles' captures. Where 048a-5ef's build put a role, or the
   interim ending, at another step or under another capture name, find each by what assumption (e)
   says it does, use the steps and names you find in decisions 10 and 11 and in "Done when", and
   report them; stop only where one is missing or F's files are others;
9. `Errors: 0`;
10. `0` and `0`: none of this order's work.

Then report the names that assumptions (a) to (f) give as found, and stop only where one is missing.

Then, by reading, report the lines of the poll arm and of its `Poll::Other` branch, the line of
`end_hook_wait`'s call in `end_sign_in`, and `waiting_for_retry`, which `respond` calls to test a
waiting sign-in (where the build tests it otherwise, report how); check that every output of one
invocation carries its `attempt` and the first output's instant, and that F passes:

```bash
(cd engine && cargo run --release -p conduit-conformance -- .. requests-sign-in-polling)
```

It ends `1 cases passed`. Where an output of one invocation may carry a later instant at the same
attempt, stop and report what 048a-5ef does instead: decision 5 would then end a later round early.
Check too that no order from 048a-5b to 048a-5ef appends an evidence item to an existing action:
where a Record, or the build on your base, shows that one does, stop and report.

Still before any change, keep the rows this order changes, and from `engine/`, one at a time, run
these and keep each last line: "Done when" compares with them.

```bash
mkdir -p engine/target && grep -E '^\| (PND-005|PND-010|PND-011|IFC-049) \|' engine/STANDING.md > engine/target/standing-048a5gh-before.txt
```

```bash
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- pending::tests
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- lifecycle
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell --lib -- relay::tests
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
```

Each passes. `requests-` runs 048a-5ef's two cases and the cases of 209, 048a-5d and 048a-5d2 among
the others, and each must pass and none be an expected failure; where one fails, stop and report
it, since on a round-1 branch a failing case of a predecessor's is that order's to fix, not this
order's.

Orders that land beside this one (050n, 051b and others): whichever lands second keeps both changes
and writes `engine/STANDING.md` again with the tool.

**What to recheck at dispatch.** For the coordinator, before this order dispatches, against the
merged texts of 207, 209, 048a-5d, 048a-5d2 and 048a-5ef and against the base it starts from: for
each of 048a-5d, 048a-5d2 and 048a-5ef, its Record where it has merged, or else its round-1 build
and the verifier's and reader's findings on it so far:

- 048a-5ef's Record or round-1 build: every name that assumption (e) gives (the phases
  `retry-pending` and `poll-pending`, SI's `poll` and `first_account`, `sign_in_wait`'s `attempt`
  and `expires`, `waiting_for_retry`, `retried`, `poll_check` and `Poll`, `end_hook_wait` inside
  `end_sign_in` and `signed_in`, `respond`'s signature and its test of a waiting sign-in, the
  interim ending for another account, its decision and its cause), the case
  `requests-sign-in-polling` as built (its length, the step of each role, its capture names, its
  step 32 and its evidence ranges), whether every output of one invocation keeps the first
  output's instant, and DL18's sentence on 048a-5gh in `engine/CONFORMANCE.md`;
- 048a-5d's and 048a-5d2's Records or round-1 builds: `end_sign_in`, the part function's last
  parameter, `hook_input` and the phase `sign-in-pending`;
- any change to `call_tool`'s relay branch, `requests::cancel`'s last branch or `reply_context`
  since `c097ec81` (decisions 4, 6 and 8);
- that no other order since has taken the evidence ranges of decisions 10 and 11 or the names this
  order adds;
- 048b-join's assumption A5 and its decision 5, and 051b-2's B7, against decisions 5 to 8.

**Who decided what.** PND-011, PND-005, PND-010, PND-001, IFC-049, DL18, IMP-007 and OPS-027 decide
that another account found while polling is undone by a `sign-out-hook.v1` part, that `sign-in.v1`
then ends failed naming both accounts and answers no action, that the action stays pending with the
attempt as evidence, what a failed undo reports and holds, what clears it, that recovery ends an
interrupted invocation unknown, that a sign-in whose wait ends without completion ends unknown,
that no hook runs later, and that the expired requestState is refused. MC9a's K11 states the
undo's paths. Ruling 88 decides that 048a-5g and 048a-5h are one order, waiving FR-264's
one-behavior rule for it, and that 048a-7, 048a-8 and 051b-3 come after dogfood. The coordinator's
decision (a), reconciling orders 048a5 and 048b-relay1, puts the requestState and its expiry in the
shell and none in the core. Everything else here is the coordinator's, each open to the owner's
strike ([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-5 that puts these behaviors
here; the changes from the two drafts listed under "Written ahead" and those of "Its dispatch
refresh"; the phase `undo-pending` and SI's member `undo`; dispatching the sign-out afresh through
the part function, as setup's undo is; ending `sign-in.v1` failed whether or not the undo succeeded, as PND-011's `then` says, with one
diagnostic naming PND-011 and both accounts, its texts, and writing the accounts as JSON strings;
raising no action for another account found while polling, since PND-005's `when` does not list it;
the two evidence items of an attempt, their words and values, and appending them only where the
instance holds an unanswered sign-in action; the `SignInUndone` message for an undo within
`sign-in.v1`; setting `SignInUndone=False` where a restart or a cancel ends the undo, which K11's
part (d) does not ask for, as 048a-4 does for setup; ending `sign-in.v1` where a cancel ends its
sign-out part; the endings where the instance is gone or no sign-out is resolved; that the shell
submits one input at each relayed output's expiry rather than the core keeping a clock; the input's
name, members and form; that a timer ends with the server's shutdown; reading "still waits on that
attempt" as decision 5's test, so that a poll or an undo in flight is left to finish; changing no
pending action at DL18's end; citing PND-010 in its diagnostic, and its words; that a step that ends
nothing changes nothing but its one decision; the decisions' names; the unit tests and the shell
test; the new case, its reuse of `requests-sign-in-hold`'s files and of F's steps, its accounts,
IDs and times; the step of F changed and the one inserted; and the sentences in
`engine/CONFORMANCE.md`. Apply them as written and report them under `decisions` as the
coordinator's.

**Read first:**

1. In `spec/`: PND-001, PND-003, PND-004, PND-005, PND-010, PND-011, PND-012 and PND-013
   (`spec/promises/pending.yaml`); IFC-049 (`spec/interface/implementation.yaml`); DL6 and DL18
   (`spec/implementation-defined.yaml`); IMP-007 (`spec/conventions/implicit.yaml`); OPS-027
   (`spec/conventions/operations.yaml`); `sign-in.v1`, `account-hook.v1` and `sign-out-hook.v1`
   (`spec/core/operations.yaml`); `condition.v1` (`spec/core/results.yaml`).
2. [Order 048a4](048a4-build-sign-in-setup-account.md), its decisions 2, 3, 5 and 8 and its Record;
   orders 048a-5b, [048a-5d](048a5d-build-sign-in-hook-part.md) and
   [048a-5ef](048a5ef-build-sign-in-relay-and-polling.md), whole, with their Records where they
   have merged; order 048b-relay1's decisions 1, 2 and 4 (its file
   `048b-relay1-build-input-required-relay.md`).
3. In `experiments/012-model-checking/README.md`, MC9a's property K11, its witnesses 8 and 9, and
   its limits.
4. In `engine/crates/core/src/`: `sign_in.rs`; `requests.rs`'s `finish`, `cancel`, `recover` and
   `respond`; `lifecycle.rs`'s `report_sign_in_undone`, `child_answered` and `cancel`;
   `pending.rs`'s `unanswered_sign_in` and `raise_sign_in`; `lib.rs`'s `Input`, `reply_context` and
   `step_body`. In `engine/crates/shell/src/`: `server.rs`'s `Handler`, `Pending` and `call_tool`;
   `lib.rs`'s `transition`, `commit` and `submit`; `server_tests.rs`.
5. The cases `requests-sign-in-hold`, `requests-sign-in-setup-account` and F.
6. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.5 to 0.6k changed lines of Rust in eight files, about half of them tests, with
three unit tests and one shell test; one new case of 47 steps on `requests-sign-in-hold`'s files;
one step of F changed and one inserted; and two places in `engine/CONFORMANCE.md`.

## The decisions it applies

Below, `Q(x)` is `c.qualified(x)`, the core's qualified reference to the operation `x`; S is the
`sign-in.v1` invocation and `i` its index in `t.state.invocations`.

1. **Another account, found while polling, is undone.** In the poll arm, the branch for
   `Poll::Other` becomes, with `X` the
   account the poll found, a string, read from the part's `retained.outputs`:
   - Call the part function for `sign-out-hook.v1`, with S as its parent, the instance as its
     object, the rules `PND-011`, `ACC-018` and `IMP-002`, and the login `None`: afresh, with no
     idempotency key, the decision `dispatch-sign-out` and the reply
     `{"pending":true,"invocation":S,"hook":H}`, as 048a-4 dispatches setup's undo.
   - Set SI's member `undo` to `{"account": X, "hook": H}`, `H` the part's ID, and SI's `phase` to
     `undo-pending`.
   - Return. S stays in progress, no output is given, nothing is raised, the waiting hook part
     stays as it is, and the Interface's retry keeps waiting.

   Where the part function gives `false` (no implementation of `sign-out-hook.v1` is resolved,
   which CTR-013's checks are expected to leave no way to reach), end S through the end, `failed`,
   under `PND-011`, with the cause
   `While sign-in.v1 polled, the account check found an account that is neither the expected one nor the one its first check found, and no implementation of sign-out-hook.v1 is resolved for the instance, so that account was not signed out and sign-in.v1 signs nothing in.`,
   and report whether any path reaches it.

   Remove 048a-5ef's interim ending for another account: its decision push, its cause and any
   constant or comment that only it uses. No text, comment or decision naming 048a-5g, 048a-5h or
   048a-5gh remains in the core, and no new code names an order. Change nothing else of the poll
   arm or of `poll_check`.

2. **The end of an undo, and the attempt kept as evidence.** In `pending.rs`, add
   `pub(crate) fn keep_sign_in_attempt(state: &mut State, instance: &str, invocation: &Value, result: &str, time: &str) -> Option<Value>`.
   It calls `unanswered_sign_in` for `{"id": instance}` at `time`. Where that gives none, it changes
   nothing and gives `None`: another account found while polling raises no action, since PND-005's
   `when` does not list it. Otherwise it appends to that stored entry's `action.evidence` two items,
   written with `fact`, in this order:
   1. value the `invocation`; place `records`; proves
      `a sign-in.v1 attempt whose account check, while polling, found an account other than the expected one`;
   2. value the `result`; place `records`; proves `what the sign-out hook did with that account`;

   and gives the action's ID. No item names an account. Nothing else in the entry changes: not its
   `id`, `expiry`, `raisedBy` or `dispatched`, and no hold. Appending evidence neither raises nor
   answers the action, so no record names it (PND-012).

   In `sign_in.rs`, add
   `pub(crate) fn undo_ended(t: &mut Transition, instance: &Value, invocation: &Value, hook: &Value, outcome: &str, time: &str) -> Result<()>`,
   `instance` the instance's ID, `invocation` S's ID and `hook` the sign-out part's ID. It:
   - where no object in `t.state.objects` has `instance` as its ID, change nothing, push no decision
     and return `Ok(())`: the instance was removed, its sign-in action was voided by its retirement,
     and no condition has anywhere to go; report whether any path reaches it;
   - otherwise, where `outcome` is not `succeeded`, calls the undone helper with the instance, the
     outcome, the hook, the time and `polling` `true` (decision 4);
   - calls `keep_sign_in_attempt` with the result `signed-out` where `outcome` is `succeeded`,
     `sign-out-failed` where it is `failed`, and `sign-out-unknown` otherwise; and, where it gave an
     ID, pushes one decision named `keep-sign-in-attempt`, citing `PND-005` and `PND-011`, with no
     implementation choice and the facts `{"action": ID, "invocation": V}`, `V` S's ID.

   Decisions 3 and 4 call it; nothing else writes the attempt.

3. **The sign-out part's end ends `sign-in.v1` failed.** In the part arm, beside the poll arm and
   before its test of `account-pending`, add the arm for SI's `phase` `undo-pending`, for the part
   whose `id` equals SI's `undo.hook`; any other part's answer in this phase returns, changing
   nothing, as 048a-5ef's arms do. Let `X` be `undo.account`, `E` the expected account from the
   account function (null where it gives no string; report whether any path reaches that), `H` the
   part's ID and `O` its outcome, a reason from IFC-026's arm included. In this order:
   1. push `{"name": "sign-out", "outcome": O, "invocation": H}` onto SI's `parts`;
   2. where no object has S's `object` as its ID: end S through the end, `failed`, under `PND-003`,
      with the cause
      `The instance was retired while its sign-out part was in progress, so sign-in.v1 signs nothing in and answers no sign-in action.`,
      and return, calling no `undo_ended`; report whether any path reaches it;
   3. call `undo_ended` with S's `object`, S's ID, `H`, `O` and the step's time;
   4. push one decision named `end-sign-in-other-account`, citing `PND-011` and `PND-005`, with no
      implementation choice and the facts `{"invocation": V, "hook": H, "outcome": O}`, `V` S's ID.
      The name is now pushed only here, with these promises and facts: 048a-5ef's interim push of
      it is gone (decision 1);
   5. remove SI's member `undo`;
   6. end S through the end, `failed`, under `PND-011`, which ends S's waiting hook part first
      (048a-5ef), with the cause:
      - where `O` is `succeeded`:
        `While sign-in.v1 polled, the account check found the account {X}, which is neither the expected account {E} nor the account its first check found, so a sign-out-hook.v1 part signed it out; any unanswered sign-in action of the instance stays pending. Send sign-in.v1 again and sign in to {E}.`
      - otherwise:
        `While sign-in.v1 polled, the account check found the account {X}, which is neither the expected account {E} nor the account its first check found, and the sign-out-hook.v1 part that was to sign it out ended {O}, so that account may still be signed in: SignInUndone is False, and requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable until an account check finds the expected account or none.`

      `{X}` and `{E}` are each written as a JSON string with its quotes, as `serde_json::to_string`
      gives it. These texts name both accounts (PND-011): a `sign-in.v1` caller is always an
      Interface (PND-010), never an Agent, so PND-005's rule against naming the account in a held
      request's diagnostic does not reach them. Add no row to `OUTCOME_CAUSES`.

   The ending answers no sign-in action, lifts no hold of PND-005 and, where `O` is `succeeded`,
   changes no `SignInUndone`: PND-011 names only an account check as clearing it. Its final result
   reaches the Interface's waiting retry through `finish`, as a decline's does, holding
   `signInLogin` (048a-5ef) and no `account` output; the end links SI's `raised`, where the first
   check raised the action, to the `invocation-ended` record (PND-012), and the reply carries no
   `pendingAction`.

4. **`SignInUndone` within `sign-in.v1`, and an undo that a restart or a cancel interrupts.**
   `set_sign_in_undone_false` and the undone helper each gain a last parameter, `polling: bool`.
   Where it is false, everything is as 048a-4 wrote it, its message byte for byte. Where it is true,
   the message is
   `The sign-out hook ended {outcome} after sign-in.v1's account check, while polling, found an account other than the expected one, so that account may still be signed in. Requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable under PND-011 until an account check finds the expected account or none.`;
   the reasons, `lastTransitionTime`, `source`, `past_conditions` and the decision
   `report-sign-in-undone` are as 048a-4 wrote them. Every existing call passes `false`, and
   048a-4's test `sign_in_undone_condition_and_hold` changes only by that argument. The helper stays
   the only code that writes the decision's name. PND-011's hold and its clearing do not change:
   048a-4's hold block answers a request to the instance, and the checks of 048a-5b and 048a-5ef
   clear the condition where they find the expected account or none.
   - **A restart.** In `recover`'s branch for a `sign-out-hook.v1` part, beside its test of a
     parent setup in `undo-pending`, add: where the parent's `operation` is `Q("sign-in.v1")`, its
     SI's `phase` is `undo-pending` and its `undo.hook` is the part's ID, call `undo_ended` with the
     parent's `object`, the parent's ID, the part's ID, `unknown` and the evidence's time, in place
     of the setup's call. The parent was ended unknown under IMP-007 earlier in the same loop, as
     `finish` leaves SI's phase, and gains no PND-011 diagnostic; it answers no action (K11, part
     (d)).
   - **A cancel of `sign-in.v1`.** In `requests::cancel`'s last branch, after its `finish`, where
     the invocation cancelled has the `operation` `Q("sign-in.v1")` and its SI's `phase` is
     `undo-pending`: call `undo_ended` with its `object`, its ID, its `undo.hook`, `unknown` and the
     evidence's time; then set SI's `phase` to `ended` and remove `undo`, so that a later restart
     reports nothing again. The sign-out part stays in progress, as 048a-5b leaves a part of a
     cancelled `sign-in.v1`; its later answer reaches `child_answered`, which returns at once, so
     even a sign-out that then succeeds leaves the condition `False` until an account check clears
     it, as after setup's cancel. The waiting hook part stays as 048a-5ef's cancel leaves it.
     `t.reply` stays the cancel's.
   - **A cancel of the sign-out part.** In the same branch, after its `finish`, where the invocation
     cancelled has the `operation` `Q("sign-out-hook.v1")` and its `setup_parent` names a
     `sign-in.v1` in progress whose SI's `phase` is `undo-pending` and whose `undo.hook` is its ID:
     keep `t.reply`, call `crate::lifecycle::child_ended` for it, so that decision 3's arm takes its
     end with the part's outcome, `unknown`, or `failed` where the cancellation was confirmed
     failed, and then set `t.reply` back to the kept value. S's final result still reaches its
     waiting retry through `finish`.

   Change nothing else in `recover` or `cancel`.

5. **Whether a sign-in still waits on the person.** In `sign_in.rs`, add
   `pub(crate) fn waits_on_person(state: &State, invocation: &str, attempt: u64, c: &Context) -> bool`,
   true exactly where `state.invocations` holds an invocation whose `id` is `invocation`, whose
   `state` is `in-progress`, whose `attempt` is `attempt`, whose `operation` is `Q("sign-in.v1")`,
   and for which 048a-5ef's `waiting_for_retry(v, attempt)` is true: as 048a-5ef's `respond` finds a
   waiting sign-in, without its test of the Interface. Write no test of SI's phase of its own, and
   no list of phases. So a poll in flight (`poll-pending`) or an undo in flight (`undo-pending`) is
   not a wait on the person: it ends as 048a-5ef and decision 3 say, and where it asks again, its
   output's own expiry ends it. A later order with a phase of its own in which a sign-in waits on
   the person (048a-7, after dogfood) changes `waiting_for_retry`, so that the retry and DL18's end
   read it alike. Where the build on your base tests a waiting sign-in under another name, call
   that, and report it.

6. **The input.** In `Input` (`lib.rs`), right after `Respond`, add the variant
   `SignInWaitEnded { evidence: Evidence, invocation: String, attempt: u64 }`; its tag, under the
   enum's `rename_all`, is `sign-in-wait-ended`. In `reply_context`, list
   `Input::SignInWaitEnded { evidence, .. }` beside `Input::Recover { evidence }`, giving
   `(evidence, None)`. In `step_body`, its arm calls
   `crate::sign_in::sign_in_wait_ended(&mut transition, &evidence, &invocation, attempt, context)`,
   as `Respond`'s arm calls `respond`.

7. **The core ends the wait.** In `sign_in.rs`, add
   `pub(crate) fn sign_in_wait_ended(t: &mut Transition, evidence: &Evidence, invocation: &str, attempt: u64, c: &Context) -> Result<()>`:
   - Where `waits_on_person(&t.state, invocation, attempt, c)` is false: push one decision named
     `sign-in-wait-ended`, citing `PND-010` and `IFC-049`, with the implementation choice `DL18`,
     the facts `{"invocation": I, "attempt": N, "ended": false}` and no affected object; set
     `t.reply` to `{"waitEnded": false}`; change nothing else and return. An invocation that ended,
     that moved to another attempt, that polls or undoes, or that does not exist is left as it is.
   - Otherwise, with an `Allocation` on `evidence` with cursor 0, as `recover` makes one: end S
     through the end, `unknown`, under `PND-010`, which ends its waiting hook part first (048a-5ef),
     with the cause
     `No retry completed the sign-in within its URL action's wait (DL18), so it ended without signing in and no hook runs for it later; send sign-in.v1 again to sign in.`;
     then push the decision `sign-in-wait-ended` as above with `"ended": true`, and leave `t.reply`
     as the end set it, S's ended reply.
   - Push no external intent in either branch: an `InputRequiredResult` left no call outstanding,
     and no hook runs later. Change no pending action, no `sign_in_holds` entry and no condition:
     the instance's sign-in action, where one was raised, stays unanswered, and its hold stays
     (PND-005: the hold is the instance's). No attempt is appended to the action's evidence:
     decision 2's items describe another account found while polling, not this ending. Name no
     promise ID other than PND-010 and IFC-049 in this function, in code or comment.

8. **The shell submits it at each output's expiry.** In `engine/crates/shell/src/server.rs`, add
   `pub(crate) fn end_sign_in_wait_at(shell: Arc<Shell>, executor: Arc<ServingExecutor>, shutdown: CancellationToken, invocation: String, attempt: u64, expires: DateTime<Utc>) -> tokio::task::JoinHandle<()>`,
   adding `DateTime` to the `chrono` import, which spawns with `tokio::spawn` one task that:
   - computes the wait as `(expires - Utc::now()).to_std()`, or zero where that fails (an expiry
     already passed);
   - waits in `tokio::select!` for `shutdown.cancelled()`, returning at once without submitting, or
     `tokio::time::sleep(wait)`;
   - then submits `json!({"kind":"sign-in-wait-ended","invocation":invocation,"attempt":attempt})`
     through `shell.submit(..., executor.as_ref())` inside `tokio::task::spawn_blocking`, awaits it
     and discards its result, as `Pending`'s `Drop` discards its cancel's.

   In `call_tool`, in relay1's branch, where `relay` succeeded and before
   `Ok(CallToolResponse::InputRequired(result))` is returned, read `invocation` (a string),
   `attempt` (a `u64`) and `expires` (parsed with `DateTime::parse_from_rfc3339`, then
   `.with_timezone(&Utc)`) from the same `inputRequired` member that was passed to `relay`; where any
   cannot be read, return `internal("invalid core input request")` (relay has checked them, so no
   path should reach it). Then call `end_sign_in_wait_at` with `self.shell.clone()`,
   `self.executor.clone()`, `self.shutdown.clone()` and those three values, and drop the handle: one
   call per relayed output, so that each output's expiry submits one input. In `Shell::transition`
   (`shell/src/lib.rs`), add `"sign-in-wait-ended"` to `needs_evidence`, right after `"respond"`.
   Change nothing in `server/relay.rs`: a retry after the expiry is refused there as it is today.
   Name no promise ID in the shell's new code or comments.

9. **Three unit tests and a shell test pin the rules.** Write the values as `json!` values; members
   a test does not need may be left out. Let T be `2026-11-12T12:00:00Z`.
   - In `sign_in.rs`'s tests, `sign_in_undone_while_polling`. On objects built by hand as 048a-4's
     test `sign_in_undone_condition_and_hold` builds them, the Agent `s1`, whose
     `status.conditions` holds a `Running` condition, and the Agent `s2`: set false on `s1` at T
     for the part `h1` that ended `failed`, with `polling` true, gives the second condition whole,
     with reason `SignOutFailed`, decision 4's message with `failed`, `lastTransitionTime` T and
     source `urn:uuid:h1`, and leaves `Running` unchanged, and `request_is_held_for_sign_in_undone`
     is true for `s1`; set false on `s2` at T for the part `h2` that ended `unknown`, with `polling`
     false, gives reason `SignOutUnknown` and 048a-4's message with `unknown`, byte for byte; and
     `undo_ended`, on `crate::explain::tests::transition` of a state whose objects hold no `absent`,
     for the instance `"absent"`, the invocation `"v1"`, the hook `"h1"`, `failed` and T, gives
     `Ok(())` and leaves the transition's state, records and decisions as they were. Assert with
     `assert_eq!`.
   - In `pending.rs`'s tests, `sign_in_attempt_kept_as_evidence`, with the objects of 048a-4's test
     `sign_in_action_raised_by_setup` (the Agent `s1` with the account `primary-account` at
     `/account`): 048a-4's raise at T for a setup `u1` with the trigger `setup-other-account` gives
     an action; `keep_sign_in_attempt` at `2026-11-12T12:00:01Z` for `"s1"`, the invocation `"v1"`
     and `signed-out` gives that action's ID and leaves the action whole as the raise gave it, but
     for decision 2's two items appended to its `evidence`, six in all, with its `expiry`
     `2026-11-19T12:00:00Z` and `raisedBy` `u1` unchanged; at `2026-11-12T12:00:02Z` for `"v2"` and
     `sign-out-failed` it gives the same ID and eight items, the last two naming `v2` and
     `sign-out-failed`; for `"s2"`, which holds no action, it gives `None` and changes no state; and
     at the action's expiry, `2026-11-19T12:00:00Z`, for `"s1"`, `"v3"` and `sign-out-unknown`, it
     gives `None` and the action still holds eight items.
   - In `sign_in.rs`'s tests, `sign_in_wait_ends_unknown_at_its_expiry`. Let T0 be
     `2026-11-18T12:00:00Z`. From `crate::explain::tests::fixture()`, push by hand to
     `state.invocations` the invocations below, each with the members `finish`, `record` and `end`
     read (`id`, `attempt`, `state`, `operation`, `target`, `caller`, `sender`, `conduit`,
     `serving`, `contract`), copied from the invocation that `requests.rs`'s test helper
     `call_fixture` builds, with `target` and `object` an object of the fixture; add any other member
     they need and say which:
     - `w1`, `sign-in.v1`, `in-progress`, attempt 1, SI `{"phase": "retry-pending", "parts": []}`
       and `sign_in_wait` `{"attempt": 1, "expires": "2026-11-18T12:10:00Z"}`;
     - `h1`, `sign-in-hook.v1`, `in-progress`, attempt 1, `setup_parent` `w1`, and `hook_input`
       `{"key": "sign-in", "request_state": "opaque-implementation-state"}`;
     - `w2`, `sign-in.v1`, `in-progress`, attempt 1, SI at `poll-pending` with `poll` `a2`, the same
       `sign_in_wait`, and `a2`, `account-hook.v1`, `in-progress`, `setup_parent` `w2` (a poll in
       flight);
     - `w3`, `sign-in.v1`, state `failed`, attempt 1, SI at `retry-pending`, the same
       `sign_in_wait`;
     - `w4`, `sign-in.v1`, `in-progress`, attempt 1, SI at `undo-pending` with `undo`
       `{"account": "third-account", "hook": "k4"}`, the same `sign_in_wait`, and `k4`,
       `sign-out-hook.v1`, `in-progress`, `setup_parent` `w4` (an undo in flight).

     Add any member of SI or of `sign_in_wait` that 048a-5ef's Record, or its build on your base,
     needs, and say which. Then seed one unanswered sign-in action for `w1`'s target in
     `state.pending_actions`, in 048a-2's stored form, keyed by its own ID, with `raisedBy` an
     invocation ID of your choice and an `expiry` after T0, and a `sign_in_holds` entry for that
     target, `{"since": T0}`; keep a copy of both.

     Step each input with `crate::step`, read with `serde_json::from_value` from
     `{"kind":"sign-in-wait-ended","evidence":E,"invocation":I,"attempt":N}`, `E` being
     `{"time": T0, "ids": D}` with `D` the strings `format!("00000000-0000-4000-8048-b0000000{n:04}")`
     for `n` in `1..50`. Assert with `assert_eq!`, in this order:
     1. `w1`, attempt 2: the reply is `{"waitEnded": false}`; the state equals the one before;
        `records` and `external_intents` are empty; the one decision's `action` is
        `sign-in-wait-ended` and its `facts` `{"invocation":"w1","attempt":2,"ended":false}`.
     2. `w2`, attempt 1: as step 1, for `w2`: a poll in flight is not ended.
     3. `w4`, attempt 1: as step 1, for `w4`: an undo in flight is not ended.
     4. `w3`, attempt 1, and `absent`, attempt 1: each as step 1.
     5. `w1`, attempt 1: `w1`'s and `h1`'s `state` are `unknown`; `w1`'s `cause` holds one diagnostic
        whose `ruleId` is `PND-010` and whose text ends `send sign-in.v1 again to sign in.`, and
        `h1`'s one whose `ruleId` is `PND-010` (048a-5ef's `end_hook_wait`); `h1` no longer holds
        `hook_input`; `w2`, `a2`, `w3`, `w4` and `k4` are unchanged; `external_intents` is empty;
        `records` holds an `invocation-ended` for `h1` before one for `w1`; `pending_actions` and
        `sign_in_holds` equal the seeded values; the reply's `_meta` `META/outcome` is `unknown`;
        and the last decision is `sign-in-wait-ended` with `"ended": true`.
     6. `w1`, attempt 1, again on the state step 5 gave: as step 1, for attempt 1.
     7. On that state, a `respond` input for `w1`, attempt 1, with `identity`
        `{"kind":"instance","id":"root"}`, `responses` `{"url":{"action":"accept"}}` and `meta` `{}`,
        and the evidence `E` with `n` in `50..100`. The step gives `Ok`; `external_intents` is empty;
        `state.invocations` has as many entries as before; `w1` is still `unknown`; and the one
        decision is 048a-5ef's refusal, `refuse-input-response`. No hook runs later. If the step
        dispatches a part or gives `Err`, stop and report it.
   - In `server_tests.rs`, `sign_in_wait_end_is_submitted_at_expiry`, a `#[tokio::test]`. Make a
     temporary directory with `temporary()`, a shell `Arc::new(established(temp.path()))`, then
     turn its trace on with `*shell.trace.lock().unwrap() = Some(Vec::new());` (established() turns
     it off before it returns), and an executor
     `Arc::new(ServingExecutor::new(Arc::new(crate::MissingExecutor)))`.
     Let `count` be the number of transitions in `shell.trace` with a decision whose `action` is
     `sign-in-wait-ended`.
     1. With a fresh `CancellationToken`, take `started = Instant::now()`, then call
        `server::end_sign_in_wait_at` for the invocation `absent-invocation`, attempt 1, and
        `Utc::now()` plus 300 milliseconds. Right after the call, `count` is 0.
     2. Await the handle within `tokio::time::timeout` of 10 seconds: it ends. `started.elapsed()` is
        at least 250 milliseconds, and `count` is 1. That transition's decision has the facts
        `{"invocation":"absent-invocation","attempt":1,"ended":false}`, and its `records` and
        `external_intents` are empty.
     3. With another token, call it for the same invocation and `Utc::now()` plus 10 seconds, then
        cancel the token. The handle ends within a `timeout` of 5 seconds, and `count` is still 1.

10. **One portable case, on `requests-sign-in-hold`'s files.** Add the directory
    `conformance/conventions/requests/sign-in-other-account-undone/` with one file, `case.yaml`: ID
    `requests-sign-in-other-account-undone`, `layers: [convention, interface]`,
    `promises: [PND-011, PND-005]`, title
    `Undo another account found while sign-in.v1 polls: the sign-in ends failed naming both accounts, its action keeps the attempt as evidence, and a failed, cancelled or interrupted undo holds the instance under SignInUndone until an account check clears it`,
    `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
    `{"fixture": "../sign-in-hold/fixture.json", "manifest": "../sign-in-hold/manifest.json", "documents": "../sign-in-hold/documents.json"}`,
    unchanged. Write it as JSON with two-space indentation and a final newline, its members in the
    order of `requests-sign-in-hold`. It must meet 046a's portable-case rule as it stands.

    Definitions as assumption (a) gives them; where F writes a `sign-in.v1` part's dispatch
    otherwise than PART(H), write PART(H) as F does, and say so. A new step's evidence is its time,
    `2026-11-12T12:00:00Z`, and `ids` the 100 strings `00000000-0000-4000-8000-` followed by the
    twelve-digit, zero-padded decimal of 570000 + 100n + k, for k = 0 to 99, `revisions` the 16
    strings `undo-` followed by the decimal of n, a `-` and the decimal of j, and `suffixes` the 16
    strings `g` followed by the four-digit, zero-padded decimal of 100n + j, for j = 0 to 15, where n
    is the step's number in this case; a copied step of `requests-sign-in-hold` keeps its own
    evidence.

    A step "from F-R" is F's step of role R, its `input` and `expect` whole, with this case's
    evidence for its number, these changes and no other: each `capture` name `x` of F's is written
    `x_N`, N this case's step number; each `$x` it uses, where F captured `x` in a step of one of the
    five roles, is written `$x_M`, M the number of the latest earlier step of this case that captured
    F's `x`; where a step below says "for CHECK(N)" or "for POLL(N)", the account-check part that F's
    step completes is replaced by that one, whatever name F used (F-AGAIN's `$poll2` among them);
    any other `$x` it uses names what this case captured for the same thing, and where nothing
    matches, stop and report; where its `expect` holds `META/unanswered`, the value is 1; and where
    the step is a completion of an account check, its result is ACCOUNT of the account given here.
    SIGNIN(N) is `$signin1_N`, the `sign-in.v1` invocation that step N captures from F-START, and
    CHECK(N) `$check1_N`; POLL(N) is `$poll1_N`, the poll that step N, from F-RETRY, captures. EV is
    the action's evidence before step 11 as four items `{}`, one for each item of 048a-2's raise.
    ITEMS(V, R) is the two items
    `{"kind": "fact", "value": V, "where": "records", "proves": "a sign-in.v1 attempt whose account check, while polling, found an account other than the expected one"}`,
    `{"kind": "fact", "value": R, "where": "records", "proves": "what the sign-out hook did with that account"}`.
    More recur:
    - UNDO(N): a completion for POLL(N), ACCOUNT(`"third-account"`), whose `expect` is
      `{"reply": {"pending": true}}` with PART(`sign-out-hook.v1`), and whose `capture` is
      `{"signoutM": "/reply/hook"}`, M this step's number: neither the expected account nor the
      first check's;
    - INSPECT is root asking `group.inspect.v1` with `{"object": {"kind": "Agent", "name": "signed"}}`;
    - SEEN(C, L): INSPECT's `expect`
      `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"conditions": C, "pending": [{"id": "$signin", "kind": "sign-in", "evidence": L}]}}}}`;
    - UNDONE(R): the list `[{"type": "Running"}, {"type": "SignInUndone", "status": "False", "reason": R}]`;
    - FAILED(V): `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/invocation": V, "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "PND-011", "level": "error"}]}}, "external_intents": [], "absent": ["/reply/structuredContent/account", P]}`;
    - HELD11: `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable", "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "PND-011", "level": "error"}]}}, "external_intents": [], "absent": [I, P]}`;
    - HELDBOTH(A): `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable", "META/pendingAction": [A], "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "PND-005", "level": "error"}, {"ruleId": "PND-011", "level": "error"}]}}, "external_intents": [], "absent": [I]}`.

    Its steps:
    1. to 10. `requests-sign-in-hold`'s steps 1 to 10, `input`, `expect` and `capture`, whole, each
       with its evidence: the setup of `signed`, its account check, the lapse that raises the action
       and holds the instance, and the inspection that captures it as `$signin`.
    11. From F-START: the first sign-in.
    12. From F-CHECK for CHECK(11), ACCOUNT(`"first-account"`): the first check finds another
        account; no second action, since `$signin` is unanswered.
    13. From F-HOOK.
    14. From F-RETRY: the person's retry polls.
    15. UNDO(14).
    16. A completion for `$signout15`, SUCCESS. `expect` FAILED(SIGNIN(11)): the sign-in ends failed
        naming both accounts, and answers nothing.
    17. INSPECT. `expect` SEEN(`[{"type": "Running"}]`, EV followed by ITEMS(SIGNIN(11),
        `"signed-out"`)): the action stays pending with the attempt as evidence, and a successful
        undo sets no `SignInUndone`.
    18. Root asks `to-signed.work.v1` with `{}`. `expect` HELD(`"$signin"`): still held under
        PND-005.
    19. From F-START: the second sign-in, which no conflict refuses, since the first has ended.
    20. From F-CHECK for CHECK(19), ACCOUNT(`"first-account"`).
    21. From F-HOOK.
    22. From F-RETRY.
    23. From F-AGAIN for POLL(22), ACCOUNT(`"FIRST-ACCOUNT"`): the first check's account, compared
        ignoring ASCII case, counts as no account yet, and no sign-out part is dispatched.
    24. From F-RETRY, answering step 23's output; POLL(24) is the poll it captures.
    25. UNDO(24).
    26. A completion for `$signout25`, SIGN-OUT-FAILED. `expect` FAILED(SIGNIN(19)): the undo did not
        succeed, and the sign-in still ends failed.
    27. INSPECT. `expect` SEEN(UNDONE(`"SignOutFailed"`), EV followed by ITEMS(SIGNIN(11),
        `"signed-out"`) and ITEMS(SIGNIN(19), `"sign-out-failed"`)).
    28. Root asks `to-signed.ping.v1` with `{}`. `expect` HELD11, with
        `"record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "unavailable", "ruleId": "PND-011"}}]`:
        held under PND-011 alone, naming no action.
    29. Step 18's request. `expect` HELDBOTH(`"$signin"`), with
        `"record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "unavailable", "ruleId": "PND-005", "pendingAction": ["$signin"]}}]`:
        MC9a's H05.
    30. to 34. From F-START; from F-CHECK for CHECK(30), ACCOUNT(`"first-account"`); from F-HOOK;
        from F-RETRY; and UNDO(33), which captures `$signout34`. The sign-in's parts are not held.
    35. `{"kind": "cancel", "evidence": E, "invocation": SIGNIN(30)}`, `E` this step's evidence.
        `expect`
        `{"reply": {"isError": true, "_meta": {"META/outcome": "unknown", "META/invocation": SIGNIN(30)}, "structuredContent": {"diagnostics": [{"ruleId": "OPS-027", "level": "error"}]}}}`:
        the Interface went away while the undo was in flight.
    36. INSPECT. `expect` SEEN(UNDONE(`"SignOutUnknown"`), EV followed by ITEMS(SIGNIN(11),
        `"signed-out"`), ITEMS(SIGNIN(19), `"sign-out-failed"`) and ITEMS(SIGNIN(30),
        `"sign-out-unknown"`)): the cancelled undo may not have run.
    37. to 41. As steps 30 to 34, each step's numbers 7 higher: UNDO(40) captures `$signout41`.
    42. `{"kind": "restore-state", "evidence": E}`, `E` this step's evidence. `expect`
        `{"reply": {"restored": true}, "external_intents": []}`: a restart while the undo is in
        flight (K11, part (d)).
    43. INSPECT. `expect` SEEN(UNDONE(`"SignOutUnknown"`), step 36's list followed by
        ITEMS(SIGNIN(37), `"sign-out-unknown"`)).
    44. From F-START: the fifth sign-in.
    45. A completion for CHECK(44), ACCOUNT(`"PRIMARY-ACCOUNT"`). `expect`
        `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/invocation": SIGNIN(44)}}, "absent": [U]}`:
        048a-5b's signed-in answer, compared ignoring ASCII case.
    46. INSPECT. `expect`
        `{"reply": {"isError": false, "structuredContent": {"status": {"conditions": [{"type": "Running"}, {"type": "SignInUndone", "status": "True"}]}}}, "absent": ["/reply/structuredContent/status/pending", U]}`:
        the account check within `sign-in.v1` cleared the condition.
    47. Step 28's request. `expect` `{"reply": {"pending": true}}`: PND-011's hold is lifted.

    The route is new and has not been run, since orders 209 to 048a-5ef are not yet built. Before
    running it, check that no ID repeats across the case's steps' `ids`, the copied steps' included.
    If 046a's runner check refuses the case, if a step taken from F or copied from
    `requests-sign-in-hold` gives a reply other than that case states, if the inspection's
    `status.pending` shows no `evidence`, or if a step's reply differs from what it states, stop and
    report the error: do not change the expectations to pass it, and do not change the inputs beyond
    what this decision says. No expectation is generated from the engine's output. The coverage
    index entry holds `id`, `case` (`conventions/requests/sign-in-other-account-undone/case.yaml`),
    `promises` and, where its neighbours in that piece hold it, `safety_rows: []`, in the members and
    order its neighbours use. Write it in a `cases` list in `conformance/manifest.yaml`, then, from
    `engine/`, run `cargo run --release -p conduit-conformance --example pieces` with no flag, which
    moves it into `conformance/conventions/requests/index.yaml` in ID order and leaves
    `manifest.yaml` byte for byte as it was.

11. **F gains the undo.** In `conformance/conventions/requests/sign-in-polling/case.yaml`, its step
    32 (index 31), the completion of `$poll5` with ACCOUNT(`"third-account"`), keeps its `input` and
    evidence; its `expect` becomes `{"reply": {"pending": true}}` with PART(`sign-out-hook.v1`), and
    it gains `capture` `{"signout5": "/reply/hook"}`. A new step 33 follows it: a completion for
    `$signout5`, SUCCESS, whose `expect` is step 32's earlier `expect`, ENDED(`signin3`, `login3`,
    `PND-011`), whole: the hook part, then the sign-in, ended in that order, the sign-in failed
    under PND-011. Its evidence is: time `2026-11-12T12:01:00Z`, `ids` the 100 strings
    `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of 580000 + 100m +
    k, `revisions` the 16 strings `undo-poll-` followed by the decimal of m, a `-` and the decimal of
    j, and `suffixes` the 16 strings `u` followed by the four-digit, zero-padded decimal of
    100m + j, `m` being 33. Every later step moves one place, unchanged; the case then has 37 steps.
    In its `title`, replace `another account ends it for now` with
    `another account is undone before it ends failed`. Nothing else in the case or its directory
    changes. Without the new step, F's next `sign-in.v1` would meet PND-010's conflict, since the
    third would still wait on its undo. Before running it, check again that no ID repeats across its
    steps. It must then pass; if a later step gives another reply than the case states, stop and
    report it. No other case completes a poll with another account; where you find one, stop and
    report it.

12. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each place by its text read across
    line breaks, as the orders before this one leave it.
    - In DL18's row, the sentence that begins "Until order 048a-5gh, another account found while
      polling ends `sign-in.v1` failed with no sign-out" and ends "and a further `sign-in.v1` of the
      instance reports conflict meanwhile." becomes, in the same line:

      ```text
      Another account found while polling is undone by a `sign-out-hook.v1` part before `sign-in.v1` ends failed (row DL19). A `sign-in.v1` that no retry completes by its output's expiry ends unknown under PND-010 (order 048a-5gh): at each relayed output's expiry the shell submits one input naming the invocation and attempt, and where the invocation still waits on that attempt for the person, the core ends its waiting `sign-in-hook.v1` part and then the invocation unknown, sends nothing and runs no hook for it later; its sign-in action and hold stay, and a retry after it is refused by the shell. A poll or an undo in flight at that instant is left to finish, and an output that a poll gives after it expires at once. A restart ends a waiting sign-in unknown by the engine's recovery (DL6, IMP-007), and the shell's timers end with the engine.
      ```

    - In DL19's row in "Choices still required", right before the sentence "No other kind is held,
      so no other duration is chosen.", and so after the sentences on setup's account check that
      orders 048a-4, 048a-5b and 209 left there, add, in the same line:

      ```text
      While `sign-in.v1` polls (PND-011, order 048a-5gh), an account check that finds an account other than the expected one and other than the one its first check found is undone by a `sign-out-hook.v1` part, and `sign-in.v1` ends failed with a diagnostic naming both accounts; the instance's unanswered sign-in action stays pending and gains the attempt and its result as evidence, and no action is raised where none is unanswered. Where that part does not succeed, or a cancel of the `sign-in.v1` or a restart ends the undo first, `SignInUndone` is `False` as after setup's undo, and the instance is held under PND-011 until an account check finds the expected account or none.
      ```

    - Where any other sentence of the file, or a clause of one, says that another account found
      while polling is not yet undone or that a sign-in's end at DL18 is not built, or names
      048a-5g, 048a-5h or 048a-5gh as not built, change it to say what this order builds, naming no
      order, and report its words before and after.

    Where 048a-5ef's build, or its merge, words DL18's sentence otherwise, find it by its sense (the
    interim ending for another account and the sign-in that no ending reaches before a restart),
    replace it as above, and report it before and after, whole; stop only where it is missing.
    Nothing else in the file changes. After the other decisions, run
    `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
    `promises`. Report the rows of PND-005, PND-010, PND-011 and IFC-049 and the totals; the totals
    change only by what those rows change.

13. **The declaration.** `docs/work-orders/048a5gh-declared.yaml` holds five shapes: one `added`
    for `requests-sign-in-other-account-undone`; one each of `input`, `expectation` and `flip`
    under `/steps/*` for `requests-sign-in-polling`, since an inserted step moves every later step's
    index; and one `metadata` at `/title` for it. The coordinator wrote it with this order; check
    that it matches and change it only to make it so.

14. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
    record changes. No fixture changes, and no case but the new one and F. No portable input kind
    is added: `sign-in-wait-ended` is the shell's, as `expire-outputs` is. `relay.rs`, `runner.rs`,
    the requestState checks and their texts do not change, and DL18's wait stays as 048a-5ef chose
    it. The returns (048a-7, 048a-8), the CLI (051b) and the packages (050) are not touched. Report
    as findings, for the coordinator:
    - that `sign-in.v1` ends failed where its undo ended unknown, as PND-011's `then` says, though
      the sign-out may have had its effect;
    - that no action is raised where another account is found while polling and none is
      unanswered, so its evidence is kept nowhere, and whether PND-005's `when` should list it;
    - that no evidence item names the account found, and no promise says whether one should;
    - that the result carries no `pendingAction` for the action that stays pending;
    - that a cancel of the sign-out part, decision 4's third path, is tested by no case here;
    - that a cancelled undo leaves `SignInUndone` `False` even where the sign-out later succeeds,
      until an account check clears it, and leaves the waiting hook part in progress, as 048a-5ef's
      cancel does;
    - that a poll or an undo in flight at DL18 is left to finish, so a sign-out part that never
      answers keeps `sign-in.v1` in progress past DL18 until a cancel or a restart;
    - what happens where the instance is retired while its sign-out part is in progress, and
      whether decision 1 should dispatch no undo while a `retire.v1` of the instance is in
      progress;
    - that a retry after the expiry is refused as `Expired` only until relay1's sweep forgets the
      state, and then as `NotIssued`, whose text names an altered state or a restart, not an expiry;
    - that the shell discards the timer's submit result, so a failed submit leaves the invocation in
      progress until the next restart's recovery, and no record says so;
    - that no portable case covers DL18's end, since its input is not a portable one, and 048b-join
      only checks it;
    - that the ended sign-in's reply at DL18 reaches no Interface, since the Interface's call
      already returned the input-required result;
    - that a sign-in ended at DL18 leaves no attempt in the action's evidence, unlike an undone one;
    - that the core ends a waiting sign-in on the shell's input without checking its time against
      the kept expiry;
    - and any place where PND-005, PND-010, PND-011, IFC-049 or DL18 left you a choice this order
      does not settle.

## Scope

Change only:

- `engine/crates/core/src/sign_in.rs`: decisions 1, 3, 5 and 7, `undo_ended` and set false's
  parameter (decisions 2 and 4), 048a-4's test's call, and the tests of decision 9;
- `engine/crates/core/src/pending.rs`: `keep_sign_in_attempt` and its test (decisions 2 and 9);
- `engine/crates/core/src/lifecycle.rs`: the undone helper's parameter and its callers (decision 4);
- `engine/crates/core/src/requests.rs`: `recover`, `cancel` and the undone helper's callers
  (decision 4);
- `engine/crates/core/src/lib.rs`: decision 6's variant, its `reply_context` entry and its arm;
- `engine/crates/shell/src/server.rs`: decision 8's function and its call in `call_tool`;
- `engine/crates/shell/src/lib.rs`: `needs_evidence` in `transition` (decision 8);
- `engine/crates/shell/src/server_tests.rs`: one new test (decision 9);
- `engine/CONFORMANCE.md` (decision 12) and `engine/STANDING.md`, written only by the tool;
- `conformance/conventions/requests/index.yaml`, the new directory
  `conformance/conventions/requests/sign-in-other-account-undone/`, and
  `conformance/conventions/requests/sign-in-polling/case.yaml` (decision 11).

Where 048a-5ef put the poll arm or `end_sign_in` in another file, change it there, and only there,
and say so. `docs/work-orders/048a5gh-declared.yaml` comes with this order; change it only as
decision 13 says. `conformance/manifest.yaml` holds the new entry only until the pieces tool moves
it, and ends unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`,
`engine/crates/shell/src/server/relay.rs`, `runner.rs`, `engine/README.md`, the experiments, the
case schema, `conformance/internal-cases.yaml`, `conformance/expected-failures.yaml`, any other case
or fixture, `requests-sign-in-hold`'s directory, `docs/implementation/` or `docs/proposals/`:
report any defect as a finding. `experiments/011-short-form-spike/` changes only if its tests need
it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- pending::tests
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- lifecycle
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell --lib -- sign_in_wait_end_is_submitted_at_expiry
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell --lib -- relay::tests
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-other-account-undone
cargo run --release -p conduit-conformance -- .. requests-sign-in-polling
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-account
cargo run --release -p conduit-conformance -- .. requests-sign-in-hold
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell first_start_presents_final_lists
```

The third reports three more tests passed than the start's run, 0 failed, the new ones being
`sign_in_undone_while_polling`, `sign_in_wait_ends_unknown_at_its_expiry` and
`sign_in_attempt_kept_as_evidence` (cargo's filter `sign_in` is a substring of each test path); the
fourth one more, `sign_in_attempt_kept_as_evidence`; report both counts. The fifth and the seventh
pass as at the start; report their counts. The sixth reports 1 passed. The pieces tool prints
`Nothing to move.` The next four runner commands each end `1 cases passed`; `requests-sign-in-`
and `requests-` each give one more case than at the start, every new one passing, with the start's
expected failures, each still failing; report each count. The shell test passes. If any other case,
or any shell, CLI or experiment test, fails because another account found while polling is now
undone or a waiting sign-in now ends at DL18, stop and report it: do not change it. The threads are
4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that each behavior is what its check tests. From `engine/`, first keep copies:

```bash
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
cp crates/core/src/requests.rs target/requests.rs.fixed
```

Then make each change below by hand, alone, run its command, restore its file with `cp` from its
copy, and run the command again. Each runner command before the restore must print
`FAIL 0 cases passed; 1 failed`, failing at the step named, and the test command must report 1
failed, failing where named; after the restore each runner command ends `1 cases passed` and the
test command reports 1 passed. Restore with `cp`, not `mv`: a moved file keeps its older
modification time, so cargo would not rebuild the crate. Git ignores `target/`. Report the error
each failing run printed.

| Change, by hand | Command | Must fail at |
| --- | --- | --- |
| In `sign_in.rs`'s `poll_check`, make the arm that gives `Poll::Other` give `Poll::NoneYet`, and nothing else | `cargo run --release -p conduit-conformance -- .. requests-sign-in-other-account-undone` | step 15, an input request where the sign-out part's dispatch is expected |
| In `requests.rs`'s `cancel`, delete decision 4's handling of a cancelled `sign-in.v1` in `undo-pending`, and nothing else | the same | step 36, the reason `SignOutFailed` and no attempt where `SignOutUnknown` and the attempt are expected |
| In `requests.rs`'s `recover`, delete decision 4's test of a parent `sign-in.v1` in `undo-pending`, and nothing else | the same | step 43, an evidence list without the restart's attempt |
| In `sign_in.rs`, make `return false;` the first line of `waits_on_person`'s body | `RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in_wait_ends_unknown_at_its_expiry` | its step 5, where `w1` is still `in-progress` |

Last, with every file restored, run the whole engine suite once, from `engine/`, and report its
last lines; it passes, the vectors among it:

```bash
RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release
```

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 tools/check_model_account.py
python3 -m unittest discover -s tools/tests
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a5gh-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/fixtures conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-setup-account conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/requests/sign-in-account-first conformance/conventions/requests/sign-in-hook-part conformance/conventions/requests/sign-in-plain-hook-part conformance/conventions/requests/sign-in-url-relayed conformance/conventions/inventory conformance/conventions/setup conformance/language engine/README.md engine/crates/shell/src/runner.rs engine/crates/shell/src/server/relay.rs engine/crates/loader engine/crates/conformance tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5gh-declared.yaml: 5 shapes.`,
then `Vector delta: C changes; F flips; 0 undeclared; 0 errors.` and `exit 0`; report `C` and `F`,
and that every change but the added case lies in `requests-sign-in-polling`.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. No promise's text changes, so no row of the model-checking account goes stale; where
`check_model_account.py` reports one stale, stop and report it. The unit tests end `OK`; report the
count. The `git status` command prints nothing, or a change under the spike's directory, reported
with its reason, is the only line it may print.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so the
local gate runs it for this change, as spec batch 1's Gate G asks. From
`experiments/011-short-form-spike/`:

```bash
RUST_TEST_THREADS=4 cargo test --release
```

Its `tests/spike.rs` run passes with 0 failed; report its count.

The `tr` checks read the file with its line breaks as spaces, so a rewrap hides nothing.

Each of these prints 1, the two sentences of decision 12 that name this order:

```bash
grep '^| DL18 |' engine/CONFORMANCE.md | grep -c 'that no retry completes by its output.s expiry ends unknown under PND-010 (order 048a-5gh)'
grep '^| DL19 |' engine/CONFORMANCE.md | grep -c 'While .sign-in.v1. polls (PND-011, order 048a-5gh), an account check that finds an account other than the expected one'
```

This prints `IFC-049:0 PND-005:1 PND-010:0 PND-011:1`, each row's number of cases against the
start's:

```bash
python3 -c 'import re; rows = lambda p: {l.split("|")[1].strip(): int(l.split("|")[4]) for l in open(p) if re.match(r"^\| (PND-005|PND-010|PND-011|IFC-049) \|", l)}; b = rows("engine/target/standing-048a5gh-before.txt"); a = rows("engine/STANDING.md"); print(" ".join(k + ":" + str(a[k] - b[k]) for k in sorted(b)))'
```

These print `47`, the new case's length, and
`37 ['$poll5', '$signout5', 'group.sign-in.v1'] {"signout5": "/reply/hook"} ['PND-011']`: F's length,
one more than at the start (37 from 36), its steps 32 to 34, the capture of its changed step and
the rule of its inserted one. Where the start found the interim ending at another step of F, read
the indexes from there, and report it:

```bash
python3 -c 'import json; print(len(json.load(open("conformance/conventions/requests/sign-in-other-account-undone/case.yaml"))["steps"]))'
python3 -c "import json; s = json.load(open('conformance/conventions/requests/sign-in-polling/case.yaml'))['steps']; print(len(s), [s[k]['input'].get('invocation', s[k]['input'].get('request', {}).get('tool')) for k in (31, 32, 33)], json.dumps(s[31]['capture']), [d['ruleId'] for d in s[32]['expect']['reply']['structuredContent']['diagnostics']])"
```

Each of these prints 0:

```bash
grep -rn '048a-5g\|048a-5h' engine/crates/core/src engine/crates/shell/src | wc -l | tr -d ' '
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Until order 048a-5gh'
grep -c 'external_intents.push' <(sed -n '/fn sign_in_wait_ended(/,/^}/p' engine/crates/core/src/sign_in.rs)
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, and give the last line of every check, each exit status
and count above, the rows decision 12 names with the totals, the names, files and lines that the
start asks for, the signatures of decisions 2, 5, 7 and 8, the lines in the poll arm where the undo
is dispatched and in the part arm where the sign-out part's end is handled, the lines in `recover`
and `cancel` that decision 4 adds, the line in `call_tool` where the timer is started and in
`step_body` where the input's arm is, the members you added to the unit test's invocations, the
sentences of `engine/CONFORMANCE.md` that decision 12 replaced or changed, before and after, and
whether any path reaches the endings decisions 1 and 3 ask about.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5gh-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. Under ruling 87 a fresh Codex verifier reads the one round, and a fresh read-only Opus
reader reads the result, and the verifier's report where it holds a must, before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2, as ruling 87 amends it). The admin writes
this order's Record at the foot of this file when it merges.

Where the coordinator dispatched this order stacked on 048a-5ef's round-1 branch, the result is
read there, with `tools/gate.py --base` set to the commit this order started from, that branch's
tip, since `origin/main` lacks the predecessors' work; it merges only after 048a-5ef has merged,
and with it orders 209, 048a-5d and 048a-5d2 before it, and then only once the coordinator has
done the following (the coordinator's, under that lever):

- rebase this order's branch onto `main` with `git rebase --onto origin/main` from 048a-5ef's
  round-1 tip, the commit this order started from, so that only this order's commits are replayed
  onto 048a-5ef's squash, committing and rebasing under the work orders' identity. Where
  048a-5ef's branch was itself rebased, onto 048a-5d2's squash or an earlier one, after this order
  started, the commit this order started from is still the one to rebase from;
- resolve any conflict where a predecessor's verifier or fixer changed, after its round 1, what
  this order changes: `sign_in.rs`, `pending.rs`, `lifecycle.rs`, `requests.rs` and `lib.rs` under
  `engine/crates/core/src/`, `server.rs`, `lib.rs` and `server_tests.rs` under
  `engine/crates/shell/src/`, `engine/CONFORMANCE.md`, the case `requests-sign-in-polling` and
  `conformance/conventions/requests/index.yaml`, keeping the predecessors' merged text and applying
  this order's decisions to it; write `engine/STANDING.md` again with
  `python3 tools/promise_standing.py --write` rather than by hand, and run the pieces tool again
  where the index conflicts;
- run this order's start check in a clean worktree of `main` after 048a-5ef's merge, and where a
  predecessor's merge changed anything that a decision reads (a step, a capture or an evidence range
  of F, the sentence of DL18's row that decision 12 replaces, the interim ending's decision or
  cause, `waiting_for_retry`, `end_hook_wait`, `respond`'s refusal, or a function's name), amend
  this order's text and the build to it, saying so in the Record;
- run again the four single-case runner commands, `requests-sign-in-` and `requests-`, the unit
  and shell tests of "Done when", and `tools/gate.py` against `origin/main`, which runs the whole
  engine suite and the `vector_delta.py` check;
- have the reader read the rebase's resolutions and any amendment as new content (ruling 79,
  point 2), and write this order's Record.

## Record

Not yet run.
