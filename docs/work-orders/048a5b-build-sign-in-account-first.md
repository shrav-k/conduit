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

**The interim ending, which 048a-5e replaces.** PND-010 says that a check finding no account or
another account goes on: "the engine invokes sign-in-hook.v1 as a part and relays its URL
elicitation under IFC-049; that initial mismatch does not undo the account or end sign-in.v1".
That hook part is 048a-5d's, and the relay 048a-5e's. Until 048a-5e, this build has no step to go
on with, so here such a check ends `sign-in.v1` failed, with a diagnostic naming PND-010 and that
the relay is order 048a-5e's (decision 4), after another account has raised the sign-in action
where PND-005 asks it. This order states it so in `engine/CONFORMANCE.md` (decision 9), and
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

**What orders 048a-2 to 048a-5 bring, which this order assumes.** Orders 048a-2 and 048a-3 are
written on `main`, 048a-4 is written on `main` (its file `048a4-build-sign-in-setup-account.md`),
and 048a-5 is written beside this one (its file `048a5-build-sign-in-refusals-before-parts.md`);
none is built yet. This order starts from 048a-5's work (below). It assumes, as their decisions
state them:

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
- From 048a-4's decision 2: the function in `requests.rs`, beside `hook`, that dispatches
  `account-hook.v1` or `sign-out-hook.v1` as a part of a parent invocation, afresh (attempt 1, key
  null), with the parent's caller, sender and filters, `setup_parent` and `serving` the parent's
  ID, the reply `{"pending":true,"invocation":<parent>,"hook":<ID>}` and the decision
  `dispatch-account-check` or `dispatch-sign-out`, and that leaves the parent's phase to its
  caller; called "the part function" below.
- From 048a-4's decision 3: `child_answered` (`lifecycle.rs`) handles a setup's phases
  `account-pending` and `undo-pending`, reading the part's `account` from its `retained.outputs`.
- From 048a-4's decision 4: the raise's third form, a setup, with its trigger, its `raisedBy` and
  its evidence items 2 to 4, and the setup's `invocation-ended` record naming the action in its
  `data`'s `pendingAction`.
- From 048a-4's decision 5: in `sign_in.rs`, the functions that set `SignInUndone` false and that
  clear it ("Clear": only where the instance's condition is `False`, it becomes `True` with the
  reason `AccountChecked` and the message
  `An account check within setup found the expected account or none.`), and the decision
  `clear-sign-in-undone`, citing `PND-011` and `TPL-012`, where clearing changes the condition.
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
  `signInReturn` and names no unknown login. This order reads none of the three members.
- From 048a-5's decision 5: the case `requests-sign-in-refusals-before-parts`, of 18 steps, whose
  step 18 asks `group.sign-in.v1` for `signed` once nothing is in progress and expects the stub's
  unavailable; its definitions S, URL and "Root asks T with A and M".
- From 048a-5's decision 6: steps 4 and 12 of `requests-sign-in-lapse-and-conflict` (indexes 3
  and 11 of 28) declare URL elicitation; step 12 expects the stub's unavailable.
- From 048a-5's decision 8, as 048a-2 and 048a-3 leave the file: in `engine/CONFORMANCE.md`, the
  words "invalid or conflict where the checks of orders 048a-2 and 048a-5 apply, and otherwise
  unavailable" in three places (DL18's row, the paragraph on public engine execution, and the list
  of what the build does not claim), and, in the section "Refusals and reasons (order 046a)", the
  sentence "The unavailable stub of `sign-in.v1` now carries a diagnostic but remains unbuilt."
  followed by 048a-2's and 048a-5's sentences; and in DL19's row 048a-3's words "and `sign-in.v1`'s
  answer is not built yet." and "and nothing in this build lifts a hold, since `sign-in.v1` does
  not yet answer."

Orders 048a-6 and 048b-relay1 may land before this one; it reads nothing of either. Where an
order merged otherwise than this says, apply the nearest sound reading of each decision below to
what merged, and say so. Where a Record names a function, use that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`3c761dd9`, before orders 048a-2 to 048a-5; find each place by the text quoted with it.

- `invoke_inner` (`engine/crates/core/src/requests.rs:848`) ends C11's stubs in the block that
  begins `// C11's temporary stubs: authority and arguments have already been checked.` (line
  1193), whose list holds `sign-in.v1` alone. `reject_diagnostics` (line 639) gives the stub's
  text in its arm `"PND-010" => Some(format!(` (line 665) with `not built yet, order 048a.`
  (line 666).
