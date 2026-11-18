# Work order 048b-relay1 (E7b-1): the server relays the core's input request to the Interface that asked, and checks the retry

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share; leave `vectors` empty, since no case changes. This order has network
access only because the engine's tests bind loopback ports, which the sandbox refuses without it;
use it for nothing else, add no crate and no crate feature, and reach no other machine. Run one
cargo command at a time. Never print a whole large file, case, fixture or log: use `head`, `tail`,
`grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an
environment variable's value, and never read or print a key, a token or any credential. Do not run
`claude`, `codex` or any client, and sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md), before the coordinator's cuts, planned
"048b-relay: the shell relays `input_required` (`server.rs`, `runner.rs`). M. After S0b." and
"048b-join: S-M. After 048a-6"; it now plans 048b-join after 048a-8 and 048b-relay2.
The coordinator cuts 048b-relay into two, since an order holds one behavior and stays small
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)) and the two halves
need different things first (the coordinator's split, open to the owner's strike):

- **048b-relay1, this order**: the core's input-required output and the input that carries a
  retry back to the core, declared; and `server.rs` relaying that output to the Interface that
  asked, as the protocol's `InputRequiredResult` with a requestState that the shell issues and
  holds, and checking each retry that echoes one. It needs nothing that is not on `main`.
- **048b-relay2, planned and not written**: the half that order 117 gives to `runner.rs`. Calls
  for requests and hooks to instances whose Type states `signIn` declare URL elicitation
  (IFC-045), and a part answered by an implementation's own `InputRequiredResult` is sent again
  with that implementation's requestState and the responses, when the core asks for it. The
  core writes each call whole, and the runner sends it as written (`ExactCall` in `runner.rs`).
  So IFC-045's declaration is a change to `requests.rs`'s `intent`, and it needs 048a-2's test
  of whether a Type states `signIn`. It comes after 048a-2 merged and this order, and order
  048a-5d comes after it.

**S0b, and what this order takes of it.** Order 117 puts 048b-relay after S0b, "the core's seams:
an operations registry, the coming State maps (tombstones, pending actions, machines) declared,
the core's input-required output declared, and `step`'s arms in modules". No order ever carried
S0b. At `e4ceb4c8`, the coordinator read what exists:

- An operations registry: none. The core's temporary stubs are a list in `invoke_inner`
  (`[("sign-in.v1", "048a", "PND-010")]`), and the shell's operations are
  `EXECUTABLE_ENGINE_OPERATIONS` in `server.rs`. This order does not need one.
- The State maps: `tombstones` (order 042) and `pending_actions` (order 047c, which declared the
  map itself) are declared, and `machines` is not. This order does not need any of them.
  `machines` belongs to the enrollment orders.
- The core's input-required output: not declared. `Transition` holds `reply` and
  `request_replies`, both final, and nothing that keeps an invocation in progress while it asks
  for input. This order declares it (decision 1), with the input that carries a retry back to
  the core (decision 2). Order 047c set the precedent by declaring its own map. S0b, if anyone
  writes it, finds both already declared.
- `step`'s arms in modules: `step_body` (`lib.rs`) calls a module for most arms and keeps some
  inline. This order does not need them moved. Its one new arm calls a function in
  `requests.rs`.

So this order starts from `main`. It has no predecessor in a chain, and no check below waits on
another order's branch.

**What it builds.** IFC-049's `then`: "With that capability tools/call may return
InputRequiredResult with resultType input_required and exactly one inputRequests entry:
elicitation/create with mode url, message and url." Its `observable`: "The engine correlates
retries through its own integrity-protected requestState, an opaque value bound to the Interface
that the request establishes under IFC-022 and to the originating invocation, expiring with DL18.
The client echoes it exactly; the engine treats it as attacker-controlled and rejects tampering,
replay of a consumed state or another principal. Each retry consumes its state; another
input-required response issues a fresh state for that same invocation." PND-013 adds: "IFC-049's
requestState checks come first: a retry they reject carries no return anywhere." This order builds
the carriage and those checks. The core decides what to ask and what a retry means. Order 048a-5e
gives the output, and order 048a-5f replaces this order's `respond` arm with polling. Until then
no step gives the output, so no state is ever issued, and every retry is refused before it reaches
the core. No promise changes.

**The stand on requestState.** IFC-049 asks for an opaque value whose integrity the engine
protects, and leaves its form open. In the plan of spec batch 1 for 048b, `server.rs` serves
"input_required with the opaque requestState handle". rmcp 3.9.0's documentation of its
`requestState` (`src/model/mrtr.rs`) names two forms: a sealed value, which needs its
`request-state` feature, or "keep the state server-side and use `requestState` only as an opaque
handle". This order takes the second. The shell issues a random handle and keeps what it binds in
memory. A handle it did not issue matches nothing, so tampering fails by lookup and no key is
needed. A restart forgets every handle, so a retry after a restart is refused as not issued. The
client need never retry (IFC-049), and the engine's recovery has already ended that invocation
unknown (DL6's row in `engine/CONFORMANCE.md`, IMP-007), so a forgotten handle never names an
invocation still waiting; the instance's sign-in action and hold stay, and the person sends a new
`sign-in.v1`, which no conflict refuses. The stand is the coordinator's, open to the owner's strike.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`e4ceb4c8`; find each place by the text quoted with it.

- `call_tool` (`engine/crates/shell/src/server.rs:396`) refuses a `serving` or `idempotencyKey`
  that is not a nonempty string. It records the call with `record_rejection(..., false)` and
  answers `ErrorData::invalid_params` with `data` `{META/reason: "invalid", "diagnostics": [...]}`
  naming IFC-036. Then it builds the input `{"kind":"invoke",...}`, calls `shell.begin` inside a
  `Pending`, and polls `pending.ticket.try_reply()`. A reply with `error` becomes that
  `ErrorData`. Any other reply becomes a `CallToolResult` and, through `.into()`,
  `CallToolResponse::Complete`. `Pending`'s `Drop` cancels the invocation only while its ticket
  holds no value.
- In rmcp 3.9.0, `CallToolRequestParams` has `request_state` and `input_responses`, and the
  handler reads neither today, so a `tools/call` that carries them is decided as a new request.
  `CallToolResponse::InputRequired(InputRequiredResult)` exists, and the SDK lets it reach only a
  peer on revision `2026-07-28` or later. Its URL elicitation always serializes an
  `elicitationId`, empty where none was given (rmcp's own test
  `test_url_elicitation_without_elicitation_id`).
- `record_rejection` (`server.rs:488`) has the core decide a call the shell rejects as unreadable.
  The core then writes its `access-decision` and its `request-ended` with reason `invalid` (OPS-023),
  as REC-001 asks for a request that ends without being dispatched.
- `Shell::commit` (`engine/crates/shell/src/lib.rs:553`) gives each entry of `t.request_replies`
  to the waiters of its `(invocation, attempt)`, removing them. Where the step's own reply is
  `pending`, it then registers the new ticket's waiter. `Shell::transition` (`lib.rs:362`) adds
  evidence to the input kinds in its `needs_evidence` list.
- `crate::identity::random_text(length)` (`identity.rs:21`) draws characters from `[a-z0-9]`
  with the OS random generator.
- The core already answers a protocol error as
  `{"error":{"code":-32602,"message":M,"data":{mark(c,"reason"):reason,"diagnostics":D}}}`
  (`requests.rs:713`).
- In `complete` (`requests.rs`), a plain implementation's `input_required` ends `unknown`
  (IFC-010). Once 048a-2 is in, a URL elicitation ends under IFC-016 and none of it reaches a
  caller. This order changes neither.

Below, `META/reason` is the key `reason` under the prefix that `spec/names.yaml` sets: in the core
`mark(c, "reason")`, in the shell `format!("{prefix}reason")`.

**When this order starts.** From `main`. Before you change anything, check from the repository
root that you find what this order assumes. Stop, change nothing and report if any check prints
otherwise:

```bash
git merge-base --is-ancestor e4ceb4c8 HEAD; echo "main $?"
grep -c 'pub request_replies: Vec<RequestReply>,' engine/crates/core/src/lib.rs
grep -c 'input_required\|InputRequired\|Respond' engine/crates/core/src/lib.rs
grep -c 'request_state\|input_responses\|InputRequired' engine/crates/shell/src/server.rs
grep -c '"respond"' engine/crates/shell/src/lib.rs
grep -A1 '^name = "rmcp"$' engine/Cargo.lock | grep -c '^version = "3.9.0"$'
grep -c '^pub(crate) fn random_text(length: usize) -> Result<String> {$' engine/crates/shell/src/identity.rs
grep -c '^mod session_tests;$' engine/crates/shell/src/server.rs
test -e engine/crates/shell/src/server/relay.rs; echo "relay $?"
grep -c -E '^\| IFC-049 \| interface \| must \| (0 \| 0 \| outside \| none \| no case, no reason|1 \| 0 \| outside \| core \| case passes) \|$' engine/STANDING.md
```

They print `main 0`, `1`, `0`, `0`, `0`, `1`, `1`, `1`, `relay 1` and `1`. The last accepts
IFC-049's row as `main` has it before order 048a-5 (`0 | 0 | outside | none`) and as 048a-5 writes
it (`1 | 0 | outside | core`), since 048a-5 may merge before this order; say which you find.
Other orders land beside this one: 048a-2 and 048a-3 change `requests.rs` and `State`, 048a-4 to
048a-6 change `call_tool`, `record_rejection` and `requests.rs`, and 049a-1 changes `Input`.
Whichever lands second keeps both changes and writes `engine/STANDING.md` again with the tool.

**Who decided what.** IFC-049 decides what the result holds, and that the requestState is
opaque, single-use and bound to the Interface and the invocation, expiring with DL18. PND-013
decides that the requestState checks come first. REC-001 decides that a refused request is
recorded. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- the cut of 048b-relay, and leaving IFC-045's declaration and the rounds toward an
  implementation to 048b-relay2;
- the stand on requestState above: a handle the shell keeps in memory, its form, and that a
  restart forgets it;
- the output's name, members and form, and the key `url` of its one entry in `inputRequests`;
- the retry input's name, members and form, and its arm's refusal in this order;
- binding a state to the tool name as well as to the Interface and the invocation;
- the order of the checks;
- one refusal, with one text, for a state not issued and a state issued to another Interface, so
  that a state's holder learns nothing about another's;
- refusing a retry that carries only one of `requestState` and `inputResponses`;
- consuming a state only when it is accepted;
- recording a refused retry through `record_rejection`, as IFC-036's refusals are recorded;
- the error's code, `-32602`, and its words;
- checking the output's form in the shell before relaying it, and cancelling the invocation
  when its output cannot be relayed;
- keeping rmcp's empty `elicitationId`;
- the decision's name, the unit tests and the server test.

Apply them as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: IFC-049, IFC-016 (its sentences on relaying), IFC-022, IFC-036 and IFC-045
   (`spec/interface/implementation.yaml`); PND-010 and PND-013 (`spec/promises/pending.yaml`);
   DL18 (`spec/implementation-defined.yaml`); REC-001 (`spec/conventions/records.yaml`); OPS-023
   (`spec/conventions/operations.yaml`).
2. `engine/crates/shell/src/server.rs`, whole; in `engine/crates/shell/src/lib.rs`, `Ticket`,
   `transition` and `commit`; in `engine/crates/shell/src/server_tests.rs`, `modern`, `call` and
   `shell_rejection_carries_diagnostic`.
3. In `engine/crates/core/src/lib.rs`, `Input`, `RequestReply`, `Transition` and `step_body`; in
   `requests.rs`, `mark`, the reply at line 713 and the head of its `tests` module.
4. rmcp 3.9.0 in cargo's registry, read only: `src/model/mrtr.rs` (`CallToolResponse`,
   `InputRequiredResult`, `InputRequests`, `InputResponses`), and in `src/model.rs`
   `CallToolRequestParams` and `ElicitRequestParams`.

**Size: M**: about 0.4 to 0.5k changed lines of Rust, about half of them tests, in one new file
and eight changed ones, with `engine/STANDING.md` written by the tool and one short section of
`engine/CONFORMANCE.md`.

## The decisions it applies

1. **The core's input-required output.** In `engine/crates/core/src/lib.rs`, right after
   `RequestReply`, add:

   ```rust
   /// IFC-049: an interim answer to one dispatched request attempt, which asks the Interface
   /// that sent it for input. The invocation stays in progress. The shell relays it and keeps
   /// its requestState. No step gives one until order 048a-5e.
   #[derive(Clone, Debug, Serialize)]
   pub struct InputRequired {
       pub invocation: String,
       pub attempt: u64,
       /// The one input request:
       /// {"method": "elicitation/create", "params": {"mode": "url", "message": M, "url": U}}.
       pub request: Value,
       /// The result's own `_meta`, such as IFC-049's signInLogin, or null for none.
       pub meta: Value,
       /// RFC 3339: the end of the URL action's wait (DL18), from which its requestState is refused.
       pub expires: String,
   }
   ```

   Add to `Transition`, right after `request_replies`,
   `#[serde(skip_serializing_if = "Vec::is_empty")] pub input_required: Vec<InputRequired>,`, so
   that a serialized transition is unchanged while the member is empty. In `step_body`'s literal,
   and in each other struct literal of `Transition` that names every member (in `explain.rs`,
   `records.rs`, `requests.rs` and `retirement.rs` at `e4ceb4c8`, tests included, and any literal
   another order added since; `cascade.rs` and `lifecycle.rs` hold none at `e4ceb4c8`), add
   `input_required: vec![]`, and say where. No step pushes to the member in
   this order.

