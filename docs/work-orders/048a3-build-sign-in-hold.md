# Work order 048a3 (E7a-3): an instance awaiting sign-in is held, and an expiry lifts nothing

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the case you add, with the promises it tests and
whether it passes. This order has network access only because the engine's tests bind loopback
ports, which the sandbox refuses without it; use it for nothing else, add no crate, and reach no
other machine. Run one cargo command at a time. Never print a whole large file, case, fixture or
log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Do not run `claude`, `codex` or any client, and sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry for 048a-2, and the coordinator's cut
that [order 048a2](048a2-build-sign-in-lapse-and-conflict.md) records under "Who decided what",
split 048a-2's entry into five orders, each after the one before, since an order holds one
behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). This is the second:
"048a-3, the hold (PND-005's `then`: held requests unavailable with `META/pendingAction` and a
PND-005 diagnostic, expiry lifting nothing, a new action on the next held request)". Order 048a-2
raises an instance's sign-in action when a call reports a lapsed login, and refuses a second
`sign-in.v1` of one instance as conflict; it leaves the instance unheld (its decision 14). Later
orders build the rest: 048a-4, setup's account check, the wrong-account undo and SignInUndone
(PND-011); 048a-5, `sign-in.v1` dispatched, with IFC-049's relay and polling, and its signed-in
answer; 048a-6, the returns (PND-013, PND-014). This order builds none of them.

**What it builds.** One behavior: the hold. PND-005's `then`, after the raise: "The instance is
then held until sign-in.v1 answers that it is signed in (PND-010, PND-011): an operation that
starts model work on an Agent, and any request to a Resource other than the parts of sign-in.v1,
of a setup and of its retirement, is not dispatched and reports unavailable with the ID of the
instance's unanswered sign-in action in `META/pendingAction`, with a diagnostic naming PND-005 and
that the instance awaits sign-in to its expected account, without naming that account (PND-004).
The hold is the instance's, not the action's: an expiry lifts nothing." Its `observable`: "Every
held request is final. Nothing is queued, resumed or dispatched after it ends; the caller sends a
new request." and "After expiry a new sign-in is raised on the next held request or setup, never
by expiry alone." (The quote writes the `_meta` key as `META/pendingAction`, the key
`pendingAction` under the prefix that `spec/names.yaml` sets.) IFC-025 adds: "`pendingAction`
names the action IDs raised or holding the request (PND-005, PND-007)" and "pendingAction is a
list of action IDs, including when it names one action, and carries no evidence value". PND-012:
"A request raises or answers an action. Its record names the action's ID." No promise changes:
PND-005 already states the hold, and this order builds it. The setup half of "the next held
request or setup" is 048a-4's, with setup's account check.

**MC9a-003, and the stand this order takes.** [Order 201](201-mc9a-pending-actions-sign-in-registered.md)
registers MC9a with the finding MC9a-003: "Where the action expired before a `sign-in.v1`
succeeds, no action is left for that success to answer, and the words do not say whether the
success lifts the hold. The model reads the hold as lifted by the next `sign-in.v1` of the
instance that ends succeeded, whatever its action's state, since the hold is the instance's, not
the action's." The coordinator takes the same reading as a stand within the specification as
written: PND-005 holds the instance "until sign-in.v1 answers that it is signed in", which is
`sign-in.v1`'s own success with output `account` (PND-010: it "succeeds with output account and
answers any sign-in action signed-in"), and "The hold is the instance's, not the action's". So
the hold is kept for the instance, apart from its actions (decision 1): an expiry lifts nothing,
and only the success of a `sign-in.v1` of the instance lifts it, whatever state its action is in.
This order builds the hold and no lift: `sign-in.v1` does not dispatch until 048a-5, which lifts
the hold on its success. The stand is the coordinator's, open to the owner's strike; MC9a-003
stays reported, not settled, until MC9a runs. In practice the case of no action left is narrow:
from this order on, the first held request after an expiry raises a new action (decision 3), so
a success finds none to answer only where no held request came between the expiry and the
`sign-in.v1`.

**What order 048a-2 brings, which this order assumes.** Order 048a-2 is written on `main` and not
yet built; it starts only once orders 047b, 047c and 201 have merged, and this order starts only
once 048a-2 has merged (below). It assumes, as 048a-2's decisions state them:

- From its decision 4: the sign-in kind stored in 047c's `State` member `pending_actions`, keyed
  by action ID, each value `{"action": A, "dispatched": null, "raisedBy": I}`, `A` in
  `pending-action.v1`'s form with `kind` `sign-in`, one subject (the instance), four `fact`
  evidence items (the expected account; the raising invocation's ID; its operation; the trigger,
  `lapsed-login` or `url-elicitation`) and an expiry seven days after the step that raised it;
  047c's function that gives the action an ID names at a time, which gives `expired` at or after
  its expiry; the actions the state holds, their views, counts, `status.pending` and `list.v1`'s
  `pending`, and the inspection's decision `show-sign-in-action`.
