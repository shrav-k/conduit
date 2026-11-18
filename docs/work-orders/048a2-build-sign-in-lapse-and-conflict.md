# Work order 048a2 (E7a-2): a lapsed login raises the instance's sign-in action, and a second sign-in of one instance reports conflict

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

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s split of batch 1 gives 048a-2 "sign-in as a
kind of pending action, PND-005, SignInUndone. L. After 047, 047b, 047c and 048a-1", and
[order 048a1](048a1-build-sign-in-type-checks.md) (merged as #352) moved IFC-016's detection here,
"which raises PND-005, its other half" (its decision 9 and Record). The targeted review of the
sign-in surface puts two must-state items on 048a-2
([its section 6, 048a-2](../proposals/review-signin-2026-11-16/README.md#048a-2)), which the owner
accepted with their carrier ([ruling 82](../proposals/ruling-82/README.md#the-decisions), point 6,
o1 and o4), and `IN_FORCE.md`, topic 9, holds both as spec pending for this order. Built whole,
that entry is spec batch 1's E7a less 048a-1: the sign-in kind, PND-005's raise and hold,
SignInUndone, setup's account check, `sign-in.v1` itself with IFC-049's relay, and the returns of
PND-013 and PND-014, about 0.8 to 1.1k lines and 18 vectors by the plan. That is several
behaviors, and an order holds one and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). So the coordinator
cuts it, keeping the name 048a-2 for the first part (see "Who decided what"):

- **048a-2, this order:** what the review's section 6 puts on 048a-2, each with its vector, and
  the least of the sign-in kind those vectors need. (1) F84's two halves in PND-005, and the
  engine raising the instance's sign-in action when a call to it reports a lapsed login by either
  half: a keyed implementation's error diagnostic naming PND-005, or a URL elicitation outside
  `sign-in.v1` (IFC-016's detection, moved here). (2) PND-010's refusal of a second `sign-in.v1`
  of one instance as `conflict`, in PND-010 and in `sign-in.v1`'s description, built before the
  stub with the invalid it must follow under OPS-023.
- **The rest of 048a-2's entry**, which later orders build, each after the one before: 048a-3,
  the hold of a held instance (PND-005's `then`); 048a-4, setup's account check, a wrong account
  undone, and SignInUndone (PND-011); 048a-5, `sign-in.v1` dispatched, with IFC-049's relay and
  polling (PND-010, PND-011), which may need its own split; and 048a-6, the returns (PND-013,
  PND-014). 048b-join follows 048a-6. The coordinator records this cut in order 117's entry. This
  order builds none of them.

**What it builds.** The review's item 2: "PND-005 gains F84's two halves: a keyed implementation
reports a lapsed login by ending the call failed or unknown with a diagnostic naming PND-005; a
plain one, only by a URL request (IFC-016). Done when PND-005 states both and a vector with a
scripted lapse shows a keyed call ending with that diagnostic." F84, the coordinator's fix
([decisions](../proposals/full-review/decisions.md#secrets-pending-actions-and-sign-in)): "A lapsed
login takes a form the engine can see. An implementation that carries Conduit's keys ends the call
failed or unknown, as it can tell, with an error diagnostic naming PND-005. A plain implementation
shows it only by asking for a URL (IFC-016)." PND-005 then raises "a sign-in action ... for the
instance, with expiry, unless one is unanswered (PND-001)", and IFC-016 says that "A URL
elicitation at any other point ends the call unknown, raises PND-005 only for a Type stating signIn
with a string account, and reaches no consumer inline, even root. Without that declaration it ends
unknown with a diagnostic naming the absent signIn or expected account, and raises nothing."

The review's item 1: "PND-010's `then`, after the fixed-port sentence: a `sign-in.v1` for an
instance whose earlier `sign-in.v1`, setup or retirement has been dispatched and has not ended is
not dispatched and reports `conflict`, with a diagnostic naming PND-010 and, where the caller may
see it (PND-002), the other invocation's pending action; the same in `sign-in.v1`'s description;
the fixed-port sentence stays `unavailable`. Done when one vector shows the refusal and MC9a is
registered with `conflict` before 048a-2 is dispatched." Under OPS-023 invalid comes before
conflict, and PND-010 says "The Type must state signIn with a string expected account, otherwise
the permitted request reports invalid", so this order builds that invalid too, before the
conflict. The section adds: "048a-2 is drafted after F96's named spec fix", which order 198 made;
`IN_FORCE.md` holds F96 as in spec.

**What orders 047, 047b and 047c bring, which this order assumes.** Order 047 is merged (#353);
047b and 047c are written on `main` and not yet built, and this order starts only once both are
merged (below). It assumes, as their orders state them:

- From 047: `engine/crates/core/src/pending.rs` with `mod pending;` in `lib.rs`; the computed setup
  action in `pending-action.v1`'s form; the test whether an action is raised at a time (before its
  expiry, compared as instants); the decision `raise-setup-action`; and the case
  `setup-action-at-root`.
- From 047b: in `pending.rs`, the holder test and the function that says whether an action is
  raised at a holder at a time, whatever its kind; the view of an action at a holder; the function
  giving every action the state holds, which its decision 3 says "no stored kind exists yet (047c
  and 047d add them here)"; the views raised at a holder, whose number is the count; the
  inspection's `status.pending`, `list.v1`'s `pending` and the `_meta` key `unanswered` on each
  result of an Interface's own request, counted over the state after the step at the step's time;
  and the case `setup-action-by-scope`.
- From 047c: `State`'s member `pending_actions`, a `BTreeMap<String, Value>` keyed by action ID,
  each value `{"action": A, "dispatched": D}`, which no purge touches; the function that gives the
  action an ID names at a time, with its answer, `expired` at or after its expiry; who may answer
  (its decision 2, in ACC-056's step (3)); `answer.v1`'s target, the target authorizing inspection
  of the action's first subject (its decision 3); `answer.v1`'s invalid and conflict in
  `records::preflight`, with the table of the answers each kind accepts through `answer.v1` (its
  decision 4); the record type `pending-action-answered` and the member `pendingAction` on a
  request's record (its decision 5); the function a retirement calls before it sets the
  instance's `state` to `retired`, and each place that calls it (its decision 7 and Record); that
  an action answered at a time is not raised (its decision 8); and the case
  `setup-action-answers`.
- Order 047d (the notice kind) is not required. If it has merged first and stores its kind in
  another form, store the sign-in kind as 047d stores the notice, apply the nearest sound reading
  of decisions 3 and 4 to it, and say so.

Where 047b or 047c merged otherwise than its order says, apply the nearest sound reading of each
decision below to what merged, and say so.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`f72c2bc3`, before orders 047b and 047c; find each place by the text quoted with it.

- `sign-in.v1` is one of C11's stubs: in `invoke_inner` (`requests.rs`, "C11's temporary stubs"),
  after the access decision, the preflights, `available` and the repeat, the entry
  `("sign-in.v1", "048a", "PND-010")` reports unavailable, and `reject_diagnostics` (the arm
  `"PND-010" => Some(format!(`) writes `not built yet, order 048a.` This order keeps the stub and
  its text.
- The preflights that give invalid and conflict run in `invoke_inner` before `available`: first
  `crate::lifecycle::preflight`, then `crate::records::preflight` (the block
  `if (previous.is_none() || redispatch) && let Some(rejection) = crate::records::preflight(`),
  whose `Rejection` carries a specific diagnostic that `reject_diagnostics` writes. Neither handles
  `sign-in.v1`. `lifecycle::busy` is REV-014's test and also counts controls in progress, which
  PND-010's sentence does not name; do not reuse it for decision 6.
- A set-up.v1 invocation keeps its instance's ID as `object`, and so does a retire.v1 invocation,
  including one that a cascade dispatches for a member (`cascade.rs`, `"object":object["id"]`).
- `complete` (`requests.rs:1668`) ends a call from its implementation's completion. For a keyed
  implementation (`plain` not true) it reads the stated outcome from the result's `_meta`, sets
  `rule = None` where the result does not contradict itself (IFC-009), and passes on none of the
  implementation's `structuredContent` for an outcome other than succeeded. A result whose
  `resultType` is `input_required` holds no stated outcome, so it ends unknown under OPS-007
  (keyed) or IFC-010 (plain); nothing reads its `inputRequests`. An engine-decided outcome's cause
  is `explain::outcome_cause` (a row of `OUTCOME_CAUSES`) or `explain::outcome_diagnostic` with a
  cause of the caller's, written `{operation}{ for subject} ended {outcome} under {rule}: {cause}`.
- `inventory.rs`'s `fn own_tools` finds an instance's Type through
  `crate::registration::publication_object` and `crate::schemas::operation`, and reads a value of
  the instance's `config` by a pointer of its Type. Decisions 2 and 6 read the account place so.
- The shell test `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`
  (`engine/crates/shell/src/tests.rs`) sends root's `system.sign-in.v1` for the Resource
  `management` and asserts unavailable with `not built yet, order 048a`. Its Type states no
  `signIn`, so after decision 6 it reports invalid (decision 9).

**When this order starts.** Orders 047b and 047c must be merged to `main` first, and MC9a must be
registered: order 201, MC9a's pre-registration, merges before this order is dispatched. Before
you change anything, check from the repository root that you find what they give, and stop,
change nothing and report if any check prints otherwise:

```bash
grep -c 'fn actions_raised_by_scope' engine/crates/core/src/pending.rs
grep -c 'fn stored_answers_follow_their_ending' engine/crates/core/src/pending.rs
grep -c 'pending_actions' engine/crates/core/src/lib.rs
grep -c '"pending-action-answered"' engine/crates/conformance/src/checks.rs
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
test -f conformance/language/contracts/sign-in-resolution/project/signed-agent.json; echo "fixture $?"
grep -c '^## Record' docs/work-orders/047b-build-actions-raised-by-scope.md
grep -c '^## Record' docs/work-orders/047c-build-stored-answers.md
grep -c 'PND-005 gains F84.s two halves (048a-2)' IN_FORCE.md
grep -c '^- \*Spec pending, order 048a-2\*: a `sign-in.v1` for an instance' IN_FORCE.md
grep -c '^## MC9a' experiments/012-model-checking/README.md
grep -A3 -E '^- id: PND-0(05|10)$' experiments/012-model-checking/account.yaml | grep -c '^  standing: left to MC9$'
```

They print `1`, `1`, a number other than `0`, `1`, `1`, `fixture 0`, `1`, `1`, `1`, `1`, a
number other than `0` and `2`. The second to last is MC9a's pre-registration (order 201), which
the review's item 1 and `IN_FORCE.md` require, with `conflict` for a second `sign-in.v1` of one
instance, before this order is dispatched; the coordinator confirms its content, and you check
only that it is there. The last checks that PND-005's and PND-010's rows in the model-checking
account still stand `left to MC9`, not a model run that carries a digest of their text, so that
this order's change to that text leaves neither row stale; the account is the coordinator's to
update with MC9a. Orders that land beside this one (046b, 047d, 049a-1, 050 and others):
whichever lands second keeps both changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** PND-001, PND-003, PND-005, PND-010, IFC-016 and OPS-023 decide what a sign-in
action is, when it is raised, what it reports and in which order. The review's two items, F84's
words and the conflict's reason are the owner's, accepted with their carrier (ruling 82, point 6,
o1 and o4). Everything else here is the coordinator's, each open to the owner's strike ([ruling
48](../proposals/ruling-48/README.md)): the cut of 048a-2 above (FR-264), as this order's writer
proposed it, into this order and four after it, each after the one before: 048a-3, the hold
(PND-005's `then`: held requests unavailable with `META/pendingAction` and a PND-005 diagnostic,
expiry lifting nothing, a new action on the next held request); 048a-4, setup's account check in
TPL-012's order (PND-011's setup half), with the signIn fixture's scripted account and sign-out
hook items, the wrong-account undo and SignInUndone with its hold; 048a-5, `sign-in.v1`
dispatched, with IFC-049's relay and its remaining invalids before this order's conflict, the
fixed-port refusal, polling to DL18, the signed-in answer and the stub removed, which may need
its own split; and 048a-6, the returns (PND-013, PND-014), lifting the expected failure
`requests-sign-in-return-kept-nowhere`; with 048b-join after 048a-6; the words of decisions 1
to 3, which state the review's items in the promises' own form, with F84's halves in PND-005's
`observable`; keeping the lapse raise and the conflict refusal together in this first part, with
the invalid OPS-023 puts before the conflict, and not cutting the conflict into an order of its
own, since ruling 82 (point 6, o1 and o4) and the review name 048a-2 as their carrier, so that
MC9a's gate and the `IN_FORCE.md` lines stay true as written; that MC9a's pre-registration is
order 201, which merges before this order is dispatched; DL19's duration for a sign-in action,
seven days, as for a setup action; the stored form, its
member `raisedBy`, the evidence items' words and the trigger values; the place where a keyed
implementation's PND-005 diagnostic is read, the result's `structuredContent.diagnostics`, which
neither PND-005 nor IFC-009 states; reading "its contract reports
a missing or lapsed login" as F84's two halves and nothing else; raising for a call to the
instance's own implementation, hook parts included, and for no other call; taking a template,
or an object that is not a Resource or Agent instance, as a Type that states no `signIn`; reading "the other
invocation's pending action" as the instance's unanswered sign-in action, since PND-001 allows one;
reading "has been dispatched and has not ended" as an invocation in progress whose `object` is
the instance; naming no invocation ID in the conflict's diagnostic; the diagnostics' words; that
`answer.v1` answers no sign-in action, with an invalid that names `sign-in.v1`; that retirement
answers it `void` with the retirement's actor and invocation; the decisions' names; the case, its
fixture, route, IDs and times; and the documentation edits. Apply them as written and report them
under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-001, PND-002, PND-003, PND-004, PND-005, PND-009, PND-010, PND-011 and PND-012
   (`spec/promises/pending.yaml`); IFC-007, IFC-009, IFC-016, IFC-025, IFC-049 and IFC-050
   (`spec/interface/implementation.yaml`); OPS-023 (`spec/conventions/operations.yaml`); CTR-013
   (`spec/promises/contracts.yaml`); DL19 (`spec/implementation-defined.yaml`); `sign-in.v1`,
   `answer.v1` and `inspect.v1` (`spec/core/operations.yaml`); `pending-action.v1`,
   `evidence-item.v1` and `diagnostic.v1` (`spec/core/results.yaml`); the rows pending action and
   raised at of `spec/definitions.md`.
2. Orders [047](047-build-setup-action.md), [047b](047b-build-actions-raised-by-scope.md) and
   [047c](047c-build-stored-answers.md), whole, and their Records; [order
   048a1](048a1-build-sign-in-type-checks.md), its decisions 5 and 9 and its Record.
3. The review's [section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists),
   048a-2's items, and V21 and V28 in [its findings](../proposals/review-signin-2026-11-16/findings.md).
4. The cases `setup-action-answers`, `e4-reconcile-setup-action-open`, `explain-outcome-causes`
   and `contracts-sign-in-resolution`, and `conformance/conventions/inventory/e4/`'s
   `manifest.json` and `documents.json`.
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.35 to 0.5k changed lines of Rust with two unit tests, one new case with its
fixture, three places in `spec/`, two lines of `IN_FORCE.md`, five places in
`engine/CONFORMANCE.md` and one entry of a shell test.

## The decisions it applies

1. **PND-005 gains F84's two halves.** In `spec/promises/pending.yaml`, PND-005's `observable`,
   replace these four lines:

   ```text
       caller sends a new request. A mid-call URL elicitation ends that call unknown under
       IFC-016. No action is raised for a Type without signIn or with a null account setting;
       its call ends unknown with an explicit diagnostic. After expiry a new sign-in is raised
       on the next held request or setup, never by expiry alone.
   ```

   with these seven:

   ```text
       caller sends a new request. A mid-call URL elicitation ends that call unknown under
       IFC-016. A contract reports a missing or lapsed login in one of two forms: an
       implementation that carries Conduit's keys ends the call failed or unknown, as it can
       tell, with an error diagnostic naming PND-005; a plain one reports it only by a URL
       request, under IFC-016. No action is raised for a Type without signIn or with a null
       account setting; its call ends as that form ends it, with an explicit diagnostic. After
       expiry a new sign-in is raised on the next held request or setup, never by expiry alone.
   ```

   Nothing else in PND-005 changes: not its
   `when`, its `then`, its `level` or its `vectors`.

2. **PND-010's `then` gains the conflict.** In PND-010's `then`, replace these three lines:

   ```text
       names PND-010 and the port, and names the other invocation's pending action only where
       the caller may see it (PND-002). A request may start its asked-for login or, without one,
       any of its Type's logins. These decisions follow OPS-023's order.
   ```

   with these six:

   ```text
       names PND-010 and the port, and names the other invocation's pending action only where
       the caller may see it (PND-002). A sign-in.v1 for an instance whose earlier sign-in.v1,
       setup or retirement has been dispatched and has not ended is not dispatched and reports
       conflict, with a diagnostic naming PND-010 and, where the caller may see it (PND-002), the
       other invocation's pending action. A request may start its asked-for login or, without one,
       any of its Type's logins. These decisions follow OPS-023's order.
   ```

   The fixed-port sentence stays `unavailable`. Nothing else in PND-010 changes.

3. **`sign-in.v1`'s description says the same.** In `spec/core/operations.yaml`, the Operation
   `sign-in.v1`'s `description`, replace the line

   ```text
       unavailable, naming the port and only a visible pending action (PND-010). The hook's URL
   ```

   with these four:

   ```text
       unavailable, naming the port and only a visible pending action (PND-010). Before any part,
       a sign-in.v1 for an instance whose earlier sign-in.v1, setup or retirement has been
       dispatched and has not ended reports conflict, naming PND-010 and only a visible pending
       action. The hook's URL
   ```

   Nothing else in the file changes.

4. **The sign-in kind, stored.** In `pending.rs`, a sign-in action is a stored kind: an entry of
   047c's `pending_actions`, keyed by the action's ID, whose value is
   `{"action": A, "dispatched": null, "raisedBy": I}`, `A` the action in `pending-action.v1`'s form
   and `I` the ID of the invocation whose call raised it (PND-001: "the action retains its
   attribution to that invocation"). Its members, in this order:
   `{"id": ID, "kind": "sign-in", "subject": [{"kind": K, "name": N, "id": S}], "evidence": E, "expiry": X}`,
   where `ID` is an engine ID allocated from the step's evidence as records' IDs are, `K`, `N` and
   `S` the instance's `kind`, `name` and `id`, and `X` the step's evidence time plus seven days,
   written as 047's expiry is (RFC 3339 in UTC with `Z`, `SecondsFormat::AutoSi`). `E` is a list of
   `evidence-item.v1` items of kind `fact`, in this order:
   1. `V` the expected account; `W` `config` followed by the Type's account pointer (for `/account`,
      `config/account`); `P` `the account this instance is configured for`.
   2. `V` `I`; `W` `records`; `P` `the request whose call reported a missing or lapsed login`.
   3. `V` that invocation's `operation`; `W` `records`; `P` `the operation of that request`.
   4. `V` `lapsed-login` where a keyed implementation's diagnostic named PND-005, and
      `url-elicitation` where the implementation asked for a URL; `W` `records`; `P`
      `how the implementation reported it`.

   No login was started and no attempt made, so no item names a login, a user code or an attempt;
   the order that dispatches `sign-in.v1` adds them. Extend 047c's and 047b's functions so:
   - **The action an ID names** (047c's decision 1): for a stored entry whose action has no answer,
     where a time is given and is at or after its expiry, the answer is
     `{"value": "expired", "at": X}`; otherwise as 047c gives it. A setup entry always holds an
     answer, so its result does not change.
   - **Every action the state holds** (047b's decision 3): add each stored entry whose action's
     `kind` is not `setup`, in the same byte order of ID as the computed actions. The views,
     counts, `status.pending` and `list.v1`'s `pending` follow, and 047c's decision 8 keeps an
     answered or expired one from being raised.
   - **At the inspection** (047b's decision 4): for each listed action whose kind is `sign-in`,
     push a decision named `show-sign-in-action`, citing `PND-001`, `PND-002`, `PND-004` and
     `PND-005`, with `DL19`, and the facts `{"action": ID}`, in place of `raise-setup-action`;
     setup actions keep theirs.
   - **The answers `answer.v1` accepts** (047c's decision 4): the kind `sign-in` accepts none. For
     it the invalid's specific diagnostic text is
     `{tool} reports invalid under PND-009: answer.v1 does not answer a sign-in action; sign-in.v1 answers it signed-in once the expected account is verified, and retirement answers it void.`
     The setup kind keeps 047c's text.

5. **Raised by a lapsed login: F84's two halves and IFC-016's detection.** Add
   `engine/crates/core/src/sign_in.rs` (`mod sign_in;` in `lib.rs`) with a function that gives an
   instance's expected account: given the instance and its Type, as `own_tools` finds them, it
   gives that the Type states no `signIn`, that the value at the Type's `signIn.account` pointer in
   the instance's `config` is not a string (null or absent), or the pointer and the string. An
   instance with no `held` was made at first start from a core template, as the Resource
   `management` of decision 9 is: `publication_object` gives it no `schema_environment`, so
   `schemas::operation` fails for it. For such an instance the function gives that the Type states
   no `signIn`, without that lookup, since no core Type states `signIn`. It gives the same, again
   without a lookup, for an object that is not a Resource or Agent instance (a Pool or a Conduit,
   say) and for a template: `registration.rs` stores a template with `held` and `spec`, its Type
   in `spec.type` and not in `type`, so `own_tools`' lookup would end in an engine error. The
   no-`signIn` texts below then follow, and nothing is raised. In `complete`
   (`requests.rs`), within `Completion::Result { result }`:
   - **A URL elicitation.** Where the result's `resultType` is `input_required` and its
     `inputRequests` holds an entry (a value of an object, or an item of a list) whose `method` is
     `elicitation/create` and whose `params.mode` is `url`, the call ends `unknown` whether the
     implementation is plain or keyed, with the cause rule `IFC-016` in place of `IFC-010` or
     `OPS-007`. The trigger is `url-elicitation`. Nothing of the result reaches the caller: not
     its URL, its message or its request (IFC-016: "reaches no consumer inline, even root").
     `sign-in.v1` never dispatches in this build, so every such call is outside it; the order that
     dispatches it relays its own. This ending follows IFC-016 as it stands, `unknown`, until the
     order that writes ruling 78 point 7 (`IN_FORCE.md`: for a call that changes nothing, an
     unanswered input request ends failed) changes it.
   - **A keyed lapse.** Where the implementation is keyed, the result's stated outcome was accepted
     (`rule` is `None`) and is `failed` or `unknown`, and the result's
     `structuredContent.diagnostics` holds an object whose `ruleId` is `PND-005` and whose `level`
     is `error`, the call ends with that outcome, with the cause rule `PND-005`. The trigger is
     `lapsed-login`. A result that contradicts itself is trusted in neither part (IFC-009), so its
     diagnostics raise nothing.

   The instance is the object whose ID is the invocation's `target`, where it is a Resource or an
   Agent that is not a template; the call may be a request's or a hook part's. For any other
   target, a template included, the account function gives that the Type states no `signIn`, as
   above, so the arms end the call under IFC-016 or PND-005 with a no-`signIn` text and raise
   nothing. Its cause is one
   error diagnostic built with `explain::outcome_diagnostic`, the cause being, by the rule and the
   expected account:
   - `IFC-016` and a string account:
     `The implementation asked for a URL elicitation outside sign-in.v1, and the instance awaits sign-in to its expected account under PND-005.`
   - `IFC-016` and a Type that states no `signIn`:
     `The implementation asked for a URL elicitation outside sign-in.v1, and the instance's Type states no signIn, so no sign-in action is raised.`
   - `IFC-016` and an account that is not a string:
     `The implementation asked for a URL elicitation outside sign-in.v1, and the instance's expected account is not set, so no sign-in action is raised.`
   - `PND-005` and a string account:
     `The implementation reports a missing or lapsed login, and the instance awaits sign-in to its expected account.`
   - `PND-005` and a Type that states no `signIn`:
     `The implementation reports a missing or lapsed login, but the instance's Type states no signIn, so no sign-in action is raised.`
   - `PND-005` and an account that is not a string:
     `The implementation reports a missing or lapsed login, but the instance's expected account is not set, so no sign-in action is raised.`

   No text names the account (PND-005: "without naming that account (PND-004)") or the action's ID:
   the caller may be an Agent, at which no action is raised (PND-002). Then, after `finish` has
   ended the invocation and before `crate::lifecycle::child_ended`, which `complete` calls right
   after it, where the account is a string, call a function in `pending.rs` that raises the
   instance's sign-in action, given the state, the instance, the account and its pointer, the
   invocation, the trigger, the step's time and the Allocation `a`. It first tests whether the
   instance holds a sign-in action that has no answer at that time (decision 4's extension of
   047c's function): where it does, it allocates nothing, stores nothing and gives none (PND-005:
   "unless one is unanswered"); only where that test finds none does it allocate the ID with
   `a.next`, store decision 4's entry and give the action. So the action and the records it adds
   take their IDs before any record that `child_ended` writes for a parent. Where it gave an
   action, add `"pendingAction": [ID]` to the `data` of the `invocation-ended` record that
   `finish` wrote for this invocation, in `t.records` and in `t.state.records`, as 047c's
   decision 5 does for `answer.v1` (PND-012: "Its record names the action's ID"; raising adds no
   record type), and push one decision named `raise-sign-in-action`, citing `PND-001`, `PND-005`
   and `PND-012`, and `IFC-016` for a URL elicitation, with `DL19`, and the facts
   `{"action": ID, "instance": S}`. Add no row to `OUTCOME_CAUSES`. Report every path by which a
   call's result reaches `complete` other than through this arm (a task, a protocol error), and do
   not change it.

6. **`sign-in.v1`'s invalid and conflict, before the stub.** In `sign_in.rs`, add a preflight for
   the core's `sign-in.v1`, and call it in `invoke_inner` right after `records::preflight`'s
   block, under the same condition (`previous.is_none() || redispatch`), giving its rejection
   through `reject_diagnostics` as that block does. It resolves `arguments.instance` with
   `access::object`; where that gives no object, it gives none and leaves the request to the paths
   that follow. Otherwise it calls decision 5's account function, which for a resolved template,
   or an object that is not a Resource or Agent instance, gives that the Type states no `signIn`
   without a lookup, so that such a request reports the first invalid below and never an engine
   error. It gives the function's result to a helper
   in `sign_in.rs`, given the state, the route, the instance, the step's time and that result,
   which decides, in this order (OPS-023):
   1. **Invalid.** Where decision 5's function gives that the Type states no `signIn`, invalid
      under PND-010 with the specific diagnostic
      `{tool} reports invalid under PND-010: the instance's Type states no signIn, so it has no expected account to sign in to.`;
      where the account is not a string,
      `{tool} reports invalid under PND-010: the instance's expected account, at its Type's account pointer {pointer}, is not a string.`
   2. **Conflict.** Where an invocation in state `in-progress` has `object` equal to the instance's
      ID and `operation` the core's `sign-in.v1`, `set-up.v1` or `retire.v1`, the first such in
      `state.invocations`, conflict under PND-010 with the specific diagnostic
      `{tool} reports conflict under PND-010: the instance's earlier {what} has been dispatched and has not ended; send sign-in.v1 again once it ends.`,
      `{what}` being `sign-in.v1`, `setup` or `retirement` by that invocation's operation. Where the
      instance holds a sign-in action that has no answer at the step's time and whose view at the
      request's caller (047b's decision 2) is not none, append ` Its pending sign-in action is {ID}.`
      The text names no invocation ID.

   `{tool}` is the request's tool (the route's `tool`), and each diagnostic is
   `{"ruleId": "PND-010", "level": "error", "message": {"text": T}}`. Otherwise the preflight gives
   none and the stub follows, unchanged. In this build no `sign-in.v1` invocation exists and a
   setup or retirement in progress holds no sign-in action that it raised, so a case reaches
   neither the first `{what}` nor the appended sentence: the unit test pins both. IFC-049's missing
   URL capability, an unknown `signInLogin` and PND-013's return on a first request, each invalid
   before conflict, are not checked here; the order that dispatches `sign-in.v1` adds them before
   this conflict.

7. **Retirement answers it void.** At each place where 047c's Record says a retirement calls its
   decision 6 function for an instance (right before `object["state"] = json!("retired");` in
   `retirement.rs`, and any other it names), also call a function in `pending.rs` that, where the
   instance holds a sign-in action with no answer at the step's time, stores its answer
   `{"value": "void", "actor": the retirement invocation's caller, "invocation": its ID, "at": the time}`
   in its entry, keeping the entry's other members, and gives the action (PND-003: "A retired
   sign-in subject answers void"). Decision 5's raise stores an action only where the instance
   holds none unanswered, so at a step's time it holds at most one unanswered (PND-001), and
   decision 7 answers that one. Where it gives one: the `pendingAction` that 047c passes in the
   `extra` of the `object-change` record whose action is `retired` holds this ID after the setup
   action's, or alone; write 047c's `pending-action-answered` record for it right after that
   record, after the setup action's where there is one; and push one decision named
   `answer-sign-in-void`, citing `PND-003`, `PND-005` and `PND-012`, with `DL19`, and the facts
   `{"action": ID}`. An expired action stays expired.

8. **Two unit tests pin the rules.** Write the values as `json!` values; members a test does not
   need may be left out. Let T be `2026-11-12T12:00:00Z`.
   - In `sign_in.rs`'s `#[cfg(test)] mod tests`, `sign_in_refusals_before_dispatch`, which calls
     decision 6's preflight on an Agent with no `held`, which decision 5's account function gives
     as a Type that states no `signIn`, and decision 6's helper, with decision 5's result written
     by hand (no `signIn`, a null account at `/account`, or the string account), for the other
     bullets, on `State` values built by hand, as 047c's test builds them (root `r`, the
     Interface `v` with an open Conduit to the Fleet `f`, the Interface `w` with none; Agents of
     `f`), and asserts with `assert_eq!` the reason, rule and text: an Agent whose Type states
     no `signIn`, invalid; one whose account is null,
     invalid naming its pointer; one whose account is a string with a set-up.v1 invocation of it
     in progress, conflict with `{what}` `setup`; the same with a retire.v1 instead, `retirement`;
     the same with an in-progress invocation of the core's `sign-in.v1` whose `object` is it,
     `sign-in.v1`; the same setup in progress with an unanswered sign-in action of the Agent, the
     appended sentence for `r` and `v`, and not for `w`; that action expired at the step's time,
     no appended sentence; an invalid Agent with a setup in progress, invalid; with nothing in
     progress, none.
   - In `pending.rs`'s tests, `sign_in_action_raised_by_a_lapsed_login`: with the objects of 047b's
     test without their setup invocations, so that no setup action is raised and every count
     below counts only sign-in actions, and an Agent `s1` of `f` whose account is `primary-account` at `/account`, decision 5's
     raise at T for an invocation `c1` with the trigger `lapsed-login` gives the whole action
     (expiry `2026-11-19T12:00:00Z`, the four evidence items) and stores decision 4's entry; a
     second raise at T+1s for `c2` with `url-elicitation` gives none and stores nothing; the
     action is raised at `r` and at `v` at T+1s, with a count of 1 at `v`, and at neither at its
     expiry; at its expiry the action its ID names has the answer `expired`, and a raise for `c3`
     then gives a new action with another ID and expiry `2026-11-26T12:00:00Z`; 047c's who-may-
     answer test is true for `r`; the accepted answers of `sign-in` are none; on a copy of the
     state as it stood after the raise at T, before `c3`'s, decision 7 at T+1h answers the first
     action `void` with actor `r`, after which it is not raised and a further void gives none; and
     after `state.records` is emptied each stored answer is unchanged.

9. **The shell test.** In `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`,
   the entry for `system.sign-in.v1` (the Resource `management`, whose Type states no `signIn`)
   now expects the reason `invalid` in place of `unavailable`, and its text check the fragment
   `the instance's Type states no signIn` in place of `not built yet, order {order}`; its expected
   `ruleId` stays `PND-010`. Nothing else in the file changes. If 047c left the loop with other
   entries, change only this one, and say how.

10. **One portable case, with its own fixture.** Add the directory
    `conformance/conventions/requests/sign-in-lapse-and-conflict/` with four files.
    - `manifest.json`: e4's `manifest.json` (`conformance/conventions/inventory/e4/manifest.json`)
      with `address` `example.org/sign-in-lapse` and one implementation, `signed-server`, whose
      `command` is e4's `fixture-server`'s, unchanged, whose `implements` are `#ping.v1` and the
      core's `sign-in-hook.v1`, `account-hook.v1` and `sign-out-hook.v1`, written as e4 writes the
      core's address, and which has no `plain` and no `tools`: it is keyed. If the loader requires
      `tools` of it, stop and report.
    - `documents.json`, a list, in this order: e4's Operation `ping.v1`, Capability
      `item-access.v1` and Conduit template `ping`, each as e4 writes it; 048a-1's Operation
      `set-limit.v1` and Type `signed-agent.v1`, each as
      `conformance/language/contracts/sign-in-resolution/project/signed-agent.json` writes it,
      except that the Type's `spec` gains, after `controls`, `"offers": ["#item-access.v1"]` and
      `"implementation"` mapping `#ping.v1` and the three hooks to `#signed-server`, written as
      e4's `agent.v1` maps its operation; two Agent templates, `signed-template` and
      `unset-template`, each with `spec` `{"type": "#signed-agent.v1", "config": {"account": A}}`,
      `A` being `"primary-account"` and `null`; e4's Fleet `group`; the Agents `signed` (from
      `signed-template`) and `unset` (from `unset-template`), each with `owner` `group`; and the
      Conduits `to-signed` and `to-unset`, each from `ping`, with `consumer`
      `{"kind": "Interface", "name": "root"}` and `target` the Agent of its name, as
      `conformance/fixtures/s1/project/instances.yaml` writes `root-workspace`'s.
    - `fixture.json`: e4's `fixture.json`, unchanged in form.
    - `case.yaml`, ID `requests-sign-in-lapse-and-conflict`, `layers: [convention, interface]`,
      `promises: [PND-005, PND-010, IFC-016, PND-001]`, title
      `Refuse a second sign-in of one instance as conflict, and raise its sign-in action when its implementation reports a lapsed login`,
      `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
      `{"fixture": "fixture.json", "manifest": "manifest.json", "documents": "documents.json"}`.
      Write it as JSON with two-space indentation and a final newline, its members in the order of
      `setup-action-answers`. It must meet 046a's portable-case rule as it stands.

    Below, ROOT is root's ID, `00000000-0000-4000-8000-000000000005`. A new step's evidence is its
    time and, as `setup-action-answers` writes its new steps, `ids` the 100 strings
    `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of 60000 + 100n +
    k, for k = 0 to 99, `revisions` the 16 strings `sign-in-` followed by the decimal of n, a `-`
    and the decimal of j, and `suffixes` the 16 strings `s` followed by the four-digit,
    zero-padded decimal of 100n + j, for j = 0 to 15, where n is the step's number; a copied step
    keeps its own evidence. Every new step's time is `2026-11-12T12:00:00Z` unless stated.
    `META/x` is the `_meta` key `x` under the prefix that `spec/names.yaml` sets, written as every
    case writes it, and `U` is the JSON pointer `/reply/_meta/` followed by that key for
    `unanswered`, its `/` written `~1`, as 047b's case writes it. "Root asks T with A" is an
    `invoke` step whose request is
    `{"identity": {"kind": "instance", "id": ROOT}, "tool": T, "arguments": A}`; "a run report"
    and "a completion" for `$I` are a `setup-report` step and a `complete` step for that
    invocation, attempt 1, written as `e4-reconcile-setup-action-open` and
    `explain-outcome-causes` write theirs. Three results recur:
    - LAPSE: `{"kind": "result", "result": {"isError": true, "content": [{"type": "text", "text": "Login lapsed."}], "structuredContent": {"diagnostics": [{"ruleId": "PND-005", "level": "error", "message": {"text": "Login lapsed."}}]}, "_meta": {"META/outcome": "failed"}}}`;
    - URL: `{"kind": "result", "result": {"resultType": "input_required", "inputRequests": {"sign-in": {"method": "elicitation/create", "params": {"mode": "url", "message": "Sign in to continue.", "url": "https://example.org/authorize"}}}}}`;
    - REFUSED(R, RULE): `{"reply": {"isError": true, "_meta": {"META/reason": R}, "structuredContent": {"diagnostics": [{"ruleId": RULE}]}}}`.

    Its steps:
    1. `list-from-first-start`'s step 1 `input`, whole, with `expect`
       `{"reply": {"started": true}, "external_intents": [], "required_promises": ["FST-001"], "state": {"started": true}, "record_facts": []}`
       and no `capture`, as 048a-1's `contracts-sign-in-resolution` step 1 is.
    2. `e4-reconcile-setup-action-open`'s step 1 `input` and `expect`, whole, its evidence kept;
       not its `capture`, whose `/objectIds` pointers a portable case may not hold.
    3. Root asks `group.set-up.v1` with `{"instance": {"kind": "Agent", "name": "signed"}}`.
       `expect` `{"reply": {"pending": true}}`; `capture` `{"setup1": "/reply/invocation"}`.
    4. Root asks `group.sign-in.v1` with `{"instance": {"kind": "Agent", "name": "signed"}}`.
       `expect` REFUSED(`conflict`, `PND-010`): its setup has been dispatched and has not ended.
       This is the review's one vector of item 1.
    5. Root asks `group.set-up.v1` with `{"instance": {"kind": "Agent", "name": "unset"}}`.
       `expect` `{"reply": {"pending": true}}`; `capture` `{"setup2": "/reply/invocation"}`.
    6. Root asks `group.sign-in.v1` with `{"instance": {"kind": "Agent", "name": "unset"}}`.
       `expect` REFUSED(`invalid`, `PND-010`): invalid before conflict (OPS-023).
    7. A run report for `$setup1`, `R` `{"kind": "inspect-run", "state": "stopped"}`. `expect`
       `{"reply": {"pending": true}}`.
    8. A run report for `$setup1`, `R` `{"kind": "run", "outcome": "succeeded"}`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}}`.
    9. and 10. Steps 7 and 8 for `$setup2`, with the same expectations.
    11. Step 4's request. `expect` REFUSED(`unavailable`, `PND-010`): nothing is in progress, and
        the stub answers.
    12. Root asks `access.open.v1` with `{"conduit": "to-signed"}`. `expect`
        `{"reply": {"isError": false}}`.
    13. Root asks `access.open.v1` with `{"conduit": "to-unset"}`. `expect` step 12's.
    14. Root asks `to-signed.ping.v1` with `{}`. `expect` `{"reply": {"pending": true}}`;
        `capture` `{"call1": "/reply/invocation"}`.
    15. A completion for `$call1`, LAPSE. `expect`
        `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "PND-005", "level": "error"}]}}}`:
        a keyed call ends with the diagnostic and raises the action. This is the review's vector
        of item 2.
    16. Root asks `group.inspect.v1` with `{"object": {"kind": "Agent", "name": "signed"}}`.
        `expect`
        `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"pending": [{"kind": "sign-in", "subject": [{"kind": "Agent", "name": "signed"}], "expiry": "2026-11-19T12:00:00Z"}]}}}}`;
        `capture` `{"signin": "/reply/structuredContent/status/pending/0/id"}`.
    17. Step 14's request. `expect` `{"reply": {"pending": true}}`; `capture`
        `{"call2": "/reply/invocation"}`.
    18. A completion for `$call2`, URL. `expect`
        `{"reply": {"isError": true, "_meta": {"META/outcome": "unknown", "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "IFC-016"}]}}}`:
        no second action while one is unanswered.
    19. Root asks `to-unset.ping.v1` with `{}`. `expect` `{"reply": {"pending": true}}`;
        `capture` `{"call3": "/reply/invocation"}`.
    20. A completion for `$call3`, LAPSE. `expect` step 15's: no action for a null account, and
        the count stays 1.
    21. Step 19's request. `expect` `{"reply": {"pending": true}}`; `capture`
        `{"call4": "/reply/invocation"}`.
    22. A completion for `$call4`, URL. `expect` step 18's.
    23. Root asks `group.answer.v1` with `{"action": "$signin", "answer": "dismissed"}`. `expect`
        REFUSED(`invalid`, `PND-009`).
    24. Time `2026-11-19T12:00:00Z`, the action's expiry: step 16's request. `expect`
        `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/status/pending", U]}`.
    25. Same time: step 14's request. `expect` `{"reply": {"pending": true}}`; `capture`
        `{"call5": "/reply/invocation"}`.
    26. Same time: a completion for `$call5`, LAPSE. `expect` step 15's: once the first expired,
        a lapse raises a new action.
    27. Same time: step 16's request. `expect` step 16's with the expiry
        `2026-11-26T12:00:00Z`; no `capture`.

    The route is new and has not been run, since orders 047b and 047c are not yet built. Before
    running it, check that no ID repeats across the case's steps' `ids`. If 046a's runner check
    refuses the case all the same, if the registration rejects the fixture, if a setup asks for a
    report other than the two that steps 7 and 8 send, if a step copied from another case gives a
    reply other than that case states, or if a step's reply differs from what it states, stop and
    report the error: do not change the expectations to pass it, and do not change the inputs
    beyond what this decision says. No expectation is generated from the engine's output. The
    coverage index entry holds `id`, `case`
    (`conventions/requests/sign-in-lapse-and-conflict/case.yaml`), `promises` and, where its
    neighbours in that piece hold it, `safety_rows: []`, in the members and order its neighbours
    use. Write it in a `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
    `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it
    into `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte
    for byte as it was.

11. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each sentence by its text read
    across line breaks, as 047c leaves it. Five places change:
    - In DL18's row in "Choices still required", "Not built: `sign-in.v1` answers unavailable
      under PND-010, with the text" becomes, in the same line, "Not built: `sign-in.v1` answers
      invalid or conflict under PND-010 where order 048a-2's checks apply, and otherwise
      unavailable, with the text"; the rest of the row stays.
    - In DL19's row in "Choices still required", the sentence "No other kind is held, so no other
      duration is chosen." becomes, in the same line:

      ```text
      A sign-in action (PND-005, order 048a-2) expires seven days after the step that raised it. It is raised, unless the instance holds one unanswered, when a call to an instance whose Type states `signIn` with a string account ends with its implementation's error diagnostic naming PND-005 or with a URL elicitation outside `sign-in.v1` (IFC-016); `answer.v1` does not answer it, retirement answers it void, and neither the hold of PND-005 nor `sign-in.v1`'s answer is built yet. No other kind is held, so no other duration is chosen.
      ```

      If 047d changed that sentence first, put the new text before the sentence that ends the row,
      and say so.
    - "The shell also passes answer.v1 to the core, which answers pending actions with it (order
      047c), and sign-in.v1, which the core answers unavailable, naming the order that builds it
      (048a; OPS-005)." becomes "The shell also passes answer.v1 to the core, which answers
      pending actions with it (order 047c), and sign-in.v1, which the core answers invalid or
      conflict under PND-010 where order 048a-2's checks apply, and otherwise unavailable, naming
      the order that builds it (048a; OPS-005)."
    - "`sign-in.v1` answers unavailable, naming order 048a (row DL18 in "Choices still
      required")." becomes "`sign-in.v1` answers invalid or conflict under PND-010 where order
      048a-2's checks apply, and otherwise unavailable, naming order 048a (row DL18 in "Choices
      still required")."
    - In the section "Refusals and reasons (order 046a)", "The unavailable stub of `sign-in.v1`
      now carries a diagnostic but remains unbuilt: order 048a-2." becomes "The unavailable stub
      of `sign-in.v1` now carries a diagnostic but remains unbuilt. Before it, order 048a-2
      decides two of PND-010's refusals: invalid for an instance whose Type states no `signIn` or
      whose expected account is not a string, and conflict while that instance's setup,
      retirement or earlier `sign-in.v1` is in progress. IFC-049's missing capability, an unknown
      `signInLogin` and PND-013's return on a first request, each invalid, are not yet checked, so
      such a request reports conflict or unavailable instead."

      Nothing else in the file changes.
    - After the other decisions, run `python3 tools/promise_standing.py --write`, as AGENTS.md
      asks of a change to a promise, to a case's `promises` and to a promise ID in the core's
      sources. Report the rows of PND-001, PND-005, PND-010 and IFC-016 and the totals; the totals
      change only by what those rows change.

12. **`IN_FORCE.md`.** In topic 9:
    - The bullet that begins `- *Spec pending, orders 050 and 048a-2*: the item that implements`
      becomes these two bullets, each one line, at the same level, in this order:

      ```text
      - *Spec pending, order 050*: the item that implements `sign-in-hook.v1` carries Conduit's keys, in `codex.v2` where it states `signIn` and in `claude-code.v2`; the account, sign-out and retire hooks need none, and an item's `plain` flag is not a member NAM-024 freezes. The turn item's kind is 050's choice, and its turn implementation follows the half of F84 that matches it (82 point 6, o1).
      - **In spec/**: F84: PND-005 says how a missing or lapsed login reaches the engine: an implementation that carries Conduit's keys ends the call failed or unknown with an error diagnostic naming PND-005, and a plain one reports it only by a URL request under IFC-016. The engine raises the instance's sign-in action on either, where its Type states `signIn` with a string account, and the case `requests-sign-in-lapse-and-conflict` shows the keyed half (82 point 6, o1).
      ```

    - The bullet that begins `- *Spec pending, order 048a-2*: a `sign-in.v1` for an instance`
      becomes this one line:

      ```text
      - **In spec/**: PND-010 and `sign-in.v1`'s description: a `sign-in.v1` for an instance whose earlier `sign-in.v1`, setup or retirement has been dispatched and has not ended is not dispatched and reports `conflict`, with a diagnostic naming PND-010 and, where the caller may see it, the other invocation's pending action; the case `requests-sign-in-lapse-and-conflict` shows it, and MC9a was registered with that reason before order 048a-2 was dispatched. PND-010's fixed-port refusal stays `unavailable`, and under OPS-023 it reaches only a sign-in of another instance (82 point 6, o4).
      ```

    Put no date in them: `check_rulings.py` refuses one. Change no other line of `IN_FORCE.md`.

13. **The declaration.** `docs/work-orders/048a2-declared.yaml` holds one `added` shape named
    `requests-sign-in-lapse-and-conflict` for that case. The coordinator wrote it with this order;
    check that it matches and change it only to make it so.

14. **What this order does not do.** No promise other than PND-005 and PND-010, no schema, no
    definition and no ruling's record changes. The instance is not held: requests to it are
    dispatched as before, whatever its sign-in action (the hold of PND-005's `then`, with
    `META/pendingAction` on held results). Setup checks no account, runs no hook of the three and
    undoes nothing, and nothing reports SignInUndone (PND-011). `sign-in.v1` dispatches nothing,
    its stub and its text `not built yet, order 048a` stay, and nothing answers a sign-in action
    signed-in; IFC-049's relay, message and checks, the fixed-port refusal, `signInLogin` and the
    returns of PND-013 and PND-014 are not built, and the expected failure
    `requests-sign-in-return-kept-nowhere` stays as it is. No result carries `pendingAction` for a
    raised sign-in action, and no task request is read. No shell code, CLI command (051b), relay
    (048b-relay, 048b-join) or package (050) changes. Report as findings, for the coordinator:
    that order 048a-4 adds setup's account check, so that this case's setups of `signed` and
    `unset` then dispatch `account-hook.v1` parts, and its steps 3 to 10 change with that order;
    that `IN_FORCE.md`'s other lines naming 048a-2, the expected failure's reason and
    `engine/CONFORMANCE.md`'s stub line name 048a-2 for work this order leaves to 048a-3 to
    048a-6; that neither PND-005 nor IFC-009 states where a keyed implementation puts its PND-005
    diagnostic, so that 050 and a spec fix can name it; that decision 5's URL-elicitation arm is a
    place the order that writes ruling 78 point 7 must change; and any place where PND-005,
    PND-010 or IFC-016 left you a choice this order does not settle.

## Scope

Change only `spec/promises/pending.yaml` (PND-005's `observable` and PND-010's `then`),
`spec/core/operations.yaml` (`sign-in.v1`'s description), `engine/crates/core/src/sign_in.rs`
(new: decisions 5, 6 and 8), `pending.rs` (decisions 4, 5, 7 and 8), `lib.rs` (the module line),
`requests.rs` (`complete` and the preflight's call in `invoke_inner`), `records.rs` only for
decision 4's inspection decision and `answer.v1`'s table where 047c put them, and the invalid
text of `answer.v1` by kind, where 047c writes it, `retirement.rs` and
each other place 047c's Record names for a retirement (decision 7), `inventory.rs` or
`registration.rs` only to expose the Type lookup that `own_tools` uses, without changing what it
gives, `engine/crates/shell/src/tests.rs` (decision 9), `engine/CONFORMANCE.md` (five places),
`engine/STANDING.md`, written only by the tool, `IN_FORCE.md` (decision 12),
`conformance/conventions/requests/index.yaml` and the new directory
`conformance/conventions/requests/sign-in-lapse-and-conflict/`. `docs/work-orders/048a2-declared.yaml`
comes with this order; change it only as decision 13 says. `conformance/manifest.yaml` holds the
new entry only until the pieces tool moves it, and ends unchanged. Not `tools/`, `packages/`, the
shell's code, the experiments, the case schema, `conformance/internal-cases.yaml`,
`conformance/expected-failures.yaml`, any other case or fixture, `docs/implementation/` or
`docs/proposals/`: report any defect as a finding. `experiments/011-short-form-spike/` changes only
if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- sign_in
cargo test --release -p conduit-core --lib -- pending::tests
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cargo run --release -p conduit-conformance -- .. setup-action-
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 3 passed and 0 failed: this order's two tests and 048a-1's
`sign_in_login_faults_name_duplicates_and_overlaps`; report the count if another test's name
matches. The fourth reports 4 passed: 047's, 047b's, 047c's and this order's. The pieces tool
prints `Nothing to move.` The runner commands end `1 cases passed`, `3 cases passed` (the cases of
047, 047b and 047c), and one more case than `requests-` gives on the `main` you start from, with
its one expected failure unchanged; report both counts. The shell test passes. If any other case,
or any shell, CLI or experiment test, fails because a call now ends under IFC-016 or PND-005, or
because `sign-in.v1` now reports invalid or conflict, stop and report it: do not change it. The
threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decisions 5 and 6. From `engine/`, one at a time:

```bash
cp crates/core/src/requests.rs target/requests.rs.fixed
git show HEAD:engine/crates/core/src/requests.rs > crates/core/src/requests.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cp target/requests.rs.fixed crates/core/src/requests.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with an error at
step 4, whose reply reports unavailable where conflict is expected; after the restore the second
ends `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older modification time,
so cargo would not rebuild the crate. Git ignores `target/`. If `HEAD`'s file does not build
against the rest of your change, revert decisions 5 and 6's lines by hand in the copy instead, and
say so.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a2-declared.yaml; echo "exit $?"
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/inventory conformance/conventions/setup conformance/language engine/crates/shell/src/lib.rs tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a2-declared.yaml: 1 shapes.`,
then `Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. Where `check_model_account.py` reports PND-005's or PND-010's row stale (its text
differs from the row's digest), stop and report it: do not change the account, which is the
coordinator's to update with MC9a, and is outside this order's scope. The unit tests, run with the interpreter that has PyYAML, end `OK`; report the count.
The `git status` command prints nothing.

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
grep -c '^mod sign_in;' engine/crates/core/src/lib.rs
grep -c 'fn sign_in_refusals_before_dispatch' engine/crates/core/src/sign_in.rs
grep -c 'fn sign_in_action_raised_by_a_lapsed_login' engine/crates/core/src/pending.rs
grep -rl '"raise-sign-in-action"' engine/crates/core/src | wc -l
grep -rl '"answer-sign-in-void"' engine/crates/core/src | wc -l
grep -c '"id": "requests-sign-in-lapse-and-conflict"' conformance/conventions/requests/index.yaml
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'A sign-in.v1 for an instance whose earlier sign-in.v1, setup or retirement has been dispatched and has not ended is not dispatched and reports conflict, with a diagnostic naming PND-010'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'an implementation that carries Conduit.s keys ends the call failed or unknown, as it can tell, with an error diagnostic naming PND-005; a plain one reports it only by a URL request, under IFC-016'
tr -s ' \n' '  ' < spec/core/operations.yaml | grep -c 'setup or retirement has been dispatched and has not ended reports conflict, naming PND-010 and only a visible pending action'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'A sign-in action (PND-005, order 048a-2) expires seven days'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Before it, order 048a-2 decides two of PND-010.s refusals'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Not built: `sign-in.v1` answers invalid or conflict under PND-010 where order 048a-2.s checks apply, and otherwise unavailable, with the text'
grep -c '^- \*Spec pending, order 050\*: the item that implements `sign-in-hook.v1`' IN_FORCE.md
grep -c '^- \*\*In spec/\*\*: F84: PND-005 says how' IN_FORCE.md
grep -c '^- \*\*In spec/\*\*: PND-010 and `sign-in.v1`.s description' IN_FORCE.md
grep -c '^| PND-005 | convention | must | 1 | 0 | outside | core | case passes |$' engine/STANDING.md
grep -c '^| IFC-016 | interface | must | 1 | 0 | outside | core | case passes |$' engine/STANDING.md
```

Each of these prints 0:

```bash
grep -c 'Spec pending, orders 050 and 048a-2' IN_FORCE.md
grep -c 'Spec pending, order 048a-2' IN_FORCE.md
git diff HEAD --stat -- conformance/expected-failures.yaml conformance/conventions/requests/sign-in-return-kept-nowhere | wc -l
```

This prints `3 2 IN_FORCE.md` (the numbers and the name separated by tabs): two bullets replaced
by three, so no other line of `IN_FORCE.md` changed:

```bash
git diff --numstat -- IN_FORCE.md
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 11 names with the totals and PND-010's row, the function names
and signatures of decisions 4 to 7, each place that decision 7 calls its function, and every path
by which a call's result reaches `complete` outside decision 5's arm.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a2-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