2. **The retry's input, and its arm until 048a-5f.** In `Input`, right after `Complete`, add the
   variant `Respond { evidence: Evidence, invocation: String, attempt: u64, identity:
   access::Identity, #[serde(default)] responses: Value, #[serde(default)] meta: Value }`. Its
   tag, under the enum's `rename_all`, is `respond`. `responses` is the retry's `inputResponses`,
   and `meta` is its request `_meta`, which carries IFC-049's `signInReturn` once order 048a-7
   reads it. In `step_body`, its arm calls a new function in `requests.rs`,
   `pub(crate) fn respond(t: &mut Transition, invocation: &str, attempt: u64, c: &Context)`, which:
   - changes no state and writes no record;
   - pushes one decision named `refuse-input-response`, citing `IFC-049`, with no implementation
     choice, the facts `{"invocation": I, "attempt": N}` and no affected object;
   - sets `t.reply` to
     `{"error": {"code": -32602, "message": T, "data": {META/reason: "invalid", "diagnostics": [{"ruleId": "IFC-049", "level": "error", "message": {"text": T}}]}}}`,
     where `T` is
     `invocation {invocation} has no input request outstanding, so the retry was not passed on; no request in this build asks for input yet (order 048a-5e)`.

   The arm reads no other member. Where rustc or clippy reports one of them as never read, put
   `#[allow(dead_code)]` on that field alone, with the comment `// Order 048a-5f reads it.`, and
   say which fields. `reply_context` does not list `Respond`. Through the server, no retry
   reaches this arm in this build: the shell passes on only a retry whose state it issued, and it
   issues one only for an output, which no step gives yet. Order 048a-5f replaces the function's
   body.

