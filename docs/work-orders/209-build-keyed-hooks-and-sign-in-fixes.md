# Work order 209: keys per hook, and two sign-in fixes, in the engine

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the three cases you add, the case whose step you
change and the three cases whose expected failures you remove, with the promises each tests and
whether it passes. This order has network access only because the engine's tests bind loopback ports,
which the sandbox refuses without it; use it for nothing else, add no crate, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, case, fixture or log:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential. Do not run `claude`, `codex` or any client, and sign in nowhere.

## What and why

**What it builds.** [Ruling 86](../proposals/ruling-86/README.md), point 4, as the owner's third
answer amends it, has the engine half of option D land in 0.2.0, "so the later keyed sign-in
release is package-only and needs no newer engine". Order 207 (its file
`207-keyed-hooks-and-sign-in-spec-fixes.md`) wrote D and points 3 (b) to (d) into `spec/`, with
four cases, three of them expected failures until this order; it merged as #399 (`0b5b1eba`),
with the amendments of its round 2 and its fixer that its Record names. This order builds, as
order 207's files on `main` state it:

- **D's engine half** (point 2, as the coordinator narrowed it in order 207): registration and
  installation check an item's `keyed` under PKG-020; the hook path reads `plain` per call, so a
  call for a hook that a plain item lists under `keyed` carries Conduit's keys, is named from
  `tools`, declares Conduit's extension and is read as a keyed implementation's; and IFC-037's
  `server/discover` is read before such a call, a result without the extension leaving only that
  hook undispatched;
- **(c)**: PND-010's refusal, before any part, of a `signInLogin` naming a stated login where the
  instance's `sign-in-hook.v1` is a plain hook;
- **(d)**: an account check that ends failed or unknown counts as no account, before
  `sign-in-hook.v1` within `sign-in.v1` and, by the coordinator's reading that order 207 wrote,
  in setup.

It builds no keyed `content` copy: the carrier of ruling 86's first point 3 (a) is dropped. Point
3 (b), as order 207's round 2 wrote it, has IFC-010 govern a URL-mode `input_required` result from
a part of `sign-in.v1`, plain or keyed alike, in place of the terminal checks of IFC-009, OPS-006
and OPS-007. It asks one thing of this order: that a hook this order keys keeps that reading, so
that a keyed sign-in hook's URL request is never read as a terminal outcome (decision 2). The
`sign-in-hook.v1` part and its relay are orders 048a-5d and 048a-5e.

**The promises.** PKG-020's `then`, as order 207 wrote it: "One reached by `command` or `package`
may state `keyed`, a list of hooks, each a selection in its `implements` and a key of its `tools`
that names `setup-hook.v1`, `retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` or
`sign-out-hook.v1`: the engine's calls for a hook that `keyed` lists carry Conduit's keys, and its
other calls to the implementation do not." IFC-037's `then`: "Before its first such call to each
run of the implementation, which for an implementation marked `plain` is its first call for a hook
that its item's `keyed` lists (PKG-020), ... the engine reads the implementation's result of
`server/discover`, and it may read it again at any time. It declares Conduit's extension,
`io.github.shrav-k.conduit/implementation-interface`, with `revision` at `v0` in its settings
object, on each call, in `io.modelcontextprotocol/clientCapabilities`. Where that discovery result
does not list the extension at the revision `v0`, the request, the hook or the provider call is not
dispatched and reports unavailable, its record names the missing extension". Its `observable`:
"Where the discovery result of an implementation marked `plain` whose item states `keyed` does not
list the extension, only the hooks that `keyed` lists are not dispatched, and its other calls are
sent as before." IFC-001 names a plain implementation's call by "the name its `tools` map gives
under PKG-020, on every call, one for a hook that its item's `keyed` lists among them". PND-010's
`then`: "one naming a stated login reports invalid where the instance's sign-in-hook.v1 is a plain
hook, its implementation marked `plain` and its item's `keyed` not listing sign-in-hook.v1
(PKG-020), with a diagnostic naming PND-010 and the login, and saying that a plain hook gets no
login name (IFC-007) and that the request may be sent without signInLogin." PND-010's
`observable`: "If that check succeeds with no account or another account, or fails or ends
unknown, the engine invokes sign-in-hook.v1 as a part ... A check there that failed or ended
unknown counts as no account: it raises nothing that PND-005 does not raise for a contract that
reports a missing or lapsed login, and clears no condition." PND-011's `observable`: "A setup
check that failed or ended unknown raises the action under PND-005 as no account does, undoes
nothing, and lets setup proceed to its setup hook (TPL-012)." TPL-012's `then`: "an account check
that failed or ended unknown counts here as one that succeeded with no account (PND-011)". OPS-022's
`then`: "Its outcome is succeeded only if every part not marked optional succeeded, an account check
that failed or ended unknown counting as succeeded under TPL-012". PKG-020's `observable` rejects
"a selection in `keyed` that is absent from `implements`, absent from the keys of `tools` or names
an operation other than those five hooks", and for a keyed hook's call the implementation "keeps
IFC-008, IFC-009, IFC-019, IFC-026 and IFC-047, and OPS-006 and OPS-007, for it". IFC-009's
`when`, and those of OPS-006 and OPS-007, end: "IFC-010 governs a URL-mode `input_required`
result from a part of sign-in.v1 in place of this rule." IFC-010's `then`: "For an
`input_required` result holding a URL-mode input request from a part of sign-in.v1, the engine
follows this rule in place of the terminal outcome and output checks of IFC-009, OPS-006 and
OPS-007, for a plain call and for a call that carries Conduit's keys. From the sign-in-hook.v1
part, it relays the request under PND-010 and keeps the part and sign-in.v1 in progress when the
request passes IFC-049's checks. ... From any other part of sign-in.v1, a URL-mode input request
ends that part unknown. An account check that so ends follows PND-010 and PND-011." No promise
changes: order 207 states all of it, and this order builds it.

**Where it sits.** Off the sign-in chain: the owner's option named it "one spec + one engine order,
off the chain". It dispatches only after order 207 merged (#399; order 048a-5b merged first, #387),
and this text is refreshed against that merge. No `sign-in-hook.v1` part runs before order
048a-5d, so this order's own case keys `account-hook.v1`, the hook that `sign-in.v1` and setup
already dispatch; the keyed sign-in hook follows from the same code once 048a-5d dispatches it.

