# Work order 048b-relay2 (E7b-2): calls toward a signIn Type declare URL elicitation, and a part's own input request reaches the core and is sent again as the core writes it

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the one case you change, with the promises it
tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, add no crate and no
crate feature, and reach no other machine. Run one cargo command at a time. Never print a whole
large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential. Do not run `claude`, `codex` or any client, and
sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md) plans "048b-relay: the shell relays
`input_required` (`server.rs`, `runner.rs`). M. After S0b." The coordinator cuts 048b-relay into
two, since an order holds one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)); the cut is the
coordinator's, open to the owner's strike:

- **048b-relay1**: the core's input-required output and its retry input declared, and the server
  relaying that output to the Interface that asked, with a requestState the shell issues, keeps
  in memory and checks. It follows nothing.
- **048b-relay2, this order**: the half order 117 gives to `runner.rs`. Calls for requests and
  hooks to an instance whose Type states `signIn` declare URL elicitation (IFC-045), written by
  the core in `requests.rs`'s `intent`; and a part that its implementation answers with its own
  `InputRequiredResult` reaches the core as a result, and is sent again as the core writes it,
  with that implementation's requestState and the responses. It comes after 048a-2 merged and
  048b-relay1, and order 048a-5d, the `sign-in-hook.v1` part, comes after it.

The orders that use what this one builds: 048a-5d dispatches `sign-in-hook.v1`, receives its
`InputRequiredResult` and keeps that implementation's requestState with the part; 048a-7 sends the
hook its retry, through this order's `retry_intent`, carrying that state, `accept` and the return;
048b-join shows the whole path over the engine's MCP server, and its test checks that the
`sign-in-hook.v1` call holds this order's declaration.

**What it builds.** IFC-045's `then`: "In the call's `io.modelcontextprotocol/clientCapabilities`
it declares no capability but Conduit's extension, where IFC-037 has it declare it, the tasks
extension where IFC-016 lets it, and elicitation URL mode {"url": {}} only for requests and hooks
to instances whose Type states signIn. It declares no form-mode elicitation and no elicitation for
provider calls." Its `observable`: "A call to an implementation marked `plain` declares no
extension of Conduit's, under IFC-007 and IFC-037, so it declares only tasks where allowed and URL
elicitation where that Type states signIn. A plain OAuth server whose Type states no signIn
therefore sees the same capability declaration as before." This order builds the URL elicitation
half of that sentence and nothing else of it (decision 8). And the protocol's carriage toward
the implementation: MCP 2026-07-28's `InputRequiredResult` carries the implementation's own
`requestState`, which a retry echoes exactly with `inputResponses` keyed as its `inputRequests`
were; IFC-049 keeps that value apart from the engine's own requestState. IFC-006 has
`sign-in-hook.v1`'s retry also carry `signInReturn`, which is 048a-7's. No promise changes.