- Further on, `invoke_inner` refuses an engine operation that no transition handles, in the block
  ending `return Err("engine operation belongs to its lifecycle or records transition".into());`
  (line 1379), and then hands the new invocation to `crate::records::execute` or
  `crate::lifecycle::execute` or, for any other operation, pushes its dispatch intent (line 1606,
  `t.external_intents.push(intent(&invocation, c));`). The core package's implementation `engine`
  implements `sign-in.v1` (`spec/core/conduit.yaml`), so its invocation's `implementation` has
  `source` `engine`.
- `complete` (`requests.rs:1729`) ends a part as any call, validating a success's
  `structuredContent` against the operation's `outputSchema` (`account-hook.v1` states one whose
  `account` is a string or null) and keeping it in `retained.outputs`; then `child_ended` and
  `child_answered` (`lifecycle.rs:1082` and `1092`) reach the parent through `setup_parent`.
  `child_answered` returns at once where the parent is not `in-progress`.
- `finish` (`requests.rs:1648`) ends an invocation, writes its `invocation-ended` record and its
  reply through `ended_reply`, whose `structuredContent` holds the outputs where the contract
  states an `outputSchema`; `sign-in.v1` states one, requiring `account`, a string.
- `records::execute` (`records.rs:254`) answers an action for `answer.v1`: `store_answer`, then
  `finish`, then `pendingAction` added to the `invocation-ended` record's `data` in `t.records`
  and `t.state.records`, then `record_answer`, then its decision. The success of decision 3
  follows that order.
- `explain::outcome_diagnostic(invocation, outcome, rule, cause)` (`explain.rs:869`) builds an
  engine-decided outcome's diagnostic.
- `engine/crates/shell/src/server.rs`, in the comment at lines 39 to 41 above
  `EXECUTABLE_ENGINE_OPERATIONS`, says the list "includes temporary stubs that answer unavailable
  and name the order that builds each operation (OPS-005)".
- No case on `main` asks `sign-in.v1` of an instance whose Type states `signIn`. The shell test
  `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders` asks
  `system.sign-in.v1` for the Resource `management`, whose Type states no `signIn`, so 048a-2's
  invalid answers it before any dispatch, and it does not change here.

**When this order starts.** Order 048a-5 must be merged to `main` first, and with it 048a-2 to
048a-4; or, where the coordinator's dispatch note says so, your tree is 048a-5's verified branch,
and the note relaxes only the first two checks below, which say that 048a-5 and 048a-4 merged.
Before you change anything, check from the repository root that you find what they give, and
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
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml"))["steps"]; print(s[11]["input"]["request"]["tool"], "unavailable" in json.dumps(s[11]["expect"]), len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-refusals-before-parts/case.yaml"))["steps"]; print(s[17]["input"]["request"]["tool"], "unavailable" in json.dumps(s[17]["expect"]), len(s))'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply, and otherwise unavailable' | wc -l
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'The unavailable stub of .sign-in.v1. now carries a diagnostic but remains unbuilt.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and .sign-in.v1..s answer is not built yet.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and nothing in this build lifts a hold, since .sign-in.v1. does not yet answer.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and the check within .sign-in.v1. is not built yet.'
grep -c 'It includes temporary stubs that answer unavailable and name the order that' engine/crates/shell/src/server.rs
test -d conformance/conventions/requests/sign-in-account-first; echo "dir $?"
```

They print `1`, `1` (048a-5's and 048a-4's merges), `1`, `1` (the stub and its text), `1`
(048a-5's refusals), `1` (048a-3's holds), `1`, `1`, `1` (048a-4's part function and clear),
`1` (048a-4's PND-011 hold),
`['first-start', 'register-project', 'group.set-up.v1', 'setup-report', 'setup-report', 'complete', 'access.open.v1', 'to-signed.work.v1', 'complete', 'group.inspect.v1', 'to-signed.work.v1']`,
`['group.set-up.v1', 'setup-report', 'complete', 'complete', 'group.inspect.v1'] 30`,
`group.sign-in.v1 True 28`, `group.sign-in.v1 True 18`, `3`, `1`, `1`, `1`, `1`, `1` and `dir 1`.

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
```

