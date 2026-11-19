# Work order 051b2 (C1b-2): `conduit sign-in` asks the person, opens the browser and retries to the end

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: this order adds no case, so `vectors` is an empty list, and its
tests go under `checks`. This order has network access only because the shell's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, add no crate and no
crate feature, and reach no other machine. Run one cargo command at a time. Never print a whole
large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run
`env`, `printenv`, `set` or `export -p`, never print an environment variable's value, and never
read or print a key, a token or any credential. Test code may read the client token it issued in
its own temporary store, as `server::client` does, and nothing prints it. Do not run `claude`,
`codex` or any client, sign in nowhere, open no browser and no URL, and never run the built
`conduit sign-in` command yourself: only the tests below drive the new code, through a scripted
person whose browser is a list.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s batch 1 gives "051b: the pending-action
commands `pending`, `sign-in` and `answer`. M. After 047 and 048b-join." Spec batch 1's plan adds
the client's URL elicitation: "show the URL and the message, ask consent, open the browser". The
coordinator cut 051b into three, each holding one behavior
([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264); the coordinator's
split, open to the owner's strike): [051b](051b-cli-pending-and-answer.md), `conduit pending`,
`conduit answer` and the unanswered hint, merged (#372); **051b-2, this order**, `conduit
sign-in`; and [051b-3](051b3-cli-pasted-return.md), the carried return from the command line.
On 2026-11-19 the owner chose the dogfood path's shape, in two answers that
[ruling 88](../proposals/ruling-88/README.md) records with the owner's words. The first,
"Defer 7 and 8 (Recommended)", moves orders 048a-7 and 048a-8 (the carried return: PND-013's checks,
and a page login's code or a redirect return passed to the hook on the retry) and 051b-3 off 0.2.0's
path and the dogfood path, to after dogfood, beside the keyed release: under
[ruling 86](../proposals/ruling-86/README.md), point 1, nothing is keyed in 0.2.0, so PND-013's
first check fails for every 0.2.0 package and the return has no 0.2.0 user, and a Mac browser login
needs no return. 048b-join then follows 048a-5gh directly and joins the URL relay and polling only.
The second, "Pairs: 5e+5f, 5g+5h (Recommended)", batches the rest of the sign-in chain: 048a-5ef,
the engine's message and the input-required output, with `signInLogin`'s members and DL18's expiry,
and polling on the retry; and 048a-5gh, another account found while polling undone by a sign-out
part, and DL18's end. The path to the Mac dogfood is order 209, 048a-5d, 048a-5d2, 048a-5ef,
048a-5gh, 048b-join, then **this order**; 050n, the 0.2.0 packages joined to `main`, follows
048b-join and A5's live check, whose client this command is. A dogfood slot on the Mac signs in
through `conduit sign-in` with its client's default login (ruling 86, point 1;
[ruling 85](../proposals/ruling-85/README.md), point 5). This order is the last link of that path:
under [ruling 89](../proposals/ruling-89/README.md), point 3, the first dogfood milestone, the
Agents of the owner's primary Claude and Codex accounts signed in through Conduit on the Mac,
comes after 050n and this order, and the owner signs each of its slots in with this command; A5's
live check and the Claude scratch-home sign-in of its sitting use it as their client
([experiment 027's amendment](../../experiments/027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs),
items 2 and 6). Ruling 89, point 6, pauses A77's pairing until 050n merges, so this order is not
paired ([work orders](../implementation/WORK_ORDERS.md#amendment-of-2026-11-19-pairing-paused-until-050n-merges)).

**This text's refresh.** The coordinator wrote this order against `main` at `3c761dd9` and refreshed
it on 2026-11-19 against `main` at `1d96faf2`, where orders 051b (#372), 048b-relay1 (#373),
048b-relay2 (#377), 048a-5 (#382), 048a-6 (#386), 048a-5b (#387) and 048a-5c (#397), ruling 86's
record, order 207 (#396, #399) and ruling 87's record (#400) have merged, and against the texts,
not yet on `main`, of order 209 and the refreshed 048a-5d, and the owner's two answers above. It was written ahead, beside the
texts of 048a-5d2, 048a-5ef, 048a-5gh and the refreshed 048b-join, none of them merged, so each
assumption that rests on them is named below and the start check stops where one is missing. Every
change below is the coordinator's, open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- the predecessors are now 048b-join, as refreshed to join the URL relay and polling only, and
  through it orders 209, 048a-5d, 048a-5d2, 048a-5ef and 048a-5gh; 048a-7, 048a-8 and 051b-3 are
  no longer before or right after this order, and the texts that name 051b-3 say it comes with
  the keyed release;
- the start check stops unless every predecessor has merged with its Record; the earlier text's
  dispatch from a verified stacked branch is withdrawn, since this order was written ahead (the
  dispatch refresh below restores a stacked start);
- the assumptions name 048a-5ef and 048a-5gh in place of 048a-5e to 048a-5h, and add order 209's
  refusal of a login named for a plain sign-in hook (ruling 86, point 3 (c));
- `--login` stays and is sent as given: the engine refuses it before any part where the instance's
  sign-in hook is plain, as in every 0.2.0 Agent package, the default login's name included
  ([IN_FORCE.md](../../IN_FORCE.md), topic 9's line on ruling 86, point 1), and the help and
  `engine/README.md` say so; the command does not refuse it itself, since a keyed hook takes it;
- the line about a return the command does not carry is kept: no 0.2.0 package's output sets
  `acceptsReturn` true, since PND-013's first check needs a keyed hook, but the keyed fixture of
  the tests does, and so will the keyed release; the first test now pins it and the `Login:` line;
- 048b-join's tests are found by their names' beginnings; its refreshed script, with no return
  leg, has a `plain` flag and a settable account, so this order's script takes its answers with a
  queue of accounts in place of that account, and a third test drives the command through 048b-join's
  plain store, the 0.2.0 packages' shape: a stated login refused before any part, then the default
  login signed in by polling;
- every cited line, name and count is rechecked against `c097ec81`;
- the order runs one Codex round ([ruling 87](../proposals/ruling-87/README.md), below), and A77's
  amendment may pair it (ruling 89, point 6, has since paused the pairing: see below).

**Its dispatch refresh.** The coordinator refreshed it again on 2026-11-19, for its stacked
dispatch, against `main` at `92632057`, where order 209 has merged (#412, its `server/discover`
carrying `_meta`), the texts of 048a-5d, 048a-5d2 and 048a-5ef have merged refreshed (#408, #410,
#411) and ruling 88's and ruling 89's records have merged, and against 048b-join's text refreshed
beside it; 048a-5d has since merged (#414), and none of 048a-5d2 to 048b-join had merged as built.
Every change below is the coordinator's, open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)):

- under the owner's choice on 2026-11-19 of the lever "Pipeline the chain", as the coordinator
  applies it ([ruling 89](../proposals/ruling-89/README.md), point 2, open to the owner's strike),
  this order may start from 048b-join's round-1 branch, before 048b-join merges, with orders
  048a-5d, 048a-5d2, 048a-5ef and 048a-5gh each merged or on its own round-1 branch below it: the
  start check accepts either base, tells them apart by the predecessors' Records and says that the
  texts on `main` govern where the base holds older copies, and the admin paragraph at the foot
  says what the coordinator rebases and rereads when 048b-join merges (the readiness review's M8,
  in the coordinator's notes (private): this text withdrew the stacked start, and called itself
  off the critical path though the dogfood path ends at it); on 048b-join's branch alone, since
  decision 8's tests build on its helpers and script; the review's other options, a base of
  048a-5ef or 048a-5gh and a pair with 048b-join, are not taken (the second would need the
  owner's third pair);
- "Where it sits" now names this order the path's last link before the first dogfood milestone
  (ruling 89, point 3) and the client of the live check and of the Claude scratch-home sign-in
  (027's amendment, items 2 and 6), in place of "off the sign-in chain's critical path"; and
  ruling 89, point 6, pauses A77's pairing until 050n merges, so this order is not paired and runs
  ruling 87's one round;
- the readiness review's B2, the coordinator's choice: a stated limit, and no reconnect-and-retry.
  The review's premise is that `conduit sign-in` holds one MCP session and that the serving
  engine's idle limit, the call timeout plus 60 seconds (180 by default, DL10), which a running
  call does not reset, can end it while the person signs in. By the coordinator's reading of rmcp
  3.9.0 (`StreamableHttpServerConfig`'s `legacy_session_mode`: "sessions are removed from the
  `2026-07-28` version, so requests negotiating that version are always served statelessly"), a
  client on revision `2026-07-28`, as this command's is, holds no session for that limit to end;
  048b-join's refreshed first test sends a retry over a second connection, so a retry needs only
  the Interface; and on `2026-07-28` no session can expire, so a failed send means the engine or
  the loopback connection failed: a resend of a state the engine took would only be refused as
  already used (relay1's `requestState` is single-use), and its answer, the sign-in's next step,
  would stay lost, so the command stops and says where the sign-in may stand. 027's amendment,
  item 4, rests its reason for `--call-timeout-seconds 1200` on a session that `conduit sign-in`
  holds through the owner's login, which this reading finds absent: the coordinator rewords that
  reason before the sitting ([ruling 45](../proposals/ruling-45/README.md)), keeping the flag,
  once decision 11's finding confirms the reading, or keeps it where the finding says the client
  holds a session. The decisions are otherwise unchanged: decision 4 gains the text for a failed
  retry and a sentence on resending, and decision 11 two
  findings (this one and the source tree that `run` reads). So the command reconnects nothing and
  resends nothing: a retry whose send fails ends the command with a text that says where the
  sign-in stands (decision 4), and decision 11 asks the worker to confirm the reading from rmcp's
  source. What `--call-timeout-seconds 1200`, which 027's amendment, item 4, sets on the live
  check's `serve`, then covers is stated under "What the shell already does";
- under the owner's choice of the lever "Less prescriptive orders", as the coordinator applies it
  (ruling 89, point 2), the start check keeps only what the work needs: the Records now only tell
  the bases apart; the greps of the predecessors' decision names, of a spec sentence and of the
  fixtures' items are dropped, and 048b-join's three tests, run at the start, are the evidence
  for B4 to B9; where a function, a helper or a sentence that a decision calls or replaces is on
  the base under another name or in other words, the worker uses what it finds and reports it,
  and stops only where it is missing; line numbers are dropped for the text quoted;
- in "Done when", the greps of names that compilation and the tests' filtered runs already show
  are dropped, and the shell's suite is folded into the whole engine suite, which runs once, last,
  after the negative control;
- B6 and B7 read 048a-5ef's text as merged (#411) and 048a-5gh's names for DL18's end, and B9
  048b-join's refreshed text;
- "What the coordinator rechecks at dispatch" reads each predecessor's Record where it has merged,
  or else its round-1 build and the verifier's and reader's findings on it so far, and its items
  now settled (ruling 88's record, A77's pairing) are gone.

**Why it is needed now.** `server.rs`'s `client`, behind `conduit call`, uses rmcp's `call_tool`
with no elicitation handler, so it cannot answer an `input_required` result (order 048b-join's
findings). [Ruling 82](../proposals/ruling-82/README.md#the-decisions), point 1, has the owner
sign a scratch home in "through Conduit's sign-in flow", and point 3, as ruling 85, point 5,
narrows it, has a dogfood slot on the Mac sign in after this order. The coordinator's plan makes
this order's command the client of A5's live check
([experiment 027](../../experiments/027-a5-sign-in/README.md#stage-1-step-1-the-account-check)),
which runs after 048b-join and this order merge and before 0.2.0 (050n) merges, at the time the
owner picks, told first. This order runs no live check and signs nothing in.

**What it builds.** One behavior: a person signs an instance in from the command line. The
command sends `sign-in.v1` through root's Conduit to the instance's owner, declaring URL
elicitation; shows the engine's message and the full URL; asks the person's consent; opens the
browser only on consent; and carries each retry, accept, decline or cancel, until the engine gives
a final result. IFC-049: "For sign-in.v1 the request must declare {"elicitation": {"url": {}}} in
_meta.io.modelcontextprotocol/clientCapabilities"; "With that capability tools/call may return
InputRequiredResult with resultType input_required and exactly one inputRequests entry:
elicitation/create with mode url, message and url"; "The client echoes it exactly"; "For
sign-in, accept means consent, not completion: retries poll under PND-010, blocking or returning
another InputRequiredResult until the account appears or the wait ends." PND-010: "Decline or
cancel received on a retry ends failed." `sign-in.v1`'s description: "The optional request _meta
signInLogin asks for one stated login; an unknown name reports invalid before any part, with the
stated names." PND-010, as order 207 writes it: a plain hook "gets no Conduit keys (IFC-007), hence
no name, so a request that asks it for a login reports invalid before any part". No promise
changes: the engine does all of this once 048b-join merges; the command line only asks it, shows
what it answers and carries the person's answer back.

**Ruling 83, which this order follows.** The owner decided on 2026-11-18
([ruling 83](../proposals/ruling-83/README.md)) that the Agent packages 0.2.0 are
subscription-only, with no `apiKey` in `.v2`. Nothing in this order reads, sets or names `apiKey`,
and the command offers no key route.

**What the orders before it bring, which this order assumes.** Orders 051b, 048b-relay1,
048b-relay2, 048a-5, 048a-5b, 048a-5c, 048a-6 and 209 have merged to `main` with their Records,
though a stacked base may hold 209's round-1 build and file; the work of 048a-5d, 048a-5d2,
048a-5ef, 048a-5gh and 048b-join must be on the base this order starts from, each merged to `main`
or on its round-1 branch ("When this order starts"). Each assumption below is named, so that the
coordinator checks it against their Records, or their round-1 builds, before dispatch and amends
this order where one differs. Where you find at the start that one merged, or was built on your
base, otherwise, apply the nearest sound reading of each decision to what you find, and say so;
where an order's Record, or the build on your base, names a function, a member or a test
otherwise, use that name; but where the start check below prints otherwise and does not say that
a difference may be used and reported, stop. Below, `META/x` is the key `x` under the prefix that
`spec/names.yaml` sets as `metaPrefix` (in the shell's tests `shell.context.meta_prefix`), and
`C` is the request `_meta` key `io.modelcontextprotocol/clientCapabilities`.

- **B1, from 051b (merged).** In `engine/crates/shell/src/author.rs`: `fn route(directory: &Path,
  object: &Value, operation: &str) -> Result<String>`, `fn named(argument: &str) ->
  Result<Value>`, `pub fn ensure_success`, `pub fn note_unanswered(reply:
  &Value)` and `pub fn unanswered_hint() -> Option<String>`; `main.rs`'s
  `main` writes the hint on standard error after `run` returns, for every command but `pending`
  and `__run-guard`. `help.rs`'s `COMMANDS` and `DESCRIPTIONS` hold 14 entries,
  `pending` and `answer` right after `inspect`. `tests/cli.rs`'s list `verbs` holds `"pending"`
  and `"answer"` after `"inspect"`. In `engine/README.md`, 051b's paragraph that begins "`conduit
  pending --store DIR` lists" ends with the sentence "The sign-in command is not built yet (order
  051b-2)." 051b's Record leaves open that `route` picks root's Conduit by a suffix match on
  `#OPERATION`; this order calls it as it is.
- **B2, from 048a-5 (merged).** A `sign-in.v1` request whose `C` does not declare
  `elicitation.url` is refused before any part with the protocol error `-32021` and one IFC-049
  diagnostic (`request_member_refusal` in `engine/crates/core/src/sign_in.rs`, and the IFC-049
  arm of `requests.rs`'s rejection). The shell forwards `C` and `META/signInLogin` to the core
  (`sign_in_members` in `server.rs`); a `signInLogin` naming no stated login reports invalid with
  one diagnostic naming PND-010 and the stated names.
- **B3, from 048b-relay1 (merged).** An input-required output reaches the Interface as
  `CallToolResponse::InputRequired`, whose `inputRequests` holds exactly one entry, of key `url`,
  `{"method": "elicitation/create", "params": {"mode": "url", "message": M, "url": U,
  "elicitationId": ""}}`, with a `requestState` of 43 characters and the core's `_meta`. A retry
  carries both `requestState` and `inputResponses`; a refused retry is answered with
  `ErrorData::invalid_params` (`-32602`) and one of relay1's texts in `server/relay.rs`, the
  expired one being `requestState expired with its URL action's wait (DL18)` and the unknown one
  beginning `requestState is not one this engine issued`. The retry's `arguments` are not read.
- **B4, from order 209 (merged, #412).** In `request_member_refusal`, after the refusal of a
  name the Type does not state, a `signInLogin` naming a stated login reports invalid before any
  part where the instance's `sign-in-hook.v1` is a plain hook, with one diagnostic naming PND-010
  whose text holds `is a plain hook, its implementation item marked plain`. The item
  `signed-server` of the case `requests-sign-in-hold` is not marked `plain`, so its hooks are
  keyed and take a login's name; that of order 207's plain manifest, which 048a-5d's case
  `requests-sign-in-plain-hook-part` uses, is marked `plain: true` and lists nothing under
  `keyed`, so its hooks are called by the names its `tools` gives, `account_hook` and
  `sign_in_hook`, and take no login's name.
- **B5, from 048a-5d and 048a-5d2.** A first check that finds no account leads to a
  `sign-in-hook.v1` part, and its URL elicitation, checked under IFC-049's match, goes on to
  048a-5ef's relay. 048b-join's URL `U` for the login `browser` passes that match.
- **B6, from 048a-5ef**, as its text on `main` (#411) gives it. The
  engine writes `M`, naming the instance, its Type and package, its node, the URL's host
  (`example.org` in the tests), the expected account and what the person brings back, and puts
  `META/signInLogin` `{name, return, node, acceptsReturn}` in the output's `_meta` (decision
  `relay-sign-in-url`); `acceptsReturn` is true exactly where PND-013's first check passes, a
  redirect or page login stating `carried: true` served by a hook that is not plain
  (`accepts_return`), so `true` for the tests' `browser` and `false` for every 0.2.0 package. An
  accepted retry polls `account-hook.v1` (decision `poll-sign-in-account`). The expected account
  ends the invocation succeeded with output `account`, answering the sign-in action and lifting
  PND-005's hold. No account yet answers another input-required output with a fresh state, the
  same attempt and the first output's DL18 instant, dispatching no other part and **repeating the
  first output's URL elicitation, with the same `url`** (decision `ask-sign-in-again`). Decline or
  cancel ends it failed under PND-010 and dispatches no part (decision `end-sign-in-declined`).
  Where 048a-5ef's Record, or its build on your base, gives a further output another URL or
  dispatches `sign-in-hook.v1` again (048b-join's first test, step 4, would then fail at the
  start), stop and report it before you change anything: decision 8's first test rests on both.
- **B7, from 048a-5gh.** Another account found while polling is undone by a `sign-out-hook.v1`
  part and ends `sign-in.v1` failed naming both accounts. At DL18 the invocation ends unknown
  (the input `sign-in-wait-ended`, which `end_sign_in_wait_at` in `server.rs` submits at each
  relayed output's expiry, as 048a-5gh's text names them), and a later retry is refused as
  expired, or, where the shell dropped the entry or restarted, as not issued (B3).
- **B8, from 048a-6 (merged), and the deferral.** A first `sign-in.v1` request that carries
  `META/signInReturn` reports invalid under PND-013. Until 048a-7 and 048a-8, after dogfood, the
  core reads no return a retry carries. This order never sends that key (051b-3's).
- **B9, from 048b-join, as refreshed** (it joins the URL relay and polling only). In
  `engine/crates/shell/src/server_tests.rs`: `established_case(path, case, through)`, which reads
  a case's files from its `files` member, `fixed_case_shell(path, case, through)`,
  `url_client(url, token, prefix)`, the executor `SignInScript`, with its flag `plain`, its
  settable `account` and its answers, and the URL `U` for the login `browser` that its script
  answers and that passes IFC-049's match; three tests whose names begin
  `sign_in_relays_through_the_interface`, `sign_in_decline_through_the_interface` and
  `sign_in_plain_hook_through_the_interface`. The first two run on
  `conformance/conventions/requests/sign-in-hold` through step 9, which has the Agent `signed` set
  up with the account `primary-account`, its sign-in action and hold standing, and root reaching
  it through the tool `group.sign-in.v1`; the third on
  `conformance/conventions/requests/sign-in-plain-hook-part` through step 2, first start and
  registration, with the same Agent and tool under the plain item (B4). Root is issued a client
  token with `issue_client_token`. A `dispatch` intent names a keyed hook's call by the
  operation's local name, `account-hook.v1` or `sign-in-hook.v1`, and a plain one by its `tools`
  name, in `call.params.name`. As refreshed for its own stacked dispatch, the first test sends its
  last retry over a second connection, opened after the first is closed, with the same token: a
  retry needs only the Interface, not the connection that got the input request (the dispatch
  refresh's reading of the readiness review's B2, above).

**What the shell already does, so that you do not redo it.** Find each place by the text quoted
with it.

- `server.rs`'s `client` (`pub async fn client(`) finds root's ID among `state.json`'s
  objects (an Interface named `root` with no `spec`), reads its token from the store's
  `client-tokens` directory with `crate::store::private_read`, connects
  `StreamableHttpClientTransport` with `auth_header(token)` and `reinit_on_expired_session(false)`
  to `crate::Config::for_store(directory)?.engine_url`, trying the initialization up to three
  times, and serves `()`, which declares no elicitation. It is unchanged here.
- In `server_tests.rs`, `modern` (`async fn modern(`) connects with
  `serve_with_lifecycle(..., ClientLifecycleMode::Discover { preferred_versions:
  vec![ProtocolVersion::V_2026_07_28] })`; an `InputRequiredResult` reaches only a peer on that
  revision or later. On such a client, rmcp 3.9.0 writes the handler's own capabilities into `C`
  of every outbound request, and it implements `ClientHandler` for `ClientConfig`
  (`rmcp::model::ClientConfig`, an alias of `InitializeRequestParams`, which implements
  `Default`), whose `capabilities` has `elicitation: Option<ElicitationCapability>` and
  `extensions: Option<ExtensionCapabilities>` (a `BTreeMap<String, JsonObject>`).
  `ElicitationCapability::new().with_url(UrlElicitationCapability::new())` declares URL
  elicitation. `call_tool_once(CallToolRequestParams) -> Result<CallToolResponse, ServiceError>`
  sends one round without driving the next; `CallToolResponse` is `#[non_exhaustive]`, with the
  variants `Complete`, `InputRequired` and `Task`; `CallToolRequestParams` has `meta`, `name`,
  `arguments`, `input_responses` and `request_state`, serialized as `_meta`, `name`,
  `arguments`, `inputResponses` and `requestState`.
- The shell's HTTP layer refuses with `-32021` under IFC-036 a request that carries a key under
  the prefix without declaring the extension `META/implementation-interface` with
  `{"revision": "v0"}` in `C` (`server.rs`'s `authenticate`, from `let carries = meta`).
- The serving engine's MCP session idle limit is the call timeout plus 60 seconds, 180 by default,
  and a running call does not reset it (`engine/CONFORMANCE.md`, row "DL10: MCP session idle limit";
  `session_manager` in `server.rs`); `authenticate` binds a session to its Interface only where a
  request carries `mcp-session-id`. By the coordinator's reading of rmcp 3.9.0 (the readiness
  review's B2, in the dispatch refresh above), a request on revision `2026-07-28` is served without
  a session, so that limit does not reach this command's requests.
  `conduit serve --call-timeout-seconds 1200`, which 027's amendment, item 4, sets for the live
  check, then covers: each hook call's dispatch deadline (DL10, 120 seconds by default), raised to
  1200 seconds; and, should decision 11's finding show that this command's client holds an MCP
  session after all, an idle limit of 1260 seconds, longer than DL18's 600, so that the session
  cannot end while the person signs in. It does not lengthen DL18, after which a retry is refused as
  expired or not issued (B3, B7) whatever the timeout.
- `boundary::value(path)` reads one YAML document as JSON; `spec/names.yaml` holds `metaPrefix`.
- `main.rs`'s `run` builds a `tokio::runtime::Runtime::new()`, the multi-thread runtime, for the
  commands it hands to `author::run`. `server_tests.rs` is the module `tests::server_tests`
  (`#[path = "server_tests.rs"]` in `tests.rs`), and reaches `tests.rs`'s `repository()` through
  `use super::*`.

**When this order starts.** Orders 051b and 209 must have merged to `main`, though a stacked base
may hold 209's round-1 build and file. The work of orders 048a-5d, 048a-5d2, 048a-5ef, 048a-5gh and
048b-join must be on your base, the commit your working tree starts from (`HEAD`), in one of two
ways, either of which serves: each has merged to `main`; or the coordinator dispatched this order
stacked on 048b-join's round-1 branch, whose tip holds 048b-join's build, committed, before its read
and merge, on top of the builds of 048a-5d to 048a-5gh, each merged or on its own round-1 branch
below it. You tell them apart by the Records, which the admin writes at each merge: until then the
files of 209, 048a-5d2, 048a-5ef and 048a-5gh end "Not yet run.", and that of 048a-5d has no
`## Record` heading (048b-join's has either form). On either base, 048b-join's three tests are
present and pass (below). Report which of the five have merged on your base, and `HEAD`'s commit.
Nothing below reads which it is. On a round-1 branch, the copies of this order's file and of its
predecessors' files in your tree may be older than the texts that merged to `main`; where they
differ, the texts on `main` govern: read them with
`git show origin/main:docs/work-orders/FILE`, `FILE` being the file's name. Before you change
anything, check from the repository root that you find what they give, and stop, change nothing
and report if any check prints otherwise, except where its item below says that you use what you
find and report it:

```bash
for f in 209-build-keyed-hooks-and-sign-in-fixes.md 048a5d-build-sign-in-hook-part.md 048a5d2-build-sign-in-url-match.md 048a5ef-build-sign-in-relay-and-polling.md 048a5gh-build-sign-in-undo-and-expiry.md 048b-join-build-sign-in-through-the-interface.md; do echo "$f $(grep -c '^## Record' "docs/work-orders/$f") $(grep -c '^Not yet run\.$' "docs/work-orders/$f")"; done
grep -c '^## Record' docs/work-orders/051b-cli-pending-and-answer.md
grep -c 'fn sign_in_relays_through_the_interface' engine/crates/shell/src/server_tests.rs
grep -c 'fn sign_in_decline_through_the_interface' engine/crates/shell/src/server_tests.rs
grep -c 'fn sign_in_plain_hook_through_the_interface' engine/crates/shell/src/server_tests.rs
grep -c 'fn fixed_case_shell' engine/crates/shell/src/server_tests.rs
grep -c 'struct SignInScript' engine/crates/shell/src/server_tests.rs
grep -rl 'fn sign_in_command_' engine/crates/shell | wc -l | tr -d ' '
grep -c 'pub fn note_unanswered(reply: &Value)' engine/crates/shell/src/author.rs
grep -c '^fn route(directory: &Path, object: &Value, operation: &str) -> Result<String> {$' engine/crates/shell/src/author.rs
grep -c '^fn named(argument: &str) -> Result<Value> {$' engine/crates/shell/src/author.rs
grep -o 'const COMMANDS: \[(&str, &str, &str); [0-9]*\]' engine/crates/shell/src/help.rs
grep -c '"sign-in"' engine/crates/shell/src/main.rs
test -e engine/crates/shell/src/sign_in.rs; echo "file $?"
grep -c 'requestState expired with its URL action' engine/crates/shell/src/server/relay.rs
grep -c 'requestState is not one this engine issued' engine/crates/shell/src/server/relay.rs
grep -c 'is a plain hook, its implementation item marked plain' engine/crates/core/src/sign_in.rs
tr -s ' \n' '  ' < engine/README.md | grep -c 'The sign-in command is not built yet (order 051b-2).'
```

They print, in order:

1. six lines, each a file's name and two counts: `1 0` where the order has merged; on a round-1
   branch, `0 0` where the file has no `## Record` heading yet (048a-5d's, and 048b-join's unless
   its refresh added one) or `1 1` where it ends "Not yet run.". Order 209's line reads `1 0`
   where your base holds its Record, or `1 1` where your base carries 209's round-1 build; in
   that case, also check that
   `git show origin/main:docs/work-orders/209-build-keyed-hooks-and-sign-in-fixes.md | grep -c '^Not yet run\.$'`
   prints 0, and stop only where it does not. Report which have merged. Where a file is missing,
   or an order shows as merged while one before it in this list does not, stop;
2. `1`: 051b's Record (B1);
3. `1` five times: 048b-join's three tests, `fixed_case_shell` and `SignInScript` (B9). Where a
   helper prints `0` because 048b-join's Record, or its build on your base, names it otherwise,
   find it by what B9 says it is or does, use that name in decision 8 and report it; stop only
   where none is found, or where a test is missing;
4. `0`: no test of this order yet;
5. `1`, `1` and `1`: 051b's hint and the private `route` and `named` (B1). Where another order has
   since made `route` or `named` `pub(crate)` or given it another signature, use it as you find it
   and report it; stop only where it is missing;
6. `const COMMANDS: [(&str, &str, &str); N]`: 051b's help, `N` being 14 or 14 plus the commands an
   order merged beside added (report which); decision 6 adds one to what you find;
7. `0` and `file 1`: no command yet;
8. `1` and `1`: relay1's expired and not-issued texts, which decision 4 matches by their
   beginnings (B3);
9. a number other than `0`: order 209's refusal for a plain hook (B4), which the third test
   asserts;
10. `1`: 051b's sentence, which decision 9 replaces. Where it prints `0` because an order merged
    beside reworded it, replace the sentence that says the sign-in command is not built, as you
    find it, and report it before and after.

Then, from `engine/`, run 048b-join's three tests, one at a time:

```bash
cargo test --release -p conduit-shell sign_in_relays_through_the_interface
cargo test --release -p conduit-shell sign_in_decline_through_the_interface
cargo test --release -p conduit-shell sign_in_plain_hook_through_the_interface
```

Each must report, in the library's run, 1 passed and 0 failed, or stop and report it: this order
builds on that path, and these runs are the start's evidence for B4 to B9. On a round-1 branch a
failing test of a predecessor's is that order's to fix, not this order's. Then name in your report,
with their files as you find them: the helpers of B9 and their parameters, `SignInScript`'s answers,
`U`, and the decision names that 048a-5ef's and 048a-5gh's builds on your base give the relay, the
poll, the repeated output, the decline and DL18's end, as names only and not as a reason to stop.

Orders that land beside this one (051a, 051c, 050n and others) touch the command line or the
shell's tests too; whichever lands second keeps both changes. Where 051a or S0c has merged first
and moved the commands into a table, add `sign-in` to that table in the places this order names,
and say so.

**What the coordinator rechecks at dispatch.** For the coordinator, before this order dispatches,
against the merged texts of the predecessors and against the base it starts from: for each of
048a-5d, 048a-5d2, 048a-5ef, 048a-5gh and 048b-join, its Record where it has merged, or else its
round-1 build and the verifier's and reader's findings on it so far:

1. 048b-join: the three tests' names, the helpers' names and parameters, `SignInScript`'s answers,
   `plain` flag and `account`, `U`, and the two stores, through step 9 of `requests-sign-in-hold`
   and through step 2 of `requests-sign-in-plain-hook-part`; and that its first test still sends a
   retry over a second connection (the reading of the readiness review's B2);
2. 048a-5ef: that a further output repeats the first URL with the same attempt and instant (B6),
   that the engine's message names the URL's host, and `accepts_return` and its rule (`true` for
   the keyed `browser`, `false` for a plain item);
3. 048a-5gh: the end at DL18, and the texts a retry then gets;
4. 048a-5d and 048a-5d2: that `U` passes IFC-049's match for `browser`;
5. `main`'s shell at dispatch: help's count, `main.rs`'s lists, `author.rs`'s `route` and `named`
   (another order may have taken up 051b's open point on `route`), and the README sentence.

The order leaves to the coordinator, after its run and before A5's sitting, the rewording of the
reason that 027's amendment, item 4, gives for `--call-timeout-seconds 1200`, as the dispatch
refresh's B2 bullet says, on decision 11's B2 finding.

**Who decided what.** IFC-049, PND-010, PND-013 and DL18 decide what the request must declare,
what the engine relays, what a retry means and when the wait ends; ACC-033 and PND-010 decide
through which target `sign-in.v1` goes; PND-010 and IFC-007, as order 207 writes them, decide that
a login named for a plain hook is refused before any part. Spec batch 1's plan gives 051b "Client
URL elicitation: show the URL and the message, ask consent, open the browser." Ruling 86, point 1,
and ruling 85, point 5, decide that a dogfood slot on the Mac signs in through `conduit sign-in`
with its client's default login; the owner's first answer of 2026-11-19 moves 051b-3 after
dogfood; ruling 89, point 3, places the first dogfood milestone after 050n and this order, and
its point 2 gives the levers that the dispatch refresh applies. Everything else here is the
coordinator's, each open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):
the split of 051b and this part's place after 048b-join and 051b; the refreshes' changes listed
above, the stacked dispatch on 048b-join's round-1 branch and the choice for the readiness review's
B2 among them; that this command is the client of A5's live check; the command's name, its
argument `KIND/NAME` and its option `--login NAME`, sent as given and refused by the engine, not
by the command, for a plain hook; routing through 051b's
`route`; a client of its own beside `server::client`, connected once as root on revision
`2026-07-28`; sending no `expectedId`; showing the message and the URL on standard error with
control characters escaped, and the result on standard output as one JSON line; consent read as
`y` or `yes`, decline as `n` or `no`, the end of input as cancel, and any other line asked again;
opening the browser only on consent, by the platform's own opener with the URL as one argument
and no shell; waiting for the person's Enter before an accepted retry, rather than retrying at
once or on a timer; not opening a URL already opened; the retry carrying no `_meta` of its own;
neither reconnecting nor sending a retry again after a failed send;
the texts; refusing to open anything but one https URL elicitation; the tests, their script and
every value in them; and the paragraph of `engine/README.md`. Apply them as written and report
them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: IFC-049, IFC-036 and IFC-007 (`spec/interface/implementation.yaml`); PND-010 and
   PND-013 (`spec/promises/pending.yaml`); DL18 (`spec/implementation-defined.yaml`);
   `sign-in.v1` in `spec/core/operations.yaml`.
2. Orders 051b and 048b-join, whole, with their Records where they have merged; the Records of
   048a-5ef and 048a-5gh, or on a round-1 base their texts on `main`; order 209's decision 4 and
   its Record; in order 048b-relay1, decision 4's refusal texts.
3. `engine/crates/shell/src/author.rs` and `main.rs` whole; `server.rs` from `pub async fn
   client`, and `session_manager`; `help.rs`; in `server_tests.rs`, `modern`, 048b-join's three
   tests and their helpers.
4. rmcp 3.9.0 in cargo's registry, read only: `src/model/mrtr.rs` (`CallToolResponse`,
   `InputRequiredResult`), `src/model/capabilities.rs` (`ClientCapabilities`,
   `ElicitationCapability`), and in `src/model.rs` `CallToolRequestParams` and `ClientConfig`;
   for decision 11's finding on the readiness review's B2,
   `src/transport/streamable_http_server/tower.rs` (`legacy_session_mode`) and `src/transport/streamable_http_client.rs` (`saved_init_request`,
   `reinit_on_expired_session`).

**Size: M**: about 0.45 to 0.55k changed lines of Rust, about half of them tests, in one new file
and six changed ones, and one sentence of `engine/README.md`.

## The decisions it applies

Each decision below is the coordinator's, as "Who decided what" says.

1. **The command.** `conduit sign-in --store DIR KIND/NAME [--login NAME]`. In `main.rs`: add
   `sign-in|` after `answer|` in the usage string; add `"sign-in"` to the list of commands that
   collect positional arguments and to the list of known commands; add a variable
   `let mut login = None;` beside `as_instance`, and the arm
   `"--login" if command == "sign-in" => login = Some(args.next().ok_or("--login needs NAME")?),`
   right after the `"--as"` arm. Right after the block that hands commands to `author::run`, add:

   ```rust
   if command == "sign-in" {
       let repository = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../..");
       let runtime = tokio::runtime::Runtime::new().map_err(|e| e.to_string())?;
       let mut person = conduit_shell::sign_in::Terminal;
       return runtime.block_on(conduit_shell::sign_in::run(
           &store,
           &positional,
           login.as_deref(),
           &repository,
           &mut person,
       ));
   }
   ```

   The multi-thread runtime keeps the client's tasks running while the terminal is read. In
   `engine/crates/shell/src/lib.rs`, add `pub mod sign_in;` right after `pub mod server;`. In
   `author.rs`, make `route` and `named` `pub(crate)`, and change nothing else there. The command
   sends `--login` as given: it does not ask whether the instance's hook is plain, and the engine
   refuses a named login aimed at a plain hook before any part (B4).

2. **The new file `engine/crates/shell/src/sign_in.rs`**, beginning with the line
   `//! conduit sign-in: one sign-in.v1 through serve, its URL action shown, consented to and retried (order 051b-2).`
   It holds:
   - `pub trait Person { fn ask(&mut self, prompt: &str) -> Result<Option<String>>; fn tell(&mut self, line: &str); fn open(&mut self, url: &str) -> Result<()>; }`:
     `ask` shows `prompt` and reads one line, `None` at the end of input; `tell` shows one line;
     `open` opens `url` in the person's browser.
   - `pub struct Terminal;` implementing `Person`: `tell` is `eprintln!("{line}")`; `ask` writes
     `prompt` with `eprint!`, flushes standard error, reads one line from standard input with
     `read_line`, gives `None` where it read 0 bytes, and otherwise the line without its trailing
     `\n` or `\r\n`; a read error gives `Err(format!("could not read the terminal: {error}"))`.
     `open` runs `browser_command(url)` with standard input, output and error set to
     `Stdio::null()` and waits for its `status()`; a spawn error gives
     `Err(error.to_string())`, and an unsuccessful status `Err(format!("{program} exited with {status}"))`,
     `program` the command's program as `get_program()` gives it, lossily.
   - `fn browser_command(url: &str) -> std::process::Command`: on `target_os = "macos"`,
     `open` with the one argument `url`; on `windows`, `rundll32` with the two arguments
     `url.dll,FileProtocolHandler` and `url`; on any other target, `xdg-open` with the one
     argument `url`. Each by `#[cfg(...)]`; no shell, and the URL is always one argument.
   - `pub(crate) fn shown(text: &str) -> String`: each character for which `char::is_control` is
     true written as `c.escape_unicode()`, every other character as it is, so that nothing the
     engine sends can move the terminal's cursor or change its colours.
   - `pub(crate) struct UrlAction { pub key: String, pub message: String, pub url: String, pub state: String }`
     and `pub(crate) fn url_action(result: &Value) -> Result<UrlAction>`, `result` an
     `InputRequiredResult` serialized with `serde_json::to_value`. It gives the action where
     `inputRequests` is an object of exactly one entry, of key `key`, whose `method` is
     `elicitation/create` and whose `params.mode` is `url`, `params.message` a string and
     `params.url` a string that `url::Url::parse` reads with scheme `https`, and where
     `requestState` is a nonempty string. Other members of `params`, such as rmcp's empty
     `elicitationId` (B3), are ignored. Otherwise it gives the error
     `the engine's input request is not one URL elicitation with an https URL and a requestState; nothing was opened and nothing more was sent, so this sign-in waits until its URL action's wait ends (DL18)`.
   - `#[derive(Debug, PartialEq, Eq)] pub(crate) enum Consent { Open, Decline, Cancel, Again }`,
     `pub(crate) fn consent(line: Option<&str>) -> Consent` and
     `pub(crate) fn waited(line: Option<&str>) -> &'static str`. Each reads the line trimmed and
     compared ignoring ASCII case. `consent`: `y` or `yes` is `Open`; `n` or `no` is `Decline`;
     `None` is `Cancel`; any other line, the empty one included, is `Again`. `waited`: `None`, `n`
     or `no` is `"cancel"`; any other line, the empty one included, is `"accept"`.
   - `pub(crate) fn subject(args: &[String]) -> Result<Value>`: where `args` holds other than
     one item, the error
     `sign-in: expected 1 positional argument(s), received N; run conduit sign-in --help for usage and an example`;
     otherwise `crate::author::named(&args[0])?`, and where its `kind` is neither `Agent` nor
     `Resource`, the error
     `sign-in: KIND/NAME is a KIND, and sign-in.v1 signs in only an Agent or a Resource; nothing was sent`,
     `KIND/NAME` being the argument and `KIND` its kind.
   - `connect`, `rounds`, `sign_in` and `run`, decisions 3 to 5.

3. **The client, `async fn connect(directory: &Path, prefix: &str) -> Result<RunningService<RoleClient, ClientConfig>>`.**
   It finds root's ID and token as `server::client` does, with the same error texts, and the
   engine URL from `crate::Config::for_store(directory)?`. It builds the handler with exactly
   these lines, so that the negative control of "Done when" finds the second:

   ```rust
   let url_elicitation = ElicitationCapability::new().with_url(UrlElicitationCapability::new());
   handler.capabilities.elicitation = Some(url_elicitation);
   ```

   `handler` being `let mut handler = ClientConfig::default();`, and its
   `capabilities.extensions` the one entry `format!("{prefix}implementation-interface")` with the
   object `{"revision": "v0"}`. It connects one `StreamableHttpClientTransport`, built as
   `server::client` builds it, through `handler.serve_with_lifecycle(transport,
   ClientLifecycleMode::Discover { preferred_versions: vec![ProtocolVersion::V_2026_07_28] })`
   under a timeout of 10 seconds, once, with no retry. A failure or the timeout gives
   `conduit sign-in could not open a session with the engine at URL: E; run conduit serve --store DIR and check its diagnostics`,
   `URL` the engine URL, `E` the error or `timed out`, and `DIR` the store's path. The tests do
   not set `C` by hand: rmcp writes it from the handler.

4. **The rounds, `async fn rounds(service: &RunningService<RoleClient, ClientConfig>, tool: &str, arguments: Value, meta: Option<Value>, label: &str, person: &mut dyn Person) -> Result<Value>`.**
   `label` is `KIND/NAME`. Keep `opened: Option<String>`, none at first. The first request is
   `{"name": tool, "arguments": arguments}`, with `"_meta": meta` only where `meta` is `Some`,
   read into `CallToolRequestParams` with `serde_json::from_value`. Then loop, sending each
   request with `service.call_tool_once`:
   - **`Err(ServiceError::McpError(e))`**: print `{"error": E}` as one JSON line on standard
     output, `E` being `e` as `serde_json::to_value` gives it, and give an error: where
     `e.message` begins `requestState expired` or `requestState is not one this engine issued`
     (relay1's two texts, B3; after DL18 a retry may get either, since relay1 drops an expired
     entry at any other output's issue or take),
     `the engine no longer holds this sign-in's state: its URL action's wait ended (DL18) or the engine restarted, so nothing more was sent; run conduit sign-in again`;
     otherwise
     `sign-in.v1 through TOOL was answered with error CODE: MESSAGE; nothing more was sent`.
     Any other `Err` of the first request gives
     `sign-in.v1 through TOOL failed: E; nothing more was sent`; any other `Err` of a retry (a
     request carrying `requestState`) gives
     `sign-in.v1 through TOOL failed on a retry: E; the retry was not sent again, since the engine may have taken it, so this sign-in may still wait in the engine until its URL action's wait ends (DL18): run conduit pending to see its sign-in action, and conduit sign-in again once that wait has ended`
     (the coordinator's choice for the readiness review's B2: nothing reconnects and nothing is
     resent). Write that literal on one source line too, with no `\` continuation.
   - **`Ok(CallToolResponse::Complete(r))`**: give `serde_json::to_value(r)`, an error mapped to
     its text.
   - **`Ok(CallToolResponse::Task(_))`** or any other variant: give the error
     `sign-in.v1 through TOOL was answered with a task, which conduit sign-in does not read; nothing more was sent`.
   - **`Ok(CallToolResponse::InputRequired(i))`**: serialize `i` with `serde_json::to_value`,
     an error mapped to its text, and read it with `url_action`; an error of either ends the
     loop, sending nothing. Where `opened` equals `action.url`, `tell`
     `conduit sign-in: the engine has not yet found the expected account signed in on LABEL; the URL below is the one already opened.`;
     otherwise `tell` `conduit sign-in: the engine asks you to open a URL to sign in LABEL.`.
     Then `tell`, in this order, `Message: ` followed by `shown(&action.message)` and `URL: `
     followed by `shown(&action.url)`. Where the result's `_meta` has a key ending in
     `/signInLogin` (found as `author.rs`'s `outcome` finds its key) whose `name` is a string,
     `tell` `Login: ` followed by `shown` of it; where that key's `acceptsReturn` is `true`, `tell`
     `conduit sign-in: the engine reports that this login takes a return carried back through the Interface (acceptsReturn), but this command carries none (order 051b-3, with the keyed release): where the browser is not on the machine where LABEL runs, the login cannot complete through this command.`
     Write that literal on one source line, with no `\` continuation.
     No 0.2.0 package's output sets `acceptsReturn` true (B6), so that line is not shown for them.
     Then decide the answer `A`:
     - where `opened` equals `action.url`, `A` is `waited` of
       `person.ask("Press Enter once you have signed in, or type n to stop: ")?`;
     - otherwise ask `"Open this URL in your browser? Type y to open it, or n to decline: "` until
       `consent` gives other than `Again`. `Decline` makes `A` `"decline"`, `Cancel` makes it
       `"cancel"`, and `Open` calls `person.open(&action.url)`: where it fails, `tell`
       `conduit sign-in: the browser could not be opened (E); open the URL above yourself.`; in
       both cases set `opened` to the URL, then `A` is `waited` of the Enter prompt above.

     The next request is `{"name": tool, "arguments": arguments, "requestState": action.state,
     "inputResponses": {action.key: {"action": A}}}`, with no `_meta` of its own, and the loop
     goes on.

   `LABEL`, `TOOL`, `CODE`, `MESSAGE` and `E` are replaced by their values. No round is limited
   in number: each accepted retry waits for the person. No request is sent twice, and the command
   opens no second client (the readiness review's B2). Nothing here prints the message or the URL
   on standard output, and nothing writes them to the store.

5. **`pub async fn sign_in(directory: &Path, object: &Value, login: Option<&str>, prefix: &str, person: &mut dyn Person) -> Result<Value>`
   and `pub async fn run(directory: &Path, args: &[String], login: Option<&str>, repository: &Path, person: &mut dyn Person) -> Result<()>`.**
   `sign_in`: `let tool = crate::author::route(directory, object, "sign-in.v1")?;`, written as
   exactly that one line; the meta `{format!("{prefix}signInLogin"): {"name": login}}` where
   `login` is `Some`, and none otherwise; `connect`; `rounds` with the arguments
   `{"instance": {"kind": K, "name": N}}` of `object`, no `expectedId`, and the label `K/N`; then
   `service.cancel()`, keeping the rounds' error where both fail, as `server::client` does; and
   it gives the final result. `run`: `let object = subject(args)?;`, the prefix as the string
   `metaPrefix` of `crate::boundary::value(&repository.join("spec/names.yaml"))?`, then
   `sign_in`, where a missing or non-string `metaPrefix` gives the error `missing metadata prefix`;
   it prints the result as one JSON line on standard output, calls
   `crate::author::note_unanswered` on it, and gives
   `crate::author::ensure_success(&result, &format!("sign-in.v1 of {label} through serve"))`,
   where `run` builds `label` as `KIND/NAME` from `subject`'s object, as `sign_in` builds its own.
   So the command exits 0 only where the sign-in succeeded, a refusal of `--login` included, and
   051b's hint follows its last result.

6. **Help.** In `help.rs`, `COMMANDS` and `DESCRIPTIONS` become one entry more each than the start
   check found (15 where it found 14), the new one right after `answer`'s:
   `("sign-in", "KIND/NAME [--login NAME]", "sign-in --store /tmp/conduit Agent/a")`,
   described
   `Sign in an Agent or Resource through serve and root's Conduit to its owner: shows the engine's message and the full URL, opens the browser only after you type y, and retries each time you press Enter until the engine answers. --login asks for one of the Type's stated logins; the engine refuses it where the sign-in hook is plain, as in the 0.2.0 Agent packages. Carries no return from another machine.`
   In `tests/cli.rs`, add `"sign-in"` to the list `verbs` of
   `help_describes_every_command_without_store_access`, right after `"answer"`, and change
   nothing else in that file.

7. **The unit tests**, in `sign_in.rs`'s `mod tests`:
   - `url_action_takes_one_https_url_elicitation`: for `V` =
     `{"resultType": "input_required", "inputRequests": {"url": {"method": "elicitation/create", "params": {"mode": "url", "message": "Sign in", "url": "https://example.org/login", "elicitationId": ""}}}, "requestState": "s"}`,
     `url_action` gives the key `url`, the message `Sign in`, the URL
     `https://example.org/login` and the state `s`. It gives an error for `V` with a second entry
     `other` equal to the first; with `mode` `form`; with `url` `http://example.org/login`; with
     `method` `sampling/createMessage`; and without `requestState`.
   - `answers_are_read_strictly`: `consent` gives `Open` for `Some("y")` and `Some(" YES ")`,
     `Decline` for `Some("n")` and `Some("No")`, `Cancel` for `None`, and `Again` for `Some("")`
     and `Some("maybe")`; `waited` gives `"accept"` for `Some("")` and `Some("done")`, and
     `"cancel"` for `None`, `Some("n")` and `Some("NO")`.
   - `shown_escapes_control_characters`: `shown("a\u{1b}[31mb\nc")` equals
     `"a\\u{1b}[31mb\\u{a}c"`, and `shown("https://example.org/a?b=c")` is unchanged.
   - `subject_takes_one_agent_or_resource`: `subject` of `["Agent/a"]` gives
     `{"kind": "Agent", "name": "a"}`; of `["Fleet/f"]` an error containing
     `signs in only an Agent or a Resource`; of `[]` an error containing
     `expected 1 positional argument(s), received 0`.
   - `browser_command_passes_the_url_as_one_argument`: for
     `https://example.org/a?b=c&d=e`, `browser_command`'s `get_program()` and `get_args()` are,
     on macOS, `open` and the URL alone; on Windows, `rundll32`,
     `url.dll,FileProtocolHandler` and the URL; elsewhere, `xdg-open` and the URL alone, each by
     `#[cfg(...)]`. The test never spawns it.

8. **The tests through the engine's MCP server**, in `server_tests.rs`, each
   `#[tokio::test(flavor = "multi_thread", worker_threads = 4)]`, on a store of its own opened as
   048b-join's tests open theirs, with `fixed_case_shell(temp, CASE, N)`, CASE and N given below for each test,
   root issued a client token with `issue_client_token`, and the executor below wrapped in
   `ServingExecutor::new` and served with `server::serve`. `prefix` is
   `shell.context.meta_prefix`.

   *The person.* Add `struct Scripted { answers: VecDeque<Option<String>>, prompts: Vec<String>, told: Vec<String>, opened: Vec<String> }`
   implementing `crate::sign_in::Person`: `ask` pushes the prompt and gives the front answer,
   popped, or `None` once none is left; `tell` pushes the line; `open` pushes the URL and gives
   `Ok(())`. It opens nothing.

   *The script.* Add an executor `PollScript`, in place of a runner, shared with the test through
   an `Arc`, built as 048b-join's `SignInScript` is, with its flag `plain` set when it is made, its
   list of every `dispatch` intent it receives and its `cancel`, and one difference: in place of the
   settable `account`, a queue of account values, since the command drives every retry itself.
   Its `dispatch` answers as `SignInScript`'s does, by the intent's `call.params.name` (B9), each as
   `Execution::Complete` of `{"kind": "result", "result": R}`:
   - `account-hook.v1`, or `account_hook` where `plain`:
     `{"content": [], "structuredContent": {"account": A}}`, `A` the queue's front value, popped,
     or `null` where the queue is empty, with `"_meta": {"META/outcome": "succeeded"}` added where
     not `plain`;
   - `sign-in-hook.v1`, or `sign_in_hook` where `plain`:
     `{"resultType": "input_required", "inputRequests": {"login": {"method": "elicitation/create", "params": {"mode": "url", "message": "Open the page to sign in.", "url": U}}}, "requestState": "hook-state-1"}`,
     `U` being 048b-join's URL for the login `browser`;
   - any other: `{"content": []}`, with `"_meta": {"META/outcome": "succeeded"}` added where not
     `plain`.

   Where 048b-join's `SignInScript` writes these answers in another form, use its form and report
   it. A popped account stands for what the person did in the browser before pressing Enter.

   The first two tests run on 048b-join's keyed store, from `requests-sign-in-hold` through step
   9, with a script not `plain`; the third on its plain store, from
   `requests-sign-in-plain-hook-part` through step 2, with a script `plain`.

   *The first test*, `sign_in_command_opens_the_url_and_polls_to_signed_in`, the queue
   `[null, null, "primary-account"]` and the answers `[Some("y"), Some(""), Some("")]`:
   1. `crate::author::route(temp, &json!({"kind": "Agent", "name": "signed"}), "sign-in.v1")`
      gives `group.sign-in.v1`.
   2. `crate::sign_in::sign_in(temp, &json!({"kind": "Agent", "name": "signed"}), None, &prefix, &mut person).await.unwrap()`
      gives `F`, so that a failure's panic message holds `rounds`' text
      `was answered with error -32021: ...`: `F`'s `isError` is not `true`, its
      `structuredContent.account` is `primary-account`, and its `_meta` `META/outcome` is `succeeded`.
   3. `person.opened` is exactly `[U]`: the URL was opened once, after consent.
   4. `person.prompts` is exactly the consent prompt, then the Enter prompt twice; `person.told`
      holds `URL: U` twice, `conduit sign-in: the engine asks you to open a URL to sign in Agent/signed.`
      once, the not-yet line once, two lines beginning `Message: ` that contain `example.org`,
      `Login: browser` twice, and two lines beginning
      `conduit sign-in: the engine reports that this login takes a return` and containing
      `this command carries none (order 051b-3, with the keyed release)` (B6: the keyed `browser` takes a
      return).
   5. The script's intents, by `call.params.name`, are `account-hook.v1`, `sign-in-hook.v1`,
      `account-hook.v1` and `account-hook.v1`, and no other.
   6. `sign_in_holds` has no entry for `signed`'s ID.

   *The second test*, `sign_in_command_declines_cancels_and_asks_for_a_login`, the queue empty
   (every account check finds none), three sign-ins on the one store, each with a fresh
   `Scripted`:
   1. No login, the answers `[Some("maybe"), Some("n")]`: `Ok(F1)` with `F1`'s `isError` `true`
      and `META/outcome` `failed` (PND-010); `prompts` is the consent prompt twice; `opened` is
      empty.
   2. The login `Some("phone")`, no answers: `Ok(F2)` with `F2`'s `isError` `true`,
      `META/reason` `invalid` and `structuredContent.diagnostics` holding one diagnostic whose
      `ruleId` is `PND-010` and whose text contains `"browser"`; `prompts`, `told` and `opened`
      are empty. The name reached the engine, which refused it before any part.
   3. No login, no answers, through the command's own function:
      `crate::sign_in::run(temp, &["Agent/signed".into()], None, &repository(), &mut person)`.
      The end of input at the consent prompt cancels, so the sign-in ends failed (PND-010) and
      `run` gives an `Err` containing `did not succeed`; `prompts` is the consent prompt once;
      `opened` is empty. This step checks that `run` reads the prefix, prints and fails where the
      sign-in did not succeed.
   4. The script holds four intents: `account-hook.v1` and `sign-in-hook.v1` for each of the
      first and third sign-ins, and none for the second. `sign_in_holds` still has an entry for
      `signed`'s ID.

   *The third test*, `sign_in_command_signs_in_through_a_plain_hook`, the 0.2.0 packages' shape
   (ruling 86, point 1), two sign-ins on the one store, each with a fresh `Scripted`:
   1. The login `Some("browser")`, the Type's default login, no answers: `Ok(F1)` with `F1`'s
      `isError` `true`, `META/reason` `invalid` and `structuredContent.diagnostics` holding one
      diagnostic whose `ruleId` is `PND-010` and whose text contains `is a plain hook`; `prompts`,
      `told` and `opened` are empty, and the script holds no intent: the engine refused the named
      login before any part (B4).
   2. No login, the queue `[null, "primary-account"]` and the answers `[Some("y"), Some("")]`:
      `Ok(F2)` with `F2`'s `isError` not `true`, its `structuredContent.account`
      `primary-account` and its `_meta` `META/outcome` `succeeded`; `opened` is exactly `[U]`;
      `prompts` is the consent prompt, then the Enter prompt once; `told` holds `Login: browser`
      once and no line beginning `conduit sign-in: the engine reports that this login takes a return`
      (`acceptsReturn` false, B6).
   3. The script holds three intents, by `call.params.name`: `account_hook`, `sign_in_hook` and
      `account_hook`, and no other.

   Before running them, check that no other test's name in `engine/crates/shell/` begins
   `sign_in_command_`. If a test fails because a reply differs from what it states, stop and
   report the step and the reply: do not change the expectations to pass it. No test
   names `Terminal`.

9. **`engine/README.md`.** In 051b's paragraph, replace the sentence "The sign-in command is not
   built yet (order 051b-2)." with this text, and change nothing else in the file:

   ```text
   `conduit sign-in --store DIR KIND/NAME [--login NAME]` signs an Agent or a Resource in
   through `serve` and root's Conduit to its owner, declaring URL elicitation and, with
   `--login`, asking for one of its Type's stated logins, which the engine refuses before any
   part where the instance's sign-in hook is plain, as in the 0.2.0 Agent packages; without it,
   the hook starts its default login. It shows the engine's message and the full URL on
   standard error, opens the browser only after you type `y`, and retries each time you press
   Enter, until the engine answers: signed in, failed where you type `n` or input ends, or
   ended once the URL action's wait ends (DL18). It prints the result as one JSON line and
   exits 0 only where the sign-in succeeded. It carries no return from another machine (order
   051b-3, with the keyed release). Interrupting it leaves the sign-in waiting until that wait
   ends.
   ```

10. **Promise IDs in the shell's sources.** The new text names no promise ID other than
    `PND-010` (the second test), which 048b-join's tests already name in `server_tests.rs`, and
    `IFC-049` nowhere in code. Where `python3 tools/promise_standing.py --check` still fails, run
    it with `--write`, as AGENTS.md asks, and report the rows it changes.

11. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line, ruling's
    record, case, fixture, core source, `server.rs`, `server/relay.rs` or `runner.rs` changes.
    No return is carried, no paste is read and no listener is opened (051b-3, after dogfood);
    `conduit call` is unchanged; no `expectedId` is sent; no client token for another Interface
    (DL13); the command does not judge `--login` itself. Report as findings, for the
    coordinator:
    - whether 048a-5ef's further output repeated the first URL (B6) in your run;
    - that an interrupted command leaves the invocation waiting until DL18, with the sign-in
      conflict of PND-010 refusing a new `sign-in.v1` of that instance until then;
    - that `connect` repeats `server::client`'s reading of root's token, tries once where
      `server::client` tries three times, and, like `server::client`, reads the engine URL from
      the store and not from `--url`;
    - that no test runs the built command, and `Terminal::open` has no switch that keeps the
      browser shut;
    - that the third test's plain store holds no sign-in action or hold, so the plain path's
      answer of an action and lifting of a hold are shown only on the keyed store;
    - that the engine's message, shown on standard error, names the expected account (B6), which
      A5's live check, filtering standard output only, does not hold back;
    - that `route` picks root's Conduit by a suffix match (B1, 051b's open point);
    - for the readiness review's B2: from rmcp 3.9.0's source, read only, whether a request of
      this command's client, on revision `2026-07-28` after a `Discover` start, is served without an
      MCP session (`legacy_session_mode`'s comment in the server, and whether the client keeps a
      `saved_init_request` or an `mcp-session-id` after such a start), so that DL10's session idle
      limit does not reach it, with the lines you read; where you find that it holds a session, say
      so first among the findings, since the live check then rests on its
      `--call-timeout-seconds 1200` alone; and that the text of a retry whose send failed
      (decision 4) is reached by no test;
    - that `run` reads the `_meta` prefix from `spec/names.yaml` under the source tree at run time
      (the repository that `CARGO_MANIFEST_DIR` names at build time), so the built command needs
      that tree in place;
    - and any place where IFC-049, PND-010 or the batch plan left you a choice this order does not
      settle.

    The three texts that say this command carries no return (decision 4's `acceptsReturn` line,
    decision 6's help sentence "Carries no return from another machine." and decision 9's README
    sentence "It carries no return from another machine (order 051b-3, with the keyed
    release).") hold only until 051b-3 merges; 051b-3 replaces them.

## Scope

Change only the new file `engine/crates/shell/src/sign_in.rs` (decisions 2 to 5 and 7),
`engine/crates/shell/src/lib.rs` (one line, decision 1), `engine/crates/shell/src/main.rs`
(decision 1), `engine/crates/shell/src/author.rs` (`route` and `named` made `pub(crate)`,
decision 1), `engine/crates/shell/src/help.rs` (decision 6),
`engine/crates/shell/tests/cli.rs` (one name, decision 6),
`engine/crates/shell/src/server_tests.rs` (decision 8) and `engine/README.md` (decision 9).
`engine/STANDING.md` changes only as decision 10 says, written only by the tool. Not `spec/`,
`IN_FORCE.md`, `tools/`, `packages/`, `conformance/`, `engine/crates/core/`,
`engine/crates/loader/`, `engine/crates/conformance/`, `server.rs`, `server/relay.rs`,
`runner.rs`, `engine/CONFORMANCE.md`, the shell's `Cargo.toml`, the experiments,
`docs/implementation/` or `docs/proposals/`: report any defect as a finding.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-shell --lib sign_in::tests
cargo test --release -p conduit-shell sign_in_command_
cargo test --release -p conduit-shell sign_in_relays_through_the_interface
cargo test --release -p conduit-shell sign_in_decline_through_the_interface
cargo test --release -p conduit-shell sign_in_plain_hook_through_the_interface
cargo test --release -p conduit-shell --test cli help_describes_every_command
```

The third reports 5 passed and 0 failed: decision 7's five. In the fourth, the library's target
reports 3 passed and 0 failed and every other target 0 passed. Both reported 0 passed on the tree
you started from. In the fifth, the sixth and the seventh, the library's target reports 1 passed
each, and the eighth reports 1 passed. If any other test fails, stop and report it: do not change
it.

Next, show that the first test fails where the command does not declare URL elicitation. From
`engine/`, one at a time:

```bash
cp crates/shell/src/sign_in.rs target/sign_in.rs.declared
sed 's/handler.capabilities.elicitation = Some(url_elicitation);/handler.capabilities.elicitation = None;/' target/sign_in.rs.declared > crates/shell/src/sign_in.rs
grep -c 'handler.capabilities.elicitation = None;' crates/shell/src/sign_in.rs
cargo test --release -p conduit-shell sign_in_command_opens_the_url
cp target/sign_in.rs.declared crates/shell/src/sign_in.rs
cargo test --release -p conduit-shell sign_in_command_opens_the_url
```

The `grep` prints 1. The first test command must fail at step 2, its error naming `-32021`: the
engine refuses a `sign-in.v1` that does not declare URL elicitation, before any part (IFC-049,
assumption B2); a failure for any other reason does not count. Report its message. After the
restore the second reports 1 passed. A warning that `url_elicitation` is unused in the first build
is expected. Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo
would not rebuild the crate. Git ignores `target/`.

Then, last, from `engine/`, the whole engine suite, once:

```bash
RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release
```

It passes; report its count of passed tests for each target, and the shell library's count against
that of the tree you started from, if you ran it there (three more for decision 8 and five for
decision 7). The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

From the repository root, each passes. `/usr/bin/python3` is the Mac's interpreter with PyYAML;
where `python3` has PyYAML, it serves as well.

```bash
python3 tools/check_docs.py
python3 tools/check_rulings.py
python3 tools/set_names.py --check
python3 tools/check_private_paths.py
python3 tools/promise_standing.py --check
git status --short -- spec IN_FORCE.md tools packages conformance engine/crates/core engine/crates/loader engine/crates/conformance engine/crates/shell/src/server.rs engine/crates/shell/src/server/relay.rs engine/crates/shell/src/runner.rs engine/crates/shell/Cargo.toml engine/CONFORMANCE.md experiments docs/implementation docs/proposals
git diff --check
```

`promise_standing.py --check` ends `Errors: 0`. The `git status` command prints nothing.

Each of these prints 1, and each printed 0 at the start. The `tr` checks read the file with its
line breaks as spaces, so a rewrap hides nothing; they hold the texts this order prescribes, and
the line the negative control changes:

```bash
grep -c 'handler.capabilities.elicitation = Some(url_elicitation);' engine/crates/shell/src/sign_in.rs
grep -c 'this command carries none (order 051b-3, with the keyed release): where the browser is not on the machine' engine/crates/shell/src/sign_in.rs
grep -c '; the retry was not sent again, since the engine may have taken it, so this sign-in may still wait' engine/crates/shell/src/sign_in.rs
grep -c '"--login" if command == "sign-in"' engine/crates/shell/src/main.rs
grep -c 'the engine refuses it where the sign-in hook is plain, as in the 0.2.0 Agent packages' engine/crates/shell/src/help.rs
tr -s ' \n' '  ' < engine/README.md | grep -c 'signs an Agent or a Resource in through `serve` and root.s Conduit to its owner'
tr -s ' \n' '  ' < engine/README.md | grep -c 'where the instance.s sign-in hook is plain, as in the 0.2.0 Agent packages'
```

`help.rs`'s `COMMANDS` holds one entry more than the start check found (item 6):

```bash
grep -o 'const COMMANDS: \[(&str, &str, &str); [0-9]*\]' engine/crates/shell/src/help.rs
```

Each of these prints 0:

```bash
tr -s ' \n' '  ' < engine/README.md | grep -c 'The sign-in command is not built yet'
grep -c 'Terminal' engine/crates/shell/src/server_tests.rs
grep -c 'apiKey' engine/crates/shell/src/sign_in.rs
grep -c 'signInReturn' engine/crates/shell/src/sign_in.rs
git diff HEAD --stat -- engine/crates/core conformance spec | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status
and count above, the start check's outputs, which predecessors had merged on your base and
`HEAD`'s commit, the names the start asks you to report, the signatures of decisions 2 to 5's
functions, the negative control's failure message, and the findings of decision 11.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new file and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. Before
the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md), point
10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and the merge does
not wait for it. Under [ruling 87](../proposals/ruling-87/README.md) this order runs one Codex
round: a fresh Codex verifier reads round 1 and no second Codex round follows; a fresh read-only
Opus reader reads the result itself, and the verifier's report where it holds a must
([ruling 79](../proposals/ruling-79/README.md), point 2, as ruling 87 amends it), and confirms or
refutes each of the verifier's musts, and an Opus fixer applies the musts it confirms. A77's
pairing is paused until 050n merges ([ruling 89](../proposals/ruling-89/README.md), point 6), so
this order is not paired. The admin writes this order's Record below when it merges.

Where the coordinator dispatched this order stacked on 048b-join's round-1 branch, the result is
read there, with `tools/gate.py --base` set to the commit this order started from, that branch's
tip, since `origin/main` lacks the predecessors' work; it merges only after 048b-join has merged,
and with it orders 048a-5d to 048a-5gh before it, and then only once the coordinator has done the
following (the coordinator's, under that lever):

- rebase this order's branch onto `main` with `git rebase --onto origin/main` from 048b-join's
  round-1 tip, the commit this order started from, so that only this order's commits are replayed
  onto 048b-join's squash, committing and rebasing under the work orders' identity. Where
  048b-join's branch was itself rebased, onto 048a-5gh's squash or an earlier one, after this
  order started, the commit this order started from is still the one to rebase from;
- resolve any conflict where a predecessor's verifier or fixer changed, after its round 1, what
  this order changes or calls: in `engine/crates/shell/src/server_tests.rs`, 048b-join's helpers,
  `SignInScript`, `U` and its three tests, beside which decision 8's tests and `PollScript` are
  added, keeping the predecessors' merged text and applying this order's decisions to it; and
  write `engine/STANDING.md` again with `python3 tools/promise_standing.py --write` rather than by
  hand;
- run this order's start check in a clean worktree of `main` after 048b-join's merge, and where a
  predecessor's merge changed anything that a decision reads (a helper's name or parameters,
  `SignInScript`'s answers, `U`, a store's steps, a relay text that decision 4 matches, the plain
  hook's refusal that the third test asserts, or whether a further output repeats the first URL),
  amend this order's text and the build to it, saying so in the Record;
- run again decision 7's and decision 8's tests, 048b-join's three, the help test, the whole
  engine suite and `tools/gate.py` against `origin/main`;
- have the reader read the rebase's resolutions and any amendment as new content (ruling 79,
  point 2), and write this order's Record.

## Record

Not yet run.