- From its decision 5: `engine/crates/core/src/sign_in.rs`, with `mod sign_in;` in `lib.rs`, and
  in it the function that gives an instance's expected account (no `signIn`, an account that is
  not a string, or the pointer and the string); in `pending.rs`, the function that raises the
  instance's sign-in action, given the state, the instance, the account and its pointer, the
  invocation, the trigger, the step's time and the Allocation `a`, which first tests whether the
  instance holds a sign-in action with no answer at that time and, where it does, allocates
  nothing, stores nothing and gives none; the decision `raise-sign-in-action`; and the arms of
  `complete` that call it.
- From its decision 6: the preflight of `sign-in.v1`'s invalid and conflict in `sign_in.rs`,
  called in `invoke_inner`, with `sign-in.v1`'s stub after it, unchanged.
- From its decision 7: retirement answers the instance's unanswered sign-in action `void`, with
  the decision `answer-sign-in-void`.
- From its decision 10: the portable case `requests-sign-in-lapse-and-conflict` in
  `conformance/conventions/requests/sign-in-lapse-and-conflict/`, with its `manifest.json`,
  `documents.json` and `fixture.json`: the keyed implementation `signed-server`, the Type
  `signed-agent.v1` whose account pointer is `/account`, the Agent template `signed-template`
  with the account `primary-account`, the Fleet `group`, the Agent `signed` and root's Conduit
  `to-signed`; its result LAPSE, a keyed result that ends failed with an error diagnostic naming
  PND-005; its helper phrases "Root asks T with A", "a run report" and "a completion"; and its
  steps 1 to 3, 7, 8, 12, 15 and 16.
- From its decision 11: DL19's row in `engine/CONFORMANCE.md` with the sentence that ends "and
  neither the hold of PND-005 nor `sign-in.v1`'s answer is built yet.".

Where 048a-2 merged otherwise than its order says, apply the nearest sound reading of each
decision below to what merged, and say so. Where 048a-2's Record names a function, use that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`f414e438`, before orders 047c and 048a-2; find each place by the text quoted with it.

- `invoke_inner` (`engine/crates/core/src/requests.rs:826`) decides a request in OPS-023's order:
  the access decision, whose record it writes with `record(t, a, c, "access-decision",
  access_data)?;`; the repeat's conflicts (OPS-010, OPS-032, OPS-034); `lifecycle::preflight`,
  `records::preflight` and, after 048a-2, `sign-in.v1`'s preflight; reconciliation's checks; the
  second access evaluation, `let available = access::evaluate(` with its `unavailable` reasons
  (CON-024 for a target that is not `ready`, NAM-026, ACC-015); then the block that answers a
  kept repeat from its outcome (`if let Some(old) = &previous && !redispatch`, the decision
  `answer-repeat`); then C11's stubs (`// C11's temporary stubs`). By then `target` is the
  target's publication object and `contract` the operation's contract as the target holds it
  (`schemas::operation(&target, op, &c.core_package)`).
- `reject_diagnostics` writes the reply with `envelope`, whose `_meta` holds the reason and no
  `pendingAction`, takes specific diagnostics where they are given, pushes the decision
  `end-with-reason`, and passes the route, with `ruleId`, to `end`, which writes it as the
  `request-ended` record's `data`, less `filters` and null members.
- `requests.rs`'s `read_only` and `inventory.rs`'s own-tools reach already read an operation's
  `startsModelWork` from its contract's `spec`, compared with `true`.
- A setup's parts that are requests target `access` (`open.v1`) or the child's owner
  (`set-up.v1`), in `lifecycle.rs` (`requests::invoke_part`), and a cascade's retirements target
  the Fleet; the hooks are not requests. None of them targets the instance, so none reaches the
  hold, as PND-005's exception for "the parts of sign-in.v1, of a setup and of its retirement"
  asks.
- A retired instance is not `ready`, so the second access evaluation ends every request to it
  under CON-024 except a management verb, which no instance's Type offers today, before the hold
  is reached. Report whether any path reaches the hold for a retired instance.
- 047b's marking sets `META/unanswered` on each result of an Interface's own request, an
  undispatched one included, counted over the state after the step; a held result to root
  carries it with no change here.

**When this order starts.** Order 048a-2 must be merged to `main` first. Before you change
anything, check from the repository root that you find what it gives, and stop, change nothing and
report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md
grep -c '^mod sign_in;' engine/crates/core/src/lib.rs
grep -c 'pending_actions' engine/crates/core/src/lib.rs
grep -c 'sign_in_holds' engine/crates/core/src/lib.rs
grep -c 'fn sign_in_refusals_before_dispatch' engine/crates/core/src/sign_in.rs
grep -c 'fn sign_in_action_raised_by_a_lapsed_login' engine/crates/core/src/pending.rs
grep -rl '"raise-sign-in-action"' engine/crates/core/src | wc -l
test -f conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml; echo "case $?"
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'neither the hold of PND-005 nor `sign-in.v1`.s answer is built yet'
grep -c '^| PND-005 | convention | must | 1 | 0 | outside | core | case passes |$' engine/STANDING.md
grep -c '^\*\*MC9a-003' experiments/012-model-checking/README.md
```

