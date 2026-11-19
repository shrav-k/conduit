# Work order 048a5c (E7a-5c): a sign-in that may share a fixed port with one in progress reports unavailable

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

**Where it sits.** Order 048a-5 (its file `048a5-build-sign-in-refusals-before-parts.md`) records
the coordinator's cut of 048a-5 into eight parts, each after the one before, since an order holds
one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). This is the third:
"048a-5c, PND-010's fixed-port refusal". Order 048a-5 builds `sign-in.v1`'s refusals before any part
(the missing URL elicitation capability, a return on a first request and an unknown `signInLogin`);
order 048a-5b binds who may ask at the access tier and dispatches `sign-in.v1`, its account check
first, with C11's stub removed. Later orders build the rest: 048a-5d the `sign-in-hook.v1` part and
IFC-049's URL match, 048a-5e the relayed output, 048a-5f to 048a-5h polling and its endings. This
order builds none of them. Nor does it build ruling 86's points 3 (c) and (d), which order 209
builds off the chain: the refusal before any part of a `signInLogin` aimed at a sign-in hook whose
operation is not keyed, and a failed account check before the hook going on to it. Neither
contradicts this order: (c), whatever reason order 207 gives it, refuses none of this order's case's
logins: the case aims its named logins at an implementation item that is not marked `plain`, which
carries Conduit's keys, so (c) refuses none of them; and the case completes no account check that
fails.

**What it builds.** One behavior: PND-010's refusal of a sign-in that may start a login on a fixed
port that another sign-in in progress may also start. PND-010's `then`: "Before dispatching any
part, the request reports unavailable where an in-progress sign-in.v1 invocation may start a login
stating a fixed port that a login this request may start also states. The diagnostic names PND-010
and the port, and names the other invocation's pending action only where the caller may see it
(PND-002). ... A request may start its asked-for login or, without one, any of its Type's logins.
These decisions follow OPS-023's order." Deciding "may start" for the invocation in progress needs
its asked-for login, which nothing keeps once the request has ended, so this order keeps it on the
invocation. A login states a fixed port where it is a redirect login whose `redirect.ports` is a
list (`sign-in-login.v1`, `spec/schemas/contracts.yaml`); `ports: any`, and a device, page or
service login, state none. No promise changes: the texts state all of it, and this order builds
it.

