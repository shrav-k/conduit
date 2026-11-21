# Work order 048a5ef (E7a-5ef): `sign-in.v1` relays its URL action and polls the account on each retry

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the two cases you add and each case whose steps
you change, with the promises each tests and whether it passes. This order has network access only
because the engine's tests bind loopback ports, which the sandbox refuses without it; use it for
nothing else, add no crate, and reach no other machine. Run one cargo command at a time. Never
print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n` or a count.
Never run `env`, `printenv`, `set` or `export -p`, never print an environment variable's value,
and never read or print a key, a token or any credential. Do not run `claude`, `codex` or any
client, and sign in nowhere.

## What and why

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry for 048a-2 cuts 048a-5 into parts, each
after the one before, since an order holds one behavior and stays small
([FR-264](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). On 2026-11-19 the
owner chose, as ruling 88 records, two things for the path to the Mac dogfood (the owner's words
are in that ruling's record):

- "Defer 7 and 8 (Recommended)": orders 048a-7 and 048a-8 (the carried return: PND-013's checks,
  a page login's code and a redirect return passed to the hook on its retry) and 051b-3 (the
  carried return from the command line) leave 0.2.0's path and the dogfood path and move after
  dogfood, beside the keyed release. Under [ruling 86](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-86/README.md) nothing
  is keyed in 0.2.0, so PND-013's first check fails for every 0.2.0 package, and a Mac browser
  login needs no return.
- "Pairs: 5e+5f, 5g+5h (Recommended)": 048a-5e (the engine's message and the input-required
  output) and 048a-5f (polling on the retry) become one order, this one, and 048a-5g and 048a-5h
  another, 048a-5gh (another account found while polling undone by a sign-out part; and DL18's
  end), waiving FR-264's one behavior per order for these two, as order 048a-5b's text waived it.

So the path to the Mac dogfood is order 209 (ruling 86's engine half), 048a-5d, 048a-5d2,
**048a-5ef, this order**, 048a-5gh, 048b-join (the URL relay and polling only), then 051b-2
(`conduit sign-in`), with 050n (the 0.2.0 packages joined to `main`) after 048b-join and the live
check. This order builds two behaviors: the relay (the engine's message, the input-required output
that order 048b-relay1's shell relays, `signInLogin`'s members and DL18's expiry), and polling on
the retry (the account checked again on each accepted retry, signing in, asking again, and a
decline or cancel ending failed). It is on the sign-in chain, so A77's amendment of 2026-11-19 does
not pair it, and [ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 6, pauses pairing until 050n
merges ([work orders](../implementation/WORK_ORDERS.md)), and under ruling 87 it runs one
Codex round, a worker and a fresh Codex verifier, whose musts go to the fresh Opus reader.

**This text.** The coordinator wrote it on 2026-11-19, ahead of its predecessors, by joining its
held drafts of 048a-5e and 048a-5f (written against `main` at `3c761dd9`, before ruling 86 and
before 048a-5d's refresh) and refreshing the result against `main` at `c097ec81`, against the texts
of orders 207 and 209 (ruling 86, neither yet built), against 048a-5d's refreshed text and against
the text of its cut 048a-5d2 (IFC-049's URL match), written beside this one. None of
048a-5d, 048a-5d2, 207 and 209 was merged as built at that commit, and "What to recheck at
dispatch" lists what the coordinator checks against their texts, builds and Records before this
order dispatches. Every change below is the coordinator's, open to the owner's strike
([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)):

- the two drafts are one order, with one start check, one scope, one declaration and one Done-when,
  as ruling 88 has it;
- everything of the carried return is dropped: no return is read from a retry, no retry of
  `sign-in-hook.v1` is sent, `signInLogin`'s `returned` is not built, and `retry_intent` stays
  unused (048a-7 and 048a-8, after dogfood);
- the final result's `signInLogin`, which the drafts left to 048a-5f and then to 048a-7, is built
  here, since 048a-7 has left the dogfood path and IFC-049 asks it of "each sign-in.v1
  InputRequiredResult and its final result" (decision 5);
- a poll that ends failed or unknown counts as no account yet and asks again, clearing nothing
  (PND-010 and PND-011 as order 207 writes them, ruling 86, point 3 (d)); only a poll that ends with
  a reason, such as `unavailable` under IFC-037, ends `sign-in.v1` failed under PND-011;
- the phases are `retry-pending` while `sign-in.v1` waits for the person's retry and `poll-pending`
  while its poll is in progress; 048a-5d's `sign-in-pending` stays the phase of the hook part before
  it answers;
- `relay_output` takes only the invocation and the step's time, so that a later output of the
  same invocation is one more call, and keeps the URL, the login's name, the hook call's `plain`
  and the login's `_meta` in the invocation's `sign_in_wait` until it ends; `finish` removes the
  URL;
- the hook part's per-call `plain`, which order 209 sets, decides `acceptsReturn` (PND-013's first
  check as order 207 writes it), so a plain hook's relay says `acceptsReturn` false, as every 0.2.0
  package's does;
- a plain hook's URL request is relayed and its part kept in progress (ruling 86, point 4): 048a-5d's
  plain case shows it relayed, and a unit test the part kept;
- each step of the earlier cases that expected the interim ending now expects the relay, and where
  a later step needs that sign-in ended, a declining retry is inserted after it, in place of the
  draft's restart, so that the sign-in ends failed under PND-010 as before and the decline is
  exercised;
- the core's step marks a `respond` reply's unanswered actions as it marks a completion's, so that
  a sign-in that a retry ends answers its caller as one that a completion ends (decision 6);
- the waiting hook part ends unknown inside the end of `sign-in.v1` and inside its signed-in
  answer, so every ending of a sign-in, 048a-5gh's included, ends it;
- an `input_required` answer of a hook part already relayed is dropped with a cause of its own
  (decision 4);
- interim texts name 048a-5gh where the drafts named 048a-5g or 048a-5h, and no new code names an
  order otherwise;
- every cited line, function, case, step and count is rechecked against `c097ec81` and the texts
  above, and those the earlier orders change are named in the start check.

**Its dispatch refresh.** The coordinator refreshed it again on 2026-11-19, for its dispatch,
against `main` at `0840a65f`, where order 207 has merged, order 209's text has merged (#407) and
048a-5d's text has merged refreshed (#408), and none of 209, 048a-5d and 048a-5d2 is built; it
also read order 209's first round as built. Every change below is the coordinator's, open to the
owner's strike ([ruling 48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)):

- the start check's grep of the line on which order 209 writes `dispatch_hook`'s literal, as the
  last read's fix wrote it, prints `0` on 209's first round, which writes `keyed_hook(` on a line
  of its own; it is dropped, as 048a-5d's refresh dropped it, and 209's cases, passing in the
  start's runner runs, show that `dispatch_hook` reads `plain` per call;
- it was checked against 048a-5d's refreshed text, whole: that order's third case now captures
  its hook as `signhook1`, so the start check, decision 12, its search and "Done when" name
  `$signhook1` there; and decisions 10 to 12 check, as 048a-5d's decision 8 does, that no
  capture name is bound twice;
- under the owner's choice on 2026-11-19 of the lever "Pipeline the chain", as the coordinator
  applies it: on the predecessor's round-1 branch ([ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md),
  point 2, open to the owner's strike), this order may start from 048a-5d2's round-1 branch, before
  048a-5d2 merges, with orders 209 and 048a-5d each merged or on its own round-1 branch below it:
  the start check accepts any of these bases, says how to tell them apart and that the texts on
  `main` govern where the base holds older copies, and the admin paragraph at the foot lists what
  the coordinator rebases and rereads when 048a-5d2 merges (a readiness finding in the coordinator's
  notes (private): the texts written ahead refused the stacked dispatch);
- under the owner's choice on 2026-11-19 of the lever "Less prescriptive orders", as the coordinator
  applies it ([ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 2, open to the owner's strike),
  the start check keeps only what the work needs: the checks of ruling 88's record, of order 117's
  entry and of the predecessors' Records are dropped (the Records now only tell the bases apart),
  and so are the greps of 048a-5d's names that no decision calls or changes (the cases' lengths
  stay, since decision 12 names their last steps and counts the steps after them); the greps that
  pinned only the name "048a-5e" in the sources and counted it in `engine/CONFORMANCE.md` are
  dropped, or kept as a list reported with no stop (a readiness finding in the coordinator's notes
  (private)), so that a predecessor's fixer who corrects that name stops nothing; and where a
  function a decision changes, a sentence decision 6 replaces or a place decision 13 replaces is on
  the base under another name or in other words, the worker uses what it finds and reports it, and
  stops only where it is missing;
- in "Done when", the lengths of the changed cases are given against the start, the vectors run is
  folded into the whole engine suite, which runs once, last, after the mutations (a readiness
  finding in the coordinator's notes (private)), the spike's test count is no longer pinned, the
  count of `signInLogin`'s marks is given against the start, the greps of the new unit tests' names
  and of `no_account_yet` and `poll_check`, which compilation and the test counts already guarantee,
  are dropped, and so is that of `TimeDelta::minutes(10)`, whose behavior decision 8's `wait_end`
  assertions test, and the greps that nothing names 048a-5e or 048a-5f, or what decisions 4, 8 and
  13 remove, no longer ask for a count at the start;
- decision 10's case gains a thirteenth step, a retry of the sign-in that the restart ended,
  refused and recorded, since no case or test sent a retry after a restart (the same readiness
  review);
- after a read of this refresh against 048a-5d's round-1 build in progress: decision 4 says what
  the passing branch becomes where the base ends and records the hook part before it branches, as
  that build does, and "What to recheck at dispatch" asks whether it does; decision 10's step 11
  asserts that the restart ends the waiting sign-in and its hook part unknown; "Done when" gives
  the count of `signInLogin`'s marks against `HEAD`'s file; and the Size line counts three changed
  unit tests.

**What it builds.** The quotes write the `_meta` prefix that `spec/names.yaml` sets as `META/`, and
give PND-010, PND-011, PND-013 and IFC-010 as order 207 writes them. IFC-049's `then`: "With that
capability tools/call may return InputRequiredResult with resultType input_required and exactly one
inputRequests entry: elicitation/create with mode url, message and url. ... For sign-in the engine
writes the message: it names the instance, its Type and package, its node, the URL's host, the
expected account and what the person brings back, if anything; for device it says the code is in
the engine's panel. For carried: false it says that nothing comes back through the Interface, and
that the person completes the login on that node's machine, at it or over a connection Conduit
manages, by remote desktop or by SSH, pasting the code or forwarding the port. No text from the hook
is relayed. No message holds a code or fingerprint." Its `observable`: "Each retry consumes its
state; another input-required response issues a fresh state for that same invocation. For sign-in,
accept means consent, not completion: retries poll under PND-010, blocking or returning another
InputRequiredResult until the account appears or the wait ends", and "`META/signInLogin` ... In
each sign-in.v1 InputRequiredResult and its final result it holds {name, return, node,
acceptsReturn} for the relayed login: node names the instance's node as object.v1 names an object;
acceptsReturn is true exactly where PND-013's first check passes. ... It is in the result's own
_meta, never in its input request, which holds only mode, message and url." IFC-010's `then`: "From
the sign-in-hook.v1 part, it relays the request under PND-010 and keeps the part and sign-in.v1 in
progress when the request passes IFC-049's checks", "for a plain call and for a call that carries
Conduit's keys". PND-013's first check: "its sign-in-hook.v1 carries Conduit's keys, its
implementation not marked `plain` or its item's `keyed` listing sign-in-hook.v1 (IFC-007,
PKG-020), else not-accepted"; its `then`: "A retry that declines or cancels ends failed under
PND-010 and passes on no value." PND-010's `observable`: "Each retry polls account-hook.v1 to check
the account after the person's sign-in, under PND-011, until an account appears or DL18 ends; no
account yet is not failed, and a poll that failed or ended unknown counts as no account yet, so that
only a later check that reports the expected account completes sign-in.v1. The same invocation may
answer another InputRequiredResult or block. Decline or cancel received on a retry ends failed."
PND-011's `then`: "During polling after its URL elicitation, the expected account succeeds with
output account and answers the action signed-in; the account that the check before
sign-in-hook.v1 found counts as no account yet", and "It counts as no account before
sign-in-hook.v1 and in setup, and as no account yet during polling; no account yet during polling
follows PND-010." Its `observable`: "Only a later account-hook.v1 part within sign-in.v1 or setup
that reports the expected account or none clears that condition". PND-004: a URL to open "is held
only until its expiry or the action's answer", and "only IFC-049 relays it, never tool content,
structuredContent or records". DL18: the wait for a person's completion, "including polling across
retries", is finite. No promise changes: the texts state all of it, and this order builds it.

**The stand on requestState.** As order 048b-relay1 settled it, the shell issues, keeps and checks
the requestState it relays, and the core holds none of it: a `sign-in.v1` that reaches the core as
an `invoke` is a first request, and one that reaches it as `respond` is a retry the shell has
accepted and consumed. The implementation's own requestState, from the hook's
`InputRequiredResult`, is another value, which 048a-5d keeps in the hook part's `hook_input`; this
order keeps the part in progress and with it that value, and sends no retry of the hook.

**What the orders before it bring, which this order assumes.** Orders 048a-2 to 048a-5c, 048a-6,
048b-relay1 and 048b-relay2 have merged, each with its Record; order 207 must have merged, and
the work of orders 209, 048a-5d and 048a-5d2 must be on the base this order starts from, each
merged to `main` or on its round-1 branch ("When this order starts"). It assumes, as their
decisions and the amendments their Records accept state them, and as they are built on your base;
where a Record, or the build on your base, names a function, member, phase or case otherwise, use
that name, and say so:

- **(a) From 048a-2 and 048a-5:** `engine/crates/core/src/sign_in.rs` with `expected_account`
  (`NoSignIn`, `NotString { pointer }`, `Set { pointer, account }`) and the shared Type lookup
  `inventory::instance_type(o, c)`; the case `requests-sign-in-lapse-and-conflict`, its three
  files (the Type `signed-agent.v1`, whose logins are `browser`, a redirect login on `127.0.0.1`,
  ports 8400 and 8401, path `/callback`, `carried: true`, and `device`, whose `authorization` is
  `https://example.org/device`; the implementation `signed-server`, not marked `plain`; the Agent
  `signed`, account `primary-account`), its definitions ROOT (`00000000-0000-4000-8000-000000000005`),
  `META/x`, "Root asks T with A", "a completion" and REFUSED(R, RULE), and its steps 1 to 3 and 7 to
  9; 048a-5's "Root asks T with A and M", S (the arguments
  `{"instance": {"kind": "Agent", "name": "signed"}}`), URL (the member
  `"clientCapabilities": {"elicitation": {"url": {}}}`) and LOGIN(X) (the member
  `"signInLogin": X`).
- **(b) From 048a-3 and 048a-4:** the case `requests-sign-in-hold`, its three files (048a-2's Type
  and logins under the package `example.org/sign-in-hold`) and its steps 1 to 9, through LAPSE,
  which raises `signed`'s sign-in action and begins its hold; SUCCESS (048a-3, decision 6);
  ACCOUNT(X), PART(H) and `K(x)` (048a-4, decision 8); `clear_sign_in_undone` and its decision
  `clear-sign-in-undone`.
- **(c) From 048a-5b:** `begin`; `first_check` (`Expected`, `NoAccount`, `Other`, `NotChecked`);
  `part_answered`, called "the answer" below; `signed_in`; `answer_and_lift`; the constants
  `ACCOUNT_NOT_STRING` and `RETIRED_DURING_CHECK`; the case `requests-sign-in-account-first`, its
  SIGN(N) (capturing `$signN` and `$checkN`), FAILED(N, RULE), whose `_meta` holds
  `"META/unanswered": 2`, and INSPECT; and that `requests::cancel` of a `sign-in.v1` leaves its part
  in progress (its Record's V7, open).
- **(d) From 048b-relay1:** `pub struct InputRequired { invocation, attempt, request, meta, expires }`
  in `lib.rs`, whose doc comment says "No step gives one until order 048a-5e."; `Transition`'s
  `input_required`, serialized only when not empty; `Input::Respond { evidence, invocation,
  attempt, identity, responses, meta }`, whose arm calls `requests::respond(t, invocation, attempt,
  c)`, which pushes the decision `refuse-input-response` and answers `-32602` with a text ending
  "no request in this build asks for input yet (order 048a-5e)", tested by
  `respond_is_refused_until_input_requests_are_built`; and the shell, which relays an entry as an
  `InputRequiredResult` whose one input request is under the key `url`, with the entry's `meta` as
  the result's `_meta` and a requestState expiring at `expires`, refusing an entry whose `params`
  hold any key but `mode`, `message` and `url` or whose URL is not https, and passes an accepted
  retry to the core as `respond`, with the retry's `inputResponses` as `responses`.
- **(e) From 048b-relay2:** each call to an instance whose Type states `signIn` declares URL
  elicitation (IFC-045); `retry_intent`, which nothing calls.
- **(f) From order 207:** the texts quoted above, in `spec/`.
- **(g) From order 209:** `dispatch_hook` setting each hook part's `plain` per call (`true` only
  where the item is marked `plain` and does not list the hook under `keyed`); for a keyed hook of a
  plain item, discovery first; a check that ends `failed` or `unknown` counted as no account; and
  the section "Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)" in
  `engine/CONFORMANCE.md`, whose third bullet ends "Polling, where a failed or unknown poll counts
  as no account yet, is order 048a-5f's."
