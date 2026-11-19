# Work order 048a5d (E7a-5d): the sign-in hook part, with the asked-for login passed, and its answer received

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the two cases you add and the two cases you
change, with the promises each tests and whether it passes. This order has network access only
because the engine's tests bind loopback ports, which the sandbox refuses without it; use it for
nothing else, add no crate, and reach no other machine. Run one cargo command at a time. Never
print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `claude`, `codex` or any
client, and sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry for 048a-2 cuts 048a-5 into parts, each
after the one before, since an order holds one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). Order 048a-5's text
(its file `048a5-build-sign-in-refusals-before-parts.md`, under "Where it sits") lists eight, the
fourth being "048a-5d, the `sign-in-hook.v1` part, with `signInLogin` passed to the hook (IFC-006),
and IFC-049's URL match, the URL matched to exactly one stated login and the asked-for one; after
048b-relay2 too." Built whole that is two behaviors, each with its vectors: the hook dispatched and
its answer received, and the URL's checks, about fifteen of them across the four kinds of return
that CTR-013 states. So the coordinator cuts it in two, each after the one before; the cut is the
coordinator's, open to the owner's strike, and this order's refresh records it in order 117's entry
and in [the index of work orders](README.md), which list both:

- **048a-5d, this order:** after an account check that finds no account or another, or that ends
  failed or unknown and so counts as no account (ruling 86, point 3 (d), for a failed check;
  PND-010 as order 207 writes it, for an unknown one), a `sign-in-hook.v1` part, with the asked-for
  login's name in a keyed call (IFC-006); its answer received, plain or keyed alike, by
  `sign-in.v1` and never read as IFC-016's lapse trigger (IFC-010, ruling 86, point 3
  (b)); an answer that is not one URL elicitation ending `sign-in.v1` failed under IFC-049, naming
  the check and the field; and the implementation's own requestState kept with the part, never
  relayed, recorded or shown. A URL elicitation is not yet relayed: it ends `sign-in.v1` with the
  interim ending of orders 048a-5b and 209, which this order moves to that point.
- **048a-5d2, next:** IFC-049's match, the URL matched to exactly one stated login and the asked-for
  one, a failing URL ending `sign-in.v1` failed with a diagnostic naming IFC-049, the check and the
  field, holding no URL value; its Done-when covers a plain hook's URL matched (ruling 86, point 4).
  048a-5ef comes after it.

The other parts stand as order 117's entry lists them since the owner joined the last four in pairs
on 2026-11-19 (ruling 88): 048a-5ef, the engine's message and the input-required output that order
048b-relay1 relays, which removes the interim ending, and polling on the retry; and 048a-5gh,
another account undone while polling, and DL18's end. This order builds none of them. The texts it
writes into the engine and `engine/CONFORMANCE.md` keep the name 048a-5e, as orders 048a-5d2 and
048a-5ef expect to find them, and 048a-5ef removes every one. It is on the sign-in chain, so
A77's amendment of 2026-11-19 does not pair it ([work orders](../implementation/WORK_ORDERS.md)).