3. **The shell gives the output to its waiters.** In `Shell::commit`, after the block that
   registers the new ticket's waiter (`let invocation = if t.reply["pending"] == true {`) and
   before `stored.state = Arc::new(std::mem::take(&mut t.state));`, take each entry of
   `t.input_required` in order. Remove the waiters of its `(invocation, attempt)`. Set each live
   cell's value to `{"inputRequired": V}`, `V` being the entry as `serde_json::to_value` gives
   it, and notify as the replies are notified. An entry with no waiter changes nothing. The
   entries come after that block, so a step that both makes an invocation pending and asks for
   input still reaches the new ticket. In `Shell::transition`, add `"respond"` to
   `needs_evidence`.

4. **The states, in a new file `engine/crates/shell/src/server/relay.rs`.** Declare it in
   `server.rs` with `mod relay;` (not under `#[cfg(test)]`), right before `#[cfg(test)] mod
   session_tests;`. Its items are visible to `server.rs` alone: say `pub(super)` on `States`,
   `Taken`, `take`, `relay` and the refusal-text helper (if any), and keep `Issued` and the fields
   private. The relay tests are a child module and still see the private fields.
   - `struct Issued { identity: String, tool: String, invocation: String, attempt: u64, expires: DateTime<Utc> }`,
     `#[derive(Default)] struct States { issued: BTreeMap<String, Issued>, consumed: BTreeMap<String, (String, DateTime<Utc>)> }`,
     where a consumed entry keeps its identity and expiry, and
     `#[derive(Debug, PartialEq, Eq)] enum Taken { Accepted { invocation: String, attempt: u64 }, NotIssued, Consumed, Expired, OtherTool }`.
   - `fn issue(&mut self, identity: &str, tool: &str, invocation: &str, attempt: u64, expires: DateTime<Utc>, now: DateTime<Utc>) -> Result<String>`:
     first remove every entry, issued or consumed, whose expiry is at or before `now`. Then the
     handle is `crate::identity::random_text(43)?`, drawn again while it equals an issued or
     consumed handle. Store it and return it. Each call issues a fresh state, for the same
     invocation too.
   - `fn take(&mut self, state: &str, identity: &str, tool: &str, now: DateTime<Utc>) -> Taken`:
     where `state` is issued: to another identity, `NotIssued`, and nothing changes; with expiry
     at or before `now`, remove it and give `Expired`; for another tool, `OtherTool`, and nothing
     changes; otherwise move it to `consumed` and give `Accepted` with its invocation and attempt.
     Where `state` is consumed, `Consumed` if its identity is `identity` and its expiry is after
     `now`, else `NotIssued`. Where it is neither, `NotIssued`. After deciding, remove every
     entry, issued or consumed, whose expiry is at or before `now`. A refusal never consumes a
     state.
   - `fn relay(value: &Value, states: &mut States, identity: &str, tool: &str, now: DateTime<Utc>) -> Result<InputRequiredResult>`,
     `value` being an output as decision 3 gives it. Check each of these first:
     - the member `request`'s `method` is `elicitation/create`;
     - its `params` is an object whose keys are exactly `mode`, `message` and
       `url`, with `mode` `url` and `message` a string;
     - `url` is a string that `url::Url::parse` reads with scheme `https`, an empty
       `username()` and no `password()`;
     - the member `invocation` is a string and `attempt` a `u64`;
     - `expires` is a string that `DateTime::parse_from_rfc3339` reads.

     Where any check fails, give the error
     `the core's input request is not a URL elicitation with an https URL and an expiry; nothing was relayed`
     and issue nothing. Otherwise issue a state with that expiry, in UTC. Then deserialize
     `{"resultType": "input_required", "inputRequests": {"url": R}, "requestState": S}`, `R` being the member `request`,
     into `InputRequiredResult`, with `"_meta": M` added only where the member `meta`, `M`, is
     an object. Where that fails, remove the state just issued and give the error
     `invalid core input request`. IFC-016 lets no other input request pass. The core checks the
     URL under IFC-049, and this check only keeps the shell from relaying anything else.
   - The refusal texts, each `T` below. In each, `requestState` and `inputResponses` are the
     protocol's words:
     - `NotIssued`:
       `requestState is not one this engine issued to this Interface: it was altered, issued to another Interface, or forgotten at a restart`;
     - `Consumed`:
       `requestState was already used: each retry consumes its state, and an answer that asks again carries a fresh one`;
     - `Expired`: `requestState expired with its URL action's wait (DL18)`;
     - `OtherTool`:
       `requestState was issued for another tool: a retry repeats the tool of the request it answers`;
     - a retry carrying one of `requestState` and `inputResponses` without the other:
       `a retry carries both requestState and inputResponses`.

     Each is answered with `ErrorData::invalid_params(T, Some(D))`, where `D` is
     `{META/reason: "invalid", "diagnostics": [{"ruleId": "IFC-049", "level": "error", "message": {"text": T}}]}`.
     That is the form `call_tool` gives IFC-036's refusals.

