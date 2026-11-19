# Work order 048a6 (E7a-6): a sign-in return where none is taken reports invalid and is kept nowhere

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the case you add and the case whose expected
failure you lift, with the promises each tests and whether it passes. This order has network access
only because the engine's tests bind loopback ports, which the sandbox refuses without it; use it
for nothing else, add no crate, and reach no other machine. Run one cargo command at a time. Never
print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and
never read or print a key, a token or any credential. Do not run `claude`, `codex` or any client,
and sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry for 048a-2, and the coordinator's cut
that [order 048a2](048a2-build-sign-in-lapse-and-conflict.md) records under "Who decided what",
split 048a-2's entry into orders that follow one another, since an order holds one behavior and
stays small ([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). The last
of them is "048a-6, the returns (PND-013, PND-014), lifting the expected failure
`requests-sign-in-return-kept-nowhere`". Built whole, the returns are three behaviors: refusing a
return where none is taken; checking a return on a retry of `sign-in.v1` and passing an accepted
one on to `sign-in-hook.v1`; and checking a redirect return as a URL and rebuilding it. That is
about 0.5 to 0.7k changed lines with several cases, so the coordinator cuts 048a-6 again, keeping
its name for the first part (see "Who decided what"):

- **048a-6, this order:** `signInReturn` on a request whose operation is not `sign-in.v1` reports
  invalid under PND-013 and is kept nowhere (PND-014). That lifts the expected failure. A
  `sign-in.v1` request stays with order 048a-5's refusal of a return on a first request.
- **Proposed, not yet written:** 048a-7, an accepted return passed on: on a retry of `sign-in.v1`
  that IFC-049 accepts, PND-013's first three checks (`not-accepted`, `already-returned`,
  `malformed` by size and characters), a page login's code passed on in a `sign-in-hook.v1` retry
  that carries the call's own `requestState`, `accept` and `signInReturn` `{value}`, recorded
  without it, and `signInLogin`'s `returned` as `delivered`, `undelivered` or `refused` with its
  reason; 048a-8, a redirect return: PND-013's fourth check (`malformed` as a URL,
  `address-mismatch`, `state-mismatch`, `issuer-mismatch`), the value rebuilt with only `code`,
  `state`, `iss` and `scope` or the error members, passed to `sign-in-hook.v1` in 048a-7's retry,
  and `error` in `signInLogin`.

The chain: this order follows 048a-5 directly, since it needs nothing from 048a-5b to 048a-5h,
and runs beside 048a-5b (below, "When this order starts", says what each changes that the other
touches); 048a-7 follows 048a-5h and this order, since it needs 048a-5f's retries, which reach the
core as order 048b-relay1's respond input; 048a-8 follows 048a-7; and 048b-join follows 048a-8 and
048b-relay2. This order builds neither of the two after it.

**What it builds.** One behavior. PND-013's `observable`: "signInReturn on a first sign-in.v1
request, any other operation's request, or a tasks/update is not a return: after access and before
dispatch the request reports invalid in OPS-023's order, with a diagnostic naming PND-013 and the
key, holding no value." PND-014's `then` forbids that "The value appears in a result,
structuredContent, response _meta, notification, diagnostic, log or record, is among the values
REC-002 keeps, reaches an Agent, or stays in the engine's memory after ... the request that carried
it has ended", and its `observable` repeats: "A request that carries signInReturn where PND-013
takes no return reports invalid, with a diagnostic naming PND-013 and the key, and holds no value."
IFC-049: "`META/signInReturn` holds {value, via} in an Interface's retry _meta ... It is kept
nowhere (PND-014). PND-013 alone accepts it, only from the Interface that received the sign-in, in
its bound invocation." (The quote writes the `_meta` key as `META/signInReturn`, the key
`signInReturn` under the prefix that `spec/names.yaml` sets.) REC-002: "A return under PND-013 is
none of the _meta values this rule keeps." No promise changes: the texts state all of it, and this order builds
it. The core has no task requests (order 048a-2 reads none), so the `tasks/update` half is not
built here.