- **(h) From 048a-5d:** the part function `setup_sign_in_hook(t, a, c, parent, object, name,
  promises, login)` and `dispatch_hook`, with the asked-for login as their last parameter; in
  `part_answered`, after a first check that finds no account or another, or ends failed or unknown,
  a `sign-in-hook.v1` part named `sign-in` and the phase `sign-in-pending`; its arm for
  `sign-in-pending`, before the test of `account-pending`, which ends `sign-in.v1` by the hook's
  outcome; `sign_in_hook_part`; in `complete`, the arm that sends a `sign-in-hook.v1` part's
  `input_required` result to `sign_in::hook_input`; `hook_input`'s steps 1 to 7, the passing branch
  being step 5 (`keep_hook_input`, then the interim ending: the part ended `unknown` with the
  cause "The hook asked for input that sign-in.v1 did not relay, so whether the login it began is
  still waiting is not known.", recorded as the part `sign-in`, the decision
  `end-sign-in-unrelayed` and `sign-in.v1` ended failed under PND-010 with `HOOK_UNRELAYED_CAUSE`);
  `hook_url_action`, `hook_answer_cause` and `keep_hook_input`; the decisions
  `refuse-sign-in-hook-answer` and `drop-sign-in-hook-answer`; the end renamed
  `end_sign_in(t, a, c, i, outcome, rule, cause)`; `finish` removing `hook_input`; the unit tests
  `sign_in_hook_answer_form`, `sign_in_hook_input_kept_nowhere`,
  `sign_in_hook_not_dispatched_while_retiring` and `sign_in_hook_call_carries_asked_login`; the
  cases `requests-sign-in-hook-part` (25 steps, with URLACTION, HOOK(M) and ENDED(O, RULE, I)) and
  `requests-sign-in-plain-hook-part` (8 steps, with PLAIN-SIGN(N) and PLAIN-HOOK); its steps
  inserted in `requests-sign-in-account-first` (36 steps), `requests-keyed-hook-dispatch` (26
  steps, its steps 14 and 26 completing a plain hook with URLACTION) and
  `requests-sign-in-setup-check-failed-hook` (17 steps, its step 16 completing a keyed hook,
  captured as `$signhook1`, with URLACTION); the section's last sentence, which order 209 words "...
  has no case here: this build runs no `sign-in-hook.v1` part; ...", made to read "... has no case
  in this section: the hook part and the relay are orders 048a-5d and 048a-5e, whose cases show
  it."; `retry_intent` marked `#[allow(dead_code)] // Order 048a-7 calls it.`; and its DL18 text,
  whose sentence "Until order 048a-5e builds the relay, a URL elicitation ends `sign-in.v1` failed
  with a diagnostic naming PND-010 and that order, ..." ends with "This is an unbuilt step, not a
  reading of PND-010."
- **(i) From 048a-5d2:** `url_match(url, logins, asked)`, giving the name of the one stated login
  the URL matches, and `url_match_cause`; in `hook_input`, after the form's check and before the
  passing branch, the URL (the string `params.url` of the input request under the key `K` that
  `hook_url_action` gave) matched against `stated_logins` of the instance and S's `asked_login`, a
  failing URL ending the part `unknown` and `sign-in.v1` failed under IFC-049, naming the check, the
  field and the login it concerns, and holding no URL value; the passing branch reached only with
  the matched login's name `L`, which the decision `end-sign-in-unrelayed` holds in its facts
  `{"invocation": S, "hook": H, "login": L}`; the unit tests `sign_in_url_match` and
  `sign_in_url_mismatch_kept_nowhere`, and `sign_in_hook_input_kept_nowhere`'s `s1` given a
  `type` and `held` whose Type states the fixtures' two logins; the cases
  `requests-sign-in-url-match` (24 steps, on 048a-2's fixture, whose steps 9 and 18 complete a hook
  with a URL that matches and expect the interim ending, each followed by a new `sign-in.v1`) and
  `requests-sign-in-plain-url-match` (8 steps, on order 207's plain manifest, whose step 5 does so,
  followed by a new `sign-in.v1`); and in DL18's row, 048a-5d's "unrelayed and, until order
  048a-5d2, with its URL unchecked," become "unrelayed, once its URL has matched a stated login
  (order 048a-5d2),", with a sentence of its own after the one on a hook answer that is not one URL
  elicitation.

Where one of them merged, or was built on your base, otherwise than this says, apply the nearest
sound reading of each decision below to what you find, and say so; but where the start check below
prints otherwise and does not say that a difference may be used and reported, stop.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`c097ec81`, before orders 207, 209, 048a-5d and 048a-5d2, which move several; find each place by
the text quoted with it.

- `week_after` (`engine/crates/core/src/pending.rs:9`) adds seven days to a step's time with
  `DateTime::parse_from_rfc3339`, `checked_add_signed(TimeDelta::days(7))`, `to_utc()` and
  `to_rfc3339_opts(SecondsFormat::AutoSi, true)`.
- `inventory::object` (`inventory.rs:7`) gives `{"kind", "name", "id"}`, the form of `object.v1`;
  `access::object` (`access.rs:57`) finds an object by a reference or an ID; an instance's
  `placement` names its node, and first start makes the node `local` from its second supplied ID,
  `00000000-0000-4000-8000-000000000002` in these cases (`lib.rs:981`).
- `inventory::instance_type` (`inventory.rs:316`) finds an instance's Type through
  `registration::publication_object` and `schemas::operation`, which gives the environment, the
  key of the scope that holds the Type, and the Type's document; a `Scope` (`schemas.rs:11`) has
  `address` and `version: Option<String>`. It has three callers.
- `access::established(state, identity)` (`access.rs:309`) gives the instance an `Identity`
  establishes, or an error.
- `mark` (`requests.rs:24`) writes a `_meta` key as `format!("{}{key}", c.meta_prefix)`; it is
  private, so `sign_in.rs` writes a key the same way.
- `end` (`requests.rs:568`) writes `request-ended` from a route, an outcome or a reason, and an
  invocation ID; `reject_diagnostics` adds `ruleId` to the route it records.
- `respond` (`requests.rs:750`), its test (line 2628); `retry_intent` (line 865).
- `finish` (`requests.rs:1857`) sets `t.reply` through `envelope`, adds `META/pendingAction` where
  the invocation has `notices`, then pushes the reply to `t.request_replies` for the invocation's
  `(id, attempt)`, then writes `invocation-ended` and `request-ended`.
- `step_owned` (`lib.rs:361`) marks a reply's unanswered actions only for the inputs that
  `reply_context` (`lib.rs:428`) lists; `Input::Complete` is listed with no caller, and
  `Input::Respond` is not listed.
- `Shell::commit` (`engine/crates/shell/src/lib.rs`) registers a waiter at the invocation and its
  attempt for a step whose reply is `{"pending": true, "invocation": I, ...}`, and gives each
  `input_required` entry to the waiters of its `(invocation, attempt)` as `{"inputRequired": V}`.
- The conformance runner serializes a core step's whole `Transition` and projects every
  expectation member that `expect::check` does not handle onto it, objects as subsets and arrays
  of the same length, so a member `input_required` in a case's `expect` is checked with no runner
  code; `record_facts` is an ordered subsequence. The case schema refuses an `expect` member or an
  input kind that no piece under `conformance/case-schema/` names; `portable.rs`'s `check_case`
  lists the input kinds a portable case may use, and `checks.rs`'s `reply` the kinds whose replies
  it checks for a reason's diagnostic. Neither lists `respond`, and no piece names
  `input_required` or `respond`.

**When this order starts.** Order 207 must have merged to `main`. The work of orders 209,
048a-5d and 048a-5d2 must be on your base, the commit your working tree starts from (`HEAD`), in
one of two ways, either of which serves: each has merged to `main`; or the coordinator dispatched
this order stacked on 048a-5d2's round-1 branch, whose tip holds 048a-5d2's build, committed,
before its read and merge, on top of 048a-5d's build and order 209's, each merged or on its own
round-1 branch below it. You tell them apart by the Records, which the admin writes at each merge:
until then the files of orders 209 and 048a-5d2 end "Not yet run.", and 048a-5d's has no
`## Record` heading (the first check's two lines). On any base, the three orders' cases are
present and pass (the second check, items 7, 9 to 11 and 13, and the start's runner runs below).
Report which of the three have merged on your base, and `HEAD`'s commit. Nothing below reads which
it is. On a round-1 branch, the copies of this order's file and of its predecessors' files in your
tree may be older than the texts that merged to `main`; where they differ, the texts on `main`
govern: read them with `git show origin/main:docs/work-orders/FILE`, `FILE` being the file's name.
Before you change anything, check from the repository root that you find what they give,
and stop, change nothing and report if any check prints otherwise, except where its item below says
that you use what you find and report it:

```bash
grep -c '^Not yet run\.$' docs/work-orders/207-keyed-hooks-and-sign-in-spec-fixes.md docs/work-orders/209-build-keyed-hooks-and-sign-in-fixes.md docs/work-orders/048a5d2-build-sign-in-url-match.md
grep -c '^## Record' docs/work-orders/048a5d-build-sign-in-hook-part.md
ls -d conformance/conventions/requests/keyed-hook-dispatch conformance/conventions/requests/keyed-not-in-implements conformance/conventions/requests/sign-in-setup-check-failed-hook conformance/conventions/requests/sign-in-hook-part conformance/conventions/requests/sign-in-plain-hook-part conformance/conventions/requests/sign-in-url-match conformance/conventions/requests/sign-in-plain-url-match 2>/dev/null | wc -l | tr -d ' '
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'From the sign-in-hook.v1 part, it relays the request under PND-010 and keeps the part and sign-in.v1 in progress when the request passes IFC-049.s checks.'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'In each sign-in.v1 InputRequiredResult and its final result it holds {name, return, node, acceptsReturn} for the relayed login'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'a poll that failed or ended unknown counts as no account yet, so that only a later check that reports the expected account completes sign-in.v1'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'and as no account yet during polling; no account yet during polling follows PND-010'
tr -s ' \n' '  ' < spec/promises/pending.yaml | grep -c 'its implementation not marked .plain. or its item.s .keyed. listing sign-in-hook.v1'
grep -c 'pub(crate) fn hook_input(' engine/crates/core/src/sign_in.rs
grep -c 'fn keep_hook_input' engine/crates/core/src/sign_in.rs
grep -c '^fn end_sign_in($' engine/crates/core/src/sign_in.rs
grep -c 'HOOK_UNRELAYED_CAUSE' engine/crates/core/src/sign_in.rs
grep -rl '"end-sign-in-unrelayed"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"drop-sign-in-hook-answer"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'remove("hook_input")' engine/crates/core/src/requests.rs
grep -c 'fn sign_in_hook_input_kept_nowhere' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn url_match(' engine/crates/core/src/sign_in.rs
grep -c 'fn sign_in_url_mismatch_kept_nowhere' engine/crates/core/src/sign_in.rs
grep -c '^pub struct InputRequired {$' engine/crates/core/src/lib.rs
grep -c 'No step gives one until order 048a-5e.' engine/crates/core/src/lib.rs
grep -c 'fn respond_is_refused_until_input_requests_are_built' engine/crates/core/src/requests.rs
grep -rn 'input_required.push(' engine/crates/core/src | wc -l | tr -d ' '
test -f engine/crates/shell/src/server/relay.rs; echo "relay $?"
grep -rlE '"retry-pending"|"poll-pending"|sign_in_wait' engine/crates/core/src | wc -l | tr -d ' '
grep -c '"respond"' engine/crates/conformance/src/portable.rs engine/crates/conformance/src/checks.rs
ls conformance/case-schema/inputs/respond.json conformance/case-schema/expect/input_required.json 2>/dev/null | wc -l | tr -d ' '
ls -d conformance/conventions/requests/sign-in-url-relayed conformance/conventions/requests/sign-in-polling 2>/dev/null | wc -l | tr -d ' '
python3 -c 'import json; a = json.load(open("conformance/conventions/requests/sign-in-url-match/case.yaml"))["steps"]; b = json.load(open("conformance/conventions/requests/sign-in-plain-url-match/case.yaml"))["steps"]; print(len(a), a[8]["input"]["invocation"], a[9]["input"]["request"]["tool"], a[17]["input"]["invocation"], a[18]["input"]["request"]["tool"], len(b), b[4]["input"]["invocation"], b[5]["input"]["request"]["tool"])'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Until order 048a-5e builds the relay, a URL elicitation ends .sign-in.v1. failed with a diagnostic naming PND-010 and that order, unrelayed, once its URL has matched a stated login (order 048a-5d2),'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'whose URL elicitation, until order 048a-5e, ends .sign-in.v1. failed under PND-010.s interim ending'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Polling, where a failed or unknown poll counts as no account yet, is order 048a-5f.s.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'the hook part and the relay are orders 048a-5d and 048a-5e, whose cases show it.'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'Order 048a-5d takes that request into .sign-in.v1., not as IFC-016.s lapse trigger (.requests-sign-in-plain-hook-part.).'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'runs as far as its .sign-in-hook.v1. part, whose URL elicitation it does not yet relay (orders 048a-5b, 048a-5d and 048a-5e;'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'signs an instance in only where its first account check finds the expected account, and where that check finds no account or another or ends failed or unknown dispatches a .sign-in-hook.v1. part, whose URL elicitation it does not yet relay (orders 048a-5d and 048a-5e; row DL18'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'which is how this build reads IFC-049.s "There is no elicitationId".'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-account-first/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (17, 18, 22, 23, 25, 26, 32, 33)], [s[k]["input"]["request"]["tool"] for k in (21, 24, 27)], len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/keyed-hook-dispatch/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (12, 13, 24, 25)], s[14]["input"]["request"]["tool"], len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-setup-check-failed-hook/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (14, 15)], s[16]["input"]["request"]["tool"], len(s))'
python3 -c 'import json; a = json.load(open("conformance/conventions/requests/sign-in-hook-part/case.yaml"))["steps"]; b = json.load(open("conformance/conventions/requests/sign-in-plain-hook-part/case.yaml"))["steps"]; print(len(a), a[9]["input"]["invocation"], a[10]["input"]["request"]["tool"], a[24]["input"]["invocation"], len(b), b[4]["input"]["invocation"], b[5]["input"]["request"]["tool"])'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-lapse-and-conflict/case.yaml"))["steps"]; print([x["input"].get("request", {}).get("tool", x["input"]["kind"]) for x in s[:3] + s[6:9]], "account0" in json.dumps(s[7]))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-hold/case.yaml"))["steps"]; print([x["input"].get("request", {}).get("tool", x["input"]["kind"]) for x in s[:9]])'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-account-first/case.yaml"))["steps"]; print(s[25]["input"]["invocation"], s[25]["input"]["completion"]["result"]["isError"])'
grep -rlE '048a-5e([^f]|$)|048a-5f' engine/crates/core/src engine/crates/shell/src | sort
```

They print, in order:

1. three lines and a number: `207-keyed-hooks-and-sign-in-spec-fixes.md:0`, order 207's Record;
   `209-build-keyed-hooks-and-sign-in-fixes.md:0` where order 209 has merged, or `:1` on its
   round-1 branch; `048a5d2-build-sign-in-url-match.md:0` where 048a-5d2 has merged, or `:1` on
   its round-1 branch; and `1` where 048a-5d has merged, or `0` on its round-1 branch; report
   which; where a file is missing, or an order shows as merged while one before it does not,
   stop;
2. `7`: the cases of order 209 (three), 048a-5d (two) and 048a-5d2 (two), on your base either way;
3. `1` five times: order 207's texts that this order builds on, IFC-010's relay from the hook
   part, IFC-049's `signInLogin`, PND-010's failed or unknown poll, PND-011's no account yet while
   polling, and PND-013's first check per hook;
4. `1`, `1`, `1`, a number other than `0`, `1`, `1`, `1` and `1`, then `1` and `1`: 048a-5d's
   `hook_input`, `keep_hook_input`, `end_sign_in`, its constant, its two decisions, `finish`
   removing `hook_input` and its unit test; then 048a-5d2's match and its unit test. Where one of
   these prints `0` because the build on your base names it otherwise, find it by what assumption
   (h) or (i) says it is or does, use that name in the decisions below and report it; stop only
   where none is found;
5. `1`, `1`, `1`, `0` and `relay 0`: 048b-relay1's output, its doc comment, its test, no step that
   gives an output yet, and its shell; where the doc comment's line prints `0` because a
   predecessor reworded it, decision 6 replaces the sentence of that comment that says no step
   gives one yet, as you find it, and you report it;
6. `0`, then two lines ending `:0`, then `0` and `0`: none of this order's work;
7. `24 $hook1 group.sign-in.v1 $hook4 group.sign-in.v1 8 $hook1 group.sign-in.v1`: 048a-5d2's two
   cases, with the steps decision 12 changes and the `sign-in.v1` that follows each;
8. `1` eight times: each place of `engine/CONFORMANCE.md` that decision 13 changes, as orders 209,
   048a-5d and 048a-5d2 leave it. Where one prints `0` because a predecessor's build or merge
   words it otherwise (for example naming another order than 048a-5e), find the place by its
   sense, replace it as decision 13 says, and report it before and after, whole; stop only where
   the place is missing;
9. `['$check1', '$hook1', '$check2', '$hook2', '$check3', '$hook3', '$check5', '$hook5'] ['group.sign-in.v1', 'group.sign-in.v1', 'group.sign-in.v1'] 36`:
   048a-5b's case as 048a-5d leaves it, its three steps after a relayed URL that start a sign-in;
10. `['$check1', '$hook1', '$check4', '$hook2'] group.sign-in.v1 26` and
    `['$check1', '$signhook1'] group.inspect.v1 17`: order 209's first and third cases as 048a-5d
    leaves them;
11. `25 $hook1 group.sign-in.v1 $hook6 8 $hook1 group.sign-in.v1`: 048a-5d's two cases;
12. `['first-start', 'register-project', 'group.set-up.v1', 'setup-report', 'setup-report', 'complete'] True`
    and
    `['first-start', 'register-project', 'group.set-up.v1', 'setup-report', 'setup-report', 'complete', 'access.open.v1', 'to-signed.work.v1', 'complete']`:
    the steps decisions 10 and 11 copy;
13. `$check3 True`: the failed account check that decision 11 copies (step 26 as 048a-5d leaves it);
14. the sources that name order 048a-5e or 048a-5f, which "Done when" asks to name none of them
    (048b-relay1's two in `lib.rs` and `requests.rs`, and 048a-5d's cause in `sign_in.rs`, as their
    texts write them); report the list, whatever it holds: no stop.

Then name in your report, with their files and lines as you find them: the part function,
`hook_input` and its passing branch, 048a-5d2's match and what it gives the passing branch (an item
or a name), `part_answered` and its `sign-in-pending` arm, `signed_in`, `end_sign_in`, `finish`,
`respond` and `reply_context`. Still before any change, keep the rows this order changes, and from
`engine/`, one at a time, run these and keep each count and last line: "Done when" compares with
them.

```bash
mkdir -p engine/target && grep -E '^\| (PND-004|PND-010|PND-011|PND-013|IFC-049) \|' engine/STANDING.md > engine/target/standing-048a5ef-before.txt
```

```bash
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-conformance --lib -- portable
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
```

Each passes. `requests-` runs the seven cases of orders 209, 048a-5d and 048a-5d2 among the
others, and each must pass and none be an expected failure; where one fails, stop and report it,
since on a round-1 branch a failing case of a predecessor's is that order's to fix, not this
order's.

Orders that land beside this one (050n, 051b and others): whichever lands second keeps both changes
and writes `engine/STANDING.md` again with the tool.

**What to recheck at dispatch.** For the coordinator, before this order dispatches, against the
merged texts of 207, 209, 048a-5d and 048a-5d2 and against the base it starts from: for each of
209, 048a-5d and 048a-5d2, its Record where it has merged, or else its round-1 build and the
verifier's and reader's findings on it so far:

- every quote of order 207's texts above, as merged, and the start check's spec lines;
- 048a-5d2's Record or round-1 build, against its text as assumption (i) gives it: `url_match`
  and the matched login's name in the passing branch (decision 4), its cases' lengths and the
  steps decision 12 changes, `sign_in_url_mismatch_kept_nowhere`'s copies and the state it gave
  `sign_in_hook_input_kept_nowhere` (decision 8), and its words in DL18's row and in the section
  of orders 207 and 209 (decision 13);
- 048a-5d's Record or round-1 build: any amendment to, or departure in, `hook_input`'s steps, the
  part's ending, whether `hook_input` ends the part before its branches (decision 4), `end_sign_in`,
  `HOOK_UNRELAYED_CAUSE`, its cases' lengths, the indexes of their URL steps and their capture
  names, and its unit test's state (decision 8 changes its assertions);
- order 209's Record or round-1 build: whether a keyed account hook of a plain item, with its
  discovery, changes the reply a poll's dispatch gives, and its section's wording in
  `engine/CONFORMANCE.md`;
- 048a-5gh's text, written ahead beside this one from the drafts: that its assumption (e) and its
  start check take this order's names (WAITING is `retry-pending`; the poll arm's `poll-pending`
  and SI's `poll`; `no_account_yet` and `poll_check`; `end_hook_wait(t, a, c, i, why)`, also called
  inside `end_sign_in` and `signed_in`, its cause capitalized and its rule PND-010; `sign_in_wait`'s
  members `attempt`, `expires`, `login`, `plain`, `meta` and, until S ends, `url`; the interim
  ending for another account, its decision and its cause naming 048a-5gh), that its case F is
  `requests-sign-in-polling` with its roles at steps 10, 11, 12, 13 and 16, that its edits of
  `engine/CONFORMANCE.md` replace this order's DL18 sentence beginning "Until order 048a-5gh,"
  rather than a paragraph on polling, which this order does not add, and that it changes the
  polling case's step 32, which this order ends with that interim ending;
