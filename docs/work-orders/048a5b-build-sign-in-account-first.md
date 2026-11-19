# Work order 048a5b (E7a-5b): `sign-in.v1` checks the account first, and the expected account signs the instance in

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

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry for 048a-2 gives 048a-5, "`sign-in.v1`
dispatched, with IFC-049's relay and polling", and the coordinator cuts it into eight orders, each
after the one before, since an order holds one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)); order 048a-5's
text (its file `048a5-build-sign-in-refusals-before-parts.md`) records the cut under "Where it
sits". This is the second, 048a-5b: "`sign-in.v1` dispatched, its account check first, and the
signed-in answer, with C11's stub removed", with an interim ending for no account or another.
048a-5 refuses a request before any part; this order dispatches the first part. Later orders build
the rest: 048a-5c, PND-010's fixed-port refusal; 048a-5d, the `sign-in-hook.v1` part and IFC-049's
URL match; 048a-5e, the engine's message and the input-required output that order 048b-relay1
relays, which replaces this order's interim ending; 048a-5f, polling on the retry; 048a-5g,
another account undone during polling; and 048a-5h, DL18's end. The shell issues, keeps and checks
a sign-in's requestState (order 048b-relay1), and a retry reaches the core only as its `respond`
input; this order reads neither.

**What it builds.** One behavior: `sign-in.v1` runs, and its first part, the account check,
decides how it ends. PND-010's `observable`: "The engine first dispatches account-hook.v1 as a
part, before sign-in-hook.v1. If that check succeeds with the expected account, compared ignoring
ASCII case, sign-in.v1 succeeds with output account and answers any sign-in action signed-in."
PND-011's `then`: "Only the expected account, compared ignoring ASCII case, completes sign-in" and
"A failed or unknown account hook does not complete sign-in, and its uncertainty is explained".
PND-011's `observable`: "Only a later account-hook.v1 part within sign-in.v1 or setup that reports
the expected account or none clears that condition", `SignInUndone=False`. PND-005's `when` names
"the account-hook.v1 part that sign-in.v1 dispatches before sign-in-hook.v1 reports a different
account" as raising the instance's sign-in action, "unless one is unanswered", and its `then`
holds the instance "until sign-in.v1 answers that it is signed in". PND-003: "Sign-in is answered
signed-in by sign-in.v1". No promise changes: the texts state all of it, and this order builds it.

**What it binds first.** C11's stub has so far answered every `sign-in.v1` that 048a-2's caller
gate passes over. Removing it would dispatch those requests, and the access tier does not stop
them: `access::routing` has no row for `sign-in.v1`, so a Conduit to any live Fleet or Pool that
lists it passes, whatever instance the request names. So before the stub goes, decision 0 binds
the instance at the access tier, as ACC-033's row 16 reads ("Signing in with sign-in.v1: the
subject Agent's owning Fleet or the subject Resource's owning Pool, with the Interface-only and
scoped limits of PND-010"). PND-010: "A caller other than root signs in only an Agent of a Fleet
in its scope; Resources remain root's". OPS-023: "A request that is not permitted reports refused
whatever else is wrong with it". This is the fix that order 048a-2's Record, R1, names as its
reader's preferred one. It is a decision of its own, a second behavior that this order holds
beside the first, so FR-264's one-behavior rule is waived for this order (below, "Who decided
what"): who may ask is decided first. Decisions 1 to 14 then build the account check for the
requests that access permits.

**The interim ending, which 048a-5e replaces.** PND-010 says that a check finding no account or
another account goes on: "the engine invokes sign-in-hook.v1 as a part and relays its URL
elicitation under IFC-049; that initial mismatch does not undo the account or end sign-in.v1".
That hook part is 048a-5d's, and the relay 048a-5e's. Until 048a-5e, this build has no step to go
on with, so here such a check ends `sign-in.v1` failed, with a diagnostic naming PND-010 and that
the relay is order 048a-5e's (decision 4), after another account has raised the sign-in action
where PND-005 asks it. This order states it so in `engine/CONFORMANCE.md` (decision 10), and
048a-5e removes the ending and that sentence. It is an unbuilt step, recorded as the file records
others, not a reading of PND-010; no release carries it, since the Agent packages 0.2.0 merge only
after order 048b-join, and no real sign-in runs before.

**MC9a, which this order follows.** [Order 201](201-mc9a-pending-actions-sign-in-registered.md)
registers MC9a. Its property K11 (a) reads: "every `sign-in.v1` of A that ends succeeded, and
every answer signed-in, comes after an `account-hook.v1` result within that invocation whose
account equals `E` ignoring ASCII case; and an uninterrupted `sign-in.v1` of A whose check or
polling finds an account equal to `E` ignoring ASCII case ends succeeded with that account as
output and answers signed-in the sign-in action of A unanswered at its end, where there is one".
So the output `account` is the account the check reported, in its own case (decision 3). H05 holds
the instance "from the raising of a sign-in action of A to the next `sign-in.v1` of A that ends
succeeded", and its finding MC9a-003 reads the hold as lifted by that success "whatever its
action's state", the stand order 048a-3 took and this order builds: a success lifts the hold even
where the action expired and nothing is left to answer. Its finding MC9a-002 reads that "an `O`
found by a sign-in dispatched before a retirement that has since been dispatched raises no
action"; this order follows it (decision 4).

**What orders 048a-2 to 048a-5 bring, which this order assumes.** Orders 048a-2 (#364), 048a-3
(#367), 048a-4 (#371, its file `048a4-build-sign-in-setup-account.md`) and 048a-5 (#382, its file
`048a5-build-sign-in-refusals-before-parts.md`) are merged, each with its Record. This order
starts from 048a-5's work (below). It assumes, as their decisions, and the amendments their
Records accept, state them:

- From 048a-2's decision 4: the sign-in kind stored in 047c's `pending_actions`, keyed by action
  ID, each value `{"action": A, "dispatched": null, "raisedBy": I}`, `A` in `pending-action.v1`'s
  form; 047c's `store_answer`, which keeps an answer in that entry, and `invocation_answer`, which
  writes `{"value": V, "actor": <the invocation's caller>, "invocation": <its ID>, "at": X}`
  (`pending.rs`); and `record_answer`, which writes the record `pending-action-answered`.
- From 048a-2's decision 5: `engine/crates/core/src/sign_in.rs`, with the function that gives an
  instance's expected account (no `signIn`, an account that is not a string, or the pointer and the
  string), called "the account function" below; and, in `pending.rs`, the raise, which first tests
  whether the instance holds a sign-in action with no answer at that time and, where it does,
  allocates nothing, stores nothing and gives none.
- From 048a-2's decision 6: the preflight of `sign-in.v1`, whose conflict reads an invocation in
  state `in-progress` whose `object` is the instance's ID and whose `operation` is the core's
  `sign-in.v1`; and C11's stub, `("sign-in.v1", "048a", "PND-010")` in `invoke_inner`, which this
  order removes.
- From 048a-2's Record: the preflight (`sign_in::preflight`) applies its checks only for root, or
  for an Interface on its own behalf through an Agent's owning Fleet (R1), and gives none, passing
  the request on to the stub, for any other caller and for an instance it does not resolve; between
  its invalid and its conflict it reports conflict under LIF-037 for a stale `expectedId` (R3); the
  raise gives none for a retired instance (R4); and its case `requests-sign-in-lapse-and-conflict`
  has 32 steps, its step 29 (index 28) a `group.sign-in.v1` with a stale `expectedId` that expects
  LIF-037's conflict (R2, with 048a-4's change to its setup steps).
- From 048a-2's decision 7: retirement answers the instance's unanswered sign-in action `void`.
- From 048a-3's decisions 1, 2 and 4: `State`'s member `sign_in_holds`, keyed by instance ID; a
  raise begins the instance's hold; the one function in `pending.rs` that gives the instance's
  sign-in action with no answer at a time, or none, called "the unanswered function" below; and
  the held test with its block in `invoke_inner`.
- From 048a-3's decision 6 and 048a-4's decision 9: the case `requests-sign-in-hold`, its
  directory's `manifest.json`, `documents.json` and `fixture.json` (the Agent `signed` with the
  account `primary-account` at `/account`, the Operations `ping.v1` and `work.v1`, the second
  stating `startsModelWork: true`, root's Conduit `to-signed` listing both, and the Fleet `group`),
  its definitions `I`, `P`, SUCCESS and HELD(A), and its steps 1 to 11: first start,
  registration, the setup of `signed` (`$setup1`), its two run reports, the second dispatching the
  account check (`$account0`), that check's completion with the expected account, root's open of
  `to-signed`, root's `to-signed.work.v1` (`$call1`), its completion LAPSE, which raises the
  sign-in action and begins the hold, root's inspection of `signed`, which captures the action as
  `$signin`, and `to-signed.work.v1` again, held.
- From 048a-4's decision 2: the function in `requests.rs`, `setup_sign_in_hook` beside
  `setup_hook`, through their shared `dispatch_hook`, that dispatches `account-hook.v1` or
  `sign-out-hook.v1` as a part of a parent invocation, afresh (attempt 1, key null), with the
  parent's caller, sender and filters, `setup_parent` and `serving` the parent's ID, the reply
  `{"pending":true,"invocation":<parent>,"hook":<ID>}` and the decision `dispatch-account-check`
  or `dispatch-sign-out`, and that leaves the parent's phase to its caller; called "the part
  function" below. Where no implementation is resolved it dispatches nothing and gives `false`,
  and its setup caller, `dispatch_sign_in_part` (`lifecycle.rs`), records the part failed.
- From 048a-4's decision 3: `child_answered` (`lifecycle.rs`) handles a setup's phases
  `account-pending` and `undo-pending`, through `sign_in_part_answered`, reading the part's
  `account` from its `retained.outputs`.