5. **The server relays and checks.** In `server.rs`, `Handler` gains
   `relay: Arc<Mutex<relay::States>>`, set to `Arc::default()` in `serve`. In `call_tool`:
   - **A retry.** After the loop over `["serving", "idempotencyKey"]` and before the `invoke`
     input is built, read `request.request_state` and `request.input_responses`. Where both are
     absent, go on as today. Where only one is present, it is refused with the last text of
     decision 4. Where both are present, call `take` with the authenticated identity, the
     request's `name` and `Utc::now()`, and a result other than `Accepted` is refused with its
     text. Every refusal in this block, the one-field case included, calls
     `record_rejection(self, Some(&identity), &json!({"params":{"name":request.name,"arguments":request.arguments,"_meta":meta}}), false)`,
     then `self.handled(&context)`, then returns the error. On `Accepted { invocation, attempt }`,
     the input is
     `{"kind":"respond","invocation":invocation,"attempt":attempt,"identity":instance(&identity),"responses":R,"meta":meta}`,
     `R` the responses as a JSON object, in place of the `invoke` input. The same `begin`,
     `Pending` and loop follow. The retry's `arguments` are not read: the invocation keeps those of
     its first request. These checks come before anything reaches the core, as PND-013 asks, and
     apply whatever the session's revision.
   - **An output.** In the loop, where `try_reply` gives a value, check first whether it has the
     member `inputRequired`. If it does, call `relay` with that member, the states, the
     authenticated identity, the request's `name` and `Utc::now()`, and return
     `Ok(CallToolResponse::InputRequired(result))`. The `Pending` guard does not cancel, since
     its ticket holds a value, and the invocation stays in progress. On a relay error, take
     `pending.ticket.invocation()` and pass `{"kind":"cancel","invocation":I}` to `shell.begin`
     inside `spawn_blocking`, as the cancel arm does, but do not propagate the cancel's result
     with `?`: return `internal(T)` with the relay error's text whether or not the cancel
     succeeds, and where the cancel fails, `T` is followed by
     `; cancelling invocation {I} failed too: {E}`, `E` being the cancel's error. Otherwise the
     reply is read as today.
   - **The lock.** Map a poisoned `relay` lock, wherever it is taken, to
     `internal("relay lock poisoned")`.

   Name no promise ID but IFC-049 in new code, comments and tests, so that only IFC-049's row of
   `engine/STANDING.md` changes. Name `signInReturn` nowhere in `server.rs` or `server/relay.rs`,
   not even in a comment: orders 048a-5 and 048a-6 rely on `server.rs` not naming it until 048a-6
   forwards it.