They print `1`, `1`, a number other than `0`, `0`, `1`, `1`, `1`, `case 0`, `1`, `1` and `1`.
Orders that land beside this one (046b, 047d, 049a-1, 050 and others): whichever lands second
keeps both changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** PND-001, PND-005, PND-012, IFC-025, OPS-005, OPS-023 and OPS-025 decide what
is held, what a held request reports, what its record names, where the hold falls in the order of
reasons, and that a held request is final. Everything else here is the coordinator's, each open to
the owner's strike ([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-2 that puts
the hold in this order (order 048a2's "Who decided what"); the stand on MC9a-003 above, that the
hold is kept for the instance apart from its actions and that only a `sign-in.v1` success lifts
it, which 048a-5 builds; keeping the hold in a `State` member of its own, its name and its form;
beginning the hold at each raise of a sign-in action and ending it at nothing in this order,
retirement included; reading "an operation that starts model work on an Agent" as a request whose
target is an Agent and whose operation's contract, as the target holds it, states
`startsModelWork: true`, and "any request to a Resource" as every request whose target is the
Resource, an onward request included; deciding the hold last among the reasons, after the
second access evaluation's `unavailable` and after a kept repeat's answer, so that a repeat
answered from its outcome is not held and a new attempt is; setting `META/pendingAction` on every
held result, whoever its caller, an Agent included, as PND-005 writes it with no exception;
naming the action in the `request-ended` record's `pendingAction`; raising a new action from a
held request only where 048a-2's raise finds none unanswered, with `raisedBy` null and the
evidence items of decision 3, the second naming the request's `access-decision` record; holding a
request all the same, with nothing raised and no `pendingAction`, where the instance's expected
account is not a string; the diagnostic's words; the decisions' names; the unit tests; the case,
its fixture, route, IDs and times, and its tags; and DL19's new words. Apply them as written and
report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-001, PND-002, PND-003, PND-004, PND-005, PND-010, PND-011 and PND-012
   (`spec/promises/pending.yaml`); IFC-025 (`spec/interface/implementation.yaml`), its paragraph
   on `pendingAction`; OPS-005, OPS-009, OPS-023 and OPS-025 (`spec/conventions/operations.yaml`);
   IMP-001 (`spec/conventions/implicit.yaml`); CTR-013 (`spec/promises/contracts.yaml`), its
   sentence on controls and the account pointer; DL19 (`spec/implementation-defined.yaml`);
   `record.v1`'s `pendingAction` (`spec/core/results.yaml`); the rows model work, pending action
   and raised at of `spec/definitions.md`.
2. [Order 048a2](048a2-build-sign-in-lapse-and-conflict.md), whole, and its Record; orders
   [047b](047b-build-actions-raised-by-scope.md) (decision 6 and its Record) and
   [047c](047c-build-stored-answers.md) (decisions 1 and 8, and its Record).
3. In `experiments/012-model-checking/README.md`, MC9a's properties H05 and P01 and the finding
   MC9a-003.
4. The cases `requests-sign-in-lapse-and-conflict` and `records-purge-succeeded`, and
   `conformance/conventions/inventory/own-tools/documents.json`'s Operation `work.v1`.
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: S-M**: about 0.2 to 0.3k changed lines of Rust with two unit tests, one new case with its
fixture, and one sentence in `engine/CONFORMANCE.md`.

## The decisions it applies

1. **The hold is the instance's, stored apart from its actions.** Add to `State` (`lib.rs`) the
   member `sign_in_holds`, a `BTreeMap<String, Value>` keyed by the held instance's ID, with
   `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]` as `tombstones` has, so that a
   state without it reads as before and serializes unchanged. Each value is `{"since": X}`, `X`
   the evidence time of the step whose raise began the hold. Nothing in this order removes an
   entry: not an expiry, not a purge (REC-004 removes records and nothing else), not a
   retirement's void answer (048a-2's decision 7), not a restart, which reads the state back.
   Order 048a-5 removes the entry when a `sign-in.v1` of the instance succeeds. Where a struct
   literal of `State` names every member and so no longer builds, add the new member empty there,
   and say where. In `engine/README.md`, the sentence listing `State`'s members gains
   `sign_in_holds` after `pending_actions`, and nothing else.

2. **A raise begins the hold.** In 048a-2's raise function in `pending.rs`, where it stores an
   action, also insert the instance's entry in `sign_in_holds`, `{"since": X}` with `X` the step's
   time, unless the instance already has one, which it keeps unchanged. Where it stores nothing,
   it changes no hold. Every sign-in action is raised through that function, so the hold begins
   with the first and lasts across every later one. Where 048a-2 wrote its test "the instance
   holds a sign-in action that has no answer at that time" inline, move it into one function in
   `pending.rs` that gives that action, or none, and call it from the raise and from decision 4;
   write no second copy of the test. Where 048a-2's decision 6 helper or decision 7 function tests
   the same thing inline, have them call this function too.

3. **A held request raises a new action after an expiry.** Make 048a-2's raise take its raising
   request as one of two: the invocation whose call reported the lapse, as 048a-2 gives it; or a
   held request, given by the ID of the `access-decision` record that `invoke_inner` wrote for
   it and the request's `operation`. For a held request the trigger is `held-request`, the stored
   entry's `raisedBy` is `null` (PND-001: "Where there is a raising invocation"; a held request
   begins none), and the evidence items are, in this order, each of kind `fact`:
   1. 048a-2's first item, the expected account, unchanged.
   2. `V` the `access-decision` record's ID; `W` `records`; `P`
      `the request that found the instance held`.
   3. `V` the request's `operation`; `W` `records`; `P` `the operation of that request`.
   4. `V` `held-request`; `W` `records`; `P` `how the instance was found awaiting sign-in`.

   Everything else follows 048a-2's decision 5: the test for an unanswered action first, then the
   ID from `a.next`, the stored entry and the decision `raise-sign-in-action`, citing `PND-001`,
   `PND-005` and `PND-012`, with `DL19`, and the facts `{"action": ID, "instance": S}`. The
   signature is your choice; report it.