- From 048a-4's decision 4: the raise's (`pending::raise_sign_in`) third form, a setup
  (`SignInRequest::Setup`, beside 048a-3's `Invocation` and `Held`), with its trigger, its
  `raisedBy` and its evidence items 2 to 4, and the setup's `invocation-ended` record naming the
  action in its `data`'s `pendingAction`.
- From 048a-4's decision 5: in `sign_in.rs`, the functions that set `SignInUndone` false and that
  clear it, `set_sign_in_undone_false` and `clear_sign_in_undone` ("Clear": only where the
  instance's condition is `False`, it becomes `True` with the reason `AccountChecked` and the
  message `An account check within setup found the expected account or none.`), and the decision
  `clear-sign-in-undone`, citing `PND-011` and `TPL-012`, where clearing changes the condition,
  which the clear's caller, `sign_in_part_answered`, pushes.
- From 048a-4's decision 6: PND-011's hold beside PND-005's in `invoke_inner`.
- From 048a-4's decision 8: the case `requests-sign-in-setup-account`, its definitions K(x),
  ACCOUNT(X), SIGN-OUT-FAILED and PART(H), and its steps 13 to 17: root's second setup of `signed`
  (`$setup2`), its run report dispatching the account check (`$account2`), that check finding
  `other-account` and dispatching the sign-out (`$signout2`), the sign-out failing, so that the
  setup ends unknown and `SignInUndone` is `False`, and root's inspection showing both actions.
- From 048a-4's decision 10: in DL19's row of `engine/CONFORMANCE.md`, the sentence that begins
  "Setup checks the account (PND-011, TPL-012, order 048a-4)" and ends "and the check within
  `sign-in.v1` is not built yet."
- From 048a-5's decisions 1 and 2: `AccessRequest`'s members `client_capabilities`,
  `sign_in_login` and `sign_in_return`, and `request_member_refusal` in `sign_in.rs`, so that a
  `sign-in.v1` request reaches dispatch only where it declares URL elicitation, carries no
  `signInReturn` and names no unknown login. This order reads none of the three members. Its
  Record names the preflight's helpers `stated_login_names`, `request_member_refusal`, `refusal`
  and `rejection`, the last writing every diagnostic of 048a-2's and 048a-5's refusals but
  LIF-037's as `{tool} reports {reason} under {rule}: {cause}`, which decision 14 changes.
- From 048a-5's decision 5: the case `requests-sign-in-refusals-before-parts`, of 18 steps, whose
  step 18 asks `group.sign-in.v1` for `signed` once nothing is in progress and expects the stub's
  unavailable; its definitions S, URL and "Root asks T with A and M".
- From 048a-5's decision 6: steps 4, 12 and 29 of `requests-sign-in-lapse-and-conflict` (indexes
  3, 11 and 28 of 32) declare URL elicitation; step 12 expects the stub's unavailable, and step 29
  LIF-037's conflict for a stale `expectedId`, as 048a-2 merged it.
- From 048a-5's decision 8, as 048a-2 to 048a-4 leave the file: in `engine/CONFORMANCE.md`, the
  words "where the checks of orders 048a-2 and 048a-5 apply, and otherwise unavailable" in three
  places (DL18's row, the paragraph on public engine execution, and the list of what the build
  does not claim), after 048a-2's "invalid or conflict", which in the last two also names LIF-037's
  conflict for a stale `expectedId` (048a-2's R3), and, in the section "Refusals and reasons
  (order 046a)", the sentence "The unavailable stub of `sign-in.v1` now carries a diagnostic but
  remains unbuilt." followed by 048a-2's and 048a-5's sentences; and in DL19's row 048a-3's words
  "and `sign-in.v1`'s answer is not built yet." and "and nothing in this build lifts a hold, since
  `sign-in.v1` does not yet answer."

Orders 048b-relay1 (#373) and 048b-relay2 (#377) have merged, and 048a-6 may land before this
one; it reads nothing of them. Since 048b-relay2, every call the core dispatches to an instance
whose Type states `signIn`, the account part's included, declares URL elicitation in its `_meta`
(IFC-045); PART(H) does not read that member. Where an order merged otherwise than this says,
apply the nearest sound reading of each decision below to what merged, and say so. Where a Record
names a function, use that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`9e2a1410`, after orders 048a-2 to 048a-5 and 048b-relay2; find each place by the text quoted
with it.

- `invoke_inner` (`engine/crates/core/src/requests.rs:919`) ends C11's stubs in the block that
  begins `// C11's temporary stubs: authority and arguments have already been checked.` (line
  1363), whose list holds `sign-in.v1` alone. `reject_diagnostics` (line 639) gives the stub's
  text in its arm `"PND-010" => Some(format!(` (line 665) with `not built yet, order 048a.`
  (line 666).
- Further on, `invoke_inner` refuses an engine operation that no transition handles, in the block
  ending `return Err("engine operation belongs to its lifecycle or records transition".into());`
  (line 1551), and then hands the new invocation to `crate::records::execute` or
  `crate::lifecycle::execute` or, for any other operation, pushes its dispatch intent (lines 1778
  and 1779, `let call = intent(&invocation, &t.state, c)?;` and
  `t.external_intents.push(call);`, 048b-relay2's form). The core package's implementation `engine`
  implements `sign-in.v1` (`spec/core/conduit.yaml`), so its invocation's `implementation` has
  `source` `engine`.
- `complete` (`requests.rs:1902`) ends a part as any call, validating a success's
  `structuredContent` against the operation's `outputSchema` (`account-hook.v1` states one whose
  `account` is a string or null) and keeping it in `retained.outputs`; then `child_ended` and
  `child_answered` (`lifecycle.rs:1181` and `1191`) reach the parent through `setup_parent`.
  `child_answered` returns at once where the parent is not `in-progress`.
- `finish` (`requests.rs:1821`) ends an invocation, writes its `invocation-ended` record and its
  reply through `ended_reply`, whose `structuredContent` holds the outputs where the contract
  states an `outputSchema`; `sign-in.v1` states one, requiring `account`, a string.
- `records::execute` (`records.rs:260`) answers an action for `answer.v1`: `store_answer`, then
  `finish`, then `pendingAction` added to the `invocation-ended` record's `data` in `t.records`
  and `t.state.records`, then `record_answer`, then its decision. The success of decision 3
  follows that order.
- `explain::outcome_diagnostic(invocation, outcome, rule, cause)` (`explain.rs:869`) builds an
  engine-decided outcome's diagnostic.
- `engine/crates/shell/src/server.rs`, in the comment at lines 40 to 42 above
  `EXECUTABLE_ENGINE_OPERATIONS`, says the list "includes temporary stubs that answer unavailable
  and name the order that builds each operation (OPS-005)".
- Of the cases on `main`, only `requests-sign-in-lapse-and-conflict` and 048a-5's
  `requests-sign-in-refusals-before-parts` ask `sign-in.v1` of an instance whose Type states
  `signIn`, and only one step of each meets the stub. In the first, it is step 12: its step 4
  gets 048a-2's conflict, its step 6 048a-2's invalid, and its step 29 LIF-037's (048a-2's R2
  and R3). In the second, it is step 18, its steps 3 to 14 being refused before dispatch. The
  shell test `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders` asks
  `system.sign-in.v1` for the Resource `management`, whose Type states no `signIn`, so 048a-2's
  invalid answers it before any dispatch, and it does not change here. Every `sign-in.v1` step
  of those cases is root's, through `group`, the Fleet that owns the instance it names, and the
  shell test's goes through `system`, the Pool that owns `management`; so decision 0 refuses none
  of them.
- `access::routing` (`engine/crates/core/src/access.rs:606`) resolves the named object in the
  arm `Some("set-up.v1" | "retire.v1" | "register-replacement.v1") => {` (line 624) and routes
  setup and retirement to its owner in the arm `Some("set-up.v1" | "retire.v1") => match named {`
  (line 646): the owning Pool or Fleet of a Resource or Agent, none where that owner is retired,
  and `management` for any other object or none. `sign-in.v1` falls to the default arm
  `_ => view` (line 702), the Conduit's own target. `evaluate_path` refuses under ACC-033 where
  the route is not the Conduit's target, in the block that begins
  `if route.is_none_or(|r| r["id"] != path["target"]) {` (line 879), after `let root =` (line
  847). Its OPS-029 list (lines 1088 to 1104) does not name `sign-in.v1`. Only
  `pool-management.v1` and `fleet-management.v1` list `sign-in.v1`, so no Conduit to
  `management` can.
- `explain::acc033_target` (`explain.rs:567`) gives an ACC-033 refusal's authorizing target. It
  has an arm `Some("set-up.v1" | "retire.v1" | "inspect.v1") => {` (line 642) and otherwise
  `_ if op.contains('#') => "the named instance itself".into(),` (line 719), which `sign-in.v1`
  reaches. `explain::tier` gives tier 1 to a sender other than root for `sign-in.v1`, in the
  list after the comment `// ACC-033 rows 9 (enrollment) and 16 are missing from routing.` (lines
  412 to 418).
- `sign_in::preflight` (`sign_in.rs:199`) returns `Ok(None)` for an instance it does not resolve
  (line 210) and, after the two comment lines that begin
  `// PND-010, OPS-023: only root, or an Interface on its own behalf through an Agent's` (lines
  212 and 213), for a caller other than root or an Interface on its own behalf through an Agent's
  owning Fleet (line 221). `sign_in_refusals_before_dispatch` asserts that it gives none for
  `through("w", "f2")`, `through("v", "f2")`, `through("s2", "f")` and the Resource `x` through
  `("v", "p")`.

**When this order starts.** Order 048a-5 has merged to `main` (#382), and with it 048a-2 to
048a-4. Before you change anything, check from the repository root that you find what they give, and
stop, change nothing and report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/048a5-build-sign-in-refusals-before-parts.md
grep -c '^## Record' docs/work-orders/048a4-build-sign-in-setup-account.md
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
grep -c 'not built yet, order 048a' engine/crates/core/src/requests.rs
grep -c 'fn request_member_refusal' engine/crates/core/src/sign_in.rs
grep -c 'pub sign_in_holds: BTreeMap<String, Value>' engine/crates/core/src/lib.rs
grep -rl '"dispatch-account-check"' engine/crates/core/src | wc -l
grep -rl '"clear-sign-in-undone"' engine/crates/core/src | wc -l
grep -c 'An account check within setup found the expected account or none.' engine/crates/core/src/sign_in.rs
grep -rl '"hold-for-sign-in-undone"' engine/crates/core/src | wc -l
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-hold/case.yaml"))["steps"]; print([x["input"].get("request", {}).get("tool", x["input"]["kind"]) for x in s[:11]])'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-setup-account/case.yaml"))["steps"]; print([x["input"].get("request", {}).get("tool", x["input"]["kind"]) for x in s[12:17]], len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml"))["steps"]; print(s[11]["input"]["request"]["tool"], "unavailable" in json.dumps(s[11]["expect"]), s[28]["input"]["request"]["tool"], "LIF-037" in json.dumps(s[28]["expect"]), len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-refusals-before-parts/case.yaml"))["steps"]; print(s[17]["input"]["request"]["tool"], "unavailable" in json.dumps(s[17]["expect"]), len(s))'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply, and otherwise unavailable' | wc -l
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'The unavailable stub of .sign-in.v1. now carries a diagnostic but remains unbuilt.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and .sign-in.v1..s answer is not built yet.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and nothing in this build lifts a hold, since .sign-in.v1. does not yet answer.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and the check within .sign-in.v1. is not built yet.'
grep -c 'It includes temporary stubs that answer unavailable and name the order that' engine/crates/shell/src/server.rs
test -d conformance/conventions/requests/sign-in-account-first; echo "dir $?"
grep -c 'Some("set-up.v1" | "retire.v1" | "register-replacement.v1") => {' engine/crates/core/src/access.rs
grep -c 'Some("set-up.v1" | "retire.v1") => match named {' engine/crates/core/src/access.rs
grep -c 'sign-in' engine/crates/core/src/access.rs
grep -c 'ACC-033 rows 9 (enrollment) and 16 are missing from routing.' engine/crates/core/src/explain.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c "gets the stub's unavailable"
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Refusals for .answer.v1., .sign-in.v1. and the enrollment operations stay at tier 1'
```

They print `1`, `1` (048a-5's and 048a-4's merges), `1`, `1` (the stub and its text), `1`
(048a-5's refusals), `1` (048a-3's holds), `1`, `1` (048a-4's part function and clear), `2` (the
clear's message, in the clear and in 048a-4's test `sign_in_undone_condition_and_hold`), `1`
(048a-4's PND-011 hold), then
`['first-start', 'register-project', 'group.set-up.v1', 'setup-report', 'setup-report', 'complete', 'access.open.v1', 'to-signed.work.v1', 'complete', 'group.inspect.v1', 'to-signed.work.v1']`,
`['group.set-up.v1', 'setup-report', 'complete', 'complete', 'group.inspect.v1'] 30`,
`group.sign-in.v1 True group.sign-in.v1 True 32`, `group.sign-in.v1 True 18`, `3`, `1`, `1`,
`1`, `1`, `1`, `dir 1`, then `1`, `1` (the two routing arms decision 0 widens), `0` (no row
for `sign-in.v1` in the access tier), `1` (the tier comment), `1` and `1` (the two sentences of
`engine/CONFORMANCE.md` that decision 10's places 6 and 7 change).

Then, still before any change, from the repository root keep the rows this order changes, and from
`engine/`, one at a time, run the runner for each family this order touches, keeping each last
line: "Done when" compares with them.

```bash
mkdir -p engine/target && grep -E '^\| PND-0(03|05|10|11) \|' engine/STANDING.md > engine/target/standing-048a5b-before.txt
```

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-core --lib -- sign_in
cargo test --release -p conduit-core --lib -- pending::tests
cargo test --release -p conduit-core --lib -- lifecycle
```