6. **The tests.** Let `T0` be `2026-11-18T12:00:00Z` and `X` be `2026-11-18T12:10:00Z`, as
   `DateTime<Utc>`.
   - In `requests.rs`'s `tests`, `respond_is_refused_until_input_requests_are_built`: from
     `explain::tests::fixture()`, an `Input` read with `serde_json::from_value` from
     `{"kind":"respond","evidence":E,"invocation":"absent-invocation","attempt":1,"identity":{"kind":"instance","id":"root"},"responses":{"url":{"action":"accept"}},"meta":{}}`,
     and stepped with `step`. `Evidence` has no `Serialize`, so write `E` as the literal JSON
     `{"time":"2026-11-09T13:00:00Z","ids":[...]}`, where ids are the fixture's form
     (`format!("00000000-0000-4000-8046-a0000000{n:04}")` for n in `1..50`), built with `json!`
     in the test. Assert with
     `assert_eq!`: the reply's `error.code` is `-32602`, its `error.data` holds `META/reason`
     `invalid` and one diagnostic whose `ruleId` is `IFC-049`; `records` and `input_required` are
     empty; the state equals the fixture's; and the one decision's `action` is
     `refuse-input-response`.
   - In `relay.rs`'s own `tests`, `request_states_bind_interface_tool_and_single_use`. Let
     `s1 = issue("i1", "t1", "v1", 1, X, T0)`: it has 43 characters, each in `[a-z0-9]`. Then,
     in this order: `take(s1, "i2", "t1", T0)` is `NotIssued`; `take` of `s1` followed by `x`,
     for `i1`, is `NotIssued`; `take(s1, "i1", "t2", T0)` is `OtherTool`;
     `take(s1, "i1", "t1", T0)` is `Accepted { invocation: "v1", attempt: 1 }`; the same again is
     `Consumed`; and `take(s1, "i2", "t1", T0)` is `NotIssued`. Next,
     `s2 = issue("i1", "t1", "v1", 1, X, T0)` differs from `s1`; `take(s2, "i1", "t1", X)` is
     `Expired`; then `take(s2, "i1", "t1", T0)` and `take(s1, "i1", "t1", T0)` are each
     `NotIssued`, since the step at `X` removed both. Last, `s3 = issue("i1", "t1", "v2", 2, X, T0)`,
     then `s4 = issue("i1", "t1", "v2", 3, X + 10 minutes, X)`, which removes `s3`:
     `take(s3, "i1", "t1", T0)` is `NotIssued` and `take(s4, "i1", "t1", X)` is
     `Accepted { invocation: "v2", attempt: 3 }`.
   - In `relay.rs`'s `tests`, `relay_passes_only_a_url_elicitation`. Let `V` be
     `{"invocation":"v1","attempt":1,"request":{"method":"elicitation/create","params":{"mode":"url","message":"Sign in","url":"https://example.org/login"}},"meta":{"k":"v"},"expires":"2026-11-18T12:10:00Z"}`.
     `relay(V, ..., "i1", "t1", T0)` gives a result whose `serde_json::to_value` equals
     `{"resultType":"input_required","inputRequests":{"url":{"method":"elicitation/create","params":{"mode":"url","message":"Sign in","url":"https://example.org/login","elicitationId":""}}},"requestState":S,"_meta":{"k":"v"}}`.
     `take(S, "i1", "t1", T0)` is then `Accepted`. `V` with `meta` null gives a result with no
     `_meta`. On a fresh `States`, each of these gives an error and leaves `issued` empty: `V`
     with `mode` `form`; with `url` `http://example.org/login`; with `url`
     `https://example.org/login` given the user information `user`, written in the test as `url::Url` writes it; with `params` also holding `"requestedSchema": {}`; with
     `method` `sampling/createMessage`; with `expires` `soon`; and without `attempt`. If rmcp
     serializes the entry otherwise than stated, stop and report what it gives. Do not change
     the expectation.
   - In `server_tests.rs`, `forged_retry_state_is_refused_and_recorded`, built as
     `shell_rejection_carries_diagnostic` is, with root's token and the `modern` client
     (revision `2026-07-28`). Count the `request-ended` records in the store's state. Send
     `client.call_tool_once` with `call("management.list.v1", json!({}))` given, through
     `serde_json`, `"requestState": "forged"` and `"inputResponses": {"url": {"action": "accept"}}`.
     It gives an error whose code is `-32602`, whose `data` holds `META/reason` `invalid`, and
     whose first diagnostic's `ruleId` is `IFC-049` with the `NotIssued` text. The state holds
     one more `request-ended` record, whose `data.reason` is `invalid`. Next, the same call with
     `requestState` alone gives the last text of decision 4, and one more such record. Last, the
     plain call through `client.call_tool` succeeds with `isError` false. The test uses nothing
     from `relay.rs`.