Orders that land beside this one (046b, 048a-6, 048b-relay1, 049a-1, 050 and others): whichever
lands second keeps both changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** PND-010, PND-011, PND-005, PND-003 and PND-012 decide that the account check
is the first part, that only the expected account compared ignoring ASCII case completes a
sign-in, with output `account` and the action answered signed-in, that a failed or unknown check
completes nothing, that a check finding the expected account or none clears `SignInUndone`, that
another account found there raises the action unless one is unanswered, and that the records name
the action raised or answered. MC9a's K11 and H05, registered by order 201, give the output as the
account the check reported and the hold as lasting to the next success. Everything else here is
the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-5 that puts this behavior here
(order 048a-5's "Where it sits"); the interim failed ending for no account or another, with its
diagnostic and its sentence in `engine/CONFORMANCE.md`, until 048a-5e; ending `sign-in.v1` failed,
not unknown, where its account check ends failed or unknown, since no part with an effect has run;
the stand on MC9a-003 that a success lifts the hold whatever its action's state (order 048a-3's);
following MC9a-002's reading that another account found while a retirement of the instance is in
progress or done raises nothing; ending `sign-in.v1` failed under PND-003 where the instance is
retired, or removed by a retirement, when the check ends, leaving its hold entry as it is; keeping the instance's ID in the invocation's `object` without marking
it `management`, so that REV-014 does not refuse a setup or retirement while a `sign-in.v1` is in
progress, as MC9a-002 reads the text as written; the invocation's member `sign_in`, its phases and
its parts; reusing 048a-4's part function, its raise and its clear, each widened by one parameter
or form; the diagnostics' words; the decisions' names; the function names; the unit tests; the
case, its reuse of 048a-3's fixture and of steps of 048a-3's and 048a-4's cases, its route, IDs and
times; the one step changed in each of 048a-2's and 048a-5's cases; and the edits to
`engine/CONFORMANCE.md` and to the shell's comment. Apply them as written and report them under
`decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-001, PND-003, PND-004, PND-005, PND-010, PND-011 and PND-012
   (`spec/promises/pending.yaml`); `sign-in.v1` and `account-hook.v1`
   (`spec/core/operations.yaml`); IMP-007 (`spec/conventions/implicit.yaml`); DL18 and DL19
   (`spec/implementation-defined.yaml`).
2. Orders [048a2](048a2-build-sign-in-lapse-and-conflict.md) (decisions 4 to 7),
   [048a3](048a3-build-sign-in-hold.md) (its stand on MC9a-003 and decisions 1 to 4),
   [048a4](048a4-build-sign-in-setup-account.md) (decisions 2 to 6, 8 and 10) and 048a-5 (its
   file `048a5-build-sign-in-refusals-before-parts.md`, decisions 1, 2, 5, 6 and 8), and the
   Records of the four.
3. In `experiments/012-model-checking/README.md`, MC9a's properties K11 and H05 and its findings
   MC9a-002 and MC9a-003.
4. The cases `requests-sign-in-hold`, `requests-sign-in-setup-account`,
   `requests-sign-in-lapse-and-conflict` and `requests-sign-in-refusals-before-parts`.
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.3 to 0.4k changed lines of Rust with two unit tests, one new case of 32 steps
on 048a-3's fixture, one step's expectation changed in each of two cases, five places in
`engine/CONFORMANCE.md` and one comment in the shell.

## The decisions it applies

1. **`sign-in.v1` dispatches, and C11's stub goes.** In `invoke_inner`, delete the block of C11's
   stubs whole, its two comment lines included (from `// C11's temporary stubs` to its
   `return Ok(());` and closing brace), and in `reject_diagnostics` delete the arm
   `"PND-010" => Some(format!(` whose text ends `not built yet, order 048a.`. Where any call of
   `reject` other than `reject_diagnostics` still passes the rule `PND-010` with no diagnostics,
   stop and report it. In the block that refuses an engine operation no transition handles, add
   `&& op != c.qualified("sign-in.v1")` to its condition. In the chain that hands the new
   invocation on, add, right before its last `else` (the dispatch intent), the arm
   `else if op == c.qualified("sign-in.v1") { crate::sign_in::begin(t, a, c, i)?; }`. Nothing
   else in `invoke_inner` changes: the access decision, 048a-2's and 048a-5's preflight, the
   record `invocation-dispatched` and the decision `dispatch-invocation` come first, as for any
   operation.

   In `sign_in.rs`, add
   `pub(crate) fn begin(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize) -> Result<()>`,
   which, for the invocation at `i`:
   1. Resolves `arguments.instance` with `access::object` and calls the account function for it.
      Where it resolves no instance or the account function gives no string, which 048a-2's
      preflight leaves no way to reach, it ends the invocation (decision 4's end) failed under
      `PND-010` with the cause
      `The instance's expected account is not a string, so sign-in.v1 cannot check it and signs nothing in.`,
      and returns; report whether any path reaches it.
   2. Sets the invocation's `object` to the instance's ID, so that 048a-2's conflict sees it in
      progress, and its member `sign_in` to `{"phase": "account-pending", "parts": []}`. It sets no
      `management`.
   3. Takes the instance's `resolved_implementations` at the qualified `account-hook.v1`. Where
      none is resolved, which CTR-013's checks (order 048a-1) leave no way to reach, it ends the
      invocation failed under `PND-011` with the cause
      `No implementation of account-hook.v1 is resolved for the instance, so its account cannot be checked and sign-in.v1 signs nothing in.`,
      and returns; report whether any path reaches it.
   4. Calls the part function for `account-hook.v1`, with the invocation at `i` as its parent and
      the instance as its object.

   The part function gains one parameter, the rules its decision cites: 048a-4's calls pass the
   lists 048a-4's decision 2 gives, unchanged, and `begin` passes `PND-010`, `PND-011`,
   `CTR-013`, `ACC-018` and `IMP-002`, with the decision's name `dispatch-account-check`
   unchanged. Where the part function reads or writes its parent's `setup` member on the path
   that dispatches a part, stop and report it: 048a-4's decision 2 leaves the parent's phase to its
   caller. Its no-implementation branch, which `begin`'s step 3 leaves unreachable for a
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
      account while the check ran), it ends the invocation failed under `PND-010` with step 1's
      cause of decision 1, and returns; report whether any path reaches it.
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
     048a-3's test `hold_outlasts_expiry` (the Agent `s1` with the account `primary-account` at
     `/account`): 048a-2's raise at T for an invocation `c1` with the trigger `lapsed-login`, as
     that test calls it, gives an action A and sets `s1`'s hold to `{"since": T}`. On a copy of
     that state, `answer_and_lift` for `s1`, the invocation `{"id": "g1", "caller": "r"}` and
     T+1s gives A with the answer
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
   end, and no later step asks `sign-in.v1` again. Change only each step's `expect`, which becomes
   `{"reply": {"pending": true}}` with PART(`account-hook.v1`), and add no `capture`:
   - `requests-sign-in-lapse-and-conflict`, its step 12 (index 11);
   - `requests-sign-in-refusals-before-parts`, its step 18 (index 17), its last.

   Nothing else in either case, and nothing in their directories' other files, changes. Both cases
   must then pass; if a later step of `requests-sign-in-lapse-and-conflict` gives another reply
   than its case states, stop and report it.

10. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each sentence by its text read
    across line breaks, as 048a-2 to 048a-5 leave it. Eight phrases in five places change; in each
    pair below, the first line is the phrase as it stands and the second what it becomes, and
    every other word of each line stays:

    1. In DL18's row in "Choices still required", in the same line of the table, its first words
       `Not built:` become `Partly built (order 048a-5b):`, and the words from
       `, and otherwise unavailable, with the text` to the row's last words
       `no wait is chosen.`, the text `not built yet, order 048a` and its link to `requests.rs`
       included, become the second line below (the first is that phrase without its link):

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

    DL18's and DL19's text stays one line in the file's table, as each row is now; places 2 to 4
    may be rewrapped at 100 columns where their paragraphs are wrapped. If an earlier order merged one of these
    sentences otherwise, replace only the words that say `sign-in.v1` is a stub, answers
    unavailable, does not answer or is not built, and say so. Nothing else in the file changes.
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

12. **The declaration.** `docs/work-orders/048a5b-declared.yaml` holds five shapes: one `added` for
    `requests-sign-in-account-first`, and one `flip` and one `expectation` for each of decision 9's
    two steps. The coordinator wrote it with this order; check that it matches and change it only
    to make it so.

13. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
    record changes. No fixture changes. No `sign-in-hook.v1` or `sign-out-hook.v1` part runs
    within `sign-in.v1`, nothing is relayed or elicited, and nothing reads `signInLogin`,
    `signInReturn` or a requestState. The fixed-port refusal (048a-5c), the sign-in hook and the URL
    match (048a-5d), the message and the relayed output (048a-5e), polling, the undo during polling
    and DL18's end (048a-5f to 048a-5h), and the returns (048a-6 to 048a-8) are not built. No CLI
    command (051b), relay (048b-relay1, 048b-relay2, 048b-join) or package (050) changes. Report as
    findings, for the coordinator:
    - that no order checks `expectedId` on `sign-in.v1` (PND-010: "expectedId follows LIF-037"):
      `records::preflight` and the lifecycle preflight check it for other operations only, so from
      this order on a `sign-in.v1` whose `expectedId` names another ID is dispatched; report
      whether any path refuses it;
    - that `requests::cancel`, which the portable input `cancel` reaches, ends a `sign-in.v1`
      invocation as an external call (it has no `setup` member) and leaves its account part in
      progress, whose later answer `child_answered` then ignores; report whether any path reaches
      it, and change nothing for it;
    - that no promise states `sign-in.v1`'s outcome where its first account check ends failed or
      unknown, so `failed` is the coordinator's;
    - that MC9a-002's one-sided exclusion stands: a setup or retirement asked for while a
      `sign-in.v1` is in progress is dispatched, since the invocation is not marked `management`;
    - that a successful `sign-in.v1`'s result names in no `pendingAction` key the action it
      answered, and a failed one none it raised, as 048a-3 and 048a-4 reported for a lapse and a
      setup;
    - as coordinator choices no check of this order tests: the `pendingAction` that the
      `invocation-ended` records name, the retirement and replacement arms of decision 2 and its
      arm for an instance a retirement removed, the
      missing implementation of decision 1, and the raise skipped while a retirement is in
      progress, so that a build that omits them passes every check here;
    - and any place where PND-003, PND-005, PND-010 or PND-011 left you a choice this order does
      not settle.