- 048b-join's assumption A4 against decisions 4 to 7;
- the line numbers of "What the engine already does", and any order merged since `c097ec81` that
  touches `sign_in.rs`, `respond`, `finish`, `reply_context`, `portable.rs`'s or `checks.rs`'s
  lists, or `conformance/README.md`'s section "Assertions".

**Who decided what.** IFC-049 decides what the output holds, what the message names for each kind
of login, that no text from the hook is relayed, what `signInLogin` holds in each input-required
result and the final result, and that each retry consumes its state and another input-required
response gets a fresh one; IFC-010, as order 207 writes it, that the hook part's URL request,
plain or keyed, is relayed and keeps the part and `sign-in.v1` in progress; PND-013's first check
decides `acceptsReturn`, and PND-013 that a decline or cancel ends failed under PND-010 and passes
on no value; PND-010 and PND-011 that each retry polls the account, that the expected account signs
in, that no account, the first check's account and a poll that failed or ended unknown count as no
account yet, and that a check reporting the expected account or none clears `SignInUndone`; DL18
that the wait is finite and spans the retries; PND-004 that the URL is held only until its expiry
or the action's answer and appears in no record. Ruling 88 decides that this is one order, that the
carried return is not on its path, and that FR-264's one behavior per order is waived for it.
Everything else here is the coordinator's, each open to the owner's strike ([ruling
48](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-48/README.md)): the two refreshes' changes listed above; DL18's wait of ten
minutes from the step that first relays the URL, kept by every later output of that invocation; the
member `sign_in_wait` and its form, which holds the URL until the invocation ends; the members
`first_account` and `poll`; the functions `no_account_yet` and `poll_check`; the phases
`retry-pending` and `poll-pending`; the message's words for each kind of login, and giving a
redirect or page login whose return is not taken the text IFC-049 gives `carried: false`; naming the
package by its address and, where it has one, its version; the node's form, `inventory::object`; the
one Type lookup's new function; reading the person's answer only as the one response under the key
`url`, with an action of exactly `accept`, `decline` or `cancel`, and ending `sign-in.v1` failed
under IFC-049 on any other response, since the shell has consumed the state; checking in the core
that a retry comes from the invocation's `sender` at its attempt while it waits, and refusing and
recording any other; ending `sign-in.v1` failed under PND-003 on a retry for an instance gone or
retired; reading no `meta` of a retry; dispatching the poll through the part function as 048a-5b
dispatches its first check; ending a poll that states a reason, or whose account cannot be read,
under PND-011; the interim ending for another account until 048a-5gh, raising nothing; ending the
waiting hook part unknown under PND-010 whenever `sign-in.v1` ends, and its cause; dropping a second
input request of a relayed hook part, and its cause; the final result's `signInLogin`, added in
`finish`; marking a `respond` reply's unanswered actions; recording a refused retry; the decisions'
names and facts; the diagnostics' words; the function names; the unit tests; the runner's input kind
and the expectation piece, and the README's sentence; the two cases, their routes, IDs and times;
the relayed steps' expectations and the declining retries inserted after them; and the text in
`engine/CONFORMANCE.md`. Apply them as written and report them under `decisions` as the
coordinator's.

**Read first:**

1. In `spec/`: IFC-010, IFC-016 (its sentences on relaying), IFC-049 and IFC-050
   (`spec/interface/implementation.yaml`); PND-004, PND-005, PND-010, PND-011 and PND-013
   (`spec/promises/pending.yaml`); REC-001 (`spec/conventions/records.yaml`); `sign-in.v1`,
   `sign-in-hook.v1` and `account-hook.v1` (`spec/core/operations.yaml`); DL6, DL18 and DL19
   (`spec/implementation-defined.yaml`).
2. The orders 048a-5d and 048a-5d2 (their files `048a5d-build-sign-in-hook-part.md` and
   `048a5d2-build-sign-in-url-match.md`), 209 (`209-build-keyed-hooks-and-sign-in-fixes.md`) and
   048b-relay1 (`048b-relay1-build-input-required-relay.md`), whole, with their Records where they
   have merged; order 207, its
   decisions 5 to 9 and its Record; order 048a-5b, its decisions on `part_answered`, `signed_in`
   and the end, and its case's definitions; order 048a-4's decisions 2 and 8.
3. The cases `requests-sign-in-lapse-and-conflict`, `requests-sign-in-hold`,
   `requests-sign-in-account-first`, `requests-sign-in-hook-part`,
   `requests-sign-in-plain-hook-part`, `requests-keyed-hook-dispatch`,
   `requests-sign-in-url-match` and `requests-sign-in-plain-url-match`.
4. `conformance/README.md`, its sections "Assertions", "Portable cases" and "The coverage index
   and the case schema in pieces"; `engine/crates/conformance/src/portable.rs`'s `check_case` and
   its test, and `checks.rs`'s `reply`.

**Size: L**, two M orders joined under ruling 88: about 0.6 to 0.8k changed lines of Rust with two
new unit tests and three changed; one input kind and one expectation piece for the runner; two new
cases, of 13 steps on 048a-2's fixture and 36 on 048a-3's; the relayed steps of seven earlier cases,
with nine declining retries inserted in them; five places in `engine/CONFORMANCE.md` and one
sentence in `conformance/README.md`.

## The decisions it applies

Below, `Q(x)` is `c.qualified(x)`, S is the `sign-in.v1` invocation and SI its member `sign_in`.

1. **The pure parts, in `sign_in.rs`.** Add these, each `pub(crate)`:
   - `fn accepts_return(login: &Value, plain: bool) -> bool`: PND-013's first check. True exactly
     where `login["return"]` is `"redirect"` or `"page"`, `login["carried"]` is `true`, and `plain`
     is false, `plain` being the hook call's own (order 209: a keyed hook of a plain item is not
     plain).
   - `fn wait_end(time: &str) -> Option<String>`: DL18. The time plus ten minutes, written as
     `week_after` writes its instant, with `TimeDelta::minutes(10)` in place of
     `TimeDelta::days(7)`; `None` where the time does not parse.
   - `fn relay_request(message: &str, url: &str) -> Value`: exactly
     `{"method": "elicitation/create", "params": {"mode": "url", "message": message, "url": url}}`.
   - `fn login_meta(login: &Value, node: &Value, accepts: bool) -> Value`: exactly
     `{"name": login["name"], "return": login["return"], "node": node, "acceptsReturn": accepts}`.
   - `struct RelayFacts<'a> { kind: &'a str, name: &'a str, type_name: &'a str, package: &'a str, node: &'a str, host: &'a str, account: &'a str, login: &'a Value, accepts: bool }`
     and `fn relay_message(f: &RelayFacts<'_>) -> String`, the engine's message: the lead, a space,
     then one ending. The lead is
     `Sign in the {kind} {name} (Type {type_name}, package {package}) on the node {node} at {host}, as the account {account}.`
     The ending, by the login's `return`, `{rhost}` and `{rpath}` being its `redirect.host` and
     `redirect.path`, and `{page}` its `page`:
     - `device`:
       `The code to enter at {host} is shown in the engine's panel, never here; nothing comes back through this Interface.`
     - `service`:
       `The service's own page {page} receives the login's return, so nothing comes back through this Interface.`
     - `redirect` where `accepts`:
       `After you sign in, the browser is sent to an address on {rhost} with the path {rpath}. Where the browser runs on the node {node}'s machine, the login completes there; elsewhere, that address is what you bring back, and your Interface carries it or asks you to paste it.`
     - `page` where `accepts`:
       `After you sign in, the page {page} shows a code: that code is what you bring back, and you paste it into your Interface.`
     - `redirect` or `page` where not `accepts`:
       `Nothing comes back through this Interface: complete the login on the node {node}'s machine, at it or over a connection Conduit manages, by remote desktop or by SSH, pasting the code or forwarding the port.`

     No text from the hook, no URL, query, code, state or fingerprint enters the message.
   - The person's answer and the poll's reading, with these bodies (`cargo fmt` may rewrap them;
     keep `found.eq_ignore_ascii_case(first)` and `matches!(outcome, "failed" | "unknown")` as
     written, since "Done when" changes them):

     ```rust
     /// IFC-049: the person's answer to the one URL input request, under its key `url` in the
     /// retry's inputResponses; None for anything else.
     pub(crate) fn url_action(responses: &Value) -> Option<&'static str> {
         let responses = responses.as_object()?;
         if responses.len() != 1 {
             return None;
         }
         let action = responses.get("url")?.get("action")?.as_str()?;
         ["accept", "decline", "cancel"]
             .into_iter()
             .find(|a| *a == action)
     }

     /// PND-011: while polling, no account, or the account that the check before
     /// sign-in-hook.v1 found, compared ignoring ASCII case, counts as no account yet.
     pub(crate) fn no_account_yet(found: &Value, first: &Value) -> bool {
         match (found.as_str(), first.as_str()) {
             (None, _) => found.is_null(),
             (Some(found), Some(first)) => found.eq_ignore_ascii_case(first),
             (Some(_), None) => false,
         }
     }

     /// PND-010 and PND-011: what a poll while sign-in.v1 waits after its URL action found.
     #[derive(Debug, PartialEq)]
     pub(crate) enum Poll {
         Expected(String),
         NoneYet,
         Other,
         NotKnown,
     }

     /// A poll that failed or ended unknown counts as no account yet (PND-010); one that states
     /// a reason, or whose account cannot be read, is not known.
     pub(crate) fn poll_check(outcome: &str, outputs: &Value, expected: &str, first: &Value) -> Poll {
         if matches!(outcome, "failed" | "unknown") {
             return Poll::NoneYet;
         }
         if outcome != "succeeded" {
             return Poll::NotKnown;
         }
         match outputs.get("account") {
             Some(Value::String(found)) if found.eq_ignore_ascii_case(expected) => {
                 Poll::Expected(found.clone())
             }
             Some(found) if no_account_yet(found, first) => Poll::NoneYet,
             Some(Value::String(_)) => Poll::Other,
             _ => Poll::NotKnown,
         }
     }
     ```

2. **The one Type lookup gives the package.** In `inventory.rs`, add
   `pub(crate) fn instance_type_and_package(o: &Value, c: &Context) -> Result<(Value, String)>`,
   holding the body of `instance_type` and giving also the package: the `address` of the scope
   that `schemas::operation` names as holding the Type, followed by a space and its `version` where
   that scope has one. `instance_type` becomes a call of it that keeps the Type alone, so its
   signature and its three callers do not change. Write no second lookup.