7. **`engine/STANDING.md`.** After the other decisions, run
   `python3 tools/promise_standing.py --write`, as AGENTS.md asks of a change to a promise ID in
   the core's and the shell's sources. IFC-049's row becomes
   `| IFC-049 | interface | must | 0 | 0 | outside | core, shell | no case, no reason |`, and the
   totals' last row, `named in engine source`, rises by one in `interface` and in `all`. That is
   the only change. Where order 048a-5 merged first, its row
   `| IFC-049 | interface | must | 1 | 0 | outside | core | case passes |` becomes
   `| IFC-049 | interface | must | 1 | 0 | outside | core, shell | case passes |`, and the totals do
   not change, since the core already named IFC-049. If any other order that landed first named
   IFC-049 in the engine's sources, report the rows as the tool writes them.

8. **What this order does not do.** No promise, schema, definition, case, fixture,
   `IN_FORCE.md` line or ruling's record changes. Nothing gives an output, so nothing is relayed
   in this build. `sign-in.v1` keeps its stub, and the core answers no retry otherwise than
   decision 2 says (048a-5f). No call declares URL elicitation and `runner.rs` does not change
   (048b-relay2). Task input requests, through `tasks/get` and `tasks/update`, are not read.
   Root's CLI client, `server::client`, is unchanged, and so is anything shown in the engine's
   panel (051b). DL18's wait is not chosen, and `engine/CONFORMANCE.md`'s DL18 row stays.
   Report as findings, for the coordinator:
   - that `engine/CONFORMANCE.md` states both rmcp 3.9.0's `"elicitationId": ""` in every URL
     elicitation, where IFC-049 says "There is no elicitationId", and that a restart forgets
     every state, by this order's stand;
   - that a refused retry is recorded as an unreadable request, invalid under OPS-023, and its
     record does not name IFC-049;
   - that the core's interim respond arm writes no record (REC-001) and must not survive
     048a-5f;
   - that IFC-049 does not say which code a refused retry gets;
   - that the SDK gives an `InputRequiredResult` only to a peer on `2026-07-28`, and that
     IFC-049's capability check on the request, which 048a-5 builds, is what keeps the core from
     asking an older one;
   - that the shell does not check that the Interface that asked is an Interface, since the
     core decides who may be asked (IFC-049, PND-002);
   - that `engine/README.md`'s paragraph on `Transition` ("A `Transition` returns state,
     decisions, new records, external_intents, reply and request_replies.") omits
     `input_required`, and that its sentence on a pending request's Ticket, which "waits for a
     core ending", does not say that a ticket may now hold an interim `inputRequired` value;
   - any place where IFC-049 or PND-013 left you a choice this order does not settle.

