# Work order 048a5 (E7a-5): `sign-in.v1`'s refusals before any part

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the case you add and the case you change, with
the promises each tests and whether it passes. This order has network access only because the
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
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). The fourth is
"048a-5, `sign-in.v1` dispatched, with IFC-049's relay and its remaining invalids before this
order's conflict, the fixed-port refusal, polling to DL18, the signed-in answer and the stub
removed, which may need its own split". It does: built whole it is eight behaviors, each with its
vectors, and the relay alone is an order's worth. So the coordinator cuts 048a-5 into eight
parts, each after the one before (see "Who decided what"):

- **048a-5, this order:** `sign-in.v1`'s remaining refusals before any part, placed after
  048a-2's invalid and before its conflict: a request that does not declare URL elicitation gets
  the protocol's error -32021 under IFC-049, recorded invalid; a request that carries
  `signInReturn` reports invalid under PND-013, since a retry reaches the core only as order
  048b-relay1's `respond` input, never as such a request; and a `signInLogin` that names no login
  the Type states reports invalid under PND-010. The core's request gains the three members these
  checks read, and the shell forwards `clientCapabilities` and `signInLogin` to it;
  048a-6 forwards `signInReturn`. The stub stays.
- **048a-5b:** `sign-in.v1` dispatched, its account check first, and the signed-in answer, with
  C11's stub removed: an `account-hook.v1` part before anything else (PND-010's `observable`,
  PND-011); the expected account, compared ignoring ASCII case, succeeds with output `account`,
  answers the instance's unanswered sign-in action `signed-in`, lifts PND-005's hold (order
  048a-3's stand on MC9a-003) and clears `SignInUndone` (PND-011); a failed or unknown check
  completes nothing; and an interim ending for no account or another, which 048a-5b states in its
  own text and in `engine/CONFORMANCE.md` until 048a-5e replaces it.
- **048a-5c:** PND-010's fixed-port refusal, unavailable, naming the port and only a visible
  pending action, with the asked-for login kept on the invocation in progress so that "may start"
  can be decided.
- **048a-5d:** the `sign-in-hook.v1` part, with `signInLogin` passed to the hook (IFC-006), and
  IFC-049's URL match, the URL matched to exactly one stated login and the asked-for one; after
  048b-relay2 too.
- **048a-5e:** the engine's own message and the input-required output, with `signInLogin`'s
  `{name, return, node, acceptsReturn}` and DL18's instant as its expiry, which order
  048b-relay1 relays.
- **048a-5f:** polling on the retry: the account checked again, the expected account signing in,
  no account yet answering another input-required output, and a decline or cancel ending failed.
- **048a-5g:** another account found while polling, undone by a `sign-out-hook.v1` part and
  ending failed naming both, the action staying pending with the attempt as evidence; a failed
  undo sets `SignInUndone=False`.
- **048a-5h:** DL18's end: a sign-in with no retry ends unknown, and no hook runs later.

The shell issues, keeps and checks the sign-in's requestState (order 048b-relay1), and a retry
reaches the core only as its respond input.

048a-6, a return where none is taken, follows this order directly; 048a-7 and 048a-8, the returns
taken, follow 048a-5h and 048a-6; and 048b-join follows 048a-8 and 048b-relay2. The coordinator
records this cut in order 117's entry and the README's rows, in this order's pull request; do not
change either file. This order builds none of 048a-5b to 048a-8.

**What it builds.** One behavior: the refusals that PND-010, PND-013 and IFC-049 put before any
part of `sign-in.v1` and that order 048a-2 left out (its decision 6: "IFC-049's missing URL
capability, an unknown `signInLogin` and PND-013's return on a first request, each invalid before
conflict, are not checked here; the order that dispatches `sign-in.v1` adds them before this
conflict"). IFC-049: "For sign-in.v1 the request must declare `{"elicitation": {"url": {}}}` in
_meta.io.modelcontextprotocol/clientCapabilities; without it the engine answers -32021 with
data.requiredCapabilities naming URL elicitation, sending no elicitation. It does so after the
access decision and before any part is dispatched; the request is not dispatched and reports
invalid, recorded so, and the error holds in its data" the `_meta` key `reason` "with the value
invalid, as IFC-036 has such an error hold it." PND-010: "Before dispatching any part, a
signInLogin naming no login the Type states reports invalid, with a diagnostic naming PND-010 and
the stated login names." PND-013's `observable`: "signInReturn on a first sign-in.v1 request ... is
not a return: after access and before dispatch the request reports invalid in OPS-023's order,
with a diagnostic naming PND-013 and the key, holding no value." No promise changes: the texts
state all of it, and this order builds it. The other halves of PND-013's sentence, `signInReturn`
on another operation's request or on a `tasks/update`, are 048a-6's, with the expected failure
`requests-sign-in-return-kept-nowhere`.

**What order 048a-4 brings, and the orders before it, which this order assumes.** Orders 048a-2
(#364), 048a-3 (#367) and 048a-4 (#371, its file `048a4-build-sign-in-setup-account.md`) have
merged, and orders 051b (#372) and 048b-relay1 (#373) after them; `main` is at `8e205447`. This
order starts from that `main` (below). It assumes, as their decisions and Records state them:

- From 048a-2's decision 5: `engine/crates/core/src/sign_in.rs`, with `mod sign_in;` in `lib.rs`,
  and in it `expected_account`, the function that gives an instance's expected account
  (`ExpectedAccount::NoSignIn`, `NotString` with the pointer, or `Set` with the pointer and the
  string), called "the account function" below, which finds the instance's Type through
  `inventory::instance_type`, the lookup `inventory.rs`'s `own_tools` calls, and gives `NoSignIn`
  without a lookup for an instance with no `held`, a template, or an object that is not a Resource
  or Agent instance.
- From 048a-2's decision 6, with its Record's R1 and R3: `preflight`, the preflight of
  `sign-in.v1` in `sign_in.rs`, called in `invoke_inner` right after `records::preflight`'s block
  under `previous.is_none() || redispatch`, giving its rejection through `reject_preflight` and so
  `reject_diagnostics`, and giving none, so leaving the stub's unavailable, unless the caller is
  root, or the instance is an Agent whose owning Fleet is the route's target and the caller is an
  Interface on its own behalf (R1); and its helper `refusal`, called "the decision-6 helper" below,
  which already takes the `Context` and decides in OPS-023's order (1) the invalid under PND-010
  for a Type that states no `signIn` or an account that is not a string, then the conflict under
  LIF-037 for a stale `expectedId` (R3), then (2) the conflict under PND-010 while the instance's
  earlier `sign-in.v1`, setup or retirement is in progress; then the stub,
  `("sign-in.v1", "048a", "PND-010")`, unchanged. Its unit test is
  `sign_in_refusals_before_dispatch`, which builds its routes as JSON and calls `preflight` and
  `refusal` directly.
- From 048a-2's decision 10, as its Record's R2 and 048a-4's decision 9 leave it: the case
  `requests-sign-in-lapse-and-conflict` in `conformance/conventions/requests/sign-in-lapse-and-conflict/`,
  with its `manifest.json`, `documents.json` and `fixture.json` (the Type `signed-agent.v1`, whose
  account pointer is `/account` and whose logins are `browser` and `device`; the Fleet `group`; the
  Agents `signed`, account `primary-account`, and `unset`, account null); its definitions ROOT,
  `META/x`, "Root asks T with A", "a run report", "a completion" and REFUSED(R, RULE); its steps 1
  to 3; its step 4, root's `group.sign-in.v1` for `signed` while its setup is in progress,
  expecting conflict; its steps 7 to 9, the setup's two run reports and 048a-4's inserted
  completion of the account check, `$account0`, with the expected account; its step 12, step
  4's request once nothing is in progress, expecting the stub's unavailable; and its step 29, of
  the steps 28 to 31 that 048a-2's Record (R2) added, root's `group.sign-in.v1` for `signed` with
  a stale `expectedId`, expecting conflict under LIF-037. It has 32 steps.
- From 048a-2's decision 11, as its Record's R3 leaves it: in `engine/CONFORMANCE.md`, the words
  "invalid or conflict under PND-010" and, after them, "where order 048a-2's checks apply" in three
  places (DL18's row, the shell's paragraph and the item of the unbuilt list; in the last two R3
  put "or conflict under LIF-037 for a stale expectedId," between them), and in the section
  "Refusals and reasons (order 046a)" the sentence "IFC-049's missing capability, an unknown
  `signInLogin` and PND-013's return on a first request, each invalid, are not yet checked, so such
  a request reports conflict or unavailable instead."
- From 048a-3 and 048a-4: nothing this order reads, but their holds and the account check, which
  the copied steps pass through. `sign-in.v1` targets the instance's Fleet, not the instance, so no
  hold reaches it.
- From 048b-relay1: `call_tool` in `server.rs` builds either the `invoke` input or, for a retry
  that echoes a requestState, its `respond` input in place of it, and a refused retry calls
  `record_rejection`. From 051b: nothing this order reads.

Where 048a-2, 048a-3 or 048a-4 merged otherwise than its order says, apply the nearest sound
reading of each decision below to what merged, and say so. Where a Record names a function, use
that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`8e205447`, after orders 048a-2 to 048a-4, 051b and 048b-relay1; find each place by the text quoted
with it.

- `AccessRequest` (`engine/crates/core/src/access.rs:17`) has the members `identity`, `tool`,
  `arguments`, `serving`, `readable` and `authority_readable`, under
  `#[serde(deny_unknown_fields)]`. Of a request's `_meta`, only what the shell maps to `serving`
  and the input's `key` reaches the core's invoke input: `call_tool`'s invoke arm
  (`crates/shell/src/server.rs`, the line `json!({"kind":"invoke","request":{...`,
  `server.rs:472`, in the arm `Ok(None) =>` of `match retry`) reads only the `_meta` keys
  `serving` and `idempotencyKey` under the prefix that `spec/names.yaml` sets. 048b-relay1's
  `respond` input carries the whole `_meta` as `meta`, which this order neither reads nor
  changes. A case's `invoke` step writes the
  request as the core reads it, so order 171b's case `requests-sign-in-return-kept-nowhere` writes
  `signInReturn` as a member of the request, beside `identity`, `tool` and `arguments`, as the
  runner writes `serving`; it fails today on that unknown member.
- Struct literals of `AccessRequest` stand in `engine/crates/core/src/explain.rs:1068` (a test),
  `inventory.rs:284`, `lifecycle.rs:475` and `:1629`, `records.rs:822`, `requests.rs:2580`,
  `:2667` and `:3020`, and `engine/crates/conformance/src/lib.rs:776`.
- `reject_diagnostics` (`requests.rs:639`) writes a reason's reply, and for the rules `ACC-001` and
  `TPL-013` replaces it with the JSON-RPC error -32602, its `data` holding the `_meta` key
  `reason` and the diagnostics (the block `if rule == "ACC-001" || rule == "TPL-013"`), and for
  `LIF-037` writes the expected and resolved IDs (the block `if rule == "LIF-037"`, from 048a-2);
  it then pushes `end-with-reason` and calls `end`, which writes the `request-ended` record. The shell
  answers a core reply that holds `error` as that protocol error. The shell's own IFC-036 error is
  `{"code": -32021, "message": "Missing required client capability", "data": {"requiredCapabilities": ..., <reason key>: "invalid", "diagnostics": [...]}}`.
- `mark_unanswered` (`lib.rs:463`) leaves a reply without `isError`, such as a protocol error, as
  it is.

**When this order starts.** Order 048a-4 has merged to `main` (#371), and with it 048a-2 and
048a-3; no order before this one in the chain is unmerged. Before you change anything, check from
the repository root that you find what they give, and stop, change nothing and report if any check
prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/048a4-build-sign-in-setup-account.md
grep -c '^## Record' docs/work-orders/048a2-build-sign-in-lapse-and-conflict.md
grep -c 'fn sign_in_refusals_before_dispatch' engine/crates/core/src/sign_in.rs
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
grep -rl '"hold-for-sign-in"' engine/crates/core/src | wc -l
grep -rl '"dispatch-account-check"' engine/crates/core/src | wc -l
grep -c '"account0"' conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml
python3 -c "import json; s = json.load(open('conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml'))['steps']; print(s[3]['input']['request']['tool'], s[11]['input']['request']['tool'], s[28]['input']['request']['tool'], len(s))"
grep -c -E 'signInReturn|signInLogin|clientCapabilities' engine/crates/core/src/access.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'invalid or conflict under PND-010' | wc -l
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o "where order 048a-2's checks apply" | wc -l
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'are not yet checked, so such a request reports conflict or unavailable instead'
grep -c '"id": "requests-sign-in-return-kept-nowhere"' conformance/expected-failures.yaml
```

They print `1`, `1`, `1`, `1`, `1`, `1`, a number other than `0`, `group.sign-in.v1
group.sign-in.v1 group.sign-in.v1 32`, `0`, `3`, `3`, `1` and `1`. The first is 048a-4's merge.
The second to fourth are 048a-2's: its merge, the decision-6 helper's test and the stub. The fifth
is 048a-3's hold, the sixth 048a-4's account check. The seventh and eighth are 048a-4's decision 9
and 048a-2's Record (R2) in 048a-2's case: 048a-4's inserted step and R2's steps 28 to 31, so that
the steps this order changes are at indexes 3, 11 and 28 of 32. The ninth says the core's request
has none of this order's members yet, and the last four are the texts this order replaces. Orders
that land beside this one (049a-1, 050 and others): whichever lands second keeps both changes and
writes `engine/STANDING.md` again with the tool. Order 048b-relay1 merged first (#373). It
rewrote `call_tool` to build either the `invoke` input or, for a retry that echoes a requestState,
its `respond` input in place of it; `sign_in_members` goes into the `invoke` input and into
`record_rejection` alone, never into the `respond` input. It names IFC-049 in the shell's sources,
so IFC-049's row of `engine/STANDING.md`, now `0 | 0 | outside | core, shell | no case, no reason`,
reads `core, shell` after this order (Done when).

**Who decided what.** PND-010, PND-013, IFC-049, IFC-036 and OPS-023 decide which requests are
refused before any part, with which reason and rule, that the missing capability is the protocol's
error -32021 recorded invalid, and that each refusal is decided after the access decision and before
conflict. The owner's [ruling 83](../proposals/ruling-83/README.md) of 2026-11-18 makes the Agent
packages 0.2.0 subscription-only, with `account` required, a string or null, with no default, and no
`apiKey`; the case's Type, `signed-agent.v1`, already states that shape, so nothing here changes
with it. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-5 above into 048a-5 and 048a-5b to
048a-5h (FR-264), leaving 048a-5b's interim ending to its own order; the order of the invalids among
themselves, 048a-2's first, then the capability, then the return, then the login, since an instance
that cannot be signed in at all is the first thing a caller must learn and the capability is a
precondition of every later answer; the three members' names, `clientCapabilities`, `signInLogin`
and `signInReturn`, following how order 171b's case writes `signInReturn` and the runner writes
`serving`; that a member absent or null is not carried; that a capability declares URL elicitation
only where `elicitation.url` is an object, as the protocol's empty object for form only has it;
reading a `signInLogin` that is not an object whose `name` is a string as naming no login, and
comparing names byte for byte, as CTR-013's "Names differ within the Type" does; that every
`sign-in.v1` request the core receives as an `invoke` is a first request, since the shell passes a
retry on only as order 048b-relay1's `respond` input, after its own requestState checks; that the
core reads only whether `signInReturn` is carried, never its value; the error's message and `data`,
mirroring the shell's IFC-036 error; the shell forwarding `clientCapabilities` and `signInLogin`,
and leaving `signInReturn` to 048a-6; the diagnostics' words; the function names; the unit test and
the shell's test; the case, its route, IDs and times; the three steps changed in 048a-2's case; the
new reason of the expected failure; and the documentation edits. Apply them as written and report
them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: PND-010 and PND-013 (`spec/promises/pending.yaml`); IFC-006, IFC-036, IFC-049 and
   IFC-050 (`spec/interface/implementation.yaml`); OPS-023 (`spec/conventions/operations.yaml`);
   CTR-013 (`spec/promises/contracts.yaml`), its sentence on login names; `sign-in.v1`
   (`spec/core/operations.yaml`).
2. [Order 048a2](048a2-build-sign-in-lapse-and-conflict.md), its decisions 5, 6, 10 and 11 and its
   Record; order 048a-4's decision 9 and its Record (its file
   `048a4-build-sign-in-setup-account.md`); [order 171b](171b-sign-in-return-and-holds.md), its
   case and its Record.
3. The cases `requests-sign-in-lapse-and-conflict`, `requests-sign-in-return-kept-nowhere` and
   `explain-refusal-tiers`, the last for how a case expects a JSON-RPC error.
4. `conformance/README.md`, its sections "Assertions" and "Portable cases".
5. `engine/crates/shell/src/server.rs`: `call_tool`'s invoke input and `record_rejection`.

**Size: S-M**: about 0.2 to 0.3k changed lines of Rust with one unit test in the core and one in
the shell, one new case of 18
steps on 048a-2's fixture, one member added to three steps of 048a-2's case, four places in
`engine/CONFORMANCE.md` and one reason in `conformance/expected-failures.yaml`.

## The decisions it applies

1. **The core's request carries the three members.** In `access.rs`, `AccessRequest` gains, after
   `authority_readable`, three members, each `Option<Value>`, with its attribute written exactly
   as given: `client_capabilities`, with `#[serde(default, rename = "clientCapabilities")]`, the
   request's `_meta.io.modelcontextprotocol/clientCapabilities`; `sign_in_login`, with
   `#[serde(default, rename = "signInLogin")]`, the `_meta` key `signInLogin` under the prefix
   that `spec/names.yaml` sets; and `sign_in_return`, with
   `#[serde(default, rename = "signInReturn")]`, the `_meta` key `signInReturn` under that
   prefix. A member that is absent or
   JSON null is not carried. Add the three as `None` to each struct literal of `AccessRequest`
   listed above, and to any other, and say where. Nothing copies a member into the route, a
   record, a decision, the reply or the state, and nothing reads `sign_in_return` but whether it is
   carried (PND-014). Only decision 2 reads the members: for any operation other than the core's
   `sign-in.v1` they change nothing.

   The shell forwards two of them, so that a client that declares URL elicitation is not told it did
   not (IFC-049 answers -32021 only "without it"; ruling 52). In
   `engine/crates/shell/src/server.rs`, add one function, `pub(crate)` so that the shell's tests
   reach it:

   ```rust
   pub(crate) fn sign_in_members(meta: &Value, prefix: &str) -> serde_json::Map<String, Value>
   ```

   Given a request's `_meta` value and the prefix, it gives the members to add to the invoke
   input's `request`, and an empty map where it gives no member: `clientCapabilities`, the value of
   `_meta`'s key `io.modelcontextprotocol/clientCapabilities`, and `signInLogin`, the value of the
   key `signInLogin` under the prefix, each only where `_meta` holds the key with a value other than
   JSON null. `call_tool` calls `sign_in_members` and adds them to the `request` of its invoke input
   (the line `json!({"kind":"invoke","request":{...`, `server.rs:472` at `8e205447`, in the arm
   `Ok(None) =>` of `match retry`), and `record_rejection` calls it and adds them to the `request`
   of its rejection record's input (`server.rs:576`), from the `_meta` of the body's `params`. Neither forwards `signInReturn`,
   which 048a-6 forwards; nothing else in the shell changes. Name `signInReturn` nowhere in
   `server.rs`, not even in a comment. The shell's IFC-036 check stands as it is: a request that
   carries `signInLogin` under the prefix declares the implementation-interface extension, or the
   shell answers its own -32021 first. Write no promise ID in the shell's new code or its test, so
   that this order adds no shell source to any row of `engine/STANDING.md`; order 048b-relay1
   names IFC-049 in the shell. Since 048b-relay1 merged first, `call_tool` builds either the
   `invoke` input or its `respond` input: add the members to the `invoke` input alone, never to
   the `respond` input.

2. **The three refusals, between 048a-2's invalid and its conflict.** In `sign_in.rs`, add
   `request_member_refusal`, given the route, the request, the instance's stated login names in
   the Type's order, and the `Context`, which gives `Option<crate::explain::Rejection>`, deciding
   in this order and giving the first that applies:
   1. **The capability (IFC-049).** Where `client_capabilities` is not carried, or
      the member `url` of its member `elicitation` is not a JSON object: reason `invalid`, rule
      `IFC-049`, with the text
      `{tool} reports invalid under IFC-049: the request does not declare URL elicitation in its client capabilities, and sign-in.v1 relays a login only by URL elicitation; send it again with {"elicitation": {"url": {}}} in io.modelcontextprotocol/clientCapabilities.`
   2. **A return on a first request (PND-013).** Where `sign_in_return` is carried: reason
      `invalid`, rule `PND-013`, with the text
      `{tool} reports invalid under PND-013: the request carries {key} on a first sign-in.v1 request, which takes no return; a return travels only on a retry of sign-in.v1 in the invocation its requestState binds.`,
      `{key}` being the `_meta` key `signInReturn` under the prefix, `c.meta_prefix` followed by
      `signInReturn`, as `requests.rs`'s `mark` writes a key. This refusal stays as it is in later
      orders: a retry never reaches it, since the shell passes a retry to the core only as order
      048b-relay1's `respond` input.
   3. **An unknown login (PND-010).** Where `sign_in_login` is carried and is not an object whose
      `name` is a string: reason `invalid`, rule `PND-010`, with the text
      `{tool} reports invalid under PND-010: signInLogin must hold the name of one login the instance's Type states; its stated logins are {names}.`;
      where its `name` is a string equal, byte for byte, to no stated name:
      `{tool} reports invalid under PND-010: signInLogin asks for the login {name}, which the instance's Type does not state; its stated logins are {names}.`
      `{name}` is the asked-for name and each of `{names}` a stated name, each written as a JSON
      string with its quotes, the names separated by `, `.

   `{tool}` is the route's `tool`. Each rejection's diagnostics are one
   `{"ruleId": R, "level": "error", "message": {"text": T}}`. No text holds the account or any part
   of the return's value. Add a function in `sign_in.rs` that gives an instance's stated login
   names, the `name` of each item of its Type's `spec.signIn.logins` in order, found through the
   same Type lookup as the account function, `inventory::instance_type`; write no second copy of
   it. The decision-6 helper gains two parameters, the request and the stated names, and also the
   `Context` where it does not already take it, and calls `request_member_refusal` right after its
   invalid (1) and before its conflicts, the LIF-037 conflict for a stale `expectedId` and (2),
   since OPS-023 puts invalid before conflict, giving its rejection where it gives one. The preflight finds the names only where the account function
   gives the pointer and a string, and otherwise passes none, since the invalid (1) answers first:
   so the names function, like the account function, never looks up the Type of an instance with no
   `held`, a template, or an object that is not a Resource or Agent instance, whose lookup would end
   in an engine error. The preflight passes the names with `request`; `invoke_inner` passes
   `request` to the preflight. As before, the preflight gives none, leaving the stub's unavailable,
   to a caller that 048a-2's Record (R1) leaves at the stub, so none of the three is decided for it.
   048a-2's test `sign_in_refusals_before_dispatch` passes, to each call of `preflight` and of
   `refusal`, a request whose only member of the three is `clientCapabilities`
   `{"elicitation": {"url": {}}}`, and any stated names; its assertions do not change.

3. **The missing capability is the protocol's error -32021.** In `reject_diagnostics`, right after
   the block for `ACC-001` and `TPL-013`, where the rule is `IFC-049`, replace `t.reply` with
   `{"error": {"code": -32021, "message": "Missing required client capability", "data": {"requiredCapabilities": {"elicitation": {"url": {}}}, <reason key>: <reason>, "diagnostics": <the reply's diagnostics>}}}`,
   `<reason key>` being `mark(c, "reason")`, as the -32602 block writes its `data`. The decision
   `end-with-reason` and `end` follow as for any reason, so the `request-ended` record holds the
   reason `invalid` and the `ruleId` `IFC-049` ("recorded so"). This is the only rejection under
   IFC-049, and nothing is dispatched or elicited.

4. **One unit test pins the rules.** In `sign_in.rs`'s tests, `sign_in_request_member_refusals`,
   which calls `request_member_refusal` with a route whose `tool` is `group.sign-in.v1`, the stated
   names `browser` and `device`, a `Context` as 048a-2's test builds one, or, where it builds
   none, as `requests.rs`'s tests build one, and requests each built with `serde_json::from_value`
   from JSON, so that the null case tests decision 1, and asserts with `assert_eq!` the reason,
   rule and whole text of each: no member, IFC-049;
   `clientCapabilities` `{"elicitation": {}}`, IFC-049; `{"elicitation": {"url": true}}`,
   IFC-049; `{"elicitation": {"url": {}}}` alone, none; that with `signInReturn` null, none; with
   `signInReturn`
   `{"value": "http://127.0.0.1:49152/callback?code=not-a-code&state=not-a-state", "via": "paste"}`,
   PND-013, whose diagnostics, serialized, hold neither `not-a-code` nor `not-a-state`; with
   `signInLogin` `{"name": "phone"}`, PND-010 naming `"phone"` and `"browser", "device"`; with
   `signInLogin` `"browser"`, the first PND-010 text; with `{"name": "BROWSER"}`, PND-010; with
   `{"name": "browser"}` and with `{"name": "device"}`, none; no capability with that return and
   `{"name": "phone"}`, IFC-049; the capability with both, PND-013. Last, it calls decision 6's
   preflight, on a `State` built by hand as 048a-2's test builds one with an Agent template of `f`
   added, stored as `registration.rs` stores a template, with a route from root (`r`), since
   048a-2's Record (R1) gives none to most other callers, and a request whose `arguments.instance`
   names that template and that carries URL elicitation and `signInLogin` `{"name": "phone"}`,
   and asserts 048a-2's first invalid, the text for a Type that states no `signIn`: the names are
   not looked up for a template. Where `access::object` resolves no template, assert instead that
   the preflight gives none, and say so.

   In the shell, in `engine/crates/shell/src/server_tests.rs`, `sign_in_members_reach_the_core`,
   a plain `#[test]`, calls decision 1's `sign_in_members` with the shell's prefix and asserts with
   `assert_eq!`: for a `_meta` holding `io.modelcontextprotocol/clientCapabilities`
   `{"elicitation": {"url": {}}, "extensions": {}}`, the prefixed `signInLogin`
   `{"name": "browser"}` and the prefixed `signInReturn` `{"value": "x", "via": "paste"}`, exactly
   the members `clientCapabilities` and `signInLogin`, with those values, and no `signInReturn`;
   for a `_meta` with the two keys JSON null, and for an empty one, no member: an empty map. No
   shell test fixture holds an instance whose Type states `signIn`, so for the end-to-end answer,
   -32021 without the capability and the stub's unavailable with it, the core's answer is the case's
   (steps 3 and 18), the shell's forwarding is the unit test's, and the grep shows the two calls
   (Done when).

5. **One portable case, on 048a-2's fixture.** Add the directory
   `conformance/conventions/requests/sign-in-refusals-before-parts/` with one file, `case.yaml`: ID
   `requests-sign-in-refusals-before-parts`, `layers: [convention, interface]`,
   `promises: [IFC-049, PND-013, PND-010]`, title
   `Refuse a sign-in before any part: a missing URL elicitation capability gets -32021, and a return on a first request or an unknown login reports invalid, after the Type's check and before a conflict`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-lapse-and-conflict/fixture.json", "manifest": "../sign-in-lapse-and-conflict/manifest.json", "documents": "../sign-in-lapse-and-conflict/documents.json"}`,
   048a-2's case's three files, unchanged. Write it as JSON with two-space indentation and a final
   newline, its members in the order of `requests-sign-in-lapse-and-conflict`. It must meet 046a's
   portable-case rule as it stands.

   Below, ROOT, `META/x`, "Root asks T with A" and REFUSED(R, RULE) are as 048a-2's decision 10
   defines them. "Root asks T with A and M" is "Root asks T with A" whose request also holds the
   members M. A new step's evidence is its time, `2026-11-12T12:00:00Z`, and, as 048a-2 writes its
   new steps, `ids` the 100 strings `00000000-0000-4000-8000-` followed by the twelve-digit,
   zero-padded decimal of 100000 + 100n + k, for k = 0 to 99, `revisions` the 16 strings
   `refusal-` followed by the decimal of n, a `-` and the decimal of j, and `suffixes` the 16
   strings `r` followed by the four-digit, zero-padded decimal of 100n + j, for j = 0 to 15, where n
   is the step's number in this case; a copied step keeps its own evidence. Let S be the arguments
   `{"instance": {"kind": "Agent", "name": "signed"}}`, URL the member
   `"clientCapabilities": {"elicitation": {"url": {}}}`, RETURN the member
   `"signInReturn": {"value": "http://127.0.0.1:49152/callback?code=not-a-code&state=not-a-state", "via": "paste"}`,
   order 171b's plainly fake value, and LOGIN(X) the member `"signInLogin": X`. `Q` is the JSON
   pointer `/reply/_meta/` followed by `META/signInReturn`, its `/` written `~1`, as 048a-2 writes
   `U`. One more result recurs:
   - MISSING: `{"reply": {"error": {"code": -32021, "data": {"requiredCapabilities": {"elicitation": {"url": {}}}, "META/reason": "invalid", "diagnostics": [{"ruleId": "IFC-049", "level": "error"}]}}}, "external_intents": [], "record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "invalid", "ruleId": "IFC-049"}}]}`.

   Its steps:
   1. and 2. `requests-sign-in-lapse-and-conflict`'s steps 1 and 2, `input` and `expect`, whole,
      each with its evidence: first start and registration.
   3. Root asks `group.sign-in.v1` with S. `expect` MISSING: no capability, no part.
   4. Root asks `group.sign-in.v1` with S and `"clientCapabilities": {"elicitation": {}}`.
      `expect` MISSING: an empty `elicitation` declares form only.
   5. Root asks `group.sign-in.v1` with `{"instance": {"kind": "Agent", "name": "unset"}}`.
      `expect` REFUSED(`invalid`, `PND-010`): 048a-2's invalid comes before the capability.
   6. Root asks `group.sign-in.v1` with S and URL and RETURN. `expect` REFUSED(`invalid`,
      `PND-013`), with
   `"absent": [Q, "/reply/structuredContent/signInReturn", "/records/0/data/signInReturn", "/records/1/data/signInReturn"]`:
   nothing copies the return into the reply or a record (decision 1, PND-014).
   7. Root asks `group.sign-in.v1` with S and URL and LOGIN(`{"name": "phone"}`). `expect`
      REFUSED(`invalid`, `PND-010`).
   8. Root asks `group.sign-in.v1` with S and URL and LOGIN(`"browser"`). `expect`
      REFUSED(`invalid`, `PND-010`).
   9. Root asks `group.sign-in.v1` with S and RETURN and LOGIN(`{"name": "phone"}`). `expect`
      MISSING: the capability comes first.
   10. Root asks `group.sign-in.v1` with S and URL and RETURN and LOGIN(`{"name": "phone"}`).
       `expect` REFUSED(`invalid`, `PND-013`): the return comes before the login; with
       `"absent": ["/records/0/data/signInReturn", "/records/1/data/signInReturn", "/records/0/data/clientCapabilities", "/records/1/data/clientCapabilities", "/records/0/data/signInLogin", "/records/1/data/signInLogin"]`.
   11. `requests-sign-in-lapse-and-conflict`'s step 3, `input`, `expect` and `capture`, whole,
       with its evidence: the setup of `signed`, `$setup1`.
   12. Step 7's request. `expect` REFUSED(`invalid`, `PND-010`): invalid before conflict
       (OPS-023).
   13. Step 3's request. `expect` MISSING: before conflict too.
   14. Root asks `group.sign-in.v1` with S and URL and LOGIN(`{"name": "browser"}`). `expect`
       REFUSED(`conflict`, `PND-010`): every check passes, and 048a-2's conflict answers.
   15. to 17. `requests-sign-in-lapse-and-conflict`'s steps 7, 8 and 9, `input`, `expect` and
       `capture`, whole, each with its evidence: the setup's two run reports and its account
       check, which finds the expected account; the setup succeeds.
   18. Root asks `group.sign-in.v1` with S and URL and LOGIN(`{"name": "device"}`). `expect`
       REFUSED(`unavailable`, `PND-010`): nothing is in progress, and the stub answers.

   The route is new and has not been run, since this order's checks are not yet built. Before
   running it, check that no ID repeats across the case's steps' `ids`, the copied steps'
   included. If 046a's runner check refuses the case, if a step copied from 048a-2's case gives a
   reply other than that case states, or if a step's reply differs from what it states, stop and
   report the error: do not change the expectations to pass it, and do not change the inputs beyond
   what this decision says. No expectation is generated from the engine's output. The coverage
   index entry holds `id`, `case` (`conventions/requests/sign-in-refusals-before-parts/case.yaml`),
   `promises` and, where its neighbours in that piece hold it, `safety_rows: []`, in the members and
   order its neighbours use. Write it in a `cases` list in `conformance/manifest.yaml`, then, from
   `engine/`, run `cargo run --release -p conduit-conformance --example pieces` with no flag, which
   moves it into `conformance/conventions/requests/index.yaml` in ID order and leaves
   `manifest.yaml` byte for byte as it was.

6. **048a-2's case declares the capability.** In
   `conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml`, the requests of its
   steps 4, 12 and 29 (indexes 3, 11 and 28), root's `group.sign-in.v1` for `signed`, each gain
   the member URL after `arguments`, so that decision 2's capability does not answer them before
   the conflict, the stub and the LIF-037 conflict for a stale `expectedId` (048a-2's Record, R2
   and R3) they show. Nothing else in the case changes: not their expectations, not another
   step, not the directory's other files. Its step 6, the `unset` Agent's invalid, keeps no
   capability: 048a-2's invalid still answers it first. The case must pass as before.

7. **The expected failure's reason.** In `conformance/expected-failures.yaml`, the entry
   `requests-sign-in-return-kept-nowhere` keeps its ID, and its `reason` becomes:

   ```text
   The core's request has the member signInReturn but reads it only on sign-in.v1 (order 048a-5), so a return on another operation's request is not refused; order 048a-6 refuses it under PND-013, builds PND-014 and removes this entry.
   ```

   Nothing else in the file changes. The case still fails, now because the inspection it sends
   succeeds where it expects invalid; report the error the runner prints for it.

8. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each sentence by its text read across
   line breaks, as 048a-2 leaves it. Four places change:
   - In each of the three places, the words "invalid or conflict under PND-010" become "invalid or
     conflict", and the words "where order 048a-2's checks apply" become "where the checks of
     orders 048a-2 and 048a-5 apply"; the rest of each sentence stays, in the shell's paragraph
     and the unbuilt list the words "or conflict under LIF-037 for a stale expectedId," between
     them included.
   - In the section "Refusals and reasons (order 046a)", the sentence "IFC-049's missing
     capability, an unknown `signInLogin` and PND-013's return on a first request, each invalid,
     are not yet checked, so such a request reports conflict or unavailable instead." becomes:

     ```text
     After that invalid and before that conflict, order 048a-5 decides three more invalids, in this order: a request that does not declare URL elicitation gets the protocol's error -32021 under IFC-049, recorded invalid; one that carries `signInReturn` reports invalid under PND-013, since a retry reaches the core only as the shell's `respond` input (order 048b-relay1), never as such a request; and one whose `signInLogin` names no login its Type states reports invalid under PND-010, naming the stated logins. The shell passes `clientCapabilities` and `signInLogin` to the core, and not yet `signInReturn` (order 048a-6), so through the shell a return on a first request is not seen, and the request is answered as one without it.
     ```

   Nothing else in the file changes. After the other decisions, run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
   `promises`, to the expected failures and to a promise ID in the core's sources. Report the rows
   of IFC-049, PND-010 and PND-013 and the totals; the totals change only by what those rows change.
   IFC-049's row reads `1 | 0 | outside | core, shell | case passes`, since order 048b-relay1
   merged first.
   PND-013's row must read `2 | 1 | outside | core | case passes`, since order 048a-6 starts from
   it (Done when).

9. **The declaration.** `docs/work-orders/048a5-declared.yaml` holds two shapes: one `added` for
   `requests-sign-in-refusals-before-parts`, and one `input` for decision 6's three steps of
   `requests-sign-in-lapse-and-conflict`. The coordinator wrote it with this order; check that it
   matches and change it only to make it so.

10. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
    record changes. No fixture changes. `sign-in.v1` dispatches nothing: its stub and its text
    `not built yet, order 048a` stay, no part runs, nothing answers a sign-in action signed-in, and
    no hold or `SignInUndone` changes (048a-5b). The fixed-port refusal (048a-5c), the sign-in hook
    and the URL match (048a-5d), the message and the relayed output (048a-5e), the requestState
    (048b-relay1), polling and its endings (048a-5f to 048a-5h), and `signInReturn` on another
    operation's request or a `tasks/update`, PND-014 and the returns (048a-6) are not built. No
    shell code but decision 1's forwarding, and no CLI command (051b), relay (048b-relay, 048b-join)
    or package (050), changes. This order builds neither the access-tier binding of `sign-in.v1`
    and its refusal, which [order 048a2](048a2-build-sign-in-lapse-and-conflict.md)'s R1 left to
    048a-5, nor the follow-up case that
    [order 048a4](048a4-build-sign-in-setup-account.md)'s R2 left to 048a-5 or 048a-6, an account
    check that ends failed with a PND-005 diagnostic; the coordinator assigns each, for example to
    048a-5b or an order of its own, and records the move in order 117's entry with this order's
    pull request. Report as findings, for the coordinator: that the shell does not yet
    forward `signInReturn`, so that through the shell a return on a first request is not refused
    under PND-013 until 048a-6 forwards it; that, until the access tier binds `sign-in.v1`
    (048a-2's R1), a caller that R1's gate leaves at the stub gets unavailable without the
    capability check, where IFC-049 and OPS-023 read -32021 and invalid, the placement standing as
    written since moving the capability check ahead of R1's gate is the coordinator's decision;
    that no promise orders the invalids of PND-010, PND-013
    and IFC-049 among themselves, so this order's order is the coordinator's; that the case asserts
    no diagnostic's words, since IFC-050 leaves wording to the engine, so the unit test alone pins
    them; and any place where PND-010, PND-013 or IFC-049 left you a choice this order does not
    settle.

## Scope

Change only `engine/crates/core/src/access.rs` (decision 1), `sign_in.rs` (decisions 2 and 4, and
048a-2's test's call), `requests.rs` (the preflight's call in `invoke_inner`, decision 3 in
`reject_diagnostics`, and its struct literals), each other struct literal of `AccessRequest`, only
to add the three members as `None` (`explain.rs`, `inventory.rs`, `lifecycle.rs`, `records.rs` and
`engine/crates/conformance/src/lib.rs`), and any other file holding a struct literal of
`AccessRequest`, only to add the three members as `None`,
`engine/crates/shell/src/server.rs` (decision 1's forwarding) and
`engine/crates/shell/src/server_tests.rs` (decision 4's shell test), `engine/CONFORMANCE.md` (four
places),
`engine/STANDING.md`, written only by the tool, `conformance/conventions/requests/index.yaml`, the
new directory `conformance/conventions/requests/sign-in-refusals-before-parts/`,
`conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml` (decision 6) and
`conformance/expected-failures.yaml` (decision 7). `docs/work-orders/048a5-declared.yaml` comes
with this order; change it only as decision 9 says. `conformance/manifest.yaml` holds the new entry
only until the pieces tool moves it, and ends unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`,
`packages/`, the rest of `engine/crates/shell/`, the experiments, the case schema,
`conformance/internal-cases.yaml`, any other case or fixture, the other files of 048a-2's case
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
cargo run --release -p conduit-conformance -- .. requests-sign-in-refusals-before-parts
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-shell first_start_presents_final_lists
cargo test --release -p conduit-shell sign_in_members_reach_the_core
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports one more test passed than on the commit you start from (`main` at `8e205447`,
048b-relay1's merge, or the `main` you start from where it has moved), 0 failed, the new one being
`sign_in_request_member_refusals`; report both counts. The pieces tool prints `Nothing to move.`
The next two runner commands each end `1 cases passed`, and `requests-` gives one more case than
on the commit you start from (`main` at `8e205447`, 048b-relay1's merge, or the `main` you start
from where it has moved), with the same three expected failures (requests-repeat-failed-earlier-succeeded,
requests-repeat-failed-earlier-unanswered and requests-sign-in-return-kept-nowhere), each still
failing; report both counts. The first shell test passes: its `system.sign-in.v1` for the
Resource `management`, whose Type states no `signIn`, still reports 048a-2's invalid. The second
runs 1 test, which passes. If any other case, or any shell, CLI or experiment test, fails because a
`sign-in.v1` request now reports -32021 or invalid, stop and report it: do not change it. The
threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decision 2's call. From `engine/`, first keep a copy:

```bash
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
```

Then, by hand, in `crates/core/src/sign_in.rs`, delete the statement in the decision-6 helper that
calls `request_member_refusal` and gives its rejection, and nothing else. Then, one at a time:

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-refusals-before-parts
cp target/sign_in.rs.fixed crates/core/src/sign_in.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-refusals-before-parts
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with an error at
step 3, whose reply reports the stub's unavailable where the error -32021 is expected; after the
restore the second ends `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older
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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a5-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/conventions/requests/sign-in-lapse-and-conflict/manifest.json conformance/conventions/requests/sign-in-lapse-and-conflict/documents.json conformance/conventions/requests/sign-in-lapse-and-conflict/fixture.json conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-setup-account conformance/conventions/requests/sign-in-return-kept-nowhere conformance/conventions/inventory conformance/conventions/setup conformance/language tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5-declared.yaml: 2 shapes.`,
then `Vector delta: 4 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`: the added case
and one input change in each of decision 6's three steps. `promise_standing.py --check`,
`check_model_account.py` and `check_roadmap.py` each end `Errors: 0`. No promise's text changes,
so no row of the model-checking account goes stale; where `check_model_account.py` reports one
stale, stop and report it. The unit tests, run with the interpreter that has PyYAML, end `OK`;
report the count. The `git status` command prints nothing.

The S1 short-form spike reads `spec/` and `conformance/fixtures/s1/` and depends on the core, so
the local gate runs it for this change, as spec batch 1's Gate G asks. From
`experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`.

Each of these prints 1. The `tr` checks read the file with its line breaks as spaces, so a rewrap
hides nothing. The IFC-049 check reads `core, shell`, since order 048b-relay1 merged first:

```bash
grep -c 'fn sign_in_request_member_refusals' engine/crates/core/src/sign_in.rs
grep -c 'rename = "clientCapabilities"' engine/crates/core/src/access.rs
grep -c 'rename = "signInLogin"' engine/crates/core/src/access.rs
grep -c 'rename = "signInReturn"' engine/crates/core/src/access.rs
grep -c '"id": "requests-sign-in-refusals-before-parts"' conformance/conventions/requests/index.yaml
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
grep -c 'reads it only on sign-in.v1 (order 048a-5)' conformance/expected-failures.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'After that invalid and before that conflict, order 048a-5 decides three more invalids'
grep -c '^| IFC-049 | interface | must | 1 | 0 | outside | core, shell | case passes |$' engine/STANDING.md
grep -c '^| PND-013 | convention | must | 2 | 1 | outside | core | case passes |$' engine/STANDING.md
grep -c 'fn sign_in_members_reach_the_core' engine/crates/shell/src/server_tests.rs
```

Each of these prints 3, the second for `sign_in_members`'s definition, `call_tool`'s call and
`record_rejection`'s call:

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -o 'where the checks of orders 048a-2 and 048a-5 apply' | wc -l
grep -o 'sign_in_members(' engine/crates/shell/src/server.rs | wc -l | tr -d ' '
```

This prints `group.sign-in.v1 {"elicitation": {"url": {}}} {"elicitation": {"url": {}}} {"elicitation": {"url": {}}} None 32`:

```bash
python3 -c "import json; s = json.load(open('conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml'))['steps']; r = [s[i]['input']['request'] for i in (3, 11, 28, 5)]; print(r[0]['tool'], json.dumps(r[0]['clientCapabilities']), json.dumps(r[1]['clientCapabilities']), json.dumps(r[2]['clientCapabilities']), r[3].get('clientCapabilities'), len(s))"
```

Each of these prints 0:

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'invalid or conflict under PND-010'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'where order 048a-2.s checks apply'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'are not yet checked, so such a request reports conflict or unavailable instead'
grep -c 'signInReturn' engine/crates/shell/src/server.rs
git diff HEAD --stat -- spec IN_FORCE.md | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the rows decision 8 names with the totals, the function names and signatures of
decision 2, each struct literal of `AccessRequest` you changed, and the error the runner prints for
the expected failure.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.

## Record

Run on 2026-11-19, from about 02:29 to 03:20 UTC, on the Mac from `main` `10ad9931` (the refresh of
orders 048a-5 and 048a-6), in two rounds: the worker (1406 s) and a fresh Codex verifier (347 s),
then the worker again (972 s) and a second fresh Codex verifier (368 s). The change is the 16 files
of Scope. `AccessRequest` gains `clientCapabilities`, `signInLogin` and `signInReturn`, and its nine
other struct literals set them to `None`; the shell's `sign_in_members(meta: &Value, prefix: &str)`
forwards the first two through `call_tool` and `record_rejection`. `sign_in.rs`'s preflight, after
048a-2's invalid and before its conflicts, checks in turn the URL capability (IFC-049), a first
request's return (PND-013) and the asked-for login (PND-010), through `stated_login_names`,
`request_member_refusal` and `refusal`, with decision 2's words; `reject_diagnostics` answers the
missing capability with the error -32021 and `requiredCapabilities`. The unit test
`sign_in_request_member_refusals` is new (8 to 9 sign-in tests on `10ad9931`), as is the 18-step
case `requests-sign-in-refusals-before-parts` (1774 unique IDs, its six copied steps equal to their
sources); steps 4, 12 and 29 of 048a-2's case declare the capability, `requests-` went from 66 to 67
cases with the same three expected failures, and `requests-sign-in-return-kept-nowhere` now fails
with `unexpected field /reply/_meta/io.github.shrav-k.conduit~1outcome`. With decision 2's call
deleted, the new case failed at step 3 with the stub's unavailable, and passed after the restore.
Round 1's worker applied every decision as written. The first verifier found one must: the three new
diagnostics name the tool and the cause but not the instance the request named, against IFC-050's
"what the request named"; and three shoulds on the case, which the second verifier repeated (below):
`/reply/inputRequired`, `external_intents` and the qualified `signInReturn` key. Since the order
prescribes those words, the second worker changed nothing and reported the remedies for the
coordinator. The second verifier found two musts, the same wording and an idempotency key reused
with another digest, which returns OPS-010 conflict before the sign-in invalids though OPS-023 puts
invalid first, and four shoulds on the case: the MISSING steps do not assert `/reply/inputRequired`
absent nor the access decision's `data.permitted`, the PND-013 and PND-010 steps assert no
`external_intents`, and the return steps check only the bare `signInReturn` key in record data.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on the branch before
its rebase. It ran each command of "Done when" (the core's sign-in tests 9 passed, `Nothing to
move.`, both cases `1 cases passed`, `requests-` 67 passed with the three expected failures, both
shell tests, the vectors and the whole engine, 0 failed), the Python checks (Errors: 0 each, 129
tool tests OK), `vector_delta.py` (4 changes; 0 flips; 0 undeclared; 0 errors), the spike (9
passed), the order's greps (each 1, 3, 0 or the line expected) and a privacy grep of the changed
files; on a scratch copy it deleted decision 2's call (step 3 failed) and restored it (passed). It
merged the branch with `origin/main` at `fa8ca9ae` in a scratch tree: only `engine/STANDING.md`
conflicted, and with it written again the core's sign-in tests gave 10, both cases passed and
`requests-` gave 67 with the three expected failures. It found no must and four shoulds:

- R1: the branch conflicts with `origin/main` in `engine/STANDING.md`, which 048b-relay2 changed
  (the IFC-045 row and the totals); rebase, write the file with the tool and check the IFC-049,
  PND-013 and IFC-045 rows.
- R2: the three new diagnostics, like 048a-2's, name the tool and the cause but not the instance
  the request named (IFC-050, ruling 52); not a fix to this work, since the order prescribes the
  words: in 048a-5b or an order of its own, `rejection()` takes the instance's label and writes
  `{tool} reports {reason} under {rule} for {Kind}/{name}: {cause}` for all of `sign-in.v1`'s
  preflight texts, 048a-2's included, with both unit tests updated.
- R3: the case does not fully prove "before any part": in 048a-5b, the first order that dispatches
  a part, add `"external_intents": []` to each REFUSED expectation, `/reply/inputRequired` to the
  absent list of the MISSING steps, and the qualified record-data pointers for `signInReturn` to
  steps 6 and 10, each an expect-only change, declared as such.
- R4: a keyed `sign-in.v1` request that reuses a key with another digest gets OPS-010 conflict
  before the sign-in invalids, from the preflight's placement that 048a-2 chose; for the
  coordinator, beside the access-tier finding: the order that binds `sign-in.v1`'s checks at the
  access tier runs the invalids before the OPS-010 and OPS-032 key conflicts, with a case step
  that reuses a setup's key without the capability and expects MISSING.

The fixer rebased the branch onto `origin/main` at `7eed5a4d` (#381) with the work-order identity.
Only `engine/STANDING.md` conflicted; the fixer took this order's side and wrote the file again
with `promise_standing.py --write` (Errors: 0), which gives IFC-049
`1 | 0 | outside | core, shell | case passes`, PND-013 `2 | 1 | outside | core | case passes`,
IFC-045 `1 | 0 | reason | core | case passes` and the totals `case passes | 129 | 153 | 27 | 309`
and `expected failure | 0 | 1 | 0 | 1`. That is R1. R2, R3 and R4 ask for no change in this order,
each by the reader's own fix, and the fixer made none: they stand here for the coordinator, R2
and R3 toward 048a-5b and R4 beside the access-tier finding. They answer the verifiers' open
points too: R2 is both verifiers' must on the wording, R4 the second verifier's must on the key
conflict, and R3 covers three of its shoulds, though not the return should's asks to assert
the records at the positions the absence checks use and to check step 9 too. The fourth, that the
MISSING steps project no `data.permitted: true` on the access decision, the reader did not raise;
these go to the coordinator with R3. On the rebased tree the fixer ran, from `engine/` with
`RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4`, each command of "Done when": `cargo fmt --check`
and `cargo clippy --all-targets -- -D warnings` (exit 0), the core's sign-in tests (10 passed, 0
failed), the pieces tool (`Nothing to move.`), both cases (`1 cases passed` each), `requests-`
(`67 cases passed; 3 expected failures`, the three of decision 7), both shell tests (1 passed
each), the vectors (2 passed, 0 failed) and the whole engine (exit 0; 364 passed, 0 failed, 4
ignored). With decision 2's call deleted, the new case printed `FAIL 0 cases passed; 1 failed`,
step 3 ending with reason unavailable, and after the restore with `cp` `1 cases passed`. From the
root, `check_schemas.py`, `set_names.py --check`, `check_rulings.py`, `check_docs.py`,
`check_roadmap.py`, `check_model_account.py`, `check_private_paths.py` and
`promise_standing.py --check` gave Errors: 0 each, the tool tests Ran 129 tests, OK, and
`vector_delta.py origin/main HEAD --declared docs/work-orders/048a5-declared.yaml`
`4 changes; 0 flips; 0 undeclared; 0 errors.`, exit 0. The order's greps printed each 1, 3, 0 or
the line expected, `git diff --check origin/main...HEAD` was clean, the branch's diff holds the 16
files of Scope and this Record, and the spike's `tests/spike.rs` gave `ok. 9 passed; 0 failed`.
The gate's run on this order's squash, which this merge waits on, is the coordinator's at the
merge.