[Order 171b](171b-sign-in-return-and-holds.md) wrote the case
`requests-sign-in-return-kept-nowhere` (PND-014, PND-013), which sends `signInReturn` on root's
inspection and expects invalid with a diagnostic naming PND-013 and seven absent pointers. It is an
expected failure. On `main` the core's request denies unknown members and has none for
`signInReturn`, so the runner stops at step 2; after order 048a-5, which adds the member and reads
it only on `sign-in.v1`, the inspection it sends succeeds where invalid is expected (048a-5's
Record: the step fails on an unexpected field, the reply's `_meta` key for `outcome`). This order
gives the member a type that withholds its value, decides such a request, removes the expected
failure, and adds a case for a call to an Agent, a held request and a refused one, with a first
`sign-in.v1` left to 048a-5's refusal. Since 048a-5 left the key to this order, the shell does not
yet pass `signInReturn` to the core, so this order also forwards it in the shell's invoke input,
and nowhere else, with one server test.

**What orders 048a-2 to 048a-5 bring, which this order assumes.** Orders 048a-2 (#364), 048a-3
(#367), 048a-4 (#371) and 048a-5 (#382) have merged, each with the amendments its Record states,
and so have orders 048b-relay1 (#373) and 048b-relay2 (#377). It assumes, as their orders and
Records state them:

- From 048a-2: `engine/crates/core/src/sign_in.rs`; its preflight of `sign-in.v1`'s invalid and
  conflict, and, by its Record's amendment R3, the conflict under LIF-037 for a stale `expectedId`,
  called in `invoke_inner` after `lifecycle::preflight` and `records::preflight`, which by its
  Record's R1 applies only for root or for an Interface on its own behalf through an Agent's owning
  Fleet, any other caller getting the stub's unavailable; and
  the fixture of `requests-sign-in-lapse-and-conflict`, with the keyed `signed-server`, the Type
  `signed-agent.v1`, the Fleet `group` and the Agent `signed` with the account `primary-account`.
- From 048a-3: the block in `invoke_inner`, after the block that answers a kept repeat from its
  outcome, that holds a request to an instance awaiting sign-in (decision `hold-for-sign-in`); and
  the case `requests-sign-in-hold`, with its `manifest.json`, `documents.json` and `fixture.json`
  (the Operations `ping.v1` and `work.v1`, the second stating `startsModelWork: true`, and root's
  Conduit `to-signed` listing both), and its definitions ROOT, `META/x`, `U`, `I`, `P`, "Root asks
  T with A", "a completion" and LAPSE, as 048a-2's decision 10 and 048a-3's decision 6 give them.
- From 048a-4: setup's account check (decision `dispatch-account-check`); PND-011's hold
  (`request_is_held_for_sign_in_undone`), decided in `invoke_inner` beside 048a-3's; and the one
  step its account check inserts in `requests-sign-in-hold`, so that the first nine of its 17
  steps are: first start; registration; root's `group.set-up.v1` of `signed`, captured as
  `$setup1`; two run reports, the second dispatching the account check, captured as `$account0`;
  the account check's completion, which ends the setup succeeded; root's `access.open.v1` of
  `to-signed`; root's `to-signed.work.v1`, captured as `$call1`; and its completion LAPSE, which
  raises the sign-in action and begins the hold.
- From 048a-5 (its file `048a5-build-sign-in-refusals-before-parts.md`, the first of its eight
  parts):
  - (a) The member: `AccessRequest` has, after `authority_readable`, the members
    `client_capabilities` (`clientCapabilities`), `sign_in_login` (`signInLogin`) and
    `sign_in_return` (`signInReturn`), each `Option<Value>` with `#[serde(default)]` and that serde
    name; absent or JSON null, a member is not carried. Each struct literal of `AccessRequest` sets
    the three to `None`. Nothing copies `sign_in_return` into the route, a record, a decision, the
    reply or the state, and nothing reads it but whether it is carried.
  - (b) The refusal: `request_member_refusal` in `engine/crates/core/src/sign_in.rs`, which the
    decision-6 helper calls for `sign-in.v1` right after 048a-2's invalid and before its conflicts,
    decides in order the missing capability (IFC-049, the error -32021), a return on a first
    request (PND-013: every `sign-in.v1` request that 048a-2's checks reach (its Record's R1) and
    that carries `signInReturn`; any other caller gets the stub's unavailable, as 048a-5 leaves it)
    and an unknown login (PND-010). Its unit test `sign_in_request_member_refusals` builds its
    requests from JSON with `serde_json::from_value::<AccessRequest>`, some carrying a
    `signInReturn` value. A retry reaches the core only as order
    048b-relay1's `respond` input, so 048a-5's refusal stays the refusal of every `sign-in.v1`
    request that 048a-2's checks reach (its Record's R1) and that carries `signInReturn`; any other
    caller gets the stub's unavailable, as 048a-5 leaves it. This order relies on nothing else of
    the retry.
  - (c) The case `requests-sign-in-refusals-before-parts`, with
    `promises: [IFC-049, PND-013, PND-010]`, on 048a-2's fixture, and the capability added to
    steps 4, 12 and 29 of `requests-sign-in-lapse-and-conflict`, now of 32 steps, as 048a-5's
    decision 6 gives them (step 29 is the stale `expectedId` of 048a-2's Record, R2). This order
    reads none of that case.
  - (d) The stub: C11's entry `("sign-in.v1", "048a", "PND-010")` stays; 048a-5b removes it. This
    order reads no dispatch of `sign-in.v1`.
  - (e) The expected failure: `requests-sign-in-return-kept-nowhere` keeps its entry, with the
    reason "The core's request has the member signInReturn but reads it only on sign-in.v1 (order
    048a-5), so a return on another operation's request is not refused; order 048a-6 refuses it
    under PND-013, builds PND-014 and removes this entry."
  - (f) In `engine/CONFORMANCE.md`, section "Refusals and reasons (order 046a)", the sentence
    beginning "After that invalid and before that conflict, order 048a-5 decides three more
    invalids, in this order", which says that a request carrying `signInReturn` reports invalid
    under PND-013 "since a retry reaches the core only as the shell's `respond` input (order
    048b-relay1), never as such a request"; and, in the same paragraph, 048a-5's sentence "The
    shell passes `clientCapabilities` and `signInLogin` to the core, and not yet `signInReturn`
    (order 048a-6), so through the shell a return on a first request is not seen, and the request
    is answered as one without it."
  - (g) In `engine/STANDING.md`, PND-013's row is `2 | 1 | outside | core | case passes`
    (048a-5's case and `requests-sign-in-return-kept-nowhere`; `sign_in.rs` names PND-013), and
    PND-014's is `1 | 1 | outside | none | expected failure`, or `core` in place of `none` where a
    comment of 048a-5 names PND-014 in the core's sources (at `9e2a1410` it is `none`).
  - (h) In `engine/crates/shell/src/server.rs`, the function `sign_in_members`, which forwards
    `clientCapabilities` and `signInLogin` from the request's `_meta` into the core's request and
    is called in `call_tool` and in `record_rejection`; the file names `signInReturn` nowhere. In
    `server_tests.rs`, its test `sign_in_members_reach_the_core`, which asserts that the function
    leaves `signInReturn` out; it stays as it is and passes.
  - It changes neither the first nine steps of `requests-sign-in-hold` nor its three fixture files.

Where an order merged otherwise than this says, apply the nearest sound reading of each decision
below to what merged, and say so. Where a Record names a function or a member, use that name.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`9e2a1410`, after orders 048a-2 to 048a-5, 048b-relay1, 048b-relay2 and 051b; find each place by
the text quoted with it.

- `AccessRequest` (`engine/crates/core/src/access.rs:17`) derives `Clone`, `Debug` and
  `Deserialize`, with `#[serde(deny_unknown_fields)]`; its members are `identity`, `tool`,
  `arguments`, `serving`, `readable` and `authority_readable`, then 048a-5's three (assumption
  (a)), `sign_in_return` at line 32. `Input::Invoke` and `Input::Access`
  (`lib.rs`) hold one, and `Input` derives `Debug`. It does not derive `Serialize`. Order
  048b-relay1's `Input::Respond` holds none; it holds a retry's whole `_meta` as `meta`.
- `invoke_inner` (`requests.rs:919`) writes the access decision's record with
  `record(t, a, c, "access-decision", access_data)?;`, pushes the decision `request-access`, and
  then ends a request the access decision did not permit in the block
  `if let Some(reason) = v.reason {` ... `return reject(t, a, c, &route, reason, rule, None, v.restriction.as_ref());`.
  A schema failure of the arguments ends there too, as invalid under OPS-033. Right after that
  block comes `let target = crate::registration::publication_object(`. Neither the route, the
  access decision's `data` nor the explanation that `explain::access_route` builds reads any
  member of the request but `identity`, `tool`, `arguments` and `serving`.
- `reject_diagnostics` (`requests.rs:639`) writes the reply with `envelope`, takes specific
  diagnostics where they are given, pushes the decision `end-with-reason` citing the rule,
  `OPS-005`, `OPS-023` and `IFC-025`, and ends the request with a `request-ended` record whose
  `data` is the route with `ruleId`, less `filters`. `explain`'s text block (`explain.rs:917`)
  copies each error diagnostic's message into the reply's first `content` entry.
- `mark(c, key)` (`requests.rs:24`) gives a `_meta` key under the prefix that `spec/names.yaml`
  sets. `Transition` (`lib.rs:281`) derives `Debug` and `Serialize`.
- Since 048a-5 the shell passes `clientCapabilities` and `signInLogin` through `sign_in_members`
  (`server.rs:566`), and not `signInReturn`, which this order forwards. `call_tool`
  (`server.rs:398`) reads the request's `_meta` into `meta`, checks `serving` and
  `idempotencyKey`, and, as order 048b-relay1 rewrote it, builds the core's input in
  `let input = match retry {`: a retry's `respond` input in the arm `Ok(Some(input)) => input`, and
  otherwise the invoke input in the arm `Ok(None) =>`, whose line begins
  `let mut input = json!({"kind":"invoke","request":{` (line 472) and which 048a-5's
  `.extend(sign_in_members(&meta, prefix));` (line 476) extends before the arm returns `input`.
  `record_rejection` (`server.rs:582`) builds its own input, in the line that begins
  `let mut input = json!({"kind":"invoke","request":{` (line 597), from chosen members of the
  body's `_meta` and `sign_in_members`, never the whole of it. `Input::Access` evaluates
  access only and dispatches nothing.

**When this order starts.** Order 048a-5 has merged to `main` (#382). Before you change anything,
check from the repository root that you find what 048a-2 to 048a-5 give, and stop, change nothing
and report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/048a5-build-sign-in-refusals-before-parts.md
grep -c 'rename = "signInReturn"' engine/crates/core/src/access.rs
grep -c -E 'sign_in_return: Option<(serde_json::)?Value>' engine/crates/core/src/access.rs
grep -c 'fn request_member_refusal' engine/crates/core/src/sign_in.rs
grep -c 'fn sign_in_request_member_refusals' engine/crates/core/src/sign_in.rs
grep -c '"id": "requests-sign-in-refusals-before-parts"' conformance/conventions/requests/index.yaml
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
grep -c '"id": "requests-sign-in-return-kept-nowhere"' conformance/expected-failures.yaml
grep -c 'reads it only on sign-in.v1 (order 048a-5)' conformance/expected-failures.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'After that invalid and before that conflict, order 048a-5 decides three more invalids'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and not yet .signInReturn. (order 048a-6)'
grep -rl '"hold-for-sign-in"' engine/crates/core/src | wc -l
grep -rl '"dispatch-account-check"' engine/crates/core/src | wc -l
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-hold/case.yaml"))["steps"]; print([x["input"].get("request", {}).get("tool", x["input"]["kind"]) for x in s[:9]])'
grep -c '^| PND-013 | convention | must | 2 | 1 | outside | core | case passes |$' engine/STANDING.md
grep -c -E '^\| PND-014 \| convention \| must not \| 1 \| 1 \| outside \| (none|core) \| expected failure \|$' engine/STANDING.md
grep -c 'signInReturn' engine/crates/shell/src/server.rs
grep -o 'sign_in_members(' engine/crates/shell/src/server.rs | wc -l | tr -d ' '
test -d conformance/conventions/requests/sign-in-return-not-taken; echo "dir $?"
```

They print `1` (048a-5's merge), `1` and `1` (assumption (a)'s member), `1` and `1` (assumption
(b)'s function and test), `1` (assumption (c)'s case), `1` (assumption (d)'s stub, or `0` where
048a-5b has merged before this order starts: either is sound, since this order reads no dispatch of
`sign-in.v1`; say which), `1` and `1` (assumption (e)'s entry and its reason), `1` and `1`
(assumption (f)'s two sentences), `1`, `1`,
`['first-start', 'register-project', 'group.set-up.v1', 'setup-report', 'setup-report', 'complete', 'access.open.v1', 'to-signed.work.v1', 'complete']`,
`1` and `1` (assumption (g)'s rows; say which form PND-014's row has), `0` and `3` (assumption
(h): the definition, `call_tool`'s call and `record_rejection`'s call) and `dir 1`. Where one of
048a-5b to 048a-5h merged before this order starts, assumption (f)'s sentence and PND-013's counts
are as that order's Record states; report the rows you find in place of the start checks of
assumption (f) and PND-013's row, and in place of the done-when rows that check 048a-5's sentence
and PND-013's row. Then, still before any change, run from `engine/`, one at a time,
`cargo run --release -p conduit-conformance -- .. requests-sign-in-` and
`cargo run --release -p conduit-conformance -- .. requests-`, and keep each last line: "Done
when" compares with them (at `9e2a1410` they end `4 cases passed; 1 expected failures` and
`67 cases passed; 3 expected failures`). Orders that land beside this one (048a-5b, 049a-1, 050
and others): whichever lands second keeps both changes and writes `engine/STANDING.md` again with
the tool. Order 048b-relay1 has merged (#373) and rewrote `call_tool`, `Input` and `Transition`;
decision 8 says where the forwarding goes, and the start checks of assumption (h) print `0` and
`3` at `9e2a1410`.

Order 048a-5b runs beside this one, and neither reads the other's behavior: 048a-5b sends no
`signInReturn`, and this order reads no dispatch of `sign-in.v1`. Where 048a-5b has merged first,
"the stub's unavailable" below, for a `sign-in.v1` caller that 048a-2's checks do not reach, reads
as 048a-5b's dispatch of the account check (decision 9's finding). Each may change what the other
touches in these places, and whichever lands second keeps both:

- `engine/crates/core/src/requests.rs`: 048a-5b changes C11's stub and its text, the
  engine-operation guard, the dispatch arm, the part function's parameter and the raise's calls;
  this order adds one block in `invoke_inner` after the access decision's block, and one test.
- `engine/crates/core/src/sign_in.rs`: 048a-5b's decisions 1 to 7 and 14 (14: every preflight
  text names the instance, and `sign_in_request_member_refusals`' texts and calls change to
  match); this order changes a line there only where decision 1 finds a literal to wrap (none at
  `9e2a1410`). Where 048a-5b landed first, the done-when's `sign_in_request_member_refusals` run
  is 048a-5b's form of the test.
- `engine/crates/shell/src/server.rs`: 048a-5b rewords the comment above
  `EXECUTABLE_ENGINE_OPERATIONS` (its decision 11); this order adds decision 8's forwarding in
  `call_tool`.
- `engine/CONFORMANCE.md`: in the section "Refusals and reasons (order 046a)", 048a-5b rewrites
  the paragraph's first sentence, "The unavailable stub of `sign-in.v1` now carries a diagnostic
  but remains unbuilt.", and this order its last sentence, each rewrapping that paragraph; 048a-5b
  also changes DL18's and DL19's rows and two other paragraphs, and this order adds a section at
  the file's end.
- `engine/STANDING.md`, written by the tool, and `conformance/conventions/requests/index.yaml`,
  where each order's pieces tool adds one entry in ID order.

048a-5b also changes the cases `requests-sign-in-lapse-and-conflict` and
`requests-sign-in-refusals-before-parts`, which this order does not change, and this order changes
`conformance/expected-failures.yaml`, which 048a-5b does not change.

**Who decided what.** PND-013, PND-014, IFC-049, REC-002 and OPS-023 decide that a return is taken
only on a retry of `sign-in.v1` that IFC-049 binds, that any other request carrying `signInReturn`
reports invalid after the access decision and before dispatch with a diagnostic naming PND-013 and
the key, and that the value appears nowhere. Everything else here is the coordinator's, each open to
the owner's strike ([ruling 48](../proposals/ruling-48/README.md)): the cut of 048a-6 above into
this order and the two proposed after it (FR-264), with this order after 048a-5, 048a-7 after
048a-5h and this order, and 048b-join after the last and 048b-relay2; the member's form, 048a-5's
`sign_in_return` with its type changed to one whose `Debug` withholds the value; the shell's
forwarding of `signInReturn` in `call_tool`'s invoke input alone, outside `sign_in_members`, so that
no rejection record's input holds it, and its server test; reading "any other operation's request"
as every request whose operation is not `sign-in.v1`, an Agent's onward request included; leaving
every `sign-in.v1` request that 048a-2's checks reach (its Record's R1) and that carries
`signInReturn` to 048a-5's `request_member_refusal` (any other caller gets the stub's unavailable,
as 048a-5 leaves it), which a retry never reaches, since the shell issues, keeps and checks a
sign-in's `requestState` (order 048b-relay1) and passes a retry to the core only as its `respond`
input, so that this order reads no `requestState`; deciding this invalid, for a request whose
operation is not `sign-in.v1`, first among the invalids after the access decision, before
`answer.v1`'s invalids, every conflict, a kept repeat's answer and the holds of PND-005 and PND-011,
and after a schema failure, which the access decision reports; the diagnostic's words,
naming the route's target as 048a-5b's decision 14 names the instance; the decision's name; the unit test; the case, its reuse of 048a-3's files, route, IDs and times; and the
new section of `engine/CONFORMANCE.md`. Apply them as written and report them under `decisions` as
the coordinator's.

**Read first:**

1. In `spec/`: PND-004, PND-010, PND-013 and PND-014 (`spec/promises/pending.yaml`); IFC-006,
   IFC-016 and IFC-049 (`spec/interface/implementation.yaml`), their sentences on `signInReturn`
   and task requests; OPS-023 (`spec/conventions/operations.yaml`); REC-002
   (`spec/conventions/records.yaml`), its last sentence.
2. [Order 171b](171b-sign-in-return-and-holds.md), its decision 8 and its Record;
   [order 048a3](048a3-build-sign-in-hold.md), decision 6; and orders 048a-4 and 048a-5, whole,
   with their Records.
3. The cases `requests-sign-in-return-kept-nowhere` and `requests-sign-in-hold`, and
   `conformance/conventions/explain/refusal-tiers/case.yaml`, its step 3.
4. `conformance/README.md`, its sections "Assertions", "Portable cases" and "Expected failures".
5. `engine/crates/shell/src/server.rs`, `call_tool`, as order 048b-relay1 rewrote it, and
   `record_rejection`, and
   `engine/crates/shell/src/server_tests.rs`, `shell_rejection_carries_diagnostic` and 048a-5's
   `sign_in_members_reach_the_core`.

**Size: S**: about 80 to 140 changed lines of Rust with one unit test and one server test, one new
case of 14 steps that reuses 048a-3's fixture, one expected failure removed, one section in
`engine/CONFORMANCE.md` and one sentence of it replaced.

## The decisions it applies

1. **The core's request carries the return, and withholds it.** In `access.rs`, add right before
   `AccessRequest`:

   ```rust
   /// PND-013 and PND-014: the value of an Interface's `signInReturn`. Only PND-013's
   /// checks read it, and its `Debug` form withholds it.
   #[derive(Clone, Deserialize)]
   #[serde(transparent)]
   pub struct SignInReturn(pub Value);

   impl std::fmt::Debug for SignInReturn {
       fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
           f.write_str("SignInReturn(withheld)")
       }
   }
   ```

   and change the type of 048a-5's member `sign_in_return` from `Option<Value>` to
   `Option<SignInReturn>`, keeping its attributes and its place, so that it reads

   ```rust
       pub sign_in_return: Option<SignInReturn>,
   ```

   Any JSON value is taken, and `null` reads as absent, as before. Where `AccessRequest` derives
   `Serialize` on your tree, also give the member `skip_serializing`; `SignInReturn` implements no
   `Serialize`. The struct literals' `sign_in_return: None`, which 048a-5 wrote, stay as they are.
   In `sign_in.rs`, 048a-5's tests build their requests
   (from JSON with `serde_json::from_value`, at `sign_in.rs:498` and `sign_in.rs:633` on
   `9e2a1410`), so nothing there changes; only where your tree writes a `Some(value)` for
   `sign_in_return` in a struct literal, wrap it as `Some(SignInReturn(value))`, writing
   `crate::access::SignInReturn` in each, or adding one `use crate::access::SignInReturn;` to the
   test module, and change nothing else in the test. Nothing else that reads
   `sign_in_return` changes: 048a-5 reads only whether it is carried. List in your report each line
   you changed for the type.

2. **A request that takes no return reports invalid.** In `invoke_inner`, right after the block
   that ends a request the access decision did not permit (`if let Some(reason) = v.reason {` ...
   `}`) and before `let target =`, add one block whose first line is exactly
   `if request.sign_in_return.is_some() && route["operation"] != c.qualified("sign-in.v1") {`.
   In the block:
   - Push one decision named `refuse-sign-in-return`, citing `PND-013`, `PND-014` and `OPS-023`,
     with no implementation choice and the facts `{}`.
   - Call `reject_diagnostics` with the reason `invalid`, the rule `PND-013`, no invocation, no
     restriction, and one specific diagnostic,
     `{"ruleId": "PND-013", "level": "error", "message": {"text": T}}`, where `T` is
     `{tool} reports invalid under PND-013 for {kind}/{name}: {key} is a return only on a retry of sign-in.v1 whose requestState IFC-049 accepts, so this request carries none; its value was not read, kept or passed on.`,
     `{tool}` the route's `tool`, `{kind}/{name}` the route's target, the `kind` and `name` of the
     object the route's `target` names, which the access decision has resolved by the time this
     block runs, and `{key}` `mark(c, "signInReturn")`. So the text names the instance the request
     named, as 048a-5b's decision 14 makes every `sign-in.v1` preflight text do (IFC-050, ruling
     52).
   - Return. Nothing is dispatched, raised or kept (OPS-025: a later request is new).

   So a request the access decision did not permit, a schema failure among them, keeps its reason
   (OPS-023), and every other request carrying `signInReturn` whose operation is not `sign-in.v1`
   reports this invalid, whatever else applies to it: a repeat with a key, a request to a held
   instance, an Agent's onward request. Every `sign-in.v1` request that 048a-2's checks reach (its
   Record's R1) and that carries `signInReturn` is decided by 048a-5's `request_member_refusal`,
   after 048a-2's invalid and the missing capability, as 048a-5 orders them; any other caller gets
   the stub's unavailable, as 048a-5 leaves it. A retry never reaches it, since the shell passes a
   retry to the core only as order 048b-relay1's `respond` input. This order reads no
   `requestState`, and a retry's return is not read, checked or passed on in this order (048a-7 and
   048a-8).

3. **One unit test pins the rule.** In `requests.rs`'s tests, add
   `sign_in_return_not_taken_is_kept_nowhere`. Let `R` be the value at
   `/requests/0/call/request/signInReturn` of
   `conformance/conventions/requests/sign-in-return-kept-nowhere/requests.json`, written as a
   `json!` literal, and `S` the three strings: `R`'s `value` whole, and the values of its `code` and
   `state` parameters. With `repeat_fixture(false)`:
   - The fixture's request, through `invoke_repeat` on the fixture's state, gives the transition
     `pending`, whose reply has `pending` `true`, as
     `unknown_repeat_replays_engine_cause_and_keeps_stated_or_legacy_unknown_bare` asserts.
   - Build a second request with `serde_json::from_value::<AccessRequest>` from the fixture's
     request written as JSON (`identity` `{"kind": "instance", "id": "root"}`, `tool`
     `management.reserved.v1`, `arguments` `{}`) with the member `"signInReturn": R`. Through
     `invoke_repeat` on `pending.state`, the state after the first request, it is a repeat with
     the key `"key"`, the same operation and the same arguments of an invocation in progress,
     which without decision 2 would report OPS-032's conflict. It gives: the `_meta` key
     `mark(&c, "reason")` `invalid`, not `conflict`; exactly one diagnostic, whose `ruleId` is `PND-013` and whose message text holds
     `mark(&c, "signInReturn")` and ` under PND-013 for {kind}/{name}: `, with `kind` and `name`
     those of the object in `pending.state` that the route's `target` names; no external intent; as many invocations in its state as
     `pending.state` holds; and one decision named `refuse-sign-in-return`.
   - The same JSON with `tool` `nope.list.v1`, through `invoke_repeat` on `pending.state` as well,
     gives the reply's `error.data` key `mark(&c, "reason")` `refused`, as `refusal-tiers`' step 3
     shows for a tool not presented.
   - For both of these transitions and both requests, no string of `S` occurs in
     `serde_json::to_string(&transition)`, in `format!("{transition:?}")` or in
     `format!("{request:?}")`.

   A request built from JSON with `"signInReturn": null` has `sign_in_return` `None`; assert it.

4. **One portable case, on 048a-3's fixture.** Add the directory
   `conformance/conventions/requests/sign-in-return-not-taken/` with one file, `case.yaml`: ID
   `requests-sign-in-return-not-taken`, `layers: [convention, interface]`,
   `promises: [PND-013, PND-014]`, title
   `A sign-in return on a call to an Agent and on a held request reports invalid before every other reason after access, as on a first sign-in.v1, and on a refused request reports refused`,
   `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
   `{"fixture": "../sign-in-hold/fixture.json", "manifest": "../sign-in-hold/manifest.json", "documents": "../sign-in-hold/documents.json"}`,
   048a-3's case's three files, unchanged. Write it as JSON with two-space indentation and a final
   newline, its members in the order of `requests-sign-in-hold`. It must meet 046a's portable-case
   rule as it stands.

   Below, ROOT, `META/x`, `I`, `P`, "Root asks T with A" and "a completion" are as 048a-3's decision
   6 defines them. "Root asks T with A, returning" is "Root asks T with A" whose request also holds
   the member `signInReturn` with the value `R` of decision 3. URL is the member
   `"clientCapabilities": {"elicitation": {"url": {}}}`, as 048a-5's case writes it. `K(x)` is the
   JSON pointer `x` followed by the `_meta` key for `signInReturn`, its `/` written `~1`, as
   `requests-sign-in-return-kept-nowhere` writes it. A new step's evidence is its time
   `2026-11-12T12:00:00Z` and, as 048a-3 writes its new steps, `ids` the 100 strings
   `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of 120000 + 100n +
   k, for k = 0 to 99, `revisions` the 16 strings `return-` followed by the decimal of n, a `-` and
   the decimal of j, and `suffixes` the 16 strings `r` followed by the four-digit, zero-padded
   decimal of 100n + j, for j = 0 to 15, where n is the step's number in this case; a copied step
   keeps its own evidence. Two expectations recur:
   - KEPT: the list of pointers `K(/reply/_meta/)`, `/reply/structuredContent/signInReturn`,
     `/records/0/data/signInReturn`, `K(/records/0/data/)`, `/records/1/data/signInReturn` and
     `K(/records/1/data/)`, the six of `requests-sign-in-return-kept-nowhere`'s step 2 other than
     its outcome pointer;
   - NOT-TAKEN: `{"reply": {"isError": true, "_meta": {"META/reason": "invalid"}, "structuredContent": {"diagnostics": [{"ruleId": "PND-013", "level": "error"}]}}, "external_intents": [], "record_facts": [{"type": "access-decision"}, {"type": "request-ended", "data": {"reason": "invalid", "ruleId": "PND-013"}}]}`
     with `absent` KEPT followed by `I`, `P` and `/reply/error`.

   Its steps:
   1. to 3. `requests-sign-in-hold`'s steps 1 to 3, `input`, `expect` and `capture`, whole, each
      with its evidence: first start, registration and root's set-up of `signed`, captured as
      `$setup1`.
   4. Root asks `group.sign-in.v1` with `{"instance": {"kind": "Agent", "name": "signed"}}`,
      returning, its request also holding URL. `expect` NOT-TAKEN: a first `sign-in.v1` takes no
      return, and 048a-5's invalid under PND-013 answers it, after the capability and before the
      conflict of the setup in progress; this order's block leaves it to 048a-5.
   5. to 8. `requests-sign-in-hold`'s steps 4 to 7, whole, each with its evidence: the two run
      reports, the account check's completion and root's open of `to-signed`.
   9. Root asks `to-signed.ping.v1` with `{}`, returning. `expect` NOT-TAKEN: the call is not
      dispatched, so the value reaches no Agent.
   10. and 11. `requests-sign-in-hold`'s steps 8 and 9, whole, each with its evidence: root's
       `to-signed.work.v1`, captured as `$call1`, and its completion LAPSE, which raises the
       sign-in action and begins the hold.
   12. Root asks `to-signed.work.v1` with `{}`, returning. `expect` NOT-TAKEN with
       `"META/unanswered": 1` added to the reply's `_meta`: invalid before the hold's unavailable,
       naming no action and raising none.
   13. Root asks `to-signed.work.v1` with `{}`. `expect`
       `{"reply": {"isError": true, "_meta": {"META/reason": "unavailable", "META/unanswered": 1}, "structuredContent": {"diagnostics": [{"ruleId": "PND-005", "level": "error"}]}}, "external_intents": []}`:
       without the return, the same request is held.
   14. Root asks `nope.list.v1` with `{}`, returning. `expect`
       `{"reply": {"error": {"code": -32602, "data": {"META/reason": "refused", "diagnostics": [{"ruleId": "ACC-001", "level": "error"}]}}}, "external_intents": []}`
       with `absent` `K(/reply/error/data/)`, `/reply/structuredContent`,
       `/records/0/data/signInReturn`, `K(/records/0/data/)`, `/records/1/data/signInReturn` and
       `K(/records/1/data/)`: a refusal keeps its reason, and the value is nowhere in it.

   The route is new and has not been run: 048a-5's case shows its return refusal only before a
   set-up, though its step 12 shows its PND-010 invalid while a set-up is in progress. Before
   running it, check that no ID repeats across the case's steps' `ids`, the copied steps' included.
   If 046a's runner check refuses the case, if a step copied from `requests-sign-in-hold` gives a
   reply other than that case states, or if a step's reply differs from what it states, stop and
   report the error: do not change the expectations to pass it, and do not change the inputs beyond
   what this decision says. No expectation is generated from the engine's output. The coverage
   index entry holds `id`, `case` (`conventions/requests/sign-in-return-not-taken/case.yaml`),
   `promises` and, where its neighbours in that piece hold it, `safety_rows: []`, in the members and
   order its neighbours use. Write it in a `cases` list in `conformance/manifest.yaml`, then, from
   `engine/`, run `cargo run --release -p conduit-conformance --example pieces` with no flag, which
   moves it into `conformance/conventions/requests/index.yaml` in ID order and leaves
   `manifest.yaml` byte for byte as it was.

5. **The expected failure is lifted.** In `conformance/expected-failures.yaml`, remove the entry
   whose `id` is `requests-sign-in-return-kept-nowhere`, with the comma that joins it to the entry
   before it, so that the file keeps its form and stays valid JSON. Nothing else in the file
   changes, and the case `requests-sign-in-return-kept-nowhere` and its `requests.json` do not
   change: it passes as order 171b wrote it.

6. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** At the end of `engine/CONFORMANCE.md`, after
   its last section and a blank line, add this section, wrapped at 100 columns as the file is:

   ```text
   ## A sign-in return where none is taken (order 048a-6)

   The core's request has a member for `signInReturn`, the `_meta` key of PND-013 and PND-014, whose
   `Debug` form withholds its value. A request that carries it and whose operation is not
   `sign-in.v1` is decided right after the access decision: where the access decision permits it,
   it reports invalid under PND-013, with a diagnostic naming the key and holding no part of the
   value, before any other invalid, any conflict and any hold of PND-005 or PND-011, and it is not
   dispatched; where it does not, it keeps that reason (OPS-023). A `sign-in.v1` request that
   carries it is decided as order 048a-5 decides it, among that operation's invalids. Nothing else
   reads the value, so no result, record, diagnostic, decision or state holds it (PND-014). A
   retry's return is not yet read: PND-013's checks and the `sign-in-hook.v1` retry that passes a
   value on are later orders' of the cut. The core has no task requests, so a `tasks/update`
   carrying the key is not decided.
   ```

   In the section "Refusals and reasons (order 046a)", replace 048a-5's sentence "The shell passes
   `clientCapabilities` and `signInLogin` to the core, and not yet `signInReturn` (order 048a-6),
   so through the shell a return on a first request is not seen, and the request is answered as
   one without it." with "The shell passes `clientCapabilities`, `signInLogin` and, in a request
   it does not reject, `signInReturn` to the core.", rewrapping only that paragraph's lines; the
   rest of that paragraph stays. Its sentence that begins "After that invalid and before that
   conflict, order 048a-5 decides three more invalids" stays as it is: it speaks of `sign-in.v1`
   alone, which this order leaves to 048a-5. Nothing else in the file changes.
   After the other decisions, run `python3 tools/promise_standing.py --write`, as AGENTS.md asks of
   a change to the expected failures, to a case's `promises` and to a promise ID in the core's
   sources. Report the rows of PND-013 and PND-014 and the totals; the totals change only by what
   those rows change.

7. **The declaration.** `docs/work-orders/048a6-declared.yaml` holds one `added` shape named
   `requests-sign-in-return-not-taken` for that case. The coordinator wrote it with this order;
   check that it matches and change it only to make it so.

8. **The shell forwards the return to the core, and nowhere else.** In
   `engine/crates/shell/src/server.rs`'s `call_tool`, in the invoke input, the arm `Ok(None) =>` of
   `let input = match retry {`, whose line begins
   `let mut input = json!({"kind":"invoke","request":{`, where `meta` holds the key
   `{prefix}signInReturn` with a value that is not JSON null, set the member `signInReturn` of that
   input's `request` to that value as sent, after 048a-5's
   `.extend(sign_in_members(&meta, prefix));` and before the arm returns `input`. Write it outside
   048a-5's `sign_in_members`, so that `record_rejection`'s input never holds it, and do not change
   `sign_in_members` or `record_rejection`. The shell writes the value nowhere else: no log line,
   error text, diagnostic or store. In `engine/crates/shell/src/server_tests.rs`, add one test,
   `sign_in_return_not_taken_reports_invalid_over_mcp`, written as
   `shell_rejection_carries_diagnostic` is: a fixed shell, root's token, a raw session at revision
   `2025-06-18`, and a client that declares the implementation-interface extension by hand in the
   request's `_meta`, as the existing tests that send a key under the prefix do. Root's
   `management.inspect.v1` with `{"object": {"kind": "Interface", "name": "root"}}`, whose `_meta`
   also holds `{prefix}signInReturn` with the value `R` of decision 3 (`{value, via}`, `via`
   `paste`), gives a reply with `isError` `true`, the `_meta` key `{prefix}reason` `invalid` and
   exactly one diagnostic, whose `ruleId` is `PND-013` and whose message text holds the key
   `{prefix}signInReturn`. No string of decision 3's `S` occurs in the response's text or in the
   store's records written with `serde_json::to_string`. Write no promise ID in the shell's
   forwarding or its comments, and none in the test but `PND-013` in its assertion, so that
   `engine/STANDING.md`'s row of PND-013 names the core and the shell, since
   `tools/promise_standing.py` counts every file under `engine/crates/shell/src`, tests included,
   and PND-014's row names the core alone.

   Order 048b-relay1 has merged, so `call_tool` builds either this invoke input or relay1's
   `respond` input in its place. Put the forwarding into the invoke input alone, never into the
   `respond` input, whose `meta` already holds a retry's whole `_meta` and whose `signInReturn`
   order 048a-7 reads, and not into `record_rejection`'s line, which also begins
   `let mut input = json!({"kind":"invoke","request":{`; the start checks of assumption (h) print
   `0` and `3` at `9e2a1410`.

9. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
   record changes, and no fixture. Nothing reads a retry's return: PND-013's four checks, its
   reasons, `signInLogin`'s `returned`, the rebuilt value and the `sign-in-hook.v1` retry that
   passes it on are the next two orders'. No task request is read, and no shell code beyond decision
   8's forwarding and test, CLI command (051b), relay (048b-relay1, 048b-relay2, 048b-join) or
   package (050) changes. Report as findings, for the coordinator: that a retry's return, which
   reaches the core only in order 048b-relay1's `respond` input, is read by no order before 048a-7;
   that PND-013 says the invalid comes "in OPS-023's order" but not where it falls among the other
   invalids after the access decision (for `sign-in.v1`, 048a-5's order; for other operations,
   `answer.v1`'s invalids), so this order's place for it is the coordinator's reading; that
   IFC-016's error for a `tasks/update` carrying the key is not built, since the core reads no task
   request; that neither PND-013 nor IFC-049 says whether a `null` `signInReturn` is carried; as a
   coordinator choice no check of this order tests, a return on an Agent's onward request (one sent
   with `serving` set), so that a build that decided it otherwise passes every check here; that a
   `sign-in.v1` request carrying `signInReturn` from a caller that 048a-2's Record (R1) leaves at
   the stub is answered with the stub's unavailable, or, where 048a-5b has merged, which removes
   the stub, is dispatched to its account check with the return read by nothing, not PND-013's
   invalid, and neither 048a-5 nor this order refuses it, so PND-013's observable (invalid after
   access, before dispatch) is not met for that caller, and no check of either order tests it; that a keyed repeat of `sign-in.v1` carrying `signInReturn`
   gets the key's conflict or the kept answer before 048a-5's PND-013 invalid, since
   `sign_in::preflight` runs only for a new or redispatched request, and this order's block
   excludes `sign-in.v1`; report it beside 048a-5's R4; and any place where PND-013, PND-014 or IFC-049 left you a choice this order does not settle.

## Scope

Change only `engine/crates/core/src/access.rs` (decision 1), `requests.rs` (`invoke_inner`, decision
2, and the test of decision 3), `engine/crates/core/src/sign_in.rs` (only a struct literal that
decision 1 wraps, none at `9e2a1410`), `engine/crates/shell/src/server.rs` (`call_tool`'s
forwarding, decision 8), `engine/crates/shell/src/server_tests.rs` (decision 8's test),
`engine/CONFORMANCE.md` (decision 6), `engine/STANDING.md`, written only by the tool,
`conformance/expected-failures.yaml` (one entry removed),
`conformance/conventions/requests/index.yaml` and the new directory
`conformance/conventions/requests/sign-in-return-not-taken/`. `docs/work-orders/048a6-declared.yaml`
comes with this order; change it only as decision 7 says. `conformance/manifest.yaml` holds the new
entry only until the pieces tool moves it, and ends unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`,
`packages/`, the shell's other files, the experiments, the case schema,
`conformance/internal-cases.yaml`, any other case or fixture, the directories of
`requests-sign-in-return-kept-nowhere`, `requests-sign-in-hold` and
`requests-sign-in-refusals-before-parts` included, `docs/implementation/` or `docs/proposals/`:
report any defect as a finding. `experiments/011-short-form-spike/` does not change either: decision
1 changes only a type, and 048a-5 set every struct literal's `sign_in_return` to `None`, so the
spike needs no change; if its tests fail, stop and report it.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- sign_in_return
cargo test --release -p conduit-core --lib -- sign_in_request_member_refusals
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-return-
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
cargo test --release -p conduit-shell sign_in_return_not_taken
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 1 passed and 0 failed, this order's test; report the count if another test's
name matches. The fourth reports 1 passed and 0 failed, 048a-5's test, unchanged unless decision 1 wrapped a literal.
The pieces tool prints `Nothing to move.` The first runner command ends
`2 cases passed`, the lifted case and this order's, with no expected failure. `requests-sign-in-`
and `requests-` each end with two more cases passed than the last lines you kept before any change,
and one expected failure fewer; report each line before and after. The first shell command reports
1 passed and 0 failed, decision 8's test, and the second passes. If any
other case, or any shell, CLI or experiment test, fails because a request carrying `signInReturn`
now reports invalid or because the member's type changed, stop and report it: do not change it. The threads are 4 because at 24 the runner ran out
of memory (order 071's Record).

Next, show that both cases fail without decision 2. From `engine/`, first keep a copy:

```bash
cp crates/core/src/requests.rs target/requests.rs.fixed
```

Then, by hand, in `crates/core/src/requests.rs`, change `request.sign_in_return.is_some()` on the
first line of decision 2's block to `false`, and nothing else. Then, one at a time:

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-return-
cp target/requests.rs.fixed crates/core/src/requests.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-return-
```

The first runner command must fail, printing `FAIL 0 cases passed; 2 failed`, with an error at step
2 of `requests-sign-in-return-kept-nowhere`, whose inspection succeeds where invalid is expected
(an unexpected field, the reply's `_meta` key for `outcome`, as 048a-5's Record shows), and one at
step 9 of `requests-sign-in-return-not-taken`, whose `to-signed.ping.v1` gives another reply than
PND-013's invalid (step 4 still passes, since 048a-5 refuses it); after the restore the second ends
`2 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older modification time, so
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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a6-declared.yaml; echo "exit $?"
python3 -c 'import json; print(len(json.load(open("conformance/expected-failures.yaml"))["cases"]))'
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/conventions/requests/sign-in-return-kept-nowhere conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/requests/sign-in-refusals-before-parts conformance/conventions/inventory conformance/conventions/setup conformance/language tools packages experiments docs/implementation docs/proposals
git status --short -- engine/crates/shell
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a6-declared.yaml: 1 shapes.`,
then `Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. No promise's text changes, so no row of the model-checking account goes stale; where
`check_model_account.py` reports one stale, stop and report it. The unit tests, run with the
interpreter that has PyYAML, end `OK`; report the count. The `python3 -c` command prints one less
than it prints on the tree you start from; report both. The first `git status` command prints
nothing, and the second lists only `engine/crates/shell/src/server.rs` and
`engine/crates/shell/src/server_tests.rs`.

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
grep -c 'pub sign_in_return: Option<SignInReturn>,' engine/crates/core/src/access.rs
grep -c 'f.write_str("SignInReturn(withheld)")' engine/crates/core/src/access.rs
grep -c 'if request.sign_in_return.is_some() && route\["operation"\] != c.qualified("sign-in.v1") {' engine/crates/core/src/requests.rs
grep -c 'fn sign_in_return_not_taken_is_kept_nowhere' engine/crates/core/src/requests.rs
grep -rl '"refuse-sign-in-return"' engine/crates/core/src | wc -l
grep -c '"id": "requests-sign-in-return-not-taken"' conformance/conventions/requests/index.yaml
grep -c '^## A sign-in return where none is taken (order 048a-6)$' engine/CONFORMANCE.md
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Nothing else reads the value, so no result, record, diagnostic, decision or state holds it (PND-014).'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'After that invalid and before that conflict, order 048a-5 decides three more invalids'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'The shell passes .clientCapabilities., .signInLogin. and, in a request it does not reject, .signInReturn. to the core.'
grep -c 'fn sign_in_return_not_taken_reports_invalid_over_mcp' engine/crates/shell/src/server_tests.rs
grep -c '^| PND-013 | convention | must | 3 | 0 | outside | core, shell | case passes |$' engine/STANDING.md
grep -c '^| PND-014 | convention | must not | 2 | 0 | outside | core | case passes |$' engine/STANDING.md
```

PND-013's three cases are `requests-sign-in-return-kept-nowhere`,
`requests-sign-in-refusals-before-parts` and this order's, and its sources are the core's
(`sign_in.rs` and decision 2) and the shell's, since decision 8's server test names PND-013 in its
assertion; PND-014's two cases are the first and this order's, decision 2's decision names PND-014
in the core's sources, and nothing in the shell names it (decision 8).

The first of these prints a number other than 0, and the second prints 3 (`sign_in_members`'s
definition and its two calls, unchanged):

```bash
grep -c 'signInReturn' engine/crates/shell/src/server.rs
grep -o 'sign_in_members(' engine/crates/shell/src/server.rs | wc -l | tr -d ' '
```

Each of these prints 0:

```bash
grep -c 'requests-sign-in-return-kept-nowhere' conformance/expected-failures.yaml
grep -c -E 'sign_in_return: Option<(serde_json::)?Value>' engine/crates/core/src/access.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and not yet .signInReturn. (order 048a-6)'
git diff HEAD --stat -- spec IN_FORCE.md conformance/conventions/requests/sign-in-return-kept-nowhere conformance/conventions/requests/sign-in-hold | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark each
choice you made yourself as your own, give the last line of every check, each exit status and count
above, the rows decision 6 names with the totals, the line in `invoke_inner` where decision 2's
block begins, each line of `sign_in.rs` you changed for decision 1, the line in `call_tool` where
decision 8's forwarding begins, and whether `AccessRequest` derives `Serialize` on your tree.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a6-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
