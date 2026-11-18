# Work order 048b-join (E7b-3): a sign-in started at the engine reaches the person through the Interface, and its return comes back

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: this order adds no case, so `vectors` is an empty list, and its two
shell tests go under `checks`. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, add no crate, and
reach no other machine. Run one cargo command at a time. Never print a whole large file, case,
fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential. Test code may read the client token it issued in its own temporary store,
as `token` and `server::client` do, and nothing prints it. Do not run `claude`, `codex` or any client, sign in nowhere, and open no
URL: the URLs below are test values that no program fetches.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md), before the coordinator's cuts, split spec batch
1's E7b, the shell relay, in two: "048b-relay: the shell relays `input_required` (`server.rs`,
`runner.rs`). M. After S0b. 048b-join: S-M. After 048a-6, the last part of 048a-2." It now reads
"048b-join: S-M. After 048a-8, the last part of 048a-2's entry, and 048b-relay2." Each side has
since been cut again by the orders written beside this one (the coordinator's cuts, each open to the owner's strike):

- **The engine's side, in the core.** Order 048a-5 cuts itself into 048a-5 (the refusals before
  any part, with the core's request gaining `client_capabilities`, `sign_in_login` and
  `sign_in_return`, and the shell forwarding `clientCapabilities` and `signInLogin`) and 048a-5b
  to 048a-5h, each after the one before: the account check first and the signed-in answer, which
  lifts PND-005's hold; the fixed-port refusal; the `sign-in-hook.v1` part and IFC-049's URL
  match; the engine's message and the input-required output that 048b-relay1 relays; polling on
  the retry; another account undone during polling; and DL18's end. The shell issues, keeps and
  checks the sign-in's `requestState` (048b-relay1), and a retry reaches the core only as its
  `respond` input (A8). Order 048a-6 cuts the returns into 048a-6 (a return where none is taken
  reports invalid, and is kept nowhere, with the shell forwarding `signInReturn`), 048a-7 (an
  accepted return passed on to `sign-in-hook.v1`, with `signInLogin`'s `returned`) and 048a-8 (a
  redirect return checked as a URL and rebuilt), and puts 048b-join after 048a-8.