Orders that land beside this one (046b, 048a-6, 049a-1, 050 and others): whichever
lands second keeps both changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** PND-010, PND-011, PND-005, PND-003 and PND-012 decide that the account check
is the first part, that only the expected account compared ignoring ASCII case completes a sign-in,
with output `account` and the action answered signed-in, that a failed or unknown check completes
nothing, that a check finding the expected account or none clears `SignInUndone`, that another
account found there raises the action unless one is unanswered, and that the records name the action
raised or answered. MC9a's K11 and H05, registered by order 201, give the output as the account the
check reported and the hold as lasting to the next success. Everything else here is the
coordinator's, each open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):
decision 0, under PND-010 ("a caller other than root signs in only an Agent of a Fleet in its
scope"; Resources remain root's) and OPS-023 (access is decided first), as ACC-033's row 16 reads,
which binds `sign-in.v1`'s instance in the access tier before the stub goes (order 048a-2's Record,
R1, its reader's preferred fix), with its choices: refusing row 16's limits under ACC-033 as well as
its route, a cause that resolves nothing, keeping `sign-in.v1` at tier 1 for a sender other than
root, and keeping 048a-2's caller gate as a second line that fails with an error rather than
dispatch, pinned by the unit test's assertion that the preflight gives `Ok` for each request access
permits, with the unit test, its two mutation steps and the change to the list of
`unimplemented_authorizing_routes_keep_non_root_refusals_at_tier_one`; keeping decision 0 in this
order rather than in an order of its own, which waives FR-264's one-behavior rule for this order;
the cut of 048a-5 that puts this behavior here (order 048a-5's "Where it sits"); the interim failed
ending for no account or another, with its diagnostic and its sentence in `engine/CONFORMANCE.md`,
until 048a-5e; ending `sign-in.v1` failed, not unknown, where its account check ends failed or
unknown, since no part with an effect has run; the stand on MC9a-003 that a success lifts the hold
whatever its action's state (order 048a-3's); following MC9a-002's reading that another account
found while a retirement of the instance is in progress or done raises nothing; ending `sign-in.v1`
failed under PND-003, dispatching no part, where the instance is already retired when `begin` runs
("A retired sign-in subject answers void"); ending it failed under PND-003 where the instance is
retired, or removed by a retirement, when the check ends, leaving its hold entry as it is; keeping
the instance's ID in the invocation's `object` without marking it `management`, so that REV-014 does
not refuse a setup or retirement while a `sign-in.v1` is in progress, as MC9a-002 reads the text as
written; the invocation's member `sign_in`, its phases and its parts; reusing 048a-4's part
function, its raise and its clear, each widened by one parameter or form; the diagnostics' words;
the decisions' names; the function names; the unit tests; the case, its reuse of 048a-3's fixture
and of steps of 048a-3's and 048a-4's cases, its route, IDs and times; the one step changed in each
of 048a-2's and 048a-5's cases, and the expect-only checks added to 048a-5's refused steps (decision
9, order 048a-5's reader's finding R3); the edits to `engine/CONFORMANCE.md` and to the shell's
comment; and, under ruling 52 and IFC-050, naming the instance in every diagnostic of `sign-in.v1`'s
preflight, with the two unit tests that pin those texts, and in its failed endings through
`named_reference` (decision 14, order 048a-5's reader's finding R2), which changes no case's text.
Apply them as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-001, PND-003, PND-004, PND-005, PND-010, PND-011 and PND-012
   (`spec/promises/pending.yaml`); ACC-033, its rows 4, 4A and 16 above all
   (`spec/conventions/access.yaml`); OPS-023 (`spec/conventions/operations.yaml`); `sign-in.v1`
   and `account-hook.v1` (`spec/core/operations.yaml`); IMP-007
   (`spec/conventions/implicit.yaml`); DL18 and DL19 (`spec/implementation-defined.yaml`).
2. Orders [048a2](048a2-build-sign-in-lapse-and-conflict.md) (decisions 4 to 7, and its Record's
   R1), [048a3](048a3-build-sign-in-hold.md) (its stand on MC9a-003 and decisions 1 to 4),
   [048a4](048a4-build-sign-in-setup-account.md) (decisions 2 to 6, 8 and 10) and 048a-5 (its
   file `048a5-build-sign-in-refusals-before-parts.md`, decisions 1, 2, 5, 6 and 8), and the
   Records of the four.
3. In `experiments/012-model-checking/README.md`, MC9a's properties K11 and H05 and its findings
   MC9a-002 and MC9a-003.
4. The cases `requests-sign-in-hold`, `requests-sign-in-setup-account`,
   `requests-sign-in-lapse-and-conflict` and `requests-sign-in-refusals-before-parts`.
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M, at its upper end**: about 0.45 to 0.55k changed lines of Rust with three unit tests,
decision 0's being about 0.1k of them; the texts of two existing unit tests and four assertions of
one; one new case of 32 steps on 048a-3's fixture; one step's expectation changed in each of two
cases; expect-only checks added to eleven refused steps of one of them; two routing arms widened,
one access block, one arm of `acc033_target`, one comment in the core and one test's list in
`explain.rs`; one arm added to `named_reference`; seven places in `engine/CONFORMANCE.md`; and one
comment in the shell.

## The decisions it applies