**This text's refresh.** The coordinator wrote this order against `main` at `1d3c7301` and order
207's text, before order 207 was built, and refreshed it on 2026-11-19 against `main` at
`0b5b1eba`, where order 207 has merged with its Record (#399) and order 048a-5c (#397) before it.
Every change below is the coordinator's, open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- the start check reads order 207 as merged: its eight case hashes (unchanged), its expected
  failures with their narrowed reasons, its section of `engine/CONFORMANCE.md` as its round 2 and
  fixer left it, PND-010's and TPL-012's texts, MAN-009's `keyed`, the exception for a URL-mode
  `input_required` result from a part of `sign-in.v1` in the `when` of IFC-009, OPS-006 and
  OPS-007, IFC-010's rule for plain and keyed calls, PKG-020's `implements` check, and the three
  places of `engine/CONFORMANCE.md` that order 048a-5c changed; it also records the start's runner
  counts;
- every cited line is that of `0b5b1eba`;
- decision 2 keeps IFC-010's reading for a keyed hook: `complete`'s URL arm stays ahead of the
  reading of a keyed result, so a URL-mode `input_required` result from a keyed part of
  `sign-in.v1` is never read under IFC-009, OPS-006 or OPS-007; the case gains steps 22 to 24,
  which show it on a keyed account check, and the mutations a row;
- two cases that order 207's merged section asks of this order (decision 8): a core hook listed in
  `keyed` and `tools` but not in `implements`, refused under PKG-020
  (`requests-keyed-not-in-implements`); and a setup whose failed account check goes on to its
  selected setup hook, followed by a failed check before `sign-in-hook.v1` that clears no `False`
  `SignInUndone` (`requests-sign-in-setup-check-failed-hook`); with three mutation rows among
  them;
- the mutation rows give what the runner prints, as a run of each against a prototype showed: the
  step-21 row's failure is the missing `invocation-ended` record, not an intent, and the
  `requests-sign-in-setup-check-failed` filter runs two cases;
- the two diagnostics of PND-010 that order 207's "What follows" asks this order to tell apart are
  told apart by their whole text in decision 7's unit test, and in the cases by their inputs, a
  stated login against an unknown one, since a diagnostic's prose is not a portable expectation
  ([the case rules](../../conformance/README.md#assertions));
- decision 4 places (c) before order 048a-5c's fixed-port check;
- decision 10 follows the text that orders 048a-5c and 207 left: its three places of "the checks of
  orders 048a-2 and 048a-5" keep 048a-5c's clause, and its replacement section keeps what order
  207's section records of the access record and of IFC-010 for a plain or keyed hook, and names
  the two new cases;
- the case's promises gain IFC-010; Size, "Done when" and the declaration follow.

**What orders 207, 048a-5b and 048a-5c bring, which this order assumes.** Order 207 (its file
`207-keyed-hooks-and-sign-in-spec-fixes.md`) has merged with its Record (#399), order 048a-5c
(#397, its file `048a5c-build-sign-in-fixed-port.md`) before it, and order 048a-5b (#387, its file
`048a5b-build-sign-in-account-first.md`) before both. This order assumes, as their decisions and
Records state them:

- From order 207's decisions 1 to 11, as its Record amends them: the texts quoted above, in
  `spec/`; `manifest.v1`'s `implementation-item.v1` with `keyed`, an array of selections,
  `minItems: 1`, `uniqueItems: true`, refused beside `remote` and beside an item not marked
  `plain: true`, so that the loader, which reads `manifest.v1` from `spec/`, already refuses those
  at the schema; MAN-009 and MAN-010 listing `keyed` among an item's keys; and the rules table row
  that leaves to PKG-020's rule that each selection is in `implements`, is a key of `tools` and
  names one of the five hooks. The schema does not check that, so the loader accepts a selection
  in `keyed` and `tools` that is not in `implements` (decision 8's second case, run against
  `0b5b1eba`).
- From order 207's decision 13: four cases under `conformance/conventions/requests/`, on the files
  of `requests-sign-in-lapse-and-conflict` with manifests of their own, or on `requests-sign-in-hold`'s:
  `requests-keyed-hook-manifest` (passes; a plain item that lists its sign-in hook under `keyed`,
  whose `group.sign-in.v1` naming `browser` dispatches a plain `account_hook` call);
  `requests-keyed-not-a-hook` (a plain item whose `keyed` lists `#ping.v1`; its step 2 expects the
  registration refused invalid under PKG-020); `requests-sign-in-login-plain-hook` (a plain item
  with no `keyed`; steps 3 and 4 name `browser` and `device` and expect invalid under PND-010
  before any part, with the records `access-decision` and `request-ended` with reason invalid and
  `ruleId` PND-010, step 5 names `phone`); and `requests-sign-in-setup-check-failed` (setup's
  account check failed at its step 6, expecting the setup succeeded with the sign-in action raised
  and no undo, and unknown at its step 17, expecting no second action, and `SignInUndone` still
  `False` at its step 18; it selects no setup hook). The last three are expected failures in
  `conformance/expected-failures.yaml`, each with a `reason` that says that order 209 makes it
  pass and removes its entry; on `0b5b1eba` they fail with `missing ordered record` of the
  `access-decision` with reason invalid and `dispatched` false, `missing ordered record` of the
  `request-ended` with reason invalid and `ruleId` PND-010, and, at step 6, outcome `unknown` where
  `succeeded` is expected.
- From order 207's decision 15, as its round 2 and fixer left it: the last section of
  `engine/CONFORMANCE.md`, `## Ahead of the engine: ruling 86 (order 207)`, which says what the
  build lacks until this order, and asks of it "a registration case for a core hook listed in both
  `keyed` and `tools` but missing from `implements`" and "a case with a selected setup hook that
  asserts its dispatch after a failed or unknown check".
- From order 048a-5c: `fixed_port_refusal`, which `sign_in::preflight` calls after `refusal`, so
  after `request_member_refusal`, LIF-037's stale `expectedId` and 048a-2's conflicts; and DL18's
  row, "Consumer server (order 022)" and "What the first build is" each reading "where the checks
  of orders 048a-2 and 048a-5 apply, unavailable where order 048a-5c's fixed-port check applies,
  and otherwise ...".
- From order 048a-5b: `sign_in::begin`, which dispatches `account-hook.v1` as the first part of
  `sign-in.v1` through the part function `setup_sign_in_hook` and its `dispatch_hook`;
  `sign_in::part_answered`, which reads the check with `first_check` (`Expected`, `NoAccount`,
  `Other`, `NotChecked`) and, for `NoAccount` or `Other`, ends `sign-in.v1` through its interim
  ending, the decision `end-sign-in-before-relay` and `end_failed` under `PND-010`, and for
  `NotChecked` ends it failed under `PND-011` with the cause
  `The account check before sign-in-hook.v1 ended {outcome}, so the account is not known and sign-in.v1 completes nothing; send sign-in.v1 again.`;
  `request_member_refusal(route, instance, request, names, context)` and `rejection`, which
  writes `{tool} reports {reason} under {rule} for {kind}/{name}: {cause}`; and the case
  `requests-sign-in-account-first`, whose step 24 completes the account check `$check3` with a
  failed keyed result and expects `sign-in.v1` failed with a diagnostic naming `PND-011`.
- From order 048a-4: `sign_in_part_answered` (`lifecycle.rs`), which records setup's account part
  through `part`, ends a setup whose check did not succeed through `settle`, and otherwise raises
  the sign-in action on no account or another, through `raise_sign_in`'s setup form with a
  trigger (`setup-no-account` or `setup-other-account`).

Where an order merged otherwise than this says, apply the nearest sound reading of each decision
below to what merged, and say so. Where a Record names a function, use that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`0b5b1eba`, after orders 048a-5b, 048a-5c and 207, of which order 207 changed no source; find each
place by the text quoted with it.

- `check_items` (`engine/crates/core/src/packages.rs:975`) checks implementation items at
  registration and installation; for an item marked `plain` (`if item["plain"] == true {`, line
  996) it refuses under `PKG-020` an operation of `implements` with no key in `tools`, or a
  provider operation, with `diag("PKG-020", ...)`.
- `dispatch_intent` (`engine/crates/core/src/requests.rs:812`) writes a call. It names the tool
  from the item's `tools` where `v["plain"] == true` (line 819), else by the operation's local
  name; writes the `_meta` of Conduit's keys only where `v["plain"] != true` (line 829), with
  `idempotencyKey` where `v["key"]` is not null and no `conduit` key for a hook; and, under the
  comment `// IFC-045: URL elicitation is declared only toward an instance whose Type states
  signIn.` (line 850), sets `_meta["io.modelcontextprotocol/clientCapabilities"]` to
  `{"elicitation": {"url": {}}}`. It declares no extension on any call: today no call the engine
  sends declares Conduit's extension, and nothing reads `server/discover`
  (`engine/STANDING.md`'s row of IFC-037: no case, no source).
- `dispatch_hook` (`requests.rs:2500`) builds every hook invocation, setup, retire, account,
  sign-out and, from order 048a-5d, sign-in hooks alike, through `hook` (a setup or retire hook,
  with a stable key, re-dispatched afresh after a failure under OPS-028) or `setup_sign_in_hook`
  (a fresh part, key null). Its literal sets `"plain"` to whether the implementation's item is
  marked `plain` (line 2528); it then writes the record `invocation-dispatched` (from line 2538),
  pushes `intent(&v, &t.state, c)?` (line 2545) and replies
  `{"pending":true,"invocation":<parent>,"hook":<ID>}`.
- `complete` (`requests.rs:1938`) reads a completion. In its `match completion`, the URL arm,
  `Completion::Result { result } if sign_in::url_elicitation(&result) => {` (line 2044), comes
  before the reading of any other result, plain or not, and reads neither `plain` nor the part:
  it sets `lapse` to `sign_in::Trigger::UrlElicitation`, so that `finish_login_lapse` (line 2169)
  ends the call unknown and raises the instance's sign-in action, as IFC-016 has a URL elicitation
  outside `sign-in.v1` do; within `sign-in.v1` it ends an account check unknown, as IFC-010 has
  one end, and `part_answered` then reads that outcome. For an invocation not plain, `complete`
  reads any other result's stated outcome (IFC-009), its reason (the IFC-026 arm, from `let
  restriction = meta.get(mark(c, "restriction"));`, line 2070, which restores or removes the invocation, calls
  `reject` with a route built from it, pushes the request reply and returns
  `crate::lifecycle::child_answered(t, &mut a, c, &v, reason.unwrap())`), and a -32021 capability
  error (IFC-008, IFC-047); for a plain one, IFC-010's table. Its first test, under
  `// IFC-046's first response ends no call: its resend remains in flight.` (line 1953), fences an
  answered attempt; then it ignores a completion whose attempt is not the invocation's or whose
  invocation is not in progress.
- `request_member_refusal` (`engine/crates/core/src/sign_in.rs:660`) decides the capability, a
  return on a first request and the asked-for login, its login check giving none where the name is
  stated (`Some(name) if names.iter().any(|stated| stated == name) => return None,`, line 691).
  `refusal` (line 720) calls it after 048a-2's invalid, then decides LIF-037's stale `expectedId`
  and 048a-2's conflicts; `preflight` (line 497) calls `refusal`, then order 048a-5c's
  `fixed_port_refusal`.
- `part_answered` (`sign_in.rs:208`) ends `sign-in.v1` under `PND-011` in its arm
  `FirstCheck::NotChecked => {` (line 253), whatever the part's outcome.
- `sign_in_part_answered` (`engine/crates/core/src/lifecycle.rs:1269`), where the part's outcome is
  not `succeeded` (`if outcome != "succeeded" {`, line 1292), raises nothing and, for an account
  check of an instance not retired, returns `settle(t, a, c, i)`. `settle` (line 400) counts the
  setup's `parts`: succeeded only where each part is optional or `succeeded` (line 405).
- `engine/crates/shell/src/runner.rs`'s `call` sends an intent's `call` as a `ClientRequest`
  parsed from it, so any client request that rmcp 3.9.0 knows, `server/discover` among them
  (`DiscoverRequest`), travels as a `tools/call` does, and its completion comes back through the
  shell's `complete`. The runner needs no change; decision 7 pins that the call parses.

**When this order starts.** Order 207 has merged to `main`, and orders 048a-5c and 048a-5b before
it. Before you change anything, check from the repository root that you find what they give, and
stop, change nothing and report if any check prints otherwise:

```bash
grep -c '^    keyed:$' spec/schemas/manifest.yaml
grep -c '^      keyed: false$' spec/schemas/manifest.yaml
ls -d conformance/conventions/requests/keyed-hook-manifest conformance/conventions/requests/keyed-not-a-hook conformance/conventions/requests/sign-in-login-plain-hook conformance/conventions/requests/sign-in-setup-check-failed | wc -l | tr -d ' '
ls -d conformance/conventions/requests/keyed-hook-dispatch conformance/conventions/requests/keyed-not-in-implements conformance/conventions/requests/sign-in-setup-check-failed-hook 2>/dev/null | wc -l | tr -d ' '
grep -c '"id": "requests-keyed-not-a-hook"\|"id": "requests-sign-in-login-plain-hook"\|"id": "requests-sign-in-setup-check-failed"' conformance/expected-failures.yaml
grep -c '^## Ahead of the engine: ruling 86 (order 207)$' engine/CONFORMANCE.md
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Order 209 also needs a registration case for a core hook listed in both .keyed. and .tools. but missing from .implements.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Order 209 needs a case with a selected setup hook that asserts its dispatch after a failed or unknown check.'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c "reports invalid where the instance's sign-in-hook.v1 is a plain hook, its implementation marked .plain. and its item's .keyed. not listing sign-in-hook.v1 (PKG-020)"
tr -s ' \n' '  ' < spec/promises/templates.yaml | grep -c 'an account check that failed or ended unknown counts here as one that succeeded with no account (PND-011).'
tr -s ' \n' '  ' < spec/promises/manifest.yaml | grep -c 'where they are given, .plain., .keyed., .tools., .supports.'
tr -s ' \n' '  ' < spec/promises/packages.yaml | grep -c 'a selection in .keyed. that is absent from .implements., absent from the keys of .tools. or names an operation other than those five hooks'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -o 'IFC-010 governs a URL-mode .input_required. result from a part of sign-in.v1 in place of this rule.' | wc -l | tr -d ' '
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -o 'IFC-010 governs a URL-mode .input_required. result from a part of sign-in.v1 in place of this rule.' | wc -l | tr -d ' '
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'in place of the terminal outcome and output checks of IFC-009, OPS-006 and OPS-007, for a plain call and for a call that carries Conduit.s keys.'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'From any other part of sign-in.v1, a URL-mode input request ends that part unknown. An account check that so ends follows PND-010 and PND-011.'
grep -c '^## Record' docs/work-orders/048a5b-build-sign-in-account-first.md docs/work-orders/048a5c-build-sign-in-fixed-port.md docs/work-orders/207-keyed-hooks-and-sign-in-spec-fixes.md
grep -c 'FirstCheck::NotChecked => {' engine/crates/core/src/sign_in.rs
grep -c '"plain":implementation\["item"\]\["plain"\] == true' engine/crates/core/src/requests.rs
grep -c 'Completion::Result { result } if sign_in::url_elicitation(&result) => {' engine/crates/core/src/requests.rs
grep -rl 'server/discover' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '\["keyed"\]' engine/crates | wc -l | tr -d ' '
grep -c '^| IFC-037 | interface | must | 0 | 0 | outside | none | no case, no reason |$' engine/STANDING.md
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-account-first/case.yaml"))["steps"]; print(s[23]["input"]["invocation"], s[23]["expect"]["reply"]["structuredContent"]["diagnostics"][0]["ruleId"], len(s))'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply, unavailable where order 048a-5c.s fixed-port check applies, and otherwise' | wc -l | tr -d ' '
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'while a failed or unknown check ends it failed and completes nothing (PND-011)'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Until order 048a-5e builds the relay, a check that finds no account or another ends .sign-in.v1. failed'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'sends first sets it .True. the same way (order 048a-5b). No other kind is held'
```

They print, in order:

1. `2` and `1`: order 207's schema, the property and its dependentSchemas entry, and its `else`;
2. `4` and `0`: its four cases, and none of this order's three;
3. `3`: its expected failures;
4. `1`, `1` and `1`: its section of `engine/CONFORMANCE.md`, and the two cases that section asks of
   this order;
5. `1`, `1`, `1` and `1`: PND-010's refusal, TPL-012's counting, MAN-009's `keyed` and PKG-020's
   rejection of a selection outside `implements`;
6. `1`, `2`, `1` and `1`: IFC-009's exception, OPS-006's and OPS-007's, IFC-010's rule for plain
   and keyed calls, and its ending of a part other than the `sign-in-hook.v1` part (decision 2);
7. three lines, each ending `:1`: the Records of orders 048a-5b, 048a-5c and 207;
8. `1`, `1` and `1`: the places decisions 5 and 2 change, and the URL arm that decision 2 keeps;
9. `0` and `0`: no discovery yet, and no source reads `keyed`;
10. `1`: IFC-037's row;
11. `$check3 PND-011 32`: the step decision 9 changes;
12. `3`, `1`, `1` and `1`: the places decision 10 changes, the first three as order 048a-5c left
    them.

Then check that order 207's cases are as its "Done when" gives them: from the repository root,
`shasum -a 256 -c` (`sha256sum -c` on Linux) on the first eight lines of order 207's hash list, its
four `case.yaml` files and three `manifest.json` files and its example, prints eight lines each
ending `: OK`. They did on `0b5b1eba`. If order 207 merged otherwise, its merged files govern:
report the hashes you find, and apply this order to them.

Then, still before any change, from `engine/`, one at a time, run these, keeping each last line:
"Done when" compares with them.

```bash
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- requests::tests
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- lifecycle
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
```

On `0b5b1eba` the two runner commands end `8 cases passed; 2 expected failures` and `72 cases
passed; 5 expected failures`.

Orders that land beside this one (048a-5d, 050 and others): whichever lands second keeps
both changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** Ruling 86 decides D as the route for keying, o1 read per operation, D's
engine half in 0.2.0 and in this order, no keyed `content` copy, and points 3 (c) and (d), as the
owner's three answers put them; that this order builds (c) and (d) is the coordinator's reading of
the second answer, which ruling 86's record states. PKG-020, IFC-001, IFC-006, IFC-007, IFC-009,
IFC-010, IFC-037, OPS-006, OPS-007, PND-010, PND-011, PND-005, TPL-012, TPL-017 and OPS-022, as
order 207 and its Record left them, decide what a keyed hook's call holds, that discovery comes
before it, that a missing extension leaves only those hooks undispatched and reports unavailable,
that a URL-mode `input_required` result from a part of `sign-in.v1`, plain or keyed, is read by
IFC-010 and not by the terminal checks of IFC-009, OPS-006 and OPS-007, the refusal of (c), that a
failed or unknown check counts as no account, and that setup goes on to its setup hook after
one. The narrowing of D to the five core hooks on an item reached by `command` or
`package`, and setup's check going on alike, are the coordinator's in order 207, open to the
owner's strike. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- **reading discovery before every call for a keyed hook**, not once per run: IFC-037 asks for it
  before the first call to each run and lets the engine "read it again at any time", and hooks are
  few, so the core keeps no discovery result between calls and needs no notion of a run, and no
  result read for one run is ever taken for another (IFC-037's `observable`); a later order may
  keep a result per run where the cost shows;
- **discovery as a call of the hook's own invocation**: the core sends `server/discover` as a
  `dispatch` intent with the hook's ID and attempt, the runner sending it as any call, and reads
  its completion through `complete` before any other test; it writes no `invocation-dispatched`
  record until the result lists the extension, so that a hook whose discovery fails was never
  dispatched (IFC-037: "not dispatched and reports unavailable");
- **what counts as listed**: a result whose `capabilities.extensions` holds the extension with
  `revision` `"v0"`; an error, a missing answer or any other result does not list it;
- **ending a hook whose discovery does not list the extension** with the reason `unavailable` as
  the IFC-026 arm ends a hook that states a reason, the `request-ended` record naming the missing
  extension in `missingExtension` and `extensionRevision`, its diagnostic's words, and the
  decisions' names `read-implementation-discovery` and `hook-without-extension`;
- **sending nothing for a hook whose parent ended while its discovery was in flight**: where
  discovery comes back listed and the hook's `setup_parent` names an invocation that is not
  `in-progress`, the hook ends unknown under OPS-027, as lifecycle's cancel finishes a child, with
  the decision `drop-hook-of-ended-parent`, since a cancelled `sign-in.v1` leaves its part in
  progress (order 048a-5b) and must open no login after it;
- **building discovery and the extension's declaration for a keyed hook of a plain item alone**:
  a call to an implementation not marked `plain` keeps carrying Conduit's keys with neither, as
  today, which is IFC-037's gap for those calls, recorded in `engine/CONFORMANCE.md` and not this
  order's; so the request path, and every existing case, is unchanged;
- naming the tool from `tools` where the call is plain or its item is marked `plain`, so that no
  existing call changes its name;
- for (c): its place, after the refusal of a name the Type does not state and before LIF-037's
  stale `expectedId`, 048a-2's conflicts and order 048a-5c's fixed-port check; a plain hook being one whose resolved implementation's item is marked `plain` and
  whose `keyed` does not list the core's `sign-in-hook.v1`; and its diagnostic's words;
- for (d) in `sign-in.v1`: sending a check that ended `failed` or `unknown` down the path of no
  account, with no clear and no raise, to 048a-5b's interim ending under `PND-010` with a cause of
  its own; and keeping 048a-5b's `PND-011` ending for a check that ended with a reason, such as
  `unavailable` under IFC-037, since PND-010 and PND-011 name only a check that "fails or ends
  unknown";
- for (d) in setup: the trigger `setup-check-failed` or `setup-check-unknown`; recording the
  account part with its own outcome and counting it as succeeded through a member of the setup,
  `accountCounted`, so that the setup's status shows the part as it ended; keeping a check that
  ended with a reason, or an instance retired meanwhile, on the path it takes today;
- keeping `complete`'s URL arm, unchanged, ahead of the reading of a keyed result, as the way this
  build meets IFC-010 for a keyed part of `sign-in.v1`, and leaving as it is the sign-in action
  that the arm raises for a part of `sign-in.v1` (decision 12's findings);
- the unit tests, the three cases, their manifests, documents, steps, IDs and times, the one step
  changed in `requests-sign-in-account-first`, the edits to `engine/CONFORMANCE.md`, and the eleven
  mutations.

Apply them as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PKG-020 (`spec/promises/packages.yaml`); MAN-009 and MAN-010
   (`spec/promises/manifest.yaml`); IFC-001, IFC-006, IFC-007, IFC-008, IFC-009, IFC-010, IFC-016,
   IFC-026, IFC-037, IFC-045 and IFC-047 (`spec/interface/implementation.yaml`); PND-005, PND-010
   and PND-011 (`spec/promises/pending.yaml`); TPL-012 and TPL-017 (`spec/promises/templates.yaml`);
   OPS-006, OPS-007, OPS-022 and OPS-028 (`spec/conventions/operations.yaml`); the rows "plain,
   `keyed`", "`keyed`" and "Conduit's extension" of `spec/definitions.md`.
2. Order 207 (its file `207-keyed-hooks-and-sign-in-spec-fixes.md`), its decisions 1 to 15,
   "What follows" and its Record, whole; [order 048a5b](048a5b-build-sign-in-account-first.md), its decisions 1 to 5 and 8 and
   its Record; [order 048a5c](048a5c-build-sign-in-fixed-port.md), its Record; [order 048a4](048a4-build-sign-in-setup-account.md), its decisions 2 to 4.
3. The cases `requests-sign-in-hold`, `requests-sign-in-account-first`,
   `requests-sign-in-refusals-before-parts`, `requests-sign-in-lapse-and-conflict` and order 207's
   four.
4. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.3 to 0.4k changed lines of Rust with three unit tests in the core and one in
the shell, three new cases (one of 24 steps on 048a-3's fixture with a manifest of its own, one of
2 steps on 048a-2's fixture with a manifest of its own, and one of 16 steps on 048a-3's fixture
with a manifest and documents of its own), one step's expectation changed in one case, three
entries removed from the expected failures, and six places and one section in
`engine/CONFORMANCE.md`.

## The decisions it applies

Below, `Q(x)` is `c.qualified(x)`, the core's qualified reference to the operation `x`, and `EXT`
is `mark(c, "implementation-interface")`, Conduit's extension, as `capability_error` writes it.

1. **`keyed` checked under PKG-020.** In `check_items` (`packages.rs`), inside the block for an
   item marked `plain`, after its loop over `implements`: for each string `s` of the item's
   `keyed` array, where `s` is not one of the strings of `implements` or not a key of `tools`, push
   `diag("PKG-020", format!("{scope}#{name}: keyed lists {s}, which is not a selection in implements and a key of tools"))`;
   else, where `s` is not `format!("{core}#{hook}")` for one of `setup-hook.v1`,
   `retire-hook.v1`, `sign-in-hook.v1`, `account-hook.v1` and `sign-out-hook.v1`, `core` being
   `check_items`'s parameter, push
   `diag("PKG-020", format!("{scope}#{name}: keyed lists {s}, which is not one of the core hooks setup-hook.v1, retire-hook.v1, sign-in-hook.v1, account-hook.v1 and sign-out-hook.v1; keyed lists only hooks whose calls carry Conduit's keys"))`.
   The schema already refuses `keyed` on an item that is not plain or is reached by `remote`
   (order 207), so add no check for those. `requests-keyed-not-a-hook` then passes, by the second
   message, and `requests-keyed-not-in-implements` (decision 8) by the first. Where `core`
   is not the core package's address as the selections in `implements` write it, use the value
   that `c.qualified` writes and say so.

2. **`plain` per call on the hook path.** In `requests.rs`, add
   `pub(crate) fn keyed_hook(item: &Value, operation: &str) -> bool`: true exactly where
   `item["plain"] == true` and `item["keyed"]` is an array holding the string `operation`. Then:
   - In `dispatch_hook`'s literal, `"plain"` becomes
     `item["plain"] == true && !keyed_hook(item, &op)`, `item` being the implementation's item
     (`implementation["item"]`). The request path, `invoke_inner`'s `let plain` from the item,
     does not change: `keyed` lists hooks alone, so a request stays plain (OPS-012, IFC-025).
   - In `dispatch_intent`, the tool is named from `tools` where `v["plain"] == true` or the
     item is marked `plain`, as IFC-001 has a plain implementation's call named
     on every call; otherwise by the local name, as now.
   - In `dispatch_intent`, where `v["plain"] != true` and the item is marked `plain`, a
     call for a hook that its item lists under `keyed`, the call's
     `_meta["io.modelcontextprotocol/clientCapabilities"]` holds `{"extensions": {EXT:
     {"revision": "v0"}}}`, and the IFC-045 block, where it applies, adds `"elicitation": {"url":
     {}}` beside `extensions` rather than replacing the object. No other call declares the
     extension (see "Who decided what").

   Everything else follows from the invocation's per-call `plain`: `dispatch_intent` writes the
   `_meta` of Conduit's keys, with a setup or retire hook's `idempotencyKey` from its stable key
   (OPS-028) and no `conduit` key; `complete` reads the result's stated outcome or reason (IFC-009,
   IFC-026, OPS-006, OPS-007) and a -32021 under IFC-008 or IFC-047; and the cancellation and
   re-dispatch paths read the invocation's `plain`. Report any place that reads the item's `plain`
   in place of the invocation's on the hook path, where this order leaves it.

   The one exception is IFC-010's: a URL-mode `input_required` result from a part of `sign-in.v1`
   is read by IFC-010, for a keyed call as for a plain one, in place of the terminal checks of
   IFC-009, OPS-006 and OPS-007. In this build `complete`'s URL arm takes it, ahead of the reading
   of a keyed or plain result and reading neither `plain` nor the part, and ends the part unknown,
   as IFC-010 ends a part other than the `sign-in-hook.v1` part; an account check so ended then
   counts as no account (decision 5). Keep the URL arm as it is and where it is: add no test of
   `plain`, of `keyed` or of `discovery` to it, and put no arm that reads a keyed result's stated
   outcome ahead of it. From order 048a-5d an arm ahead of it takes the `sign-in-hook.v1` part's
   `input_required` result into `sign-in.v1`, plain or keyed alike, so that a keyed sign-in hook's
   URL request is never read as a terminal outcome; this order dispatches no such part. Steps 22 to
   24 of decision 8's first case show the exception on a keyed account check whose URL request
   also states succeeded with the expected account, which IFC-009's reading would take as signed
   in.

3. **Discovery before each call for a keyed hook (IFC-037).** In `dispatch_hook`, where
   `keyed_hook(&implementation["item"], &op)` holds:
   - set the invocation's member `discovery` to `"pending"`;
   - write no `invocation-dispatched` record, and push in place of `intent(&v, &t.state, c)?` the
     intent `{"kind": "dispatch", "invocation": <ID>, "attempt": <attempt>, "target": <the
     object's ID>, "implementation": <implementation>, "call": {"method": "server/discover",
     "params": {}}}`;
   - reply as now, `{"pending":true,"invocation":<parent>,"hook":<ID>}`.

   Move the record's `data`, as `dispatch_hook` writes it today, into one function that both places
   call, so that the record is written alike. In `complete`, right after
   `let v = t.state.invocations[i].clone();` and before the IFC-046 test, where `v["discovery"] ==
   "pending"`, `v["attempt"] == attempt` and `v["state"] == "in-progress"`, read the completion
   and return, with an `Allocation` made as `complete` makes it further down:
   - **Listed**: the completion is a `Completion::Result { result }` whose
     `result.capabilities.extensions` holds EXT with `revision` `"v0"`. Where the hook's
     `setup_parent` names an invocation that is not `in-progress`, send nothing: finish the hook
     unknown under OPS-027 with `explain::outcome_cause(.., "unknown", "OPS-027")`, as lifecycle's
     cancel finishes a child, push the decision `hook-without-extension`'s sibling
     `drop-hook-of-ended-parent` citing OPS-027 and IFC-037, and return. Otherwise, set the
     invocation's `discovery` to `"read"`; write the `invocation-dispatched` record; push one
     decision named `read-implementation-discovery`, citing `IFC-037` and `PKG-020`, with the facts
     `{"invocation": <ID>}`; push `intent` of the invocation as it now stands, the call for the
     hook; reply `{"pending":true,"invocation":<ID>,"attempt":<attempt>}`, as the IFC-046 resend
     replies, and push no request reply.
   - **Not listed**: any other completion, an error, a missing answer or a result without the
     extension at `v0`. Push one decision named `hook-without-extension`, citing `IFC-037` and
     `PKG-020`, with the facts `{"invocation": <ID>, "extension": EXT, "revision": "v0"}`; then end
     the hook as the IFC-026 arm ends one that states a reason, restoring or removing the
     invocation as it does, through `reject_diagnostics` with the reason `unavailable`, the rule
     `IFC-037`, and the route the IFC-026 arm builds with two more members,
     `"missingExtension": EXT` and `"extensionRevision": "v0"`, which `end` copies into the
     `request-ended` record's `data`; its diagnostics one
     `{"ruleId": "IFC-037", "level": "error", "message": {"text": T}}`, `T` being
     `{op} reports unavailable under IFC-037 for {kind}/{name}: the implementation's result of server/discover does not list Conduit's extension {EXT} at revision v0, so this hook, which its item lists under keyed, was not sent; the item's other calls are sent as before.`,
     `{op}` the hook's operation and `{kind}/{name}` the instance as `sign_in::rejection` writes
     it; push the request reply as that arm does; and return
     `crate::lifecycle::child_answered(t, &mut a, c, &v, "unavailable")`.

   Nothing else reads `discovery`, and this arm reads only the completion of `server/discover`: the
   hook's own answer, which comes once `discovery` is `"read"`, goes through `complete` as any
   other, the URL arm included (decision 2). A later call for the same hook, or a re-dispatch under
   OPS-028, reads discovery again. Where `reject_diagnostics` cannot take this route or these diagnostics,
   use the nearest sound reading that keeps the reason, the rule, the two record members and the
   text, and say so.

4. **(c): a login name aimed at a plain hook.** In `request_member_refusal`, in place of the
   `return None` for a stated name (line 691), where the item of the instance's
   resolved implementation of `Q("sign-in-hook.v1")` (in `resolved_implementations`) is marked
   `plain` and `!keyed_hook(item, &Q("sign-in-hook.v1"))`, give
   `rejection("invalid", "PND-010", tool, instance, &cause)` with `cause`
   `signInLogin asks for the login {name}, but the instance's sign-in-hook.v1 is a plain hook, its implementation item marked plain and not listing sign-in-hook.v1 under keyed (PKG-020), which gets no login name (IFC-007); send the request again without signInLogin, and the hook chooses among the stated logins {names}.`,
   `{name}` and each of `{names}` written as the unknown-name text writes them; otherwise give
   none, as now. Where no implementation of `sign-in-hook.v1` is resolved, which CTR-013's checks
   leave no way to reach, give none. So the order is 048a-2's invalid, the capability, the return,
   the unknown name, then this, then, in `refusal`, LIF-037's stale `expectedId` and 048a-2's
   conflicts, and, in `preflight`, order 048a-5c's fixed-port check. Make `keyed_hook` reachable from `sign_in.rs`
   (`pub(crate)` in `requests.rs`); write no second copy.

5. **(d) in `sign-in.v1`.** In `part_answered`, the arm `FirstCheck::NotChecked` splits by the
   part's `outcome`:
   - `failed` or `unknown`: the check counts as no account (PND-010, PND-011). It clears nothing
     (the clear above it stays for `Expected` and `NoAccount` alone) and raises nothing; push the
     decision `end-sign-in-before-relay`, citing `PND-010` and `PND-011`, with the facts
     `{"invocation": <the sign-in.v1 ID>, "found": "none", "check": <outcome>}`; and end the
     invocation through `end_failed` under `PND-010` with the cause
     `The account check before sign-in-hook.v1 ended {outcome}, which counts as no account (PND-010, PND-011); sign-in.v1 would go on to start a login through sign-in-hook.v1 and relay it under IFC-049, which this build does not yet do (order 048a-5e), so it ends failed and nothing was signed in.`
     It is the interim ending of 048a-5b, which order 048a-5d moves and order 048a-5e removes. A
     check that the URL arm ended unknown (decision 2) takes it too.
   - any other outcome, a reason such as `unavailable` under IFC-037 among them: the `PND-011`
     ending as 048a-5b wrote it, unchanged.

   `first_check` and its test do not change. Step 24 of `requests-sign-in-account-first` (decision
   9) shows a failed check so ended; steps 15 and 16 of `requests-sign-in-setup-check-failed-hook`
   (decision 8) show that it leaves a `False` `SignInUndone` as it is.

6. **(d) in setup.** In `sign_in_part_answered`, where the part is the account check, its outcome
   is `failed` or `unknown`, and the instance is not retired, in place of the early `settle`:
   - where `expected_account` gives `Set { pointer, account }`, call the raise in its setup form
     with the trigger `format!("setup-check-{outcome}")`, as the no-account branch calls it, keep
     an action it gives in `setup.raised` and push `raise-sign-in-action` with the promises that
     branch cites; a raise that finds an unanswered action gives none, as now;
   - set the setup's member `accountCounted` to the outcome; dispatch no sign-out part; clear no
     condition;
   - set the setup's phase to `hook` and call `advance`, as the end of the function does.

   In `settle`, a part counts as succeeded where `p["outcome"] == "succeeded"`, or where
   `p["name"] == "account"` and the setup's `accountCounted` is a string; the `parts` that
   `status.setup` shows keep the check's own outcome. An account check that ended with a reason,
   a sign-out part, and a check of an instance retired meanwhile take the path they take today.
   `requests-sign-in-setup-check-failed` then passes, and so does
   `requests-sign-in-setup-check-failed-hook` (decision 8), whose template selects a setup hook,
   which the setup dispatches after the failed check (TPL-012, TPL-017). Where `settle`'s cause (`explain::settle_cause`)
   reads the parts so that a succeeded setup gains a diagnostic, stop and report it. In
   `explain::settle_cause`, skip a part named `account` where the setup's `accountCounted` is a
   string, as `settle` counts it (`explain.rs` is in scope for decision 6's cause). In
   `explain.rs`'s tests, add `settle_cause_skips_a_counted_account_check`, which settles a setup
   with a counted account part and a failed hook part and asserts that the cause names `hook`.

7. **The unit tests.** Write the values as `json!` values.
   - In `requests.rs`'s tests, `keyed_hooks_read_plain_per_call`. With
     `item = {"plain": true, "tools": {Q("setup-hook.v1"): "setup_hook", Q("sign-out-hook.v1"): "sign_out_hook"}, "keyed": [Q("setup-hook.v1")]}`,
     assert that `keyed_hook(&item, &Q("setup-hook.v1"))` holds (`assert!`), and that it does not
     (`assert!(!...)`) for `Q("sign-out-hook.v1")`, for `{"keyed": [Q("setup-hook.v1")]}`, not
     plain, and for `{"plain": true}`; assert the values below with `assert_eq!`. Then
     `dispatch_intent` of a hook invocation built as `call_fixture` builds its invocation, with
     `"hook": true`, `"operation": Q("setup-hook.v1")`, `"plain": false`,
     `"key": "s1:" + Q("setup-hook.v1")`, `"target": "s1"` and
     `"implementation": {"item": item}`: with `url_elicitation` `false`, `call.params.name` is
     `"setup_hook"`, its `_meta` holds `mark(c, "operation")` equal to `Q("setup-hook.v1")` and
     `mark(c, "idempotencyKey")` equal to the key, and no `mark(c, "conduit")`, and its
     `_meta["io.modelcontextprotocol/clientCapabilities"]` is exactly
     `{"extensions": {EXT: {"revision": "v0"}}}`; with `true`, exactly
     `{"extensions": {EXT: {"revision": "v0"}}, "elicitation": {"url": {}}}`. The same invocation
     with `"operation": Q("sign-out-hook.v1")`, `"plain": true` and `"key": null`, with `false`:
     `call.params.name` is `"sign_out_hook"` and `call.params` has no `_meta`. And with
     `"implementation": {"item": {"command": ["x"]}}`, `"plain": false`: `call.params.name` is
     `"setup-hook.v1"`, `_meta` holds `mark(c, "operation")`, and, with `false`, no
     `io.modelcontextprotocol/clientCapabilities`: the extension is declared only on a keyed hook
     of a plain item.
   - In `sign_in.rs`'s tests, `sign_in_login_refused_for_a_plain_hook`, which calls
     `request_member_refusal` with a route whose `tool` is `group.sign-in.v1`, the stated names
     `browser` and `device`, a `Context` as `sign_in_request_member_refusals` builds one, a
     request with `clientCapabilities` `{"elicitation": {"url": {}}}`, and an instance
     `{"kind": "Agent", "name": "signed", "resolved_implementations": {Q("sign-in-hook.v1"): {"item": I}}}`,
     and asserts with `assert_eq!` the reason, rule and whole text, or none: with `I`
     `{"plain": true, "tools": {}}` and `signInLogin` `{"name": "browser"}`, `PND-010` with the
     text
     `group.sign-in.v1 reports invalid under PND-010 for Agent/signed: signInLogin asks for the login "browser", but the instance's sign-in-hook.v1 is a plain hook, its implementation item marked plain and not listing sign-in-hook.v1 under keyed (PKG-020), which gets no login name (IFC-007); send the request again without signInLogin, and the hook chooses among the stated logins "browser", "device".`;
     with `{"name": "phone"}`, the unknown-name text, which comes first; with no `signInLogin`,
     none; with `I` `{"plain": true, "tools": {}, "keyed": [Q("sign-in-hook.v1")]}` and
     `{"name": "browser"}`, none; with `I` `{"command": ["x"]}` and `{"name": "device"}`, none.
   - In `engine/crates/shell/src/runner.rs`'s tests, `discover_call_parses_as_a_client_request`:
     `serde_json::from_value::<ClientRequest>(json!({"method": "server/discover", "params": {}}))`
     is `Ok`, as `call` parses an intent's call.

8. **Three portable cases.** The first, on 048a-3's fixture with a manifest of its own. Add the
   directory
   `conformance/conventions/requests/keyed-hook-dispatch/` with two files. `manifest.json` is
   `requests-sign-in-hold`'s `manifest.json` whose one item, `signed-server`, gains, after
   `implements`, `"plain": true`, `"tools"`, mapping each selection of its `implements` to the
   name after its `#`, less `.v1`, with `-` written `_` (`ping`, `work`, `set_limit`,
   `sign_in_hook`, `account_hook`, `sign_out_hook`), and `"keyed": [Q("account-hook.v1")]`, the
   selection written as `implements` writes it. `case.yaml`: ID `requests-keyed-hook-dispatch`,
   `layers: [language, convention, interface]`,
   `promises: [IFC-037, PKG-020, IFC-001, IFC-006, IFC-007, IFC-009, IFC-010, PND-010, PND-011]`, title
   `Send a hook that a plain item lists under keyed with Conduit's keys once discovery lists the extension, read its answer as a keyed implementation's, keep the item's other calls plain, and leave only that hook undispatched where discovery lacks the extension`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-hold/fixture.json", "manifest": "manifest.json", "documents": "../sign-in-hold/documents.json"}`.
   Write both as JSON with two-space indentation and a final newline, the case's members in the
   order of `requests-sign-in-hold`. It must meet 046a's portable-case rule as it stands.

   Below, ROOT, `META/x`, "Root asks T with A", "a completion" and `K(x)` are as orders 048a-3
   and 048a-4 define them; S, URL, LOGIN(X) and "Root asks T with A and M" as order 048a-5's
   decision 5 does. `CORE(x)` is the core's qualified reference to `x` written as the cases write
   it, as in `requests-sign-in-hold`'s step 5. A new step's evidence is its time,
   `2026-11-12T12:00:00Z`, `ids` the 100 strings `00000000-0000-4000-8000-` followed by the
   twelve-digit, zero-padded decimal of 190000 + 100n + k, for k = 0 to 99, `revisions` the 16
   strings `keyed-dispatch-` followed by the decimal of n, a `-` and the decimal of j, and
   `suffixes` the 16 strings `q` followed by the four-digit, zero-padded decimal of 100n + j, for
   j = 0 to 15, where n is the step's number in this case; a copied step keeps its own evidence.
   These recur:
   - DISCOVER: `{"kind": "dispatch", "attempt": 1, "call": {"method": "server/discover", "params": {}}}`;
   - LISTED: `{"kind": "result", "result": {"resultType": "complete", "supportedVersions": ["2026-07-28"], "capabilities": {"tools": {}, "extensions": {"META/implementation-interface": {"revision": "v0"}}}, "ttlMs": 0, "cacheScope": "private"}}`;
   - UNLISTED: LISTED with `capabilities` `{"tools": {}}`;
   - KEYED: `{"kind": "dispatch", "attempt": 1, "call": {"method": "tools/call", "params": {"name": "account_hook", "_meta": {"META/operation": CORE(account-hook.v1), "META/caller": ROOT, "io.modelcontextprotocol/clientCapabilities": {"extensions": {"META/implementation-interface": {"revision": "v0"}}, "elicitation": {"url": {}}}}}}}`;
   - WORK: `{"kind": "dispatch", "attempt": 1, "call": {"method": "tools/call", "params": {"name": "work"}}}`, with `"absent": ["/external_intents/0/call/params/_meta/io.github.shrav-k.conduit~1operation", "/external_intents/0/call/params/_meta/io.modelcontextprotocol~1clientCapabilities/extensions"]`;
   - SIGN(N): Root asks `group.sign-in.v1` with S and URL; `expect`
     `{"reply": {"pending": true}, "external_intents": [DISCOVER]}`; `capture`
     `{"signN": "/reply/invocation", "checkN": "/reply/hook"}`;
   - ENDED(N, RULE): `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/invocation": "$signN"}, "structuredContent": {"diagnostics": [{"ruleId": RULE, "level": "error"}]}}, "external_intents": [], "absent": ["/reply/_meta/io.github.shrav-k.conduit~1unanswered"]}`.

   Its steps:
   1. to 4. `requests-sign-in-hold`'s steps 1 to 4, `input`, `expect` and `capture`, whole, each
      with its evidence: first start, registration, which accepts the plain item and its `keyed`,
      the setup of `signed` (`$setup1`) and its first run report.
   5. That case's step 5, its `input` and `capture` whole, with its evidence; `expect`
      `{"reply": {"pending": true}, "external_intents": [DISCOVER]}`: the run is up, and the
      account check, a hook that `keyed` lists, first asks for discovery (IFC-037).
   6. A completion for `$account0`, LISTED. `expect` `{"reply": {"pending": true}, "external_intents": [KEYED], "record_facts": [{"type": "invocation-dispatched", "data": {"invocation": "$account0"}}], "absent": [K(conduit), K(idempotencyKey)]}`:
      the call carries Conduit's keys, is named from `tools` and declares the extension (IFC-001,
      IFC-006, IFC-037).
   7. `requests-sign-in-hold`'s step 6, `input` and `expect`, whole, with its evidence: the keyed
      answer states succeeded with the expected account, and the setup succeeds.
   8. That case's step 7, whole, with its evidence: root opens `to-signed`.
   9. That case's step 8, its `input` and `capture` whole, with its evidence; `expect`
      `{"reply": {"pending": true}, "external_intents": [WORK]}` with WORK's `absent`: a request
      to the same item stays plain (IFC-007).
   10. A completion for `$call1`, `{"kind": "result", "result": {"content": []}}`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}}`: read by IFC-010's
       table.
   11. SIGN(1).
   12. A completion for `$check1`, LISTED. `expect` `{"reply": {"pending": true}, "external_intents": [KEYED]}`:
       discovery is read again before this call.
   13. A completion for `$check1`, `{"kind": "result", "result": {"content": [], "structuredContent": {"account": "primary-account"}}}`.
       `expect` ENDED(1, `PND-010`): a keyed result that states no outcome is unknown (IFC-009),
       though a plain one would succeed with that account; an unknown check counts as no account
       (PND-010), and until order 048a-5e ends `sign-in.v1` failed; nothing is raised.
   14. SIGN(2).
   15. A completion for `$check2`, UNLISTED. `expect` ENDED(2, `PND-011`) with
       `"record_facts": [{"type": "request-ended", "data": {"reason": "unavailable", "ruleId": "IFC-037", "missingExtension": "META/implementation-interface", "extensionRevision": "v0"}}]`:
       the hook is not sent and reports unavailable (IFC-037), and a check that ended with a reason
       completes nothing (PND-011).
   16. Step 9's request. `expect` step 9's; `capture` `{"call2": "/reply/invocation"}`: the item's
       other calls are sent as before.
   17. A completion for `$call2`, step 10's. `expect` step 10's.
   18. Root asks `group.sign-in.v1` with S and URL and LOGIN(`{"name": "browser"}`). `expect`
       `{"reply": {"isError": true, "_meta": {"META/reason": "invalid"}, "structuredContent": {"diagnostics": [{"ruleId": "PND-010", "level": "error"}]}}, "external_intents": [], "record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "invalid", "ruleId": "PND-010"}}]}`:
       the item lists its account check, not its sign-in hook, so a stated login is refused before
       any part (PND-010).
   19. SIGN(3).
   20. Root cancels `$sign3` (`{"kind": "cancel", "invocation": "$sign3"}`). `expect`
       `{"reply": {"isError": true, "_meta": {"META/outcome": "unknown", "META/invocation": "$sign3"}}, "external_intents": [{"kind": "cancel", "invocation": "$sign3", "attempt": 1}]}`:
       the cancel leaves the account part in progress (order 048a-5b).
   21. A completion for `$check3`, LISTED. `expect`
       `{"external_intents": [], "record_facts": [{"type": "invocation-ended", "data": {"invocation": "$check3", "outcome": "unknown"}}]}`:
       discovery listed the extension, but the parent has ended, so the hook is not sent
       (decision 3).
   22. SIGN(4).
   23. A completion for `$check4`, LISTED. `expect` `{"reply": {"pending": true}, "external_intents": [KEYED]}`.
   24. A completion for `$check4`,
       `{"kind": "result", "result": {"resultType": "input_required", "inputRequests": {"login": {"method": "elicitation/create", "params": {"mode": "url", "message": "Sign in to the account.", "url": "https://example.org/authorize", "elicitationId": "keyed-dispatch-login"}}}, "isError": false, "content": [], "structuredContent": {"account": "primary-account"}, "_meta": {"META/outcome": "succeeded"}}}`.
       `expect`
       `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/invocation": "$sign4"}, "structuredContent": {"diagnostics": [{"ruleId": "PND-010", "level": "error"}]}}, "external_intents": [], "record_facts": [{"type": "invocation-ended", "data": {"invocation": "$check4", "outcome": "unknown"}}]}`:
       a keyed part's URL-mode `input_required` result is read by IFC-010 and not by IFC-009, so the
       outcome it states and the expected account it holds count for nothing: the account check
       ends unknown and counts as no account (PND-010, PND-011), where IFC-009's reading would sign
       the instance in (decision 2). The step asserts nothing of the `unanswered` key, since this
       build's URL arm raises the sign-in action for the part (decision 12), and it is the case's
       last step, since that action holds the instance (PND-005).

   The second, `requests-keyed-not-in-implements`, on 048a-2's fixture with a manifest of its own:
   a core hook in `keyed` and `tools` that is not in `implements` (PKG-020's `observable`). Add the
   directory `conformance/conventions/requests/keyed-not-in-implements/` with two files.
   `manifest.json` is `requests-keyed-not-a-hook`'s `manifest.json` whose item's `tools` gains, as
   its last member, `Q("setup-hook.v1")`, written as `implements` writes a selection, mapped to
   `"setup_hook"`, and whose `keyed` is `[Q("setup-hook.v1")]`, so written; its `implements` does
   not change and lists no setup hook. `case.yaml` is `requests-keyed-not-a-hook`'s `case.yaml`,
   whole, but for its `id`, `requests-keyed-not-in-implements`, and its `title`,
   `Refuse to register a plain implementation whose keyed lists a core hook that is a key of its tools but not in its implements, as invalid naming PKG-020`;
   its `files` still name `manifest.json`, now this directory's. Its step 2 expects the
   registration refused invalid under PKG-020, its `access-decision` and `request-ended` records
   with reason invalid and `dispatched` false.

   The third, `requests-sign-in-setup-check-failed-hook`, on 048a-3's fixture with a manifest and
   documents of its own: a failed setup check goes on to the setup hook that the template selects
   (TPL-012, TPL-017), and a failed check before `sign-in-hook.v1` clears no condition (PND-011).
   Add the directory `conformance/conventions/requests/sign-in-setup-check-failed-hook/` with three
   files. `documents.json` is `requests-sign-in-hold`'s `documents.json` whose Agent
   `signed-template` gains, after its `spec.config`, `"implementation": {Q("setup-hook.v1"): "#signed-server"}`,
   the selection written in full: a template's selection selects the hook, while a Type's would
   be a default that selects nothing, since the Type does not require it (TPL-004, TPL-017).
   `manifest.json` is `requests-sign-in-hold`'s `manifest.json` whose item's `implements` gains,
   last, `Q("setup-hook.v1")`, so written. `case.yaml`: ID
   `requests-sign-in-setup-check-failed-hook`, `layers: [convention, interface]`,
   `promises: [PND-011, PND-005, PND-010, TPL-012, TPL-017, OPS-022]`, title
   `Count a setup account check that failed as no account and go on to the selected setup hook, then end sign-in.v1 after a failed check with no condition cleared`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-hold/fixture.json", "manifest": "manifest.json", "documents": "documents.json"}`,
   the case's members in the order of `requests-sign-in-setup-check-failed`, which is called C
   below. A new step's evidence is the first case's, with 193000 in place of 190000,
   `check-failed-hook-` in place of `keyed-dispatch-` and `h` in place of `q`. HOOK is
   `{"kind": "dispatch", "attempt": 1, "call": {"params": {"name": "setup-hook.v1", "_meta": {"META/operation": CORE(setup-hook.v1), "META/caller": ROOT}}}}`.
   Its steps:
   1. to 5. C's steps 1 to 5, whole, each with its evidence: first start, registration, which
      accepts the template's selection, the setup (`$setup1`), its two run reports, and the
      account check (`$account0`).
   6. C's step 6, its `input` whole with its evidence, the check failed; `expect`
      `{"reply": {"pending": true}, "external_intents": [HOOK], "absent": [K(conduit)]}`; `capture`
      `{"hook1": "/reply/hook"}`: the failed check counts as no account and the setup goes on to its
      setup hook (TPL-012, TPL-017, PND-011).
   7. A completion for `$hook1`,
      `{"kind": "result", "result": {"isError": false, "content": [], "_meta": {"META/outcome": "succeeded"}}}`;
      `expect` C's step 6's `expect`, whole: the setup succeeds, its failed check counting as
      succeeded, and the sign-in action is raised (OPS-022, PND-005).
   8. C's step 7, whole, with its evidence: the action is pending (`$signin`).
   9. to 13. C's steps 10 to 14, whole, each with its evidence: a second setup (`$setup2`) whose
      check finds another account and whose sign-out fails, so it ends unknown and `SignInUndone`
      is `False`.
   14. Root asks `group.sign-in.v1` with S and URL. `expect`
       `{"reply": {"pending": true}, "external_intents": [the intent of C's step 11's expect]}`,
       with the `absent` of C's step 11; `capture` `{"sign1": "/reply/invocation", "check1": "/reply/hook"}`.
   15. A completion for `$check1`, C's step 6's `completion`. `expect`
       `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/invocation": "$sign1", "META/unanswered": 2}, "structuredContent": {"diagnostics": [{"ruleId": "PND-010", "level": "error"}]}}, "external_intents": []}`:
       the failed check counts as no account and takes 048a-5b's interim ending (decision 5).
   16. C's step 14's `input`, with this step's evidence, and its `expect`, whole: `SignInUndone` is
       still `False`, since a failed check clears no condition (PND-011).

   The coordinator ran these three routes against a prototype of decisions 1 to 6 in a scratch
   copy of `0b5b1eba` outside the repository: each case passed there, and each failed on
   `0b5b1eba` at the step that "Done when" names. Your build is not that prototype, so the rule
   stands: before running a case, check that no ID repeats across its steps' `ids`, the copied
   steps' included. If 046a's runner check refuses a case, if a step copied from another case
   gives a reply other than that case states, or if a step's reply differs from what it states,
   stop and report the error: do not change the expectations to pass it, and do not change the
   inputs beyond what this decision says. No expectation is generated from the engine's output.
   Each case's coverage index entry holds `id`, `case`
   (`conventions/requests/<directory>/case.yaml`), `promises` and, where its neighbours in that
   piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write the three
   in a `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves them into
   `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte for
   byte as it was.

9. **The step that changes, and the expected failures.** In
   `conformance/conventions/requests/sign-in-account-first/case.yaml`, step 24 (index 23), whose
   completion of `$check3` states failed, expects `"ruleId": "PND-010"` in place of `"PND-011"` in
   its reply's one diagnostic, and nothing else in the case, nor in its directory, changes: the
   failed check now counts as no account, and 048a-5b's interim ending answers it. In
   `conformance/expected-failures.yaml`, remove the three entries `requests-keyed-not-a-hook`,
   `requests-sign-in-login-plain-hook` and `requests-sign-in-setup-check-failed`, whole, and
   nothing else. Order 207's four cases do not change, and each must pass.

10. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each sentence by its text read
    across line breaks, as orders 048a-5c and 207 left it. Seven places change:
    - In DL18's row, "answers invalid or conflict where the checks of orders 048a-2 and 048a-5
      apply, unavailable where order 048a-5c's fixed-port check applies, and otherwise checks the
      account first" becomes "answers invalid or conflict where the checks of orders 048a-2, 048a-5
      and 209 apply, unavailable where order 048a-5c's fixed-port check applies, and otherwise
      checks the account first"; the row's "Where the checks of orders 048a-2 and 048a-5 apply, a
      stale `expectedId`" does not change.
    - In "Consumer server (order 022)", "where the checks of orders 048a-2 and 048a-5 apply,
      unavailable where order 048a-5c's fixed-port check applies, and otherwise runs as far as its
      first account check" becomes "where the checks of orders 048a-2, 048a-5 and 209 apply,
      unavailable where order 048a-5c's fixed-port check applies, and otherwise runs as far as its
      first account check".
    - In "What the first build is", "where the checks of orders 048a-2 and 048a-5 apply,
      unavailable where order 048a-5c's fixed-port check applies, and otherwise signs an instance
      in" becomes "where the checks of orders 048a-2, 048a-5 and 209 apply, unavailable where order
      048a-5c's fixed-port check applies, and otherwise signs an instance in".
    - In DL18's row, "while a failed or unknown check ends it failed and completes nothing
      (PND-011)" becomes "while a check that ends with a reason, such as unavailable under
      IFC-037, ends it failed and completes nothing (PND-011)".
    - In DL18's row, "Until order 048a-5e builds the relay, a check that finds no account or
      another ends `sign-in.v1` failed" becomes "Until order 048a-5e builds the relay, a check that
      finds no account or another, or that ends failed or unknown and so counts as no account,
      clearing no condition (order 209), ends `sign-in.v1` failed".
    - In DL19's row, after "and the check that `sign-in.v1` sends first sets it `True` the same
      way (order 048a-5b).", before "No other kind is held", add "A check that ends failed or unknown counts as no account (order 209): it raises
      the sign-in action unless one is unanswered, undoes nothing, clears no condition and lets
      setup go on to its setup hook, counting as succeeded in the setup's outcome (TPL-012,
      OPS-022); one that ends with a reason, such as unavailable under IFC-037, stops the setup."
    - The section that begins `## Ahead of the engine: ruling 86 (order 207)`, from that heading to
      the end of the file, is replaced whole by:

      ```text
      ## Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)

      - **Keys per hook** (ruling 86, point 2; PKG-020, IFC-037; order 209): an implementation item marked `plain` and reached by `command` or `package` may list under `keyed` hooks of its `implements`, each a key of its `tools` and one of the five core hooks. Registration and installation refuse any other selection under PKG-020, one that is not a core hook (`requests-keyed-not-a-hook`) as one that is not in `implements` (`requests-keyed-not-in-implements`), and the schema refuses `keyed` on an item that is not plain or is reached by `remote` (`PKG-020-manifest-keyed-not-plain.yaml`). The core reads `plain` per call on the hook path: a call for a hook that `keyed` lists carries Conduit's keys, is named from `tools`, declares Conduit's extension at `v0`, holds a setup or retire hook's idempotency key (IFC-006, OPS-028), and has its result read as a keyed implementation's (IFC-009, IFC-008, IFC-019, IFC-026, IFC-047, OPS-006, OPS-007). Of these, only IFC-009's reading has a case here (`requests-keyed-hook-dispatch`, step 13); a keyed setup or retire hook's idempotency key and its re-dispatch under OPS-028, and IFC-008, IFC-019, IFC-026 and IFC-047 on a keyed hook, follow from the invocation's `plain` and have no case: the unit test `keyed_hooks_read_plain_per_call` shows the key. Every other call to the item, every request among them, stays plain (IFC-007, IFC-010). A URL-mode `input_required` result from a part of `sign-in.v1` is read by IFC-010, keyed or plain alike, and never by the terminal checks of IFC-009, OPS-006 and OPS-007: the core ends such an account check unknown, whatever outcome it states, and it counts as no account (`requests-keyed-hook-dispatch`). The core also raises the instance's sign-in action for it, as for a URL elicitation outside `sign-in.v1` (IFC-016); whether PND-005 asks that of a part of `sign-in.v1` is open (order 209's Record). Before each such call the core sends `server/discover` to the implementation: IFC-037 asks for its result before the first call to each run and lets the engine read it again at any time, and this build reads it before every call for a keyed hook, keeping no result from one call to the next. Where the result does not list the extension at `v0`, or discovery ends in an error or no answer, the hook is not sent and reports unavailable under IFC-037, its `request-ended` record naming the missing extension, while the item's other calls are sent as before (`requests-keyed-hook-dispatch`). A hook whose discovery is in flight when the engine restarts ends unknown under IMP-007, as any call in progress, though it was never sent. When its setup or retirement is cancelled, it ends unknown under OPS-027. When it is a part of a `sign-in.v1` that is cancelled, the cancel leaves it in progress (order 048a-5b), and a discovery result that comes after sends nothing: one that lists the extension ends it unknown under OPS-027, and any other ends it unavailable under IFC-037, as above (order 209). Only these calls read discovery or declare the extension: a call to an implementation not marked `plain` carries Conduit's keys without either, as before order 209. No `sign-in-hook.v1` part runs before order 048a-5d, so the case keys `account-hook.v1`.
      - **A login name aimed at a plain hook** (ruling 86, point 3 (c); PND-010; order 209): after the refusal of a name the Type does not state, a `signInLogin` naming a stated login reports invalid before any part where the instance's `sign-in-hook.v1` is a plain hook, its item marked `plain` and not listing it under `keyed`, with a diagnostic naming PND-010 and the login (`requests-sign-in-login-plain-hook`, `requests-keyed-hook-dispatch`). The `access-decision` record reports whether access was permitted, and the refusal's reason and deciding rule are in `request-ended`.
      - **A failed or unknown account check counts as no account** (ruling 86, point 3 (d); PND-010, PND-011, TPL-012, OPS-022; order 209): in `sign-in.v1` it takes the path of no account, clearing no condition and raising nothing, and so, until order 048a-5e, ends `sign-in.v1` failed under PND-010's interim ending (`requests-sign-in-account-first`, `requests-keyed-hook-dispatch`); in setup it raises the sign-in action unless one is unanswered, undoes nothing, clears no condition and lets the setup go on to its setup hook, the check counting as succeeded in the setup's outcome (`requests-sign-in-setup-check-failed`, `requests-sign-in-setup-check-failed-hook`). The second of those also shows a failed check in `sign-in.v1` leaving a `False` `SignInUndone` as it is. A check that ends with a reason, such as unavailable under IFC-037, is neither: it ends `sign-in.v1` failed under PND-011 and stops a setup, as before. Polling, where a failed or unknown poll counts as no account yet, is order 048a-5f's.

      IFC-010's relay of a URL request from the `sign-in-hook.v1` part, plain or keyed, has no case here: this build runs no `sign-in-hook.v1` part; orders 048a-5d and 048a-5e build the hook part and the relay, whose cases show it.
      ```

    Nothing else in the file changes. After the other decisions, run
    `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
    `promises`, to the expected failures and to a promise ID in the core's sources. Report the
    rows of IFC-037, PKG-020, IFC-007, PND-005, PND-010, PND-011, TPL-012 and OPS-022 and the
    totals; IFC-037's row reads `core` in its sources column and `case passes`, and each of the
    other seven counts no expected failure (Done when).

11. **The declaration.** `docs/work-orders/209-declared.yaml` holds four shapes: one `added` for
    each of the three new cases, and one `expectation` for step 24's `ruleId` in
    `requests-sign-in-account-first`. The tool counts
    a `ruleId` as an expectation, not a flip, since its flips are reasons, outcomes, `accepted`
    and `tool`. The coordinator wrote it with this order; check that it matches and change it only
    to make it so.

12. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
    record changes; order 207's four cases, their manifests and the fixtures do not change. No
    package lists anything under `keyed` (ruling 86, point 1). No keyed `content` copy is built,
    IFC-025 being unchanged. The request path's `plain` does not change. No `sign-in-hook.v1` part
    runs (048a-5d), nothing is relayed (048a-5e), and polling (048a-5f) is not built. Discovery and
    the extension's declaration are not built for an implementation not marked `plain`. The shell
    changes only by decision 7's test. Report as findings, for the coordinator:
    - whether any path other than a cancelled `sign-in.v1` reaches decision 3's Listed arm with
      an ended parent;
    - that a keyed repeat of `sign-in.v1` still bypasses the login checks (order 048a-5's Record,
      R4, as 048a-5b's Record leaves it), so it bypasses (c) too; report whether any path reaches it;
    - that IFC-037's "each run" is met by reading discovery before every keyed hook call, and
      whether any path sends a keyed hook's call without a discovery result read for it;
    - that the shell's runner sends `server/discover` on a session it initialized as rmcp 3.9.0's
      client does, and that a server which does not answer the method makes the hook unavailable,
      as IFC-037 asks; no test here runs a real server;
    - that `complete`'s URL arm raises the instance's sign-in action for a part of `sign-in.v1`
      whose call returned a URL-mode `input_required` result, an account check among them (step
      24 of the first case), through `finish_login_lapse`, as IFC-016 has a URL elicitation outside
      `sign-in.v1` do and under that rule's name, while PND-005's `when` names URL-mode elicitation
      "outside sign-in.v1" and IFC-010 has such a part end unknown and follow PND-010 and PND-011;
      report whether a keyed part, a plain part, or both, reach it, and change nothing of it;
    - any place where PKG-020, IFC-010, IFC-037, PND-010, PND-011, TPL-012 or OPS-022 left you a
      choice this order does not settle.

## Scope

Change only `engine/crates/core/src/packages.rs` (decision 1), `requests.rs` (decisions 2 and 3,
and decision 7's test), `sign_in.rs` (decisions 4 and 5, and decision 7's test), `lifecycle.rs`
(decision 6), `explain.rs` (decision 6's cause and its test), `engine/crates/shell/src/runner.rs` (decision 7's test alone),
`engine/CONFORMANCE.md` (seven places), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/requests/index.yaml`, the new directories
`conformance/conventions/requests/keyed-hook-dispatch/`,
`conformance/conventions/requests/keyed-not-in-implements/` and
`conformance/conventions/requests/sign-in-setup-check-failed-hook/`,
`conformance/conventions/requests/sign-in-account-first/case.yaml` (decision 9's one value) and
`conformance/expected-failures.yaml` (three entries removed). `docs/work-orders/209-declared.yaml`
comes with this order; change it only as decision 11 says. `conformance/manifest.yaml` holds the
new entries only until the pieces tool moves them, and ends unchanged. Where `pending.rs` or
`explain.rs` must change for decision 6's raise or its cause, change only what it needs and say
why. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, the rest of `engine/crates/shell/`, the
experiments, the case schema, `conformance/internal-cases.yaml`, any other case or fixture,
`docs/implementation/` or `docs/proposals/`: report any defect as a finding.
`experiments/011-short-form-spike/` changes only if its tests need it, since it depends on the
core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- requests::tests
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- lifecycle
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- settle_cause_skips_a_counted_account_check
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell discover_call_parses_as_a_client_request
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-keyed-hook-dispatch
cargo run --release -p conduit-conformance -- .. requests-keyed-not-in-implements
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-check-failed-hook
cargo run --release -p conduit-conformance -- .. requests-keyed-hook-manifest
cargo run --release -p conduit-conformance -- .. requests-keyed-not-a-hook
cargo run --release -p conduit-conformance -- .. requests-sign-in-login-plain-hook
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-check-failed
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports one more test passed than the start's run, `sign_in_login_refused_for_a_plain_hook`,
and the fourth one more, `keyed_hooks_read_plain_per_call`, each 0 failed; the fifth passes as at
the start; the sixth and seventh each run 1 test, which passes. Report each count. The pieces tool prints
`Nothing to move.` The next eight runner commands each pass with no expected failure: seven end
`1 cases passed`, and `requests-sign-in-setup-check-failed`, whose filter is a prefix of the third
command's case too, ends `2 cases passed`. `requests-sign-in-` gives the start's passed count plus three
(requests-sign-in-login-plain-hook, requests-sign-in-setup-check-failed and
requests-sign-in-setup-check-failed-hook) and no expected failure, `11 cases passed` from
`0b5b1eba`'s `8 cases passed; 2 expected failures`; `requests-` gives the start's passed count
plus six (the three new cases and the three that were expected failures), and the start's expected
failures less those three, each still failing, `78 cases passed; 2 expected failures` from
`0b5b1eba`'s `72 cases passed; 5 expected failures`; report each count and each expected failure. If any other case, or any
shell, CLI or experiment test, fails, stop and report it: do not change it. The threads are 4
because at 24 the runner ran out of memory (order 071's Record).

Next, show that each behavior is what its case checks. From `engine/`, first keep copies:

```bash
cp crates/core/src/requests.rs target/requests.rs.fixed
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
cp crates/core/src/lifecycle.rs target/lifecycle.rs.fixed
cp crates/core/src/packages.rs target/packages.rs.fixed
```

Then make each change below by hand, alone, run its command, restore its file with `cp` from its
copy, and run the command again. Each first run must fail as said, printing
`FAIL 0 cases passed; 1 failed`, and each run after the restore end `1 cases passed`; the command
`requests-sign-in-setup-check-failed` runs two cases, so its row prints
`FAIL 0 cases passed; 2 failed`, both failing at step 6, and then `2 cases passed`. Restore with
`cp`, not `mv`: a moved file keeps its older modification time, so cargo would not rebuild the
crate. Git ignores `target/`.

| Change, by hand | Command (`cargo run --release -p conduit-conformance -- ..` and) | Must fail at |
| --- | --- | --- |
| In `requests.rs`, make `keyed_hook` return `false` | `requests-keyed-hook-dispatch` | step 5, a `tools/call` where DISCOVER is expected |
| In `requests.rs`'s discovery arm, replace the test that the result lists the extension with `true` | `requests-keyed-hook-dispatch` | step 15, `missing ordered record` of the `request-ended` naming IFC-037 and the missing extension |
| In `sign_in.rs`, delete the statement of decision 4 that gives (c)'s rejection | `requests-sign-in-login-plain-hook` | step 3, `missing ordered record` of the `request-ended` naming `PND-010` |
| In `sign_in.rs`'s `part_answered`, send `failed` and `unknown` to the `PND-011` ending again | `requests-sign-in-account-first` | step 24, `PND-011` where `PND-010` is expected |
| In `lifecycle.rs`, delete decision 6's branch, so that a failed check settles the setup | `requests-sign-in-setup-check-failed` | step 6 of each case: in `requests-sign-in-setup-check-failed`, `unknown` where `succeeded` is expected; in `requests-sign-in-setup-check-failed-hook`, no setup-hook intent where one is expected (`/external_intents: expected 1 items, found 0`) |
| In `packages.rs`, delete decision 1's loop over `keyed` | `requests-keyed-not-a-hook` | step 2, `missing ordered record` of the `access-decision` with reason invalid and `dispatched` false: the registration is accepted |
| In `requests.rs`'s discovery arm, delete the test of the hook's `setup_parent` | `requests-keyed-hook-dispatch` | step 21, `missing ordered record` of `$check3`'s `invocation-ended` with outcome unknown: the hook is sent |
| In `requests.rs`'s `complete`, add `plain &&` to the URL arm's guard | `requests-keyed-hook-dispatch` | step 24, `missing ordered record` of `$check4`'s `invocation-ended` with outcome unknown: the keyed check is read under IFC-009 as succeeded |
| In `packages.rs`, delete decision 1's test that the selection is in `implements` | `requests-keyed-not-in-implements` | step 2, `missing ordered record` of the `access-decision` with reason invalid and `dispatched` false: the registration is accepted |
| In `lifecycle.rs`'s decision 6 branch, call `settle` in place of `advance` | `requests-sign-in-setup-check-failed-hook` | step 6, no intent where the setup hook's is expected: the setup succeeds without its hook |
| In `sign_in.rs`'s `part_answered`, let the clear above the arms apply to a check that ended failed too | `requests-sign-in-setup-check-failed-hook` | step 16, `SignInUndone`'s reason `AccountChecked` where `SignOutFailed` is expected |

The runner prints no step for a `missing ordered record`; the step named is the one whose
expectation it misses.

Report the line at which each failing run failed.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/209-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md tools packages experiments docs/implementation docs/proposals conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/requests/sign-in-setup-account conformance/conventions/requests/sign-in-refusals-before-parts conformance/conventions/requests/keyed-hook-manifest conformance/conventions/requests/keyed-not-a-hook conformance/conventions/requests/sign-in-login-plain-hook conformance/conventions/requests/sign-in-setup-check-failed engine/crates/shell/src/server.rs engine/crates/shell/src/lib.rs
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/209-declared.yaml: 4 shapes.`,
then `Vector delta: 4 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`: the three added
cases and step 24's `ruleId`. `promise_standing.py --check`, `check_model_account.py` and
`check_roadmap.py` each end `Errors: 0`. No promise's text changes, so no row of the
model-checking account goes stale; where `check_model_account.py` reports one stale, stop and
report it. The unit tests end `OK`; report the count. The `git status` command prints nothing.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so
the local gate runs it for this change. From `experiments/011-short-form-spike/`:

```bash
RUST_TEST_THREADS=4 cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`.

Each of these prints 1. The `tr` checks read the file with its line breaks as spaces:

```bash
grep -c 'pub(crate) fn keyed_hook(item: &Value, operation: &str) -> bool' engine/crates/core/src/requests.rs
grep -c 'fn keyed_hooks_read_plain_per_call' engine/crates/core/src/requests.rs
grep -c 'fn sign_in_login_refused_for_a_plain_hook' engine/crates/core/src/sign_in.rs
grep -c 'fn settle_cause_skips_a_counted_account_check' engine/crates/core/src/explain.rs
grep -c 'fn discover_call_parses_as_a_client_request' engine/crates/shell/src/runner.rs
grep -rl '"read-implementation-discovery"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"hook-without-extension"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'which is not one of the core hooks setup-hook.v1, retire-hook.v1, sign-in-hook.v1, account-hook.v1 and sign-out-hook.v1' engine/crates/core/src/packages.rs
grep -c 'which counts as no account (PND-010, PND-011); sign-in.v1 would go on' engine/crates/core/src/sign_in.rs
grep -c '"id": "requests-keyed-hook-dispatch"' conformance/conventions/requests/index.yaml
grep -c '"id": "requests-keyed-not-in-implements"' conformance/conventions/requests/index.yaml
grep -c '"id": "requests-sign-in-setup-check-failed-hook"' conformance/conventions/requests/index.yaml
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/keyed-hook-dispatch/case.yaml"))["steps"]; print(int(len(s) == 24 and s[23]["input"]["completion"]["result"]["resultType"] == "input_required"))'
grep -c '^## Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)$' engine/CONFORMANCE.md
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'while a check that ends with a reason, such as unavailable under IFC-037, ends it failed and completes nothing (PND-011)'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'or that ends failed or unknown and so counts as no account, clearing no condition (order 209), ends .sign-in.v1. failed'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'A check that ends failed or unknown counts as no account (order 209): it raises the sign-in action unless one is unanswered'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'answers invalid or conflict where the checks of orders 048a-2, 048a-5 and 209 apply, unavailable where order 048a-5c.s fixed-port check applies, and otherwise checks the account first'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'where the checks of orders 048a-2, 048a-5 and 209 apply, unavailable where order 048a-5c.s fixed-port check applies, and otherwise runs as far as its first account check'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'where the checks of orders 048a-2, 048a-5 and 209 apply, unavailable where order 048a-5c.s fixed-port check applies, and otherwise signs an instance in'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'one that is not a core hook (.requests-keyed-not-a-hook.) as one that is not in .implements. (.requests-keyed-not-in-implements.)'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and so, until order 048a-5e, ends .sign-in.v1. failed under PND-010.s interim ending (.requests-sign-in-account-first., .requests-keyed-hook-dispatch.)'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'is read by IFC-010, keyed or plain alike, and never by the terminal checks of IFC-009, OPS-006 and OPS-007'
grep -cE '^\| IFC-037 \| interface \| must \| [1-9][0-9]* \| 0 \| [a-z]+ \| core \| case passes \|$' engine/STANDING.md
```

This prints 2 (decision 4's cause and decision 7's test):

```bash
grep -c 'is a plain hook, its implementation item marked plain and not listing sign-in-hook.v1 under keyed (PKG-020)' engine/crates/core/src/sign_in.rs
```

This prints `$check3 PND-010 32`:

```bash
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-account-first/case.yaml"))["steps"]; print(s[23]["input"]["invocation"], s[23]["expect"]["reply"]["structuredContent"]["diagnostics"][0]["ruleId"], len(s))'
```

This prints `7`, each of the seven rows counting no expected failure:

```bash
grep -cE '^\| (PKG-020|IFC-007|PND-005|PND-010|PND-011|TPL-012|OPS-022) \| [a-z]+ \| [a-z ]+ \| [0-9]+ \| 0 \| ' engine/STANDING.md
```

Each of these prints 0:

```bash
grep -c 'requests-keyed-not-a-hook\|requests-sign-in-login-plain-hook\|requests-sign-in-setup-check-failed' conformance/expected-failures.yaml
grep -c '^## Ahead of the engine: ruling 86 (order 207)$' engine/CONFORMANCE.md
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'while a failed or unknown check ends it failed and completes nothing (PND-011)'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Until order 048a-5e builds the relay, a check that finds no account or another ends .sign-in.v1. failed'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply, unavailable where' | wc -l | tr -d ' '
git diff HEAD --stat -- spec IN_FORCE.md packages | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 10 names with the totals, the signatures of `keyed_hook` and of
any function you added, the lines where `dispatch_hook` defers its call and where `complete` reads
discovery, the line in `check_items` that checks `keyed`, and the findings of decision 12.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `209-declared.yaml` through `--declared-changed`, and the merge does not wait for
it. One Codex round, a worker and a fresh Codex verifier
([ruling 87](../proposals/ruling-87/README.md), point 1), or, if the coordinator records the order as paired, the round 2 that A77's registration prescribes as well (ruling 87, point 3), and a fresh read-only Opus reader, which
also judges any must of the verifier (ruling 87, point 2;
[ruling 79](../proposals/ruling-79/README.md), point 2), read the result before it merges. The
order is off the sign-in chain, so whether it is paired under A77's amendment of 2026-11-19 is the coordinator's to record. The admin
writes this order's Record at the foot of this file when it merges.

## What follows

For the coordinator, not the worker:

- **Order 048a-5d** dispatches `sign-in-hook.v1` after a check that ended failed or unknown too, in
  place of this order's interim ending for it, and its hook call reaches a keyed sign-in hook of a
  plain item through `dispatch_hook`'s discovery: its part's first call, which `retry_intent`
  copies, is the `tools/call` intent, not the discovery intent. Its arm for the hook part's
  `input_required` answer goes ahead of the URL arm that decision 2 keeps, and takes a plain and a
  keyed answer alike (IFC-010). Its Done-when covers a plain hook's URL request taken by the
  sign-in arm (ruling 86, point 4). Since this refresh it also changes step 24 of
  `requests-keyed-hook-dispatch`, whose unknown check then goes on to the plain sign-in hook, and
  step 15 of `requests-sign-in-setup-check-failed-hook`, whose failed check then goes on to a keyed
  sign-in hook; and it follows the section's last sentence as this order words it ("from the
  `sign-in-hook.v1` part, plain or keyed").
- **The URL arm's sign-in action for a part of `sign-in.v1`** (decision 12): whether PND-005 asks
  for it, and if not which order removes it, is the coordinator's to settle; IFC-016's task path
  (order 207's Record, R207-9) is beside it.
- **Order 048a-5e** reads `accepts_return` from the hook part's per-call `plain`, which this order
  sets, and removes this order's interim cause with 048a-5b's.
- **Order 048a-5f** counts a failed or unknown poll as no account yet, with no clear.
- **The later keyed sign-in release**: the Agent packages list `sign-in-hook.v1` under `keyed` and
  answer `server/discover` listing the extension at `v0`; NAM-024's reading of that change is the
  owner's.
- **Cases for the keyed hook's other readings**, outstanding: a keyed setup or retire hook given
  its idempotency key and re-dispatched under OPS-028, and its call and result under IFC-008,
  IFC-019, IFC-026 and IFC-047, which order 207's "What follows" asked of this order with cases of its own.
- **IFC-037 for implementations not marked `plain`**: discovery and the extension's declaration
  for every keyed call, an order of its own if the coordinator puts it on 0.2.0's path.

## Record

Not yet run.