4. **The held test, and where it falls.** In `sign_in.rs`, add a function that says whether a
   request is held, given the state, the request's target object and the operation's contract as
   the target holds it: true where the target is a Resource or an Agent with no `spec` (not a
   template), `sign_in_holds` has an entry for its ID, and either the target is a Resource or the
   contract's `spec.startsModelWork` is `true`. In `invoke_inner`, keep the ID of the record that
   `record(t, a, c, "access-decision", access_data)?;` pushes, from the last entry of `t.records`
   right after that line. Then, right after the block that answers a kept repeat from its outcome
   and before C11's stubs, where the request is held:
   - Find the instance's expected account with 048a-2's account function. Where it gives a
     string, call decision 3's raise for a held request, with that record's ID, the request's
     `operation`, the step's time and `a`; it gives a new action only where the instance holds
     none unanswered at that time. Then let `ID` be the instance's sign-in action with no answer
     at the step's time (decision 2's function), new or not.
   - Push one decision named `hold-for-sign-in`, citing `PND-005`, `IFC-025` and `OPS-023`, with
     no implementation choice, and the facts `{"instance": S, "action": ID}`, `S` the instance's
     ID.
   - Set `route["pendingAction"]` to `[ID]`, so that `end` writes it in the `request-ended`
     record's `data` (PND-012). If `record_content` drops it, add it to that record afterwards in
     `t.records` and in `t.state.records`, as 048a-2's decision 5 adds it to `invocation-ended`,
     and say so.
   - Call `reject_diagnostics` with the reason `unavailable`, the rule `PND-005` and one specific
     diagnostic, `{"ruleId": "PND-005", "level": "error", "message": {"text": T}}`, where `T` is
     `{tool} reports unavailable under PND-005: the instance awaits sign-in to its expected account, so the request was not dispatched; send it again once sign-in.v1 has signed the instance in.`
     and `{tool}` is the route's `tool`. The text names neither the account (PND-004) nor the
     action's ID: the caller may be an Agent, at which no action is raised (PND-002).
   - Set the `_meta` key `pendingAction`, qualified by `mark(c, "pendingAction")`, on `t.reply`
     to `[ID]`, a list of one ID (IFC-025).
   - Return. Nothing is dispatched, queued or kept: a later request is new (OPS-025).

   Where the account function gives no string for a held instance, which CTR-013's check on
   controls (order 048a-1) should leave no way to reach, the request is held all the same, with
   the same diagnostic, nothing raised, no `pendingAction` in the result or the record, and the
   decision's facts `{"instance": S}`; report whether any path reaches it.

   A request that is not held is decided as before. An Agent's request whose operation starts no
   model work is dispatched while the Agent is held (PND-005 holds only model work on an Agent),
   and its call may raise nothing new while the action is unanswered (048a-2). A request to the
   instance that a reason before this one ends, such as CON-024 for an instance that is not
   `ready` or OPS-032 for a repeat in progress, keeps that reason (OPS-023).

5. **Two unit tests pin the rules.** Write the values as `json!` values; members a test does not
   need may be left out. Let T be `2026-11-12T12:00:00Z`.
   - In `sign_in.rs`'s tests, `held_requests_by_family_and_model_work`, on a `State` built by
     hand with the Agents `s1` and `s2`, the Resource `x1` and an Agent template `t1` (with a
     `spec`), and `sign_in_holds` entries for `s1`, `x1` and `t1`; and two contracts, `M`
     `{"spec": {"startsModelWork": true}}` and `P` `{"spec": {"effect": "A fixture effect."}}`.
     It asserts with `assert_eq!` decision 4's test: `s1` with `M`, held; `s1` with `P`, not;
     `x1` with `P`, held; `x1` with `M`, held; `s2` with `M`, not; `t1` with `M`, not.
   - In `pending.rs`'s tests, `hold_outlasts_expiry`, with the objects of 048a-2's test
     `sign_in_action_raised_by_a_lapsed_login`, the Agent `s1` with the account `primary-account`
     at `/account`: 048a-2's raise at T for an invocation `c1` with the trigger `lapsed-login`
     gives an action and sets `s1`'s hold to `{"since": T}`; at its expiry, `2026-11-19T12:00:00Z`,
     the action its ID names has the answer `expired`, decision 2's function gives none, and the
     hold is unchanged; decision 3's raise at that time for a held request whose record ID is
     `h1` and whose operation is `example.org/sign-in-hold#work.v1` gives a new action with
     another ID, expiry `2026-11-26T12:00:00Z`, the four items of decision 3 and a stored entry
     whose `raisedBy` is `null`, and the hold is still `{"since": T}`; a second such raise at that
     time for `h2` gives none and stores nothing; decision 2's function then gives the new
     action; after `state.records` is emptied the hold and both entries are unchanged; and on a
     copy of the state, 048a-2's decision 7 at `2026-11-19T13:00:00Z` answers the new action
     `void` and leaves the hold unchanged.