3. **The output, in `sign_in.rs`.** Add
   `pub(crate) fn keep_sign_in_wait(v: &mut Value, login: &str, url: &str, plain: bool)`, which
   sets S's member `sign_in_wait` to `{"login": login, "url": url, "plain": plain}`, replacing any
   earlier one, and is the only code that writes those three members; and
   `pub(crate) fn relay_output(t: &mut Transition, sign_in: &str, time: &str, c: &Context) -> Result<()>`,
   given only S's ID and the step's time, so that a later output of the same S is one more call. It:
   1. finds S in `t.state.invocations` by its ID and its instance `O`,
      `access::object(&t.state, &S["object"])`, and reads `W`, S's `sign_in_wait`: `login` and
      `url`, each a string, and `plain`, a boolean, returning an engine error
      `sign-in wait missing` where any is not;
   2. gets the expected account `A` from `expected_account`; where it gives no `Set`, which the
      checks before any part leave no way to reach, returns an engine error naming PND-010, and you
      report whether any path reaches it;
   3. gets the Type and the package from `instance_type_and_package(O, c)`, the Type's name being
      its `metadata.name`, and `L`, the item of the Type's `spec.signIn.logins` whose `name` is
      `W`'s `login`, returning an engine error where none is;
   4. finds the node, `access::object(&t.state, &O["placement"])`, and `N`, `inventory::object` of
      it, returning an engine error `sign-in node missing` where none is found;
   5. reads the URL's host from `W`'s `url` with `url::Url::parse` and `host_str()`, returning an
      engine error where either fails (048a-5d2's match has already read it);
   6. sets `accepts` to `accepts_return(L, W.plain)`;
   7. sets `X`, the expiry: `W`'s `expires` where it is a string (an output after the first), else
      `wait_end(time)`, returning an engine error where that gives none;
   8. sets `W`'s members `attempt` to S's `attempt`, `expires` to `X` and `meta` to
      `login_meta(L, N, accepts)`, keeping its others;
   9. sets SI's `phase` to `retry-pending`;
   10. pushes onto `t.input_required` the `InputRequired` with `invocation` S's ID, `attempt` S's
       `attempt` as `u64`, `request` `relay_request(M, W.url)`, `M` being `relay_message` of `O`'s
       `kind` and `name`, the Type's name, the package, `N`'s `name`, the host, `A`, `L` and
       `accepts`, `meta` `{K: W.meta}`, `K` being `format!("{}signInLogin", c.meta_prefix)`, and
       `expires` `X`;
   11. pushes one decision named `relay-sign-in-url`, citing `IFC-049`, `IFC-010` and `PND-010`,
       with the implementation choice `DL18`, the facts
       `{"invocation": ID, "attempt": S["attempt"], "login": W["login"], "expires": X}` and no
       affected object;
   12. sets `t.reply` to `{"pending": true, "invocation": ID}`.

   It writes no record, and keeps the message nowhere in the state or in a decision. The URL is
   held only in `W` while S is in progress (PND-004) and in the output, which the shell relays and
   does not store; it enters no record, reply, decision or diagnostic.

4. **The relay replaces the interim ending, and the hook part stays in progress (IFC-010).** In
   `hook_input`:
   - Its passing branch, reached by an answer that passes the form's checks (048a-5d) and the URL's
     match (048a-5d2), from its call of `keep_hook_input` to its return, becomes: call
     `keep_hook_input` on the part as now; call `keep_sign_in_wait` on S with the matched login's
     name, the URL exactly as the hook's answer gives it, and the part's `plain == true`; call
     `relay_output` with S's ID and the step's time; and return. The part is not ended and not
     recorded in SI's `parts`, so it stays `in-progress`, attempt 1, holding `hook_input`; S is not
     ended. Remove the decision `end-sign-in-unrelayed`, the constant `HOOK_UNRELAYED_CAUSE`, and
     anything that only they use. The part's ending cause "The hook asked for input that
     sign-in.v1 did not relay, so whether the login it began is still waiting is not known." stays
     for the branches that end the part on a failing answer or a retired instance. Where the base
     ends the part and records it in SI's `parts` before it branches (048a-5d's round-1 build does:
     `keep_hook_input`, then the part's ending and its record, then the tests of a retired instance
     and a failing form), the part's ending and its record in SI's `parts` move into the branches
     that end it: a retired instance, a failing form and 048a-5d2's failing URL. The passing branch
     then calls `keep_hook_input` without ending the part.
   - Its step 1 splits. Where S's `state` is not `in-progress`, it stays as 048a-5d wrote it. Where
     S is in progress and SI's `phase` is not `sign-in-pending` (`retry-pending` or `poll-pending`:
     the part was already relayed and asks again), take the part's ending with the cause
     `The sign-in hook asked for input again after its URL request was relayed, so nothing was relayed again and sign-in.v1 goes on waiting.`,
     push the decision `drop-sign-in-hook-answer` with the facts `{"invocation": S, "hook": H}` as
     step 1 does, and return, changing nothing of S.

   Nothing else of `hook_input` changes: an answer that fails a check of the form or of the URL
   ends the part `unknown` and S failed under IFC-049, as 048a-5d and 048a-5d2 leave it. Name no
   order in the new code or comments.

5. **What S keeps, and its endings.**
   - In `part_answered`, where the first check leads to the hook part (048a-5d's decision 1, for
     `NoAccount`, `Other`, and `NotChecked` that ended `failed` or `unknown`), set SI's
     `first_account` to the check's `account` from the part's `retained.outputs` where it is a
     string, and otherwise null, before the part function is called.
   - In `sign_in.rs`, add
     `pub(crate) fn end_hook_wait(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize, why: &str) -> Result<()>`,
     `i` being S's index. For each invocation, in the order of `t.state.invocations`, whose
     `setup_parent` is S's ID, whose `operation` is `Q("sign-in-hook.v1")`, whose `state` is
     `in-progress` and which holds `hook_input`, `H` its ID: end it through
     `crate::requests::finish` with the outcome `unknown`, outputs `{}`, content `[]` and the cause
     one diagnostic from `explain::outcome_diagnostic` for it, the outcome `unknown`, the rule
     `PND-010` and the cause
     `The sign-in.v1 invocation it serves ended {why}, so its retry is never sent.`; then push
     `{"name": "sign-in", "outcome": "unknown", "invocation": H}` onto SI's `parts`. It pushes no
     external intent: the hook's call has already completed, so no cancel is sent. `finish`
     removes the part's `hook_input`.
   - Call `end_hook_wait` first in `end_sign_in`, with `why` its outcome, `under` and its rule (for
     example `failed under PND-010`), and first in `signed_in`, with `why` `succeeded`, before each
     reads or changes anything else, so that every ending of S, this order's, 048a-5gh's and the
     first check's, ends a waiting hook part before S, and the last reply of the step is S's. Where
     no such part exists, it does nothing; a caller that ends the part first and then S, as
     048a-5gh may, finds it already ended.
   - In `requests.rs`'s `finish`, right after the line that 048a-5d adds to remove `hook_input`,
     add `if let Some(w) = v.get_mut("sign_in_wait").and_then(Value::as_object_mut) { w.remove("url"); }`,
     so that an ended S keeps no URL, whatever ends it (this order, 048a-5gh, a cancel or a
     restart). Then, right after the block that adds `META/pendingAction` for `notices` and before
     the reply is pushed to `t.request_replies`, add: where the member `meta` of `v`'s
     `sign_in_wait` is an object, set the key `mark(c, "signInLogin")` of `t.reply`'s `_meta` to
     it, so that the final result of a relayed sign-in holds `signInLogin` (IFC-049) in its reply
     and its request's reply alike.
   - `first_account`, `poll` and `sign_in_wait` are kept in the state alone, never in a record, a
     decision's facts (but `sign_in_wait`'s `login` and `expires`, decision 3), a reply (but
     `signInLogin`), an intent or a diagnostic.

6. **The retry, in `requests.rs`, `lib.rs` and `sign_in.rs`.** Replace `respond` with
   `pub(crate) fn respond(t: &mut Transition, evidence: &Evidence, invocation: &str, attempt: u64, identity: &access::Identity, responses: &Value, c: &Context) -> Result<()>`,
   and change `step_body`'s arm for `Input::Respond` to pass `&evidence`, `&invocation`, `attempt`,
   `&identity`, `&responses` and `context`, with `?`; `meta` stays unread. In `reply_context`, add
   `Input::Respond { evidence, .. }` to the inputs that give `(evidence, None)`, beside
   `Input::Complete`, so that a reply that ends S on a retry is marked with its caller's unanswered
   actions as a completion's is. `respond` takes an `Allocation` from `evidence` with cursor 0, then:
   - **Is a sign-in waiting?** It finds the index `i` of the invocation whose `id` is `invocation`,
     whose `state` is `in-progress`, whose `operation` is `Q("sign-in.v1")`, whose `attempt` is
     `attempt`, and for which `crate::sign_in::waiting_for_retry(v, attempt)` is true, a
     `pub(crate) fn waiting_for_retry(v: &Value, attempt: u64) -> bool` in `sign_in.rs` that is
     true exactly where SI's `phase` is `retry-pending` and `sign_in_wait.attempt` equals
     `attempt` (`requests.rs` names no phase), where also `access::established(&t.state, identity)` gives an instance whose `id`
     equals that invocation's `sender`. Where there is none: push one decision named
     `refuse-input-response`, citing `IFC-049`, with the facts
     `{"invocation": invocation, "attempt": attempt}` and no affected object; set `t.reply` to
     `{"error": {"code": -32602, "message": T, "data": {META/reason: "invalid", "diagnostics": [{"ruleId": "IFC-049", "level": "error", "message": {"text": T}}]}}}`,
     `T` being
     `invocation {invocation} is not a sign-in.v1 waiting at attempt {attempt} for a retry from this Interface, so the retry was not passed on`;
     call `end` with the route `{"consumer": C, "ruleId": "IFC-049"}`, `C` the established
     instance's `id` or null, no outcome, the reason `invalid` and the invocation ID `invocation`,
     so that the refusal is recorded (REC-001); and return. Nothing else changes.
   - Otherwise it returns `crate::sign_in::retried(t, &mut a, c, i, responses)`.

   In `sign_in.rs`, add
   `pub(crate) fn retried(t: &mut Transition, a: &mut Allocation<'_>, c: &Context, i: usize, responses: &Value) -> Result<()>`,
   which acts by `url_action(responses)`:
   - **accept:** where no object has S's `object` as its ID, or it is `retired`, push nothing and
     end S through `end_sign_in`, `failed`, under `PND-003`, with the cause
     `The instance was retired while sign-in.v1 waited for the person, so it polls nothing and signs nothing in.`;
     otherwise call the part function for `account-hook.v1`, with S as its parent, the instance as
     its object, the rules `PND-010`, `PND-011`, `IFC-049`, `ACC-018` and `IMP-002`, and no login,
     as 048a-5b's `begin` calls it. Where it gives `false`, end S as `begin` does when no
     implementation of `account-hook.v1` is resolved, and report whether any path reaches it.
     Otherwise set SI's `phase` to `poll-pending` and its `poll` to the part's ID, `H`, which the
     part function's reply `{"pending": true, "invocation": S, "hook": H}` gives at `/hook`, keep
     that reply as the step's, and push one decision named `poll-sign-in-account`, citing
     `IFC-049`, `PND-010` and `PND-011`, with the facts `{"invocation": S, "hook": H}`.
   - **decline or cancel:** push one decision named `end-sign-in-declined`, citing `PND-010`,
     `PND-013` and `IFC-049`, with the facts `{"invocation": S, "action": A}`; end S through
     `end_sign_in`, `failed`, under `PND-010`, with the cause
     `The person {did} the URL action on a retry, so no part was dispatched and nothing the retry carried was passed on.`,
     `{did}` being `declined` for `decline` and `cancelled` for `cancel`.
   - **anything else:** push one decision named `end-sign-in-malformed-response`, citing
     `IFC-049`, with the facts `{"invocation": S}`; end S through `end_sign_in`, `failed`, under
     `IFC-049`, with the cause
     `The retry's inputResponses held no response under url whose action is accept, decline or cancel, and its requestState was used, so no part was dispatched and nothing the retry carried was passed on.`

   No arm reads the retry's `meta`, changes S's `attempt`, answers or raises a pending action, or
   touches a hold, but as `end_sign_in` and its link of `sign_in.raised` already do.

   In `lib.rs`, the last sentence of `InputRequired`'s doc comment, "No step gives one until order
   048a-5e.", becomes "`sign-in.v1` gives one when it relays its URL action.", naming no order, so
   that no source names 048a-5e ("Done when").