**The two requestStates.** The coordinator's decision (a) of the reconciled plan binds: the shell
issues, keeps and checks the requestState it relays to the person's Interface (048b-relay1), and
the core holds none of it; an `invoke` that reaches the core is a first request and a `respond` a
retry. The implementation's own requestState, from `sign-in-hook.v1`'s `InputRequiredResult`, is
another value: the core keeps it with the part (048a-5d) and sends it back on the hook's retry
(this order's `retry_intent`, called by 048a-7), never relaying, recording or showing it. This
order builds only the second value's carriage; nothing here reads or writes
`engine/crates/shell/src/server/relay.rs`.

**What the runner already does, so that you do not redo it.** Line numbers are those of `main` at
`3c761dd9`, before orders 048a-2 and 048b-relay1; find each place by the text quoted with it.

- `Inner::call` (`engine/crates/shell/src/runner.rs:1199`) reads the core's `call` as a
  `ClientRequest` (`serde_json::from_value(call.clone())`, refusing with `invalid intent call`
  where it cannot) and attaches `ExactCall(call)`. In rmcp 3.9.0 `CallToolRequestParams` has
  `input_responses` (a map of string to JSON value) and `request_state` (a string), so a call
  that carries `inputResponses` and `requestState` reads.
- `Wire::send` (`runner.rs:1054`) replaces the serialized request's `method` and `params` with
  the core's, byte for byte, so whatever the core writes in `params`, `_meta` and both retry
  members included, is what the implementation receives.
- `Wire::receive` (`runner.rs:1083`) keeps each response's raw value before rmcp reads it, and
  `call` gives the completion `{"kind": "result", "result": R}`, `R` that raw `result`, for any
  result. rmcp's `ServerResult` holds an `InputRequiredResult` variant, so such a response is
  read and its raw value reaches the core whole.
- `fn intent` (`engine/crates/core/src/requests.rs:768`) writes every call for a request or a
  hook: for a plain implementation `{"name": T, "arguments": A}` with no `_meta`, and for a
  keyed one the same with a `_meta` of Conduit's keys. It writes no
  `io.modelcontextprotocol/clientCapabilities` at all. It is called in `invoke_inner`
  (`t.external_intents.push(intent(&invocation, c));`, line 1606), in `complete` for IFC-046's
  resend (`let mut call = intent(&v, c);`, line 1789) and in `hook` (`let mut call = intent(&v,
  c);`, line 2160), which then removes the `conduit` key.
- `complete`'s `-32021` arm carries the comment "IFC-045 declares only Conduit's extension: every
  other named capability is undeclared" (line 1805).
- The runner's integration tests (`engine/crates/shell/tests/runner.rs`) run the S1 project's
  filesystem implementation as the stdio process `scripted_server`, whose behavior a `mode` file
  selects, and record every line it receives in `wire.jsonl`.
  `plain_wire_environment_workdir_records_and_restart` asserts that a plain call's `params` are
  exactly `{"name": "read_text_file", "arguments": {"path": "hello.txt"}}`: S1's Types state no
  `signIn`, so that assertion is unchanged by this order and is IFC-045's other half shown end to
  end.

**What orders 048a-2 and 048b-relay1 bring, which this order assumes.** Both were written beside
this order's chain; it assumes what each brings, as their texts give it:

- **B1, from 048a-2.** `engine/crates/core/src/sign_in.rs`, with
  `pub(crate) enum ExpectedAccount<'a> { NoSignIn, NotString { pointer: String }, Set { pointer: String, account: &'a str } }`
  and `pub(crate) fn expected_account<'a>(instance: &'a Value, context: &Context) -> Result<ExpectedAccount<'a>>`,
  which gives `NoSignIn` without a Type lookup for an object that is not a Resource or an Agent,
  for a template (an object with `spec`) and for an object with no `held`, and otherwise looks up
  the instance's Type and gives `NoSignIn` where it states no `signIn.account`, `NotString` where
  the account at that pointer is not a string, and `Set` otherwise. Its engine errors come from
  the Type lookup.
- **B2, from 048a-2.** In `complete`, the arm
  `Completion::Result { result } if sign_in::url_elicitation(&result)`: a result whose
  `resultType` is `input_required` and whose `inputRequests` holds a URL `elicitation/create`
  ends the call `unknown` under IFC-016, plain or keyed, nothing of the result reaching the
  caller; for a Type that states no `signIn`, its cause text is
  `The implementation asked for a URL elicitation outside sign-in.v1, and the instance's Type states no signIn, so no sign-in action is raised.`
- **B3, from 048a-2.** The case `requests-sign-in-lapse-and-conflict`
  (`conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml`), JSON with two-space
  indentation, its `promises` `[PND-005, PND-010, IFC-016, PND-001]`, and its entry in
  `conformance/conventions/requests/index.yaml` with the same `promises`. Its Agents `signed`
  (account `primary-account`) and `unset` (account null) are of the Type `signed-agent.v1`, which
  states `signIn`, served by the keyed implementation `signed-server`. In it, root asks
  `to-signed.ping.v1` and `to-unset.ping.v1` more than once, and the first step of each is
  expected `{"reply": {"pending": true}}`: one call dispatched.
- **B4, from 048b-relay1.** `InputRequired` and `Transition.input_required` in
  `engine/crates/core/src/lib.rs`, the `respond` input, and `server/relay.rs` with its states.
  This order uses none of it in code: the plan puts it after 048b-relay1 so that the relay's two
  halves land in order, and so that the shell's requestState (B4's) and the implementation's
  (this order's) are never confused.

Orders 048a-3, 048a-4 and 048a-5 may land before this one. Order 048a-4 inserts a step into B3's
case and adds a function that dispatches setup's account check and sign-out parts; order 048a-5,
its decision 6, adds the request member `"clientCapabilities": {"elicitation": {"url": {}}}` to
B3's two `group.sign-in.v1` steps for `signed` (its steps 4 and 12 as 048a-2 numbers them), and
adds the case `requests-sign-in-refusals-before-parts`; 048a-3 changes other steps and files. Where any merged first, find the case's steps by their input
as decision 5 does, change every call of `intent` that `requests.rs` then holds (decision 1), and
say so. Where 048a-2 or 048b-relay1 merged otherwise than B1 to B4 say, apply the nearest sound
reading of each decision below to what merged, and say so.

**When this order starts.** Orders 048a-2 and 048b-relay1 must be merged to `main` first, or, by
the coordinator's dispatch note, this order starts from a verified branch that holds both. Before
you change anything, check from the repository root that you find what they give, and stop,
change nothing and report if any check prints otherwise:

```bash
git merge-base --is-ancestor 3c761dd9 HEAD; echo "main $?"
grep -c 'pub(crate) fn expected_account<' engine/crates/core/src/sign_in.rs
grep -c '^    NoSignIn,$' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn url_elicitation(result: &Value) -> bool {' engine/crates/core/src/sign_in.rs
grep -c 'sign_in::url_elicitation(&result)' engine/crates/core/src/requests.rs
test -f conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml; echo "case $?"
grep -c '"tool": "to-signed.ping.v1"' conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml
grep -c '"tool": "to-unset.ping.v1"' conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml
grep -c 'io.modelcontextprotocol/clientCapabilities\|IFC-045' conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml
grep -c '^pub struct InputRequired {$' engine/crates/core/src/lib.rs
grep -c '^mod relay;$' engine/crates/shell/src/server.rs
grep -c '^fn intent(v: &Value, c: &Context) -> Value {$' engine/crates/core/src/requests.rs
grep -c 'intent(&' engine/crates/core/src/requests.rs
sed -n '/^fn intent(/,/^}/p' engine/crates/core/src/requests.rs | grep -c 'clientCapabilities'
grep -c 'IFC-045 declares only Conduit.s extension: every other named capability' engine/crates/core/src/requests.rs
grep -c 'requestState\|inputResponses\|request_state\|input_responses' engine/crates/shell/src/runner.rs
grep -c '"input-required"' engine/crates/shell/tests/runner.rs
grep -A1 '^name = "rmcp"$' engine/Cargo.lock | grep -c '^version = "3.9.0"$'
grep -c '^| IFC-045 | interface | must | 0 | 0 | reason | core | reason, no case |$' engine/STANDING.md
```

They print `main 0`; `1`, `1`, `1` and `1` (B1 and B2); `case 0`, a number other than `0`, a
number other than `0` and `0` (B3); `1` and `1` (B4); `1`, a number other than `0`, `0` and `1`
(`intent` as this order finds it: report the second, the calls of `intent`); `0` and `0` (the
runner and its tests as they stand); `1`; and `1` (IFC-045's row). Orders that land beside this
one (048a-3 to 048a-5 and others): whichever lands second keeps both changes and writes
`engine/STANDING.md` again with the tool.

**Who decided what.** IFC-045 decides where the declaration sits, its value, that it is made for
requests and hooks to an instance whose Type states `signIn` and for no other call, and that a
plain call holds it too. The protocol (MCP 2026-07-28's `InputRequiredResult`) decides that a
retry echoes the implementation's `requestState` exactly with `inputResponses`, and IFC-049 only
that the engine's own requestState is another value. IFC-016 decides that nothing of an input
request outside `sign-in.v1` reaches a caller. Decision (a) of the coordinator's reconciled plan decides
which requestState is the shell's and which the core keeps. Everything else here is the
coordinator's, each open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):

- the cut of 048b-relay, and this order's place after 048a-2 and 048b-relay1 and before 048a-5d;
- reading "Type states signIn" as any instance for which `expected_account` gives other than
  `NoSignIn`, an account that is null included;
- deciding the declaration in `intent` for every call it writes, a hook's and IFC-046's resend
  included, and letting an engine error of the Type lookup end the step, as 048a-2's arms do;
- splitting `intent` so that the declaration can be tested without a registered Type;
- the retry as the part's first intent unchanged but for two members of `params`, with the
  responses as the caller gives them, and leaving to 048a-5d and 048a-7 where the state is kept,
  whether the retry is a new attempt and the `_meta` it adds;
- changing nothing in `runner.rs` and pinning what it does with a test;
- the case's two new expectations and IFC-045 added to its promises, so that IFC-045's standing
  is shown by a case;
- the comment's new words;
- the functions' and tests' names, and the test's scripted values.

Apply them as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: IFC-045, IFC-049, IFC-016, IFC-010, IFC-007 and IFC-006
   (`spec/interface/implementation.yaml`); CTR-013 (`spec/promises/contracts.yaml`).
2. [Order 048a2](048a2-build-sign-in-lapse-and-conflict.md), its decisions 5 and 10 and its
   Record; order 048b-relay1, its "Where it sits" and "The stand on requestState".
3. In `engine/crates/core/src/requests.rs`, `intent`, its three callers and `complete`; in
   `sign_in.rs`, `expected_account`; in `engine/crates/shell/src/runner.rs`, `ExactCall`, `Wire`,
   `dispatch_later` and `call`; in `engine/crates/shell/tests/runner.rs`, `register`,
   `scripted_server`, `plain_wire_environment_workdir_records_and_restart` and
   `deadlines_exit_repair_write_and_cancel`.
4. rmcp 3.9.0 in cargo's registry, read only: `src/model.rs`'s `CallToolRequestParams` and
   `ServerResult`, and `src/model/mrtr.rs`'s `InputRequiredResult` and `InputResponses`.
5. `conformance/README.md`, its sections "Assertions" and "The coverage index and the case schema
   in pieces".

**Size: S**: about 0.15 to 0.25k changed lines of Rust, most of them tests, in two files; two
expectations and one promise in one case, the same promise in its index entry; and
`engine/STANDING.md` written by the tool.

## The decisions it applies

1. **`intent` decides the declaration from the target's Type.** In `requests.rs`, change `fn
   intent(v: &Value, c: &Context) -> Value` to
   `fn intent(v: &Value, state: &State, c: &Context) -> Result<Value>`, with the doc comment
   `/// The call for a request's or a hook's dispatch. IFC-045: it declares URL elicitation only toward an instance whose Type states signIn, whatever its account holds.`
   Its body:
   - find the target with `access::object_by_id(state, &v["target"])`, as `finish_login_lapse`
     does;
   - where there is one, call `crate::sign_in::expected_account(object, c)?`, and let the flag be
     true where it gives `NotString` or `Set` and false where it gives `NoSignIn`; where there is
     none, the flag is false;
   - return `Ok(dispatch_intent(v, flag, c))` (decision 2).

   Change every call of `intent` in `requests.rs` to pass `&t.state` and to take its result with
   `?`, binding it first where the borrow checker asks:
   `let call = intent(&invocation, &t.state, c)?; t.external_intents.push(call);` in
   `invoke_inner`, `let mut call = intent(&v, &t.state, c)?;` in `complete` and in `hook`, and the
   same in any function another order added since (048a-4's part dispatcher among them). Each
   enclosing function already returns `Result<()>`. Report each call you changed. An engine error
   of the Type lookup now ends the step that dispatches; where any test or case fails so, stop
   and report it: do not catch it.

2. **The declaration, written in `dispatch_intent`.** Rename the present body of `intent` into a
   new function `fn dispatch_intent(v: &Value, url_elicitation: bool, c: &Context) -> Value`,
   unchanged but for one block: after the block that sets the keyed call's `_meta` (`= meta;`) for
   a keyed implementation, and before the final `json!`, add

   ```rust
   // IFC-045: URL elicitation is declared only toward an instance whose Type states signIn.
   if url_elicitation {
       call["params"]["_meta"]["io.modelcontextprotocol/clientCapabilities"] =
           json!({"elicitation": {"url": {}}});
   }
   ```

   For a keyed call the key joins Conduit's keys; for a plain call, which has no `_meta`, it makes
   one holding that key alone, which IFC-007 allows, since the key is the protocol's. Write the
   key as this literal once in the function. Nothing else of a call changes, and a call toward a
   Type that states no `signIn` is byte for byte what it was.

3. **The retry of a part, `retry_intent`.** Add, right after `dispatch_intent`:

   ```rust
   /// The retry of a part whose implementation answered with its own InputRequiredResult
   /// (MCP 2026-07-28): the part's first call as the core wrote it, unchanged but for
   /// `params.requestState`, that implementation's state, echoed exactly, and
   /// `params.inputResponses`, keyed as its input requests were. The runner sends it as written.
   /// The core keeps that state with the part and never relays, records or shows it.
   /// IFC-049: the requestState the shell relays to an Interface is another value.
   #[allow(dead_code)] // Orders 048a-5d and 048a-7 call it.
   pub(crate) fn retry_intent(
       first: &Value,
       request_state: &str,
       responses: &serde_json::Map<String, Value>,
   ) -> Value {
       let mut retry = first.clone();
       retry["call"]["params"]["requestState"] = json!(request_state);
       retry["call"]["params"]["inputResponses"] = Value::Object(responses.clone());
       retry
   }
   ```

   It changes no state, writes no record and pushes no decision. Its `invocation`, `attempt`,
   `target`, `implementation`, `_meta` and arguments are the first call's; whether a retry is a
   new attempt, and any `_meta` key it adds (`signInReturn`), are 048a-5d's and 048a-7's.

4. **The comment at `-32021`.** In `complete`'s arm for `Completion::ProtocolError { code: -32021,
   data }`, replace the two comment lines

   ```text
               // IFC-045 declares only Conduit's extension: every other named capability
               // is undeclared. Both IFC-008 and IFC-047 promise that nothing was performed.
   ```

   with

   ```text
               // IFC-045 has a call declare Conduit's extension, and URL elicitation only toward a Type
               // that states signIn. Both IFC-008 and IFC-047 promise that nothing was performed.
   ```

   The arm's code does not change: a keyed implementation's `-32021` stays failed under IFC-008
   or IFC-047 whatever it names.

5. **The case shows the declaration toward both signIn Agents.** In
   `conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml`, change only these:
   - Append `"IFC-045"` at the end of `promises`.
   - In the first step whose `input` is root's `invoke` of the tool `to-signed.ping.v1`, and in
     the first step whose `input` is root's `invoke` of the tool `to-unset.ping.v1`, add to
     `expect`, right after its `reply`, the member

     ```json
     "external_intents": [{"kind": "dispatch", "call": {"params": {"_meta": {"io.modelcontextprotocol/clientCapabilities": {"elicitation": {"url": {}}}}}}}]
     ```

     written in the file's two-space indentation. Under the runner's assertions the array has
     exactly one item, the maps are projections, and `{}` asserts an empty map. `signed`'s
     account is a string and `unset`'s is null, so the two steps show that the account plays no
     part (decision 1).

   Nothing else in the case, and nothing in its directory's other files, changes. In
   `conformance/conventions/requests/index.yaml`, append `"IFC-045"` at the end of that case's
   entry's `promises`, so that the entry and the case agree, and change nothing else. If either
   step gives more than one external intent, or a reply other than its case states, stop and
   report it: do not change the expectations to pass it.

6. **The runner is not changed; a test pins it.** Change nothing in
   `engine/crates/shell/src/runner.rs`. In `engine/crates/shell/tests/runner.rs`:
   - In `scripted_server`'s arm for `"tools/call"`, replace its last expression,
     `json!({"content":[{"type":"text","text":"test result"}],"isError":false})`, with an `if`
     whose first branch, taken where `mode == "input-required" && calls == 1`, gives
     `json!({"resultType":"input_required","inputRequests":{"sign-in":{"method":"elicitation/create","params":{"mode":"url","message":"Sign in to continue.","url":"https://example.org/authorize"}}},"requestState":"impl-state-1"})`,
     and whose `else` gives that former expression. No other mode changes.
   - Add the test `implementation_input_request_reaches_the_core_and_its_retry_is_sent_as_written`,
     built as `deadlines_exit_repair_write_and_cancel` builds its run, with `Config::default()`
     and the mode `input-required`. In this order:
     1. `register(&shell, parent.path(), false, "input-required")`, `new_runner(&shell)`; then
        `setup` of `workspace` and of `shared`, each asserted `succeeded`, and `setup` of
        `agent-b`, as that test does.
     2. `let ticket = shell.begin(input(&shell, "agent-b", "project.file.read.v1", json!({"path":"x"})), &runner).unwrap();`,
        take `let invocation = ticket.invocation().unwrap().to_owned();`, then
        `let reply = ticket.wait().unwrap();`. Assert: `outcome(&shell, &reply)` is `unknown`;
        some item of the reply's `structuredContent.diagnostics` has `ruleId` `IFC-016` and a
        `message.text` that contains `the instance's Type states no signIn` (B2: the result
        reached the core as a result, with its input request); and `reply.to_string()` contains
        neither `impl-state-1` nor `example.org/authorize` (IFC-016: nothing of it reached the
        caller).
     3. Read `shared/wire.jsonl` with `lines`. Let `first` be its first line whose `method` is
        `tools/call`; assert `first["params"]` equals
        `json!({"name":"read_text_file","arguments":{"path":"x"}})`: toward a Type that states no
        `signIn`, no `_meta` and no declaration (IFC-045). Print, with `eprintln!`,
        `initialize protocolVersion: ` followed by the `params.protocolVersion` of the wire's
        first `initialize` line, or `null` where there is none.
     4. Let `params` be `first["params"]` with `"requestState": "impl-state-1"` and
        `"inputResponses": {"sign-in": {"action": "accept"}}` added, and `retry` be
        `json!({"kind":"dispatch","invocation":invocation,"attempt":1,"target":id(&shell,"shared"),"call":{"method":"tools/call","params":params}})`,
        the members of retry_intent's form that the runner reads (decision 3). Assert that
        `runner.dispatch(&retry).unwrap()` is `Execution::Deferred`; then
        `until(|| call_count(parent.path()) == 2)`; then read the wire again and assert that the `params` of its second `tools/call` line equal `params`
        exactly: the runner sends a retry's `requestState` and `inputResponses` as written.

     The retry's own result reaches the core as a completion of an ended invocation, which the
     core ignores (`ignore-late-completion`); the test asserts nothing about it. Import
     `Executor` from `conduit_shell` if the file does not already. If step 2's diagnostic is not
     IFC-016's, or step 4's call is not sent or not sent as written, stop and report what the
     wire and the reply hold: do not change `runner.rs` to make it pass.

7. **Unit tests in `requests.rs`.** In its `tests`, add
   `intent_declares_url_elicitation_only_toward_sign_in_types`. Let `(_, c)` be
   `crate::explain::tests::fixture()`, `CAP` the string
   `io.modelcontextprotocol/clientCapabilities`, and `v`
   `json!({"id":"v1","attempt":1,"target":"s1","operation":"example.org/p#ping.v1","arguments":{"x":1},"implementation":{"item":{}},"caller":"r","conduit":"k","filters":[],"config":{},"revision":"1","key":null})`,
   a keyed call. Assert with `assert_eq!`:
   - the `call.params._meta` of `dispatch_intent(&v, false, &c)` is an object of 8 keys, none of
     them `CAP`;
   - `dispatch_intent(&v, true, &c)`'s `_meta` has 9 keys, its `CAP` is
     `{"elicitation":{"url":{}}}`, and with `CAP` removed the whole intent equals
     `dispatch_intent(&v, false, &c)`;
   - for `p`, `v` with `"plain": true` and `"implementation": {"item": {"tools": {"example.org/p#ping.v1": "ping_tool"}}}`,
     the `call.params` of `dispatch_intent(&p, false, &c)` equal
     `{"name":"ping_tool","arguments":{"x":1}}`, and with `true` equal
     `{"name":"ping_tool","arguments":{"x":1},"_meta":{CAP:{"elicitation":{"url":{}}}}}`;
   - `intent(&v, &state, &c).unwrap()` equals `dispatch_intent(&v, false, &c)` for each of three
     states built by hand as `State { objects: vec![O], ..State::default() }`: `O` the Agent
     `{"id":"s1","kind":"Agent","name":"signed"}`, which has no `held`; `O` the template
     `{"id":"s1","kind":"Agent","name":"signed","spec":{"type":"x"},"held":{}}`; and `O` the
     Agent `{"id":"other","kind":"Agent"}`, so that no object is the target. None needs a Type
     lookup (B1);
   - for `first = dispatch_intent(&v, true, &c)` and the responses
     `{"sign-in":{"action":"accept"}}` as a map, `retry_intent(&first, "impl-state-1", &map)`
     has `call.params.requestState` `impl-state-1` and `call.params.inputResponses` equal to the
     responses, and with both members removed from its `params` equals `first`.

   A Type that states `signIn` needs a registered project, so the case (decision 5) shows that
   path, and the runner test shows a registered Type that states none.

8. **`engine/STANDING.md`.** After the other decisions, run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a case's
   `promises`. IFC-045's row becomes
   `| IFC-045 | interface | must | 1 | 0 | reason | core | case passes |`, and the totals change
   only by what that row changes. Name no promise ID but IFC-045 and IFC-049 in new code,
   comments and the unit test, so that no other row changes; the integration test under
   `engine/crates/shell/tests/` is not read by the tool. If an order that landed first already
   lists IFC-045 in a case, report the row as the tool writes it.

9. **What this order does not do.** No promise, schema, definition, fixture, `IN_FORCE.md` line
   or ruling's record changes. `runner.rs`, `server.rs`, `server/relay.rs`, `lib.rs` and
   `sign_in.rs` do not change. No step calls `retry_intent` in this build; nothing keeps an
   implementation's requestState, no hook is retried, and `sign-in.v1` is not dispatched
   (048a-5b to 048a-5d, 048a-7). Conduit's extension (IFC-037) and the tasks extension (IFC-016)
   are not declared in a call's client capabilities, and provider calls do not change.
   `engine/CONFORMANCE.md` does not change. Report as findings, for the coordinator:
   - that no call the core writes declares Conduit's extension or the tasks extension in its
     `io.modelcontextprotocol/clientCapabilities`, which IFC-045 and IFC-037 name, and that
     `engine/CONFORMANCE.md` states neither;
   - the `initialize protocolVersion` line that decision 6's test prints, and that on a session
     at a revision before `2026-07-28` an implementation asks for input with a server-to-client
     `elicitation/create` request, which rmcp 3.9.0's default client handler, the runner's `()`,
     declines, so such a request never reaches the core and is not read under IFC-016;
   - that conformance/manifest.yaml's IFC-045 entry keeps its increment-2 reason while a case
     now lists IFC-045;
   - that the 2026-07-28 schema, as rmcp 3.9.0 states it, requires protocolVersion and
     clientCapabilities in each request's _meta, and which calls now hold neither or only one;
   - that every dispatch now looks up its target's Type, and whether the vectors' run time
     changed noticeably;
   - that `retry_intent` is unused outside its test until 048a-5d and 048a-7, and marked so;
   - any place where IFC-045 or IFC-049 left you a choice this order does not settle.

## Scope

Change only:

- `engine/crates/core/src/requests.rs`: `intent`, `dispatch_intent` and `retry_intent`
  (decisions 1 to 3), each call of `intent` (decision 1), the comment (decision 4) and the unit
  test (decision 7);
- `engine/crates/shell/tests/runner.rs`: `scripted_server`'s one branch and the new test
  (decision 6);
- `conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml` and that case's entry
  in `conformance/conventions/requests/index.yaml` (decision 5);
- `engine/STANDING.md`, written only by the tool (decision 8).

`docs/work-orders/048b-relay2-declared.yaml` comes with this order; check that it matches the
case's change and change it only to make it so. Not `engine/crates/shell/src/`, `sign_in.rs`,
`lib.rs`, `spec/`, `IN_FORCE.md`, `engine/CONFORMANCE.md`, `conformance/manifest.yaml`,
`conformance/case.schema.json`, `conformance/case-schema/`, `conformance/expected-failures.yaml`,
any other case or fixture, the case directory's other files, `tools/`, `packages/`,
`experiments/`, `docs/implementation/` or `docs/proposals/`: report any defect as a finding.
`experiments/011-short-form-spike/` changes only if its build needs it, since it depends on the
core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- intent_declares_url_elicitation_only_toward_sign_in_types
cargo test --release -p conduit-shell --test runner -- implementation_input_request_reaches_the_core_and_its_retry_is_sent_as_written --nocapture
cargo test --release -p conduit-shell --test runner -- plain_wire_environment_workdir_records_and_restart
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cargo run --release -p conduit-conformance -- .. requests-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third, fourth and fifth commands each report 1 passed and 0 failed; report the fourth's
`initialize protocolVersion` line. The first runner command ends `1 cases passed`; `requests-`
gives as many cases as on the tree you start from, with its expected failures unchanged; report
the count. If any other test or case fails, stop and report it: do not change it. The threads are
4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decision 1. From `engine/`, first keep a copy:

```bash
cp crates/core/src/requests.rs target/requests.rs.fixed
```

Then, by hand, in `crates/core/src/requests.rs`, change `intent`'s last line to
`Ok(dispatch_intent(v, false, c))`, and nothing else. Then, one at a time:

```bash
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
cp target/requests.rs.fixed crates/core/src/requests.rs
cargo run --release -p conduit-conformance -- .. requests-sign-in-lapse-and-conflict
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with an error at
the first step of root's `to-signed.ping.v1`, whose dispatch holds no client capabilities where
URL elicitation is expected; after the restore the second ends `1 cases passed`. Restore with
`cp`, not `mv`: a moved file keeps its older modification time, so cargo would not rebuild the
crate. Git ignores `target/`.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048b-relay2-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md engine/CONFORMANCE.md engine/crates/shell/src engine/crates/loader engine/crates/conformance conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/language conformance/conventions/requests/sign-in-lapse-and-conflict/manifest.json conformance/conventions/requests/sign-in-lapse-and-conflict/documents.json conformance/conventions/requests/sign-in-lapse-and-conflict/fixture.json tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048b-relay2-declared.yaml: 2 shapes.`,
then `Vector delta: N changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`; report N, and
that every change lies in `requests-sign-in-lapse-and-conflict`, in its `promises` and in the
two steps of decision 5. `promise_standing.py --check`, `check_model_account.py` and
`check_roadmap.py` each end `Errors: 0`. The unit tests, run with the interpreter that has PyYAML,
end `OK`; report the count. The `git status` command prints nothing.

The S1 short-form spike depends on the core, so the local gate runs it for this change, as spec
batch 1's Gate G asks. From `experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`.

Each of these prints 1:

```bash
grep -c '^fn intent(v: &Value, state: &State, c: &Context) -> Result<Value> {$' engine/crates/core/src/requests.rs
grep -c '^fn dispatch_intent(v: &Value, url_elicitation: bool, c: &Context) -> Value {$' engine/crates/core/src/requests.rs
grep -c '^pub(crate) fn retry_intent($' engine/crates/core/src/requests.rs
sed -n '/^fn dispatch_intent(/,/^}/p' engine/crates/core/src/requests.rs | grep -c '"io.modelcontextprotocol/clientCapabilities"'
grep -c '// IFC-045 has a call declare Conduit.s extension, and URL elicitation only toward a Type$' engine/crates/core/src/requests.rs
grep -c 'fn intent_declares_url_elicitation_only_toward_sign_in_types' engine/crates/core/src/requests.rs
grep -c 'fn implementation_input_request_reaches_the_core_and_its_retry_is_sent_as_written' engine/crates/shell/tests/runner.rs
grep -c '"IFC-045"' conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml
grep -c '"IFC-045"' conformance/conventions/requests/index.yaml
grep -c '^| IFC-045 | interface | must | 1 | 0 | reason | core | case passes |$' engine/STANDING.md
```

This prints 2:

```bash
grep -c '"io.modelcontextprotocol/clientCapabilities"' conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml
```

Each of these prints 0:

```bash
grep -c 'IFC-045 declares only Conduit.s extension' engine/crates/core/src/requests.rs
grep -c '^fn intent(v: &Value, c: &Context) -> Value {$' engine/crates/core/src/requests.rs
git diff HEAD --stat -- engine/crates/shell/src spec IN_FORCE.md engine/CONFORMANCE.md conformance/expected-failures.yaml | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, and give the last line of every check, each exit
status and count above, IFC-045's row and the totals as the tool writes them, each call of
`intent` you changed, the step numbers of decision 5's two steps in the case as you found it,
and the `initialize protocolVersion` line.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the changed files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048b-relay2-declared.yaml` through `--declared-changed`, and the merge does not
wait for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it
merges ([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's
Record at the foot of this file when it merges, naming
`intent_declares_url_elicitation_only_toward_sign_in_types` in the crate `conduit-core` as the
test for URL elicitation declared toward signIn Types, which order 048b-join's start runs (its
A5).