**This text's refresh.** The coordinator wrote this order against `main` at `3c761dd9` and
refreshed it on 2026-11-19 against `main` at `c097ec81`, where orders 048a-5b (#387), 048a-5c
(#397) and 048a-6 (#386) have merged with their Records and ruling 86's record and order 207's text
(#396) have merged, and against the texts of order 207, whose build is in progress, and order 209
(`209-build-keyed-hooks-and-sign-in-fixes.md`, not yet on `main`). Every change below is the
coordinator's, open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):

- the cut into 048a-5d and 048a-5d2 is settled and recorded in order 117's entry and the index;
- this order dispatches only after order 207 has merged and order 209's work is on its base, and
  its start check stops where either was merged or built otherwise than its text says (the
  dispatch refresh below lets that base be 209's round-1 branch);
- the hook part also follows a check that ended failed or unknown, replacing order 209's interim
  ending for it as well as 048a-5b's (ruling 86, point 3 (d), for a failed check; PND-010 as order
  207 writes it, for an unknown one);
- the hook's answer is taken by `sign-in.v1` under IFC-010's sentence that order 207 adds (ruling
  86, point 3 (b)), even where its `sign-in.v1` has already ended, so that no URL answer of a
  sign-in hook is ever read as IFC-016's lapse trigger, and a new case shows a plain hook's URL
  request so taken (ruling 86, point 4);
- the asked-for login travels through a new parameter of the part function, set before any intent
  is written (the second reading's M3), and a plain hook is never asked for one, since order 209
  refuses such a request before any part (ruling 86, point 3 (c));
- the hook part's interim and failed endings cite IFC-010 for the part;
- the steps of order 048a-5b's case that this order changes are four, not three, its step 24
  having taken the no-account path since order 209, and three steps of order 209's cases change
  too: steps 13 and 24 of `requests-keyed-hook-dispatch`, the second added by 209's refresh, and
  step 15 of `requests-sign-in-setup-check-failed-hook`, added by it (209's "What follows");
- order 048a-5c's case completes two account checks of `sign-in.v1` (its steps 22 and 23), each
  with the expected account, so neither reaches the hook (the earlier assumption said none);
- the decision name `end-sign-in-unrelayed`, the constant `HOOK_UNRELAYED_CAUSE` and the opening
  "From order 048a-5d," of the new DL18 text stand on this order's side; order 048a-5ef's text,
  written after this refresh by joining the drafts of 048a-5e and 048a-5f, takes them as they
  stand (the second reading's M2);
- the comment on `retry_intent`, which names this order among its callers, is corrected;
- every cited line, test count, case step, expected failure, STANDING row and grep is rechecked
  against `c097ec81`, and those that orders 207 and 209 change are named in the start check.

**Its dispatch refresh.** The coordinator refreshed it again on 2026-11-19, for its dispatch,
against `main` at `d4343f71`, where order 209's text has merged (#407) and 209's build has not.
Every change below is the coordinator's, open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- it was checked against order 209's text as merged, whole: the four points that 209's refresh
  asks of this order already stand here (the start check's line for `requests-keyed-hook-dispatch`
  at 24 steps; decision 10's arithmetic, that case having 26 steps after this order, its old
  step 24 now 25 and going on to the plain hook; step 15 of
  `requests-sign-in-setup-check-failed-hook`; and the section's last sentence as 209 words it,
  "from the `sign-in-hook.v1` part, plain or keyed"), and no decision changes for them;
- decision 10's third case captures its hook as `signhook1`, since its step 6 already binds
  `hook1`, and decision 8 adds a check that no capture name is bound twice (the dispatch review);
- under the owner's choice on 2026-11-19 of the lever "Pipeline the chain", as the coordinator
  applies it: on the predecessor's round-1 branch (the coordinator's notes (private); not yet in a
  ruling), this order may start from order 209's round-1 branch, before 209 merges: the start
  check accepts either base and says how to tell them apart, and the admin paragraph at the foot
  lists what the coordinator rebases when 209 merges;
- under the owner's choice on 2026-11-19 of the lever "Less prescriptive orders", as the coordinator
  applies it (the coordinator's notes (private); not yet in a ruling), the start check keeps only
  what the work needs: the checks of predecessors' Records and of this order's cut in order 117 are
  dropped, so is the grep that pinned the line on which order 209 writes `dispatch_hook`'s literal,
  which 209's cases, passing in the start's runner runs, replace, and so are the lengths of cases
  that no decision reads; the lengths of 209's two cases stay, since decision 10 names their last
  steps;
- in "Done when", the lengths of the three changed cases are given against the start, the
  vectors run is folded into the whole engine suite, which runs once, last, after the mutations
  (a readiness finding in the coordinator's notes (private)), the spike's test count is no longer
  pinned, and the greps of function and test names that compilation and the test count already
  guarantee are dropped (the dispatch review).

**What it builds.** One behavior, the hook part and its answer. The quotes below write the `_meta`
prefix that `spec/names.yaml` sets as `META/`, and give PND-010, IFC-006, IFC-009 and IFC-010 as
order 207 writes them. PND-010's `observable`: "If that check succeeds with no account or another
account, or fails or ends unknown, the engine invokes sign-in-hook.v1 as a part and relays its URL
elicitation under IFC-049; that initial mismatch or failure does not undo the account or end
sign-in.v1. A check there that failed or ended unknown counts as no account: it raises nothing that
PND-005 does not raise for a contract that reports a missing or lapsed login, and clears no
condition", up to the relay; "The engine passes the asked-for name to sign-in-hook.v1 (IFC-006),
which starts that login or fails with a diagnostic naming PND-010 and why it could not ... Without
a name the hook chooses among the Type's logins. A plain hook, one whose implementation is marked
`plain` and whose item's `keyed` does not list sign-in-hook.v1, gets no Conduit keys (IFC-007),
hence no name, so a request that asks it for a login reports invalid before any part". IFC-006: "A
sign-in-hook.v1 call also holds `META/signInLogin`, {name}, where the person asked for a login",
for a call to an implementation that carries Conduit's keys "or a call for a hook that the `keyed`
of an implementation marked `plain` lists (PKG-020)". IFC-010's `then`: "For an `input_required`
result holding a URL-mode input request from a part of sign-in.v1, the engine follows this rule in
place of the terminal outcome and output checks of IFC-009, OPS-006 and OPS-007, for a plain call
and for a call that carries Conduit's keys. From the sign-in-hook.v1 part, it relays the request
under PND-010 and keeps the part and sign-in.v1 in progress when the request passes IFC-049's
checks. This is IFC-016's relay of URL-mode elicitation arising as a part of sign-in.v1. A request
from that part that fails a check of IFC-049 ends sign-in.v1 failed under IFC-049." IFC-009's
`when`: "IFC-010 governs a URL-mode `input_required` result from a part of sign-in.v1 in place of
this rule." IFC-049's form of the answer: "InputRequiredResult with resultType input_required and
exactly one inputRequests entry: elicitation/create with mode url, message and url", and "No text
from the hook is relayed". IFC-049: "A URL failing a check is not relayed: sign-in.v1 ends failed
with a diagnostic naming IFC-049, the check and field, holding no URL value, code or state." This
order checks the answer's form; the checks of the URL itself are 048a-5d2's, and the relay, which
keeps the part and `sign-in.v1` in progress, is 048a-5ef's. No promise changes: order 207 states all
of it, and this order builds it.

**The decision this order follows.** The coordinator's plan for the chain binds where a sign-in's
requestState lives: the shell issues, keeps and checks the requestState it relays (order
048b-relay1), and the core holds none of it. The implementation's own requestState, from the hook's
`InputRequiredResult`, is another value: the core keeps it with the hook part, for that part's
retry (048b-relay2; 048a-7, after the dogfood under ruling 88), and never relays, records or shows
it. This order keeps it, with the key of the hook's one input request, which that retry answers
under. Since this order ends the part as soon as its answer is read (decision 4), the value lives
only within that step here; 048a-5ef keeps the part in progress, and with it the value. Order
048b-relay2's Record (its R1) asks to show that the runner neither drops nor rewrites the implementation's requestState, the kept value
equalling the implementation's byte for byte. This order answers only the core's half of R1:
decision 6's first test, `sign_in_hook_answer_form`, pins that `hook_url_action` gives the value
byte for byte. The runner's half stays open, for 048a-5ef or 048a-7. Nothing observable in this
order pins the call of `keep_hook_input` in `hook_input`, since `finish` removes the member in the
same step: a build that never makes that call passes every unit test, case and mutation here.

**What the orders before it bring, which this order assumes.** Orders 048a-2 to 048a-5c, 048a-6,
048b-relay1 and 048b-relay2 have merged, each with its Record; order 207 must have merged, and
order 209's work must be on the base this order starts from, merged to `main` or on 209's
round-1 branch ("When this order starts"). It assumes, as their decisions and the amendments their Records accept
state them; where a Record names a function or member otherwise, use that name:

- **(a) From 048a-2:** `engine/crates/core/src/sign_in.rs`, its `expected_account` (giving
  `NoSignIn`, `NotString { pointer }` or `Set { pointer, account }`) and `url_elicitation(result)`;
  and in `complete` (`requests.rs`) the arm `Completion::Result { result } if
  sign_in::url_elicitation(&result) =>`, which sets `lapse` to `sign_in::Trigger::UrlElicitation`,
  so that `finish_login_lapse` ends the call unknown under IFC-016 and raises the instance's sign-in
  action, called "the URL arm" below. It reads neither `plain` nor the part.
- **(b) From 048a-4 and 048a-5b:** `setup_sign_in_hook(t, a, c, parent, object, name, promises) ->
  Result<bool>` in `requests.rs`, through `dispatch_hook`, called "the part function" below: it
  dispatches a part afresh (attempt 1, key null), with the parent's caller, sender and filters,
  `setup_parent` and `serving` the parent's ID, no `conduit` key, and the reply
  `{"pending":true,"invocation":<parent>,"hook":<part>}`, leaving the parent's phase to its caller;
  it gives `false` and dispatches nothing where no implementation is resolved, and names its
  decision by the operation (`dispatch-account-check` or `dispatch-sign-out`). Its callers are
  `sign_in::begin` and `lifecycle.rs`'s `dispatch_sign_in_part`.
- **(c) From 048a-5:** `request_member_refusal` in `sign_in.rs`; `AccessRequest`'s members
  `client_capabilities`, `sign_in_login` and `sign_in_return`; the case
  `requests-sign-in-refusals-before-parts` and its definitions S, URL, LOGIN(X) and "Root asks T
  with A and M".
- **(d) From 048a-5b:** C11's stub removed; `sign_in::begin`, which sets the invocation's `object`
  and its member `sign_in` to `{"phase": "account-pending", "parts": []}` and dispatches the account
  check; `child_answered` (`lifecycle.rs`) returning `sign_in::part_answered(t, a, c, i, v, outcome)`
  where the parent's `operation` is the core's `sign-in.v1` and it is in progress; `part_answered`,
  which returns, changing nothing, where `sign_in.phase` is not `account-pending`, and otherwise
  pushes `{"name": "account", "outcome": O, "invocation": ID}` onto `sign_in.parts`, ends the
  sign-in failed under `PND-003` where no object has the parent's `object` as its ID or the instance
  is retired, reads the check with `first_check` (`Expected`, `NoAccount`, `Other`, `NotChecked`),
  clears `SignInUndone` for `Expected` and `NoAccount`, signs in on `Expected` (`signed_in`), and,
  for `Other` unless a retirement of the instance is in progress, raises the sign-in action in its
  fourth form (`SignInRequest::SignIn`), keeping its ID in `sign_in.raised`; the end, `end_failed(t,
  a, c, i, rule, cause)`, which ends the invocation through `finish` failed with one diagnostic from
  `explain::outcome_diagnostic`, sets `sign_in.phase` to `ended` and links `sign_in.raised` to the
  `invocation-ended` record as `pendingAction` (`link_action`); the case
  `requests-sign-in-account-first` (32 steps), with SIGN(N), FAILED(N, RULE), HELD2(A) and INSPECT;
  and that a cancel of `sign-in.v1` (`requests::cancel`) ends it but leaves its part in progress
  (its Record's V7, open).
- **(e) From 048a-5c:** the asked-for login's name kept on the `sign-in.v1` invocation as the
  member `asked_login`, a string, absent where the request asked for none, written once in
  `invoke_inner` and read once, in `other_sign_in_ports`; 048a-5c foresaw this order reading it.
  Its fixed-port refusal answers after the conflict and before any part. Its case
  `requests-sign-in-fixed-port` completes two account checks of `sign-in.v1`, its steps 22 and 23,
  each with the expected account, so neither reaches the hook, and it changes nothing for this
  order.
- **(f) From 048b-relay1:** `InputRequired`, `Transition.input_required` and the `respond` input.
  This order reads none of them.
- **(g) From 048b-relay2:** `intent(v, state, c) -> Result<Value>`, which calls
  `dispatch_intent(v, url_elicitation, c) -> Value`, so that each call to an instance whose Type
  states `signIn`, a plain call included, declares `{"elicitation": {"url": {}}}` under
  `io.modelcontextprotocol/clientCapabilities` in its `_meta` (IFC-045); `retry_intent(first,
  request_state, responses)`, which this order does not call (048a-7 does); and a part's own
  `InputRequiredResult` reaching the core as the completion `{"kind": "result", "result": R}`.
- **(h) From order 207 (keys per hook and the sign-in fixes in `spec/`):** the texts quoted above;
  `manifest.v1`'s `keyed`; and its four cases, among them `requests-sign-in-login-plain-hook`,
  whose `manifest.json` is `requests-sign-in-lapse-and-conflict`'s with its one item,
  `signed-server`, marked `plain: true`, given `tools` (the core's `sign-in-hook.v1` named
  `sign_in_hook`, `account-hook.v1` `account_hook`) and listing nothing under `keyed`, and whose
  steps 1 and 2 are first start and registration.
- **(i) From order 209 (D's engine half, (c) and (d)), as its text states it and as built on your
  base:** `keyed_hook(item, operation) -> bool` in `requests.rs`; `dispatch_hook` setting the invocation's `plain` per call,
  `false` for a hook that a plain item lists under `keyed`; `dispatch_intent` naming the tool from
  `tools` where the call or its item is plain; for a keyed hook of a plain item, a `server/discover`
  intent first, the `invocation-dispatched` record and the hook's own `tools/call` intent written
  only once discovery lists Conduit's extension, from the invocation as it then stands, a result
  without it ending the hook `unavailable` under IFC-037 through `child_answered`, and a listed
  result whose parent has ended ending it unknown under OPS-027; (c), `request_member_refusal`
  refusing before any part a stated `signInLogin` aimed at a plain sign-in hook; (d) in
  `sign-in.v1`, `part_answered`'s `NotChecked` arm split by the part's outcome, `failed` or
  `unknown` clearing and raising nothing and taking the interim ending, the decision
  `end-sign-in-before-relay` with the facts `{"invocation": S, "found": "none", "check": O}` and
  `end_failed` under `PND-010` with a cause that holds "which counts as no account (PND-010,
  PND-011); sign-in.v1 would go on", any other outcome, a reason, keeping 048a-5b's `PND-011`
  ending; `complete`'s URL arm kept ahead of the reading of a keyed result (its decision 2), so
  that an account check of `sign-in.v1` whose call returns a URL-mode `input_required` result ends
  unknown, whatever outcome it states, and counts as no account, the URL arm raising the
  instance's sign-in action as it does today; the step 24 of `requests-sign-in-account-first`
  expecting `PND-010`; its case `requests-keyed-hook-dispatch` (24 steps, on 048a-3's fixture with
  a manifest of its own whose plain `signed-server` lists only `account-hook.v1` under `keyed`),
  whose step 13 completes the account check `$check1` with a keyed result that states no outcome
  and expects `sign-in.v1` ended failed under `PND-010`, with no external intent and no
  `unanswered` key, and whose step 24, its last, completes the keyed account check `$check4` of
  `$sign4` with a URL-mode `input_required` result and expects `$check4` ended unknown and
  `sign-in.v1` ended failed under `PND-010`; its case `requests-sign-in-setup-check-failed-hook`
  (16 steps, on 048a-3's fixture with a manifest and documents of its own, its `signed-server` not
  marked `plain`), whose step 15 completes the account check `$check1` of `$sign1` failed and
  expects `sign-in.v1` ended failed under `PND-010` with `META/unanswered` 2, and whose step 16
  inspects the instance; its case `requests-keyed-not-in-implements`; order 207's expected
  failures removed; and in `engine/CONFORMANCE.md` DL18's sentence "Until order 048a-5e builds the
  relay, a check that finds no account or another, or that ends failed or unknown and so counts as
  no account, clearing no condition (order 209), ends `sign-in.v1` failed, ..." and its section
  "Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)", whose last sentence reads
  "IFC-010's relay of a URL request from the `sign-in-hook.v1` part, plain or keyed, has no case
  here: this build runs no `sign-in-hook.v1` part; orders 048a-5d and 048a-5e build the hook part
  and the relay, whose cases show it."

Where one of them merged, or order 209 was built on your base, otherwise than this says, apply the
nearest sound reading of each decision below to what you find, and say so; but where the start
check below prints otherwise for order 207's or 209's work, stop.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`c097ec81`, before orders 207 and 209; order 209 moves several. Find each place by the text quoted
with it.

- `dispatch_intent` (`engine/crates/core/src/requests.rs:812`) writes every call: for an
  invocation whose `plain` is not true, in the block `if v["plain"] != true {` (line 829), the
  `_meta` of Conduit's keys, then `if !v["key"].is_null() {` (line 845) with `idempotencyKey`,
  then the line that sets the call's `params._meta` to `meta` (line 848); then, under the comment
  that begins `// IFC-045: URL elicitation is declared only` (line 850), the capability, for a
  plain call too.
- `retry_intent` (line 865) is marked `#[allow(dead_code)] // Orders 048a-5d and 048a-7 call it.`
  (line 864).
- `setup_sign_in_hook` (line 2458) names its decision in `let decision_name = if name ==
  "account-hook.v1" {` (line 2480). `dispatch_hook` (line 2500) builds the part's literal (line
  2528), pushes it, writes the record `invocation-dispatched` and pushes `intent(&v, &t.state, c)?`
  (line 2545); order 209 sets the literal's `plain` per call and, for a keyed hook of a plain item,
  defers the record and the call.
- `complete` (line 1938) makes its `Allocation` (`let mut a = Allocation {`, line 1985) and `let
  plain = v["plain"] == true;` (line 1989) before its `match completion`, in which the URL arm
  (line 2044) comes before the plain and keyed reading of any other `Completion::Result`; the
  IFC-026 arm returns `crate::lifecycle::child_answered(t, &mut a, c, &v, reason.unwrap())` (line
  2109) with `refused` or `unavailable`. Otherwise it ends the call through `finish` or
  `finish_login_lapse` and calls `crate::lifecycle::child_ended`.
- `finish` (line 1857) sets `state`, `ended` and `cause`, removes `previous`
  (`v.as_object_mut().unwrap().remove("previous");`, line 1879), writes `invocation-ended` and the
  request's end, and sets `t.reply` to the invocation's envelope; the runner adds `unanswered` to an
  error reply where the caller sees unanswered actions.
- `requests::cancel` (line 2232) ends an invocation with no `setup` or `retirement` member, a
  `sign-in.v1` among them, as an external call; `requests::recover` (line 2333) ends every
  invocation in progress unknown under IMP-007 through `finish`, calling no `child_ended`.
- In `sign_in.rs`: `begin` (line 132); `part_answered` (line 208), its test of the phase
  `account-pending` (line 217), its arm `FirstCheck::NotChecked => {` (line 253) and its decision
  `"end-sign-in-before-relay"` (line 305); `end_failed` (line 378); `url_elicitation` (line 475);
  `request_member_refusal` (line 660).
- `child_answered` (`lifecycle.rs:1201`) returns at once where the parent is not `in-progress`, and
  calls `sign_in::part_answered` (line 1221).
- `explain::outcome_diagnostic(invocation, outcome, rule, cause)` (`explain.rs:875`) writes
  `{operation}{ for subject} ended {outcome} under {rule}: {cause}`.
- `sign-in-hook.v1` states an `effect` and no `outputSchema`, so a plain hook's error is unknown
  under IFC-010; `account-hook.v1` states neither `effect` nor `startsModelWork`, so a plain account
  check's error is failed (`spec/core/operations.yaml`).

**When this order starts.** Order 207 must have merged to `main`, after orders 048a-5c and 048a-6.
Order 209's work must be on your base, the commit your working tree starts from (`HEAD`), in one of
two ways, either of which serves: 209 has merged to `main`; or the coordinator dispatched this
order stacked on 209's round-1 branch, whose tip holds 209's build, committed, before 209's read
and merge. You tell them apart by 209's Record: the admin writes it at the merge, so on 209's
round-1 branch its file still ends "Not yet run." (the first check's second line). On either base,
209's three cases are present and pass (the second check, and the start's runner runs below).
Report which base you started from and `HEAD`'s commit. Nothing below reads which it is. Before you
change anything, check from the repository root that you find what they give, and stop, change
nothing and report if any check prints otherwise:

```bash
grep -c '^Not yet run\.$' docs/work-orders/207-keyed-hooks-and-sign-in-spec-fixes.md docs/work-orders/209-build-keyed-hooks-and-sign-in-fixes.md
ls -d conformance/conventions/requests/keyed-hook-dispatch conformance/conventions/requests/keyed-not-in-implements conformance/conventions/requests/sign-in-setup-check-failed-hook 2>/dev/null | wc -l | tr -d ' '
grep -c '^    keyed:$' spec/schemas/manifest.yaml
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'From the sign-in-hook.v1 part, it relays the request under PND-010 and keeps the part and sign-in.v1 in progress when the request passes IFC-049.s checks.'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'A request from that part that fails a check of IFC-049 ends sign-in.v1 failed under IFC-049.'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'IFC-010 governs a URL-mode .input_required. result from a part of sign-in.v1 in place of this rule.'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'or a call for a hook that the .keyed. of an implementation marked .plain. lists (PKG-020)'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'If that check succeeds with no account or another account, or fails or ends unknown, the engine invokes sign-in-hook.v1 as a part'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'so a request that asks it for a login reports invalid before any part'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'for a plain call and for a call that carries Conduit.s keys'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'This rule also applies when a call that carries Conduit.s keys for a part of sign-in.v1 returns an .input_required. result holding a URL-mode input request.'
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -o 'IFC-010 governs a URL-mode .input_required. result from a part of sign-in.v1 in place of this rule.' | wc -l | tr -d ' '
grep -c 'requests-keyed-not-a-hook\|requests-sign-in-login-plain-hook\|requests-sign-in-setup-check-failed' conformance/expected-failures.yaml
grep -c 'pub(crate) fn keyed_hook(item: &Value, operation: &str) -> bool' engine/crates/core/src/requests.rs
grep -rl 'server/discover' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'which counts as no account (PND-010, PND-011); sign-in.v1 would go on' engine/crates/core/src/sign_in.rs
grep -c 'sign-in.v1 would go on to start a login' engine/crates/core/src/sign_in.rs
grep -rl '"end-sign-in-before-relay"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'pub(crate) fn part_answered(' engine/crates/core/src/sign_in.rs
grep -c '^fn end_failed($' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn setup_sign_in_hook($' engine/crates/core/src/requests.rs
grep -c '^pub(crate) fn retry_intent($' engine/crates/core/src/requests.rs
grep -c '^pub struct InputRequired {$' engine/crates/core/src/lib.rs
grep -c 'Completion::Result { result } if sign_in::url_elicitation(&result) => {' engine/crates/core/src/requests.rs
grep -rho '"asked_login"' engine/crates/core/src | wc -l | tr -d ' '
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'No .sign-in-hook.v1. part runs, so no URL action waits and no wait is chosen. Until order 048a-5e builds the relay, a check that finds no account or another, or that ends failed or unknown and so counts as no account, clearing no condition (order 209), ends .sign-in.v1. failed'
grep -c '^## Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)$' engine/CONFORMANCE.md
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'has no case here: this build runs no .sign-in-hook.v1. part; orders 048a-5d and 048a-5e build the hook part and the relay, whose cases show it.'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-account-first/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (17, 21, 23, 29)], [[d["ruleId"] for d in s[k]["expect"]["reply"]["structuredContent"]["diagnostics"]] for k in (17, 21, 23, 29)])'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/keyed-hook-dispatch/case.yaml"))["steps"]; e = s[12]["expect"]; print(s[12]["input"]["invocation"], [v for k, v in e["reply"]["_meta"].items() if k.endswith("/outcome")], [d["ruleId"] for d in e["reply"]["structuredContent"]["diagnostics"]], e["external_intents"], s[13]["input"]["request"]["tool"], len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/keyed-hook-dispatch/case.yaml"))["steps"]; e = s[23]["expect"]; print(s[23]["input"]["invocation"], s[23]["input"]["completion"]["result"]["resultType"], [d["ruleId"] for d in e["reply"]["structuredContent"]["diagnostics"]], e["external_intents"], [r["data"] for r in e["record_facts"]])'
python3 -c 'import json; i = json.load(open("conformance/conventions/requests/keyed-hook-dispatch/manifest.json"))["implementations"]["signed-server"]; print(i["plain"], [k.split("#")[1] for k in i["keyed"]], [v for k, v in i["tools"].items() if k.endswith("#sign-in-hook.v1")])'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-setup-check-failed-hook/case.yaml"))["steps"]; e = s[14]["expect"]; print(s[13]["input"]["request"]["tool"], s[14]["input"]["invocation"], [d["ruleId"] for d in e["reply"]["structuredContent"]["diagnostics"]], e["external_intents"], s[15]["input"]["request"]["tool"], len(s))'
python3 -c 'import json; i = json.load(open("conformance/conventions/requests/sign-in-login-plain-hook/manifest.json"))["implementations"]["signed-server"]; print(i["plain"], "keyed" in i, [v for k, v in sorted(i["tools"].items()) if k.endswith("-hook.v1")])'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-login-plain-hook/case.yaml"))["steps"]; print(s[0]["input"]["kind"], s[1]["input"]["kind"])'
python3 -c "import json; print(*[{k: v.get('plain') for k, v in json.load(open('conformance/conventions/requests/%s/manifest.json' % d))['implementations'].items()} for d in ('sign-in-lapse-and-conflict', 'sign-in-hold', 'sign-in-setup-check-failed-hook')])"
grep -rl '"dispatch-sign-in-hook"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'mark(c, "signInLogin")' engine/crates/core/src/requests.rs
grep -rl 'hook_input' engine/crates/core/src | wc -l | tr -d ' '
ls -d conformance/conventions/requests/sign-in-hook-part conformance/conventions/requests/sign-in-plain-hook-part 2>/dev/null | wc -l | tr -d ' '
```

They print, in order:

1. two lines: `207-keyed-hooks-and-sign-in-spec-fixes.md:0`, order 207's Record; and
   `209-build-keyed-hooks-and-sign-in-fixes.md:0` where 209 has merged, or `:1` on 209's round-1
   branch; report which;
2. `3`: order 209's three cases, on your base either way;
3. `2`: order 207's `keyed` in `manifest.v1`;
4. to 9. `1` six times: order 207's texts that this order builds on, IFC-010's relay from the
   `sign-in-hook.v1` part and its IFC-049 failure, IFC-009's deferral to IFC-010 for a URL-mode
   result of a part of `sign-in.v1`, keyed or plain, IFC-006's call for a keyed hook, PND-010's
   failed or unknown check going on to the hook, and its refusal of a login aimed at a plain hook;
   where any of these prints `0`, stop: decision 3 relies on them;
10. to 12. `1`, `1` and `2`: order 207's texts that decision 3's "plain or keyed alike" relies on,
    IFC-010's rule "for a plain call and for a call that carries Conduit's keys", IFC-010's `when`
    sentence for a call that carries Conduit's keys for a part of `sign-in.v1`, and the deferral to
    IFC-010 in the `when` of OPS-006 and OPS-007; where any prints otherwise, stop;
13. `0`: order 209 removed order 207's three expected failures;
14. `1` and a number other than `0`: order 209's `keyed_hook` and discovery; that `dispatch_hook`
    reads `plain` per call is shown by 209's cases passing in the start's runner runs below;
15. `1` and `3`: order 209's interim cause, and the three interim causes, 048a-5b's two and 209's,
    that hold "sign-in.v1 would go on to start a login";
16. `1`: the file that writes `end-sign-in-before-relay`;
17. `1`, `1`, `1`, `1`, `1` and `1`: 048a-5b's `part_answered` and `end_failed`, the part
    function, 048b-relay2's `retry_intent`, 048b-relay1's `InputRequired` and 048a-2's URL arm;
18. `2`: 048a-5c's `asked_login`, written once and read once;
19. `1`, `1` and `1`: DL18's sentence, the section and its last sentence as order 209 leaves
    them;
20. `['$check1', '$check2', '$check3', '$check5'] [['PND-010'], ['PND-010'], ['PND-010'], ['PND-010']]`:
    the four steps of 048a-5b's case that decision 9 changes, the third as order 209 left it;
21. `$check1 ['failed'] ['PND-010'] [] group.sign-in.v1 24`,
    `$check4 input_required ['PND-010'] [] [{'invocation': '$check4', 'outcome': 'unknown'}]` and
    `True ['account-hook.v1'] ['sign_in_hook']`: the two steps of order 209's first case that
    decision 10 changes, the step after the first, that case's length, whose last step is the
    second, and that case's item, whose sign-in hook is plain; then
    `group.sign-in.v1 $check1 ['PND-010'] [] group.inspect.v1 16`: the step of order 209's third
    case that decision 10 changes, the request before it, the inspection after it and that case's
    length, whose last step is the inspection;
22. `True False ['account_hook', 'sign_in_hook', 'sign_out_hook']` and
    `first-start register-project`: order 207's plain manifest and the two steps decision 8 copies;
23. `{'signed-server': None} {'signed-server': None} {'signed-server': None}`: the items of
    048a-2's and 048a-3's fixtures and of order 209's third case are not marked `plain`, so their
    calls carry Conduit's keys and need no discovery;
24. `0`, `0`, `0` and `0`: none of this order's work.

Then name in your report the part function, `dispatch_hook`, `part_answered`, the end and the URL
arm, with their files and lines as you find them. Still before any change, keep the rows this order
changes, and from `engine/`, one at a time, run these and keep each last line: "Done when" compares
with them.

```bash
mkdir -p engine/target && grep -E '^\| (PND-010|IFC-006|IFC-007|IFC-010|IFC-049) \|' engine/STANDING.md > engine/target/standing-048a5d-before.txt
```

```bash
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- lifecycle
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
```

Each passes. `requests-` runs order 209's three cases among the others, and each must pass and
none be an expected failure; where one fails, stop and report it, since on 209's round-1 branch a
failing case of 209's is 209's to fix, not this order's.

Orders that land beside this one (050, 051b and others): whichever lands second keeps both changes
and writes `engine/STANDING.md` again with the tool.

**Who decided what.** PND-010, PND-011, IFC-006, IFC-007, IFC-009, IFC-010 and IFC-049, as order
207 writes them, decide that a hook part follows an account check that finds no account or another
or that ends failed or unknown, and undoes nothing; that the asked-for name reaches a keyed hook and
no plain one, a request asking a plain hook for one being refused before any part; that a URL-mode
request from the hook part is taken by `sign-in.v1`, plain or keyed alike, not as IFC-016's lapse
trigger; the form of the answer the engine takes; that an answer failing a check is not relayed and
ends `sign-in.v1` failed with a diagnostic naming IFC-049, the check and the field; and that no
text of the hook is relayed. The coordinator's plan for the chain decides that the core keeps the
implementation's requestState with the part. Everything else here is the coordinator's, each open
to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-5d into this
order and 048a-5d2; the two refreshes' changes listed above; moving the interim ending of orders 048a-5b
and 209 from the account check to a URL elicitation the hook gives, with one cause in place of
their three, under the new decision name `end-sign-in-unrelayed`; the part function's new
parameter; ending `sign-in.v1` failed under `PND-003`, dispatching no hook, where a `retire.v1` of
the instance is in progress (decision 1); the six checks of the answer's form, their names, fields and order, and requiring
`inputRequests` to be an object with one member; taking every `input_required` answer of the hook
part into decision 4, so that one that is not URL-mode fails a check of IFC-049's form rather than
ending unknown under IFC-009 or IFC-010's table; reading a hook that succeeds without asking for
input as failing the first check; ending the hook part `unknown`, under IFC-010, once its answer is
read, and its cause; ending a hook part whose `sign-in.v1` has already ended unknown when it asks
for input, with nothing relayed and nothing raised; ending `sign-in.v1` with the hook's outcome
under PND-010 where the hook ended failed or unknown, failed for a reason, and the end's new
parameter and name; keeping the requestState and the input request's key in the part's member
`hook_input`, removed whenever the part ends; the part's name `sign-in`, the phase `sign-in-pending`
and the member `sign_in_login`; the diagnostics' words; the decisions' names; the unit tests; the
two cases, their routes, IDs and times; the steps changed and inserted in the cases of orders
048a-5b and 209; the comment on `retry_intent`; and the text in `engine/CONFORMANCE.md`. Apply them
as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-010 and PND-011 (`spec/promises/pending.yaml`); IFC-006, IFC-007, IFC-009,
   IFC-010, IFC-016, IFC-045 and IFC-049 (`spec/interface/implementation.yaml`); OPS-006 and
   OPS-007 (`spec/conventions/operations.yaml`); PKG-020
   (`spec/promises/packages.yaml`); CTR-013 (`spec/promises/contracts.yaml`); `sign-in.v1` and
   `sign-in-hook.v1` (`spec/core/operations.yaml`).
2. The orders 048a-5b, 048a-5c, 048b-relay2 and 209 (its file
   `209-build-keyed-hooks-and-sign-in-fixes.md`), whole, with their Records; order 207 (its file
   `207-keyed-hooks-and-sign-in-spec-fixes.md`), its decisions 4 to 11 and 13 and its Record; order
   048a-5 (its file `048a5-build-sign-in-refusals-before-parts.md`) and order 048b-relay1 (its file
   `048b-relay1-build-input-required-relay.md`), their Records; order 048a-4's decisions 2, 3 and
   8, and order 048a-2's decisions 5 and 10.
3. The cases `requests-sign-in-lapse-and-conflict`, `requests-sign-in-account-first`,
   `requests-sign-in-login-plain-hook` and `requests-keyed-hook-dispatch`.
4. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.35 to 0.45k changed lines of Rust with four unit tests; two new cases, of 25
steps on 048a-2's fixture and of 8 steps on order 207's plain manifest; four steps changed and four
inserted in 048a-5b's case, two changed and two inserted in order 209's first case and one changed
and one inserted in its third; and four places in `engine/CONFORMANCE.md`.

## The decisions it applies

Below, `Q(x)` is `c.qualified(x)`, the core's qualified reference to the operation `x`.

1. **The hook part, after a check that counts as no account.** In `part_answered`, for `NoAccount`
   or `Other`, and for `NotChecked` where the part's outcome is `failed` or `unknown` (order 209's
   branch), in place of the decision `end-sign-in-before-relay` and the end that follow each of
   them: the clear above stays as it is, for `Expected` and `NoAccount` alone (a failed or unknown
   check clears nothing and raises nothing, order 209), and so does the raise for `Other`; then:
   - Where a `retire.v1` of the instance is in progress (the test `retiring` already makes), end
     the invocation failed under `PND-003` with the cause
     `A retirement of the instance is in progress, so sign-in.v1 starts no login and signs nothing in.`,
     dispatch nothing, and return.
   - Where the instance's `resolved_implementations` has no member `Q("sign-in-hook.v1")`, which
     CTR-013's checks (order 048a-1) leave no way to reach, end the invocation failed under
     `PND-010` with the cause
     `No implementation of sign-in-hook.v1 is resolved for the instance, so no login can be started and sign-in.v1 signs nothing in.`,
     and return; report whether any path reaches it.
   - Call the part function for `sign-in-hook.v1`, with the `sign-in.v1` invocation as its parent,
     the instance as its object, the rules `PND-010`, `IFC-006`, `ACC-018` and `IMP-002`, and the
     login of decision 2: the `sign-in.v1` invocation's `asked_login` where that is a string, and
     otherwise none.
   - Set `sign_in.phase` to `sign-in-pending`.

   The account found is not undone (PND-010: "that initial mismatch or failure does not undo the
   account or end sign-in.v1"). The reply is the part function's,
   `{"pending":true,"invocation":<the sign-in.v1 ID>,"hook":<the part's ID>}`. A `NotChecked` check
   whose outcome is any other, a reason such as `unavailable` under IFC-037, keeps the `PND-011`
   ending as 048a-5b wrote it. Remove the decision `end-sign-in-before-relay` and the three interim
   causes, 048a-5b's two and order 209's, from `sign_in.rs`; decision 4 moves the interim ending.

2. **The part function carries the asked-for login (IFC-006).** `setup_sign_in_hook` and
   `dispatch_hook` each gain a last parameter, `login: Option<&str>`, and each gains
   `#[allow(clippy::too_many_arguments)]`, as requests.rs marks its other functions of eight
   parameters. `dispatch_hook`, where it is
   `Some`, sets the new invocation's member `sign_in_login` to that string right after building its
   literal, before it pushes the invocation and before it writes any intent or record, so that
   every call of the part, the one order 209 defers for a keyed hook of a plain item until
   discovery lists the extension among them, is written from an invocation that holds it. `hook`
   passes `None` to `dispatch_hook`; `begin` and `dispatch_sign_in_part` pass `None` to the part
   function; decision 1 passes the login. The part function names its decision by the operation:
   `dispatch-account-check` for `account-hook.v1`, `dispatch-sign-in-hook` for `sign-in-hook.v1`,
   with the facts `{"hook": ID}`, and `dispatch-sign-out` otherwise, as now.

   In `dispatch_intent`, inside the block `if v["plain"] != true {`, after the closing brace of the
   `if !v["key"].is_null()` block and before the line that sets the call's `params._meta` to
   `meta`, add: where `v["sign_in_login"]` is a string,
   `meta[mark(c, "signInLogin")] = json!({"name": v["sign_in_login"]})`. That is the only member of
   the value (IFC-006: "{name}"). Since order 209 reads `plain` per call, that block serves a call to
   an implementation not marked `plain` and a call for a hook that a plain item lists under `keyed`.
   A plain hook is never asked: PND-010 refuses before any part a `signInLogin` aimed at one (order
   209), so its `sign-in.v1` has no `asked_login`; should a name reach a plain call all the same,
   that call holds no Conduit key and so no name (IFC-007). The part's `tools/call` intent is the
   call that `retry_intent` copies for the part's retry (048a-7): for a keyed hook of a plain item,
   the one pushed once discovery lists the extension, never the `server/discover` intent. Read
   `asked_login` in decision 1 alone, and give no other invocation the member `sign_in_login`.
   Since this order does not call `retry_intent`, its comment becomes
   `#[allow(dead_code)] // Order 048a-7 calls it.`; nothing else of it changes.

3. **The hook's answer reaches `sign_in.rs` (IFC-010).** In `sign_in.rs` add
   `pub(crate) fn sign_in_hook_part(state: &State, part: &Value, c: &Context) -> bool`: true exactly
   where the part's `operation` is `Q("sign-in-hook.v1")` and its `setup_parent` names an invocation
   in `state.invocations` whose `operation` is `Q("sign-in.v1")`, whatever that invocation's state.
   In `complete`, compute it once for `v` beside `let plain`, and add, in the `match completion`, as
   the arm right before the URL arm,
   `Completion::Result { result } if sign_in_hook && result["resultType"] == "input_required" => { return crate::sign_in::hook_input(t, &mut a, c, i, &result); }`.
   It reads neither `plain` nor the item, so a plain hook's answer and a keyed one's take it alike,
   as IFC-010's sentence that order 207 adds has the engine follow it "for a plain call and for a
   call that carries Conduit's keys", and IFC-009 defers to it. Every other result, and any result
   of another invocation, takes the path it takes today: the URL arm still ends a URL elicitation
   outside `sign-in.v1` under IFC-016, and an account check's within it as order 048a-5b leaves it.
   In `finish`, right after the line that removes `previous`, add
   `v.as_object_mut().unwrap().remove("hook_input");`, so that a part keeps decision 4's member
   only while it is in progress.

4. **What `hook_input` does.** In `sign_in.rs`, add
   `pub(crate) fn hook_url_action(result: &Value) -> std::result::Result<(String, Option<String>), (&'static str, &'static str)>`,
   which gives the key of the hook's one input request and its requestState, or the first check
   the answer fails, as `(check, field)`, deciding in this order:

   | Check | Field | Passes where |
   | --- | --- | --- |
   | `asks for input` | `resultType` | `result["resultType"]` is `"input_required"` |
   | `exactly one input request` | `inputRequests` | `result["inputRequests"]` is a JSON object with exactly one member |
   | `an elicitation` | `method` | that member's `method` is `"elicitation/create"` |
   | `URL mode` | `params.mode` | its `params.mode` is `"url"` |
   | `a message` | `params.message` | its `params.message` is a string |
   | `a URL` | `params.url` | its `params.url` is a string |

   On passing, it gives the member's key and `result["requestState"]` where that is a string, byte
   for byte, or `None`. Other members of `params`, such as `elicitationId`, are not read. Add
   `pub(crate) fn hook_answer_cause(check: &str, field: &str) -> String`, giving
   `The sign-in hook's answer is not one URL elicitation, so nothing was relayed and the instance is not signed in: it fails the check "{check}" at the field {field}.`,
   and `pub(crate) fn keep_hook_input(part: &mut Value, key: &str, state: Option<&str>)`, which
   sets the part's member `hook_input` to `{"key": K, "request_state": S}`, leaving out
   `request_state` where there is none, and is the only code that writes the member. Then add
   `pub(crate) fn hook_input(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize, result: &Value) -> Result<()>`,
   `i` being the part's index in `t.state.invocations`, `P` its parent, the `sign-in.v1`
   invocation, `S` the parent's ID and `H` the part's. "The part's ending" below is: end the part
   `unknown` through `crate::requests::finish`, with outputs `{}`, content `[]` and the cause one
   diagnostic from `explain::outcome_diagnostic` for the part, the outcome `unknown`, the rule
   `IFC-010` and the cause given. In this order it:
   1. where `P`'s `state` is not `in-progress` or its `sign_in.phase` is not `sign-in-pending` (its
      `sign-in.v1` was cancelled or recovered while the hook ran): takes the part's ending with the
      cause
      `The sign-in hook asked for input after its sign-in.v1 had ended, so nothing was relayed and no sign-in action was raised.`,
      pushes one decision named `drop-sign-in-hook-answer`, citing `IFC-010` and `OPS-027`, with
      the facts `{"invocation": S, "hook": H}`, and returns, changing nothing of `P`;
   2. calls `hook_url_action`;
   3. where no object has `P`'s `object` as its ID, or that object's `state` is `retired`: takes
      the part's ending with step 5's cause, without calling `keep_hook_input`, pushes the part as
      in step 6, ends `P` failed under `PND-003` with the cause
      `The instance was retired while its sign-in hook was in progress, so sign-in.v1 relays nothing and signs nothing in.`,
      and returns;
   4. where the answer failed a check `(C, F)`: takes the part's ending with step 5's cause,
      without calling `keep_hook_input`, pushes the part as in step 6, pushes one decision named
      `refuse-sign-in-hook-answer`, citing `IFC-049` and `IFC-010`, with the facts
      `{"invocation": S, "hook": H, "check": C, "field": F}`, ends `P` failed under `IFC-049` with
      the cause `hook_answer_cause(C, F)`, and returns;
   5. where it passed with `(K, R)`: calls `keep_hook_input` on the part with `K` and `R`; then, as
      this order's interim ending, takes the part's ending with the cause
      `The hook asked for input that sign-in.v1 did not relay, so whether the login it began is still waiting is not known.`
      (`finish` removes `hook_input`, decision 3);
   6. pushes `{"name": "sign-in", "outcome": "unknown", "invocation": H}` onto `P`'s
      `sign_in.parts`;
   7. pushes the decision `end-sign-in-unrelayed`, citing `PND-010`, `IFC-010` and `IFC-049`, with
      the facts `{"invocation": S, "hook": H}`, and ends `P` failed under `PND-010` with the cause
      `HOOK_UNRELAYED_CAUSE`, a constant of `sign_in.rs` that the code and the unit test both use,
      holding
      `The sign-in hook asked for a URL action, and its relay to the Interface is not built yet (order 048a-5e), so nothing was relayed and the instance is not signed in.`

   Steps 5 to 7, from the part's ending, are the interim ending, which order 048a-5ef replaces whole
   by its relay, keeping the part in progress and with it `hook_input`; until 048a-5d2, the URL is
   not matched either. The decision `end-sign-in-before-relay` is no longer written anywhere
   (decision 1); `end-sign-in-unrelayed` is written in that one place alone, and 048a-5ef removes
   it. No step raises a sign-in action or begins a hold. Nothing of the answer, its URL, its message,
   its requestState, its key or any other member, reaches a reply, a record, a decision's facts, an
   intent or a diagnostic (IFC-049: "No text from the hook is relayed"; "holding no URL value"); the
   key and the requestState are kept only in the part's `hook_input` while it is in progress, in the
   core's state.

5. **The hook part's other endings.** In `part_answered`, before its test of `account-pending`:
   where `sign_in.phase` is `sign-in-pending`, for the part that `complete`'s ordinary path ended
   (`finish`, then `child_ended`) or that its IFC-026 arm, or order 209's discovery arm, answered
   with a reason: push `{"name": "sign-in", "outcome": O, "invocation": H}` onto `sign_in.parts`;
   where no object has the parent's `object` as its ID, or it is retired, end `sign-in.v1` failed
   under `PND-003` with decision 4's step 3 cause, the part being already ended and pushed; otherwise, by the part's outcome:
   - **succeeded:** the hook answered without asking for input. Push `refuse-sign-in-hook-answer`
     as decision 4's step 4 does, with the check `asks for input` and the field `resultType`, and
     end `sign-in.v1` failed under `IFC-049` with `hook_answer_cause("asks for input", "resultType")`.
   - **failed:** end `sign-in.v1` failed under `PND-010` with the cause
     `The sign-in hook ended failed, so no login was started and sign-in.v1 signs nothing in.`
   - **unknown:** end `sign-in.v1` unknown under `PND-010` with the cause
     `The sign-in hook ended unknown, so whether it began a login is not known, and sign-in.v1 signs nothing in.`
   - **any other outcome**, a reason (`refused` or `unavailable` from the IFC-026 arm, or
     `unavailable` under IFC-037 where order 209's discovery for a keyed hook of a plain item does
     not list the extension): end `sign-in.v1` failed under `PND-010` with the cause
     `The sign-in hook reported {outcome}, so no login was started and sign-in.v1 signs nothing in.`,
     `{outcome}` being the reason. Report which paths reach it.

   The hook's own diagnostics and text are relayed nowhere. For the `unknown` arm, the end gains
   one parameter, the outcome, `failed` or `unknown`, between `i` and the rule, and is renamed
   `end_sign_in`; every call of `end_failed` passes `"failed"`, so that it ends as before. A cancel
   or a restart that ends the `sign-in.v1` while its hook is in progress leaves the part as
   048a-5b leaves its account check: `requests::cancel` leaves it in progress, so a later answer
   reaches decision 4's step 1 where it asks for input and otherwise `child_answered`, which returns
   at once; `requests::recover` ends it unknown with its parent. Report what you find, and change
   nothing else for it.

6. **Four unit tests pin the rules.** Write the values as `json!` values. Let URLACTION be
   `{"resultType": "input_required", "inputRequests": {"sign-in": {"method": "elicitation/create", "params": {"mode": "url", "message": "Open the device page.", "url": "https://example.org/device"}}}, "requestState": "opaque-hook-state-0001"}`.
   - In `sign_in.rs`'s tests, `sign_in_hook_answer_form`, asserting with `assert_eq!`:
     `hook_url_action` of URLACTION gives `Ok(("sign-in".into(), Some("opaque-hook-state-0001".into())))`;
     with `requestState` `" opaque/state+=\u{e9} "`, that string exactly, its spaces kept (the core's
     half of order 048b-relay2's R1: the core keeps the implementation's value byte for byte); without
     `requestState`, and with `requestState` `7`, `Ok(("sign-in".into(), None))`; with
     `elicitationId` `""` added to `params`, the first answer again; and each of these gives the
     `Err` named: `resultType` `"complete"`, and no `resultType`,
     `("asks for input", "resultType")`; `inputRequests` a list holding the one entry, and an
     object of two members (the entry under `sign-in` and under `again`),
     `("exactly one input request", "inputRequests")`; `method` `"sampling/createMessage"`,
     `("an elicitation", "method")`; `mode` `"form"`, and `params` `"url"`,
     `("URL mode", "params.mode")`; no `message`, `("a message", "params.message")`; `url` `7`,
     `("a URL", "params.url")`. Last, `hook_answer_cause("URL mode", "params.mode")` gives decision
     4's text whole, and `keep_hook_input` on `{"id": "h1"}` with `"sign-in"` and
     `Some("opaque-hook-state-0001")` gives
     `{"id": "h1", "hook_input": {"key": "sign-in", "request_state": "opaque-hook-state-0001"}}`,
     and with `None`, `{"id": "h1", "hook_input": {"key": "sign-in"}}`.
   - In `sign_in.rs`'s tests, `sign_in_hook_input_kept_nowhere`: build a `Transition` whose state
     holds the objects of 048a-5b's test `sign_in_success_answers_and_lifts_the_hold` (the Agent
     `s1` with the account `primary-account`, not retired, with no sign-in action and no hold), an
     in-progress `sign-in.v1` invocation `g1` whose `object` is `s1` and whose `sign_in` is
     `{"phase": "sign-in-pending", "parts": [{"name": "account", "outcome": "succeeded", "invocation": "k1"}]}`,
     and its in-progress `sign-in-hook.v1` part `h1`, attempt 1, `setup_parent` `g1`, `plain`
     `true`; give each the members `call_fixture` (`requests.rs`'s tests) gives its invocation, and
     any more that `finish` and the end read, and say which. Call `complete` for `h1`, attempt 1,
     with `Completion::Result { result: URLACTION }`, and assert: `h1`'s `state` is `unknown`, it
     has no member `hook_input`, and its `cause` is one diagnostic whose `ruleId` is `IFC-010`;
     `g1`'s `state` is `failed`, its `sign_in.phase` `ended` and its last part
     `{"name": "sign-in", "outcome": "unknown", "invocation": "h1"}`; `t.reply`'s diagnostics are
     one, whose `ruleId` is `PND-010` and whose text ends with `HOOK_UNRELAYED_CAUSE`;
     `t.state.pending_actions` and `t.state.sign_in_holds` are empty and no decision is named
     `raise-sign-in-action` (not IFC-016's trigger); and the string `serde_json::to_string` gives
     for `json!([t.reply, t.records, t.request_replies, t.external_intents, t.state.records])`, and
     the facts of every decision in `t.decisions`, hold none of `opaque-hook-state-0001`,
     `example.org/device` and `Open the device page.`. On a fresh copy of the same `Transition`
     with `h1`'s `plain` `false`, the same call gives the same `g1` and the same absences. On a
     fresh copy, URLACTION with `mode` `form` gives `g1` failed, one diagnostic whose `ruleId` is
     `IFC-049` and whose text ends with `hook_answer_cause("URL mode", "params.mode")`, and the same
     absences. On a fresh copy with `g1`'s `state` `unknown` and its `ended` set, as a cancel leaves
     it, the call gives `h1` `unknown` with one diagnostic whose `ruleId` is `IFC-010`, `g1`
     unchanged, the decision `drop-sign-in-hook-answer`, no external intent, empty
     `pending_actions` and `sign_in_holds`, no decision `raise-sign-in-action`, and the same
     absences.
   - In `sign_in.rs`'s tests, `sign_in_hook_not_dispatched_while_retiring`: use
     `sign_in_hook_input_kept_nowhere`'s state, without its hook part `h1`, but with `g1`'s
     `sign_in` set to `{"phase": "account-pending", "parts": []}`, an in-progress
     `account-hook.v1` part `k1` (`setup_parent` `g1`), and an in-progress `retire.v1` invocation
     `r1` whose `object` is `s1`. Complete `k1` with ACCOUNT(`null`) (decision 7), as a keyed
     result, and assert that `g1` is `failed` with one diagnostic whose `ruleId` is `PND-003`, that
     no external intent was pushed, and that no invocation of `Q("sign-in-hook.v1")` exists.
   - In `requests.rs`'s tests, `sign_in_hook_call_carries_asked_login`: `dispatch_intent(&v, false, &c)`
     of an invocation of `Q("sign-in-hook.v1")`, built as `call_fixture` builds its invocation, with
     `implementation` `{"item": {"command": ["x"]}}`, `plain` `false` and `sign_in_login`
     `"device"`, gives `call.params._meta` holding, under `mark(&c, "signInLogin")`, written so in
     the test, exactly
     `{"name": "device"}`; without `sign_in_login`, no such key; with `implementation`
     `{"item": {"plain": true, "tools": {Q("sign-in-hook.v1"): "sign_in_hook"}, "keyed": [Q("sign-in-hook.v1")]}}`,
     `plain` `false` (order 209's keyed hook of a plain item) and `sign_in_login` `"device"`,
     `call.params.name` `"sign_in_hook"` and the same `{"name": "device"}`; and with that item
     without `keyed`, `plain` `true` and `sign_in_login` `"device"`, `call.params.name`
     `"sign_in_hook"` and no `_meta`.

7. **One portable case, on 048a-2's fixture.** Add the directory
   `conformance/conventions/requests/sign-in-hook-part/` with one file, `case.yaml`: ID
   `requests-sign-in-hook-part`, `layers: [convention, interface]`,
   `promises: [PND-010, IFC-006, IFC-010, IFC-049]`, title
   `Dispatch the sign-in hook after an account check that finds no account or another or that ends unknown, pass a keyed hook the asked-for login, and end sign-in failed on an answer that is not one URL elicitation`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-lapse-and-conflict/fixture.json", "manifest": "../sign-in-lapse-and-conflict/manifest.json", "documents": "../sign-in-lapse-and-conflict/documents.json"}`,
   048a-2's case's three files, unchanged: its Type `signed-agent.v1` states the logins `browser`
   and `device`, and its `signed-server`, not marked `plain`, implements `sign-in-hook.v1` and
   carries Conduit's keys, needing no discovery (order 209). Write it as JSON with two-space
   indentation and a final newline, its members in the order of
   `requests-sign-in-lapse-and-conflict`. It must meet 046a's portable-case rule as it stands.

   Below, ROOT, `META/x`, "Root asks T with A", "a completion" and REFUSED(R, RULE) are as 048a-2's
   decision 10 defines them; ACCOUNT(X), PART(H) and `K(x)` as 048a-4's decision 8 does; and "Root
   asks T with A and M", S, URL and LOGIN(X) as 048a-5's decision 5 does. A new step's evidence is
   its time, `2026-11-12T12:00:00Z`, and `ids` the 100 strings `00000000-0000-4000-8000-` followed
   by the twelve-digit, zero-padded decimal of 150000 + 100n + k, for k = 0 to 99, `revisions` the
   16 strings `hook-` followed by the decimal of n, a `-` and the decimal of j, and `suffixes` the
   16 strings `h` followed by the four-digit, zero-padded decimal of 100n + j, for j = 0 to 15,
   where n is the step's number in this case; a copied step keeps its own evidence. These recur:
   - URLACTION: `{"kind": "result", "result": R}`, `R` decision 6's URLACTION;
   - FORM: URLACTION with `params` `{"mode": "form", "message": "Enter the code.", "requestedSchema": {"type": "object", "properties": {}}}` and `requestState` `opaque-hook-state-0002`;
   - HOOK-FAILED: `{"kind": "result", "result": {"isError": true, "content": [{"type": "text", "text": "The device login is not available."}], "structuredContent": {"diagnostics": [{"ruleId": "PND-010", "level": "error", "message": {"text": "The device login is not available."}}]}, "_meta": {"META/outcome": "failed"}}}`;
   - HOOK-DONE: `{"kind": "result", "result": {"_meta": {"META/outcome": "succeeded"}, "content": [], "structuredContent": {}}}`;
   - HOOK-UNKNOWN: `{"kind": "result", "result": {"isError": true, "_meta": {"META/outcome": "unknown"}, "structuredContent": {}}}`;
   - HOOK(M): `"external_intents": [{"kind": "dispatch", "attempt": 1, "call": {"params": {"_meta": M}}}]`,
     `M` being `{"META/operation": Q, "META/caller": ROOT, "io.modelcontextprotocol/clientCapabilities": {"elicitation": {"url": {}}}}`
     with any members given, `Q` the core's qualified reference to `sign-in-hook.v1`, written as
     PART(H) writes its `Q`;
   - ENDED(O, RULE, I): `{"reply": {"isError": true, "_meta": {"META/outcome": O, "META/invocation": I}, "structuredContent": {"diagnostics": [{"ruleId": RULE, "level": "error"}]}}, "external_intents": []}`.

   Its steps:
   1. and 2. `requests-sign-in-lapse-and-conflict`'s steps 1 and 2, `input` and `expect`, whole,
      each with its evidence: first start and registration.
   3. That case's step 3, `input`, `expect` and `capture`, whole, with its evidence: the setup of
      `signed`, `$setup1`.
   4. to 6. That case's steps 7, 8 and 9, `input`, `expect` and `capture`, whole, each with its
      evidence: the setup's two run reports and its account check, which finds the expected
      account; the setup succeeds.
   7. Root asks `group.sign-in.v1` with S and URL. `expect` `{"reply": {"pending": true}}` with
      PART(`account-hook.v1`); `capture`
      `{"signin1": "/reply/invocation", "check1": "/reply/hook"}`, as 048a-5b's SIGN(N) does.
   8. A completion for `$check1`, ACCOUNT(`null`). `expect` `{"reply": {"pending": true}}` with
      HOOK(M), and `"absent": [K(conduit), K(idempotencyKey), K(signInLogin)]`; `capture`
      `{"hook1": "/reply/hook"}`: no account, so the hook is dispatched, and no login was asked for.
   9. Step 7's request. `expect` REFUSED(`conflict`, `PND-010`) with `"external_intents": []`: the
      sign-in waits on its hook.
   10. A completion for `$hook1`, URLACTION. `expect` ENDED(`"failed"`, `"PND-010"`,
       `"$signin1"`): the URL is not relayed in this build.
   11. Root asks `group.sign-in.v1` with S and URL and LOGIN(`{"name": "device"}`). `expect` step
       7's; `capture` `{"signin2": "/reply/invocation", "check2": "/reply/hook"}`: no conflict,
       since the earlier sign-in ended.
   12. A completion for `$check2`, ACCOUNT(`"other-account"`). `expect`
       `{"reply": {"pending": true}}` with HOOK(M) whose `M` also holds
       `"META/signInLogin": {"name": "device"}`, and `"absent": [K(conduit), K(idempotencyKey)]`;
       `capture` `{"hook2": "/reply/hook"}`: another account does not end the sign-in, and the
       asked-for name reaches the keyed hook (IFC-006).
   13. A completion for `$hook2`, HOOK-FAILED. `expect` ENDED(`"failed"`, `"PND-010"`,
       `"$signin2"`): the hook could not start the login, and nothing of its text is relayed.
   14. Step 7's request. `expect` step 7's; `capture`
       `{"signin3": "/reply/invocation", "check3": "/reply/hook"}`.
   15. A completion for `$check3`, ACCOUNT(`null`). `expect` step 8's; `capture`
       `{"hook3": "/reply/hook"}`.
   16. A completion for `$hook3`, FORM. `expect` ENDED(`"failed"`, `"IFC-049"`, `"$signin3"`): a
       form elicitation is not relayed.
   17. to 19. Steps 14 to 16, each capture's names ending in `4` in place of `3`, step 19's
       completion for `$hook4` being HOOK-DONE. Step 19 `expect`s ENDED(`"failed"`, `"IFC-049"`,
       `"$signin4"`): a hook that asks for nothing gives nothing to relay.
   20. to 22. Steps 14 to 16, each capture's names ending in `5`, step 22's completion for
       `$hook5` being HOOK-UNKNOWN. Step 22 `expect`s ENDED(`"unknown"`, `"PND-010"`,
       `"$signin5"`): a hook that may have begun a login leaves the sign-in unknown.
   23. to 25. Steps 14 to 16, each capture's names ending in `6`, step 24's completion for
       `$check6` being HOOK-UNKNOWN in place of ACCOUNT(`null`) and step 25's completion for
       `$hook6` URLACTION. Step 24 `expect`s step 8's: an account check that ends unknown counts as
       no account and goes on to the hook (PND-010, order 209). Step 25 `expect`s
       ENDED(`"failed"`, `"PND-010"`, `"$signin6"`).

8. **One portable case of a plain hook, on order 207's plain manifest.** Add the directory
   `conformance/conventions/requests/sign-in-plain-hook-part/` with one file, `case.yaml`: ID
   `requests-sign-in-plain-hook-part`, `layers: [convention, interface]`,
   `promises: [PND-010, IFC-007, IFC-010, IFC-049]`, title
   `Take a plain sign-in hook's URL request into sign-in rather than as a lapse, after an account check that finds no account or fails, and end sign-in failed on a plain hook that asks for nothing`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-lapse-and-conflict/fixture.json", "manifest": "../sign-in-login-plain-hook/manifest.json", "documents": "../sign-in-lapse-and-conflict/documents.json"}`:
   order 207's plain item, whose calls carry none of Conduit's keys and whose results are read by
   IFC-010's table, so that a plain hook asks the 0.2.0 Agent packages' way. Write it as decision 7
   writes its case. Definitions as decision 7's; a new step's `ids` are the 100 strings
   `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
   170000 + 100n + k, `revisions` the 16 strings `plain-hook-part-` followed by the decimal of n, a
   `-` and the decimal of j, and `suffixes` the 16 strings `y` followed by the four-digit,
   zero-padded decimal of 100n + j. These recur:
   - PLAIN-SIGN(N): Root asks `group.sign-in.v1` with S and URL. `expect`
     `{"reply": {"pending": true}, "external_intents": [{"kind": "dispatch", "attempt": 1, "call": {"method": "tools/call", "params": {"name": "account_hook"}}}], "absent": [K(operation)]}`;
     `capture` `{"signinN": "/reply/invocation", "checkN": "/reply/hook"}`;
   - PLAIN-HOOK: `{"reply": {"pending": true}, "external_intents": [{"kind": "dispatch", "attempt": 1, "call": {"method": "tools/call", "params": {"name": "sign_in_hook", "_meta": {"io.modelcontextprotocol/clientCapabilities": {"elicitation": {"url": {}}}}}}}], "absent": [K(operation), K(signInLogin)]}`.

   Its steps:
   1. and 2. `requests-sign-in-login-plain-hook`'s steps 1 and 2, `input` and `expect`, whole,
      each with its evidence: first start and registration.
   3. PLAIN-SIGN(1).
   4. A completion for `$check1`, `{"kind": "result", "result": {"content": [], "structuredContent": {"account": null}}}`.
      `expect` PLAIN-HOOK; `capture` `{"hook1": "/reply/hook"}`: a plain check that finds no
      account goes on to the plain hook, named from `tools`, with no Conduit key.
   5. A completion for `$hook1`, URLACTION. `expect` ENDED(`"failed"`, `"PND-010"`, `"$signin1"`):
      the plain hook's URL request is taken by `sign-in.v1` (IFC-010), so the sign-in meets the
      interim ending, not IFC-016's lapse trigger, which would end it unknown.
   6. PLAIN-SIGN(2).
   7. A completion for `$check2`, `{"kind": "result", "result": {"isError": true, "content": [{"type": "text", "text": "The stored login could not be refreshed."}]}}`.
      `expect` PLAIN-HOOK; `capture` `{"hook2": "/reply/hook"}`: the check fails under IFC-010's
      table, counts as no account and goes on to the hook (PND-010, order 209).
   8. A completion for `$hook2`, `{"kind": "result", "result": {"content": []}}`. `expect`
      ENDED(`"failed"`, `"IFC-049"`, `"$signin2"`): a plain hook that succeeds without asking for
      input gives nothing to relay.

   The routes of decisions 7 and 8 are new and have not been run, since orders 207 and 209 are not
   yet built. Before running either, check that no ID repeats across its steps' `ids`, the copied
   steps' included. Before running a changed or new case, check that no capture name is bound
   twice. If 046a's runner check refuses a case, if a step copied from another case
   gives a reply other than that case states, or if a step's reply differs from what it states,
   stop and report the error: do not change the expectations to pass it, and do not change the
   inputs beyond what these decisions say. No expectation is generated from the engine's output.
   Each coverage index entry holds `id`, `case` (`conventions/requests/sign-in-hook-part/case.yaml`
   and `conventions/requests/sign-in-plain-hook-part/case.yaml`), `promises` and, where its
   neighbours in that piece hold it, `safety_rows: []`, in the members and order its neighbours
   use. Write both in a `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves them
   into `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte
   for byte as it was.

9. **048a-5b's case gains the hook.** In
   `conformance/conventions/requests/sign-in-account-first/case.yaml`, its steps 18, 22, 24 and 30
   as orders 048a-5b and 209 leave them (indexes 17, 21, 23 and 29), the completions of `$check1`,
   `$check2`, `$check3` and `$check5`, with no account, another, a failed check and another, now
   dispatch the hook. Each keeps its `input` and evidence; its `expect` becomes
   `{"reply": {"pending": true}}` with HOOK(M) as decision 7 defines it, with
   `"absent": [K(conduit), K(idempotencyKey), K(signInLogin)]`, since SIGN(N) asks for no login and
   048a-3's `signed-server`, not marked `plain`, carries Conduit's keys; and it gains `capture`
   `{"hookN": "/reply/hook"}`, `N` being 1, 2, 3 and 5. A new step follows each: a completion for
   `$hookN`, decision 6's URLACTION written as decision 7's URLACTION, whose `expect` is that
   step's earlier `expect`, FAILED(N, `PND-010`), whole, now decision 4's interim ending, and whose
   evidence is: time `2026-11-12T12:00:00Z`, `ids` the 100 strings `00000000-0000-4000-8000-`
   followed by the twelve-digit, zero-padded decimal of 160000 + 100m + k, for k = 0 to 99,
   `revisions` the 16 strings `hook-step-` followed by the decimal of m, a `-` and the decimal of
   j, and `suffixes` the 16 strings `g` followed by the four-digit, zero-padded decimal of
   100m + j, for j = 0 to 15, `m` being the new step's number in the changed case: 19, 24, 27 and
   34. Every later step moves one place for each step inserted before it, unchanged; the case then
   has 36 steps. Nothing else in the case, and nothing in 048a-3's case directory, changes. Before
   running it, check again that no ID repeats across its steps. It must then pass; if a step after
   an inserted one gives another reply than the case states, stop and report it.

10. **Order 209's cases gain the hook's answer.** In
    `conformance/conventions/requests/keyed-hook-dispatch/case.yaml`, its step 13 (index 12), the
    completion of `$check1` with a keyed result that states no outcome, keeps its `input` and
    evidence; its `expect` becomes decision 8's PLAIN-HOOK, since that case's item lists only
    `account-hook.v1` under `keyed`, so its sign-in hook is plain; and it gains `capture`
    `{"hook1": "/reply/hook"}`. A new step 14 follows it: a completion for `$hook1`, decision 7's
    URLACTION, whose `expect` is step 13's earlier `expect`, whole, as order 209 wrote it
    (`sign-in.v1` ended failed under `PND-010`, no external intent and no `unanswered` key, so no
    sign-in action was raised), and whose evidence is: time `2026-11-12T12:00:00Z`, `ids` the 100
    strings `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
    180000 + 100m + k, `revisions` the 16 strings `hook-keyed-` followed by the decimal of m, a
    `-` and the decimal of j, and `suffixes` the 16 strings `z` followed by the four-digit,
    zero-padded decimal of 100m + j, `m` being 14. Every later step moves one place, unchanged.
    Without the step, the case's next `sign-in.v1` would meet 048a-2's conflict, since the first
    now waits on its hook.

    In the same case, its last step, 24 as order 209 numbers it and now 25 (index 24), the
    completion of `$check4` with a URL-mode `input_required` result, keeps its `input` and
    evidence; the check still ends unknown and counts as no account (order 209's decision 2), so
    it now goes on to the plain hook: its `expect` becomes PLAIN-HOOK with
    `"record_facts": [{"type": "invocation-ended", "data": {"invocation": "$check4", "outcome": "unknown"}}]`,
    the record order 209's step held, and it gains `capture` `{"hook2": "/reply/hook"}`. A new
    step 26 follows it: a completion for `$hook2`, decision 7's URLACTION, whose `expect` is order
    209's step 24's `reply` and `external_intents`, whole (`$sign4` ended failed under `PND-010`),
    with `"record_facts": [{"type": "invocation-ended", "data": {"invocation": "$hook2", "outcome": "unknown"}}]`
    in place of its record of `$check4`, and whose evidence is as step 14's, `m` being 26. The
    case then has 26 steps. Nothing else in the case or its directory changes.

    In `conformance/conventions/requests/sign-in-setup-check-failed-hook/case.yaml`, its step 15
    (index 14), the completion of `$check1` with a keyed result that states failed, keeps its
    `input` and evidence; its `expect` becomes `{"reply": {"pending": true}}` with HOOK(M) as
    decision 7 defines it and `"absent": [K(conduit), K(idempotencyKey), K(signInLogin)]`, since
    that case's `signed-server` is not marked `plain` and its request asks for no login; and it
    gains `capture` `{"signhook1": "/reply/hook"}`, a name the case does not use, since its step 6
    already binds `hook1` and the runner refuses a rebound symbol. A new step 16 follows it: a
    completion for `$signhook1`, decision 7's URLACTION, whose `expect` is step 15's earlier
    `expect`, whole (`$sign1` ended failed under `PND-010`, with `META/unanswered` 2), and whose
    evidence is as step 14's of the first case, with 185000 in place of 180000,
    `hook-check-failed-` in place of `hook-keyed-` and `x` in place of `z`, `m` being 16. Its last
    step, the inspection that shows `SignInUndone` still `False`, moves to 17, unchanged; the case
    then has 17 steps. Nothing else in the case or its directory changes.

    Each case must then pass; if a later step gives another reply than the case states, stop and
    report it. No other case completes an account check of `sign-in.v1` with no account, another, or
    a failed or unknown result (the start check's items 20 and 21 name the steps that do; decisions 9
    and 10 change them): 048a-5c's two complete it with the expected account. Where you find one,
    stop and report it.

11. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each place by its text read across
    line breaks, as orders 048a-5b, 048a-5c and 209 leave it.
    - In DL18's row, the sentences from "No `sign-in-hook.v1` part runs" to "This is an unbuilt
      step, not a reading of PND-010.", in the same line, become:

      ```text
      From order 048a-5d, a check that finds no account or another, or that ends failed or unknown and so counts as no account, clearing no condition (order 209), leads to a `sign-in-hook.v1` part, unless a retirement of the instance is in progress, which ends `sign-in.v1` failed (PND-003), after another account has raised the sign-in action unless one is unanswered (PND-005); a call that carries Conduit's keys holds the asked-for login's name in `signInLogin` (IFC-006), and a plain hook is never asked for one, since PND-010 refuses such a request before any part (order 209). The hook's `input_required` answer, plain or keyed alike, is taken by `sign-in.v1` and never read as IFC-016's lapse trigger, even once that `sign-in.v1` has ended (IFC-010; `requests-sign-in-plain-hook-part`). A hook answer that is not one URL elicitation ends `sign-in.v1` failed under IFC-049, naming the check and the field, and a hook that ends failed or unknown ends it so under PND-010. Until order 048a-5e builds the relay, a URL elicitation ends `sign-in.v1` failed with a diagnostic naming PND-010 and that order, unrelayed and, until order 048a-5d2, with its URL unchecked, and the hook part ends unknown, so no URL action waits and no wait is chosen. This is an unbuilt step, not a reading of PND-010. The implementation's own requestState is kept with the hook part only while the part is in progress, and is never relayed, recorded or shown.
      ```

    - In the section "Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)",
      replace "and so, until order 048a-5e, ends `sign-in.v1` failed under PND-010's interim
      ending (`requests-sign-in-account-first`, `requests-keyed-hook-dispatch`)" with "and so goes
      on to the `sign-in-hook.v1` part (order 048a-5d), whose URL elicitation, until order
      048a-5e, ends `sign-in.v1` failed under PND-010's interim ending
      (`requests-sign-in-account-first`, `requests-keyed-hook-dispatch`,
      `requests-sign-in-hook-part`)". In the section's last sentence, as order 209 words it
      ("IFC-010's relay of a URL request from the `sign-in-hook.v1` part, plain or keyed, has no
      case here: this build runs no `sign-in-hook.v1` part; orders 048a-5d and 048a-5e build the
      hook part and the relay, whose cases show it."), replace "has no case here: this build runs
      no `sign-in-hook.v1` part; orders 048a-5d and 048a-5e build the hook part and the relay,
      whose cases show it." with "has no case in this section: the hook part and the relay are
      orders 048a-5d and 048a-5e, whose cases show it.", and add after it: "Order 048a-5d takes
      that request into `sign-in.v1`, not as IFC-016's lapse trigger
      (`requests-sign-in-plain-hook-part`)." Change no other word of the section.
    - In "Consumer server (order 022)", found by its text as order 209 leaves it, replace "runs as
      far as its first account check (order 048a-5b;" with "runs as far as its `sign-in-hook.v1`
      part, whose URL elicitation it does not yet relay (orders 048a-5b, 048a-5d and 048a-5e;".
    - In "What the first build is", found by its text as order 209 leaves it, replace "finds the
      expected account, starting no login (row DL18" with "finds the expected account, and where
      that check finds no account or another or ends failed or unknown dispatches a
      `sign-in-hook.v1` part, whose URL elicitation it does not yet relay (orders 048a-5d and
      048a-5e; row DL18".

    If order 209 merged any of these places otherwise, replace what it merged in their place, and
    report each before and after, whole. Nothing else in the file changes. After the other decisions, run
    `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
    `promises` and to a promise ID in the core's sources. Report the rows of PND-010, IFC-006,
    IFC-007, IFC-010 and IFC-049 and the totals; the totals change only by what those rows change.

12. **The declaration.** `docs/work-orders/048a5d-declared.yaml` holds eleven shapes: one `added`
    for each new case, and one each of `input`, `expectation` and `flip` under `/steps/*` for
    `requests-sign-in-account-first`, for `requests-keyed-hook-dispatch` and for
    `requests-sign-in-setup-check-failed-hook`, since an inserted step moves every later step's
    index. The coordinator wrote it with this order; check that it matches
    and change it only to make it so.

13. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or
    ruling's record changes. No fixture or manifest changes. Nothing is relayed: the URL's checks
    under IFC-049 (048a-5d2), the engine's message, the input-required output and polling
    (048a-5ef), polling's endings (048a-5gh), the hook's retry and the returns (048a-7 and 048a-8,
    after the dogfood under ruling 88) are not built, and no shell code, `runner.rs`, CLI command
    (051b) or package (050) changes. Report as findings, for the coordinator: that the interim ending answers a URL elicitation without
    checking its URL, so until 048a-5d2 a hook's URL is neither checked nor relayed; that no
    promise says what `sign-in.v1` does with a hook that succeeds without asking for input, so
    decision 5 reads it as failing IFC-049's first check; that no promise says which outcome the
    hook part takes when `sign-in.v1` ends without relaying its input request, or when its
    `sign-in.v1` has already ended, so decision 4 ends it `unknown`; that a hook's `input_required`
    answer that is not URL-mode is read as failing IFC-049's form, where IFC-009 and IFC-010's
    table would read the part unknown; whether `inputRequests` given as a list reaches the core
    from a real hook through rmcp 3.9.0, which this order refuses; that an account check's URL
    elicitation within `sign-in.v1` still takes the URL arm, raising PND-005's action, where IFC-010
    as order 207 writes it ends that part unknown and has it follow PND-010 and PND-011; that the
    cases assert no diagnostic's words, since IFC-050 leaves wording to the engine, so the unit
    tests alone pin them and the absence of the hook's values; that this order keeps the hook's
    requestState but sends no retry, so `retry_intent` keeps its `#[allow(dead_code)]` until
    048a-7; that a cancel leaves the hook part in progress, as 048a-5b leaves its account check;
    that order 048b-relay2's R1 is answered only in the core's half, by `sign_in_hook_answer_form`,
    its runner half staying open for 048a-5ef or 048a-7, and that nothing observable pins the call
    of `keep_hook_input` in `hook_input`; that the interim ending and a cancel end sign-in.v1 while
    the hook's login may still hold a fixed port, so 048a-5c's refusal does not cover it until 048a-5ef keeps sign-in.v1 in progress; and any place where PND-010, IFC-006, IFC-010 or IFC-049 left you a choice this order does not
    settle.

## Scope

Change only `engine/crates/core/src/sign_in.rs` (`part_answered`, the end's parameter and name,
`sign_in_hook_part`, and decisions 1, 4, 5 and 6), `engine/crates/core/src/requests.rs` (the part
function's and `dispatch_hook`'s parameter and the decision name, decision 2; `dispatch_intent` and
the comment on `retry_intent`, decision 2; `complete` and `finish`, decision 3; its test, decision
6), `engine/crates/core/src/lifecycle.rs` (its call of the part function, decision 2, alone),
`engine/CONFORMANCE.md` (decision 11's four places), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/requests/index.yaml`, the new directories
`conformance/conventions/requests/sign-in-hook-part/` and
`conformance/conventions/requests/sign-in-plain-hook-part/`, and the three files
`conformance/conventions/requests/sign-in-account-first/case.yaml` (decision 9),
`conformance/conventions/requests/keyed-hook-dispatch/case.yaml` and
`conformance/conventions/requests/sign-in-setup-check-failed-hook/case.yaml` (decision 10). Where
048a-5b or 209 put `part_answered` or the end in another file, change it there, and only there, and say so.
`docs/work-orders/048a5d-declared.yaml` comes with this order; change it only as decision 12 says.
`conformance/manifest.yaml` holds the new entries only until the pieces tool moves them, and ends
unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, the shell, the experiments, the case
schema, `conformance/internal-cases.yaml`, `conformance/expected-failures.yaml`, any fixture or
manifest, any other case or file of a case's directory, `docs/implementation/` or
`docs/proposals/`: report any defect as a finding. `experiments/011-short-form-spike/` changes only
if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- lifecycle
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-hook-part
cargo run --release -p conduit-conformance -- .. requests-sign-in-plain-hook-part
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
cargo run --release -p conduit-conformance -- .. requests-keyed-hook-dispatch
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-check-failed-hook
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell first_start_presents_final_lists
```

The third reports four more tests passed than the start's run, 0 failed, the new ones being
`sign_in_hook_answer_form`, `sign_in_hook_input_kept_nowhere`,
`sign_in_hook_not_dispatched_while_retiring` and `sign_in_hook_call_carries_asked_login`; report both counts. The fourth passes as at the start;
report its count. The pieces tool prints `Nothing to move.` The five single-case runner commands
each end `1 cases passed`; `requests-sign-in-` gives two more cases than at the start, every one
passing, and `requests-` two more cases than at the start, with the start's expected failures, each
still failing; report each count. The shell test passes. If any other case, or any shell, CLI or
experiment test, fails because an account check now leads to a hook part, stop and report it: do
not change it. The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that each behavior is what its case checks. From `engine/`, first keep copies:

```bash
cp crates/core/src/requests.rs target/requests.rs.fixed
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
```

Then make each change below by hand, alone, run its commands, restore its file with `cp` from its
copy, and run them again. Each run before the restore must print `FAIL 0 cases passed; 1 failed`,
failing at the step named, and each run after it `1 cases passed`. Restore with `cp`, not `mv`: a
moved file keeps its older modification time, so cargo would not rebuild the crate. Git ignores
`target/`. Report the error each failing run printed.

| Change, by hand | Commands (`cargo run --release -p conduit-conformance -- ..` and) | Must fail at |
| --- | --- | --- |
| In `requests.rs`'s `complete`, delete decision 3's arm, the one that returns `crate::sign_in::hook_input(...)`, and nothing else | `requests-sign-in-hook-part`, then `requests-sign-in-plain-hook-part` | step 10, then step 5: the URL arm ends the hook unknown under IFC-016 and raises the sign-in action, so `sign-in.v1` ends unknown where failed is expected |
| In `sign_in.rs`'s `part_answered`, send a `NotChecked` check whose outcome is `failed` or `unknown` to the `PND-011` ending in place of the hook | `requests-sign-in-plain-hook-part` | step 7, an error reply where the hook's dispatch is expected |
| In `requests.rs`'s `dispatch_intent`, delete decision 2's statement that writes `signInLogin` | `requests-sign-in-hook-part` | step 12, the dispatch without `META/signInLogin` |

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a5d-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/fixtures conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-setup-account conformance/conventions/requests/sign-in-refusals-before-parts conformance/conventions/requests/sign-in-fixed-port conformance/conventions/requests/sign-in-login-plain-hook conformance/conventions/requests/keyed-hook-manifest conformance/conventions/requests/keyed-not-a-hook conformance/conventions/requests/sign-in-setup-check-failed conformance/conventions/requests/keyed-not-in-implements conformance/conventions/requests/keyed-hook-dispatch/manifest.json conformance/conventions/requests/sign-in-setup-check-failed-hook/manifest.json conformance/conventions/requests/sign-in-setup-check-failed-hook/documents.json conformance/conventions/inventory conformance/conventions/setup conformance/language engine/crates/shell tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5d-declared.yaml: 11 shapes.`,
then `Vector delta: C changes; F flips; 0 undeclared; 0 errors.` and `exit 0`; report `C` and `F`,
and that every change outside the two added cases lies in `requests-sign-in-account-first`,
`requests-keyed-hook-dispatch` or `requests-sign-in-setup-check-failed-hook`. `promise_standing.py --check`, `check_model_account.py` and
`check_roadmap.py` each end `Errors: 0`. No promise's text changes, so no row of the
model-checking account goes stale; where `check_model_account.py` reports one stale, stop and report
it. The unit tests end `OK`; report the count. The `git status` command prints nothing, or a change
under the spike's directory, reported with its reason, is the only line it may print.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so
the local gate runs it for this change, as spec batch 1's Gate G asks. From
`experiments/011-short-form-spike/`:

```bash
RUST_TEST_THREADS=4 cargo test --release
```

Its `tests/spike.rs` run passes with 0 failed; report its count.

Each of these prints 1, and each printed 0 at the start. The `tr` checks read the file with its
line breaks as spaces, so a rewrap hides nothing:

```bash
grep -rl '"dispatch-sign-in-hook"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"end-sign-in-unrelayed"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"refuse-sign-in-hook-answer"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"drop-sign-in-hook-answer"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'mark(c, "signInLogin")' engine/crates/core/src/requests.rs
grep -c 'remove("hook_input")' engine/crates/core/src/requests.rs
grep -c '#\[allow(dead_code)\] // Order 048a-7 calls it.' engine/crates/core/src/requests.rs
grep -c 'its relay to the Interface is not built yet (order 048a-5e)' engine/crates/core/src/sign_in.rs
grep -c '"id": "requests-sign-in-hook-part"' conformance/conventions/requests/index.yaml
grep -c '"id": "requests-sign-in-plain-hook-part"' conformance/conventions/requests/index.yaml
grep '^| DL18 |' engine/CONFORMANCE.md | grep -c 'From order 048a-5d, a check that finds no account or another, or that ends failed or unknown'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'never read as IFC-016.s lapse trigger, even once that .sign-in.v1. has ended'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'goes on to the .sign-in-hook.v1. part (order 048a-5d)'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'has no case in this section: the hook part and the relay are orders 048a-5d and 048a-5e, whose cases show it. Order 048a-5d takes that request into .sign-in.v1.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'runs as far as its .sign-in-hook.v1. part, whose URL elicitation it does not yet relay (orders 048a-5b, 048a-5d and 048a-5e;'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'ends failed or unknown dispatches a .sign-in-hook.v1. part, whose URL elicitation it does not yet relay (orders 048a-5d and 048a-5e; row DL18'
```

The `its relay to the Interface` count is 1 because the cause is written once, as the constant
`HOOK_UNRELAYED_CAUSE` (decision 4), which the unit test reads.

This prints 1: IFC-006's row names the core among its sources, since decision 1 cites it. It printed
0 on `c097ec81`, and may print 1 at the start where order 209's sources name IFC-006:

```bash
grep -c -E '^\| IFC-006 \| interface \| must \| [0-9]+ \| [0-9]+ \| [a-z]+ \| core(, shell)? \| case passes \|$' engine/STANDING.md
```

This prints `IFC-006:1 IFC-007:1 IFC-010:2 IFC-049:2 PND-010:2`, each row's number of cases
against the start's, and printed all zeros at the start:

```bash
python3 -c 'import re; rows = lambda p: {l.split("|")[1].strip(): int(l.split("|")[4]) for l in open(p) if re.match(r"^\| (PND-010|IFC-00[67]|IFC-010|IFC-049) \|", l)}; b = rows("engine/target/standing-048a5d-before.txt"); a = rows("engine/STANDING.md"); print(" ".join(k + ":" + str(a[k] - b[k]) for k in sorted(b)))'
```

These print `['$check1', '$hook1', '$check2', '$hook2', '$check3', '$hook3', '$check5', '$hook5'] L1`,
`['$check1', '$hook1', '$check4', '$hook2'] group.sign-in.v1 L2` and
`['$check1', '$signhook1'] group.inspect.v1 L3`, the steps decisions 9 and 10 change and insert, and
the cases' lengths, each its length at the start plus the steps inserted: `L1` four more (36 from
32), `L2` two more (26 from 24) and `L3` one more (17 from 16); and `25 8`, the new cases' lengths:

```bash
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-account-first/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (17, 18, 22, 23, 25, 26, 32, 33)], len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/keyed-hook-dispatch/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (12, 13, 24, 25)], s[14]["input"]["request"]["tool"], len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-setup-check-failed-hook/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (14, 15)], s[16]["input"]["request"]["tool"], len(s))'
python3 -c 'import json; print(*[len(json.load(open("conformance/conventions/requests/%s/case.yaml" % d))["steps"]) for d in ("sign-in-hook-part", "sign-in-plain-hook-part")])'
```

Each of these prints 0, and each printed a number other than 0 at the start (the first, 1):

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'No .sign-in-hook.v1. part runs, so no URL action waits'
grep -c 'sign-in.v1 would go on to start a login' engine/crates/core/src/sign_in.rs
grep -rl '"end-sign-in-before-relay"' engine/crates/core/src | wc -l | tr -d ' '
grep -c '^fn end_failed($' engine/crates/core/src/sign_in.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'runs as far as its first account check'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'starting no login'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'this build runs no .sign-in-hook.v1. part'
```

The first says that the interim DL18 sentence of 048a-5b and 209 is gone, the second their three
causes, the third their decision, the fourth the end's old name, the fifth and sixth the old words
of "Consumer server (order 022)" and "What the first build is", and the last the old words of the
section's last sentence (decision 11); the count of
`its relay to the Interface is not built yet (order 048a-5e)` above is the one interim cause of
decision 4. This prints 0, as at the start:

```bash
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the rows decision 11 names with the totals, the names, files and lines that the start
asks for, the function names and signatures of decisions 2 to 4, the line in `complete` where
decision 3's arm stands, the DL18 sentences and the other sentences that decision 11 replaced,
and whether any path reaches the cases decisions 1 and 5 ask about.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5d-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.

Where the coordinator dispatched this order stacked on order 209's round-1 branch, the result is
read there, with `tools/gate.py --base` set to that branch's tip, since `origin/main` lacks 209's
work; it merges only after 209 has merged, and then only once the coordinator has done the
following (the coordinator's, under that lever):

- rebase this order's branch onto `main` with `git rebase --onto origin/main` from 209's round-1
  tip, so that only this order's commits are replayed onto 209's squash, committing and rebasing
  under the work orders' identity;
- resolve any conflict where 209's verifier or fixer changed what this order changes after 209's
  round 1: `requests.rs`, `sign_in.rs` and `lifecycle.rs` under `engine/crates/core/src/`,
  `engine/CONFORMANCE.md`, the cases `requests-keyed-hook-dispatch`,
  `requests-sign-in-setup-check-failed-hook` and `requests-sign-in-account-first`, and
  `conformance/conventions/requests/index.yaml`, keeping 209's merged text and applying this
  order's decisions to it; write `engine/STANDING.md` again with
  `python3 tools/promise_standing.py --write` rather than by hand, and run the pieces tool again
  where the index conflicts;
- run this order's start check in a clean worktree of `main` after 209's merge, and where 209's
  merge changed anything that a decision reads (a step of 209's cases, a sentence of
  `engine/CONFORMANCE.md`, a cause, a name), amend this order's text and the build to it, saying
  so in the Record;
- run again the five single-case runner commands, `requests-sign-in-` and `requests-`, the
  `vector_delta.py` check with this order's declaration, and `tools/gate.py` against `origin/main`;
- have the reader read the rebase's resolutions and any amendment as new content (ruling 79,
  point 2), and write this order's Record.