7. **The poll's answer.** In `part_answered`, before its test of `account-pending` and beside
   048a-5d's arm for `sign-in-pending`, add the arm where SI's `phase` is `poll-pending` and the
   part's `id` equals SI's `poll`. It pushes `{"name": "poll", "outcome": O, "invocation": H}` onto
   SI's `parts`, then, in this order:
   - where no object has S's `object` as its ID: end S failed under `PND-003` with
     `RETIRED_DURING_CHECK`, and return;
   - where `expected_account` gives no `Set`: end S failed under `PND-010` with
     `ACCOUNT_NOT_STRING`, and return;
   - where `first_check(O, outputs, account)` gives `Expected` or `NoAccount`: call
     `clear_sign_in_undone` and push `clear-sign-in-undone` as the first check does (PND-011);
   - where the instance is `retired`: end S failed under `PND-003` with `RETIRED_DURING_CHECK`, and
     return;
   - then, by `poll_check(O, outputs, account, &SI["first_account"])`, `outputs` being the part's
     `retained.outputs`:
     - `Expected(found)`: `signed_in(found)`, as the first check does;
     - `NoneYet`: push one decision named `ask-sign-in-again`, citing `IFC-049`, `PND-010` and
       `PND-011`, with the implementation choice `DL18` and the facts
       `{"invocation": S, "hook": H}`; then `relay_output` with S's ID and the step's time, which
       reads `W`'s URL and keeps its instant, so the output is given again with S's `attempt` and
       the first output's expiry, and sets the phase `retry-pending`;
     - `Other`: push one decision named `end-sign-in-other-account`, citing `PND-011`, with the
       facts `{"invocation": S, "hook": H}`, and end S failed under `PND-011` with the cause
       `While polling, the account check found an account that is neither the expected one nor the one the first check found; undoing it with a sign-out-hook.v1 part is not built yet (order 048a-5gh), so sign-in.v1 ends failed and nothing was signed in.`
       This is an interim ending, which 048a-5gh replaces; it raises nothing and names no account.
     - `NotKnown`: end S failed under `PND-011` with the cause
       `The account check while polling after the person's sign-in reported {outcome}, so the account is not known and sign-in.v1 completes nothing; send sign-in.v1 again.`,
       `{outcome}` being `O`; report which paths reach it (a reason from the IFC-026 arm, or
       `unavailable` under IFC-037 from order 209's discovery).

   Any other part's answer while SI's `phase` is `retry-pending` or `poll-pending`, such as the
   waiting hook part ending without asking for input, takes the path the answer already gives a
   phase it does not handle: it returns, changing nothing of S. Report what reaches it.

8. **The unit tests.** Write the values as `json!` values.
   - In `sign_in.rs`'s tests, `sign_in_relay_message_and_wait`. Let BROWSER-LOGIN and DEVICE-LOGIN
     be 048a-2's fixture's `browser` and `device` logins; PAGE-LOGIN
     `{"name": "code", "return": "page", "authorization": "https://example.org/authorize", "clientId": "placeholder-signed-agent", "page": "https://example.org/code", "carried": true}`;
     SERVICE-LOGIN
     `{"name": "service", "return": "service", "authorization": "https://example.org/authorize", "clientId": "placeholder-signed-agent", "page": "https://example.org/done"}`;
     and LOCAL-LOGIN BROWSER-LOGIN with `carried` false. Let F be the facts `kind` `Agent`, `name`
     `signed`, `type_name` `signed-agent.v1`, `package` `example.org/sign-in-lapse`, `node`
     `local`, `host` `example.org` and `account` `primary-account`. Assert with `assert_eq!`:
     `accepts_return` of BROWSER-LOGIN not plain, true; plain, false; LOCAL-LOGIN not plain,
     false; PAGE-LOGIN not plain, true; DEVICE-LOGIN and SERVICE-LOGIN not plain, false;
     `relay_message` with F and, in turn, BROWSER-LOGIN with `accepts` true, BROWSER-LOGIN with
     false, PAGE-LOGIN with true, DEVICE-LOGIN with false and SERVICE-LOGIN with false: the whole
     text, as decision 1 gives it with these values, written out in full as a string literal in the
     test, none of the five holding `Open the device page`; `relay_request("M", "https://example.org/x")`
     and `login_meta(&BROWSER-LOGIN, &json!({"kind": "Resource", "name": "local", "id": "n1"}), true)`,
     each against its whole value; `wait_end("2026-11-12T12:00:00Z")` is
     `Some("2026-11-12T12:10:00Z")`, `wait_end("2026-11-12T23:55:00+02:00")` is
     `Some("2026-11-12T22:05:00Z")`, and `wait_end("soon")` is `None`.
   - In `sign_in.rs`'s tests, `sign_in_polling_answers`. Assert with `assert_eq!` that `url_action`
     gives `Some("accept")` for `{"url": {"action": "accept"}}` and for
     `{"url": {"action": "accept", "content": {}}}`, `Some("decline")` for
     `{"url": {"action": "decline"}}`, `Some("cancel")` for `{"url": {"action": "cancel"}}`, and
     `None` for `{"url": {"action": "ACCEPT"}}`, `{"url": {}}`, `{"url": "accept"}`, `{}`,
     `{"login": {"action": "accept"}}`,
     `{"url": {"action": "accept"}, "login": {"action": "accept"}}` and `null`; that
     `no_account_yet` is true for (`null`, `"other-account"`), (`null`, `null`),
     (`"other-account"`, `"other-account"`) and (`"OTHER-ACCOUNT"`, `"other-account"`), and false
     for (`"third-account"`, `"other-account"`), (`"other-account"`, `null`) and (`7`, `null`); and
     that `poll_check`, with the expected account `primary-account`, gives: for `succeeded` and
     `{"account": "Primary-Account"}`, `Expected("Primary-Account")`; for `succeeded` and
     `{"account": null}`, with `first` `"other-account"` and with `null`, `NoneYet`; for
     `succeeded` and `{"account": "OTHER-ACCOUNT"}` with `first` `"other-account"`, `NoneYet`;
     for `failed` and `{}`, and for `unknown` and `{}`, each with `first` `null`, `NoneYet`; for
     `succeeded` and `{"account": "third-account"}` with `first` `"other-account"`, and for
     `succeeded` and `{"account": "other-account"}` with `first` `null`, `Other`; and for
     `unavailable` and `{}`, and for `succeeded` and `{"account": 7}`, `NotKnown`.
   - 048a-5d's `sign_in_hook_input_kept_nowhere`, and the last fresh copy of 048a-5d2's
     `sign_in_url_mismatch_kept_nowhere` (URLACTION with no `asked_login`), which drive a URL that
     passes through `complete`, change only in their assertions for that URL and in the state they
     build. Where its state cannot give `relay_output` the Type, the package, the account or
     the node, add to it only what `relay_output` reads, as 048a-5d2 gave its match what it reads,
     and report what you added. For URLACTION, the assertions become: `h1` `in-progress`, its
     `hook_input` `{"key": "sign-in", "request_state": "opaque-hook-state-0001"}`; `g1`
     `in-progress`, its `sign_in.phase` `retry-pending`, its `sign_in_wait.url`
     `https://example.org/device`, its `sign_in_wait.expires` the step's time plus ten minutes;
     `t.reply` `{"pending": true, "invocation": "g1"}`; one entry in `t.input_required`, whose
     `request.params` holds exactly the keys `mode`, `message` and `url`, whose `params.message`
     equals the whole text that decision 1 gives for the test's state, written out in full as a
     string literal, and whose `meta` holds under `mark(&c, "signInLogin")` the `name` `device`,
     the `return` `device` and `acceptsReturn` false; and, as before, empty `pending_actions` and
     `sign_in_holds`, no decision `raise-sign-in-action`, and none of `opaque-hook-state-0001`,
     `example.org/device` and `Open the device page.` in what the test already serializes, and
     neither `opaque-hook-state-0001` nor `Open the device page.` in that entry serialized. With
     `h1`'s `plain` `false`, the same, `acceptsReturn` false still, since the device login takes no
     return. Its other assertions, the failing answers' and the cancelled sign-in's, do not change.
     Add one more fresh copy: with `g1`'s `sign_in.phase` `retry-pending` and `h1` holding
     `hook_input`, the call gives `h1` `unknown` with one diagnostic whose `ruleId` is `IFC-010`,
     `g1` unchanged, the decision `drop-sign-in-hook-answer` and an empty `t.input_required`. In
     048a-5d2's copy, the assertions for that URL become the same as these, its decision
     `end-sign-in-unrelayed` being gone: the decision `relay-sign-in-url` holds the facts' `login`
     `device`, and no decision is named `refuse-sign-in-hook-answer`; its other copies do not
     change.
   - In `requests.rs`'s tests, rename `respond_is_refused_until_input_requests_are_built` to
     `respond_without_a_waiting_sign_in_is_refused_and_recorded`, keep its input whole, and assert
     with `assert_eq!`: the reply's `error.code` is `-32602`, its `error.message` is decision 6's `T`
     with `absent-invocation` and `1`, its `error.data` holds `mark(&c, "reason")` `invalid` and
     one diagnostic whose `ruleId` is `IFC-049` and whose text is that `T`; `t.records` holds one
     record, whose `type` is `request-ended` and whose `data` holds `reason` `invalid`, `ruleId`
     `IFC-049` and `invocation` `absent-invocation`; `t.input_required` and `t.external_intents`
     are empty; the state's invocations equal the fixture's; and the one decision's `action` is
     `refuse-input-response`.

9. **The runner reads `respond`, and asserts `input_required`.** Add
   `conformance/case-schema/inputs/respond.json`, written as `complete.json` is, two-space
   indentation and a final newline:

   ```json
   {
     "type": "object",
     "additionalProperties": false,
     "properties": {
       "kind": {
         "const": "respond"
       },
       "evidence": {
         "type": "object"
       },
       "invocation": {
         "type": "string"
       },
       "attempt": {
         "type": "integer",
         "minimum": 1
       },
       "identity": {
         "type": "object"
       },
       "responses": {
         "type": "object"
       },
       "meta": {
         "type": "object"
       }
     },
     "required": [
       "kind",
       "evidence",
       "invocation",
       "attempt",
       "identity",
       "responses"
     ]
   }
   ```

   and `conformance/case-schema/expect/input_required.json`, written as `external_intents.json`
   is:

   ```json
   {
     "type": "array",
     "items": {
       "type": "object"
     }
   }
   ```

   In `engine/crates/conformance/src/portable.rs`, add `"respond"` right after `"complete"` in
   `check_case`'s list of kinds and in its test's list; in `checks.rs`, add `"respond"` right after
   `"complete"` in `reply`'s list, so that a retry's reason is checked as a completion's is. The
   runner needs no step module: it already passes the kind to the core. In `conformance/README.md`'s
   section "Assertions", right after the sentence that ends "for example `/reply/outcome` on an
   undispatched request.", add, in the same paragraph:

   ```text
   `input_required` projects the step's input requests, each `{invocation, attempt, request, meta, expires}`, which the shell relays to the Interface that asked (IFC-049); a step that gives none has no such member.
   ```

   Nothing else in the README, the case schema or the runner changes.

10. **One portable case of the relay, on 048a-2's fixture.** Add the directory
    `conformance/conventions/requests/sign-in-url-relayed/` with one file, `case.yaml`: ID
    `requests-sign-in-url-relayed`, `layers: [convention, interface]`,
    `promises: [IFC-049, IFC-010, PND-010]`, title
    `Relay a sign-in's URL action: the input request holds only mode, message and the URL, signInLogin names the login, its node and whether a return is taken, the wait ends ten minutes later, the waiting sign-in conflicts until it ends, and a retry of the sign-in a restart ended is refused`,
    `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
    `{"fixture": "../sign-in-lapse-and-conflict/fixture.json", "manifest": "../sign-in-lapse-and-conflict/manifest.json", "documents": "../sign-in-lapse-and-conflict/documents.json"}`,
    048a-2's three files, unchanged: its `signed-server`, not marked `plain`, carries Conduit's
    keys. Write it as JSON with two-space indentation and a final newline, its members in the order
    of `requests-sign-in-lapse-and-conflict`. It must meet 046a's portable-case rule as it stands.

    Below, ROOT, `META/x`, "Root asks T with A", "a completion", REFUSED(R, RULE), "Root asks T
    with A and M", S, URL, LOGIN(X), ACCOUNT(X) and HOOK(M) are as (a), (b) and (h) define them. A
    new step's evidence is its time, `2026-11-12T12:00:00Z`, `ids` the 100 strings
    `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
    190000 + 100n + k, for k = 0 to 99, `revisions` the 16 strings `relay-` followed by the decimal
    of n, a `-` and the decimal of j, and `suffixes` the 16 strings `r` followed by the four-digit,
    zero-padded decimal of 100n + j, for j = 0 to 15, where n is the step's number in this case; a
    copied step keeps its own evidence. Let BU be the URL
    `https://example.org/authorize?client_id=placeholder-signed-agent&response_type=code&redirect_uri=http%3A%2F%2F127.0.0.1%3A8400%2Fcallback&code_challenge_method=S256&code_challenge=not-a-code-challenge-not-a-code-challenge-0&state=not-a-state-not-a-state`,
    plainly fake values that meet IFC-049's checks for `browser`: origin and path equal to its
    `authorization`, its `clientId`, `response_type` `code`, a `redirect_uri` with its host, one of
    its ports and its path, `S256`, a challenge of 43 base64url characters and a state of 23
    unreserved characters. BROWSER is the completion
    `{"kind": "result", "result": {"resultType": "input_required", "inputRequests": {"sign-in": {"method": "elicitation/create", "params": {"mode": "url", "message": "Open this page to sign in.", "url": BU}}}, "requestState": "not-a-request-state"}}`.

    Its steps:
    1. to 3. `requests-sign-in-lapse-and-conflict`'s steps 1, 2 and 3, `input`, `expect` and
       `capture`, whole, each with its evidence: first start, registration and root's setup of
       `signed`, `$setup1`.
    4. to 6. Its steps 7, 8 and 9, likewise: the setup's two run reports and its account check,
       which finds the expected account; the setup succeeds.
    7. Root asks `group.sign-in.v1` with S and URL and LOGIN(`{"name": "browser"}`). `expect`
       `{"reply": {"pending": true}}`; `capture`
       `{"signin1": "/reply/invocation", "check1": "/reply/hook"}`.
    8. A completion for `$check1`, ACCOUNT(`null`). `expect` `{"reply": {"pending": true}}` with
       HOOK(M) whose `M` also holds `"META/signInLogin": {"name": "browser"}`; `capture`
       `{"hook1": "/reply/hook"}`.
    9. A completion for `$hook1`, BROWSER. `expect`
       `{"reply": {"pending": true, "invocation": "$signin1"}, "external_intents": [], "input_required": [{"invocation": "$signin1", "attempt": 1, "request": {"method": "elicitation/create", "params": {"mode": "url", "url": BU}}, "meta": {"META/signInLogin": {"name": "browser", "return": "redirect", "node": {"kind": "Resource", "name": "local", "id": "00000000-0000-4000-8000-000000000002"}, "acceptsReturn": true}}, "expires": "2026-11-12T12:10:00Z"}], "absent": ["/reply/isError", "/reply/content", "/reply/structuredContent", "/reply/requestState", "/reply/inputRequests"]}`:
       the URL is relayed as the hook gave it, the login named in the result's own `_meta` with its
       node and `acceptsReturn` true (a redirect login stating `carried: true`, served by a hook
       that carries Conduit's keys), and the wait ends ten minutes after this step; nothing is
       dispatched, and the sign-in does not end.
    10. Step 7's request. `expect` REFUSED(`conflict`, `PND-010`) with `"external_intents": []`:
        the waiting `sign-in.v1` is still in progress (048a-2's conflict).
    11. A `restore-state` step, `{"kind": "restore-state", "evidence": E}`, `E` this step's
        evidence. `expect`
        `{"reply": {"restored": true}, "external_intents": [], "record_facts": [{"type": "invocation-ended", "data": {"invocation": "$signin1", "outcome": "unknown"}}, {"type": "invocation-ended", "data": {"invocation": "$hook1", "outcome": "unknown"}}]}`
        (an ordered subsequence, in recover's index order): a restart ends the waiting sign-in and
        its hook part unknown (DL6, IMP-007).
    12. Step 7's request. `expect` `{"reply": {"pending": true}}`; `capture`
        `{"signin2": "/reply/invocation", "check2": "/reply/hook"}`: no conflict, since the
        earlier sign-in has ended.
    13. Root answers `$signin1` with `"accept"`, the step as decision 11 writes it. `expect`
        REFUSED-RETRY(`signin1`) as decision 11 defines it: the restart ended the waiting
        sign-in, so its retry is refused and recorded (IFC-049), and `$signin2` is not touched.

11. **One portable case of polling, on 048a-3's fixture.** Add the directory
    `conformance/conventions/requests/sign-in-polling/` with one file, `case.yaml`: ID
    `requests-sign-in-polling`, `layers: [convention, interface]`,
    `promises: [PND-010, PND-011, PND-013, IFC-049]`, title
    `Each accepted retry of sign-in.v1 polls the account: no account, the first check's account or a failed poll asks again with the same expiry, the expected account signs in with signInLogin, another account ends it for now, and a decline or cancel ends it failed with no part`,
    `profile: first-build`, `initial` `{"recipe": "empty"}`, and `files`
    `{"fixture": "../sign-in-hold/fixture.json", "manifest": "../sign-in-hold/manifest.json", "documents": "../sign-in-hold/documents.json"}`,
    048a-3's three files, unchanged. Write it as decision 10 writes its case, its members in the
    order of `requests-sign-in-hold`.

    Definitions as decision 10's, with SUCCESS and PART(H) from (b). A new step's `ids` are the 100
    strings `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
    200000 + 100n + k, `revisions` the 16 strings `poll-` followed by the decimal of n, a `-` and
    the decimal of j, and `suffixes` the 16 strings `p` followed by the four-digit, zero-padded
    decimal of 100n + j. Steps 10 to 12 are at `2026-11-12T12:00:00Z` and steps 13 to 36 at
    `2026-11-12T12:01:00Z`, so that an output given again shows that its expiry is the first one's.
    "Root answers `$X` with A" is the step
    `{"kind": "respond", "evidence": E, "invocation": "$X", "attempt": 1, "identity": {"kind": "instance", "id": ROOT}, "responses": {"url": {"action": A}}, "meta": {}}`.
    These recur:
    - HOOK: 048a-5d's URLACTION, `{"kind": "result", "result": R}`, `R` being
      `{"resultType": "input_required", "inputRequests": {"sign-in": {"method": "elicitation/create", "params": {"mode": "url", "message": "Open the device page.", "url": "https://example.org/device"}}}, "requestState": "opaque-hook-state-0001"}`;
    - CHECKFAIL: the completion of `requests-sign-in-account-first`'s step 26 as 048a-5d leaves
      it (index 25, the completion of `$check3`; step 24 at c097ec81), read before this order
      inserts its DECLINEs, `input.completion`, whole: a keyed account check that ends failed;
    - HOOKPART: `{"reply": {"pending": true}, "external_intents": [{"kind": "dispatch", "attempt": 1, "call": {"params": {"_meta": {"META/operation": Q}}}}]}`,
      `Q` the core's qualified reference to `sign-in-hook.v1`, written as PART(H) writes its `Q`;
    - WAIT(X): `{"reply": {"pending": true, "invocation": "$X"}, "external_intents": [], "input_required": [{"invocation": "$X", "attempt": 1, "request": {"method": "elicitation/create", "params": {"mode": "url", "url": "https://example.org/device"}}, "meta": {"META/signInLogin": {"name": "device", "return": "device", "acceptsReturn": false}}}]}`;
    - AGAIN(X): WAIT(X) with `"expires": "2026-11-12T12:10:00Z"` added to its one entry;
    - POLL(X): `{"reply": {"pending": true, "invocation": "$X"}}` with PART(`account-hook.v1`);
    - REFUSED-RETRY(X): `{"reply": {"error": {"code": -32602, "data": {"META/reason": "invalid", "diagnostics": [{"ruleId": "IFC-049", "level": "error"}]}}}, "external_intents": [], "record_facts": [{"type": "request-ended", "data": {"reason": "invalid", "ruleId": "IFC-049", "invocation": "$X"}}]}`;
    - ENDED(X, L, RULE): `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/invocation": "$X", "META/signInLogin": {"name": "device"}}, "structuredContent": {"diagnostics": [{"ruleId": RULE, "level": "error"}]}}, "external_intents": [], "record_facts": [{"type": "invocation-ended", "data": {"invocation": "$L", "outcome": "unknown"}}, {"type": "invocation-ended", "data": {"invocation": "$X", "outcome": "failed"}}]}`.

    Its steps:
    1. to 9. `requests-sign-in-hold`'s steps 1 to 9, `input`, `expect` and `capture`, whole, each
       with its evidence: through LAPSE, which raises `signed`'s sign-in action and begins its
       hold.
    10. Root asks `group.sign-in.v1` with S and URL. `expect` `{"reply": {"pending": true}}` with
        PART(`account-hook.v1`); `capture`
        `{"signin1": "/reply/invocation", "check1": "/reply/hook"}`.
    11. A completion for `$check1`, ACCOUNT(`"other-account"`). `expect` HOOKPART; `capture`
        `{"login1": "/reply/hook"}`: the first check's account, kept.
    12. A completion for `$login1`, HOOK. `expect` WAIT(`signin1`): the relay.
    13. Root answers `$signin1` with `"accept"`. `expect` POLL(`signin1`); `capture`
        `{"poll1": "/reply/hook"}`: accept polls the account.
    14. A completion for `$poll1`, ACCOUNT(`null`). `expect` AGAIN(`signin1`): no account yet asks
        again, with the first output's expiry though a minute has passed.
    15. Root answers `$signin1` with `"accept"`. `expect` POLL(`signin1`); `capture`
        `{"poll2": "/reply/hook"}`.
    16. A completion for `$poll2`, ACCOUNT(`"OTHER-ACCOUNT"`). `expect` AGAIN(`signin1`): the
        first check's account, compared ignoring ASCII case, counts as no account yet.
    17. Root answers `$signin1` with `"accept"`. `expect` POLL(`signin1`); `capture`
        `{"poll3": "/reply/hook"}`.
    18. A completion for `$poll3`, CHECKFAIL. `expect` AGAIN(`signin1`): a poll that failed counts
        as no account yet (PND-010, PND-011).
    19. Root answers `$signin1` with `"accept"`. `expect` POLL(`signin1`); `capture`
        `{"poll4": "/reply/hook"}`.
    20. A completion for `$poll4`, ACCOUNT(`"Primary-Account"`). `expect`
        `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded", "META/invocation": "$signin1", "META/signInLogin": {"name": "device", "return": "device", "acceptsReturn": false}}}, "external_intents": [], "record_facts": [{"type": "invocation-ended", "data": {"invocation": "$login1", "outcome": "unknown"}}, {"type": "invocation-ended", "data": {"invocation": "$signin1", "outcome": "succeeded"}}]}`:
        the expected account, compared ignoring ASCII case, signs in, the waiting hook part ends
        unknown first, and the final result holds `signInLogin`.
    21. Root asks `to-signed.work.v1` with `{}`. `expect` `{"reply": {"pending": true}}`; `capture`
        `{"call2": "/reply/invocation"}`: the success lifted PND-005's hold.
    22. A completion for `$call2`, SUCCESS. `expect`
        `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}}`.
    23. Root answers `$signin1` with `"accept"`. `expect` REFUSED-RETRY(`signin1`): an ended
        sign-in waits for no retry, and the refusal is recorded.
    24. Root asks `group.sign-in.v1` with S and URL. `expect` step 10's; `capture`
        `{"signin2": "/reply/invocation", "check2": "/reply/hook"}`.
    25. A completion for `$check2`, ACCOUNT(`null`). `expect` HOOKPART; `capture`
        `{"login2": "/reply/hook"}`.
    26. A completion for `$login2`, HOOK. `expect` WAIT(`signin2`).
    27. Root answers `$signin2` with `"decline"`. `expect` ENDED(`signin2`, `login2`, `PND-010`):
        no part is dispatched.
    28. to 30. Steps 24 to 26, each capture's names and each `$` name ending in `3` in place of
        `2`.
    31. Root answers `$signin3` with `"accept"`. `expect` POLL(`signin3`); `capture`
        `{"poll5": "/reply/hook"}`.
    32. A completion for `$poll5`, ACCOUNT(`"third-account"`). `expect`
        ENDED(`signin3`, `login3`, `PND-011`): another account, which the first check did not
        find, ends the sign-in until 048a-5gh undoes it.
    33. to 35. Steps 24 to 26, each capture's names and each `$` name ending in `4` in place of
        `2`.
    36. Root answers `$signin4` with `"cancel"`. `expect` ENDED(`signin4`, `login4`, `PND-010`).

    The routes of decisions 10 and 11 are new and have not been run. Before running either, check
    that no ID repeats across its steps' `ids`, the copied steps' included, and that no capture
    name is bound twice. If 046a's runner check
    refuses a case, if a step copied from another case gives a reply other than that case states,
    or if a step's reply differs from what it states, stop and report the error: do not change the
    expectations to pass it, and do not change the inputs beyond what these decisions say. No
    expectation is generated from the engine's output. The cases assert no message's words: IFC-050
    leaves wording to the engine, so decision 8's unit tests pin them. Each coverage index entry
    holds `id`, `case` (`conventions/requests/sign-in-url-relayed/case.yaml` and
    `conventions/requests/sign-in-polling/case.yaml`), `promises` and, where its neighbours in that
    piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write both in a
    `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
    `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves them
    into `conformance/conventions/requests/index.yaml` in ID order and leaves `manifest.yaml` byte
    for byte as it was.

12. **The steps that expected the interim ending.** Let RELAYED(X) be
    `{"reply": {"pending": true, "invocation": "$X"}, "external_intents": [], "input_required": [{"invocation": "$X", "attempt": 1, "request": {"method": "elicitation/create", "params": {"mode": "url"}}}]}`,
    and DECLINE(X) the step "Root answers `$X` with `"decline"`" as decision 11 writes it, its
    identity being that of the request that started `$X` (root's in each case below). Each step
    below completes a `sign-in-hook.v1` part with a URL that passes 048a-5d2's match and expected
    the interim ending. It keeps its `input`, `capture` and evidence, and its `expect` becomes
    RELAYED(X), X naming the `sign-in.v1` it serves by that case's capture. Where named below, a
    DECLINE(X) step is inserted right after it, whose `expect` is the changed step's earlier
    `expect`, whole: the decline ends `sign-in.v1` failed under PND-010 as the interim ending did,
    so the later steps, which send `sign-in.v1` again, meet no conflict. Its evidence: time that of
    the step before it, `ids` the 100 strings `00000000-0000-4000-8000-` followed by the
    twelve-digit, zero-padded decimal of 220000 + 100m + k, `revisions` the 16 strings `decline-`
    followed by the decimal of m, a `-` and the decimal of j, and `suffixes` the 16 strings `d`
    followed by the four-digit, zero-padded decimal of 100m + j, `m` being the inserted step's
    number in its changed case. Every later step moves one place for each step inserted before it,
    unchanged.
    - `requests-sign-in-hook-part`: step 10 (`$hook1`, serving `$signin1`), a DECLINE inserted as
      step 11; step 25 (`$hook6`, serving `$signin6`, after an account check that ended unknown),
      now step 26, with nothing after it. The case then has 26 steps.
    - `requests-sign-in-plain-hook-part`: step 5 (`$hook1`, serving `$signin1`), whose `expect`
      becomes, in place of RELAYED, the plain hook's relay whole:
      `{"reply": {"pending": true, "invocation": "$signin1"}, "external_intents": [], "input_required": [{"invocation": "$signin1", "attempt": 1, "request": {"method": "elicitation/create", "params": {"mode": "url", "url": "https://example.org/device"}}, "meta": {"META/signInLogin": {"name": "device", "return": "device", "acceptsReturn": false}}, "expires": "2026-11-12T12:10:00Z"}]}`
      (ruling 86, point 4: a plain hook's URL request is relayed, IFC-010, and takes no return,
      PND-013); a DECLINE inserted as step 6. The case then has 9 steps.
    - `requests-sign-in-account-first`: steps 19 (`$hook1`, `$sign1`), 24 (`$hook2`, `$sign2`), 27
      (`$hook3`, `$sign3`) and 34 (`$hook5`, `$sign5`), as 048a-5d numbers them; a DECLINE inserted
      after each of the first three, as steps 20, 26 and 30, so that `$hook5`'s step is 37. No
      DECLINE follows it: its later steps, an INSPECT and a held work request, send no
      `sign-in.v1`, and `$sign5` stays waiting to the case's end. The case then has 39 steps. Each
      DECLINE's `expect`, FAILED(N, `PND-010`), holds `"META/unanswered": 2`, which decision 6's
      change to `reply_context` gives.
    - `requests-keyed-hook-dispatch`: step 14 (`$hook1`, its plain hook), serving `$sign1`, which
      order 209's SIGN(1) captures at its step 11; a DECLINE inserted as step 15; and step 26
      (`$hook2`, serving `$sign4`, after a keyed account check whose URL request ended it unknown),
      now step 27, the case's last, with nothing after it. The case then has 27 steps.
    - `requests-sign-in-setup-check-failed-hook` (order 209, as 048a-5d leaves it): step 16
      (`$signhook1`, serving `$sign1`, after a failed account check). No DECLINE follows it: its
      later step, an inspection, sends no `sign-in.v1`, and `$sign1` stays waiting to the case's
      end. The case keeps its 17 steps.
    - `requests-sign-in-url-match` (048a-5d2): steps 9 (`$hook1`, `$signin1`, the redirect login's
      URL, so its relay's `acceptsReturn` is true) and 18 (`$hook4`, `$signin4`, the asked-for
      `device`), each followed by a new `sign-in.v1`; a DECLINE inserted after each, as steps 10 and
      20, so that `$hook4`'s step is 19. The case then has 26 steps.
    - `requests-sign-in-plain-url-match` (048a-5d2): step 5 (`$hook1`, `$signin1`, a plain hook's
      URL for the redirect login, so `acceptsReturn` false); a DECLINE inserted as step 6. The case
      then has 9 steps.

    Nothing else in those cases, and nothing in their directories' other files, changes. Before
    running them, check that no ID repeats across each case's steps and that no capture name is
    bound twice. Each must then pass; if a changed, inserted or later step gives another reply than
    it states, stop and report it: do not change it. No other case completes a `sign-in-hook.v1`
    part with a URL that passes (search `conformance/conventions/requests/*/case.yaml` for
    completions of a capture that a `sign-in-hook.v1` dispatch gives, `$hook*`, `$signhook*` or
    `$login*`, and list any outside the seven cases above and this order's two new cases,
    `requests-sign-in-url-relayed` and `requests-sign-in-polling`); where you find one, stop and
    report it.

13. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.** Find each place by its text read across
    line breaks, as orders 209, 048a-5d and 048a-5d2 leave it.
    - In DL18's row, the sentences from "Until order 048a-5e builds the relay," to "This is an
      unbuilt step, not a reading of PND-010.", in the same line, become:

      ```text
      From order 048a-5ef, a URL elicitation whose URL passes IFC-049's checks is relayed to the Interface that asked: one input request holding only `mode`, the engine's message and the URL, and in the result's own `_meta` a `signInLogin` naming the login, its return, the instance's node and whether a return is taken (PND-013's first check). The message names the instance, its Type and package, its node, the URL's host, the expected account and what the person brings back: for a device login, that the code is shown in the engine's panel; for a service login, that its page receives the return; and for a login whose return is not taken, that nothing comes back through the Interface and the login is completed on the node's machine. The hook part stays in progress, and `sign-in.v1` waits for the person's retry for ten minutes from the step that first relays the URL: every input request of the invocation carries that instant as its expiry, and the shell refuses its requestState from then on. Each accepted retry polls `account-hook.v1`: the expected account, compared ignoring ASCII case, ends `sign-in.v1` succeeded as the first check does; no account, the account the first check found, or a poll that ends failed or unknown counts as no account yet and answers another input request with the same expiry, for which the shell issues a fresh requestState; and a poll that ends with a reason ends it failed (PND-011). A decline or cancel ends it failed under PND-010, dispatching no part, and any other response ends it failed under IFC-049; a retry that names no `sign-in.v1` waiting at that attempt for that Interface is refused invalid under IFC-049 and recorded. Whenever `sign-in.v1` ends, but by a cancel of its request, its waiting hook part ends unknown; a cancelled sign-in leaves it in progress until a restart ends it (DL6), and its final result holds `signInLogin`. The URL is kept with the waiting invocation until it ends, and in no record. Until order 048a-5gh, another account found while polling ends `sign-in.v1` failed with no sign-out, and nothing ends the invocation at its expiry: a sign-in that gets no retry stays in progress until a restart ends it unknown (DL6), and a further `sign-in.v1` of the instance reports conflict meanwhile. A return that a retry carries is not read, and no retry of the hook is sent: PND-013's checks of a return come after the dogfood path (ruling 88).
      ```

    - In the section "Keys per hook and the sign-in fixes: ruling 86 (orders 207 and 209)":
      replace "whose URL elicitation, until order 048a-5e, ends `sign-in.v1` failed under PND-010's
      interim ending (`requests-sign-in-account-first`, `requests-keyed-hook-dispatch`,
      `requests-sign-in-hook-part`)" with "whose URL elicitation is relayed, and the account polled
      on each retry (order 048a-5ef; `requests-sign-in-account-first`,
      `requests-keyed-hook-dispatch`, `requests-sign-in-hook-part`, `requests-sign-in-polling`)";
      replace "Polling, where a failed or unknown poll counts as no account yet,
      is order 048a-5f's." with "Polling, where a failed or unknown poll counts as no account yet,
      is order 048a-5ef's (`requests-sign-in-polling`)."; replace "the hook part and the relay are
      orders 048a-5d and 048a-5e, whose cases show it." with "the hook part and the relay are orders
      048a-5d and 048a-5ef, whose cases show it."; and replace "Order 048a-5d takes that request
      into `sign-in.v1`, not as IFC-016's lapse trigger (`requests-sign-in-plain-hook-part`)." with
      "Order 048a-5d takes that request into `sign-in.v1`, not as IFC-016's lapse trigger, and order
      048a-5ef relays it, its `acceptsReturn` false (`requests-sign-in-plain-hook-part`)."
    - In "Consumer server (order 022)", replace "runs as far as its `sign-in-hook.v1` part, whose
      URL elicitation it does not yet relay (orders 048a-5b, 048a-5d and 048a-5e;" with "relays its
      `sign-in-hook.v1` part's URL elicitation and polls the account on each retry (orders 048a-5b,
      048a-5d and 048a-5ef;".
    - In "What the first build is", replace "signs an instance in only where its first account
      check finds the expected account, and where that check finds no account or another or ends
      failed or unknown dispatches a `sign-in-hook.v1` part, whose URL elicitation it does not yet
      relay (orders 048a-5d and 048a-5e; row DL18" with "signs an instance in where its first
      account check finds the expected account, and where that check finds no account or another
      or ends failed or unknown dispatches a `sign-in-hook.v1` part, relays its URL elicitation and
      signs the instance in once a poll on the person's retry finds the expected account (orders
      048a-5d and 048a-5ef; row DL18".
    - In "IFC-049's relay (order 048b-relay1)", after its last sentence, which ends "which is how
      this build reads IFC-049's "There is no elicitationId".", add: "From order 048a-5ef,
      `sign-in.v1` gives such an output when it relays its URL action, and an accepted retry
      reaches the core as its `respond` input, which polls the account (row DL18)."

    If order 209, 048a-5d or 048a-5d2 merged or built any of these places otherwise (the start
    check's item 8), replace what you find in their place, keeping 048a-5d2's words on the URL's
    checks, and report each before and after, whole. Nothing else in
    the file changes. After the other decisions, run `python3 tools/promise_standing.py --write`,
    as AGENTS.md asks of a change to a case's `promises` and to a promise ID in the core's sources.
    Report the rows of PND-004, PND-010, PND-011, PND-013 and IFC-049 and the totals; the totals
    change only by what those rows change.

14. **The declaration.** `docs/work-orders/048a5ef-declared.yaml` holds five shapes: one `added`
    for each new case, and one each of `input`, `expectation` and `flip` under `/steps/*` for the
    cases `requests-sign-in-*` and `requests-keyed-hook-dispatch`, since an inserted step moves
    every later step's index. The coordinator wrote it with this order; check that it matches and
    change it only to make it so.

15. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or ruling's
    record changes. No fixture or manifest changes, and no shell code: 048b-relay1's shell relays
    the output and passes the retry as they are. Not built: another account undone by a sign-out
    part and the end at DL18 (048a-5gh); a return checked or passed on, a retry of
    `sign-in-hook.v1`, and `signInLogin`'s `returned` (048a-7 and 048a-8, after dogfood under
    ruling 88); the end-to-end path through the Interface (048b-join); the command line (051b-2);
    the engine's panel. Report as findings, for the coordinator:
    - that a waiting sign-in stays in progress until a restart, so a further `sign-in.v1` of its
      instance reports conflict, until 048a-5gh ends it at its expiry, and that a poll may be in
      progress (`poll-pending`) when the expiry passes, or answer no account after it, giving an
      output whose state the shell refuses as expired;
    - that `requests::cancel` of a waiting `sign-in.v1` ends it but leaves its hook part in
      progress, holding `hook_input`, until a restart (048a-5b's V7);
    - that an accepted retry answered by another input request ends no request of its own, so no
      `request-ended` names it, and whether REC-001 asks for one;
    - that a retry carrying `signInReturn` polls and its return is not read, though `acceptsReturn`
      may be true on a hook that carries Conduit's keys, as decision 10's case shows, and the
      message for such a login asks the person to bring the return back;
    - that the device text points to the engine's panel, which does not yet show a code;
    - that the interim ending for another account raises nothing, which 048a-5gh replaces, and
      changes the polling case's step 32;
    - that the waiting sign-in now keeps a fixed port in progress, so 048a-5c's refusal covers a
      relayed login until its sign-in ends;
    - that an output whose Interface no longer waits is dropped by the shell, and the sign-in waits
      all the same;
    - that `retry_intent` stays unused until 048a-7;
    - as coordinator choices no check of this order tests: the package's version, the engine errors
      of decision 3, a retry for an instance gone or retired, a poll that states a reason, the
      waiting hook part ending without asking for input while S waits, the core's check of the
      retry's Interface, and the refused retry's record naming an invocation ID that may name no
      invocation;
    - and any place where IFC-010, IFC-049, PND-010, PND-011, PND-013 or DL18 left you a choice this
      order does not settle.

## Scope

Change only `engine/crates/core/src/sign_in.rs` (decisions 1, 3 to 7, and the tests of decision
8), `engine/crates/core/src/requests.rs` (`respond`, `finish` and the renamed test, decisions 5, 6
and 8), `engine/crates/core/src/lib.rs` (the `Respond` arm, `reply_context` and
`InputRequired`'s doc comment, decision 6), `engine/crates/core/src/inventory.rs` (decision 2), a
unit test of 048a-5d2 only as decision 8 says, `engine/crates/conformance/src/portable.rs` and
`checks.rs` (decision 9), the new files
`conformance/case-schema/inputs/respond.json` and `conformance/case-schema/expect/input_required.json`,
one sentence of `conformance/README.md`, `conformance/conventions/requests/index.yaml`, the new
directories `conformance/conventions/requests/sign-in-url-relayed/` and
`conformance/conventions/requests/sign-in-polling/`, the `case.yaml` of each case that decision 12
names, only at its steps and the steps it inserts, `engine/CONFORMANCE.md` (decision 13's five
places) and `engine/STANDING.md`, written only by the tool. Where 048a-5d or 048a-5d2 put a function
this order changes in another file, change it there, and only there, and say so.
`docs/work-orders/048a5ef-declared.yaml` comes with this order; change it only as decision 14
says. `conformance/manifest.yaml` holds the new entries only until the pieces tool moves them, and
ends unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, `engine/crates/shell/`, the
experiments, `conformance/case.schema.json`, any other piece, `conformance/internal-cases.yaml`,
`conformance/expected-failures.yaml`, any fixture or manifest, any other case or file of a case's
directory, `docs/implementation/` or `docs/proposals/`: report any defect as a finding.
`experiments/011-short-form-spike/` changes only if its tests need it, since it depends on the core;
report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- sign_in
RUST_TEST_THREADS=4 cargo test --release -p conduit-core --lib -- respond_without_a_waiting_sign_in_is_refused_and_recorded
RUST_TEST_THREADS=4 cargo test --release -p conduit-conformance --lib -- portable
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-sign-in-url-relayed
cargo run --release -p conduit-conformance -- .. requests-sign-in-polling
cargo run --release -p conduit-conformance -- .. requests-sign-in-hook-part
cargo run --release -p conduit-conformance -- .. requests-sign-in-plain-hook-part
cargo run --release -p conduit-conformance -- .. requests-sign-in-account-first
cargo run --release -p conduit-conformance -- .. requests-keyed-hook-dispatch
cargo run --release -p conduit-conformance -- .. requests-sign-in-setup-check-failed-hook
cargo run --release -p conduit-conformance -- .. requests-sign-in-url-match
cargo run --release -p conduit-conformance -- .. requests-sign-in-plain-url-match
cargo run --release -p conduit-conformance -- .. requests-sign-in-
cargo run --release -p conduit-conformance -- .. requests-
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell --lib -- relay::tests
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell forged_retry_state_is_refused_and_recorded
RUST_TEST_THREADS=4 cargo test --release -p conduit-shell first_start_presents_final_lists
```

The third reports four more tests passed than the start's run, 0 failed: the new
`sign_in_relay_message_and_wait` and `sign_in_polling_answers`, `requests.rs`'s renamed
`respond_without_a_waiting_sign_in_is_refused_and_recorded`, whose new name holds `sign_in`, and
`requests.rs`'s `succeeded_repeat_keeps_sign_in_login` (the read's R3); report both counts.
The fourth reports 1 passed. The fifth passes as at the start; report its count. The pieces tool
prints `Nothing to move.` The nine single-case runner commands each end `1 cases passed`;
`requests-sign-in-` gives two more cases than at the start, every one passing, and `requests-` two
more cases than at the start, with the start's expected failures, each still failing; report each
count. The shell tests pass. If any other case, or any shell, CLI or experiment test, fails because
a sign-in now waits in progress or a step now gives an output, stop and report it: do not change
it. The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that each behavior is what its case checks. From `engine/`, first keep copies:

```bash
cp crates/core/src/sign_in.rs target/sign_in.rs.fixed
cp crates/core/src/lib.rs target/lib.rs.fixed
```

Then make each change below by hand, alone, run its command, restore its file with `cp` from its
copy, and run it again. Each run before the restore must print `FAIL 0 cases passed; 1 failed`,
failing at the step named, and each run after it `1 cases passed`. Restore with `cp`, not `mv`: a
moved file keeps its older modification time, so cargo would not rebuild the crate. Git ignores
`target/`. Report the error each failing run printed.

| Change, by hand | Command (`cargo run --release -p conduit-conformance -- ..` and) | Must fail at |
| --- | --- | --- |
| In `sign_in.rs`'s `relay_output`, delete the one statement that pushes onto `t.input_required`, and nothing else | `requests-sign-in-url-relayed` | step 9, naming `/input_required` |
| In `sign_in.rs`'s `no_account_yet`, change `found.eq_ignore_ascii_case(first)` to `found == first` | `requests-sign-in-polling` | step 16, the interim ending for another account where another output is expected |
| In `sign_in.rs`'s `poll_check`, change `matches!(outcome, "failed" \| "unknown")` to `matches!(outcome, "unknown")` | `requests-sign-in-polling` | step 18, an ending under PND-011 where another output is expected |
| In `lib.rs`'s `reply_context`, delete the `Input::Respond` arm that decision 6 adds, and nothing else | `requests-sign-in-account-first` | step 20, the decline's reply without `META/unanswered` |

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/048a5ef-declared.yaml; echo "exit $?"
git status --short -- spec IN_FORCE.md conformance/manifest.yaml conformance/case.schema.json conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/fixtures conformance/conventions/requests/sign-in-lapse-and-conflict conformance/conventions/requests/sign-in-hold conformance/conventions/requests/sign-in-setup-account conformance/conventions/requests/sign-in-refusals-before-parts conformance/conventions/requests/sign-in-fixed-port conformance/conventions/requests/sign-in-login-plain-hook conformance/conventions/inventory conformance/conventions/setup conformance/language engine/crates/shell engine/crates/loader tools packages experiments ':(exclude)experiments/011-short-form-spike' docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/048a5ef-declared.yaml: 5 shapes.`,
then `Vector delta: C changes; F flips; 0 undeclared; 0 errors.` and `exit 0`; report `C` and `F`,
and that every change outside the two added cases lies in a case that decision 12 names.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. No promise's text changes, so no row of the model-checking account goes stale; where
`check_model_account.py` reports one stale, stop and report it. The unit tests end `OK`; report the
count. The `git status` command prints nothing; it leaves out `experiments/011-short-form-spike`,
which changes only as the scope allows.

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
grep -c 'pub(crate) fn relay_output(' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn keep_sign_in_wait(' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn retried(' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn waiting_for_retry(' engine/crates/core/src/sign_in.rs
grep -c 'found.eq_ignore_ascii_case(first)' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn end_hook_wait(' engine/crates/core/src/sign_in.rs
grep -c 'pub(crate) fn instance_type_and_package(' engine/crates/core/src/inventory.rs
grep -rn 'input_required.push(' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"retry-pending"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"poll-pending"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"relay-sign-in-url"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"poll-sign-in-account"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"ask-sign-in-again"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"end-sign-in-declined"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"end-sign-in-malformed-response"' engine/crates/core/src | wc -l | tr -d ' '
grep -rl '"end-sign-in-other-account"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'w.remove("url");' engine/crates/core/src/requests.rs
grep -c 'is not a sign-in.v1 waiting at attempt {attempt} for a retry from this Interface' engine/crates/core/src/requests.rs
grep -c 'Input::Respond { evidence, .. }' engine/crates/core/src/lib.rs
grep -c 'order 048a-5gh' engine/crates/core/src/sign_in.rs
grep -c '"respond",' engine/crates/conformance/src/checks.rs
grep -c '"const": "respond"' conformance/case-schema/inputs/respond.json
test -f conformance/case-schema/expect/input_required.json; echo $((1 - $?))
grep -c '`input_required` projects the step' conformance/README.md
grep -c '"id": "requests-sign-in-url-relayed"' conformance/conventions/requests/index.yaml
grep -c '"id": "requests-sign-in-polling"' conformance/conventions/requests/index.yaml
grep '^| DL18 |' engine/CONFORMANCE.md | grep -c 'From order 048a-5ef, a URL elicitation whose URL passes'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'is order 048a-5ef.s (.requests-sign-in-polling.).'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'relays its .sign-in-hook.v1. part.s URL elicitation and polls the account on each retry (orders 048a-5b, 048a-5d and 048a-5ef;'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'signs the instance in once a poll on the person.s retry finds the expected account (orders 048a-5d and 048a-5ef; row DL18'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'an accepted retry reaches the core as its .respond. input, which polls the account (row DL18).'
```

`grep -c '"respond",' engine/crates/conformance/src/portable.rs` prints 2, and
`grep -c 'mark(c, "signInLogin")' engine/crates/core/src/requests.rs` prints two more than
`git show HEAD:engine/crates/core/src/requests.rs | grep -c 'mark(c, "signInLogin")'`: 048a-5d's
in `dispatch_intent`, decision 5's in `finish`, and the read's R3 in `invoke_inner`'s repeat path.
These print, in order,
`['$hook1', '$sign1', '$hook2', '$sign2', '$hook3', '$sign3', '$hook5'] ['respond', 'respond', 'respond'] L1`,
`['$hook1', 'respond', '$hook2', L2] $sign1`, `L3 $signhook1 group.inspect.v1`,
`L4 respond L5 respond L6 respond respond L7 respond`, and
`13 36 browser True 2026-11-12T12:10:00Z ['mode', 'url'] restore-state respond respond`, each `L`
being that case's length at the start plus the DECLINEs that decision 12 inserts in it: `L1`
three more (39 from 36), `L2` one more (27 from 26), `L3` none (17), `L4` one more (26 from 25),
`L5` one more (9 from 8), `L6` two more (26 from 24) and `L7` one more (9 from 8); `13` and `36`
are the new cases' lengths:

```bash
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-account-first/case.yaml"))["steps"]; print([s[k]["input"]["invocation"] for k in (18, 19, 24, 25, 28, 29, 36)], [s[k]["input"]["kind"] for k in (19, 25, 29)], len(s))'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/keyed-hook-dispatch/case.yaml"))["steps"]; print([s[13]["input"]["invocation"], s[14]["input"]["kind"], s[26]["input"]["invocation"], len(s)], s[14]["input"]["invocation"])'
python3 -c 'import json; s = json.load(open("conformance/conventions/requests/sign-in-setup-check-failed-hook/case.yaml"))["steps"]; print(len(s), s[15]["input"]["invocation"], s[16]["input"]["request"]["tool"])'
python3 -c 'import json; a = json.load(open("conformance/conventions/requests/sign-in-hook-part/case.yaml"))["steps"]; b = json.load(open("conformance/conventions/requests/sign-in-plain-hook-part/case.yaml"))["steps"]; c = json.load(open("conformance/conventions/requests/sign-in-url-match/case.yaml"))["steps"]; d = json.load(open("conformance/conventions/requests/sign-in-plain-url-match/case.yaml"))["steps"]; print(len(a), a[10]["input"]["kind"], len(b), b[5]["input"]["kind"], len(c), c[9]["input"]["kind"], c[19]["input"]["kind"], len(d), d[5]["input"]["kind"])'
python3 -c 'import json; r = json.load(open("conformance/conventions/requests/sign-in-url-relayed/case.yaml"))["steps"]; p = json.load(open("conformance/conventions/requests/sign-in-polling/case.yaml"))["steps"]; o = r[8]["expect"]["input_required"][0]; m = [v for k, v in o["meta"].items() if k.endswith("signInLogin")][0]; print(len(r), len(p), m["name"], m["acceptsReturn"], o["expires"], sorted(o["request"]["params"]), r[10]["input"]["kind"], r[12]["input"]["kind"], p[12]["input"]["kind"])'
```

This prints `IFC-049:2 PND-004:0 PND-010:2 PND-011:1 PND-013:1`, each row's number of cases
against the start's, and printed all zeros at the start: PND-010 and IFC-049 gain the two new
cases, PND-011 and PND-013 the polling case:

```bash
python3 -c 'import re; rows = lambda p: {l.split("|")[1].strip(): int(l.split("|")[4]) for l in open(p) if re.match(r"^\| (PND-004|PND-01[013]|IFC-049) \|", l)}; b = rows("engine/target/standing-048a5ef-before.txt"); a = rows("engine/STANDING.md"); print(" ".join(k + ":" + str(a[k] - b[k]) for k in sorted(b)))'
```

Each of these prints 0: no source and no sentence of `engine/CONFORMANCE.md` names order 048a-5e
or 048a-5f, and what decisions 4, 8 and 13 remove is gone:

```bash
grep -rnE '048a-5e([^f]|$)|048a-5f' engine/crates/core/src engine/crates/shell/src | wc -l | tr -d ' '
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -oE '048a-5e[^f]|048a-5f' | wc -l | tr -d ' '
grep -c 'HOOK_UNRELAYED_CAUSE' engine/crates/core/src/sign_in.rs
grep -rl '"end-sign-in-unrelayed"' engine/crates/core/src | wc -l | tr -d ' '
grep -c 'fn respond_is_refused_until_input_requests_are_built' engine/crates/core/src/requests.rs
grep '^| DL18 |' engine/CONFORMANCE.md | grep -c 'no wait is chosen'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'it does not yet relay'
```

This prints 0, as at the start:

```bash
git diff HEAD --stat -- spec IN_FORCE.md conformance/expected-failures.yaml engine/crates/shell | wc -l | tr -d ' '
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the rows decision 13 names with the totals, the names, files and lines that the start
asks for, the signatures of decisions 1 to 3 and 6, the line in `hook_input` where the passing
branch calls `relay_output`, the lines in `part_answered` where `first_account` is set and the
`poll-pending` arm begins, what you added to a test's state under decision 8, each step decision 12
changed and each DECLINE it inserted, the places decision 13 replaced, before and after, and
whether any path reaches the engine errors of decision 3 and the cases decisions 6 and 7 ask about.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `048a5ef-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. Under ruling 87 a fresh Codex verifier reads the one round, and a fresh read-only Opus
reader reads the result, and the verifier's report where it holds a must, before it merges
([ruling 79](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-79/README.md), point 2, as ruling 87 amends it). The admin writes
this order's Record below when it merges.

Where the coordinator dispatched this order stacked on 048a-5d2's round-1 branch, the result is
read there, with `tools/gate.py --base` set to the commit this order started from, that branch's
tip, since `origin/main` lacks the predecessors' work; it merges only after 048a-5d2 has merged, and
with it orders 209 and 048a-5d before it, and then only once the coordinator has done the following
(the coordinator's, under that lever):

- rebase this order's branch onto `main` with `git rebase --onto origin/main` from 048a-5d2's
  round-1 tip, the commit this order started from, so that only this order's commits are replayed
  onto 048a-5d2's squash, committing and rebasing under the work orders' identity. Where
  048a-5d2's branch was itself rebased, onto 048a-5d's squash or 209's, after this order started,
  the commit this order started from is still the one to rebase from;
- resolve any conflict where a predecessor's verifier or fixer changed, after its round 1, what
  this order changes: `sign_in.rs`, `requests.rs`, `lib.rs` and `inventory.rs` under
  `engine/crates/core/src/`, `engine/CONFORMANCE.md`, the seven cases that decision 12 names and
  `conformance/conventions/requests/index.yaml`, keeping the predecessors' merged text and
  applying this order's decisions to it; write `engine/STANDING.md` again with
  `python3 tools/promise_standing.py --write` rather than by hand, and run the pieces tool again
  where the index conflicts;
- run this order's start check in a clean worktree of `main` after 048a-5d2's merge, and where a
  predecessor's merge changed anything that a decision reads (a step or a capture of their cases, a
  sentence of `engine/CONFORMANCE.md` that decision 13 replaces, a cause, a function's name or its
  passing branch), amend this order's text and the build to it, saying so in the Record;
- run again the nine single-case runner commands, `requests-sign-in-` and `requests-`, the
  whole engine suite, the `vector_delta.py` check with this order's declaration, and
  `tools/gate.py` against `origin/main`;
- have the reader read the rebase's resolutions and any amendment as new content (ruling 79,
  point 2), and write this order's Record.

## Record

Run on 2026-11-19, from about 18:48 to 19:24 UTC, on the Mac, stacked on `59ddb296` (order
048a-5d2's round 1, itself on 048a-5d's and 209's), against this order as refreshed on
`origin/main`, in one round: the worker (1837 s) and a fresh Codex verifier (320 s). The worker
built decisions 1 to 13 in the files of Scope: in `sign_in.rs`, `accepts_return`, `wait_end`,
`relay_request`, `login_meta`, `RelayFacts` and `relay_message`, `url_action`, `no_account_yet`,
`Poll` and `poll_check`, `keep_sign_in_wait`, `relay_output`, `end_hook_wait`, `waiting_for_retry`,
`retried` and `poll_answered`, with `hook_input`'s passing branch relaying the URL and the hook part
left in progress; in `inventory.rs`, `instance_type_and_package`; in `requests.rs`, the new
`respond` and `finish`'s removal of the URL and its `signInLogin`; in `lib.rs`, the `Respond` arm
and `reply_context`; the two new cases `requests-sign-in-url-relayed` and
`requests-sign-in-polling`, the seven cases decision 12 names, the two schema pieces, `index.yaml`,
the README's sentence, the five places of `engine/CONFORMANCE.md` and the tool's
`engine/STANDING.md`. Its own choices: the instance lookup, the clearing of `SignInUndone` and the
retirement check shared by the first check and the poll; the device relay's and privacy's assertions
shared by 048a-5d's two unit tests; and the cases generated from the order's steps and compared with
their sources. It reported the start's counts (24 `sign_in` tests, 15 and 82 cases), and after the
build 27 tests, 17 and 84 cases with the start's 2 expected failures, the engine's suite at 384
passed and 4 ignored, and the spike's 9. It could not make the stock `vector_delta.py` pass: it
stopped on a recursion, and with a guard in memory it gave 11553 changes, 0 flips, 0 undeclared and
0 errors.

The verifier, which ran the cases with the built runner (17 and 84, 2 expected failures) and rebuilt
nothing, found four musts and four shoulds: V1, that the relay could pass on personal data in a
query parameter that 048a-5d2's match does not check; V2, that a repeat of a sign-in that succeeded
after a relay answers without `signInLogin`; V3, that a poll that ends failed with no diagnostic is
answered by another relay that explains nothing, against PND-011; V4, the stock `vector_delta.py`'s
recursion; V5, that DL18's row says every ending of `sign-in.v1` ends its waiting hook part, which a
cancel of the request does not; V6, that the redirect relay's step does not pin the request's whole
key set; V7, that the stale retry's step does not show `$signin2` untouched; and V8, that the
polling case does not pin the first relay's expiry.

A fresh read-only Opus reader then verified the order (ruling 79, point 2, as ruling 87 amends it),
on `1b22f371`. It ran fmt and clippy, the unit tests (27 `sign_in`, 1 `respond`, 4 `portable`, the
core's whole library 143), the pieces tool (`Nothing to move.`), the nine single cases (1 each),
`requests-sign-in-` (17), `requests-` (84, 2 expected failures), `inventory-` (3) and `setup-` (43),
the shell's three tests and the spike (9), the order's four mutations (each failed at its named step
and passed after a `cp` restore), the tools' checks (Errors: 0 each) and unit tests (129, OK), the
order's greps, one-liners and standing rows, and the stock `vector_delta.py` (exit 2), and on a
scratch copy guarded as R1 says (11553 changes, 0 flips, 0 undeclared, 0 errors, every change in the
two added cases or the seven named). It refuted V1 (decision 4 relays the URL as the hook gives it
after 048a-5d2's match, whose order carries the login hint to the coordinator as an open choice), V6
(`assert_device_relay` asserts the whole `params`, and `relay_request` is the only constructor), V7
(the step is the order's own, and the refusal changes no invocation, as the renamed unit test
asserts) and V8 (step 14's expiry, 12:10 at 12:01, shows the first relay's instant kept). It found
two musts and two shoulds:

- R1 (V4): the stock `tools/vector_delta.py` recursed where an empty map under a step met a missing
  value, since its fallback for a container called `differences` again with the same arguments;
  `tools/gate.py` runs the same tool and would fail this merge.
- R2 (V3, with the coordinator's amendment carried from 048a-5d's reader): an account check, the
  first or a poll, that ends failed or unknown without diagnostics had its uncertainty explained
  nowhere, against PND-011.
- R3 (V2, downgraded to a should): an idempotent repeat of a sign-in that ended after a relay
  answered through `ended_reply` without `signInLogin`.
- R4 (V5): DL18's row, the order's verbatim text, overstated which endings close the waiting hook
  part.

The fixer left the branch on `59ddb296`, without a rebase, and applied each finding:

- R1: at the line the reader named, the fallback for a container under a step recurses only into a
  non-empty map, `(isinstance(before, dict) and before) or (isinstance(after, dict) and after)`, and
  `tools/tests/test_vector_delta.py` gains `test_added_empty_map_under_a_step`, which adds `"meta":
  {}` to a step's input and expects one `add` at `/steps/0/input/meta`; it errors on the stock tool.
  It lies outside this order's Scope (`tools/`) and landed as its own change (#420) before this
  merge, so the branch's copy was dropped at the rebase.
- R2, the coordinator's amendment as the reader wrote it: SI keeps the member `check`, set by
  `unexplained_check` to the outcome where it is `failed` or `unknown` and otherwise null, in
  `part_answered` beside `first_account` and in `poll_answered`'s `NoneYet` arm before
  `relay_output`. `RelayFacts` gains `check: Option<&str>`; where it is set, `relay_message` puts
  between the lead and the ending the sentence `The last account check ended {check}, so whether the
  account is signed in is not known; it counts as no account yet (PND-011).`, and
  `relay-sign-in-url`'s facts gain `"check"`. `check` joins `first_account`, `poll` and
  `sign_in_wait` among the members decision 5 keeps in the state alone, but for these facts.
  `sign_in_relay_message_and_wait` asserts the whole text for DEVICE-LOGIN with `Some("failed")`,
  and passes `None` for the five as before; no case changes, since the cases assert no words
  (IFC-050). Where S ends from the hook before any relay after a failed or unknown first check, its
  end cause names only the hook; the second read's R3 carries this to the follow-up order after
  the dogfood (see the rebase below).
- R3: after `ended_reply` on a repeat, where the old invocation's `sign_in_wait.meta` is an object,
  the reply's `_meta` holds it under `signInLogin`; the new test
  `succeeded_repeat_keeps_sign_in_login` repeats a succeeded call without and with that member.
  This lies outside the requests.rs functions that Scope names (respond, finish and the renamed
  test); the coordinator took it as an amendment. With it, Done when's `sign_in` count is four more
  than the start, and its count of `mark(c, "signInLogin")` in `requests.rs` two more, both amended
  so by the coordinator.
- R4: the row and decision 13 now read "Whenever `sign-in.v1` ends, but by a cancel of its request,
  its waiting hook part ends unknown; a cancelled sign-in leaves it in progress until a restart ends
  it (DL6), and its final result holds `signInLogin`."

With R2's sentence taken out of `relay_message`, `sign_in_relay_message_and_wait` failed, and with
R3's lines taken out, `succeeded_repeat_keeps_sign_in_login` failed; both passed once restored. On a
scratch change of `requests-sign-in-polling`, restored with `cp` and compared clean, the relay's
message at step 16, after a poll that found no account, held no such sentence, and at step 18, after
a poll that ended failed, held it with `failed`. The fixer then ran, in the worktree: `cargo fmt
--check` and `cargo clippy --all-targets -- -D warnings` (clean); the core's `sign_in` tests (28
passed, the new repeat test's name holding `sign_in`), the `respond` test (1), the new test (1), the
core's whole library (144) and `portable` (4); the pieces tool (`Nothing to move.`); the nine single
cases (1 each), `requests-sign-in-` (17), `requests-` (84, 2 expected failures), `inventory-` (3)
and `setup-` (43); the order's four mutations, each failing at its named step (9, 16, 18 and 20) and
passing after a `cp` restore; the shell's three tests (2, 1 and 1) and the spike (9); the tools'
checks, `check_docs.py` and `check_private_paths.py` among them (Errors: 0 each), and their unit
tests (130, OK); `vector_delta.py 59ddb296 WORKTREE --declared
docs/work-orders/048a5ef-declared.yaml` with the fixed tool (5 shapes, 11553 changes, 0 flips, 0
undeclared, 0 errors, exit 0); and `git diff --check` (clean). Early on it ran one `git fetch` by
mistake, which moved only a remote-tracking ref, not the branch.

The same reader then reread the result, read-only, on `b43b2707` (the round and the fix, still on
`59ddb296`), against the order as refreshed on `origin/main`, to which the fix's text applies with
offsets only. It reran the checks above (fmt and clippy clean; 28 `sign_in`, 1 `respond`, the
core's whole library 144, `portable` 4; `Nothing to move.`; the nine single cases, 17, 84 with the
2 expected failures, `inventory-` 3, `setup-` 43, `records-` 17 and `first-start-` 8; the shell's
three tests and the spike's 9; the four mutations at steps 9, 16, 18 and 20; Errors: 0 from the
tools' checks and 130 unit tests OK; the stock `vector_delta.py` against `59ddb296`, now with the
guard, 11553 changes, 0 flips, 0 undeclared, 0 errors, exit 0), took R3's lines out (the new test
failed, and passed once restored), and on a scratch copy of `requests-sign-in-polling`, restored
with `cp` and compared clean, made step 18's poll end unknown with an engine cause. It found every
fix as described, and one must and four shoulds:

- R1 (must): two Done-when lines, moved by R3 above, no longer held as worded: the `sign_in` count
  is four more than the start (28 against 24), and `requests.rs`'s `mark(c, "signInLogin")` count
  two more (3 against 1). The behaviour is right; the text was stale.
- R2: the sentence's clause "and said why nowhere" was false where the account part carries an
  engine cause: a keyed poll with no stated outcome ends unknown with an OPS-007 cause, and the
  probe's relay still said it.
- R3: where the first check ends failed or unknown and `sign-in.v1` then ends before any relay
  (no `sign-in-hook.v1` implementation, or the hook's own endings), `end_sign_in` never reads SI's
  `check`, so nothing explains that check's uncertainty; the coordinator's amendment covers only
  this order's relay.
- R4: R1 of the first read landed on `main` as its own change (#420), so the branch's `tools/`
  hunks are now redundant and the Record's R1 bullet was out of date.
- R5: the Record's R3 bullet did not say the fix lies outside the `requests.rs` functions that
  Scope names.

The fixer again left the branch on `59ddb296`, without a rebase, and applied each:

- R1: Done when's two lines now read "four more tests passed than the start's run", adding
  `requests.rs`'s `succeeded_repeat_keeps_sign_in_login` (the read's R3), and "prints 3, two more
  than at the start", adding the read's R3 in `invoke_inner`'s repeat path; the R3 bullet above
  records them as the coordinator's amendment.
- R2: `relay_message`'s sentence and `sign_in_relay_message_and_wait`'s literal drop the clause, so
  it reads "The last account check ended {check}, so whether the account is signed in is not known;
  it counts as no account yet (PND-011).", as the R2 bullet above now quotes; no case changes
  (IFC-050).
- R3: of the reader's two options, carried as a finding to a later order (at the rebase, the
  follow-up order after the dogfood; see below); no code changes here.
- R4: the R1 bullet above now says the branch's copy was dropped at the rebase; without a rebase,
  the fixer left that step to the coordinator: at the rebase onto `origin/main`, take `main`'s
  `tools/vector_delta.py` and `tools/tests/test_vector_delta.py`, so the squash changes no `tools/`
  file.
- R5: the R3 bullet above gains the reader's sentence on Scope.

The fixer then ran, in the worktree, after refreshing the kept copies in `target/`: `cargo fmt
--check` and `cargo clippy --all-targets -- -D warnings` (clean); the core's `sign_in` tests (28),
the `respond` test (1), `succeeded_repeat_keeps_sign_in_login` (1), the core's whole library (144)
and `portable` (4); the pieces tool (`Nothing to move.`); the nine single cases (1 each),
`requests-sign-in-` (17), `requests-` (84, 2 expected failures), `inventory-` (3) and `setup-`
(43); the order's four mutations, each failing at its named step (9, 16, 18 and 20) and passing
after a `cp` restore, compared clean; the shell's three tests (2, 1 and 1) and the spike (9); the
tools' checks, `check_docs.py` and `check_private_paths.py` among them (Errors: 0 each), and their
unit tests (130, OK); `vector_delta.py 59ddb296 WORKTREE --declared
docs/work-orders/048a5ef-declared.yaml` (5 shapes, 11553 changes, 0 flips, 0 undeclared, 0 errors,
exit 0); and `git diff --check` (clean). The rebase, the engine's full suite and the gate's run on
this order's squash are the coordinator's at the merge.

A third read, read-only, on `303b8a5a` (the round and both fixes, still on `59ddb296`), found no
must and three shoulds, each the coordinator's at the rebase: R1, that the second fix's hunk on the
`mark(c, "signInLogin")` line would not apply to the governing text on `main`; R2, that the
branch's `tools/` copy was not yet dropped; and R3, that the second read's R3 was not in 048a-5gh's
text. After order 048a-5d2 merged (#419, `f2d972df`), on 2026-11-19, the branch was rebased onto
`main` at `9c313097` (#420 on top of it) with `git rebase --onto origin/main 59ddb296`, after a
check, in the same command, that `59ddb296`, 048a-5d2's round-1 checkpoint this order started from,
was an ancestor of the tip and that 048a-5d2's squash was on `main`, so that only this order's three
commits were replayed, under the work orders' identity. One conflict: in this file, at the second
fix's commit, whose amendment of the `mark(c, "signInLogin")` line did not apply to `main`'s
wording ("one more than `git show HEAD:...`"). It was resolved by keeping `main`'s form and writing
the same amendment into it, as the coordinator's: the line now reads "two more than", naming the
read's R3 in `invoke_inner`'s repeat path beside 048a-5d's mark and decision 5's, and the lengths
stay `main`'s `L1` to `L7`. The commit's other hunks applied: the `sign_in` count of four more
tests, adding `succeeded_repeat_keeps_sign_in_login` (the governing text now says so), decision
13's DL18 sentence as V5 amended it, and this Record. Everything else merged without a conflict.
`tools/tests/test_vector_delta.py` kept the branch's `test_added_empty_map_under_a_step` beside
#420's own test, so, as the third read's R2 asks, `main`'s `tools/vector_delta.py` and
`tools/tests/test_vector_delta.py` were checked out over the branch's: the squash changes no
`tools/` file, `git diff origin/main --stat -- tools/` prints nothing, and the vector delta below
passes with `main`'s tool. `sign_in.rs` took 048a-5d2's fixes (`url_match_cause`'s two whole
texts, `sign_in_url_match`'s three new rejections and its empty-username URL, and the mismatch
test's `t.input_required` check); `requests-keyed-hook-dispatch` took order 209's `_meta` on its
five `server/discover` intents; `requests.rs` took 048a-5d's and 209's `fence_attempt` calls in
`complete_hook_discovery`, and the shell's `discover_call_parses_as_a_client_request` test took
209's `_meta`, each merging cleanly; and `requests-sign-in-plain-hook-part` took 048a-5d's R2, the
`invocation-ended` fact for `$check2` ending failed, now at step 8, one step later than on `main`,
after the DECLINE that decision 12 inserts at step 6. `engine/STANDING.md` did not conflict and
`promise_standing.py --check` found it current, so it was not written again; the index did not
conflict, and the pieces tool prints `Nothing to move.` Following 048a-5d2's fixes needed no change
in this order's build: the mismatch test's new check sits in the assertion its failing answers
share, while the matched URL's assertions are decision 8's; `url_match_cause`'s words are unchanged
and this order quotes none; the fixes changed neither of 048a-5d2's cases; and neither new case
holds a `server/discover` intent or copies the plain case's step 7. The only text amendment is the
`mark` line above. The second read's R3, where `sign-in.v1` ends from the hook before any relay
after a failed or unknown first check, its end cause naming only the hook, is not carried to
048a-5gh, which is already built: as the bullets above now say, it is carried to the follow-up order
after the dogfood, beside 048a-5gh's deferred shoulds (the third read's R3).

This order's start check, run on a clean copy of `main`'s tree at `9c313097` (by `git archive`),
printed what the order gives at every item: `:0` for 207, 209 and 048a-5d2 and `1` for 048a-5d
(all three merged); `7`; `1` five times; `1`, `1`, `1`, `4`, `1`, `1`, `1` and `1`, then `1` and
`1`; `1`, `1`, `1`, `0` and `relay 0`; `0`, two lines ending `:0`, `0` and `0`; item 7's
`24 $hook1 group.sign-in.v1 $hook4 group.sign-in.v1 8 $hook1 group.sign-in.v1`; `1` eight times;
items 9 to 13 as written; and item 14's three sources, `lib.rs`, `requests.rs` and `sign_in.rs`. On
`main`, `part_answered` is at `sign_in.rs` line 215, the part function `sign_in_hook_part` at 341,
`url_match` at 387, `hook_input` at 563, `signed_in` at 762 and `end_sign_in` at 808; `respond` at
`requests.rs` line 750 and `finish` at 1865; and `reply_context` at `lib.rs` line 428. Its runs gave
the `sign_in` filter 24 passed, `portable` 4, `requests-sign-in-` 15 cases passed and `requests-` 82
with the 2 expected failures, and the five rows stand at IFC-049 5, PND-004 0, PND-010 12, PND-011
5 and PND-013 3. So 048a-5d2's merge changed nothing a decision reads. Warm in the worktree, from
`engine/` with 4 test and vector threads: `cargo fmt --check` and clippy with `-D warnings` (exit
0); the `sign_in` tests (28 passed, four more than on `main`), the `respond` test (1) and
`succeeded_repeat_keeps_sign_in_login` (1); `portable` (4); the pieces tool (`Nothing to move.`);
the nine single-case runs (`1 cases passed` each); `requests-sign-in-` (17 cases passed) and
`requests-` (84 cases passed; the same 2 expected failures); and the shell's `relay::tests` (2),
`forged_retry_state_is_refused_and_recorded` (1) and `first_start_presents_final_lists` (1). From
the root: `promise_standing.py --check` (Errors: 0; the five rows move by `IFC-049:2 PND-004:0
PND-010:2 PND-011:1 PND-013:1` against `main`); `vector_delta.py origin/main WORKTREE --declared
docs/work-orders/048a5ef-declared.yaml` (5 shapes, 11564 changes, 0 flips, 0 undeclared, 0 errors,
exit 0; eleven more than against `59ddb296`, five in `requests-keyed-hook-dispatch` and six in
`requests-sign-in-plain-hook-part`, from the predecessors' fixes in steps that decision 12 moves);
`check_schemas.py`, `set_names.py --check`, `check_rulings.py`, `check_docs.py`,
`check_roadmap.py`, `check_model_account.py` and `check_private_paths.py` (Errors: 0 each); the
tools' unit tests (130, OK); every grep and one-liner of "Done when" as the governing text gives
them (31 lines print 1, the five lengths' lines, seven zeros, and the protected paths' 0); and `git
diff --check` (clean). The whole engine suite, the spike, the four mutations, `tools/gate.py`
against `origin/main` and the reader's read of this rebase remain the coordinator's before the
merge.
