# Work order 048a4 (E7a-4): setup checks the account, undoes another, and a failed undo holds the instance

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the case you add and the two cases you change,
with the promises each tests and whether it passes. This order has network access only because the
engine's tests bind loopback ports, which the sandbox refuses without it; use it for nothing else,
add no crate, and reach no other machine. Run one cargo command at a time. Never print a whole
large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential. Do not run `claude`, `codex` or any client, and
sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry for 048a-2, and the coordinator's cut
that [order 048a2](048a2-build-sign-in-lapse-and-conflict.md) records under "Who decided what",
split 048a-2's entry into five orders, each after the one before, since an order holds one
behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). This is the third:
"048a-4, setup's account check in TPL-012's order (PND-011's setup half), with the signIn
fixture's scripted account and sign-out hook items, the wrong-account undo and SignInUndone with
its hold". Order 048a-2 raises an instance's sign-in action on a lapsed login and refuses a second
`sign-in.v1` as conflict; [order 048a3](048a3-build-sign-in-hold.md) holds the instance from that
raise (PND-005's `then`). Later orders build the rest: 048a-5, `sign-in.v1` dispatched, with its
own account check before `sign-in-hook.v1`, IFC-049's relay and polling, and its signed-in answer,
which lifts PND-005's hold; 048a-6, the returns (PND-013, PND-014). This order builds neither.

**What it builds.** One behavior: setup's account check and what follows from it. TPL-012's
`then`: after the run and the connections, "where its Type states signIn and its account pointer
holds a string, checking the account with account-hook.v1 and undoing any different account with
sign-out-hook.v1, under PND-005 and PND-011; then dispatching its setup hook", each part begun only
once every earlier part that is not optional has succeeded. Its `observable`: "The account check
waits for all of them, any undo waits for the check, and the setup hook waits for those parts. No
account or a different account raises the sign-in action under PND-005; setup does not wait for
the person to sign in." PND-011's `observable`: "Setup checks the account after its run and
connections and before its setup hook, in TPL-012's order; no account or a different account
raises the action under PND-005, and a different account is undone by a sign-out-hook.v1 part
before setup proceeds." And: "If sign-out does not succeed, the engine reports SignInUndone=False
with its cause and, until that condition clears, dispatches to that instance no request other than
the parts of sign-in.v1, of a setup and of its retirement: every other request to it, an operation
that starts its model work among them, reports unavailable, with a diagnostic naming PND-011 and
the condition. Only a later account-hook.v1 part within sign-in.v1 or setup that reports the
expected account or none clears that condition; reconciliation never clears it." PND-011's `then`:
"Only the expected account, compared ignoring ASCII case, completes sign-in" and "A failed or
unknown account hook does not complete sign-in". PND-005's `observable` gives the setup half of
"After expiry a new sign-in is raised on the next held request or setup", which order 048a-3 left
here. No promise changes: the texts state all of it, and this order builds it.

**MC9a's H05, which this order follows.** [Order 201](201-mc9a-pending-actions-sign-in-registered.md)
registers MC9a, whose property H05 reads: "A turn held under both PND-005 and PND-011's
`SignInUndone=False` reports unavailable with both diagnostics, PND-005's (with `pendingAction`)
and PND-011's." This order builds exactly that: one result, reason unavailable, the PND-005
diagnostic first and the PND-011 one second, and `pendingAction` naming the sign-in action
(decision 6). It also names a gap in MC9a (decision 10): MC9a's script reaches
`SignInUndone=False` only within `sign-in.v1`'s polling (K11, witness 9), its setup's account
check reports no account (stage 1), and its limits say that the script "cannot tell" PND-011's
hold from PND-005's; so no MC9a witness covers a setup's undo or its failure, and this order's case
tells the two holds apart with an operation that starts no model work.

**Review row 45, which this order does not settle.** The owner's review list, row 45, asks whether
a setup whose account check finds the expected account lifts PND-005's hold. This order does not
answer it. It builds PND-005 as written: the instance "is then held until sign-in.v1 answers that
it is signed in", and PND-003 has `sign-in.v1` alone answer a sign-in action signed-in, so a
setup's account check that finds the expected account clears PND-011's condition and lifts no hold
of PND-005, and the action stays unanswered. The case shows it (decision 8, step 26); if the owner
rules otherwise, a later order changes that step.

**What orders 048a-2 and 048a-3 bring, which this order assumes.** Order 048a-2 is dispatched and
048a-3 is written on `main`; neither is built, and this order starts only once 048a-3 has merged
(below). It assumes, as their decisions state them:

- From 048a-2's decision 4: the sign-in kind stored in 047c's `pending_actions`, keyed by action
  ID, each value `{"action": A, "dispatched": null, "raisedBy": I}`, with four `fact` evidence
  items, the first naming the expected account, and an expiry seven days after the step that
  raised it; the function that gives the action an ID names at a time; and `status.pending`,
  `list.v1`'s `pending` and the `unanswered` key, which list and count it.
- From 048a-2's decision 5: `sign_in.rs`, with the function that gives an instance's expected
  account (no `signIn`, an account that is not a string, or the pointer and the string), called
  "the account function" below; and, in `pending.rs`, the raise, which first tests whether the
  instance holds a sign-in action with no answer at that time and, where it does, allocates
  nothing, stores nothing and gives none; and the decision `raise-sign-in-action`.
- From 048a-2's decision 10: the case `requests-sign-in-lapse-and-conflict`, its fixture, in
  which the keyed implementation `signed-server` implements the core's `account-hook.v1` and
  `sign-out-hook.v1` and the Type `signed-agent.v1` maps both to it, its helper phrases and its
  steps 3, 7 and 8, the setup of the Agent `signed` and its two run reports.
- From 048a-3's decision 1: `State`'s member `sign_in_holds`, keyed by instance ID.
- From 048a-3's decision 2: a raise begins the instance's hold, and the one function in
  `pending.rs` that gives the instance's sign-in action with no answer at a time, or none, called
  "the unanswered function" below.
- From 048a-3's decision 3: the raise takes its raising request as one of two forms, an
  invocation whose call reported a lapse or a held request, each with its trigger, its
  `raisedBy` and its evidence items 2 to 4.
- From 048a-3's decision 4: the held test in `sign_in.rs`, called "the PND-005 test" below, and
  the block in `invoke_inner`, right after the block that answers a kept repeat from its outcome
  and before C11's stubs, that raises for a held request, pushes `hold-for-sign-in`, sets
  `route["pendingAction"]`, calls `reject_diagnostics` with reason `unavailable`, rule `PND-005`
  and its one diagnostic, sets the `_meta` key `pendingAction` and returns.
- From 048a-3's decision 6: the case `requests-sign-in-hold`, its directory's `manifest.json`,
  `documents.json` and `fixture.json` (the Agent `signed` with the account `primary-account` at
  `/account`, the Operations `ping.v1` and `work.v1`, the second stating `startsModelWork: true`,
  and root's Conduit `to-signed` listing both), its definitions `I`, `P`, SUCCESS and HELD(A), and
  its steps 1 to 5, the first two copied from 048a-2's case and the next three from its steps 3,
  7 and 8.
- From 048a-3's decision 7: DL19's sentence in `engine/CONFORMANCE.md` that begins "From its first
  raise the instance is held (PND-005, order 048a-3)" and ends "since `sign-in.v1` does not yet
  answer."

Where 048a-2 or 048a-3 merged otherwise than its order says, apply the nearest sound reading of
each decision below to what merged, and say so. Where a Record names a function, use that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`e4ceb4c8`, before orders 048a-2 and 048a-3; find each place by the text quoted with it.

- `advance` (`engine/crates/core/src/lifecycle.rs:502`) runs a setup's parts: the declarations,
  then the run and the connections (phase `work`), then, once every required part has succeeded,
  the line that sets the setup's phase with `json!("hook")` (line 791) and the block for phase
  `hook`, which dispatches the setup hook through `crate::requests::setup_hook` or settles. Its
  first branch, for an instance that is `retired`, waits while a part is in progress (`hook-pending`
  among others), records `retired-before-part` where a part is left, and settles.
- `requests::hook` (`requests.rs:2098`) dispatches a hook as an invocation with `hook: true`,
  `setup_parent` the setup, `serving` the setup, `needs_result: false`, the dispatch intent without
  the `conduit` key, the record `invocation-dispatched`, the reply
  `{"pending":true,"invocation":<setup>,"hook":<ID>}` and the decision `dispatch-setup-hook`, and
  sets the setup's phase `hook-pending`. Through `previous_hook` it keeps an earlier hook's outcome
  under OPS-021 and OPS-028, and gives the hook the key `{instance}:{operation}`.
- `complete` (`requests.rs:1723`) ends a hook part as any call: for a keyed result with a stated
  outcome, and where the operation states an `outputSchema`, it validates `structuredContent`
  against it (line 1911), ends a mismatch unknown under OPS-007, and `finish` keeps a success's
  outputs in the invocation's `retained.outputs`. Then `lifecycle::child_ended` and
  `child_answered` (`lifecycle.rs:1082` and `1092`) record the part in the setup's `parts` and
  settle or advance; `child_answered` returns early for a phase other than `hook-pending` and
  `entry`. `account-hook.v1` states an `outputSchema` whose `account` is a string or null, and no
  `effect`; `sign-out-hook.v1` states an `effect` (`spec/core/operations.yaml`).
- `settle` (`lifecycle.rs:352`) ends the setup: succeeded where every required part succeeded,
  otherwise unknown where a part with an effect succeeded or ended unknown, and failed otherwise,
  with `explain::settle_cause`'s OPS-022 diagnostic, "Required part {name} ended {status}.", then
  `requests::finish`, which writes the setup's `invocation-ended` record.
- `observe_run` (`lifecycle.rs:1232`) keeps an instance's `Running` condition in
  `status.conditions`, in `condition.v1`'s form, moving the old one to `past_conditions` with
  `endedAt` when its status changes; the inspection shows `status.conditions`
  (`records.rs:478`).
- `cancel` (`lifecycle.rs:1150`) ends any hook part in progress unknown under OPS-027, records
  it as `cancelled-hook` and settles; it never passes through `child_answered`, so decision 3
  changes it for a sign-out part that a cancel ends.
- No case before 048a-2's sets up an instance whose Type states `signIn`, so no other case reaches
  the account check.

**When this order starts.** Order 048a-3 must be merged to `main` first, and so 048a-2. Before you
change anything, check from the repository root that you find what they give, and stop, change
nothing and report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md
grep -c '^## Record' docs/work-orders/048a3-build-sign-in-hold.md
grep -c 'pub sign_in_holds: BTreeMap<String, Value>' engine/crates/core/src/lib.rs
grep -rl '"hold-for-sign-in"' engine/crates/core/src | wc -l
grep -rl '"raise-sign-in-action"' engine/crates/core/src | wc -l
test -f conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml; echo "case $?"
test -f conformance/conventions/requests/sign-in-hold/case.yaml; echo "case $?"
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'From its first raise the instance is held (PND-005, order 048a-3)'
grep -rl 'SignInUndone' engine/crates/core/src | wc -l
grep -c 'account-hook' engine/crates/core/src/lifecycle.rs
grep -c '^| PND-005 | convention | must | 2 | 0 | outside | core | case passes |$' engine/STANDING.md
grep -c '^| PND-011 | convention | must | 0 | 0 | outside | none | no case, no reason |$' engine/STANDING.md
grep -c '^\*\*H05 (PND-005, must)' experiments/012-model-checking/README.md
```

They print `1`, `1`, `1`, `1`, `1`, `case 0`, `case 0`, `1`, `0`, `0`, `1`, `1` and `1`. Orders
that land beside this one (046b, 047d, 049a-1, 050 and others): whichever lands second keeps both
changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** TPL-012, PND-011, PND-005, PND-001, PND-003, PND-012, OPS-022 and OPS-023
decide where the account check falls among a setup's parts, what each answer of it does, that a
different account is undone before setup goes on, what a failed undo reports and holds, what clears
it, and the order of reasons. MC9a's H05, registered by order 201, gives a request held under both
holds both diagnostics. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-2 that puts this behavior here
(order 048a2's "Who decided what"); reading "the signIn fixture's scripted account and sign-out
hook items" as 048a-2's fixture, whose keyed `signed-server` already implements both hooks, with
the account and the undo's outcome scripted by each case's completions, so that no fixture
changes; dispatching the two parts afresh at each setup, with no idempotency key, since OPS-028's
rules for repeats apply to the setup and retire hooks only (`spec/definitions.md`, "setup hook");
the part names `account` and `sign-out`; reading a failed or unknown account check as a required
part that did not succeed, which raises nothing and ends the setup under OPS-022; that the setup is
the raising invocation, its `raisedBy` and its evidence items' words and triggers; naming the
action in the setup's `invocation-ended` record; raising nothing and beginning no undo where the
instance is retired when the check ends, as MC9a-002's reading has it, while still setting
`SignInUndone=False` where its undo did not succeed and clearing it where its check found the
expected account or none, since PND-011 makes no exception for a retirement; setting
`SignInUndone=False` where a cancel ends the sign-out part, and where a restart's `recover` ends
it, with no PND-011 diagnostic on the setup's result; `SignInUndone` as a condition of the
instance in `condition.v1`'s form, its reasons, messages and source, and its clearing to `True`;
a second diagnostic naming PND-011 on the setup's result; that a successful undo clears no earlier
`SignInUndone=False`, as PND-011 names only an account check; the PND-011 test and its place,
shared with 048a-3's block; the rule `PND-005` where both holds apply, the diagnostics' order,
`pendingAction` only where PND-005 holds, and no raise under PND-011's hold alone; the
diagnostics' words; the decisions' names; the unit tests; the case, its reuse of 048a-3's files,
route, IDs and times; the step inserted in 048a-2's and 048a-3's cases; and DL19's new sentence.
Apply them as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: TPL-012 (`spec/promises/templates.yaml`); PND-001, PND-003, PND-004, PND-005,
   PND-010, PND-011 and PND-012 (`spec/promises/pending.yaml`); OPS-022 and OPS-023
   (`spec/conventions/operations.yaml`); CTR-013 (`spec/promises/contracts.yaml`); `account-hook.v1`
   and `sign-out-hook.v1` (`spec/core/operations.yaml`); `condition.v1` (`spec/core/results.yaml`);
   the rows setup hook and set up of `spec/definitions.md`.
2. [Order 048a2](048a2-build-sign-in-lapse-and-conflict.md) and
   [order 048a3](048a3-build-sign-in-hold.md), whole, and their Records.
3. In `experiments/012-model-checking/README.md`, MC9a's properties H05, K11 and P01, its finding
   MC9a-002 and its limits.
4. The cases `requests-sign-in-lapse-and-conflict`, `requests-sign-in-hold` and
   `setup-hook-failed`.
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.3 to 0.45k changed lines of Rust with two unit tests, one new case of 30 steps
that reuses 048a-3's fixture, one step inserted in each of two cases, and one sentence in
`engine/CONFORMANCE.md`.

## The decisions it applies

1. **The account check, in TPL-012's order.** In `advance`, where phase `work` ends with every
   required part succeeded, set the phase `account` in place of `hook`, and add a block for phase
   `account` right before the block for phase `hook`. In it, call the account function for the
   setup's object. Where it gives the pointer and a string, dispatch an `account-hook.v1` part
   (decision 2), set the phase `account-pending` and return. Otherwise (no `signIn`, an account
   that is not a string, a template, an object that is not a Resource or Agent instance) set the
   phase `hook` and go on, so that the setup hook follows as before. In the first branch, for a
   retired instance, wait as for `hook-pending` while the phase is `account-pending` or
   `undo-pending`, and count the account check as a part left, like the hook, where the account
   function gives a string and the phase is `declarations`, `implementations`, `work` or
   `account`. A setup of an instance whose Type states no `signIn` dispatches and records exactly
   what it did before.

2. **The two parts are dispatched afresh.** Add to `requests.rs` a function beside `hook` that
   dispatches `account-hook.v1` or `sign-out-hook.v1` as a part of a setup, as `hook` does, with
   these differences: it never consults `previous_hook` and keeps no earlier outcome; it allocates a
   new ID with `a.next` and pushes a new invocation with `attempt` 1 and `key` null, so that its
   dispatch carries no `idempotencyKey`; and it leaves the setup's phase to its caller. Its
   implementation is `o["resolved_implementations"]` at the operation's qualified reference; where
   none is resolved, which CTR-013's checks (order 048a-1) should leave no way to reach, record the
   part `failed` with no invocation and settle, and report whether any path reaches it. Push one
   decision, `dispatch-account-check` citing `TPL-012`, `PND-011`, `CTR-013`, `ACC-018` and
   `IMP-002`, or `dispatch-sign-out` citing `TPL-012`, `PND-011`, `ACC-018` and `IMP-002`, with
   no implementation choice and the facts `{"hook": ID}`. The reply is
   `{"pending":true,"invocation":<setup>,"hook":ID}`, as `hook` gives it.

3. **What the check's answer does.** In `child_answered`, before its present test of the phase,
   handle the two new phases, each first recording the part, named `account` or `sign-out`, with
   its outcome, its effect read from the contract as the hook's is, not optional, and the part's
   invocation ID. Then, where the instance is retired: in `undo-pending`, where the part did not
   succeed, first call decision 5's helper with the part's outcome and ID and keep the part's
   outcome in the setup's `setup.signOut`, as the arm below does, since PND-011's "If sign-out does not succeed,
   the engine reports SignInUndone=False with its cause" makes no exception for a retirement; in
   `account-pending`, where the part succeeded and found the expected account or none, clear the
   condition (decision 5); then, in either phase, set the phase `hook` and call `advance`, whose
   first branch records `retired-before-part` where a setup hook is selected and settles
   (decision 5 adding its diagnostic where `setup.signOut` is held). Raise nothing and begin no
   undo (as MC9a-002 reads a retirement already dispatched). Otherwise:
   - **`account-pending`**, where the part did not succeed: settle. The setup ends under OPS-022
     naming the part `account`, nothing is raised and no condition changes (PND-011: "A failed or
     unknown account hook does not complete sign-in").
   - **`account-pending`**, where it succeeded: read `account` from the part's
     `retained.outputs`, which `complete` validated as a string or null, and the expected account
     from the account function (where it no longer gives a string, set the phase `hook` and call
     `advance`, and report whether any path reaches it). Where the account equals the expected one
     ignoring ASCII case, clear the condition (decision 5), set the phase `hook` and call
     `advance`. Where it is null, raise (decision 4) with the trigger `setup-no-account`, clear the
     condition, set the phase `hook` and call `advance`: setup does not wait for the person to sign
     in. Where it is another string, raise with the trigger `setup-other-account`, then dispatch a
     `sign-out-hook.v1` part (decision 2) and set the phase `undo-pending`; the condition does not
     change.
   - **`undo-pending`**, where the part succeeded: set the phase `hook` and call `advance`. An
     earlier `SignInUndone=False` stays: PND-011 names only an account check as clearing it.
   - **`undo-pending`**, where it did not succeed: call decision 5's helper with the part's
     outcome and ID, keep the part's outcome in the setup's `setup.signOut`, and settle
     (decision 5 adds its diagnostic).

   After each arm, where the setup is no longer `in-progress`, call `child_ended` for it, as the
   hook's arm does. No other phase changes.

   `cancel` never passes through `child_answered`, so change it too: where the setup's phase is
   `undo-pending`, after the sign-out child has ended unknown under OPS-027 and been recorded as
   `cancelled-hook`, call decision 5's helper with that part's outcome and ID, and keep the
   outcome in `setup.signOut`, so that `settle` adds the PND-011 diagnostic. Where the phase is
   `account-pending`, the account check ends unknown as any hook does and no condition changes.

   `recover` (`requests.rs:2039`), which the portable input `restore-state` reaches, ends every
   invocation in progress unknown under IMP-007 through `finish`, which never calls `child_ended` or
   `child_answered`, so change it too: after it ends a hook invocation whose `setup_parent` is a
   setup in phase `undo-pending`, call decision 5's helper with outcome `unknown` and that part's
   ID. The setup itself still ends under IMP-007 as now, and no PND-011 diagnostic is added to its
   result, since `settle` is not reached. Where the setup that recover ends holds `setup.raised`,
   add `"pendingAction": [ID]` to its `invocation-ended` record's `data`, in `t.records` and
   `t.state.records`, as decision 4 does in `settle`.

4. **Setup raises the action, and its record names it.** Make the raise take a third form of
   raising request, beside 048a-3's two: a setup, given the setup invocation. Its trigger is
   `setup-no-account` or `setup-other-account`, the stored entry's `raisedBy` is the setup's ID
   (PND-001: "Where there is a raising invocation, the action retains its attribution to that
   invocation"), and the evidence items are, in this order, each of kind `fact`:
   1. 048a-2's first item, the expected account, unchanged.
   2. `V` the setup's ID; `W` `records`; `P`
      `the setup whose account check found the instance not signed in to its expected account`.
   3. `V` the setup's `operation`; `W` `records`; `P` `the operation of that request`.
   4. `V` the trigger; `W` `records`; `P` `what the setup's account check reported`.

   No item names the account the check found. Everything else follows 048a-2's decision 5 and
   048a-3's decision 2: the unanswered test first, then the ID from `a.next`, the stored entry, the
   hold begun where the instance has none, and the decision `raise-sign-in-action`, here citing
   `PND-001`, `PND-005`, `PND-011`, `PND-012` and `TPL-012`, with `DL19`, and the facts
   `{"action": ID, "instance": S}`. So an action is raised only where none is unanswered, and once
   one has expired the next setup whose check finds no account or another raises a new one
   (PND-005: "After expiry a new sign-in is raised on the next held request or setup"). Where it
   gives an action, keep its ID in the setup's `setup.raised`; in `settle`, after `finish` has
   written the setup's `invocation-ended` record, add `"pendingAction": [ID]` to that record's
   `data`, in `t.records` and in `t.state.records`, as 048a-2's decision 5 does (PND-012). The
   signature is your choice; report it.

5. **`SignInUndone`, the condition.** In `sign_in.rs`, add two functions on an instance object and
   the step's time, written as `observe_run` writes `Running`:
   - **Set false**, given also the sign-out part's outcome and ID: in `status.conditions`, the
     condition
     `{"type": "SignInUndone", "status": "False", "reason": R, "message": M, "lastTransitionTime": X, "source": "urn:uuid:<part ID>"}`,
     `R` being `SignOutFailed` where the part ended failed and `SignOutUnknown` otherwise, `M`
     `The sign-out hook ended {outcome} after setup's account check found an account other than the expected one, so that account may still be signed in. Requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable under PND-011 until an account check finds the expected account or none.`,
     and `X` the step's time, or the old condition's `lastTransitionTime` where it was already
     `False`; where the status changes, the old condition moves to `past_conditions` with
     `endedAt` the step's time.
   - **Clear**: only where the instance has a `SignInUndone` condition whose status is `False`,
     the condition becomes
     `{"type": "SignInUndone", "status": "True", "reason": "AccountChecked", "message": "An account check within setup found the expected account or none.", "lastTransitionTime": X, "source": S}`,
     `S` the old condition's source, and the old one moves to `past_conditions`; otherwise nothing
     changes, and no condition is written where none was.

   Neither message names an account (PND-004). Decision 3 sets it false in one place: add a
   `pub(crate)` helper in `lifecycle.rs`, beside `settle`, that takes the Transition, the
   instance ID, the sign-out part's outcome and ID, and the step's time, applies set false on the
   instance in `t.state.objects`, and pushes one decision named `report-sign-in-undone`, citing
   `PND-011`, with the facts `{"instance": S, "hook": <part ID>}`. `child_answered`, `cancel` and
   `recover` each call it, and it is the only place that writes the decision's name. Where
   clearing changes the condition, push one decision named `clear-sign-in-undone`, citing
   `PND-011` and `TPL-012`, with the facts `{"instance": S}`. In `settle`, where the setup
   holds `setup.signOut`, the cause that `settle_cause` gives gains a second diagnostic after
   OPS-022's, built with `explain::outcome_diagnostic` for the setup's outcome, the rule `PND-011`
   and the cause
   `The sign-out hook ended {outcome} after the account check found an account other than the expected one, so SignInUndone is False and requests to the instance other than the parts of sign-in.v1, of a setup and of its retirement report unavailable until an account check finds the expected account or none.`,
   so that the setup's result reports SignInUndone=False with its cause before it ends. Add no row
   to `OUTCOME_CAUSES`. `SignInUndone` survives a purge and a restart, as the state does, and
   reconciliation never touches it.

6. **PND-011's hold, beside PND-005's.** In `sign_in.rs`, add a function that says whether a
   request is held under PND-011, given the state and the target's ID: true where the object with
   that ID in `state.objects` is a Resource or an Agent with no `spec` (not a template) and its
   `status.conditions` holds a `SignInUndone` condition whose status is `False`. In `invoke_inner`,
   048a-3's block becomes one block for both holds, at the same place. Where neither the PND-005
   test nor this one holds, the request goes on as before. Otherwise:
   - Where the PND-005 test holds, everything 048a-3's decision 4 does before `reject_diagnostics`
     stays as it is: its raise for a held request, its `ID`, its decision `hold-for-sign-in` and
     `route["pendingAction"]`. Where only PND-011 holds, nothing is raised.
   - Where PND-011 holds, push one decision named `hold-for-sign-in-undone`, citing `PND-011` and
     `OPS-023`, with no implementation choice and the facts `{"instance": S}`.
   - Call `reject_diagnostics` once, with the reason `unavailable`, the rule `PND-005` where the
     PND-005 test holds and `PND-011` otherwise, and the diagnostics in this order: 048a-3's
     PND-005 diagnostic where the PND-005 test holds, then, where PND-011 holds,
     `{"ruleId": "PND-011", "level": "error", "message": {"text": T}}`, `T` being
     `{tool} reports unavailable under PND-011: SignInUndone is False for the instance, since a sign-out after an account check did not succeed, so the request was not dispatched; send it again once an account check within sign-in.v1 or setup finds the expected account or none.`
     and `{tool}` the route's `tool`. This is H05's "both diagnostics, PND-005's (with
     `pendingAction`) and PND-011's".
   - Set the `_meta` key `pendingAction` as 048a-3 does, only where the PND-005 test gave an `ID`.
   - Return. Nothing is dispatched, queued or kept (OPS-025).

   So an Agent's request whose operation starts no model work is held under PND-011 alone, with no
   `pendingAction`, while the condition is `False`, and dispatched once it clears. A request that a
   reason before this one ends keeps that reason (OPS-023).

7. **Two unit tests pin the rules.** Write the values as `json!` values; members a test does not
   need may be left out. Let T be `2026-11-12T12:00:00Z`.
   - In `sign_in.rs`'s tests, `sign_in_undone_condition_and_hold`, on a `State` built by hand with
     the Agent `s1`, whose `status.conditions` holds a `Running` condition, the Agent `s2`, the
     Resource `x1` and an Agent template `t1` (with a `spec`): decision 5's set false on `s1` at T
     for the part `h1` that ended `failed` gives the second condition whole, with reason
     `SignOutFailed`, `lastTransitionTime` T and source `urn:uuid:h1`, and leaves `Running`
     unchanged; decision 6's test is true for `s1`; set false again at T+1s for the part `h2` that
     ended `unknown` gives the reason `SignOutUnknown`, source `urn:uuid:h2`, `lastTransitionTime`
     still T and no `past_conditions`; the same condition, written by hand into
     `status.conditions`, on `x1`, held, and on `t1`, not; `s2`, not; clear on `s1` at T+2s gives status
     `True`, reason `AccountChecked`, `lastTransitionTime` T+2s and one `past_conditions` entry,
     the `False` condition with `endedAt` T+2s, and decision 6's test is then false; clear again at
     T+3s changes nothing; clear on `s2` writes no condition. Assert with `assert_eq!`.
   - In `pending.rs`'s tests, `sign_in_action_raised_by_setup`, with the objects of 048a-3's test
     `hold_outlasts_expiry` (the Agent `s1` with the account `primary-account` at `/account`):
     decision 4's raise at T for a setup `u1` whose operation is the core's `set-up.v1`, with the
     trigger `setup-other-account`, gives the whole action (expiry `2026-11-19T12:00:00Z`, the four
     items of decision 4) and a stored entry whose `raisedBy` is `u1`, and sets `s1`'s hold to
     `{"since": T}`; a raise at T+1s for `u1` with `setup-no-account` gives none and stores
     nothing; at the action's expiry the action its ID names has the answer `expired`, and a raise
     for a setup `u2` with `setup-no-account` gives a new action with another ID, expiry
     `2026-11-26T12:00:00Z` and `raisedBy` `u2`, and the hold is still `{"since": T}`.

8. **One portable case, on 048a-3's fixture.** Add the directory
   `conformance/conventions/requests/sign-in-setup-account/` with one file, `case.yaml`: ID
   `requests-sign-in-setup-account`, `layers: [convention, interface]`,
   `promises: [PND-011, PND-005, TPL-012, PND-001]`, title
   `Check the account in setup: no account or another raises the sign-in action, another is undone, and a failed undo holds the instance under SignInUndone until an account check clears it`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-hold/fixture.json", "manifest": "../sign-in-hold/manifest.json", "documents": "../sign-in-hold/documents.json"}`,
   048a-3's case's three files, unchanged. Write it as JSON with two-space indentation and a final
   newline, its members in the order of `requests-sign-in-hold`. It must meet 046a's portable-case
   rule as it stands.

   Below, ROOT, `META/x`, `U`, `I`, `P`, "Root asks T with A", "a run report", "a completion",
   SUCCESS and HELD(A) are as 048a-3's decision 6 defines them. A new step's evidence is its time
   and, as 048a-3 writes its new steps, `ids` the 100 strings `00000000-0000-4000-8000-` followed
   by the twelve-digit, zero-padded decimal of 80000 + 100n + k, for k = 0 to 99, `revisions` the
   16 strings `account-` followed by the decimal of n, a `-` and the decimal of j, and `suffixes`
   the 16 strings `a` followed by the four-digit, zero-padded decimal of 100n + j, for j = 0 to 15,
   where n is the step's number in this case; a copied step keeps its own evidence. Every new
   step's time is `2026-11-12T12:00:00Z`, but that of steps 27 to 30, which is
   `2026-11-19T12:00:00Z`, `$signin`'s expiry. `K(x)` is the pointer
   `/external_intents/0/call/params/_meta/` followed by the `_meta` key `x`, its `/` written `~1`, as `setup-hook-failed` writes the key
   `conduit`. More results recur:
   - ACCOUNT(X): `{"kind": "result", "result": {"_meta": {"META/outcome": "succeeded"}, "content": [], "structuredContent": {"account": X}}}`;
   - SIGN-OUT-FAILED: `{"kind": "result", "result": {"isError": true, "_meta": {"META/outcome": "failed"}, "structuredContent": {}}}`,
     as `setup-hook-failed` writes a failed hook;
   - PART(H): `"external_intents": [{"kind": "dispatch", "attempt": 1, "call": {"params": {"_meta": {"META/operation": Q, "META/caller": ROOT}}}}]`
     with `"absent": [K(conduit), K(idempotencyKey)]`, `Q` the core's qualified reference to the
     Operation H, each written as `setup-hook-failed` writes its setup hook's dispatch, in the
     core's short form;
   - HELD11: `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable", "META/unanswered": 2}, "structuredContent": {"diagnostics": [{"ruleId": "PND-011", "level": "error"}]}}, "external_intents": [], "absent": [I, P]}`;
   - HELDBOTH(A): `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable", "META/pendingAction": [A], "META/unanswered": 2}, "structuredContent": {"diagnostics": [{"ruleId": "PND-005", "level": "error"}, {"ruleId": "PND-011", "level": "error"}]}}, "external_intents": [], "absent": [I]}`.

   Its steps:
   1. to 5. `requests-sign-in-hold`'s steps 1 to 5 as decision 9 leaves them, `input`, `expect`
      and `capture`, whole, each with its evidence: first start, registration, the setup of
      `signed` and its two run reports, the second now dispatching the account check, captured as
      `$account0`.
   6. A completion for `$account0`, ACCOUNT(`"other-account"`). `expect`
      `{"reply": {"pending": true}}` with PART(`sign-out-hook.v1`); `capture`
      `{"signout1": "/reply/hook"}`: another account raises the action and is undone before setup
      goes on.
   7. A completion for `$signout1`, SUCCESS. `expect`
      `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/invocation": "$setup1", "META/unanswered": 1}}}`:
      undone, the setup succeeds without waiting for the person.
   8. Root asks `group.inspect.v1` with `{"object": {"kind": "Agent", "name": "signed"}}`. `expect`
      `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"conditions": [{"type": "Running"}], "pending": [{"kind": "sign-in", "subject": [{"kind": "Agent", "name": "signed"}], "expiry": "2026-11-19T12:00:00Z"}]}}}}`;
      `capture` `{"signin": "/reply/structuredContent/status/pending/0/id"}`: a successful undo
      leaves no `SignInUndone`.
   9. Root asks `access.open.v1` with `{"conduit": "to-signed"}`. `expect`
      `{"reply": {"isError": false}}`.
   10. Root asks `to-signed.ping.v1` with `{}`. `expect` `{"reply": {"pending": true}}`; `capture`
       `{"call1": "/reply/invocation"}`: an operation that starts no model work is dispatched.
   11. A completion for `$call1`, SUCCESS. `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/unanswered": 1}}}`.
   12. Root asks `to-signed.work.v1` with `{}`. `expect` HELD(`"$signin"`): the setup's raise began
       PND-005's hold.
   13. Root asks `group.set-up.v1` with `{"instance": {"kind": "Agent", "name": "signed"}}`.
       `expect` `{"reply": {"pending": true}}`; `capture` `{"setup2": "/reply/invocation"}`.
   14. A run report for `$setup2`, `R` `{"kind": "inspect-run", "state": "running"}`. `expect`
       `{"reply": {"pending": true}}` with PART(`account-hook.v1`); `capture`
       `{"account2": "/reply/hook"}`.
   15. A completion for `$account2`, ACCOUNT(`"other-account"`). `expect`
       `{"reply": {"pending": true}}` with PART(`sign-out-hook.v1`); `capture`
       `{"signout2": "/reply/hook"}`: no second action while one is unanswered.
   16. A completion for `$signout2`, SIGN-OUT-FAILED. `expect`
       `{"reply": {"isError": true, "_meta": {"META/outcome": "unknown", "META/invocation": "$setup2", "META/unanswered": 2}, "structuredContent": {"diagnostics": [{"ruleId": "OPS-022", "level": "error"}, {"ruleId": "PND-011", "level": "error"}]}}}`:
       the setup goes no further, ends unknown since its run has an effect (`settle`), and reports
       SignInUndone=False; its setup action is raised beside the sign-in action.
   17. Step 8's request. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 2}, "structuredContent": {"status": {"conditions": [{"type": "Running"}, {"type": "SignInUndone", "status": "False", "reason": "SignOutFailed"}], "pending": [{"id": "$signin", "kind": "sign-in"}, {"id": "$setup2", "kind": "setup"}]}}}}`.
   18. Step 10's request. `expect` HELD11, with
       `"record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "unavailable", "ruleId": "PND-011"}}]`:
       held under PND-011 alone, naming no action.
   19. Step 12's request. `expect` HELDBOTH(`"$signin"`), with
       `"record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "unavailable", "ruleId": "PND-005", "pendingAction": ["$signin"]}}]`:
       MC9a's H05.
   20. Step 13's request. `expect` `{"reply": {"pending": true}}`; `capture`
       `{"setup3": "/reply/invocation"}`: its dispatch answers the setup action `retried`.
   21. A run report for `$setup3`, step 14's `R`. `expect` step 14's; `capture`
       `{"account3": "/reply/hook"}`.
   22. A completion for `$account3`, ACCOUNT(`"PRIMARY-ACCOUNT"`). `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/invocation": "$setup3", "META/unanswered": 1}}}`:
       the expected account, compared ignoring ASCII case; no sign-out.
   23. Step 8's request. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"conditions": [{"type": "Running"}, {"type": "SignInUndone", "status": "True", "reason": "AccountChecked"}], "pending": [{"id": "$signin", "kind": "sign-in"}]}}}}`:
       the condition cleared, and the sign-in action is still unanswered.
   24. Step 10's request. `expect` `{"reply": {"pending": true}}`; `capture`
       `{"call2": "/reply/invocation"}`: PND-011's hold is lifted.
   25. A completion for `$call2`, SUCCESS. `expect` step 11's.
   26. Step 12's request. `expect` HELD(`"$signin"`): the expected account found by setup lifts no
       hold of PND-005, as written (review row 45, not settled here).
   27. Time `2026-11-19T12:00:00Z`, `$signin`'s expiry: step 13's request. `expect`
       `{"reply": {"pending": true}}`; `capture` `{"setup4": "/reply/invocation"}`.
   28. A run report for `$setup4`, step 14's `R`. `expect` step 14's; `capture`
       `{"account4": "/reply/hook"}`.
   29. A completion for `$account4`, ACCOUNT(`null`). `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/invocation": "$setup4", "META/unanswered": 1}}}`:
       no account raises an action, and setup does not wait for the person to sign in.
   30. Step 8's request. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"pending": [{"kind": "sign-in", "subject": [{"kind": "Agent", "name": "signed"}], "expiry": "2026-11-26T12:00:00Z"}]}}}}`:
       after expiry a new sign-in is raised on the next setup (PND-005).

   The route is new and has not been run, since orders 048a-2 and 048a-3 are not yet built. Before
   running it, check that no ID repeats across the case's steps' `ids`, the copied steps'
   included. If 046a's runner check refuses the case, if a setup asks for a report other than those
   its steps send, if a step copied from another case gives a reply other than that case states, or
   if a step's reply differs from what it states, stop and report the error: do not change the
   expectations to pass it, and do not change the inputs beyond what this decision says. No
   expectation is generated from the engine's output. The coverage index entry holds `id`, `case`
   (`conventions/requests/sign-in-setup-account/case.yaml`), `promises` and, where its neighbours
   in that piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write it
   in a `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it into
   `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte for
   byte as it was.

9. **The two earlier cases gain the check.** Their setup of `signed`, whose account is
   `primary-account`, now dispatches an account check before it ends, as 048a-2's and 048a-3's
   orders foresaw. Change only these, in each `case.yaml`:
   - `requests-sign-in-lapse-and-conflict`: its step 8, the run report `run` for `$setup1`, keeps
     its `input` and evidence; its `expect` becomes `{"reply": {"pending": true}}` with
     PART(`account-hook.v1`), and it gains `capture` `{"account0": "/reply/hook"}`. A new step 9
     follows: a completion for `$account0`, ACCOUNT(`"primary-account"`), with `expect`
     `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}}`, step 8's earlier
     one, and the evidence: time `2026-11-12T12:00:00Z`, `ids` the 100 strings
     `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of 90000 + k for
     k = 0 to 99, `revisions` the 16 strings `account-check-` followed by the decimal of j, and
     `suffixes` the 16 strings `c` followed by the four-digit, zero-padded decimal of j, for j = 0
     to 15. Every later step moves one place, unchanged. The setup of `unset`, whose account is
     null, checks no account and does not change.
   - `requests-sign-in-hold`: its step 5 becomes 048a-2's step 8 as changed above, whole, and a new
     step 6 follows, 048a-2's new step 9, whole with its evidence. Every later step moves one
     place, unchanged.

   Nothing else in either case, and nothing in their directories' other files, changes. Before
   running them, check again that no ID repeats across each case's steps. Both cases must then
   pass as before; if either step after the inserted one gives another reply than its case states,
   stop and report it.

10. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find the sentence by its text read across
    line breaks, as 048a-3 leaves it. In DL19's row in "Choices still required", right after the
    sentence that begins "From its first raise the instance is held (PND-005, order 048a-3)" and
    ends "since `sign-in.v1` does not yet answer.", add, in the same line:

    ```text
    Setup checks the account (PND-011, TPL-012, order 048a-4): where the instance's Type states `signIn` with a string account, an `account-hook.v1` part reports the account after the run and connections and before the setup hook; no account or another raises the sign-in action unless one is unanswered, and another is undone by a `sign-out-hook.v1` part before setup goes on. Where that part does not succeed, the setup goes no further, the instance's condition `SignInUndone` is `False`, and every request to the instance reports unavailable under PND-011, beside PND-005's hold, until a setup's account check finds the expected account or none and sets it `True`; such a check lifts no hold of PND-005, and the check within `sign-in.v1` is not built yet.
    ```

    If 048a-3 merged its sentence otherwise, put the new one right after it, and say so. Nothing
    else in the file changes. After the other decisions, run
    `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
    `promises` and to a promise ID in the core's sources. Report the rows of PND-001, PND-005,
    PND-011 and TPL-012 and the totals; the totals change only by what those rows change.

11. **The declaration.** `docs/work-orders/048a4-declared.yaml` holds four shapes: one `added` for
    `requests-sign-in-setup-account`, and one each of `input`, `expectation` and `flip` for the two
    cases of decision 9 under `/steps/*`, since an inserted step moves every later step's index.
    The coordinator wrote it with this order; check that it matches and change it only to make it
    so.

12. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
    record changes. No fixture changes. `sign-in.v1` keeps its stub and 048a-2's preflight: its own
    account check before `sign-in-hook.v1`, its polling, its undo and its clearing of
    `SignInUndone` are 048a-5's, and nothing answers a sign-in action signed-in or lifts PND-005's
    hold. No hook, CLI command (051b), shell code, relay (048b-relay, 048b-join) or package (050)
    changes. Report as findings, for the coordinator: that review row 45 stays open, and step 26
    pins the reading as written; MC9a's gap named above, that no MC9a witness covers a setup's
    undo or its failure; that a successful undo leaves an earlier `SignInUndone=False` in place
    until an account check clears it, as PND-011 is written; that the setup ends unknown, not
    failed, when its undo fails after a run with an effect, by `settle`'s rule, and whether OPS-022
    asks otherwise; that no item of the action names the account the check found, and no promise
    says whether one should; that the setup's result does not carry `META/pendingAction` for the
    sign-in action it raised, which IFC-025's "raised" asks for, as 048a-3 reported for a lapse,
    and no order of the cut builds it; as coordinator choices no check of this order tests, the
    `pendingAction` that the setup's `invocation-ended` record names, a failed or unknown account
    check, a retirement during the check or the undo, `SignInUndone=False` set where a cancel ends
    the sign-out part and where a restart's `recover` ends it, and the `pendingAction` that
    `recover` adds to the record of a setup it ends, so that a build that omits them passes every
    check here; that `requests::cancel` (`requests.rs:1938`), which the portable input `cancel`
    reaches when it names a setup's part by its ID, the sign-out part included, ends that part
    through `finish` without `child_ended`, so the setup stays in its pending phase and no
    `SignInUndone` is set, a gap setup hooks already have: report whether any path reaches it, and
    change nothing for it; and any place where TPL-012, PND-005 or PND-011 left you a choice this
    order does not settle.

## Scope

Change only `engine/crates/core/src/lifecycle.rs` (`advance`, `child_answered`, `cancel`,
`settle` and decision 5's helper, decisions 1, 3, 4 and 5), `requests.rs` (decision 2's function; `recover`, decision 3;
and `invoke_inner`'s hold block, decision 6), `pending.rs` (decisions 4 and 7), `sign_in.rs` (decisions 5, 6 and 7),
`engine/CONFORMANCE.md` (one sentence), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/requests/index.yaml`, the new directory
`conformance/conventions/requests/sign-in-setup-account/`, and the two files
`conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml` and
`conformance/conventions/requests/sign-in-hold/case.yaml` (decision 9).
`docs/work-orders/048a4-declared.yaml` comes with this order; change it only as decision 11 says.
`conformance/manifest.yaml` holds the new entry only until the pieces tool moves it, and ends
unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, the shell, the experiments, the case
schema, `conformance/internal-cases.yaml`, `conformance/expected-failures.yaml`, any other case or
fixture, the other files of 048a-2's and 048a-3's case directories, `docs/implementation/` or
`docs/proposals/`: report any defect as a finding. `experiments/011-short-form-spike/` changes only
if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- sign_in
cargo test --release -p conduit-core --lib -- pending::tests
cargo test --release -p conduit-core --lib -- lifecycle
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-account
cargo run --release -p conduit-conformance -- .. requests-sign-in-hold
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cargo run --release -p conduit-conformance -- .. setup-
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 6 passed and 0 failed: 048a-3's four and this order's two, both of whose paths
hold `sign_in`. The fourth reports 6 passed: 048a-3's five and this order's. Report the count if
another test's name matches. The fifth passes as on the `main` you start from; report its count.
The pieces tool prints `Nothing to move.` The first three runner commands each end
`1 cases passed`; `setup-` gives as many cases as on the `main` you start from, and `requests-` one
more, with its one expected failure unchanged; report each count. The shell test passes. If any
other case, or any shell, CLI or experiment test, fails because a setup now checks an account or a
request is now held, stop and report it: do not change it. The threads are 4 because at 24 the
runner ran out of memory (order 071's Record).

Next, show that the case fails without decision 1. From `engine/`, first keep a copy:

```bash
cp crates/core/src/lifecycle.rs target/lifecycle.rs.fixed
```

Then, by hand, in `crates/core/src/lifecycle.rs`, change the line of `advance` that sets the
setup's phase `account` where phase `work` ends back to `json!("hook")`, and nothing else. Then,
one at a time:

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-account
cp target/lifecycle.rs.fixed crates/core/src/lifecycle.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-account
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with an error at
step 5, whose reply is the setup's success where the account check's dispatch is expected; after
the restore the second ends `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its
older modification time, so cargo would not rebuild the crate. Git ignores `target/`.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a4-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/requests/sign-in-lapse-and-conflict/manifest.json conformance/conventions/requests/sign-in-lapse-and-conflict/documents.json conformance/conventions/requests/sign-in-lapse-and-conflict/fixture.json conformance/conventions/requests/sign-in-hold/manifest.json conformance/conventions/requests/sign-in-hold/documents.json conformance/conventions/requests/sign-in-hold/fixture.json conformance/conventions/inventory conformance/conventions/setup conformance/language engine/crates/shell tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a4-declared.yaml: 4 shapes.`,
then `Vector delta: N changes; M flips; 0 undeclared; 0 errors.` and `exit 0`; report N and M, and
that every change outside the added case lies in the two cases of decision 9.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. No promise's text changes, so no row of the model-checking account goes stale; where
`check_model_account.py` reports one stale, stop and report it. The unit tests, run with the
interpreter that has PyYAML, end `OK`; report the count. The `git status` command prints nothing.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so
the local gate runs it for this change, as spec batch 1's Gate G asks. From
`experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`.

Each of these prints 1. The `tr` check reads the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
grep -c 'fn sign_in_undone_condition_and_hold' engine/crates/core/src/sign_in.rs
grep -c 'fn sign_in_action_raised_by_setup' engine/crates/core/src/pending.rs
grep -rl '"dispatch-account-check"' engine/crates/core/src | wc -l
grep -rl '"hold-for-sign-in-undone"' engine/crates/core/src | wc -l
grep -rl '"report-sign-in-undone"' engine/crates/core/src | wc -l
grep -c '"id": "requests-sign-in-setup-account"' conformance/conventions/requests/index.yaml
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Setup checks the account (PND-011, TPL-012, order 048a-4)'
grep -c '^| PND-005 | convention | must | 3 | 0 | outside | core | case passes |$' engine/STANDING.md
grep -c '^| PND-011 | convention | must | 1 | 0 | outside | core | case passes |$' engine/STANDING.md
```

Each of these prints 0:

```bash
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 10 names with the totals, the function names and signatures of
decisions 2 to 6, the line in `advance` where the account check is begun and the line in
`invoke_inner` where both holds are decided, and whether any path reaches the cases decisions 2
and 3 ask about.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a4-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