0. **Access decides first who may ask `sign-in.v1`.** This is the coordinator's decision, under
   PND-010 and OPS-023 as ACC-033's row 16 reads them (above, "What it binds first"), open to the
   owner's strike. Apply it before any other decision of this order. Line numbers are those of
   `9e2a1410`.
   1. **The route.** In `engine/crates/core/src/access.rs`, in `routing`, add `| "sign-in.v1"`
      to the named arm that resolves `args["instance"]` (line 624, beside `set-up.v1`,
      `retire.v1` and `register-replacement.v1`) and to the route arm that matches the named
      object (line 646, beside `set-up.v1` and `retire.v1`). The route of a `sign-in.v1` is then
      the named Agent's owning Fleet or the named Resource's owning Pool, none where that owner
      is retired, and `management` for any other named object or for none. No Conduit to
      `management` lists `sign-in.v1`, so line 879's check refuses such a request under ACC-033.
   2. **Row 16's limits.** In `evaluate_path`, right after the block at lines 879 to 881, add:

      ```rust
      // ACC-033 row 16, PND-010: an Interface on its own behalf; a Resource is root's.
      if op == context.qualified("sign-in.v1")
          && (request.serving.is_some()
              || instance["kind"] != "Interface"
              || (instance["id"] != root["id"] && named.is_none_or(|o| o["kind"] != "Agent")))
      {
          return Ok(Verdict::reason("refused", "ACC-033"));
      }
      ```

      `instance` is the requesting instance and `root` the binding at line 847. A request serving
      an invocation is not on its own behalf, root's included, as ACC-032 reads root at line 849.
      Without this block, the route of part 1 alone permits an Agent, an Interface serving
      another caller, and a caller other than root with a Conduit to a Resource's owning Pool.
   3. **The cause, which resolves nothing.** In `engine/crates/core/src/explain.rs`, in
      `acc033_target`, add right before the arm at line 642 the arm
      `Some("sign-in.v1") => "the named Agent's owning Fleet, from an Interface acting on its own behalf; a Resource only from root, through its owning Pool (PND-010)".into(),`
      so that every refused `sign-in.v1` gives the same cause, whatever it names and whether or
      not that exists. Without it, the cause falls to line 719's `the named instance itself`.
      `sign-in.v1` stays in `tier`'s list at lines 414 to 418, so a sender other than root gets
      tier 1, whose refusal names no target, no Conduits and no `exists`. Only that list's
      two-line comment at lines 412 and 413 changes, to these three lines:

      ```rust
      // ACC-033 row 9 (enrollment) is missing from routing, so the default Conduit target cannot
      // establish disclosure; row 16 (sign-in.v1) stays at tier 1 so that a refusal discloses
      // nothing of the named instance (PND-010, OPS-023).
      ```

   4. **What each caller now gets.** Each of these is refused under ACC-033 at access, before any
      preflight. The reply carries OPS-023's refusal: ACC-056's one diagnostic, with part 3's
      cause, at tier 1 for a sender other than root, whose refusal names no account state, work
      in progress or ID beyond what the request itself named; root's names its Conduit's target,
      its Conduits to it and whether the named instance exists, as any tier-2 refusal does.
      - An Interface through a Conduit to a Fleet that does not own the named Agent (another
        Fleet's Conduit): the route is the owning Fleet, so line 879 refuses it.
      - An Agent caller: line 879 through any other Fleet, and part 2 (not an Interface) through
        the owning Fleet.
      - A caller other than root for a Resource: line 879 through any other Pool, and part 2
        through the owning Pool.
      - An Interface not on its own behalf, that is, a request serving an invocation: part 2.
      - An unresolved instance, or one naming an Interface, Pool, Fleet or template: the route is
        `management`, so line 879 refuses it. That is what `set-up.v1` and `retire.v1` get today
        through a Pool's or Fleet's Conduit. Access refuses it first, and the OPS-029 list at
        lines 1088 to 1104 does not name `sign-in.v1`, so OPS-029 is never reached for it.
      - An Agent or Resource whose owner is retired: no route (rows 4 and 4A, and ACC-033's "No
        request is authorized against a retired Pool or Fleet"), so line 879 refuses it.

      Root on its own behalf through the owning Fleet or Pool, and an Interface on its own behalf
      through an Agent's owning Fleet, are permitted as before and reach 048a-2's and 048a-5's
      preflight. Root through a Conduit to another Fleet or Pool is now refused under ACC-033 at
      tier 2, whose refusal names its Conduit's target, its Conduits to it and whether the named
      instance exists, and whose cause (part 3) describes the authorizing target; before, the
      preflight checked it. No case and no shell test asks so (above, "What the engine already
      does").
   5. **048a-2's caller gate becomes a second line.** After parts 1 and 2, access permits exactly
      the callers that `sign_in::preflight`'s gate (lines 212 to 222) checks, and the instance
      always resolves. So the preflight's two `return Ok(None);` for those requests are
      unreachable: line 210, for an instance it does not resolve, and line 221, for a caller it
      passes over. Keep both as a second line, but make them stop passing the request on: once
      the stub is gone, `Ok(None)` would dispatch it. Each becomes
      `return Err("sign-in.v1 reached its preflight from a caller or for an instance that access does not permit (ACC-033 row 16, PND-010)".into());`.
      Lines 212 and 213's comment becomes these two lines:

      ```rust
      // PND-010, ACC-033 row 16: access permits only root, or an Interface on its own behalf
      // through an Agent's owning Fleet; should another request reach here, fail, never dispatch.
      ```

      In `sign_in_refusals_before_dispatch`, the four assertions that `preflight` gives none (for
      `through("w", "f2")`, `through("v", "f2")`, `through("s2", "f")` and the Resource `x`
      through `("v", "p")`) become assertions that it gives an error. Nothing else in that test
      changes.
   6. **The test.** In `explain.rs`'s `mod tests`, add
      `sign_in_access_refuses_out_of_scope_callers_without_detail`, built from `fixture()` and
      driving `access::decide` as
      `operation_identification_names_the_package_operation_on_a_name_collision` does:
      - Add `sign-in.v1` to the operation contracts the test's state resolves. Build it within
        the test if adding it to `fixture()` would change another test.
      - Add the Fleets `f` and `f2`, the Pool `p`, the Interfaces `v` and `w`, the Agent with ID
        `signed-instance-id`, name `signed` and owner `f`, the Agent `s2` with owner `f`, and the
        Resource `x` with owner `p`, each as `fixture()` writes its objects.
      - Add open Conduits listing `sign-in.v1`, each with the local name its target's name: `r-f`
        from `root` to `f`, `r-p` from `root` to `p`, `v-f` from `v` to `f`, `v-f2` from `v` to
        `f2`, `v-p` from `v` to `p`, and `s2-f` from `s2` to `f`.
      - Add an in-progress invocation `call` whose target is `v` and caller `w`, through an open
        Conduit `w-v` from `w` to `v`, so that a request from `v` with `serving` `call` is not on
        its own behalf.

      With `access::evaluate`, it asserts that these are permitted (no reason): root's
      `f.sign-in.v1` naming `{"kind": "Agent", "name": "signed"}`, root's `p.sign-in.v1` naming
      `{"kind": "Resource", "name": "x"}`, and `v`'s `f.sign-in.v1` naming `signed`. For each of the
      three, it also asserts that `sign_in::preflight`, given the `route` of the `access::evaluate`
      verdict that permitted it, with its member `tool` set to the request's tool as `invoke_inner`
      sets it (`access_route`'s route has no `caller`, so the gate would pass every request over),
      gives `Ok`, not `Err`: a rejection within `Ok`, such as an invalid for a Type that states no
      `signIn`, is enough, since after part 5 a request that the gate passes over gives `Err`.
      That pins that access and the gate agree. With `access::decide`, it asserts that each
      of these is refused: `v`'s `f2.sign-in.v1` naming `signed`; `s2`'s `f.sign-in.v1` naming
      `signed`; `v`'s `p.sign-in.v1` naming `x`; `v`'s `f.sign-in.v1` naming `signed` with `serving`
      `call`; and `v`'s `f.sign-in.v1` naming `{"kind": "Agent", "name": "nobody"}`. For each
      refusal, the reply's `reason` is `refused`, its one diagnostic's `ruleId` `ACC-033`, its
      `properties.refusal.tier` 1, and its `properties.refusal` has no member `target`, `conduits`
      or `exists`. Its text ends with part 3's cause and a full stop. Its serialized reply contains
      none of `signed-instance-id`, `account`, `in progress`, `conflict` and `invalid`. If the
      fixture cannot publish `sign-in.v1` on a Fleet or Pool, stop and report it.
   7. **The test whose premise no longer holds for `sign-in.v1`.** In `explain.rs`'s test
      `unimplemented_authorizing_routes_keep_non_root_refusals_at_tier_one`, delete `"sign-in.v1"`
      from the list its `for op in [` loop reads, since its ACC-033 route is now built, and add
      right above that loop these two comment lines:

      ```rust
      // sign-in.v1's route is built (order 048a-5b); its tier 1 is pinned by
      // sign_in_access_refuses_out_of_scope_callers_without_detail.
      ```

      Nothing else in that test changes.

   Nothing else in `access.rs` or `explain.rs` changes. The opening of a Conduit that lists
   `sign-in.v1` for a consumer that is not an Interface stays unbuilt (decision 13); part 2 refuses
   such a consumer's request instead.

1. **`sign-in.v1` dispatches, and C11's stub goes.** In `invoke_inner`, delete the block of C11's
   stubs whole, its two comment lines included (from `// C11's temporary stubs` to its
   `return Ok(());` and closing brace), and in `reject_diagnostics` delete the arm
   `"PND-010" => Some(format!(` whose text ends `not built yet, order 048a.`. Where any call of
   `reject` other than `reject_diagnostics` still passes the rule `PND-010` with no diagnostics,
   stop and report it. In the block that refuses an engine operation no transition handles, add
   `&& op != c.qualified("sign-in.v1")` to its condition. In the chain that hands the new
   invocation on, add, right before its last `else` (the dispatch intent), the arm
   `else if op == c.qualified("sign-in.v1") { crate::sign_in::begin(t, a, c, i)?; }`. Nothing
   else in `invoke_inner` changes: the access decision (with decision 0), 048a-2's and 048a-5's
   preflight, the record `invocation-dispatched` and the decision `dispatch-invocation` come
   first, as for any operation.

   In `sign_in.rs`, add
   `pub(crate) fn begin(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()>`,
   which, for the invocation at `i`:
   1. Resolves `arguments.instance` with `access::object` and calls the account function for it.
      Where it resolves no instance or the account function gives no string, it ends the
      invocation (decision 4's end) failed under `PND-010`, and returns. Decision 0 and 048a-2's
      preflight leave neither reachable from a request: access refuses an unresolved instance,
      and the preflight, which now checks every request access permits, refuses an account that
      is not a string as invalid. Report whether any path reaches each. The cause, where it
      resolves no instance, is
      `The request's instance names no object, so sign-in.v1 has no account to check and signs nothing in.`,
      and, where the account function gives no string,
      `The instance's expected account is not a string, so sign-in.v1 cannot check it and signs nothing in.`
   2. Where the instance's `state` is `retired`, it ends the invocation (decision 4's end) failed
      under `PND-003` with the cause
      `The instance is retired, so sign-in.v1 signs nothing in and dispatches no part.`,
      dispatches no part, and returns. This is the coordinator's choice under PND-003 ("A retired
      sign-in subject answers void"). An instance is retired as soon as `retire.v1` is dispatched
      (LIF-005) and stays nameable until it is removed (LIF-007), so after a retirement that ended
      failed or unknown nothing is in progress, 048a-2's conflict does not apply, access permits
      root through the owning Fleet, and, with the stub gone, the request reaches here; before
      this order, the stub answered it unavailable and dispatched nothing.
   3. Sets the invocation's `object` to the instance's ID, so that 048a-2's conflict sees it in
      progress, and its member `sign_in` to `{"phase": "account-pending", "parts": []}`. It sets no
      `management`.
   4. Takes the instance's `resolved_implementations` at the qualified `account-hook.v1`. Where
      none is resolved, which CTR-013's checks (order 048a-1) leave no way to reach, it ends the
      invocation failed under `PND-011` with the cause
      `No implementation of account-hook.v1 is resolved for the instance, so its account cannot be checked and sign-in.v1 signs nothing in.`,
      and returns; report whether any path reaches it.
   5. Calls the part function for `account-hook.v1`, with the invocation at `i` as its parent and
      the instance as its object.

   The part function gains one parameter, the rules its decision cites: 048a-4's calls pass the
   lists 048a-4's decision 2 gives, unchanged, and `begin` passes `PND-010`, `PND-011`,
   `CTR-013`, `ACC-018` and `IMP-002`, with the decision's name `dispatch-account-check`
   unchanged. Where the part function reads or writes its parent's `setup` member on the path
   that dispatches a part, stop and report it: 048a-4's decision 2 leaves the parent's phase to its
   caller. Its no-implementation branch, which `begin`'s step 4 leaves unreachable for a
   `sign-in.v1` parent, may stay as 048a-4 wrote it. So the request's reply is the part function's,
   `{"pending":true,"invocation":<the sign-in.v1 ID>,"hook":<the part's ID>}`,
   and the part's dispatch intent is a setup part's: the requester's caller, no `conduit` key and
   no `idempotencyKey`.

2. **The check's answer reaches `sign-in.v1`.** In `child_answered`, right after the test that
   returns where the parent is not `in-progress` and before the test of its `retirement` member,
   where the parent's `operation` is the core's `sign-in.v1`, return
   `crate::sign_in::part_answered(t, a, c, i, v, outcome)`. In `sign_in.rs`, add
   `pub(crate) fn part_answered(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize, part: &Value, outcome: &str) -> Result<()>`.
   Where the parent's `sign_in.phase` is not `account-pending`, it returns and changes nothing:
   later phases are later orders'. Otherwise:
   1. It pushes `{"name": "account", "outcome": outcome, "invocation": <the part's ID>}` onto the
      parent's `sign_in.parts`.
   2. It finds the instance, the object whose ID is the parent's `object`. Where no object has
      that ID (a retirement that succeeded removed it while the check ran), it clears nothing,
      ends the invocation failed under `PND-003` with step 5's cause, and returns: it raises
      nothing, answers nothing, lifts no hold and leaves the instance's `sign_in_holds` entry as it
      is, as step 5 does; report whether any path reaches it. Otherwise it calls the account
      function for the instance. Where it gives no string (a `replace.v1` may have changed the
      account while the check ran), it ends the invocation failed under `PND-010` with decision
      1's step 1 cause for an account that is not a string, and returns; report whether any path
      reaches it.
   3. It decides the check with `first_check` (decision 5) from `outcome`, the part's
      `retained.outputs` and the expected account.
   4. Where the check is `Expected` or `NoAccount`, it calls 048a-4's clear on the instance at the
      step's time, with decision 6's `within` set to `sign-in.v1` (PND-011: "Only a later
      account-hook.v1 part within sign-in.v1 or setup that reports the expected account or none
      clears that condition", with no exception for a retirement).
   5. Where the instance's `state` is `retired`, it ends the invocation failed under `PND-003`
      with the cause
      `The instance was retired while its account check was in progress, so sign-in.v1 signs nothing in and answers no sign-in action.`,
      and returns: it raises nothing, answers nothing and lifts no hold.
   6. Otherwise, by the check: `Expected(found)`, decision 3 with `found`; `NotChecked`, it ends
      the invocation failed under `PND-011` with the cause
      `The account check before sign-in-hook.v1 ended {outcome}, so the account is not known and sign-in.v1 completes nothing; send sign-in.v1 again.`,
      `{outcome}` being the part's outcome; `NoAccount` or `Other`, decision 4.

   Nothing in this decision undoes an account or dispatches a further part.

3. **The expected account signs the instance in.** In `sign_in.rs`, add
   `pub(crate) fn answer_and_lift(state: &mut State, instance: &str, invocation: &Value, time: &str) -> (Option<Value>, bool)`,
   with a trailing `c: &Context` where the unanswered function needs one (report the signature):
   where the unanswered function gives the instance's sign-in action at `time`, it stores the
   answer `invocation_answer("signed-in", invocation, time)` with `store_answer` and gives the
   answered action first; and where `sign_in_holds` has an entry for the instance, it removes it
   and gives `true` second. It reads no action's expiry but through the unanswered function, so a
   success after the action expired answers nothing and still lifts the hold (MC9a-003). Then add
   `pub(crate) fn signed_in(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize, found: &str) -> Result<()>`,
   which 048a-5f calls again for polling, and which, in this order:
   1. Calls `answer_and_lift` for the parent's `object`, the parent invocation and the step's time.
   2. Ends the invocation with `finish`, outcome `succeeded`, outputs `{"account": found}`, the
      account as the check reported it, in its own case (MC9a's K11), content `[]` and no cause,
      and sets `sign_in.phase` to `ended`.
   3. Where an action was answered: adds `"pendingAction": [ID]` to the `data` of the
      `invocation-ended` record that `finish` wrote for the invocation, in `t.records` and in
      `t.state.records`, as `records::execute` does for `answer.v1` (PND-012); calls
      `record_answer` for the action; and pushes one decision named `answer-sign-in-signed-in`,
      citing `PND-003`, `PND-010`, `PND-011` and `PND-012`, with `DL19`, and the facts
      `{"action": ID}`.
   4. Where the hold was lifted: pushes one decision named `lift-sign-in-hold`, citing `PND-005`
      and `PND-010`, with the facts `{"instance": S}`, `S` the instance's ID.

   The reply is `finish`'s: `structuredContent` `{"account": found}`, and no `pendingAction` key.

4. **No account or another: the interim ending.** In `part_answered`, for `NoAccount` or
   `Other`:
   - For `Other` only, unless a `retire.v1` invocation in state `in-progress` has the instance's
     ID as its `object` (MC9a-002's reading), call the raise in its fourth form (below) with the
     trigger `sign-in-other-account`; where it gives an action, keep its ID in the invocation's
     `sign_in.raised`. `NoAccount` raises nothing: PND-005's `when` names only a different account
     for the check before `sign-in-hook.v1`.
   - Push one decision named `end-sign-in-before-relay`, citing `PND-010` and `PND-011`, with the
     facts `{"invocation": <the sign-in.v1 ID>, "found": F}`, `F` being `none` or `other`. It is
     the only place that writes that name, and 048a-5e removes it.
   - End the invocation failed under `PND-010` with the cause, for `NoAccount`,
     `The account check before sign-in-hook.v1 found no account; sign-in.v1 would go on to start a login through sign-in-hook.v1 and relay it under IFC-049 (PND-010), which this build does not yet do (order 048a-5e), so it ends failed and nothing was signed in.`,
     and for `Other`,
     `The account check before sign-in-hook.v1 found an account other than the expected one; sign-in.v1 would go on to start a login through sign-in-hook.v1 and relay it under IFC-049 (PND-010), which this build does not yet do (order 048a-5e), so it ends failed, that account stays as it is, and nothing was signed in.`
     Neither names an account (PND-004).

   **The end.** Decisions 1, 2 and 4 end the invocation through one function in `sign_in.rs`,
   given the Transition, the Allocation, the Context, `i`, the rule and the cause: it ends the
   invocation with `finish`, outcome `failed`, outputs `{}`, content `[]` and the cause one
   diagnostic built with `explain::outcome_diagnostic` for the invocation, `failed`, the rule and
   the cause; sets `sign_in.phase` to `ended`; and, where `sign_in.raised` holds an ID, adds
   `"pendingAction": [ID]` to the `invocation-ended` record's `data` as decision 3 does (PND-012).
   Add no row to `OUTCOME_CAUSES`.

   **The raise's fourth form.** Make the raise take a fourth form of raising request, beside
   048a-4's setup: a `sign-in.v1` invocation, given that invocation. Its trigger is
   `sign-in-other-account`, the stored entry's `raisedBy` is the invocation's ID (PND-001), and the
   evidence items are, in this order, each of kind `fact`:
   1. 048a-2's first item, the expected account, unchanged.
   2. `V` the invocation's ID; `W` `records`; `P`
      `the sign-in.v1 whose account check found an account other than the expected one`.
   3. `V` the invocation's `operation`; `W` `records`; `P` `the operation of that request`.
   4. `V` the trigger; `W` `records`; `P` `what the account check before sign-in-hook.v1 reported`.

   No item names the account the check found. Everything else follows 048a-4's decision 4: the
   unanswered test first, then the ID from `a.next`, the stored entry, the hold begun where the
   instance has none, and the decision `raise-sign-in-action`, here citing `PND-001`, `PND-005`,
   `PND-010` and `PND-012`, with `DL19`, and the facts `{"action": ID, "instance": S}`. The
   signature is your choice; report it, and report which of the raise's existing calls it
   changed.

5. **The check's reading, in one function.** In `sign_in.rs`, add
   `#[derive(Debug, PartialEq)] pub(crate) enum FirstCheck { Expected(String), NoAccount, Other, NotChecked }`
   and `pub(crate) fn first_check(outcome: &str, outputs: &Value, expected: &str) -> FirstCheck`:
   `NotChecked` where `outcome` is not `succeeded`, or where `outputs` has no member `account`
   that is a string or JSON null; `NoAccount` where it is null; `Expected` with the reported
   string where `found.eq_ignore_ascii_case(expected)` holds, written exactly so, on one line, and
   the only comparison of the two in the function; `Other` otherwise. It compares ASCII letters
   only, so a non-ASCII letter's case is never folded.

6. **The clear names where it ran.** 048a-4's clear gains one parameter, `within: &str`, and
   writes its message `An account check within {within} found the expected account or none.`;
   048a-4's calls pass `setup`, so that the message stays byte for byte as 048a-4 writes it, and
   decision 2 passes `sign-in.v1`. Where clearing changes the condition, the decision
   `clear-sign-in-undone` cites `PND-011` and `TPL-012` for a setup, as 048a-4 writes it, and
   `PND-011` and `PND-010` for `sign-in.v1`; where 048a-4 pushes it inside the clear, pass its
   rules as a parameter too. Otherwise `part_answered` pushes `clear-sign-in-undone`, citing
   `PND-011` and `PND-010`, with the facts `{"instance": S}`, where its call to the clear changed
   the condition.

7. **Two unit tests pin the rules.** Write the values as `json!` values; members a test does not
   need may be left out. Let T be `2026-11-12T12:00:00Z`.
   - In `sign_in.rs`'s tests, `sign_in_first_check_ignores_ascii_case`, which asserts with
     `assert_eq!`, with the expected account `primary-account`: `succeeded` with
     `{"account": "PRIMARY-ACCOUNT"}` gives `Expected("PRIMARY-ACCOUNT")`; with
     `{"account": "primary-account"}`, `Expected("primary-account")`; with `{"account": null}`,
     `NoAccount`; with `{"account": "other-account"}`, `Other`; with
     `{"account": "pr\u{0131}mary-account"}` (a dotless i), `Other`; with `{}`, `NotChecked`;
     `failed` with `{}` and `unknown` with `{"account": "primary-account"}`, each `NotChecked`.
   - In `sign_in.rs`'s tests, `sign_in_success_answers_and_lifts_the_hold`, with the objects of
     048a-3's test `hold_outlasts_expiry` in `pending.rs`, which 048a-4's test helper
     `sign_in_hold_fixture` there now gives (the Agent `s1` with the account `primary-account` at
     `/account`), rebuilt in `sign_in.rs`'s tests: that helper is private to `pending.rs`'s
     private `mod tests`, so leave it and its module as they are: 048a-2's raise at T for an
     invocation `c1` with the trigger `lapsed-login`, as that test calls it, gives an action A
     and sets `s1`'s hold to `{"since": T}`. On a copy of that state, `answer_and_lift` for `s1`,
     the invocation `{"id": "g1", "caller": "r"}` and T+1s gives A with the answer
     `{"value": "signed-in", "actor": "r", "invocation": "g1", "at": "2026-11-12T12:00:01Z"}` and
     `true`; the unanswered function then gives none at T+1s, `sign_in_holds` has no entry for
     `s1`, and the stored entry keeps its `raisedBy` `c1`; a second call at T+2s gives none and
     `false`, and changes nothing. On another copy of the state after the raise, `answer_and_lift`
     at A's expiry, `2026-11-19T12:00:00Z`, gives none and `true`: the action an ID names then has
     the answer `expired`, and `sign_in_holds` has no entry for `s1` (MC9a-003).

8. **One portable case, on 048a-3's fixture.** Add the directory
   `conformance/conventions/requests/sign-in-account-first/` with one file, `case.yaml`: ID
   `requests-sign-in-account-first`, `layers: [convention, interface]`,
   `promises: [PND-010, PND-011, PND-005, PND-003]`, title
   `Sign in by checking the account first: the expected account, compared ignoring ASCII case, signs the instance in, answers its sign-in action and lifts its hold; a failed check completes nothing; no account or another ends failed until the relay is built`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-hold/fixture.json", "manifest": "../sign-in-hold/manifest.json", "documents": "../sign-in-hold/documents.json"}`,
   048a-3's case's three files, unchanged. Write it as JSON with two-space indentation and a final
   newline, its members in the order of `requests-sign-in-hold`. It must meet 046a's portable-case
   rule as it stands.

   Below, ROOT, `META/x`, `U`, `I`, "Root asks T with A", "a completion", SUCCESS and HELD(A) are
   as 048a-3's decision 6 defines them; K(x), ACCOUNT(X), SIGN-OUT-FAILED and PART(H) as 048a-4's
   decision 8 does; and S, URL and "Root asks T with A and M" as 048a-5's decision 5 does. A new
   step's evidence is its time, `2026-11-12T12:00:00Z`, and, as 048a-3 writes its new steps, `ids`
   the 100 strings `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
   110000 + 100n + k, for k = 0 to 99, `revisions` the 16 strings `account-first-` followed by the
   decimal of n, a `-` and the decimal of j, and `suffixes` the 16 strings `f` followed by the
   four-digit, zero-padded decimal of 100n + j, for j = 0 to 15, where n is the step's number in
   this case; a copied step keeps its own evidence. More results and steps recur:
   - SIGN(N): Root asks `group.sign-in.v1` with S and URL. `expect` `{"reply": {"pending": true}}`
     with PART(`account-hook.v1`); `capture` `{"signN": "/reply/invocation", "checkN": "/reply/hook"}`,
     `N` the number given: dispatched, with the account check as its first part.
   - FAILED(N, RULE): `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/invocation": "$signN", "META/unanswered": 2}, "structuredContent": {"diagnostics": [{"ruleId": RULE, "level": "error"}]}}}`;
   - HELD2(A): HELD(A) with `"META/unanswered": 2` in place of 1;
   - INSPECT: root asks `group.inspect.v1` with `{"object": {"kind": "Agent", "name": "signed"}}`.

   Its steps:
   1. to 11. `requests-sign-in-hold`'s steps 1 to 11, `input`, `expect` and `capture`, whole, each
      with its evidence: the instance set up, its call's lapse raising the sign-in action
      `$signin`, and the hold.
   12. to 16. `requests-sign-in-setup-account`'s steps 13 to 17, `input`, `expect` and `capture`,
       whole, each with its evidence: a second setup finds `other-account`, its undo fails, the
       setup ends unknown with its setup action `$setup2`, and `SignInUndone` is `False`.
   17. SIGN(1).
   18. A completion for `$check1`, ACCOUNT(`null`). `expect` FAILED(1, `PND-010`): no account
       clears `SignInUndone` and, until 048a-5e, ends `sign-in.v1` failed; nothing is raised.
   19. INSPECT. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 2}, "structuredContent": {"status": {"conditions": [{"type": "Running"}, {"type": "SignInUndone", "status": "True", "reason": "AccountChecked"}], "pending": [{"id": "$signin", "kind": "sign-in"}, {"id": "$setup2", "kind": "setup"}]}}}}`.
   20. Root asks `to-signed.work.v1` with `{}`. `expect` HELD2(`"$signin"`): a failed
       `sign-in.v1` lifts no hold of PND-005.
   21. SIGN(2).
   22. A completion for `$check2`, ACCOUNT(`"other-account"`). `expect` FAILED(2, `PND-010`):
       another account is not undone, and no second action is raised while `$signin` is
       unanswered.
   23. SIGN(3).
   24. A completion for `$check3`, SIGN-OUT-FAILED. `expect` FAILED(3, `PND-011`): a failed check
       completes nothing.
   25. SIGN(4).
   26. A completion for `$check4`, ACCOUNT(`"PRIMARY-ACCOUNT"`). `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/invocation": "$sign4", "META/unanswered": 1}, "structuredContent": {"account": "PRIMARY-ACCOUNT"}}}`:
       the expected account, compared ignoring ASCII case, signs the instance in, with the account
       as reported.
   27. INSPECT. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"conditions": [{"type": "Running"}, {"type": "SignInUndone", "status": "True"}], "pending": [{"id": "$setup2", "kind": "setup"}]}}}}`:
       the sign-in action is answered, and only the setup action is left.
   28. Step 20's request. `expect` `{"reply": {"pending": true}}`: the hold is lifted, and model
       work is dispatched.
   29. SIGN(5).
   30. A completion for `$check5`, ACCOUNT(`"other-account"`). `expect` FAILED(5, `PND-010`): with
       no action unanswered, another account raises a new one.
   31. INSPECT. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 2}, "structuredContent": {"status": {"pending": [{"id": "$setup2", "kind": "setup"}, {"kind": "sign-in", "subject": [{"kind": "Agent", "name": "signed"}], "expiry": "2026-11-19T12:00:00Z"}]}}}}`;
       `capture` `{"signin3": "/reply/structuredContent/status/pending/1/id"}`. The actions are
       listed in byte order of ID, and step 30's ID follows `$setup2`'s.
   32. Step 20's request. `expect` HELD2(`"$signin3"`): the new action began the hold again.

   The route is new and has not been run, since orders 048a-2 to 048a-5 are not yet built. Before
   running it, check that no ID repeats across the case's steps' `ids`, the copied steps'
   included. If 046a's runner check refuses the case, if a step copied from another case gives a
   reply other than that case states, or if a step's reply differs from what it states, stop and
   report the error: do not change the expectations to pass it, and do not change the inputs
   beyond what this decision says. No expectation is generated from the engine's output. The
   coverage index entry holds `id`, `case` (`conventions/requests/sign-in-account-first/case.yaml`),
   `promises` and, where its neighbours in that piece hold it, `safety_rows: []`, in the members
   and order its neighbours use. Write it in a `cases` list in `conformance/manifest.yaml`, then,
   from `engine/`, run `cargo run --release -p conduit-conformance --example pieces` with no flag,
   which moves it into `conformance/conventions/requests/index.yaml` in ID order and leaves
   `manifest.yaml` byte for byte as it was.