**Where it falls among the refusals.** The coordinator's plan for the chain fixes the order, for a
first `sign-in.v1` request after the access decision, which since order 048a-5b's decision 0 lets
through only root on its own behalf through the instance's owning Fleet or Pool, or an Interface
on its own behalf through an Agent's owning Fleet in its scope: LIF-031's invalid for an instance
named by a resolved `{kind, id}` (048a-5b's Record, R3); 048a-2's invalid (no `signIn`, or an
account that is not a string); 048a-5's three invalids (the capability, the return, the login);
048a-2's conflicts (a stale `expectedId` under LIF-037, which its Record's R3 adds, then the
instance's own `sign-in.v1`, setup or retirement in progress); then this order's unavailable; then
dispatch. So a second `sign-in.v1` of one instance reports conflict even where it also shares a
fixed port, and this refusal reaches only a sign-in of another instance. A retry of `sign-in.v1`
reaches the core only as the shell's `respond` input (order 048b-relay1), never as an `invoke`, so
this refusal decides only a first request. A keyed request that the engine dispatches again
(`redispatch` in `invoke_inner`) passes the same preflight and is decided alike; a keyed repeat
that returns a kept outcome dispatches nothing and is not decided (OPS-012).

**What orders 048a-2 to 048a-5b bring, which this order assumes.** Orders 048a-2 (#364), 048a-3
(#367), 048a-4 (#371), 048a-5 (#382) and 048a-5b (#387, its file
`048a5b-build-sign-in-account-first.md`) have merged, each with its Record, and so has 048a-6
(#386). This order starts from `main` at `95b9fc3b` or later. It assumes, as their decisions, and
the amendments their Records accept, state them:

- From 048a-2's decision 5: `engine/crates/core/src/sign_in.rs` and its function that gives an
  instance's expected account (no `signIn`, an account that is not a string, or the pointer and the
  string), called "the account function" below, which looks up no Type for an instance with no
  `held`, a template, or an object that is not a Resource or Agent instance. It merged as
  `expected_account(instance, context) -> Result<ExpectedAccount>`, giving `NoSignIn`,
  `NotString { pointer }` or `Set { pointer, account }`, the last being "the pointer and a string".
- From 048a-2's decision 6, as 048a-5 and 048a-5b leave it: the preflight of `sign-in.v1`,
  `sign_in::preflight(state, route, request: &AccessRequest, time: &str, context) -> Result<Option<Rejection>>`,
  called with `?` in `invoke_inner` right after `records::preflight`'s block, giving its rejection
  through `reject_preflight` to `reject_diagnostics`; and its helper, called "the decision-6 helper"
  below, `refusal(state, route, instance, time, expected, request, names, context)`, which takes
  the account function's answer by value and decides 048a-2's invalid, then (048a-5)
  `request_member_refusal`'s three invalids, then 048a-2's two conflicts: a stale `expectedId`
  under LIF-037 (its Record's R3), and the conflict that reads an in-progress invocation's `object`
  and `operation`. The preflight finds the stated login names, through `stated_login_names`, called
  "the names function" below, only where the account function gives `Set`. Before the helper, it
  fails with an error, never dispatching, for a caller or an instance that access does not permit
  (048a-5b's decision 0, part 5, a second line behind the access tier), and rejects an instance
  named by a resolved `{kind, id}` under LIF-031 (048a-5b's Record, R3). In the in-progress
  conflict, the rule that names the instance's sign-in action only where it has no answer at the
  step's time and its view at the request's caller (047b's decision 2) is not none, called "the
  visible-action rule" below; 048a-2 wrote it inline in `refusal`, through
  `pending::unanswered_sign_in` and `pending::raised_view`, which takes the `Context` and returns a
  `Result`. Its unit test is `sign_in_refusals_before_dispatch`.
- From 048a-5b's decision 14: every diagnostic of the preflight but LIF-037's is written by
  `rejection(reason, rule, tool, instance, cause)` in `sign_in.rs`, as
  `{tool} reports {reason} under {rule} for {kind}/{name}: {cause}`, naming the instance the
  request named.
- From 048a-2's decision 10: the case `requests-sign-in-lapse-and-conflict`, its
  `manifest.json`, `documents.json` and `fixture.json` (the Type `signed-agent.v1`, whose logins
  are `browser`, a redirect login stating the ports 8400 and 8401, and `device`; the Agent
  template `signed-template`, account `primary-account`; the Fleet `group`; the Agents `signed` and
  `unset`; one implementation item, `signed-server`, not marked `plain`), its definitions ROOT,
  `META/x`, "Root asks T with A", "a completion" and REFUSED(R, RULE), and its steps 1 to 3 and 7
  to 9 (of 32) as 048a-4's decision 9 leaves them: first start, registration, the setup of
  `signed` (`$setup1`), its two run reports, the second dispatching the account check
  (`$account0`), and that check's completion with the expected account, ending the setup
  succeeded.
- From 048a-4's decision 8: the definition ACCOUNT(X), an `account-hook.v1` result reporting the
  account X.
- From 048a-5's decisions 1 and 5: `AccessRequest`'s member `sign_in_login`; and the definitions
  S, URL, LOGIN(X), MISSING and "Root asks T with A and M" of its case, MISSING being the -32021
  refusal of IFC-049 with reason `invalid`.
- From 048a-5b's decision 1: `invoke_inner` builds an accepted `sign-in.v1` request's invocation,
  as it builds any operation's, and pushes it into `state.invocations`; then
  `crate::sign_in::begin(t, a, c, i)?`, which is not given the request, sets `object` to the
  instance's ID on the invocation already pushed. That invocation stays in state `in-progress`
  while its account check is pending, with `operation` the core's `sign-in.v1`, as 048a-2's
  conflict reads them, and the reply is `{"pending": true, ...}`, whose `hook` names the account
  check, as 048a-5b's SIGN(N) captures it at `/reply/hook`; it keeps nothing of `signInLogin`. A
  completion of that check with ACCOUNT of the expected account ends the `sign-in.v1` succeeded
  (048a-5b's decision 3), so that it is no longer in progress.

Where any of them merged otherwise than this says, apply the nearest sound reading of each decision
below to what merged, and say so. Where a Record names a function, use that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`95b9fc3b`, after orders 048a-5b and 048a-6; find each place by the text quoted with it.

- `invoke_inner` (`engine/crates/core/src/requests.rs:917`) runs `records::preflight`'s block
  under `previous.is_none() || redispatch` and gives a rejection through `reject_preflight`
  (`requests.rs:618`), which passes `Rejection`'s reason, rule and diagnostics to
  `reject_diagnostics`; right after it, under the same test, it calls the sign-in preflight
  (`requests.rs:1219`, `sign_in::preflight(&t.state, &route, request, &a.evidence.time, c)?`).
  Before the key lookup it calls the same preflight once more, at `requests.rs:1117`, only for a
  `sign-in.v1` whose instance is a resolved `{kind, id}` or that declares no URL elicitation (the
  comment `// LIF-031 and IFC-049 apply to each sign-in request, including keyed repeats.`); an
  invalid answers there before any conflict, so that call never reaches this order's check.
- `crate::explain::Rejection` (`explain.rs:8`) holds `reason`, `rule` and `diagnostics`.
- A request's invocation is a JSON value (`requests.rs:1580`, the line
  `let mut invocation = json!({"id":id,"key":key,...`); `State`'s `invocations` keeps every
  invocation, ended or not, in the order pushed. The chain that hands it on calls
  `crate::sign_in::begin(t, a, c, i)?;` for `sign-in.v1` (`requests.rs:1804`).

**When this order starts.** Order 048a-5b has merged to `main` (#387). Before you change anything,
check from the repository root that you find what it and the orders before it give, and stop,
change nothing and report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/048a5b-build-sign-in-account-first.md
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
grep -c 'fn request_member_refusal' engine/crates/core/src/sign_in.rs
grep -c 'fn sign_in_refusals_before_dispatch' engine/crates/core/src/sign_in.rs
grep -c 'rename = "signInLogin"' engine/crates/core/src/access.rs
grep -c 'pub(crate) fn begin(' engine/crates/core/src/sign_in.rs
grep -c 'reports {reason} under {rule} for {kind}/{name}: {cause}' engine/crates/core/src/sign_in.rs
test -f conformance/conventions/requests/sign-in-refusals-before-parts/case.yaml; echo "case $?"
python3 -c "import json; d = json.load(open('conformance/conventions/requests/sign-in-lapse-and-conflict/documents.json')); t = [x for x in d if x['kind'] == 'Type' and x['metadata']['name'] == 'signed-agent.v1'][0]; print([[l['name'], l.get('redirect', {}).get('ports')] for l in t['spec']['signIn']['logins']], sorted(x['metadata']['name'] for x in d if x['kind'] == 'Agent' and 'from' in x))"
python3 -c "import json; s = json.load(open('conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml'))['steps']; print(s[0]['input']['kind'], s[1]['input']['kind'], s[2]['input']['request']['tool'], s[6]['input']['kind'], s[7]['input']['kind'], s[8]['input']['kind'], json.dumps(s[7]['capture']), len(s))"
python3 -c "import json; m = json.load(open('conformance/conventions/requests/sign-in-lapse-and-conflict/manifest.json')); print({k: v.get('plain') for k, v in m['implementations'].items()})"
grep -rl '"asked_login"' engine/crates/core/src | wc -l | tr -d ' '
grep -c -i 'fixed port' engine/crates/core/src/sign_in.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'After order 048a-2.s invalid and before its conflict, order 048a-5 decides three more invalids'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply, and otherwise' | wc -l | tr -d ' '
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c '048a-5c'
awk -F'|' '$2 == " PND-010 " || $2 == " OPS-023 " {print $2 $5}' engine/STANDING.md
```

They print `1`, `0`, `1`, `1`, `1`, `1`, `1`, `case 0`,
`[['browser', [8400, 8401]], ['device', None]] ['signed', 'unset']`,
`first-start register-project group.set-up.v1 setup-report setup-report complete {"account0": "/reply/hook"} 32`,
`{'signed-server': None}`, `0`, `0`, `1`, `3` and `0`, and the last prints two lines, each a
promise ID and its number of cases, on `95b9fc3b` `OPS-023  25` and `PND-010  3`: report both
numbers. The first is 048a-5b's Record, the second its removal of the stub. The third and fifth
are 048a-5's, the fourth 048a-2's, the sixth 048a-5b's `begin` and the seventh its diagnostics'
form (decision 14); the eighth is 048a-5's case. The ninth and tenth are the fixture and the steps
this order's case reuses, and the eleventh says that its one implementation item is not marked
`plain`, so it carries Conduit's keys and ruling 86's point 3 (c) refuses none of its named
logins. The twelfth and thirteenth say that nothing of this order is built yet, the fourteenth is
the sentence decision 6 adds after, the fifteenth the three places decision 6 changes, as 048a-5b
leaves them, and the sixteenth that the file names this order nowhere yet.

After the start checks, from `engine/`, one at a time, run
`cargo test --release -p conduit-core --lib -- sign_in` and
`cargo run --release -p conduit-conformance -- .. requests-`, and keep each last line for Done
when. On `95b9fc3b` they end `16 passed; 0 failed` (with `115 filtered out`) and
`70 cases passed; 2 expected failures`.

Orders that land beside this one (209, 050 and others): whichever lands second keeps both changes
and writes `engine/STANDING.md` again with the tool.

**Who decided what.** PND-010, PND-002 and OPS-023 decide that the refusal comes before any part,
its reason, that its diagnostic names PND-010, the port and only a visible pending action, what a
request may start, and its place after conflict. `sign-in-login.v1` and CTR-013 decide which logins
state ports. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the cut that puts this behavior here (order 048a5's
"Where it sits"); the order of the refusals above, from the coordinator's plan for the chain;
reading "a fixed port" as an integer in a redirect login's `redirect.ports` list, compared whatever
the login's `redirect.host`, so that `ports: any` and the other returns state none; comparing every
other instance's `sign-in.v1` in progress, whatever its Fleet or node, since PND-010 names neither;
keeping the asked-for login's name on the invocation as `asked_login`, and reading a kept name that
its Type no longer states as starting no login; naming the first such invocation in
`state.invocations` that shares a port, and every port the two share, in ascending order; reading
"the other invocation's pending action" as its instance's sign-in action under the visible-action
rule, named in the diagnostic's text alone, with no `_meta` key `pendingAction`; deciding it in the
preflight right after the decision-6 helper, with no Type lookup where no other `sign-in.v1` is in
progress; the diagnostic's words, written through `rejection` so that it names the request's own
instance as 048a-5b's decision 14 has every preflight diagnostic do; the function names; the unit
test; the case, its documents, route, IDs and times; and the sentence in `engine/CONFORMANCE.md`.
Apply them as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-002 and PND-010 (`spec/promises/pending.yaml`); OPS-023
   (`spec/conventions/operations.yaml`); CTR-013 (`spec/promises/contracts.yaml`), its sentence on
   overlapping fixed ports; `sign-in-login.v1` (`spec/schemas/contracts.yaml`); `sign-in.v1`'s
   description (`spec/core/operations.yaml`).
2. [Order 048a2](048a2-build-sign-in-lapse-and-conflict.md), its decisions 5, 6 and 10 and its
   Record; [order 048a4](048a4-build-sign-in-setup-account.md), its decisions 8 and 9;
   [order 048a5](048a5-build-sign-in-refusals-before-parts.md), whole, with its Record;
   [order 048a5b](048a5b-build-sign-in-account-first.md), its decisions 0, 1, 8, 9, 10 and 14 and its
   Record.
3. The cases `requests-sign-in-lapse-and-conflict`, `requests-sign-in-refusals-before-parts` and
   `requests-sign-in-account-first`.
4. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: S**: about 0.2 to 0.3k changed lines of Rust with one unit test, one new case of 24
steps with one documents file of its own, and one sentence and three phrases in
`engine/CONFORMANCE.md`.

## The decisions it applies

1. **The invocation keeps its asked-for login.** In `invoke_inner` (`requests.rs`), right after the
   `let mut invocation = json!({...` line, where `op == c.qualified("sign-in.v1")` and
   `request.sign_in_login` is an object whose `name` is a string, set `invocation["asked_login"]` to
   that string; otherwise set no member. 048a-5b's `begin` does not change: neither its signature in
   `sign_in.rs` nor its call in `requests.rs`. 048a-5's invalid has by then refused a name the Type
   does not state. Nothing copies the member into a record, a decision, the reply or the inspection;
   decision 2 reads it, and order 048a-5d may read it to pass the name to the hook (IFC-006). Write
   `"asked_login"` nowhere else but decision 2's read.

2. **The fixed ports a sign-in may start.** In `sign_in.rs` add:
   - `pub(crate) fn may_start_fixed_ports(logins: &[Value], asked: Option<&str>) -> BTreeSet<u64>`:
     the logins it may start are, where `asked` is given, the logins whose `name` equals it byte
     for byte (none where no login has that name), and otherwise every login; it gives the union
     of the integers in each such login's `redirect.ports` where `return` is `redirect` and
     `redirect.ports` is a list. A login with `ports: any`, and any other return, gives none.
   - A function giving an instance's stated logins, each item of its Type's `spec.signIn.logins`
     whole and in order, through the Type lookup the names function makes now
     (`inventory::instance_type`). The names function then gives the `name` of each item this
     function gives, taking those items in place of the instance, so that no second lookup is
     written.
   - `pub(crate) fn other_sign_in_ports(state: &State, instance: &Value, c: &Context) -> Result<Vec<(Value, BTreeSet<u64>)>>`:
     for each invocation in `state.invocations`, in order, whose `state` is `in-progress`, whose
     `operation` is the core's `sign-in.v1` (`c.qualified("sign-in.v1")`) and whose `object` is
     not the instance's ID: find the object with that ID in `state.objects`, and skip the
     invocation where there is none or where the account function does not give it the pointer
     and a string; otherwise push the object's ID with `may_start_fixed_ports` of its stated
     logins and the invocation's `asked_login` as a string, or `None` where it has none. It looks
     up no Type where no invocation passes the first three tests.

3. **The refusal, in the preflight after the decision-6 helper.** In `sign_in.rs` add
   `pub(crate) fn fixed_port_refusal(state: &State, route: &Value, instance: &Value, own: &BTreeSet<u64>, others: &[(Value, BTreeSet<u64>)], time: &str, context: &Context) -> Result<Option<crate::explain::Rejection>>`,
   `instance` being the object the request's `arguments.instance` resolved to, as the decision-6
   helper takes it, the time a `&str`, as that helper takes it, and the `Context` and the `Result`
   those the visible-action rule needs, since `pending::raised_view` takes the one and returns the
   other. Where `own` is empty it gives none. Otherwise it takes the first item of
   `others` whose set shares a port with `own`, and gives none where there is none. For that item
   it gives `rejection("unavailable", "PND-010", tool, instance, C)`, `tool` being the route's
   `tool`, so reason `unavailable`, rule `PND-010` and one diagnostic
   `{"ruleId": "PND-010", "level": "error", "message": {"text": T}}`, `T` being

   ```text
   {tool} reports unavailable under PND-010 for {kind}/{name}: another sign-in.v1 in progress may start a login on the fixed {ports}, which a login this request may start also states, and one fixed port serves one login at a time; send sign-in.v1 again once that sign-in ends.
   ```

   `{kind}/{name}` as `rejection` writes them for the request's own instance, the cause `C` being
   the text after `: `, and `{ports}` `port P` where the two share one port and
   `ports P1, P2, ...` where they share more, every shared port in ascending order, separated by
   `, `. Where the visible-action rule, applied to the item's instance, the route's `caller` and
   the step's time, gives an action, append ` The other instance's pending sign-in action is {ID}.`
   with that action's ID. 048a-2 wrote that rule inline in its conflict, in `refusal`: move it
   into one function that both call, and write no second copy. No text names the other instance,
   an invocation ID or an account, and no `_meta` key is set.

   In the preflight, right after the decision-6 helper gives none, and so past the access tier's
   decision and the preflight's second line and LIF-031's invalid before the helper (048a-5b's
   decision 0 and its Record's R3), and only where the account function gave the pointer and a
   string: call `other_sign_in_ports` for the instance; where it gives an empty list, give none,
   looking nothing else up; otherwise compute `own`, `may_start_fixed_ports` of the instance's
   stated logins and the request's asked-for name (the `name` of `sign_in_login` where it is an
   object whose `name` is a string, else `None`), and give `fixed_port_refusal`'s rejection where it
   gives one, through the same path as the helper's. The preflight finds the instance's stated
   logins once, where it finds the names, and derives both the names and `own` from them. The helper
   giving none already means `Set`, so no second account or Type lookup is written. Otherwise the
   request goes on to dispatch as 048a-5b leaves it. The decision-6 helper's signature and answers
   do not change, so 048a-2's unit test and its calls pass unchanged. The preflight already returns
   a `Result`, called with `?` at both its calls in `invoke_inner` and with `.unwrap()` in the
   tests, so neither its calls nor its tests change.

4. **One unit test pins the rules.** In `sign_in.rs`'s tests, `sign_in_fixed_port_refusal`. Let
   B be `{"name": "browser", "return": "redirect", "redirect": {"host": "127.0.0.1", "ports": [8400, 8401], "path": "/callback"}}`,
   D `{"name": "device", "return": "device"}`, A
   `{"name": "any-port", "return": "redirect", "redirect": {"host": "127.0.0.1", "ports": "any", "path": "/cb"}}`,
   P `{"name": "pasted", "return": "page", "page": "https://example.org/code"}` and B2
   `{"name": "second", "return": "redirect", "redirect": {"host": "localhost", "ports": [8402, 8401], "path": "/other"}}`,
   and L the list B, D, A, P, B2. Assert with `assert_eq!`:
   - `may_start_fixed_ports`: L with `None` gives {8400, 8401, 8402}; with `browser` {8400, 8401};
     with `second` {8401, 8402}; with `device`, `any-port`, `pasted` and `phone` the empty set;
     the empty list with `None` the empty set.
   - `fixed_port_refusal`, each call with `.unwrap()`, on a `State` built by hand as
     `sign_in_refusals_before_dispatch` builds one (root `r`, the Interfaces `v`, with an open
     Conduit to the Fleet `f` listing `inspect.v1`, and `w`, with none, the built-in `management`,
     without which the visible-action rule fails with an error for `v`, the Fleet `f`, that test's
     Agent `signed` of `f`, ID `s1`, and its Agent `s2` of `f`), with `s2` holding a sign-in action
     with no answer, stored as that test stores one for `s1`, a route whose `tool` is
     `group.sign-in.v1`, `signed` as the request's instance, and the step's time before the
     action's expiry: `own` {8400, 8401} and `others` `[("s2", {8401, 8402})]`, for the caller
     `r`, reason `unavailable`, rule `PND-010` and the whole text, beginning
     `group.sign-in.v1 reports unavailable under PND-010 for Agent/signed: `, with `port 8401` and
     the appended sentence naming the action's ID; the
     same for `v`, the same text; for `w`, the text without the appended sentence; at the action's
     expiry, for `r`, without it; `own` {8400, 8401, 8402} and `others`
     `[("s1", {}), ("s2", {8401, 8402})]`, for `w`, the text with `ports 8401, 8402`; `own` {8400}
     with `others` `[("s2", {8401, 8402})]`, none; `own` empty with the same `others`, none;
     `others` empty, none. Where the appended sentence is absent, also assert that the serialized
     diagnostics hold no action ID.

5. **One portable case, with one documents file of its own.** Add the directory
   `conformance/conventions/requests/sign-in-fixed-port/` with two files.
   - `documents.json`: 048a-2's case's `documents.json`, whole and in its order, with two Agents
     inserted right after the Agent `unset`: `paired` and then `third`, each written as that file
     writes the Agent `signed` (from `signed-template`, `owner` `group`), with only its name
     changed. Write it as that file is written, with a final newline. Nothing else differs.
   - `case.yaml`: ID `requests-sign-in-fixed-port`, `layers: [convention, interface]`,
     `promises: [PND-010, OPS-023]`, title
     `Refuse as unavailable a sign-in that may start a login on a fixed port that another instance's sign-in in progress may start, each deciding by its asked-for login, after conflict`,
     `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
     `{"fixture": "../sign-in-lapse-and-conflict/fixture.json", "manifest": "../sign-in-lapse-and-conflict/manifest.json", "documents": "documents.json"}`.
     Write it as JSON with two-space indentation and a final newline, its members in the order of
     `requests-sign-in-refusals-before-parts`. It must meet 046a's portable-case rule as it
     stands.

   Below, ROOT, `META/x`, "Root asks T with A", "a completion" and REFUSED(R, RULE) are as
   048a-2's decision 10 defines them, ACCOUNT(X) as 048a-4's decision 8 defines it, and URL,
   LOGIN(X), MISSING and "Root asks T with A and M" as 048a-5's decision 5 defines them. A new
   step's evidence is its time, `2026-11-12T12:00:00Z`, and `ids` the 100 strings
   `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of 130000 + 100n +
   k, for k = 0 to 99, `revisions` the 16 strings `port-` followed by the decimal of n, a `-` and
   the decimal of j, and `suffixes` the 16 strings `p` followed by the four-digit, zero-padded
   decimal of 100n + j, for j = 0 to 15, where n is the step's number in this case; a copied step
   keeps its own evidence. `A(X)` is the arguments `{"instance": {"kind": "Agent", "name": X}}`, and
   `PA` the JSON pointer `/reply/_meta/` followed by `META/pendingAction`, its `/` written `~1`. Two
   results recur:
   - PENDING: `{"reply": {"pending": true}}`;
   - BUSY: `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable"}, "structuredContent": {"diagnostics": [{"ruleId": "PND-010", "level": "error"}]}}, "external_intents": [], "record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "unavailable", "ruleId": "PND-010"}}], "absent": [PA]}`.

   Its steps:
   1. to 3. `requests-sign-in-lapse-and-conflict`'s steps 1 to 3, `input`, `expect` and `capture`,
      whole, each with its evidence: first start, registration of this case's documents, and the
      setup of `signed`, `$setup1`.
   4. to 6. That case's steps 7 to 9, whole, each with its evidence: the setup's two run reports,
      the second dispatching the account check `$account0`, and that check's completion with the
      expected account; the setup succeeds.
   7. Root asks `group.set-up.v1` with `A("paired")`. `expect` PENDING; `capture`
      `{"setup2": "/reply/invocation"}`.
   8. to 10. Steps 4 to 6 again, each a new step with its own evidence: its `input` with `$setup2`
      in place of `$setup1` and `$account2` in place of `$account0`, its `expect` unchanged, and
      step 9's `capture` `{"account2": "/reply/hook"}`.
   11. Root asks `group.set-up.v1` with `A("third")`. `expect` PENDING; `capture`
       `{"setup3": "/reply/invocation"}`. `third`'s setup stays in progress through step 16.
   12. Root asks `group.sign-in.v1` with `A("signed")` and URL and LOGIN(`{"name": "device"}`).
       `expect` PENDING: dispatched; its account check is pending, so it stays in progress, and it
       may start `device` alone.
   13. Root asks `group.sign-in.v1` with `A("paired")` and URL. `expect` PENDING; `capture`
       `{"check13": "/reply/hook"}`: the sign-in in progress kept its asked-for login, which states
       no fixed port (decision 1), and `third`'s setup in progress is not a `sign-in.v1`, so it is
       ignored (decision 2).
   14. to 16. Steps 4 to 6 again, as steps 8 to 10 are, with `$setup3` and `$account3`, and step
       15's `capture` `{"account3": "/reply/hook"}`; `third`'s setup succeeds.
   17. Root asks `group.sign-in.v1` with `A("third")` and URL and LOGIN(`{"name": "browser"}`).
       `expect` BUSY: `paired`'s sign-in asked for no login, so it may start `browser`, on the
       ports 8400 and 8401.
   18. Root asks `group.sign-in.v1` with `A("third")` and URL. `expect` BUSY: without a login this
       request may start `browser` too.
   19. Root asks `group.sign-in.v1` with `A("third")` and URL and LOGIN(`{"name": "device"}`).
       `expect` PENDING: a login that states no fixed port; `capture` `{"check19": "/reply/hook"}`.
   20. Step 18's request. `expect` REFUSED(`conflict`, `PND-010`), with `"external_intents": []`:
       `third`'s own sign-in is in progress, and conflict comes before unavailable (OPS-023).
   21. Root asks `group.sign-in.v1` with `A("third")` and no URL. `expect` MISSING with
       `"absent": ["/reply/inputRequired"]` right after `record_facts`, as 048a-5b's decision 9
       writes the MISSING steps of 048a-5's case: `third`'s own sign-in and `paired`'s are still in
       progress, and the capability's invalid comes before both conflict and unavailable (OPS-023).
   22. A completion for `$check19`, ACCOUNT(`"primary-account"`). `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}}`: `third`'s own
       sign-in ends.
   23. A completion for `$check13`, ACCOUNT(`"primary-account"`). `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}}`: `paired`'s sign-in
       ends.
   24. Step 18's request. `expect` PENDING: `paired`'s sign-in has ended, so it is no longer
       compared (decision 2), while `signed`'s, still in progress, may start `device` alone.

   The coordinator built the case and its documents from this text on a scratch copy of `95b9fc3b`,
   outside the repository: its 2374 IDs are unique, its steps 1 to 16 pass on that engine, and step
   17 fails, the refusal not being built; on a scratch build of decisions 1 to 3 every step passed.
   Before running it, check that no ID repeats across the case's steps' `ids`, the copied steps'
   included. If 046a's runner check refuses the case, if the registration rejects the documents or
   runs out of IDs, if a setup asks for a report other than those its steps send, if a step copied
   from 048a-2's case gives a reply other than that case states, or if a step's reply differs from
   what it states, stop and report the error: do not change the expectations to pass it, and do not
   change the inputs beyond what this decision says. No expectation is generated from the engine's
   output, and the case asserts no diagnostic's words, since IFC-050 leaves wording to the engine:
   the unit test pins them. The coverage index entry holds `id`, `case`
   (`conventions/requests/sign-in-fixed-port/case.yaml`), `promises` and, where its neighbours in
   that piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write it in a
   `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it into
   `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte for
   byte as it was.

6. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find the sentence by its text read across
   line breaks, as 048a-5b leaves it. In the section "Refusals and reasons (order 046a)", at the end
   of the item that holds the sentence beginning "After order 048a-2's invalid and before its
   conflict, order 048a-5 decides three more invalids", which ends "`signInReturn` to the core.",
   add, in the same item:

   ```text
   After order 048a-2's conflicts and before any part, order 048a-5c decides PND-010's fixed-port refusal: where another instance's `sign-in.v1` is in progress and may start a login stating a fixed port that a login this request may start also states, the request reports unavailable, naming every shared port and, only where the caller may see it, that instance's sign-in action. A `sign-in.v1` may start its asked-for login, which the engine keeps on the invocation, or without one any of its Type's logins; a redirect login's `ports: any`, and a device, page or service login, state no fixed port. Every other instance's sign-in in progress is compared, whatever its Fleet or node, since PND-010 names neither.
   ```

   and rewrap the item at 100 columns, changing no word.

   In the three places that 048a-5b's decision 10 (its places 1 to 3) leaves holding the words
   "where the checks of orders 048a-2 and 048a-5 apply, and otherwise" (DL18's row in "Choices
   still required", the paragraph on public engine execution, and the list of what the build does
   not claim), those words, in each place, become "where the checks of orders 048a-2 and 048a-5
   apply, unavailable where order 048a-5c's fixed-port check applies, and otherwise", keeping every
   word before "where" and after "otherwise".

   The file names order 048a-5c nowhere on `95b9fc3b` (the start check), so nothing says the
   fixed-port refusal is not built yet; where a later change added such words, remove them, keep the
   rest of their sentence, and say so. Nothing else in the file changes but that rewrap. After the
   other decisions, run `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change
   to a case's `promises` and to a promise ID in the core's sources. Report the rows of PND-010 and
   OPS-023 and the totals; each row's number of cases is one more than the start check printed,
   `OPS-023  26` and `PND-010  4` from `95b9fc3b`, and no total changes, as on the coordinator's
   scratch build.

7. **The declaration.** `docs/work-orders/048a5c-declared.yaml` holds one shape, `added`, for
   `requests-sign-in-fixed-port`. The coordinator wrote it with this order; check that it matches
   and change it only to make it so.

8. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
   record changes. No fixture or other case changes. No shell code, CLI command, hook, relay or
   package changes. The `sign-in-hook.v1` part and IFC-049's URL match (048a-5d), the relayed output
   (048a-5e), the requestState (048b-relay1), polling and its endings (048a-5f to 048a-5h) and the
   returns (048a-7 and 048a-8; 048a-6 has merged) are not built here, nor ruling 86's points 3 (c)
   and (d) (order 209). Report as findings, for the coordinator: that PND-010 names no node, so two
   instances on different nodes whose logins share a fixed port are refused one at a time, as
   written; that the refusal tells a caller other than root that some sign-in it cannot inspect is
   in progress, though the text names no instance, while its refused requests stay at tier 1; that a
   login with `ports: any` may bind a port another login states fixed, and PND-010 refuses only a
   shared fixed port; that a sign-in in progress keeps "may start" every login its request allowed,
   even once its hook has relayed one login, since the same invocation may answer another
   InputRequiredResult; that no case shows the pending action withheld from a caller that may not
   see it (PND-002), so the unit test alone pins it, as it pins 048a-2's conflict sentence; that the
   case asserts no diagnostic's words; that a keyed repeat that returns a kept outcome is not
   compared, while a keyed request dispatched again is (report whether any path dispatches a
   `sign-in.v1` past this check otherwise); that once order 209 refuses every named login aimed at a
   hook whose operation is not keyed, a kept `asked_login` is read only for an implementation item
   that carries Conduit's keys, as this case's does; that ruling 86 point 3 (c)'s reason is order
   207's to write, and where it falls against this order's unavailable under OPS-023 follows from
   it; that ports are compared whatever the redirect's host, so a login on [::1] and one on
   127.0.0.1 stating one port are refused one at a time, as written; and any place where PND-010
   left you a choice this order does not settle.

## Scope

Change only `engine/crates/core/src/sign_in.rs` (decisions 2, 3 and 4, and the names function's
body and parameter), `engine/crates/core/src/requests.rs` (decision 1's member),
`engine/CONFORMANCE.md` (one sentence, the three places decision 6 names, and decision 6's
removal where it applies),
`engine/STANDING.md`, written only by the tool, `conformance/conventions/requests/index.yaml` and
the new directory `conformance/conventions/requests/sign-in-fixed-port/`.
`docs/work-orders/048a5c-declared.yaml` comes with this order; change it only as decision 7 says.
`conformance/manifest.yaml` holds the new entry only until the pieces tool moves it, and ends
unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, `engine/crates/shell/`, the
experiments, the case schema, `conformance/internal-cases.yaml`,
`conformance/expected-failures.yaml`, any other case or fixture, the files of 048a-2's case
directory, `docs/implementation/` or `docs/proposals/`: report any defect as a finding.
`experiments/011-short-form-spike/` changes only if its tests need it, since it depends on the
core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- sign_in
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-fixed-port
cargo run --release -p conduit-conformance -- .. requests-sign-in-refusals-before-parts
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports one more test passed than on the commit you start from, 0 failed, the new one
being `sign_in_fixed_port_refusal` (17 against 16 from `95b9fc3b`); report both counts. The
pieces tool prints `Nothing to move.` The next four runner commands each end `1 cases passed`, and
`requests-` gives one more case than on the commit you start from, with the same expected
failures, each still failing (`71 cases passed; 2 expected failures` from `95b9fc3b`); report both
counts. The shell test passes. If any other case, or any shell, CLI or experiment test, fails
because a `sign-in.v1` request now reports unavailable under PND-010, stop and report it: do not
change it. The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decision 3's call, and without decision 2's read of the
kept login. From `engine/`, first keep a copy:

```bash
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
```

Then, by hand, in `crates/core/src/sign_in.rs`, in the preflight, replace the call of
`fixed_port_refusal` with a value that gives none: `None` where the call is followed by `?`, and
`Ok(None)` where it is not; nothing else. Then run:

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-fixed-port
cp target/sign_in.rs.fixed crates/core/src/sign_in.rs
```

The runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with the error
`missing ordered record {"data":{"reason":"unavailable","ruleId":"PND-010"},"type":"request-ended"}`:
the runner checks a step's `record_facts` before its reply and names no step for them, and step 17
is the first step that expects that record, its reply now pending. Then, by hand, in
`other_sign_in_ports`, pass `None` in place of the invocation's `asked_login`, and nothing else.
Then, one at a time:

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-fixed-port
cp target/sign_in.rs.fixed crates/core/src/sign_in.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-fixed-port
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with the error
`portable case: step 13 ends with reason unavailable but does not assert it and its deciding ruleId`:
`signed`'s sign-in in progress is then read as starting `browser`; after the restore the second ends
`1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older modification time, so
cargo would not rebuild the crate. Git ignores `target/`.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a5c-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-setup-account conformance/conventions/requests/sign-in-refusals-before-parts conformance/conventions/requests/sign-in-account-first conformance/conventions/requests/sign-in-return-kept-nowhere conformance/conventions/requests/sign-in-return-not-taken conformance/conventions/inventory conformance/conventions/setup conformance/language engine/crates/shell tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5c-declared.yaml: 1 shapes.`,
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
grep -c 'fn sign_in_fixed_port_refusal' engine/crates/core/src/sign_in.rs
grep -c 'fn may_start_fixed_ports' engine/crates/core/src/sign_in.rs
grep -c 'fn other_sign_in_ports' engine/crates/core/src/sign_in.rs
grep -c 'fn fixed_port_refusal' engine/crates/core/src/sign_in.rs
grep -c '"id": "requests-sign-in-fixed-port"' conformance/conventions/requests/index.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'After order 048a-2.s conflicts and before any part, order 048a-5c decides PND-010.s fixed-port refusal'
```

This prints `3`, decision 6's three places:

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply, unavailable where order 048a-5c.s fixed-port check applies, and otherwise' | wc -l | tr -d ' '
```

This prints `2`, the member's write (decision 1) and its read (decision 2):

```bash
grep -rho '"asked_login"' engine/crates/core/src | wc -l | tr -d ' '
```

This prints the two lines of the start check, each number of cases one more
(`OPS-023  26` and `PND-010  4` from `95b9fc3b`):

```bash
awk -F'|' '$2 == " PND-010 " || $2 == " OPS-023 " {print $2 $5}' engine/STANDING.md
```

This prints `['browser', 'device', 'paired', 'signed', 'third', 'unset'] 24`: the names of the
Type's logins and of the documents' Agents, sorted together, and the case's number of steps:

```bash
python3 -c "import json; d = json.load(open('conformance/conventions/requests/sign-in-fixed-port/documents.json')); t = [x for x in d if x['kind'] == 'Type' and x['metadata']['name'] == 'signed-agent.v1'][0]; print(sorted([l['name'] for l in t['spec']['signIn']['logins']] + [x['metadata']['name'] for x in d if x['kind'] == 'Agent' and 'from' in x]), len(json.load(open('conformance/conventions/requests/sign-in-fixed-port/case.yaml'))['steps']))"
```

Each of these prints 0:

```bash
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml | wc -l | tr -d ' '
grep -c 'asked_login' engine/crates/shell/src/server.rs
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 6 names with the totals, the function names and signatures of
decisions 2 and 3, the file and line where decision 1 sets `asked_login`, and the line in the
preflight where the fixed-port refusal is decided.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5c-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