## Scope

Change only:

- `engine/crates/core/src/lib.rs`: decision 1's struct and member, and decision 2's variant and
  arm;
- `engine/crates/core/src/requests.rs`: `respond` and its test (decisions 2 and 6), and its
  struct literals of `Transition` (decision 1);
- each other struct literal of `Transition` that names every member, only to add the member
  empty (decision 1);
- `engine/crates/shell/src/lib.rs`: `commit` and `transition` (decision 3);
- `engine/crates/shell/src/server.rs`: decision 5, and `mod relay;`;
- `engine/crates/shell/src/server/relay.rs`, new (decisions 4 and 6);
- `engine/crates/shell/src/server_tests.rs`: one new test (decision 6);
- `engine/STANDING.md`, written only by the tool;
- `engine/CONFORMANCE.md`: one new last section, after the file's last line, and nothing else:
  a blank line, the heading `## IFC-049's relay (order 048b-relay1)`, a blank line and one
  paragraph. The file's last section on `main`, "Tier 2 for inspectors and closed Conduits (order
  046b)", is about another matter, so the paragraph takes its own heading. Write the paragraph as
  it stands here, wrapped within 100 columns as the file is:

  ```text
  IFC-049's URL action is relayed by the shell (order 048b-relay1). Its requestState is a random
  handle of 43 characters that the shell issues and keeps in memory, bound to the authenticated
  Interface, the tool, the invocation and its attempt, and expiring at the output's DL18 instant;
  each retry is checked there before anything reaches the core, and an accepted one consumes its
  handle. A restart forgets every handle; the engine's recovery ends the invocations they named
  unknown (DL6), so a retry after it is refused as not issued. rmcp 3.9.0 writes every URL
  elicitation with `elicitationId` empty: the engine correlates nothing by it and sends no
  completion notification, which is how this build reads IFC-049's "There is no elicitationId".
  ```