6. **One portable case, with its own fixture.** Add the directory
   `conformance/conventions/requests/sign-in-hold/` with four files.
   - `manifest.json`: 048a-2's case's `manifest.json` with `address` `example.org/sign-in-hold`,
     and `#work.v1` added to `signed-server`'s `implements` right after `#ping.v1`; nothing else
     changes, and `signed-server` stays keyed.
   - `documents.json`, a list, in this order: e4's Operation `ping.v1` as 048a-2's
     `documents.json` writes it; the Operation `work.v1` as
     `conformance/conventions/inventory/own-tools/documents.json` writes it (it states
     `startsModelWork: true`); the Capability `item-access.v1` as 048a-2's `documents.json`
     writes it, except that its `operations` are `["#ping.v1", "#work.v1"]`; a Conduit template
     `access`, `{"capability": "#item-access.v1", "operations": ["#ping.v1", "#work.v1"]}`,
     written as e4 writes its template `ping`; the Operation `set-limit.v1` and the Type
     `signed-agent.v1` as 048a-2's `documents.json` writes them, except that the Type's
     `implementation` also maps `#work.v1` to `#signed-server`, right after `#ping.v1`; the
     Agent template `signed-template`, the Fleet `group` and the Agent `signed`, each as 048a-2's
     `documents.json` writes it; and the Conduit `to-signed`, as 048a-2's `documents.json` writes
     it, except that it is `from` `access`. No `unset` Agent, template or Conduit.
   - `fixture.json`: 048a-2's case's `fixture.json`, unchanged.
   - `case.yaml`, ID `requests-sign-in-hold`, `layers: [convention, interface]`,
     `promises: [PND-005, PND-001]`, title
     `Hold an instance awaiting sign-in: its model work is not dispatched and names its sign-in action, and an expiry lifts nothing`,
     `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
     `{"fixture": "fixture.json", "manifest": "manifest.json", "documents": "documents.json"}`.
     Write it as JSON with two-space indentation and a final newline, its members in the order of
     `requests-sign-in-lapse-and-conflict`. It must meet 046a's portable-case rule as it stands.

   Below, ROOT, `META/x`, `U`, "Root asks T with A", "a run report", "a completion" and LAPSE are
   as 048a-2's decision 10 defines them. A new step's evidence is its time and, as 048a-2 writes
   its new steps, `ids` the 100 strings `00000000-0000-4000-8000-` followed by the twelve-digit,
   zero-padded decimal of 70000 + 100n + k, for k = 0 to 99, `revisions` the 16 strings `hold-`
   followed by the decimal of n, a `-` and the decimal of j, and `suffixes` the 16 strings `h`
   followed by the four-digit, zero-padded decimal of 100n + j, for j = 0 to 15, where n is the
   step's number in this case; a copied step keeps its own evidence. Every new step's time is
   `2026-11-12T12:00:00Z` unless stated. `I` and `P` are the JSON pointers `/reply/_meta/`
   followed by the key for `invocation` and for `pendingAction`, each `/` written `~1`, as `U`
   is written. Two more results recur:
   - SUCCESS: `{"kind": "result", "result": {"_meta": {"META/outcome": "succeeded"}, "content": []}}`,
     as `records-purge-succeeded` writes a keyed success;
   - HELD(A): `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable", "META/pendingAction": [A], "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "PND-005", "level": "error"}]}}, "external_intents": [], "absent": [I]}`.

   Its steps:
   1. and 2. `requests-sign-in-lapse-and-conflict`'s steps 1 and 2, `input` and `expect`, whole,
      each with its evidence.
   3. to 6. That case's steps 3, 7, 8 and 12, `input`, `expect` and `capture`, whole, each with
      its evidence: the setup of `signed`, its two run reports, and root's open of `to-signed`.
   7. Root asks `to-signed.work.v1` with `{}`. `expect` `{"reply": {"pending": true}}`;
      `capture` `{"call1": "/reply/invocation"}`: nothing is held yet.
   8. A completion for `$call1`, LAPSE. `expect` 048a-2's step 15's: the call ends failed with
      the PND-005 diagnostic and raises the action.
   9. 048a-2's step 16's `input` with this case's evidence, `expect` and `capture`: the action,
      captured as `$signin`.
   10. Step 7's request. `expect` HELD(`"$signin"`), with
       `"record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "unavailable", "ruleId": "PND-005", "pendingAction": ["$signin"]}}]`:
       held, not dispatched, and its record names the action.
   11. Root asks `to-signed.ping.v1` with `{}`. `expect` `{"reply": {"pending": true}}`;
       `capture` `{"call2": "/reply/invocation"}`: an operation that starts no model work is
       dispatched while the Agent is held.
   12. A completion for `$call2`, SUCCESS. `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/unanswered": 1}}}`.
   13. Time `2026-11-19T12:00:00Z`, the action's expiry: step 9's request. `expect`
       `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/status/pending", U]}`.
   14. Same time: step 7's request. `expect`
       `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable", "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "PND-005", "level": "error"}]}}, "external_intents": [], "absent": [I]}`;
       `capture` `{"signin2": P + "/0"}`, the pointer `P` followed by `/0`: the expiry lifted
       nothing, and the held request raised a new action.
   15. Same time: step 9's request. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"pending": [{"id": "$signin2", "kind": "sign-in", "subject": [{"kind": "Agent", "name": "signed"}], "expiry": "2026-11-26T12:00:00Z"}]}}}}`;
       no `capture`.
   16. Same time: step 7's request. `expect` HELD(`"$signin2"`), with step 10's `record_facts`
       naming `"$signin2"`: no further action while one is unanswered.

   The route is new and has not been run, since order 048a-2 is not yet built. Before running it,
   check that no ID repeats across the case's steps' `ids`, the copied steps' included. If 046a's
   runner check refuses the case all the same, if the registration rejects the fixture, if a
   setup asks for a report other than the two that steps 4 and 5 send, if a step copied from
   048a-2's case gives a reply other than that case states, or if a step's reply differs from
   what it states, stop and report the error: do not change the expectations to pass it, and do
   not change the inputs beyond what this decision says. No expectation is generated from the
   engine's output. The coverage index entry holds `id`, `case`
   (`conventions/requests/sign-in-hold/case.yaml`), `promises` and, where its neighbours in that
   piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write it in a
   `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it into
   `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte for
   byte as it was.

7. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find the sentence by its text read across
   line breaks, as 048a-2 leaves it. In DL19's row in "Choices still required", the words "and
   neither the hold of PND-005 nor `sign-in.v1`'s answer is built yet." become, in the same line:

   ```text
   and `sign-in.v1`'s answer is not built yet. From its first raise the instance is held (PND-005, order 048a-3): a request to it where it is a Resource, or one whose operation starts model work where it is an Agent, is not dispatched and reports unavailable, naming its unanswered sign-in action in `pendingAction`; the hold is the instance's and outlasts the action's expiry, the next held request after an expiry raises a new action, and nothing in this build lifts a hold, since `sign-in.v1` does not yet answer.
   ```

   If 048a-2 merged that sentence otherwise, replace only the words that say the hold is not
   built, and say so. Nothing else in the file changes. After the other decisions, run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
   `promises` and to a promise ID in the core's sources. Report the rows of PND-001, PND-005 and
   IFC-025 and the totals; the totals change only by what those rows change.

8. **The declaration.** `docs/work-orders/048a3-declared.yaml` holds one `added` shape named
   `requests-sign-in-hold` for that case. The coordinator wrote it with this order; check that it
   matches and change it only to make it so.

9. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
   record changes. Nothing lifts a hold: `sign-in.v1` keeps its stub and 048a-2's preflight, and
   nothing answers a sign-in action signed-in (048a-5). Setup checks no account and raises
   nothing, so "or setup" in PND-005's `observable` is not built, and nothing reports
   SignInUndone or holds under PND-011 (048a-4). No hook, CLI command (051b), shell code, relay
   (048b-relay, 048b-join) or package (050) changes, and the inspection shows no hold. Report as
   findings, for the coordinator: that order 048a-4 adds setup's account check, so that this
   case's setup, steps 3 to 5, then dispatches an `account-hook.v1` part and changes with that
   order; that PND-005 names the action in `pendingAction` on every held result, an Agent's
   included, while PND-010's conflict names it only where the caller may see it (PND-002), and
   PND-002 raises no action at an Agent, so the owner may want PND-005 to say which; that PND-005
   names only `sign-in.v1`'s success as ending the hold, so a setup whose account check reports
   the expected account (048a-4) lifts nothing as written; that MC9a's H05 asks a request held
   under both PND-005 and PND-011's SignInUndone to carry both diagnostics, which 048a-4 must
   meet; that a call whose lapse raises a sign-in action ends without pendingAction naming it,
   which IFC-025's "raised" asks for and no order of the cut builds; as coordinator choices no
   check of this order tests, the hold of a request to a Resource, `pendingAction` on an Agent
   caller's held result, and a kept repeat answered rather than held, so that a build that omits
   `pendingAction` for Agent callers, or places the hold before the answer-repeat block, passes
   every check here; and any place where PND-005 or IFC-025 left you a choice this order does not
   settle.

## Scope