- **The shell's side.** Order 048b-relay cuts itself into 048b-relay1 (the core's input-required
  output and its retry's input declared, and `server.rs` relaying the output to the Interface
  that asked with a `requestState` the shell issues and holds, and checking each retry) and,
  planned, 048b-relay2 (`runner.rs`'s half: URL elicitation declared toward signIn Types under
  IFC-045, and a part that an implementation answered with its own `InputRequiredResult` sent
  again with that implementation's `requestState` and the responses, as the core writes it).

This order comes after 048a-8 and 048b-relay2, and so 048a-5h. It joins the two sides,
so that one `sign-in.v1` runs end to end over the engine's real MCP server: from the Interface's
`tools/call`, through the core and the hook parts, to the person as a URL action, and back on the
Interface's retry, with the person's return carried to the hook that started the login, to a
signed-in result that lifts the instance's hold.

[The targeted review's section 6](../proposals/review-signin-2026-11-16/README.md#048b-join) puts
nothing on 048b-join. The owner's [ruling 82](../proposals/ruling-82/README.md#the-decisions),
point 3, merges the Agent packages 0.2.0 only after 048b-join, onto an engine that runs the
account check; order 050n waits for this order's merge, and so does A5's live check
([experiment 027](../../experiments/027-a5-sign-in/README.md), "When, and under which Type").

**Ruling 83, which this order follows.** The owner decided on 2026-11-18
([ruling 83](../proposals/ruling-83/README.md)) that the Agent packages 0.2.0 are
subscription-only: `apiKey` is not in `.v2`; `account` stays required with no default, a string
or null, with no `oneOf` of `account` and `apiKey`; API access comes later as a declared gateway
route. Nothing in this order reads, sets or names
`apiKey`. Its test signs in an Agent whose account is a string; an account that is null has no
account check, and `sign-in.v1` for it reports invalid under PND-010, which order 048a-2 builds
and this order does not touch.

**What it builds.** One behavior: `sign-in.v1` through the shell, out and back. IFC-049: "With
that capability tools/call may return InputRequiredResult with resultType input_required and
exactly one inputRequests entry: elicitation/create with mode url, message and url", and "The
client echoes it exactly; the engine treats it as attacker-controlled and rejects tampering,
replay of a consumed state or another principal." PND-010: "Each retry polls account-hook.v1 to
check the account after the person's sign-in, under PND-011, until an account appears or DL18
ends". PND-013: the engine "retries the sign-in-hook.v1 call that started the login, carrying
that call's own requestState, accept in inputResponses for its URL elicitation, and signInReturn
{value}", and "The result answering the Interface retry, another InputRequiredResult or the final
result, holds signInLogin with returned: delivered where the hook reports the program took the
value". IFC-045: the engine declares "elicitation URL mode {"url": {}} only for requests and
hooks to instances whose Type states signIn". PND-014: the value never "appears in a result,
structuredContent, response _meta, notification, diagnostic, log or record". No promise changes:
the texts state all of it, and the orders before this one build each half. What is left is the
proof that the halves meet (decision 6), with the first request's seam joined by 048a-5 and
048a-6 and checked here (decision 2).

**What the orders before it bring, which this order assumes.** Orders 048a-5, 048a-6 and
048b-relay1 were written beside this one; 048a-5b to 048a-5h, 048a-7, 048a-8 and 048b-relay2 are
planned and not written. Each assumption below is named, so that the coordinator checks it against
their texts and Records and amends this order before dispatch where one differs. Where you find at
the start that one merged otherwise, apply the nearest sound reading of each decision below to
what merged, and say so; where an order's Record names a function, a member or a case, use that
name. Below, `META/x` is the key `x` under the prefix that `spec/names.yaml` sets (in the tests,
`shell.context.meta_prefix`); `META/signInReturn`, `META/outcome` and so on.

- **A1, from 048a-5 and 048a-6.** `AccessRequest` (`engine/crates/core/src/access.rs`) has the
  members `client_capabilities` (serde name `clientCapabilities`), `sign_in_login`
  (`signInLogin`) and `sign_in_return` (`signInReturn`), each with `#[serde(default)]`, absent or
  null meaning not carried; `sign_in_return`'s type is 048a-6's `SignInReturn`, whose `Debug`
  withholds the value (PND-014). 048a-5 forwards `clientCapabilities` and `signInLogin` through
  its function `sign_in_members` in `server.rs`, in `call_tool`'s `invoke` input and in
  `record_rejection`'s; 048a-6 forwards `signInReturn` in `call_tool`'s `invoke` input alone,
  outside `sign_in_members`, so that `record_rejection`'s input never holds it. Decision 2 checks
  that the three reach the core.
- **A2, from 048b-relay1.** `Transition` has `input_required: Vec<InputRequired>`, each entry
  `{invocation, attempt, request, meta, expires}`; `Shell::commit` gives an entry to the waiters
  of its `(invocation, attempt)` as `{"inputRequired": V}`; `server.rs`'s `call_tool` relays it as
  `CallToolResponse::InputRequired`, its one input request under the key `url`, with a
  `requestState` the shell issues in `server/relay.rs` (43 characters of `[a-z0-9]`), bound to the
  authenticated identity, the tool, the invocation and the attempt, expiring at `expires`; a
  retry carrying `requestState` and `inputResponses` is checked there and, accepted, reaches the
  core as the input `{"kind": "respond", "invocation", "attempt", "identity", "responses", "meta"}`,
  `meta` being the retry's whole request `_meta`; a refused one is answered with
  `ErrorData::invalid_params` and one of relay1's texts, the consumed state's beginning
  `requestState was already used`.
- **A3, from 048a-5b to 048a-5h.** `sign-in.v1` is no longer one of C11's stubs. Its invocation
  dispatches `account-hook.v1` first and, for no account or another, `sign-in-hook.v1`, each a
  part sent to the instance's keyed implementation by a `dispatch` intent whose `call.params.name`
  is the hook's local name, as `requests.rs`'s `fn intent` names a keyed call. The hook's
  `InputRequiredResult`, received as the completion `{"kind": "result", "result": R}` that
  `runner.rs`'s `call` already gives for any result, is checked under IFC-049 and relayed through
  A2's `input_required`, with the engine's own message and `signInLogin` in `meta` (048a-5e).
  048a-5f replaces relay1's `respond` arm: an accepted retry polls `account-hook.v1`, and the
  expected account ends the invocation succeeded as 048a-5b does (output `account`, any
  unanswered sign-in action answered `signed-in`, and the instance's entry removed from 048a-3's
  `State` member `sign_in_holds`, which lifts PND-005's hold); decline or cancel ends it failed
  (PND-010) and dispatches no part. 048a-5g undoes another account, and 048a-5h ends a waiting
  sign-in unknown at DL18. The coordinator checks this against 048a-5b's and 048a-5f's texts.
- **A4, from 048a-7 and 048a-8.** On an accepted retry carrying `signInReturn`, the core's next
  intent retries the `sign-in-hook.v1` call as a `dispatch` intent whose `call.params` carry that
  call's own `requestState`, `inputResponses` holding `accept` under its elicitation's key, and
  `_meta` `META/signInReturn` `{"value": V}`. 048a-7 builds that retry of `sign-in-hook.v1` for an
  accepted return; for a redirect login such as `browser`, 048a-8 passes through that same retry
  the value it rebuilds, `V`, with only `code`, `state`, `iss` and `scope`. The result answering
  the Interface's retry holds `signInLogin` with `returned` `delivered` where that hook call
  succeeded (048a-7). Order 048a-6 lifted the expected failure
  `requests-sign-in-return-kept-nowhere`.
- **A5, from 048b-relay2.** Each call for a request or hook to an instance whose Type states
  `signIn` holds `{"elicitation": {"url": {}}}` in `call.params._meta`'s
  `io.modelcontextprotocol/clientCapabilities` (IFC-045), written by the core in `fn intent`, and
  `runner.rs` sends a call as the core writes it, its retry included.
- **A6, from 048a-3, 048a-4 and 048a-6.** The case `requests-sign-in-hold` (048a-3), with
  048a-4's inserted step, has these first nine steps, as order 048a-6 lists them: first start;
  registration; root's `group.set-up.v1` of `signed`; two run reports; the account check's
  completion, which ends the setup succeeded; root's `access.open.v1` of `to-signed`; root's
  `to-signed.work.v1`; and its completion LAPSE, which raises the instance's sign-in action and
  begins its hold. Its Type `signed-agent.v1` is 048a-1's, with the login `browser` (a redirect
  login stating `carried: true`) served by the keyed `signed-server`; its Agent `signed` has the
  account `primary-account`; and root reaches it through the Fleet `group`'s tool
  `group.sign-in.v1`, as order 048a2's case asks `group.sign-in.v1` with `{"instance": {"kind": "Agent", "name": "signed"}}`.
- **A7, from 048a-5h.** At the expiry of each output it relays, the shell submits one input
  naming the invocation and attempt, and the core ends that `sign-in.v1` unknown if it still
  waits on that attempt, with no hook run later (DL18). Decision 5 checks it.
- **A8, requestState.** The shell issues, keeps and checks a sign-in's requestState
  (048b-relay1, decision 5); the core holds none, and a retry reaches it only as `respond`.
  048a-5's cut issues none in the core.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`e4ceb4c8`, before every order named above; find each place by the text quoted.

- `server.rs`'s `call_tool` (`async fn call_tool(`) builds the core's `invoke` input from the
  request's name, its arguments and two `_meta` keys,
  `let input = json!({"kind":"invoke","request":{"identity":instance(&identity),"tool":request.name,"arguments":request.arguments.unwrap_or_default(),"serving":serving},"key":key});`,
  after reading `meta` from `context.meta` and refusing a `serving` or `idempotencyKey` that is not
  a nonempty string with `record_rejection`. `record_rejection` builds its own `invoke` input from
  `serving` and `idempotencyKey` alone.
- rmcp 3.9.0 gives the client `call_tool_once`, which returns `CallToolResponse` without driving
  the rounds; `CallToolRequestParams` has `meta`, `request_state` and `input_responses`; and an
  `InputRequiredResult` reaches only a peer on revision `2026-07-28` or later. In
  `server_tests.rs`, `modern` connects such a client, `fixed_shell` gives a store with a fixed
  engine URL, the executor `Memory` answers `dispatch` intents in place of a runner, and
  `pending_http_completion_and_client_cancellation` serves a request over MCP with it.
- `tests.rs`'s `established` opens a store and commits the S1 case's steps through
  `Shell::commit`, pushing each step's evidence, keeping its captures through the test trace, and
  building the register input's project and snapshots in `fixture_input`.
- `server.rs`'s `client`, the helper behind `conduit call`, uses rmcp's `call_tool` with no
  elicitation handler, so it cannot answer an `input_required` result.

**When this order starts.** Orders 048a-5h, 048a-8 and 048b-relay2, and every order before each,
must be merged to `main`. Under the owner's approval of stacked dispatch, the coordinator may
instead start this order from a verified branch that holds all of them, saying so in a dispatch
note, which relaxes only the first three checks below, the three Records. Before you change
anything, check from the
repository root that you find what they give, and stop, change nothing and report if any check
but the last three prints otherwise:

```bash
cat docs/work-orders/048a5h-*.md | grep -c '^## Record'
cat docs/work-orders/048a8-*.md | grep -c '^## Record'
cat docs/work-orders/048b-relay2-*.md | grep -c '^## Record'
grep -cE 'pub sign_in_return: Option<' engine/crates/core/src/access.rs
grep -rc 'request_state' engine/crates/core/src/access.rs
grep -c 'pub input_required: Vec<InputRequired>' engine/crates/core/src/lib.rs
grep -c 'fn request_states_bind_interface_tool_and_single_use' engine/crates/shell/src/server/relay.rs
grep -c '("sign-in.v1", "048a", "PND-010")' engine/crates/core/src/requests.rs
grep -rl '"delivered"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"state-mismatch"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'requests-sign-in-return-kept-nowhere' conformance/expected-failures.yaml
grep -c 'pub sign_in_holds: BTreeMap<String, Value>' engine/crates/core/src/lib.rs
test -f conformance/conventions/requests/sign-in-hold/case.yaml; echo "case $?"
grep -c '"clientCapabilities"' engine/crates/shell/src/server.rs
grep -c 'signInLogin' engine/crates/shell/src/server.rs
grep -cE 'signInReturn|sign_in_return' engine/crates/shell/src/server.rs
```

They print `1` for each of the three order files (each order's Record: the merged check, which a
dispatch note may relax); `1` (048a-5's member, A1, whatever its type); `0` (no `requestState` in
the core's request, A8); `1` (relay1's output, A2); `1` (relay1's states, A2); `0` (the stub
removed, A3); a number other than `0` (048a-7's `returned`, A4); a number other than `0`
(048a-8's redirect check, A4); `0` (048a-6 lifted the expected failure); `1` (048a-3's member);
and `case 0`. The last three are reported, not checked, and none of them stops the run. Each is
expected to print a number other than `0`: 048a-5 forwards `clientCapabilities` and `signInLogin`,
and 048a-6 forwards `signInReturn` (A1). Report the three counts; decision 2 decides by reading
`call_tool`'s `invoke` input, not by them, since a comment that names a key also counts and a key
built otherwise, from the protocol's own key or a constant, may not.

Then take from 048b-relay2's Record the name of the test it gives for URL elicitation declared
toward signIn Types (A5, IFC-045), and run it from `engine/` with `cargo test --release` and that
name, with the crate the Record gives; it must report at least 1 passed and 0 failed, or stop and
report it. Where the Record names no such test, report that and go on: decision 6's first test,
step 3, decides A5. Report as well, as a count only and not as a reason to stop, what this prints,
since relay2 may write the declaration through a helper or a constant or change the function's
signature:

```bash
grep -cE '"url": ?\{\}' engine/crates/core/src/requests.rs
```

Then take from 048a-5f's Record the decision name, or the test name, it gives an accepted retry
that polls `account-hook.v1`, and check that `grep -rl` of that name in `engine/crates/core/src`
(for a decision name) or `engine/crates/core/` (for a test name), piped to `wc -l`, prints a
number other than `0`; report the name and the count (relay1's `respond` arm replaced, A3).
Report too the count of `grep -rl '"refuse-input-response"' engine/crates/core/src | wc -l`: a
number other than `0` is accepted where the `respond` arm also polls `account-hook.v1`, since a
respond that reaches an invocation with no input request outstanding may keep that decision.
Orders that land beside this one: whichever lands second keeps both changes and writes
`engine/STANDING.md` again with the tool.

**Who decided what.** IFC-045, IFC-049, PND-010, PND-011, PND-013, PND-014 and DL18 decide what
crosses the shell, in which form, and what is kept nowhere. Everything else here is the
coordinator's, each open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):
the split of 048b into 048b-relay and this order (order 117), and this order's place after the
last part of each of the cuts above; reading A8 as the plan settles it; the four seams of
decision 1, the rule that a seam a predecessor already joins is left as it is, and that only
decision 2's may be joined here; keeping this order to the shell, with a core change or an
unjoined seam of decision 3 or 4 a reason to stop; taking 048b-relay2's own test as the start's
evidence for A5; checking that the three `_meta` keys of a
first request reach the core as 048a-5 and 048a-6 forward them; checking DL18's end as 048a-5h
builds it (decision 5); the two tests, their fixture taken from `requests-sign-in-hold`, their
script, the URL, the return and every other value in them; the check that the path fails without
048a-6's forwarding of `signInReturn`; the sentence of decision 7; and the findings of decision
8. Apply them as
written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: IFC-045, IFC-049 and IFC-016 (`spec/interface/implementation.yaml`); PND-005,
   PND-010, PND-011, PND-013 and PND-014 (`spec/promises/pending.yaml`); DL18
   (`spec/implementation-defined.yaml`); `sign-in.v1`, `sign-in-hook.v1` and `account-hook.v1` in
   `spec/core/operations.yaml`.
2. Orders 048a-5 to 048a-5h, 048a-6 to 048a-8, 048b-relay1 and 048b-relay2, whole, with their
   Records; orders [048a3](048a3-build-sign-in-hold.md) (decisions 1 and 6) and
   [048a2](048a2-build-sign-in-lapse-and-conflict.md) (its case, decision 10).
3. The case `requests-sign-in-hold`, its `case.yaml`, `fixture.json`, `manifest.json` and
   `documents.json`, and `conformance/language/contracts/sign-in-resolution/project/signed-agent.json`,
   the Type `signed-agent.v1` and its login `browser`.
4. In `engine/crates/shell/src/`: `server.rs` (`call_tool`, `record_rejection`), `server/relay.rs`,
   `runner.rs` (`dispatch_later` and `call`), `lib.rs` (`commit` and the intent loop),
   `server_tests.rs` (`Memory`, `fixed_shell`, `modern`,
   `pending_http_completion_and_client_cancellation`) and `tests.rs` (`established`,
   `fixture_input`); in the conformance runner,
   `engine/crates/conformance/src/steps/resolve_project.rs` and
   `engine/crates/conformance/src/packages.rs` (`run`, `load`, `builtin`), for how a case's own
   project is built.

**Size: S-M**: about 0.3 to 0.45k changed lines of Rust, nearly all of them the two tests, their
helpers and their script, `established_case`'s rebuilding of the runner's project being about 80 to
120 of them; decision 2 changes nothing where 048a-5 and 048a-6 forward as A1 states, and about 5
lines for a key that is missing; and one sentence in
`engine/CONFORMANCE.md`. If decision 2 needs more than about 20 lines outside the tests, stop
and report what is missing, for the coordinator to split.

## The decisions it applies

1. **Four seams, and the rule for each.** The path crosses the shell at four seams: the first
   request in (decision 2); the input request out (A2); the hook parts out and their answers back,
   the return's retry of `sign-in-hook.v1` included (A3 to A5); and the retry in (A2). For each,
   first check whether an order before this one already joins it. Where one does, change nothing
   there and report which order joined it, by file and function. Only decision 2's seam may be
   joined here, in the shell alone and as decision 2 states it. A seam of decision 3 or 4 that no
   order before this one joins is a reason to stop and report, as those decisions say, and so is a
   seam that cannot be joined without a change to `engine/crates/core/`: change nothing there and
   report what is missing; the core is 048a-5's to 048a-8's, the relay 048b-relay1's and the runner
   048b-relay2's. Decision 6's tests decide whether the path is joined.

2. **The first request's `_meta` reaches the core: a check.** Read `server.rs`'s `call_tool` and
   check that its `invoke` input's `request` holds the three members of A1: `clientCapabilities`,
   the request's `_meta` key `io.modelcontextprotocol/clientCapabilities`, and `signInLogin`, its
   key `META/signInLogin`, both through 048a-5's `sign_in_members`; and `signInReturn`, its key
   `META/signInReturn`, added by 048a-6 outside `sign_in_members`. Each is passed only where the
   key is present and not JSON null, as sent, unchanged, for every tool. Report, by file and
   function, where each is forwarded. Where all three are, this seam is joined and you change
   nothing. Where your reading shows one missing from `call_tool`'s `invoke` input, add it there
   alone, in the same form, and report it: add `signInReturn` outside `sign_in_members` and never
   to `record_rejection`'s input, and leave 048a-5's forwarding of the other two as it is, in
   rejections too. A retry's `respond` input does not change: its `meta` already carries the
   whole `_meta` (A2). The value of `signInReturn` is written nowhere else in the shell: not in a
   log line, an error string, the shell's diagnostics or the store. Decide by reading the code,
   not by the start's counts.

3. **The input request out, and the retry in.** These are 048b-relay1's (A2), and a sign-in's
   output is relayed as any other. Change nothing in `server/relay.rs` or in `call_tool`'s handling
   of `inputRequired` and of a retry; where decision 6's test shows that a sign-in's output or
   retry does not pass them, stop and report the step and the reply, less any value of `R`.

4. **The hook parts.** These are 048a-5d's, 048a-7's and 048b-relay2's (A3 to A5): the core writes
   each part's call whole, its retry included, and the runner sends it as written. Change nothing
   in `runner.rs`, unless decision 6's test shows a part's answer not reaching the core as
   `{"kind": "result", "result": R}`; then stop and report it.

5. **DL18's end: a check.** This order builds no timer. Report that a `sign-in.v1` invocation
   that gets no retry ends unknown at its output's expiry by 048a-5h's input (A7), naming the
   function its Record gives for submitting that input. Only where it does not, report it as a
   finding for the coordinator, with what you found; it is not this order's to fix.

6. **Two shell tests pin the path.** In `engine/crates/shell/src/server_tests.rs`. `META/outcome`
   below is the key `outcome` under the prefix that `spec/names.yaml` sets, written in the tests as
   `shell.context.meta_prefix`, and so on, as defined above.

   *The store.* Add a helper `established_case(path: &Path, case: &Path, through: usize) -> Shell`
   that opens a store as `established` does and commits, through `Shell::commit`, the case's steps
   1 to `through`, each with its evidence pushed and its captures kept as `established` keeps
   them; a step that registers or resolves a project gets the project and snapshots built from the
   case's `fixture.json` and files as the conformance runner builds them: `steps/resolve_project.rs`
   routes the step to `packages.rs`'s `run`, which parses the fixture (its snapshots with archive
   digests, its artifacts and the project recipe), builds each project with
   `read_submitted_files` and adds `builtin(spec)`. `established_case` reimplements them in the
   test with `conduit_loader::read_submitted_files` and adds no dependency: the shell does not use
   `conduit-conformance`, and its `Cargo.toml` is out of scope. It then clears the evidence queue.
   Add `fixed_case_shell(path, case, through)`, which is `fixed_shell` with `established_case` in
   place of `established`. Each test opens
   `fixed_case_shell(temp, <the repository>/conformance/conventions/requests/sign-in-hold, 9)`, so
   that `signed` is set up, root's Conduits are open, and its sign-in action and hold stand (A6);
   issues root a client token with `issue_client_token` and reads it from the store's
   `client-tokens` directory, as `server::client` reads it; and connects root with a client that
   declares URL elicitation. No fixture file is added or copied.

   *The client.* `modern` serves `()`, whose `ClientConfig::default()` declares no elicitation, and
   on the `modern` client rmcp 3.9.0 writes the client's own configured capabilities into the
   `_meta` key `io.modelcontextprotocol/clientCapabilities` of every outbound request, over any
   value set by hand in the params `_meta`. Add a helper `url_client(url, token)` beside `modern`
   that serves a `ClientConfig` (rmcp implements `ClientHandler` for `ClientConfig`) whose
   `capabilities.elicitation` is `ElicitationCapability::new().with_url(..)` and whose
   `capabilities.extensions` holds the one entry
   `format!("{}implementation-interface", shell.context.meta_prefix)` with the value
   `{"revision": "v0"}`, as the existing tests in `server_tests.rs` that send a key under the
   prefix declare it by hand: without it the shell's HTTP layer refuses with `-32021` under
   IFC-036 any request that carries a key under the prefix, before `call_tool` runs. It connects
   through the same `serve_with_lifecycle` Discover on `2026-07-28`. Every request below is sent
   through it. Let `C` be the `_meta` key `io.modelcontextprotocol/clientCapabilities` that rmcp
   writes from it, which holds elicitation `{"url": {}}` and that extension; the core's check
   reads only `elicitation.url`. The tests do not set `C` by hand.

   *The script.* Add an executor `SignInScript`, in place of a runner, holding a list of every
   `dispatch` intent it receives and one flag `returned`, false at first. Its `dispatch`, by the
   intent's `call.params.name`:
   - `account-hook.v1`: while `returned` is false,
     `{"kind": "result", "result": {"_meta": {"META/outcome": "succeeded"}, "structuredContent": {"account": null}, "content": []}}`;
     once it is true, the same with `"account": "primary-account"`.
   - `sign-in-hook.v1` with no `requestState` in `call.params`: the InputRequiredResult
     `{"kind": "result", "result": {"resultType": "input_required", "inputRequests": {"login": {"method": "elicitation/create", "params": {"mode": "url", "message": "Open the page to sign in.", "url": U}}}, "requestState": "hook-state-1"}}`.
   - `sign-in-hook.v1` with a `requestState`: set `returned` to true and answer
     `{"kind": "result", "result": {"_meta": {"META/outcome": "succeeded"}, "content": []}}`.
   - any other: `{"kind": "result", "result": {"_meta": {"META/outcome": "succeeded"}, "content": []}}`.

   `U` is the URL for the login `browser`, as 048a-1's `signed-agent.json` states it
   (authorization `https://example.org/authorize`, client ID `placeholder-signed-agent`, redirect
   host `127.0.0.1`, ports 8400 and 8401, path `/callback`):

   ```text
   https://example.org/authorize?client_id=placeholder-signed-agent&response_type=code&redirect_uri=http%3A%2F%2F127.0.0.1%3A8400%2Fcallback&code_challenge_method=S256&code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM&state=join-state-0123456789abcdef
   ```

   Where the case's Type states the login `browser` otherwise, build `U` from its stated fields in
   the same form and report it. Let `R` be
   `http://127.0.0.1:8400/callback?code=join-code-0001&state=join-state-0123456789abcdef`, `T` the
   tool `group.sign-in.v1`, and `G` the arguments
   `{"instance": {"kind": "Agent", "name": "signed"}}`.
   The script is wrapped in `ServingExecutor::new` and served with `server::serve`, as in
   `pending_http_completion_and_client_cancellation`.

   *The first test*, `sign_in_relays_through_the_interface_and_carries_the_return`, asserts in
   this order:
   1. `sign_in_holds` has an entry for `signed`'s ID (A6).
   2. Root's `call_tool_once` of `T` with `G`, carrying `C` and no `_meta` of its own, gives
      `CallToolResponse::InputRequired` `I`. `I`'s `inputRequests` has exactly one entry, of key
      `url`; serialized, its `method` is `elicitation/create`, its `params.mode` is `url`, its
      `params.url` equals `U`, and its `params.message` contains `example.org` and contains
      neither `hook-state-1` nor `Open the page to sign in.` (IFC-049: the engine writes the
      message, and no text from the hook is relayed). `I`'s `requestState` is a string `S1` of 43
      characters that does not contain `hook-state-1`. `I`'s `_meta` holds `META/signInLogin` with
      `name` `browser` and `acceptsReturn` `true`.
   3. The script holds two intents: an `account-hook.v1` call, then a `sign-in-hook.v1` call whose
      `call.params._meta` key `io.modelcontextprotocol/clientCapabilities` holds `elicitation`
      `{"url": {}}` (IFC-045).
   4. Root's `call_tool_once` of `T` with `G`, `request_state` `S1`, `input_responses`
      `{"url": {"action": "accept"}}`, and `_meta` `META/signInReturn`
      `{"value": R, "via": "paste"}`, gives `CallToolResponse::Complete` `F`: `F.is_error` is not
      `Some(true)`, its `structuredContent`'s `account` is `primary-account`, its `_meta`
      `META/outcome` is `succeeded`, and its `_meta` `META/signInLogin`'s `returned` is `delivered`.
   5. The script holds four intents: after step 3's two, a `sign-in-hook.v1` call whose
      `call.params.requestState` is `hook-state-1`, whose `call.params.inputResponses` holds
      `login` with `action` `accept`, and whose `call.params._meta` `META/signInReturn`'s `value`
      parses, with the `url` crate, to scheme `http`, host `127.0.0.1`, port 8400, path
      `/callback` and the query pairs `code=join-code-0001` and
      `state=join-state-0123456789abcdef` and no other; then an `account-hook.v1` call.
   6. No part of the value is kept (PND-014): neither `join-code-0001` nor `R` whole appears in
      step 4's serialized reply, in any record in the store's state, or in any file under the
      store's directory, read as bytes.
   7. `sign_in_holds` has no entry for `signed`'s ID: the success lifted the hold.
   8. Root's `call_tool_once` of step 4's request again, with the same `S1`, gives an `Err` that is
      `ServiceError::McpError(e)`, with `e.code` `-32602` and `e.message` beginning
      `requestState was already used`, and the script still holds four intents: a consumed state
      reaches no invocation (IFC-049).
   9. Step 6's checks again, after relay1 recorded step 8's refusal through `record_rejection`:
      neither `join-code-0001` nor `R` whole appears in step 8's error, serialized, in any record
      in the store's state, or in any file under the store's directory, read as bytes.

   *The second test*, `sign_in_decline_through_the_interface_ends_failed`, on a store of its
   own: steps 1 to 3 as the first test; then root's retry of `T` with `G`, `request_state` `S1`,
   `input_responses` `{"url": {"action": "decline"}}` and no `_meta` of its own (so carrying `C`
   alone) gives `CallToolResponse::Complete` whose `is_error` is `Some(true)` and whose `_meta`
   `META/outcome` is `failed` (PND-010); the script still holds two intents; and `sign_in_holds`
   still has an entry for `signed`'s ID. Then two first requests of `T` with `G`, each through
   `url_client`:
   - one carrying `META/signInLogin` `{"name": "phone"}` gives `CallToolResponse::Complete` whose
     `is_error` is `Some(true)`, whose `_meta` `META/reason` is `invalid`, and whose
     `structuredContent`'s `diagnostics` hold one diagnostic of `ruleId` `PND-010` naming
     `"browser"`;
   - the other carrying `META/signInReturn` `{"value": R, "via": "paste"}` gives
     `CallToolResponse::Complete` whose `is_error` is `Some(true)`, whose `_meta` `META/reason`
     is `invalid`, and whose `structuredContent`'s `diagnostics` hold one diagnostic of `ruleId`
     `PND-013`, with neither `join-code-0001` nor `R` whole in the serialized
     reply, in any record in the store's state or in any file under the store's directory.

   For both, no new intent reached the script: it still holds two intents.

   Before running them, check that no other test's name in `engine/crates/shell/` begins
   `sign_in_relays_through_the_interface` or `sign_in_decline_through_the_interface`. If a test
   fails because a reply differs from what it states, stop and report the step and the reply,
   less any value of `R`: do not change the expectations to pass it.

7. **One sentence in `engine/CONFORMANCE.md`.** This decision replaces no predecessor's text. In
   "Refusals and reasons (order 046a)", find, read across line breaks, 048a-6's sentence ``The
   shell passes `clientCapabilities`, `signInLogin` and, in a request it does not reject,
   `signInReturn` to the core.``, and add right after it, in the same paragraph, this sentence:

   ```text
   `sign-in.v1` runs through the shell: the request's URL elicitation, signInLogin and signInReturn reach the core, its URL action reaches the calling Interface as an input-required result with the shell's own requestState, and the Interface's retry, with a carried return, reaches the hook that started the login (order 048b-join).
   ```

   Where 048a-6's sentence is not found as written, find by its text read across line breaks the
   words in `engine/CONFORMANCE.md` that an order before this one left saying which members the
   shell passes to the core, add the sentence after them, and report it; where a text there still
   says that the shell does not pass a key on, change nothing there and report it, since order
   048a-6 removes both such texts and this order changes no other. Then run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a promise ID in
   the shell's sources; report the rows it changes and the totals, or that it changes nothing.

8. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line, ruling's
   record, case, fixture, expected failure or core source changes. No CLI command (051b-2), no
   Interface listener for a return (PND-013's), no managed connection (050o) and no package (050).
   Report as findings, for the coordinator: that `server.rs`'s `client`, behind `conduit call`,
   cannot answer an `input_required` result, so A5's live check, which runs after this order
   merges, uses order 051b-2's `conduit sign-in` as its client; decision 5's DL18 ending, only
   where 048a-5h's input does not end the invocation; each seam an order before this one had
   already joined; and any place where IFC-049, PND-010 or PND-013 left you a choice this order
   does not settle.

## Scope

Change only `engine/crates/shell/src/server.rs` (decision 2 only, where a key is missing);
`engine/crates/shell/src/server_tests.rs`
(decision 6); `engine/CONFORMANCE.md` (one sentence, decision 7); and `engine/STANDING.md`,
written only by the tool. Not `engine/crates/core/`, `engine/crates/loader/`,
`engine/crates/conformance/`, `server/relay.rs`, `runner.rs`, `spec/`, `IN_FORCE.md`,
`conformance/`, `tools/`, `packages/`, `experiments/`, `docs/implementation/` or
`docs/proposals/`: report any defect as a finding.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell sign_in_relays_through_the_interface
cargo test --release -p conduit-shell sign_in_decline_through_the_interface
cargo test --release -p conduit-shell
cargo run --release -p conduit-conformance -- .. requests-
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third and the fourth each report 1 passed and 0 failed. The fifth reports every shell test
passed; report the count, and that of the tree you started from. The runner command ends with as
many cases passed as on the tree you start from, with its expected failures unchanged: this order
changes no case; report both counts. If any other test or case fails, stop and report it: do not
change it. The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, a negative control: show that the path fails without 048a-6's forwarding of
`signInReturn`. From `engine/`, one at a time, first save the file:

```bash
cp crates/shell/src/server.rs target/server.rs.joined
```

Then, in `crates/shell/src/server.rs`, remove by hand the line or lines in `call_tool` that add
`signInReturn` to the `invoke` input (048a-6's, or decision 2's where you added it), and nothing
else, and run:

```bash
cargo test --release -p conduit-shell sign_in_decline_through_the_interface
cp target/server.rs.joined crates/shell/src/server.rs
cargo test --release -p conduit-shell sign_in_decline_through_the_interface
```

The first run must fail at the second test's first request that carries `META/signInReturn`,
which is then answered as a first request without it, not as `invalid`; report its step and the
reply's kind, less any value of `R`. After the restore the second run reports 1 passed. Restore
with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not rebuild
the crate. Git ignores `target/`.

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
git status --short -- spec IN_FORCE.md conformance engine/crates/core engine/crates/loader engine/crates/conformance engine/crates/shell/src/server/relay.rs engine/crates/shell/src/runner.rs tools packages experiments docs/implementation docs/proposals
git diff --check
```

`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. The unit tests end `OK`; report the count. The `git status` command prints nothing.

The S1 short-form spike depends on the core, which this order does not change, so the local gate
does not run it for this change.

Each of these prints 1:

```bash
grep -c 'fn sign_in_relays_through_the_interface_and_carries_the_return' engine/crates/shell/src/server_tests.rs
grep -c 'fn sign_in_decline_through_the_interface_ends_failed' engine/crates/shell/src/server_tests.rs
grep -c 'fn established_case' engine/crates/shell/src/server_tests.rs
grep -c 'struct SignInScript' engine/crates/shell/src/server_tests.rs
```

Decision 7's edit, read across line breaks; the first prints 1, the second and the third 0:

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'runs through the shell: the request.s URL elicitation, signInLogin and signInReturn reach the core'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'and not yet .signInReturn. (order 048a-6)'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'the shell does not yet pass the key to the core'
```

Each of these prints 0:

```bash
grep -c 'apiKey' engine/crates/shell/src/server_tests.rs
git diff HEAD --stat -- engine/crates/core conformance spec | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, each seam with the order that joined it or the lines where you joined it, decision 5's
answer, and the sentence decision 7's sentence follows.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the change and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. Before
the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and the
merge does not wait for it. A fresh Codex verifier and a fresh read-only Opus reader read the
result before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes
this order's Record at the foot of this file when it merges.