Not `runner.rs`, `spec/`, `IN_FORCE.md`, `conformance/`, `engine/README.md`, `tools/`,
`packages/`, `experiments/`, `docs/implementation/` or `docs/proposals/`: report any defect as a
finding. `experiments/011-short-form-spike/` changes only if its build needs it, since it depends
on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- respond_is_refused_until_input_requests_are_built
cargo test --release -p conduit-shell --lib -- relay::tests
cargo test --release -p conduit-shell --lib -- forged_retry_state_is_refused_and_recorded
cargo test --release -p conduit-shell --lib -- shell_rejection_carries_diagnostic
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third and fifth commands each report 1 passed, the fourth 2 passed and the sixth 1 passed.
Report the count if another test's name matches. If any other test fails, stop and report it. Do
not change it. The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the server test fails without decision 5. From `engine/`, one at a time:

```bash
cp crates/shell/src/server.rs target/server.rs.fixed
git show HEAD:engine/crates/shell/src/server.rs > crates/shell/src/server.rs
cargo test --release -p conduit-shell --lib -- forged_retry_state_is_refused_and_recorded
cp target/server.rs.fixed crates/shell/src/server.rs
cargo test --release -p conduit-shell --lib -- forged_retry_state_is_refused_and_recorded
```

The first test command must fail. The forged retry is decided as a new `list.v1` request and
succeeds where the IFC-049 error is expected. After the restore, the second reports 1 passed.
Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild the crate. Git ignores `target/`. With `HEAD`'s `server.rs`, `relay.rs` is no module and
is not built. If the crate does not build all the same, say why.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance engine/README.md engine/crates/shell/src/runner.rs engine/crates/loader engine/crates/conformance tools packages experiments docs
git diff --check
```

`vector_delta.py` prints `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.` and then
`exit 0`. `promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. The unit tests, run with the interpreter that has PyYAML, end `OK`; report the
count. The `git status` command prints nothing.

The S1 short-form spike depends on the core, so the local gate runs it for this change, as spec
batch 1's Gate G asks. From `experiments/011-short-form-spike/`:

```bash
cargo test --release
```

Its `tests/spike.rs` run reports `test result: ok. 9 passed; 0 failed`.

Each of these prints 1:

```bash
grep -c '^pub struct InputRequired {$' engine/crates/core/src/lib.rs
grep -c 'pub input_required: Vec<InputRequired>,' engine/crates/core/src/lib.rs
grep -c '^    Respond {$' engine/crates/core/src/lib.rs
grep -rl '"refuse-input-response"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'fn respond_is_refused_until_input_requests_are_built' engine/crates/core/src/requests.rs
grep -c '"respond"' engine/crates/shell/src/lib.rs
grep -c '"inputRequired"' engine/crates/shell/src/lib.rs
grep -c '"inputRequired"' engine/crates/shell/src/server.rs
grep -c 'CallToolResponse::InputRequired' engine/crates/shell/src/server.rs
grep -c '^mod relay;$' engine/crates/shell/src/server.rs
grep -c 'fn request_states_bind_interface_tool_and_single_use' engine/crates/shell/src/server/relay.rs
grep -c 'fn relay_passes_only_a_url_elicitation' engine/crates/shell/src/server/relay.rs
grep -c 'fn forged_retry_state_is_refused_and_recorded' engine/crates/shell/src/server_tests.rs
grep -c -E '^\| IFC-049 \| interface \| must \| (0 \| 0 \| outside \| core, shell \| no case, no reason|1 \| 0 \| outside \| core, shell \| case passes) \|$' engine/STANDING.md
grep -c "^## IFC-049's relay (order 048b-relay1)$" engine/CONFORMANCE.md
tr '\n' ' ' < engine/CONFORMANCE.md | grep -c 'which is how this build reads IFC-049'
```

This prints a number other than `0`; report it:

```bash
grep -c 't.input_required' engine/crates/shell/src/lib.rs
```

Each of these prints 0:

```bash
grep -c '^| IFC-049 | interface | must | 0 | 0 | outside | none |' engine/STANDING.md
{ git diff HEAD -- engine/crates/shell/src/server.rs | grep '^+'; cat engine/crates/shell/src/server/relay.rs; } | grep -c 'signInReturn'
git diff HEAD --stat -- engine/crates/shell/src/runner.rs spec conformance IN_FORCE.md | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, and give the last line of every check, each exit
status and count above, IFC-049's row and the totals' last row as the tool writes them, each
struct literal of `Transition` you changed, each field you marked `#[allow(dead_code)]`, the
line in `call_tool` where a retry is taken and where an output is relayed, and the line in
`commit` where the `input_required` entries are given to their waiters, with the line of
`let invocation = if t.reply["pending"] == true {`, so that a reader sees the entries come after
that block (decision 3).

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new file and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`. CI runs on the pull request as well, and the
merge does not wait for it. A fresh Codex verifier and a fresh read-only Opus reader read the
result before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes
this order's Record at the foot of this file when it merges.