Change only `engine/crates/core/src/lib.rs` (decision 1's member), `pending.rs` (decisions 2, 3
and 5), `sign_in.rs` (decisions 2, 4 and 5), `requests.rs` (`invoke_inner`, decision 4, and the
raise's call in `complete` where decision 3's signature changes it), each place that names every
member of `State` in a struct literal, only to add the member empty (decision 1),
`engine/README.md` (its sentence listing `State`'s members, decision 1, and nothing else),
`engine/CONFORMANCE.md` (one sentence), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/requests/index.yaml` and the new directory
`conformance/conventions/requests/sign-in-hold/`. `docs/work-orders/048a3-declared.yaml` comes with
this order; change it only as decision 8 says. `conformance/manifest.yaml` holds the new entry only
until the pieces tool moves it, and ends unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`,
`packages/`, the shell, the experiments, the case schema, `conformance/internal-cases.yaml`,
`conformance/expected-failures.yaml`, any other case or fixture, 048a-2's case directory included,
`docs/implementation/` or `docs/proposals/`: report any defect as a finding.
`experiments/011-short-form-spike/` changes only if its tests need it, since it depends on the
core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- sign_in
cargo test --release -p conduit-core --lib -- pending::tests
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-hold
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cargo run --release -p conduit-conformance -- .. setup-action-
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 4 passed and 0 failed: this order's `held_requests_by_family_and_model_work`,
048a-2's two tests and 048a-1's `sign_in_login_faults_name_duplicates_and_overlaps`. The fourth
reports 5 passed: 047's, 047b's, 047c's, 048a-2's and this order's. Report the count if another
test's name matches. The pieces tool prints `Nothing to move.` The runner commands end
`1 cases passed`, `1 cases passed`, `3 cases passed` (the cases of 047, 047b and 047c), and one
more case than `requests-` gives on the `main` you start from, with its one expected failure
unchanged; report both counts. The shell test passes. If any other case, or any shell, CLI or
experiment test, fails because a request is now held, stop and report it: do not change it. The
threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decision 4. From `engine/`, one at a time:

```bash
cp crates/core/src/requests.rs target/requests.rs.fixed
git show HEAD:engine/crates/core/src/requests.rs > crates/core/src/requests.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-hold
cp target/requests.rs.fixed crates/core/src/requests.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-hold
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with an error at
step 10, whose reply is `{"pending": true}` where the hold's unavailable is expected; after the
restore the second ends `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older
modification time, so cargo would not rebuild the crate. Git ignores `target/`. If `HEAD`'s file
does not build against the rest of your change, revert decision 4's lines by hand in the copy
instead, and say so.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a3-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/inventory conformance/conventions/setup conformance/language engine/crates/shell tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a3-declared.yaml: 1 shapes.`,
then `Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.
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
grep -c 'pub sign_in_holds: BTreeMap<String, Value>' engine/crates/core/src/lib.rs
grep -c 'fn held_requests_by_family_and_model_work' engine/crates/core/src/sign_in.rs
grep -c 'fn hold_outlasts_expiry' engine/crates/core/src/pending.rs
grep -rl '"hold-for-sign-in"' engine/crates/core/src | wc -l
grep -c '"id": "requests-sign-in-hold"' conformance/conventions/requests/index.yaml
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'From its first raise the instance is held (PND-005, order 048a-3)'
grep -c '^| PND-005 | convention | must | 2 | 0 | outside | core | case passes |$' engine/STANDING.md
```

Each of these prints 0:

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'neither the hold of PND-005 nor'
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml conformance/conventions/requests/sign-in-lapse-and-conflict | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 7 names with the totals, the function names and signatures of
decisions 2 to 4, the line in `invoke_inner` where the hold is decided, each struct literal of
`State` you changed, and whether any path reaches decision 4's case of an account that is not a
string.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a3-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.

## Record

Run on 2026-11-18, from about 16:07 to 17:12 UTC, on the Mac, by the stacked dispatch the owner
approved that day: from order 048a-2's verified branch at `2a1af4cb`, before 048a-2 merged, in two
rounds: the worker (1784 s) and a fresh Codex verifier (471 s), then the worker again (1227 s) and a
second fresh Codex verifier (404 s). The change is the twelve files of Scope. `State` gains
`sign_in_holds`, read empty where a state lacks it; no struct literal of `State` names every member,
so none changed. 048a-2's raise, `raise_sign_in`, takes its raising request as a
`SignInRequest<'_>`, either `SignInRequest::Invocation` or `SignInRequest::Held` (the
`access-decision` record's ID and the request's `operation`), and inserts the instance's hold where
it stores an action, keeping an earlier hold's `since`. `unanswered_sign_in` is the one test for an
unanswered action, called from the raise, the hold, 048a-2's refusal and the retirement's void
answer. `sign_in::request_is_held` gives the hold from the instance and the operation's contract,
and `invoke_inner` decides it after ordinary availability and the kept repeat's answer and before
C11's stubs: the request ends unavailable with the action's ID in `META/pendingAction` on its result
and its `request-ended` record, a PND-005 diagnostic that names neither the account nor the action,
and the decision `hold-for-sign-in`; after an expiry the first held request raises a new action. The
unit tests `held_requests_by_family_and_model_work` and `hold_outlasts_expiry` and the portable case
`requests-sign-in-hold` (16 steps, 1574 unique IDs) pin it. DL19 takes the order's sentence, and
`STANDING.md` changes only PND-001 (2 to 3) and PND-005 (1 to 2), its totals unchanged. The worker's
own choices: the borrowed enum `SignInRequest`, the route's `pendingAction` list used for both the
record and the reply, the expiry test's check of the allocation cursor, and, for the negative
control, the hold block cut out by hand, since `HEAD`'s file did not build against the new raise
(round 1) or already held the hold (round 2), with a temporary trace in round 1 showing step 10's
reply `{"pending": true}` before the runner's record check failed. Its findings, for the
coordinator: 048a-4's account-hook part will change the case's setup at steps 3 to 5; a held result
names the action for an Agent caller too (row 44 of the owner's review list); setup's account check
does not lift the hold (row 45); MC9a's H05 wants both diagnostics where PND-005's hold and
SignInUndone meet, 048a-4's; a lapse's own result does not carry `pendingAction`, though IFC-025
names raised actions, and no order holds it; and no check reaches a held Resource through dispatch,
an Agent caller's held result or a kept repeat answered while held. MC9a-003 stays reported, and
nothing lifts a hold.

The first verifier found no must and three shoulds: a state written by a 048a-2 build with an
unanswered sign-in action loads with no hold; step 10 did not check the diagnostic's text; and no
step tied the replacement action's evidence to the `access-decision` record of the request that
raised it. Round 2 changed only the case: step 10 asserts the diagnostic's full text, step 14
asserts its two ordered records and no third and captures the `access-decision` record's ID as
`heldAccess`, and step 15 checks that ID in the replacement action's evidence; the upgrade case it
reported as the finding 048a3-upgrade. The second verifier found one must, that the first start
check printed 0, since 048a-2's Record was not yet on the stacked base, as the dispatch note
foresaw; and two shoulds: step 14 does not assert that its reply and its `request-ended` record name
the same replacement action, and the report gave only `OK` for the Python unit tests and the spike's
last zero-test line for its `tests/spike.rs`. Order 048a-2 then merged (#364), and the coordinator
rebased the branch onto `main` at `506aa144` (order 202's merge).

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on the rebased branch
`d7cce4e8`. It ran the start checks against `origin/main`, which printed as the order expects,
settling the second verifier's must; `cargo fmt --check` and clippy (exit 0); the `sign_in` filter
(6 passed, not the order's 4: main added `retirement_answers_an_unanswered_sign_in_action_void` and
`records::answer_tests::inspection_withholds_sign_in_answer_evidence_from_agents`) and
`pending::tests` (6, not 5: 047d's `notices_follow_their_fleet`); the pieces tool in a scratch copy
(`Nothing to move.`, `conformance/` unchanged); the runner's `requests-sign-in-hold`,
`requests-sign-in-lapse-and-conflict` and `setup-action-` (1, 1 and 3 cases passed) and `requests-`
(65 cases passed; 3 expected failures, one more than 048a-2's Record's 64, with
`expected-failures.yaml` unchanged); the shell test (1 passed), the vectors (2 passed) and the whole
suite (351 passed, 0 failed, 4 ignored); the Python checks (Errors: 0 each), the tool tests (129,
OK), `vector_delta.py` against `origin/main` (1 change, 0 flips, 0 undeclared, exit 0), the
protected paths' status (nothing), `git diff --check origin/main...HEAD` (clean), the spike's
`tests/spike.rs` in a scratch copy (9 passed; 0 failed), the eight checks that print 1 and the two
that print 0. Its negative control, in a scratch copy with the hold block cut out by hand, printed
`FAIL 0 cases passed; 1 failed` on step 10's missing `request-ended` record naming the action, not
on the reply the order foresaw, since the runner checks the records first. It found the code true to
decisions 1 to 4, the tests to decision 5 and the case to decision 6, the prep's three conflict
resolutions true to both orders, and the commits the work-order identity's alone. It did not file
the second verifier's step-14 point, since step 16 checks the same path and the order's step 14 does
not ask for it. It found no must and one should:

- 048a3-R1: a state written by a 048a-2 build that holds an unanswered sign-in action loads with no
  hold, so the instance's model work is dispatched until that action expires and a later lapse
  raises a new action; nothing in the merge recorded it.

The fixer rebased the branch onto `origin/main` at `d891543f` (the thirteen order texts, #366),
which changed only `docs/work-orders/`, with the work-order identity and without conflict, and
applied the finding as it asked, keeping the order as written, since decision 1 has a state without
the member read as before:

- 048a3-R1: a hold begins only at a raise from this build on. A state that a 048a-2 build wrote,
  with an unanswered sign-in action, loads with no hold: the instance's model work is dispatched
  until that action expires and a later lapse raises a new action, which begins the hold. The case
  is row 46 of the owner's review list, for the coordinator; a later order could seed
  `sign_in_holds` on load, `{"since": the action's raise time}` for each unanswered sign-in action.

The fixer then ran the order's checks on the rebased branch: from `engine/` with
`RUST_TEST_THREADS=4` and `CONDUIT_VECTOR_THREADS=4`, `cargo fmt --check` and clippy (exit 0); the
`sign_in` filter and `pending::tests` (6 passed each, as the reader found); the pieces tool
(`Nothing to move.`); the runner's `requests-sign-in-hold`, `requests-sign-in-lapse-and-conflict`
and `setup-action-` (1, 1 and 3 cases passed) and `requests-` (65 cases passed; 3 expected
failures); the shell test (1 passed), the vectors (2 passed) and the whole suite (351 passed, 0
failed, 4 ignored); and the spike's `tests/spike.rs` (9 passed; 0 failed). From the root, with
`/usr/bin/python3`: the order's eight Python checks (Errors: 0 each, `set_names.py --check` its
names line, each exit 0), the tool tests (129, OK) and `vector_delta.py` against `origin/main` (1
change, 0 flips, 0 undeclared, exit 0); the protected paths' status printed nothing, the eight
checks that print 1 printed 1, and the two that print 0 printed 0, the second against `origin/main`.
The negative control was not rerun: the engine and the case are unchanged since the reader's run. On
this commit, `python3 tools/check_docs.py` and `python3 tools/check_private_paths.py` gave Errors: 0
each, and `git diff --check origin/main...HEAD` was clean. The gate's run on this order's squash,
which this merge waits on, is the coordinator's at the merge.