9. **The two steps that met the stub.** Each now dispatches the account check, and the case's
   later steps, if any, do not change: the `sign-in.v1` it starts stays in progress to the case's
   end, and the one later step that asks `sign-in.v1` again, step 29 of
   `requests-sign-in-lapse-and-conflict` (048a-2's R2), keeps LIF-037's conflict, which 048a-2's
   preflight decides before its conflict for a `sign-in.v1` in progress. Change only each step's
   `expect`, which becomes `{"reply": {"pending": true}}` with PART(`account-hook.v1`), and add no
   `capture`:
   - `requests-sign-in-lapse-and-conflict`, its step 12 (index 11);
   - `requests-sign-in-refusals-before-parts`, its step 18 (index 17), its last.

   Since this order is the first that dispatches a part, it also takes up order 048a-5's Record's
   R3, so that a refusal of `requests-sign-in-refusals-before-parts` that began dispatching an
   account part would fail the case. In that case, with `META/x` and `Q` as 048a-5's decision 5
   defines them, and each an expect-only change:
   - each REFUSED expectation, steps 5, 6, 7, 8, 10, 12 and 14 (indexes 4 to 7, 9, 11 and 13),
     gains the member `"external_intents": []` right after `reply`;
   - each MISSING expectation, steps 3, 4, 9 and 13 (indexes 2, 3, 8 and 12), gains the member
     `"absent": ["/reply/inputRequired"]` right after `record_facts`;
   - steps 6 and 10 (indexes 5 and 9) each gain in their `absent` list, right after
     `"/records/1/data/signInReturn"`, the two pointers `/records/0/data/` and `/records/1/data/`,
     each followed by `META/signInReturn`, its `/` written `~1`, as `Q` is written: the qualified
     key, in record data.

   Nothing else in either case, and nothing in their directories' other files, changes. Both cases
   must then pass; if a later step of `requests-sign-in-lapse-and-conflict` gives another reply
   than its case states, or a step of `requests-sign-in-refusals-before-parts` fails on a check
   this decision adds, stop and report it: do not drop the check to pass it.

10. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each sentence by its text read
    across line breaks, as 048a-2 to 048a-5 leave it. Eleven phrases in seven places change (in
    place 1, the row's last sentence as well, as place 1 says); in each pair below, the first line
    is the phrase as it stands and the second what it becomes, and every other word of each line
    stays:

    1. In DL18's row in "Choices still required", in the same line of the table, its first words
       `Not built:` become `Partly built (order 048a-5b):`, and the words from
       `, and otherwise unavailable, with the text` to the words `no wait is chosen.`, the text
       `not built yet, order 048a` and its link to `requests.rs` included, become the second line
       below (the first is that phrase without its link); the row's last sentence, which 048a-2's
       R3 added, ``Where those checks apply, a stale `expectedId` reports conflict under LIF-037.``,
       stays after them, its first words reworded so that it reads
       ``Where the checks of orders 048a-2 and 048a-5 apply, a stale `expectedId` reports conflict under LIF-037.``:

       ```text
       , and otherwise unavailable, with the text "not built yet, order 048a" (its link to requests.rs), so no URL action waits and no wait is chosen.
       , and otherwise checks the account first, in `sign_in.rs` (order 048a-5b): an `account-hook.v1` part reports it, and the expected account, compared ignoring ASCII case, ends `sign-in.v1` succeeded with output `account`, answers the instance's unanswered sign-in action signed-in and lifts PND-005's hold, while a failed or unknown check ends it failed and completes nothing (PND-011); a check that finds the expected account or none sets a `False` `SignInUndone` to `True`. No `sign-in-hook.v1` part runs, so no URL action waits and no wait is chosen. Until order 048a-5e builds the relay, a check that finds no account or another ends `sign-in.v1` failed, with a diagnostic naming PND-010 and that order, where PND-010 goes on to `sign-in-hook.v1`; another account first raises the sign-in action unless one is unanswered (PND-005). This is an unbuilt step, not a reading of PND-010.
       ```

    2. In the paragraph on public engine execution:

       ```text
       , and otherwise unavailable, naming the order that builds it (048a; OPS-005).
       , and otherwise runs as far as its first account check (order 048a-5b; row DL18 in "Choices still required").
       ```

    3. In the list of what the build does not claim:

       ```text
       , and otherwise unavailable, naming order 048a (row DL18
       , and otherwise signs an instance in only where its first account check finds the expected account, starting no login (row DL18
       ```

    4. In the section "Refusals and reasons (order 046a)":

       ```text
       The unavailable stub of `sign-in.v1` now carries a diagnostic but remains unbuilt. Before it, order 048a-2
       `sign-in.v1` has had no stub since order 048a-5b. Before it dispatches any part, order 048a-2
       ```

    5. In DL19's row, in the same line of the table, three phrases:

       ```text
       and `sign-in.v1`'s answer is not built yet.
       and a `sign-in.v1` of the instance that succeeds answers it signed-in (order 048a-5b).

       and nothing in this build lifts a hold, since `sign-in.v1` does not yet answer.
       and only a `sign-in.v1` of the instance that succeeds lifts it, whatever its action's state (order 048a-5b; MC9a-003).

       and the check within `sign-in.v1` is not built yet.
       and the check that `sign-in.v1` sends first sets it `True` the same way (order 048a-5b).
       ```

    6. In the same section, the sentence on the callers 048a-2's checks pass over, which decision
       0 makes the access tier's (order 048a-5b's reader's M2):

       ```text
       They apply only for root, or for an Interface on its own behalf through an Agent's owning Fleet; any other caller, and any caller but root for a Resource, gets the stub's unavailable.
       Since order 048a-5b the access tier decides first who may ask (ACC-033 row 16, PND-010, OPS-023): root on its own behalf through the instance's owning Fleet or Pool, or an Interface on its own behalf through an Agent's owning Fleet. Any other request, one naming no Agent or Resource included, reports refused under ACC-033 before these checks, at tier 1 for a sender other than root, whose refusal names no account state, work in progress or ID; root's names its Conduit's target, its Conduits to it and whether the named instance exists, as any tier-2 refusal does. So the checks apply to every request that access permits.
       ```

    7. In the section "Tier 2 for inspectors and closed Conduits (order 046b)":

       ```text
       Refusals for `answer.v1`, `sign-in.v1` and the enrollment operations stay at tier 1 for senders other than root until their ACC-033 routes are built.
       Refusals for `answer.v1` and the enrollment operations stay at tier 1 for senders other than root until their ACC-033 routes are built. Those for `sign-in.v1` stay at tier 1 although order 048a-5b built its route, so that a refusal names no target, Conduit or existence of the instance (PND-010, OPS-023).
       ```

    DL18's and DL19's text stays one line in the file's table, as each row is now; places 2 to 4,
    6 and 7 may be rewrapped at 100 columns where their paragraphs are wrapped. If an earlier
    order merged one of these sentences otherwise, replace only the words that say `sign-in.v1` is
    a stub, answers unavailable, does not answer or is not built, or that its route is not built,
    and say so. Nothing else in the file changes.
    After the other decisions, run `python3 tools/promise_standing.py --write`, as AGENTS.md asks
    of a change to a case's `promises` and to a promise ID in the core's sources. Report the rows of
    PND-003, PND-005, PND-010 and PND-011 and the totals; the totals change only by what those rows
    change.

11. **The shell's comment.** In `engine/crates/shell/src/server.rs`, the two comment lines
    `// It includes temporary stubs that answer unavailable and name the order that` and
    `// builds each operation (OPS-005).` become
    `// None is a temporary stub since order 048a-5b; an operation not yet built would` and
    `// answer unavailable and name the order that builds it (OPS-005).` Nothing else in the shell
    changes.

12. **The declaration.** `docs/work-orders/048a5b-declared.yaml` holds six shapes: one `added` for
    `requests-sign-in-account-first`, one `flip` and one `expectation` for each of decision 9's
    two steps that met the stub, and one `expectation` for the checks decision 9 adds to the
    refused steps of `requests-sign-in-refusals-before-parts` (048a-5's R3). The coordinator wrote
    it with this order; check that it matches and change it only to make it so.

13. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
    record changes. No fixture changes. No `sign-in-hook.v1` or `sign-out-hook.v1` part runs
    within `sign-in.v1`, nothing is relayed or elicited, and nothing reads `signInLogin`,
    `signInReturn` or a requestState. The fixed-port refusal (048a-5c), the sign-in hook and the URL
    match (048a-5d), the message and the relayed output (048a-5e), polling, the undo during polling
    and DL18's end (048a-5f to 048a-5h), and the returns (048a-6 to 048a-8) are not built. No CLI
    command (051b), relay (048b-relay1, 048b-relay2, 048b-join) or package (050) changes. Report as
    findings, for the coordinator:
    - that `expectedId` on `sign-in.v1` (PND-010: "expectedId follows LIF-037") is checked only by
      048a-2's preflight (its Record's R3), which after decision 0 checks every request that access
      permits; report whether any path dispatches a `sign-in.v1` whose `expectedId` names another
      ID;
    - order 048a-5's Record's R4: a keyed `sign-in.v1` request that reuses a key with another
      digest gets OPS-010 conflict before the sign-in invalids, against OPS-023's order, from the
      preflight's placement that 048a-2 chose. Decision 0 binds the access tier's route and limits
      only, not the checks R4 would move before the key conflicts, so R4 stays open; report
      whether any path refuses such a request with an invalid first, and change nothing for it;
    - that PND-010's "A Conduit whose consumer is not an Interface cannot be opened listing
      sign-in.v1; the open reports refused" is still not built (048a-2's Record): decision 0
      refuses such a consumer's `sign-in.v1` request, not the open; report whether any path opens
      such a Conduit, and change nothing for it;
    - that `requests::cancel`, which the portable input `cancel` reaches, ends a `sign-in.v1`
      invocation as an external call (it has no `setup` member) and leaves its account part in
      progress, whose later answer `child_answered` then ignores; report whether any path reaches
      it, and change nothing for it;
    - that no promise states `sign-in.v1`'s outcome where its first account check ends failed or
      unknown, so `failed` is the coordinator's;
    - that no promise states `sign-in.v1`'s reply for an instance that is already retired, so
      decision 1's failed ending under PND-003, before any part, is the coordinator's;
    - that MC9a-002's one-sided exclusion stands: a setup or retirement asked for while a
      `sign-in.v1` is in progress is dispatched, since the invocation is not marked `management`;
    - that a successful `sign-in.v1`'s result names in no `pendingAction` key the action it
      answered, and a failed one none it raised, as 048a-3 and 048a-4 reported for a lapse and a
      setup;
    - that an `account-hook.v1` part within `sign-in.v1` that ends with a keyed PND-005 lapse
      diagnostic or a URL elicitation goes through `requests::complete`'s `finish_login_lapse`,
      which raises a sign-in action (`SignInRequest::Invocation`, the part as raiser) and begins the
      hold before `part_answered`'s `NotChecked` ending, so that the `sign-in.v1`'s
      `invocation-ended` record names no `pendingAction`; report whether any path reaches it;
    - as coordinator choices no check of this order tests: the `pendingAction` that the
      `invocation-ended` records name, the retirement and replacement arms of decision 2 and its
      arm for an instance a retirement removed, the missing implementation of decision 1, and the
      place of its retired arm before any part (a grep pins only that arm's text), and the raise
      skipped while a retirement is in progress, so that a build that omits them passes every
      check here;
    - and any place where PND-003, PND-005, PND-010 or PND-011 left you a choice this order does
      not settle.

14. **Every preflight diagnostic names the instance.** This is the coordinator's decision, under
    [ruling 52](../proposals/ruling-52/README.md) and IFC-050, open to the owner's strike; it takes
    up order 048a-5's reader's finding R2 (its Record). IFC-050 asks each diagnostic to say "the
    operation, what the request named, the reason or outcome, the deciding promise's ID and the
    cause in words". The texts of 048a-2's and 048a-5's refusals name the tool, the reason, the
    rule and the cause, but not the instance. In `sign_in.rs`, `rejection` writes all of them;
    make it write them, on one line, as
    `format!("{tool} reports {reason} under {rule} for {kind}/{name}: {cause}")`, `kind` and
    `name` being `instance["kind"]` and `instance["name"]` read as `&str` (so written unquoted,
    `Agent/signed`, not `"Agent"/"signed"`), `instance` the object the request's
    `arguments.instance` resolved to, which the preflight has resolved before any of them; where
    the instance has no top-level `name` that is a string, `name` is its `instance["id"]` read as
    `&str`. `rejection` and
    `request_member_refusal` each gain one parameter, that instance, which `refusal` passes;
    report both signatures. No cause, rule, reason, error code or other member changes, and
    LIF-037's conflict, which `rejection` does not write, keeps its text. So root's request for
    `signed` without URL elicitation reads
    `group.sign-in.v1 reports invalid under IFC-049 for Agent/signed: the request does not declare URL elicitation`
    and goes on as before.

    The unit tests that pin those texts change to match, and nothing else in them changes:
    - in `sign_in_request_member_refusals`, the five texts that begin
      `group.sign-in.v1 reports invalid under` (`missing_text`, `return_text`,
      `unknown_login_text`, `malformed_login_text` and the preflight's text for an instance whose
      Type states no `signIn`) gain `for Agent/agent-template` after the rule, and the test's
      calls of `request_member_refusal` pass an instance with that `kind` and `name`, as its
      state's object `t` holds them;
    - in `sign_in_refusals_before_dispatch`, `assert_refusal`'s text becomes
      `group.sign-in.v1 reports {reason} under PND-010 for Agent/signed: {cause}`, since every
      refusal it checks is for `signed`.

    No case pins those texts: the cases match the preflight's refusals by reason, rule and error
    code, and the shell test
    `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders` reads only that its
    text contains `the instance's Type states no signIn`. If any case, or any shell, CLI or
    experiment test, fails on this change, stop and report it. No promise, case or declaration
    changes for it, and `engine/CONFORMANCE.md` changes for it nowhere.

    The failed endings this order adds name the instance too. Their diagnostics, built by
    `explain::outcome_diagnostic` from the invocation's `named_subject`, would otherwise read
    `<core>#sign-in.v1 for tool group.sign-in.v1 ended failed under PND-010: ...`, since
    `named_reference` in `requests.rs` has no arm for `sign-in.v1`. Add to `named_reference`'s
    match the arm `"sign-in.v1" => ("instance", "Object"),`, so that they read
    `<core>#sign-in.v1 for Agent/signed via tool group.sign-in.v1 ended failed under PND-010: ...`.
    `named_reference` has two callers, both in `requests.rs`: the OPS-029 text in
    `reject_diagnostics` (line 701) and the invocation's `named_subject` (line 1559), which only
    `explain::outcome_diagnostic` reads. After decision 0 no OPS-029 refusal of a `sign-in.v1` is
    reachable, since access refuses an unresolved instance first. Before this order no
    `sign-in.v1` ends, since the stub refuses it before dispatch, so no outcome text of one
    exists. The arm therefore changes the text of no case: on `9e2a1410` no case, and no shell,
    CLI or experiment test, holds `named_subject` or an OPS-029 or outcome text for `sign-in.v1`.
    The sign-in cases match its refusals by reason, rule and error code, the other cases name it
    only in lists of operations or in other operations' texts, and the case of decision 8 matches
    its failed endings by rule, not text. Before the change, check that this still holds; if any
    case, or any shell, CLI or experiment test, pins such a text, stop and report it.

## Scope

Change only `engine/crates/core/src/access.rs` (decision 0: the two routing arms and row 16's
block), `explain.rs` (decision 0: the arm of `acc033_target`, the comment in `tier`, the test, and
the list and comment of `unimplemented_authorizing_routes_keep_non_root_refusals_at_tier_one`),
`sign_in.rs` (decision 0's second line and its test's four assertions, decisions 1 to 7 and 14),
`requests.rs` (decision 1: the stub, its text, the engine-operation guard, the dispatch arm and the
part function's parameter, and the raise's existing calls only where decision 4's signature reaches
them; decision 14: the `sign-in.v1` arm of `named_reference`), `lifecycle.rs` (decision 2's call in
`child_answered`, the part function's and the clear's calls, decisions 1 and 6, and the raise's
existing calls only where decision 4's signature reaches them), `pending.rs` (decision 4's fourth
form of the raise), `engine/crates/shell/src/server.rs` (decision 11's comment),
`engine/CONFORMANCE.md` (seven places), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/requests/index.yaml`, the new directory
`conformance/conventions/requests/sign-in-account-first/`, and the two files
`conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml` and
`conformance/conventions/requests/sign-in-refusals-before-parts/case.yaml` (decision 9).
`docs/work-orders/048a5b-declared.yaml` comes with this order; change it only as decision 12 says.
`conformance/manifest.yaml` holds the new entry only until the pieces tool moves it, and ends
unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, the rest of `engine/crates/shell/`,
the experiments, the case schema, `conformance/internal-cases.yaml`,
`conformance/expected-failures.yaml`, any other case or fixture, the other files of the case
directories named here, `docs/implementation/` or `docs/proposals/`: report any defect as a finding.
`experiments/011-short-form-spike/` changes only if its tests need it, since it depends on the core;
report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- sign_in
cargo test --release -p conduit-core --lib -- pending::tests
cargo test --release -p conduit-core --lib -- lifecycle
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cargo run --release -p conduit-conformance -- .. requests-sign-in-refusals-before-parts
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-account
cargo run --release -p conduit-conformance -- .. requests-sign-in-hold
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports three more tests passed than the start's run, 0 failed, the new ones being
decision 0's `explain::tests::sign_in_access_refuses_out_of_scope_callers_without_detail`,
`sign_in_first_check_ignores_ascii_case` and `sign_in_success_answers_and_lifts_the_hold`; report
both counts. The fourth and fifth pass as on the tree you start from; report each count. The pieces
tool prints `Nothing to move.` The next five runner commands each end `1 cases passed`;
`requests-sign-in-` and `requests-` each give one more case than the start's run, with the same
expected failures, each still failing; report each count and each expected failure. The shell test
passes: its `system.sign-in.v1` for the Resource `management` still reports 048a-2's invalid,
since access permits root through `system`, the Pool that owns `management`. If any other case,
or any shell, CLI or experiment test, fails because `sign-in.v1` now dispatches or because access
now refuses it, stop and report it: do not change it. The threads are 4 because at 24 the runner
ran out of memory (order 071's Record).

Next, show that the case fails without decision 5's comparison. From `engine/`, first keep a copy:

```bash
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
```

Then, by hand, in `crates/core/src/sign_in.rs`, in `first_check`, replace
`found.eq_ignore_ascii_case(expected)` with `found == expected`, and nothing else. Then, one at a
time:

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
cp target/sign_in.rs.fixed crates/core/src/sign_in.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with an error at
step 26, whose reply reports the outcome failed where succeeded is expected; after the restore the
second ends `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older
modification time, so cargo would not rebuild the crate. Git ignores `target/`.

Then show that decision 0's test fails without its route arm, and without row 16's block. From
`engine/`, keep a copy:

```bash
cp crates/core/src/access.rs target/access.rs.fixed
```

By hand, in `crates/core/src/access.rs`, delete ` | "sign-in.v1"` only from the route arm
`Some("set-up.v1" | "retire.v1" | "sign-in.v1") => match named {`, keep the named arm, and change
nothing else. Then, one at a time:

```bash
cargo test --release -p conduit-core --lib -- sign_in_access_refuses_out_of_scope_callers_without_detail
cp target/access.rs.fixed crates/core/src/access.rs
```

The test must fail at `v`'s request through `f2` (another Fleet's Conduit): permitted where
refused under ACC-033 is expected. With the named arm kept, the route is the Conduit's target, so
line 879 passes it, and part 2 passes it too, an Interface on its own behalf naming an Agent; the
request naming `nobody` is still refused, by part 2. So this step proves the route arm (line 646):
only it refuses another Fleet's Conduit. Next, by hand, delete row 16's block of decision 0's
part 2, its comment line included, and nothing else. Then, one at a time:

```bash
cargo test --release -p conduit-core --lib -- sign_in_access_refuses_out_of_scope_callers_without_detail
cp target/access.rs.fixed crates/core/src/access.rs
cargo test --release -p conduit-core --lib -- sign_in_access_refuses_out_of_scope_callers_without_detail
```

The first must fail, at `s2`'s request through `f`, at `v`'s request for `x` through `p` or at
`v`'s request serving `call`, permitted where refused is expected. After the restore, the last
reports `1 passed; 0 failed`. Report the line at which each failing run failed.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a5b-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/requests/sign-in-lapse-and-conflict/manifest.json conformance/conventions/requests/sign-in-lapse-and-conflict/documents.json conformance/conventions/requests/sign-in-lapse-and-conflict/fixture.json conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-setup-account conformance/conventions/inventory conformance/conventions/setup conformance/language tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5b-declared.yaml: 6 shapes.`,
then `Vector delta: N changes; M flips; 0 undeclared; 0 errors.` and `exit 0`; report N and M,
and that every change outside the added case lies in the steps decision 9 names.
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

Each of these prints 1. The `tr` checks read the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
grep -c 'fn sign_in_first_check_ignores_ascii_case' engine/crates/core/src/sign_in.rs
grep -c 'fn sign_in_success_answers_and_lifts_the_hold' engine/crates/core/src/sign_in.rs
grep -c 'found.eq_ignore_ascii_case(expected)' engine/crates/core/src/sign_in.rs
grep -c 'crate::sign_in::begin(t, a, c, i)' engine/crates/core/src/requests.rs
grep -c 'crate::sign_in::part_answered(' engine/crates/core/src/lifecycle.rs
grep -rl '"answer-sign-in-signed-in"' engine/crates/core/src | wc -l
grep -rl '"lift-sign-in-hold"' engine/crates/core/src | wc -l
grep -rl '"end-sign-in-before-relay"' engine/crates/core/src | wc -l
grep -c '"id": "requests-sign-in-account-first"' conformance/conventions/requests/index.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Until order 048a-5e builds the relay, a check that finds no account or another ends .sign-in.v1. failed'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'has had no stub since order 048a-5b'
grep -c 'None is a temporary stub since order 048a-5b' engine/crates/shell/src/server.rs
grep -c '^| DL18 | Partly built (order 048a-5b): ' engine/CONFORMANCE.md
grep -c '"{tool} reports {reason} under {rule} for {kind}/{name}: {cause}"' engine/crates/core/src/sign_in.rs
grep -c 'reports {reason} under PND-010 for Agent/signed: {cause}' engine/crates/core/src/sign_in.rs
grep -c '"sign-in.v1" => ("instance", "Object")' engine/crates/core/src/requests.rs
grep -c 'Some("set-up.v1" | "retire.v1" | "register-replacement.v1" | "sign-in.v1") => {' engine/crates/core/src/access.rs
grep -c 'Some("set-up.v1" | "retire.v1" | "sign-in.v1") => match named {' engine/crates/core/src/access.rs
grep -c '// ACC-033 row 16, PND-010: an Interface on its own behalf; a Resource is root.s.' engine/crates/core/src/access.rs
grep -c "the named Agent's owning Fleet, from an Interface acting on its own behalf; a Resource only from root, through its owning Pool (PND-010)" engine/crates/core/src/explain.rs
grep -c 'row 16 (sign-in.v1) stays at tier 1 so that a refusal discloses' engine/crates/core/src/explain.rs
grep -c 'fn sign_in_access_refuses_out_of_scope_callers_without_detail' engine/crates/core/src/explain.rs
grep -c "// sign-in.v1's route is built (order 048a-5b); its tier 1 is pinned by" engine/crates/core/src/explain.rs
grep -c 'should another request reach here, fail, never dispatch.' engine/crates/core/src/sign_in.rs
grep -c 'The instance is retired, so sign-in.v1 signs nothing in and dispatches no part.' engine/crates/core/src/sign_in.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Since order 048a-5b the access tier decides first who may ask'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'stay at tier 1 although order 048a-5b built its route'
```

This prints `2`, decision 0's second line in `sign_in::preflight`:

```bash
grep -c 'sign-in.v1 reached its preflight from a caller or for an instance that access does not permit' engine/crates/core/src/sign_in.rs
```

This prints `5`, decision 14's five texts in `sign_in_request_member_refusals`:

```bash
grep -c 'reports invalid under [A-Z0-9-]* for Agent/agent-template: ' engine/crates/core/src/sign_in.rs
```

This prints `PND-003:1 PND-005:1 PND-010:1 PND-011:1`, each row's count of cases one more than at
the start:

```bash
python3 -c 'import re; rows = lambda p: {l.split("|")[1].strip(): int(l.split("|")[4]) for l in open(p) if re.match(r"^\| PND-0(03|05|10|11) \|", l)}; b = rows("engine/target/standing-048a5b-before.txt"); a = rows("engine/STANDING.md"); print(" ".join(k + ":" + str(a[k] - b[k]) for k in sorted(b)))'
```

This prints `4`:

```bash
grep -c -E '^\| PND-0(03|05|10|11) \| convention \| must \| [0-9]+ \| 0 \| outside \| [a-z, ]+ \| case passes \|$' engine/STANDING.md
```

Each of these prints 0:

```bash
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
grep -c 'not built yet, order 048a' engine/crates/core/src/requests.rs
grep -c "C11's temporary stubs" engine/crates/core/src/requests.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'not built yet, order 048a'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply, and otherwise unavailable' | wc -l
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'does not yet answer'
grep -c 'It includes temporary stubs' engine/crates/shell/src/server.rs
grep -c '^| DL18 | Not built:' engine/CONFORMANCE.md
grep -c '"{tool} reports {reason} under {rule}: {cause}"' engine/crates/core/src/sign_in.rs
grep -cE 'sign-in\.v1 reports [a-z{}]+ under [A-Z0-9{}-]+: ' engine/crates/core/src/sign_in.rs
grep -c 'ACC-033 rows 9 (enrollment) and 16 are missing from routing.' engine/crates/core/src/explain.rs
sed -n '/fn unimplemented_authorizing_routes_keep_non_root_refusals_at_tier_one/,/^    }$/p' engine/crates/core/src/explain.rs | grep -c '"sign-in.v1"'
grep -c 'learns more than the stub' engine/crates/core/src/sign_in.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c "gets the stub's unavailable"
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Refusals for .answer.v1., .sign-in.v1. and the enrollment'
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 10 names with the totals, the function names and signatures of
decisions 1 to 6 and 14, the lines of `access.rs` and `explain.rs` that decision 0 changed, the
line in `invoke_inner` where `sign-in.v1` is handed to `begin` and the line in `child_answered`
where its part's answer is handed on, and whether any path reaches decision 0's second line, the
cases decisions 1 and 2 ask about and the findings of decision 13.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5b-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