## Scope

Change only `engine/crates/core/src/sign_in.rs` (decisions 1 to 7), `requests.rs` (decision 1:
the stub, its text, the engine-operation guard, the dispatch arm and the part function's
parameter, and the raise's existing calls only where decision 4's signature reaches them),
`lifecycle.rs` (decision 2's call in `child_answered`, the part function's and the clear's calls,
decisions 1 and 6, and the raise's existing calls only where decision 4's signature reaches them),
`pending.rs` (decision 4's fourth form of the raise), `engine/crates/shell/src/server.rs` (decision
11's comment), `engine/CONFORMANCE.md` (five places),
`engine/STANDING.md`, written only by the tool, `conformance/conventions/requests/index.yaml`, the
new directory `conformance/conventions/requests/sign-in-account-first/`, and the two files
`conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml` and
`conformance/conventions/requests/sign-in-refusals-before-parts/case.yaml` (decision 9).
`docs/work-orders/048a5b-declared.yaml` comes with this order; change it only as decision 12 says.
`conformance/manifest.yaml` holds the new entry only until the pieces tool moves it, and ends
unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, the rest of `engine/crates/shell/`,
the experiments, the case schema, `conformance/internal-cases.yaml`,
`conformance/expected-failures.yaml`, any other case or fixture, the other files of the case
directories named here, `docs/implementation/` or `docs/proposals/`: report any defect as a
finding. `experiments/011-short-form-spike/` changes only if its tests need it, since it depends on
the core; report why.

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

The third reports two more tests passed than the start's run, 0 failed, the new ones being
`sign_in_first_check_ignores_ascii_case` and `sign_in_success_answers_and_lifts_the_hold`; report
both counts. The fourth and fifth pass as on the tree you start from; report each count. The pieces
tool prints `Nothing to move.` The next five runner commands each end `1 cases passed`;
`requests-sign-in-` and `requests-` each give one more case than the start's run, with the same
expected failures, each still failing; report each count and each expected failure. The shell test
passes: its `system.sign-in.v1` for the Resource `management` still reports 048a-2's invalid. If
any other case, or any shell, CLI or experiment test, fails because `sign-in.v1` now dispatches,
stop and report it: do not change it. The threads are 4 because at 24 the runner ran out of memory
(order 071's Record).

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

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5b-declared.yaml: 5 shapes.`,
then `Vector delta: N changes; M flips; 0 undeclared; 0 errors.` and `exit 0`; report N and M,
and that every change outside the added case lies in decision 9's two steps.
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
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 10 names with the totals, the function names and signatures of
decisions 1 to 6, the line in `invoke_inner` where `sign-in.v1` is handed to `begin` and the line
in `child_answered` where its part's answer is handed on, and whether any path reaches the cases
decisions 1 and 2 and the findings of decision 13 ask about.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5b-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
