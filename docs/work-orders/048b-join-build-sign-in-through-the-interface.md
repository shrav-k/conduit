# Work order 048b-join (E7b-3): a sign-in started at the engine reaches the person through the Interface, and polling signs it in

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: this order adds no case, so `vectors` is an empty list, and its three
shell tests go under `checks`. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, add no crate, and
reach no other machine. Run one cargo command at a time. Never print a whole large file, case,
fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set`
or `export -p`, never print an environment variable's value, and never read or print a key, a
token or any credential. Test code may read the client token it issued in its own temporary store,
as `token` and `server::client` do, and nothing prints it. Do not run `claude`, `codex` or any
client, sign in nowhere, and open no URL: the URLs below are test values that no program fetches.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md) split spec batch 1's E7b, the shell relay, in two:
"048b-relay: the shell relays `input_required` (`server.rs`, `runner.rs`). M. After S0b.
048b-join: S-M." Each side was cut again (the coordinator's cuts, each open to the owner's strike):
the shell's side into 048b-relay1 and 048b-relay2, both merged; the engine's side, in the core,
into the parts of 048a-5 and 048a-6, each after the one before. On 2026-11-19 the owner answered two
questions on the path to the Mac dogfood, which ruling 88 records with the owner's words (until its
record merges, the coordinator's notes (private) hold them):

- "Defer 7 and 8 (Recommended)", whose option read: "048b-join comes right after 048a-5h and joins
  the URL relay and polling only; 048a-7, 048a-8 and 051b-3 (the return through Conduit, used by
  Codex from other machines once keyed) move after dogfood, beside the keyed release. Saves about 6
  hours on the critical path. Records a dated ruling." Under
  [ruling 86](../proposals/ruling-86/README.md) nothing is keyed in 0.2.0, so PND-013's first
  check fails for every 0.2.0 package and the carried return has no 0.2.0 user, and a browser login
  on the Mac needs no return.
- "Pairs: 5e+5f, 5g+5h (Recommended)": 048a-5e and 048a-5f become one order, 048a-5ef (the engine's
  message and the input-required output, with `signInLogin`'s members and DL18's expiry; and
  polling on the retry), and 048a-5g and 048a-5h another, 048a-5gh (another account found while
  polling undone by a sign-out part; and DL18's end), waiving FR-264's one behavior per order for
  these two, as 048a-5b did.

So the path to the Mac dogfood is order 209 (ruling 86's engine half), 048a-5d, 048a-5d2, 048a-5ef,
048a-5gh, this order, then order [051b-2](051b2-cli-sign-in.md) (`conduit sign-in`); order 050n,
the Agent packages 0.2.0 joined to `main`, follows this order and A5's live check. This order
follows 048a-5gh directly. Its return leg, the Interface's retry carrying `signInReturn` on to the
hook that started the login, moves with 048a-7 and 048a-8 after the dogfood, and is not built or
tested here. It is on the sign-in chain, so A77's amendment of 2026-11-19 does not pair it
([work orders](../implementation/WORK_ORDERS.md)).

[The targeted review's section 6](../proposals/review-signin-2026-11-16/README.md#048b-join) puts
nothing on 048b-join. The owner's [ruling 82](../proposals/ruling-82/README.md#the-decisions),
point 3, merges the Agent packages 0.2.0 only after 048b-join, onto an engine that runs the
account check; order 050n waits for this order's merge, and so does A5's live check
([experiment 027](../../experiments/027-a5-sign-in/README.md), "When, and under which Type").

**This text's refresh.** The coordinator wrote this order on `main` before order 048a-5b and
refreshed it on 2026-11-19, written ahead against `main` at `c097ec81`, where orders 048a-5,
048a-5b, 048a-5c, 048a-6, 048b-relay1 and 048b-relay2 have merged with their Records, and against
the texts of orders 209 and 048a-5d (refreshed, not yet merged) and of 048a-5d2, 048a-5ef and
048a-5gh, written beside it that day. Every change below is the coordinator's, open to the owner's
strike ([ruling 48](../proposals/ruling-48/README.md)), and follows the owner's two answers above:

- the order follows 048a-5gh directly, after 209, 048a-5d, 048a-5d2 and 048a-5ef, and no longer
  after 048a-7, 048a-8 or 048b-relay2's planned successor; its start check stops where any of them
  has not merged;
- the path it pins ends in polling: the Interface's retry carries the person's acceptance and no
  return, `account-hook.v1` is polled, and the expected account signs the instance in; the earlier
  text's retry of `sign-in-hook.v1` carrying a return, `signInLogin`'s `returned` and the rebuilt
  redirect value are gone, with assumption A4;
- the seam of the first request is joined on `main` already (orders 048a-5 and 048a-6), so this
  order changes no source file: `server.rs` leaves its scope, and decision 2 is a reading only;
- a test of a plain hook is added, on the plain manifest of orders 207 and 048a-5d, since the
  Agent packages 0.2.0 are plain (ruling 86) and the dogfood runs them;
- the polling test asks again once before the account appears, and a declined retry that carries
  a return passes on and keeps none of it (PND-013's first sentence for a decline);
- the negative control removes the forwarding of `clientCapabilities` and `signInLogin`, the seam
  the dogfood path needs, in place of the forwarding of `signInReturn`;
- the run is one Codex round, under ruling 87 (below, "Done when").

**What it builds.** One behavior: `sign-in.v1` through the shell, out to the person and back by
polling. IFC-049: "With that capability tools/call may return InputRequiredResult with resultType
input_required and exactly one inputRequests entry: elicitation/create with mode url, message and
url", "For sign-in the engine writes the message: it names the instance, its Type and package, its
node, the URL's host, the expected account and what the person brings back, if anything", "No text
from the hook is relayed", and "The client echoes it exactly; the engine treats it as
attacker-controlled and rejects tampering, replay of a consumed state or another principal. Each
retry consumes its state; another input-required response issues a fresh state for that same
invocation. For sign-in, accept means consent, not completion: retries poll under PND-010, blocking
or returning another InputRequiredResult until the account appears or the wait ends." IFC-049 too:
`signInLogin` "In each sign-in.v1 InputRequiredResult and its final result ... holds {name, return,
node, acceptsReturn} for the relayed login: ... acceptsReturn is true exactly where PND-013's first
check passes." PND-010: "Each retry polls account-hook.v1 to check the account after the person's
sign-in, under PND-011, until an account appears or DL18 ends; no account yet is not failed, and a
poll that failed or ended unknown counts as no account yet, so that only a later check that reports
the expected account completes sign-in.v1. The same invocation may answer another
InputRequiredResult or block. Decline or cancel received on a retry ends failed." PND-013: "A
retry that declines or cancels ends failed under PND-010 and passes on no value." IFC-045: the engine declares "elicitation URL mode {"url": {}} only for requests and
hooks to instances whose Type states signIn". PND-014: the value is kept nowhere. No promise
changes: the texts state all of it, and the orders before this one build each half. What is left
is the proof that the halves meet over the engine's real MCP server (decision 6).

**What the orders before it bring, which this order assumes.** Below, `META/x` is the key `x` under
the prefix that `spec/names.yaml` sets (in the tests, `shell.context.meta_prefix`). Where you find
at the start that one merged otherwise, apply the nearest sound reading of each decision below to
what merged, and say so; where an order's Record names a function, a member or a case, use that
name; but where the start check prints otherwise, stop.

- **A1, from 048a-5 and 048a-6 (merged).** `AccessRequest` (`engine/crates/core/src/access.rs`)
  has the members `client_capabilities`, `sign_in_login` and `sign_in_return`. In `server.rs`'s
  `call_tool`, the `invoke` input's `request` is extended with `sign_in_members(&meta, prefix)`,
  which copies the request's `_meta` key `io.modelcontextprotocol/clientCapabilities` as
  `clientCapabilities` and `META/signInLogin` as `signInLogin`, each only where present and not
  null; `signInReturn` is added after it, outside `sign_in_members`, only to the `invoke` input;
  and `record_rejection` uses `sign_in_members` alone.
- **A2, from 048b-relay1 (merged).** `Transition` has `input_required: Vec<InputRequired>`, each
  `{invocation, attempt, request, meta, expires}`; `call_tool` relays an entry given to its ticket
  as `{"inputRequired": V}` through `relay::relay`, as `CallToolResponse::InputRequired` whose one
  input request is under the key `url`, whose `_meta` is the entry's `meta`, and whose
  `requestState` is 43 characters the shell issues in `server/relay.rs`, bound to the
  authenticated Interface, the tool, the invocation and the attempt, expiring at `expires`. A
  retry carrying `requestState` and `inputResponses` is taken there and, accepted, reaches the
  core as `{"kind": "respond", "invocation", "attempt", "identity", "responses", "meta"}`, `meta`
  being the retry's whole `_meta`; a refused one is answered with `ErrorData::invalid_params`, a
  consumed state's text beginning `requestState was already used`.
- **A3, from 209, 048a-5d and 048a-5d2.** `sign-in.v1` dispatches `account-hook.v1` first and, for
  a check that finds no account or another or that ends failed or unknown, a `sign-in-hook.v1`
  part, each by a `dispatch` intent; a call that carries Conduit's keys is named by the hook's local
  name and holds the asked-for login in `META/signInLogin` (IFC-006), and a call to a plain item is
  named from its `tools` and holds none of Conduit's keys. The hook's `InputRequiredResult`, received
  as `{"kind": "result", "result": R}`, is taken by `sign-in.v1` (IFC-010), its form checked
  (048a-5d) and its URL matched to exactly one stated login (048a-5d2); the implementation's
  requestState is kept with the part and never relayed, recorded or shown.
- **A4, from 048a-5ef.** A URL that passes is relayed through A2's `input_required`, with the
  engine's own message and `signInLogin`'s members in `meta`, `acceptsReturn` read from the hook
  call's own `plain`, the interim ending of orders 048a-5b, 209 and 048a-5d gone, and the hook part
  kept in progress; an accepted retry polls `account-hook.v1`; the expected account ends the
  invocation succeeded as 048a-5b does (output `account`, any unanswered sign-in action answered
  `signed-in`, and the instance's entry removed from `State`'s `sign_in_holds`, which lifts
  PND-005's hold); no account yet answers another input request, with a fresh `requestState`;
  decline or cancel ends it failed (PND-010), dispatching no part; and whenever the invocation ends,
  its waiting hook part ends first (`end_hook_wait`, which removes `hook_input`), and its final
  result holds `signInLogin`.
- **A5, from 048a-5gh.** Another account found while polling is undone by a sign-out part; and at
  the expiry of each output it relays, an input naming the invocation and attempt ends that
  `sign-in.v1` unknown if it still waits on that attempt, with no hook run later (DL18).
- **A6, from 048b-relay2 (merged).** Each call to an instance whose Type states `signIn` holds
  `{"elicitation": {"url": {}}}` in `call.params._meta`'s
  `io.modelcontextprotocol/clientCapabilities` (IFC-045), written by the core in `dispatch_intent`,
  and `runner.rs` sends a call as the core writes it; its Record names the test
  `intent_declares_url_elicitation_only_toward_sign_in_types`, in `conduit-core`.
- **A7, the fixtures.** The case `requests-sign-in-hold` (048a-3, with 048a-4's inserted step) has
  these first nine steps: first start; registration; root's `group.set-up.v1` of `signed`; two run
  reports; the account check's completion, which ends the setup succeeded; root's
  `access.open.v1` of `to-signed`; root's `to-signed.work.v1`; and its completion LAPSE, which
  raises the instance's sign-in action and begins its hold. Its Type `signed-agent.v1` is 048a-1's,
  with the logins `browser` (`return: redirect`, `carried: true`) and `device`; its `signed-server`
  is not marked `plain`, so its calls carry Conduit's keys; its Agent `signed` has the account
  `primary-account`. The case `requests-sign-in-plain-hook-part` (048a-5d's decision 8) uses the
  same Type with order 207's plain manifest, whose `signed-server` is marked `plain: true`, names
  the hooks `account_hook` and `sign_in_hook` in `tools` and lists nothing under `keyed`; its steps
  1 and 2 are first start and registration, and its step 3 is root's first `group.sign-in.v1` of
  `signed`.
- **A8, requestState.** The shell issues, keeps and checks a sign-in's requestState (048b-relay1);
  the core holds none, and a retry reaches it only as `respond`.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`c097ec81`; the orders above move some. Find each place by the text quoted.

- `server.rs`'s `call_tool` (`async fn call_tool(`, line 398) takes a retry first (`let retry =
  match (request.request_state.as_deref(), request.input_responses) {`), then builds the `invoke`
  input and extends it with `sign_in_members` (line 476), relays an `inputRequired` reply (line
  520), and otherwise returns the core's result. `sign_in_members` is at line 572 and
  `record_rejection` at line 588. Order 048a-5gh adds to the relay branch, before the input-required
  result is returned, a call of `end_sign_in_wait_at` (server.rs), which starts one task per relayed
  output; that task submits `sign-in-wait-ended` at the output's expiry, or returns at shutdown.
  Decision 6's tests cancel `stop` at their end, as `forged_retry_state_is_refused_and_recorded`
  does.
- rmcp 3.9.0 gives the client `call_tool_once`, which returns `CallToolResponse` without driving
  the rounds; `CallToolRequestParams` has `meta`, `request_state` and `input_responses`; and an
  `InputRequiredResult` reaches only a peer on revision `2026-07-28` or later. In
  `server_tests.rs`, `modern` connects such a client, `fixed_shell` gives a store with a fixed
  engine URL, `token` reads a client token, `ready` waits for the server,
  `forged_retry_state_is_refused_and_recorded` drives a refused retry over MCP, and
  `pending_http_completion_and_client_cancellation` serves a request with an executor of its own.
- `tests.rs`'s `established` opens a store and commits the S1 case's steps through
  `Shell::commit`, pushing each step's evidence and keeping its captures through the test trace;
  `fixture_input` builds a register input's project and snapshots, from S1's own project alone.
- `server.rs`'s `client`, the helper behind `conduit call`, uses rmcp's `call_tool` with no
  elicitation handler, so it cannot answer an `input_required` result; order 051b-2 builds the
  client that can.

**When this order starts.** Orders 209, 048a-5d, 048a-5d2, 048a-5ef and 048a-5gh, and every order
before each, must have merged to `main`. Before you change anything, check from the repository root
that you find what they give, and stop, change nothing and report if any check prints otherwise:

```bash
grep -c '^## Record' docs/work-orders/209-build-keyed-hooks-and-sign-in-fixes.md docs/work-orders/048a5d-build-sign-in-hook-part.md docs/work-orders/048a5d2-*.md docs/work-orders/048a5ef-*.md docs/work-orders/048a5gh-*.md
grep -c '^Not yet run\.$' docs/work-orders/209-build-keyed-hooks-and-sign-in-fixes.md docs/work-orders/048a5d-build-sign-in-hook-part.md docs/work-orders/048a5d2-*.md docs/work-orders/048a5ef-*.md docs/work-orders/048a5gh-*.md
grep -c 'pub input_required: Vec<InputRequired>' engine/crates/core/src/lib.rs
grep -c 'pub sign_in_holds: BTreeMap<String, Value>' engine/crates/core/src/lib.rs
grep -c 'request_state' engine/crates/core/src/access.rs
grep -c 'fn request_states_bind_interface_tool_and_single_use' engine/crates/shell/src/server/relay.rs
grep -c 'pub(crate) fn sign_in_members(' engine/crates/shell/src/server.rs
grep -c '\.extend(sign_in_members(&meta, prefix));' engine/crates/shell/src/server.rs
grep -rl 'hook_input' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"end-sign-in-unrelayed"' engine/crates/core/src | wc -l | tr -d ' '
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Until order 048a-5e builds the relay'
grep -c 'requests-sign-in-return-kept-nowhere' conformance/expected-failures.yaml
ls -d conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-plain-hook-part conformance/conventions/requests/sign-in-login-plain-hook 2>/dev/null | wc -l | tr -d ' '
python3 -c 'import json; i = json.load(open("conformance/conventions/requests/sign-in-login-plain-hook/manifest.json"))["implementations"]["signed-server"]; print(i["plain"], "keyed" in i, [v for k, v in sorted(i["tools"].items()) if k.endswith("-hook.v1")])'
python3 -c 'import json; c = json.load(open("conformance/conventions/requests/sign-in-plain-hook-part/case.yaml")); s = c["steps"]; print(c["files"]["manifest"], s[0]["input"]["kind"], s[1]["input"]["kind"], s[2]["input"]["request"]["tool"])'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'The shell passes .clientCapabilities., .signInLogin. and, in a request it does not reject, .signInReturn. to the core\.'
grep -rlE 'sign_in_(relays|decline|plain_hook)_through_the_interface|fn established_case|struct SignInScript|fn url_client' engine/crates/shell/src | wc -l | tr -d ' '
```

They print, in order:

1. five lines, each ending `:1`, and 2. five lines, each ending `:0`: the five orders have Records
   and none of them still reads "Not yet run."; where a file is missing, grep says so: its order
   has not merged, so stop;
3. `1`, `1` and `0`: relay1's output (A2), 048a-3's holds (A7), and no `requestState` in the core's
   request (A8);
4. `1`, `1` and `1`: relay1's states (A2), and 048a-5's `sign_in_members` and its call in
   `call_tool`, the line the negative control removes (A1);
5. a number other than `0`: 048a-5d's kept requestState (A3);
6. `0` and `0`: the interim ending's decision and its DL18 sentence are gone (A4);
7. `0`: 048a-6 lifted the expected failure;
8. `3`: the two cases and order 207's plain manifest (A7);
9. `True False ['account_hook', 'sign_in_hook', 'sign_out_hook']`: the plain item (A7);
10. `../sign-in-login-plain-hook/manifest.json first-start register-project group.sign-in.v1`:
    048a-5d's plain case (A7);
11. `1`: 048a-6's sentence;
12. `0`: none of this order's work.

Then, from `engine/`, run 048b-relay2's test for IFC-045 (A6); it must report 1 passed and 0
failed, or stop and report it:

```bash
cargo test --release -p conduit-core intent_declares_url_elicitation_only_toward_sign_in_types
```

Then take from the Records of 048a-5ef and 048a-5gh, and report, as names only and not as a reason
to stop: the decision name an accepted retry takes when it polls `account-hook.v1`; the one it takes
when the poll finds no account yet and asks again; the function or decision that ends a hook part
whose `sign-in.v1` has ended, if any; and the function that submits DL18's input at an output's
expiry, with its file. Still before any change, from `engine/`, one at a time, run these and keep
each last line, which "Done when" compares with:

```bash
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell
cargo run --release -p conduit-conformance -- .. requests-
```

Orders that land beside this one (050 and others): whichever lands second keeps both
changes and writes `engine/STANDING.md` again with the tool.

**What the coordinator rechecks at dispatch.** This text is written ahead, beside the texts of
048a-5d2, 048a-5ef and 048a-5gh, which it follows. Before dispatch the coordinator reads their
Records and amends this order where any differs from those texts, in particular:

- the files' names in the start check, and ruling 88's record, which holds the owner's two answers
  of 2026-11-19, and ruling 87's (merged as #400), each cited with its link;
- order 117's entry for 048b-join and the index row in [the index of work orders](README.md), which
  must read "after 048a-5gh", with the return leg after the dogfood;
- 048a-5ef's message (decision 6 asserts only that it contains `example.org` and
  `primary-account` and no text of the hook), the members of `signInLogin` it writes, and that it
  gives `acceptsReturn` `true` for the keyed `browser` login and `false` for a plain item, as its
  text has them (its decisions 1, 3 and 8);
- that a poll finding no account answers another input request rather than blocking, with a fresh
  `requestState` on the same tool (decision 6's step 4), and the attempt it names;
- that a hook part does not outlive the end of its `sign-in.v1` through a decline or a signed-in
  answer: 048a-5ef's text ends it with `end_hook_wait`, which removes `hook_input`, so the file
  half of decision 6's step 9 stands; where its Record says otherwise, that half is dropped before
  dispatch (a cancel or a restart leaves the part, and no test here cancels or restarts);
- whether 048a-5ef or 048a-5gh changed, beyond 048a-5gh's timer call in the relay branch,
  `call_tool`, `sign_in_members` or `server_tests.rs`'s helpers, or the section of
  `engine/CONFORMANCE.md`, "IFC-049's relay (order 048b-relay1)", that decision 7 follows;
- each line number, count and expected output above, against `main` at dispatch.

**Who decided what.** IFC-045, IFC-049, PND-010, PND-011, PND-013, PND-014 and DL18 decide what
crosses the shell, in which form, and what is kept nowhere. The owner decided on 2026-11-19, as
ruling 88 records, that this order follows 048a-5gh directly and joins the URL relay and polling
only, its return leg moving with 048a-7 and 048a-8 after the dogfood, and that the chain before it
is batched in two orders. Everything else here is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)): the split of 048b into 048b-relay and this order
(order 117); the refresh's changes listed above; reading A8 as the plan settles it; the five seams
of decision 1 and the rule that this order joins none of them, a seam that does not meet being a
reason to stop; keeping this order to tests and one sentence; taking 048b-relay2's own test as the
start's evidence for A6; checking DL18's end by its Record (decision 5); the three tests, their
fixtures, their script, the URL, the value and every other value in them; asking again once in the
first test; the plain test; the negative control; the sentence of decision 7; and the findings of
decision 8. Apply them as written and report them under `decisions` as the coordinator's.

**Read first:**

1. In `spec/`: IFC-045, IFC-049, IFC-006, IFC-007 and IFC-010 (`spec/interface/implementation.yaml`);
   PND-005, PND-010, PND-011, PND-013 and PND-014 (`spec/promises/pending.yaml`); DL18
   (`spec/implementation-defined.yaml`); `sign-in.v1`, `sign-in-hook.v1` and `account-hook.v1` in
   `spec/core/operations.yaml`.
2. Orders 048a-5d, 048a-5d2, 048a-5ef and 048a-5gh, whole, with their Records; the Records of
   orders 209, [048b-relay1](048b-relay1-build-input-required-relay.md) and
   [048b-relay2](048b-relay2-build-url-elicitation-and-part-retry.md); order
   [048a3](048a3-build-sign-in-hold.md) (decisions 1 and 6).
3. The cases `requests-sign-in-hold` and `requests-sign-in-plain-hook-part`, their `case.yaml` and
   the files their `files` member names.
4. In `engine/crates/shell/src/`: `server.rs` (`call_tool`, `sign_in_members`, `record_rejection`),
   `server/relay.rs`, `runner.rs` (`call`), `lib.rs` (`Executor`, `commit` and the intent loop),
   `server_tests.rs` (`modern`, `fixed_shell`, `token`, `ready`,
   `forged_retry_state_is_refused_and_recorded`, `pending_http_completion_and_client_cancellation`)
   and `tests.rs` (`established`, `fixture_input`); in the conformance runner,
   `engine/crates/conformance/src/steps/resolve_project.rs` and
   `engine/crates/conformance/src/packages.rs` (`run`, `load`, `builtin`), for how a case's own
   project is built.

**Size: S-M**: about 0.35 to 0.5k changed lines of Rust, all in `server_tests.rs`: three tests, four
helpers and the script, `established_case`'s rebuilding of the runner's project being about 80 to
120 of them; and one sentence in `engine/CONFORMANCE.md`. No source file outside the tests changes.

## The decisions it applies

1. **Five seams, each a predecessor's.** The path crosses the shell at five seams: the first
   request in (A1, decision 2); the hook parts out and their answers back (A3, A6, decision 4); the
   input request out (A2, A4, decision 3); the retry in and its polling (A2, A4, decision 3); and
   DL18's end (A5, decision 5). Every one is joined by an order before this one, so this order
   changes no file under `engine/crates/` but `server_tests.rs`. For each, report which order
   joined it, by file and function. Where decision 6's tests show that one does not meet, stop and
   report the seam, the step and the reply, less any value the tests say is kept nowhere: change
   nothing outside Scope, since the core is the 048a orders', the relay 048b-relay1's and the
   runner 048b-relay2's. Decision 6's tests decide whether the path is joined.

2. **The first request's `_meta` reaches the core: a reading.** Read `call_tool` and report, by file
   and function, where each of `clientCapabilities`, `signInLogin` and `signInReturn` reaches the
   `invoke` input's `request` (A1). Change nothing in `server.rs`: where one is missing, stop and
   report it.

3. **The input request out, and the retry in.** These are 048b-relay1's and 048a-5ef's (A2, A4),
   and a sign-in's output is relayed as any other. Change nothing in `server/relay.rs` or in
   `call_tool`.

4. **The hook parts.** These are 209's, 048a-5d's, 048a-5d2's and 048b-relay2's (A3, A6): the core
   writes each part's call whole and the runner sends it as written. Change nothing in `runner.rs`.
   Report every `cancel` intent the script receives in decision 6's tests, with the operation it
   names, as an observation.

5. **DL18's end: a check.** This order builds no timer and its tests do not wait for an expiry.
   Report the function that 048a-5gh's Record gives for the input that ends a `sign-in.v1` with no
   retry unknown at its output's expiry (A5), and that it is reached from the shell. Only where it
   is not, report it as a finding for the coordinator; it is not this order's to fix.

6. **Three shell tests pin the path.** In `engine/crates/shell/src/server_tests.rs`. `META/x` is
   written in the tests as `format!("{}x", shell.context.meta_prefix)`.

   *The store.* Add a helper `established_case(path: &Path, case: &Path, through: usize) -> Shell`
   that opens a store as `established` does and commits, through `Shell::commit`, the case's steps
   1 to `through`, each with its evidence pushed and its captures kept as `established` keeps them.
   The case's files are those its `files` member names, each path read relative to the case's
   directory. A step that registers or resolves a project gets the project and snapshots built
   from those files as the conformance runner builds them: `steps/resolve_project.rs` routes the
   step to `packages.rs`'s `run`, which parses the fixture (its snapshots with archive digests, its
   artifacts and the project recipe) and builds each project with `read_submitted_files`. Every
   input is otherwise changed as `fixture_input` changes it: captures and the core package's
   shorthand expanded, `evidence` removed (and pushed), `trustDomain` removed from first start, and
   `file` and `reverse` removed from a registration, with `builtin` set to `shell.context.builtin`
   as `fixture_input` sets it. `established_case` reimplements `run` and `load` in the test with
   `conduit_loader::read_submitted_files` and adds no dependency: the shell does not use
   `conduit-conformance`, and its `Cargo.toml` is out of scope. It then clears the evidence queue
   and drops the trace, as `established` does. Add `fixed_case_shell(path, case, through)`, which is
   `fixed_shell` with `established_case` in place of `established`. No fixture file is added or
   copied.

   *The client.* `modern` serves `()`, whose `ClientConfig::default()` declares no elicitation, and
   on the `modern` client rmcp 3.9.0 writes the client's own configured capabilities into the
   `_meta` key `io.modelcontextprotocol/clientCapabilities` of every outbound request, over any
   value set by hand in the params `_meta`. Add a helper `url_client(url, token, prefix)` beside
   `modern` that serves a `ClientConfig` (rmcp implements `ClientHandler` for `ClientConfig`) whose
   `capabilities.elicitation` is `ElicitationCapability::new().with_url(..)` and whose
   `capabilities.extensions` holds the one entry `format!("{prefix}implementation-interface")`
   with the value `{"revision": "v0"}`, as the tests in `server_tests.rs` that send a key under the
   prefix declare it: without it the shell's HTTP layer refuses with `-32021` under IFC-036 any
   request that carries a key under the prefix, before `call_tool` runs. It connects through the
   same `serve_with_lifecycle` Discover on `2026-07-28`. Every request below is sent through it. Let
   `C` be the `_meta` key `io.modelcontextprotocol/clientCapabilities` that rmcp writes from it,
   holding elicitation `{"url": {}}`; the tests do not set `C` by hand.

   *The script.* Add an executor `SignInScript`, in place of a runner, shared with the test through
   an `Arc`, holding a flag `plain` set when it is made, a list of every `dispatch` intent it
   receives, a list of every `cancel` intent it receives (its `cancel` answers `Execution::Done`,
   as the trait's default does), and an `account` that the test sets, JSON null at first. Every
   other method of `Executor` keeps its default. Its `dispatch`, by the intent's
   `call.params.name`:
   - `account-hook.v1`, or `account_hook` where `plain`: the result
     `{"content": [], "structuredContent": {"account": A}}`, `A` being the script's `account` at
     that moment, with `"_meta": {"META/outcome": "succeeded"}` added where not `plain`;
   - `sign-in-hook.v1`, or `sign_in_hook` where `plain`: the InputRequiredResult
     `{"resultType": "input_required", "inputRequests": {"login": {"method": "elicitation/create", "params": {"mode": "url", "message": "Open the page to sign in.", "url": U}}}, "requestState": "hook-state-1"}`;
   - any other: `{"content": []}`, with `"_meta": {"META/outcome": "succeeded"}` added where not
     `plain`.

   Each answer is given as `Execution::Complete` of `{"kind": "result", "result": R}`, `R` the
   result above. Setting `account` to `"primary-account"` between two retries stands for the person
   finishing the browser login on the node's machine, where the program's own listener takes the
   redirect and nothing comes back through the Interface.

   `U` is the URL for the login `browser` as `signed-agent.v1` states it (authorization
   `https://example.org/authorize`, client ID `placeholder-signed-agent`, redirect host
   `127.0.0.1`, ports 8400 and 8401, path `/callback`), which passes IFC-049's match:

   ```text
   https://example.org/authorize?client_id=placeholder-signed-agent&response_type=code&redirect_uri=http%3A%2F%2F127.0.0.1%3A8400%2Fcallback&code_challenge_method=S256&code_challenge=E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM&state=join-state-0123456789abcdef
   ```

   Let `R` be `http://127.0.0.1:8400/callback?code=join-code-0001&state=join-state-0123456789abcdef`,
   a plainly fake return used only in the second test, `T` the tool `group.sign-in.v1`, and `G` the
   arguments `{"instance": {"kind": "Agent", "name": "signed"}}`. The script is wrapped in
   `ServingExecutor::new` and served with `server::serve`, as in
   `pending_http_completion_and_client_cancellation`; root is issued a client token with
   `issue_client_token`, read with `token`, and connected with `url_client`. "Kept nowhere" below
   means: in no reply named, serialized; in no record of the store's state; and in no file under the
   store's directory, read as bytes.

   *The first test*, `sign_in_relays_through_the_interface_and_polls_to_signed_in`, on
   `fixed_case_shell(temp, <the repository>/conformance/conventions/requests/sign-in-hold, 9)`
   with a script not `plain`, asserts in this order:
   1. `sign_in_holds` has an entry for `signed`'s ID (A7).
   2. Root's `call_tool_once` of `T` with `G` and `_meta` `META/signInLogin` `{"name": "browser"}`
      gives `CallToolResponse::InputRequired` `I1`. `I1`'s `inputRequests` has exactly one entry,
      of key `url`; serialized, its `method` is `elicitation/create`, its `params.mode` is `url`,
      its `params.url` equals `U`, and its `params.message` contains `example.org` and
      `primary-account` and contains neither `hook-state-1` nor `Open the page to sign in.`
      (IFC-049: the engine writes the message, and no text from the hook is relayed). `I1`'s
      `requestState` is a string `S1` of 43 characters. `I1`'s `_meta` holds `META/signInLogin`
      with `name` `browser`, `return` `redirect`, a `node` member, and `acceptsReturn` `true`, and
      no `returned` member.
   3. The script holds two `dispatch` intents: an `account-hook.v1` call, then a `sign-in-hook.v1`
      call whose `call.params._meta` holds `META/signInLogin` `{"name": "browser"}` (IFC-006) and
      whose key `io.modelcontextprotocol/clientCapabilities` holds `elicitation` `{"url": {}}`
      (IFC-045).
   4. Root's `call_tool_once` of `T` with `G`, `request_state` `S1` and `input_responses`
      `{"url": {"action": "accept"}}`, the script's `account` still null, gives
      `CallToolResponse::InputRequired` `I2`, with one entry of key `url` whose `params.url` equals
      `U`, and a `requestState` `S2` of 43 characters, not `S1`; the script holds three `dispatch`
      intents, the third an `account-hook.v1` call; and `sign_in_holds` still has the entry: no
      account yet is not failed (PND-010).
   5. The test sets the script's `account` to `"primary-account"`. Root's `call_tool_once` of `T`
      with `G`, `request_state` `S2` and `input_responses` `{"url": {"action": "accept"}}` gives
      `CallToolResponse::Complete` `F`: `F.is_error` is not `Some(true)`, its `structuredContent`'s
      `account` is `primary-account`, its `_meta` `META/outcome` is `succeeded`, and its `_meta`
      `META/signInLogin` has `name` `browser` and no `returned` member.
   6. The script holds four `dispatch` intents, the fourth an `account-hook.v1` call; none of the
      four holds `requestState` or `inputResponses` in `call.params`: with no return, the hook that
      started the login is not called again (PND-013 retries it only to pass a return on).
   7. `sign_in_holds` has no entry for `signed`'s ID: the success lifted the hold.
   8. Root's `call_tool_once` of step 5's request again, with the same `S2`, gives an `Err` that is
      `ServiceError::McpError(e)`, with `e.code` `-32602` and `e.message` beginning
      `requestState was already used`, and the script still holds four `dispatch` intents: a
      consumed state reaches no invocation (IFC-049).
   9. `hook-state-1`, the implementation's own requestState, is kept nowhere, in the replies of
      steps 2, 4, 5 and 8 (A3); and `U` is in neither `F` nor any record of the store's state
      (IFC-049: no tool content, `structuredContent` or record holds it).

   *The second test*, `sign_in_decline_through_the_interface_ends_failed`, on a store of its own
   built as the first test's, with a script not `plain`: steps 1 to 3 of the first test, the first
   request with no `_meta` of its own (so carrying `C` alone) and step 3 asserting no
   `META/signInLogin` in the hook call; then root's retry of `T` with `G`, `request_state` `S1`,
   `input_responses` `{"url": {"action": "decline"}}` and `_meta` `META/signInReturn`
   `{"value": R, "via": "paste"}` gives `CallToolResponse::Complete` whose `is_error` is
   `Some(true)`, whose `_meta` `META/outcome` is `failed`, and whose `structuredContent`'s
   `diagnostics` hold one of `ruleId` `PND-010` (PND-010, and PND-013: a retry that declines passes
   on no value); the script still holds two `dispatch` intents; `sign_in_holds` still has an entry
   for `signed`'s ID; and neither `R` whole nor `join-code-0001` is kept anywhere (PND-014).

   *The third test*, `sign_in_plain_hook_through_the_interface_signs_in`, on
   `fixed_case_shell(temp, <the repository>/conformance/conventions/requests/sign-in-plain-hook-part, 2)`,
   so that the store holds the plain item after first start and registration (A7), with a script
   `plain`:
   1. Root's `call_tool_once` of `T` with `G`, carrying `C` and no `_meta` of its own, gives
      `CallToolResponse::InputRequired` whose one entry, of key `url`, has `params.url` equal to
      `U`, whose `requestState` `S1` has 43 characters, and whose `_meta` `META/signInLogin` has
      `name` `browser` and `acceptsReturn` `false`: PND-013's first check fails for a plain hook,
      as for every 0.2.0 package (ruling 86).
   2. The script holds two `dispatch` intents, an `account_hook` call and then a `sign_in_hook`
      call, neither of whose `call.params._meta` holds a key that begins with the prefix (IFC-007),
      and the second's key `io.modelcontextprotocol/clientCapabilities` holds `elicitation`
      `{"url": {}}` (IFC-045).
   3. The test sets the script's `account` to `"primary-account"`. Root's `call_tool_once` of `T`
      with `G`, `request_state` `S1` and `input_responses` `{"url": {"action": "accept"}}` gives
      `CallToolResponse::Complete` whose `is_error` is not `Some(true)`, whose
      `structuredContent`'s `account` is `primary-account`, and whose `_meta` `META/outcome` is
      `succeeded`; the script holds three `dispatch` intents, the third an `account_hook` call.
   4. `hook-state-1` is kept nowhere, in the replies of steps 1 and 3.

   Before running them, check that no other test's name in `engine/crates/shell/` begins
   `sign_in_relays_through_the_interface`, `sign_in_decline_through_the_interface` or
   `sign_in_plain_hook_through_the_interface`. If a test fails because a reply differs from what it
   states, stop and report the test, the step and the reply, less any value of `R`: do not change
   the expectations to pass it.

7. **One sentence in `engine/CONFORMANCE.md`.** This decision replaces no predecessor's text. In
   "IFC-049's relay (order 048b-relay1)", find, read across line breaks, 048a-5ef's sentence ``From
   order 048a-5ef, `sign-in.v1` gives such an output when it relays its URL action, and an accepted
   retry reaches the core as its `respond` input, which polls the account (row DL18).``, and add
   right after it, in the same paragraph, this sentence:

   ```text
   `sign-in.v1` runs through the shell as far as polling: the request's URL elicitation and `signInLogin` reach the core, its URL action reaches the calling Interface as an input-required result with the shell's own requestState, and each accepted retry polls the account check, asking again until the expected account signs the instance in (order 048b-join).
   ```

   Where 048a-5ef's sentence is not found as written, add the sentence after that section's last
   sentence, and report it. Then run `python3 tools/promise_standing.py --write`, as AGENTS.md asks
   of a change to a promise ID in the shell's sources; report the rows it changes and the totals, or
   that it changes nothing.

8. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line, ruling's
   record, case, fixture, expected failure or source outside `server_tests.rs` changes. No retry
   that carries a return to the hook (048a-7, 048a-8), no CLI command (051b-2), no Interface
   listener for a return (PND-013's), no managed connection (050o) and no package (050). Report as
   findings, for the coordinator: that `server.rs`'s `client`, behind `conduit call`, cannot answer
   an `input_required` result, so A5's live check, which runs after this order merges, uses order
   051b-2's `conduit sign-in` as its client; that `acceptsReturn` is `true` for the keyed fixture's
   `browser` login while no return is yet passed on, where 048a-5ef's Record does not already say
   so; decision 5's DL18 ending, only where 048a-5gh's input does not end the invocation; each seam
   with the order that joined it; the `cancel` intents of decision 4; and any place where IFC-049,
   PND-010 or PND-013 left you a choice this order does not settle.

## Scope

Change only `engine/crates/shell/src/server_tests.rs` (decision 6); `engine/CONFORMANCE.md` (one
sentence, decision 7); and `engine/STANDING.md`, written only by the tool.
`docs/work-orders/048b-join-declared.yaml`, which declares no vector change, comes with this order;
do not change it. Not `engine/crates/shell/src/server.rs`, `server/relay.rs`, `runner.rs`,
`lib.rs` or `tests.rs`, `engine/crates/core/`, `engine/crates/loader/`,
`engine/crates/conformance/`, `spec/`, `IN_FORCE.md`, `conformance/`, `tools/`, `packages/`,
`experiments/`, `docs/implementation/` or `docs/proposals/`: report any defect as a finding.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell sign_in_relays_through_the_interface
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell sign_in_decline_through_the_interface
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell sign_in_plain_hook_through_the_interface
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell
cargo run --release -p conduit-conformance -- .. requests-
RUST_TEST_THREADS=4 CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third, the fourth and the fifth each report, in the library's run, 1 passed and 0 failed; at
the start each ran 0 tests. The sixth reports three more shell tests passed than the start's run,
0 failed; report both counts. The runner command ends with as many cases passed as at the start,
with its expected failures unchanged: this order changes no case; report both counts. If any other
test or case fails, stop and report it: do not change it. The threads are 4 because at 24 the runner
ran out of memory (order 071's Record).

Next, a negative control: show that the path fails without 048a-5's forwarding of
`clientCapabilities` and `signInLogin`. From `engine/`, one at a time, first save the file:

```bash
cp crates/shell/src/server.rs target/server.rs.joined
```

Then, in `crates/shell/src/server.rs`'s `call_tool`, remove by hand the one statement that
extends the `invoke` input's `request` with `sign_in_members(&meta, prefix)`, from
`input["request"]` to `.extend(sign_in_members(&meta, prefix));`, and nothing else, and run:

```bash
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell sign_in_relays_through_the_interface
cp target/server.rs.joined crates/shell/src/server.rs
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell sign_in_relays_through_the_interface
```

The first run must fail at the first test's step 2, the first request answered with the error
`-32021` under IFC-049 where an input request is expected, since the core sees no URL elicitation;
report its step and the error's code. After the restore the second run reports 1 passed. Restore
with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not rebuild
the crate. Git ignores `target/`. From `engine/`, `git diff -- crates/shell/src/server.rs` then
prints nothing.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048b-join-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance engine/crates/core engine/crates/loader engine/crates/conformance engine/crates/shell/src/server.rs engine/crates/shell/src/server engine/crates/shell/src/runner.rs engine/crates/shell/src/lib.rs engine/crates/shell/src/tests.rs engine/crates/shell/Cargo.toml engine/Cargo.lock tools packages experiments docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048b-join-declared.yaml: 0 shapes.`,
then `Vector delta: 0 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. The unit tests end `OK`; report the count. The `git status` command prints nothing.

The S1 short-form spike depends on the core, which this order does not change, so the local gate
does not run it for this change.

Each of these prints 1, and printed 0 at the start:

```bash
grep -c 'fn sign_in_relays_through_the_interface_and_polls_to_signed_in' engine/crates/shell/src/server_tests.rs
grep -c 'fn sign_in_decline_through_the_interface_ends_failed' engine/crates/shell/src/server_tests.rs
grep -c 'fn sign_in_plain_hook_through_the_interface_signs_in' engine/crates/shell/src/server_tests.rs
grep -c 'fn established_case' engine/crates/shell/src/server_tests.rs
grep -c 'fn fixed_case_shell' engine/crates/shell/src/server_tests.rs
grep -c 'fn url_client' engine/crates/shell/src/server_tests.rs
grep -c 'struct SignInScript' engine/crates/shell/src/server_tests.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'runs through the shell as far as polling: the request.s URL elicitation and .signInLogin. reach the core'
```

Each of these prints 0:

```bash
grep -c 'apiKey' engine/crates/shell/src/server_tests.rs
git diff HEAD --stat -- engine/crates/core engine/crates/shell/src/server.rs engine/crates/shell/src/server engine/crates/shell/src/runner.rs conformance spec | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the names the start asks you to take from the Records, each seam with the order that
joined it, decision 5's answer, the `cancel` intents of decision 4, and the sentence of "IFC-049's
relay (order 048b-relay1)" that decision 7's sentence follows, 048a-5ef's or that section's last.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the change and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. Before
the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048b-join-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. Under ruling 87 this order runs one Codex round: a fresh Codex verifier reads it, and a
fresh read-only Opus reader reads the result, and the verifier's report where it holds a must,
before it merges ([ruling 79](../proposals/ruling-79/README.md), point 2, as ruling 87 amends it);
where the verifier reported a must, the coordinator merges it. The admin writes this order's Record
at the foot of this file when it merges.
