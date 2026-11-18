# Work order 047c (E6c): `answer.v1` with stored answers: dismissed, retried and retired, conflict, and answered state that survives a purge

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

**Where it sits.** [Order 117](117-plan-s2-s3.md)'s entry 047 and order 047's "Where it sits" cut
the pending-action core into four orders, each after the one before (the coordinator's, open to
the owner's strike, under [FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264):
an order holds one behavior and stays small). This is the third: "047c: `answer.v1` in place of
its stub (PND-009), with stored answers: `dismissed` for a setup, `retried` by a further setup and
`retired` by retirement recorded as answers, a dismissal or expiry that stays answered,
`conflict` for an answered or expired action, and answered state that survives a purge (PND-003,
PND-006, PND-012). Its refusal of who may answer takes its place in ACC-056's order of refusing
rules, which order [171a](171a-so-a-form-pools-refusals.md) wrote and 046a builds."
[Order 047](047-build-setup-action.md) computes a failed setup's action and raises it at root's
own inspection; [order 047b](047b-build-actions-raised-by-scope.md) raises it wherever PND-002
puts it. 047d builds the notice kind (PND-007), its `pendingAction` key and `acknowledged`;
048a-2 sign-in as a kind of pending action, PND-005 and `SignInUndone`. This order builds none of
them, and no kind but setup.

**What it builds.** One behavior: answering an action, and keeping the answer. PND-009: "An
Interface asks answer.v1 with {action, answer}. It is dispatched only on the Interface's own
behalf, through an open Conduit listing it to the target that authorizes inspection of the
subject (ACC-033)", and "An action not raised at the caller reports refused before invalid,
revealing nothing of whether it exists. An answer not accepted by the kind reports invalid and
names the answering operation ... answer.v1 accepts dismissed for setup ... It has no outputs and
dispatches no hook or model work. Already answered actions report conflict (PND-003)." PND-006:
"Dispatch of a further set-up.v1 answers retried; retirement answers retired; answer.v1 may answer
dismissed. ... A dismissal or expiry for that invocation stays answered and is not recomputed as
unanswered. A later unsuccessful setup makes a new action." PND-003: "Once it passes the answer
is expired, with no actor, and nothing is dispatched. An operation answering an answered, expired
or void action reports conflict after the access decision." PND-012: "Its record names the
action's ID. The recorded answer holds the kind, the evidence shown and the answer with actor,
invocation and time", and "Pending state and answered state used to prevent repeated answers
survive a purge of records under REC-004". ACC-056 puts "PND-009 ... on who may answer" in its
step (3), "the rules on the caller and the operation it asks for alone", after SEC-007 and before
ACC-033, whose row 17 gives answer.v1's target: "the target authorizing inspection of the
subject, under rows 6, 6A and 7, including their retired-owner fallback (PND-009)". [Spec batch
1's plan](../proposals/spec-batch-1/areas/pending-actions.md) lists the vectors `setup-failed`,
`setup-retried`, `answer-errors` and `expiry` (its cases 17, 18, 22 and 24); this order's one case
covers them for the setup kind, as the promises now word them.

**What orders 047 and 047b bring, which this order assumes.** Both are written on `main`; 047 is
dispatched and 047b is not yet built. This order starts only once both are merged (below). From
047, as its Record names them:

- `engine/crates/core/src/pending.rs`, with `mod pending;` in `lib.rs`, holding 047's two
  functions: the setup action of an object, computed from the state whatever the time, in
  `pending-action.v1`'s form `{"id", "kind": "setup", "subject": [{"kind", "name", "id"}],
  "evidence", "expiry"}`, its ID the latest set-up.v1 invocation's (PND-001), none where the
  object is retired; and the test whether an action is raised at a time, true before its expiry
  and false at it and after. Its unit test `setup_action_follows_the_latest_setup`.
- The members `ended` and `dispatched` on set-up.v1 invocations (047's decision 1), set in
  `requests` where a set-up.v1 invocation is written to `state.invocations`, at a first dispatch
  and at every redispatch; a redispatch under OPS-012 keeps the invocation's ID, starts without
  `ended` and takes a new `dispatched`.
- 047's reading that a redispatched setup that ends failed or unknown again raises an action
  under the same invocation ID, with an expiry counted from its new ending, and its decision 10,
  which asks the worker to report "that PND-001 gives the new action of a redispatched setup
  (decision 2) the ID of the old, answered or expired, one". Decision 1 below takes the stand.
- The decision `raise-setup-action`, the portable case `setup-action-at-root`, the expected
  failure `e4-reconcile-setup-action-open` lifted, `answer.v1`'s stub naming 047c (the entry
  `("answer.v1", "047c", "PND-009")` in C11's stub table in `requests.rs`, and the tag `"047c"`
  beside `management.answer.v1` in the shell test
  `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`), and four places
  in `engine/CONFORMANCE.md` naming 047c.

From 047b, as its Record names them:

- In `pending.rs`: the scope test and the function that says whether an action is raised at a
  holder at a time, whatever its kind (047b's decision 1), which calls 047's time test first and
  then tests the holder (not an Interface or a template: false; root: true; otherwise its scope
  holds a subject); the view of an action at a holder, which removes `code` and `fingerprint`
  items at every Interface, root included (decision 2); the function giving every action the
  state holds, in byte order of ID, which says "no stored kind exists yet (047c and 047d add them
  here)", and the views raised at a holder, whose number is the count (decision 3); and its unit
  test `actions_raised_by_scope`.
- `status.pending` at each Interface's own inspection, `list.v1`'s `pending` output with the
  decision `list-pending-actions`, and the `_meta` key `unanswered` on each result of an
  Interface's own request where its count is not zero (decisions 4 to 6).
- The portable case `setup-action-by-scope` in `conformance/conventions/setup/`, its route
  through first start and the e4 fixture (`conformance/conventions/inventory/e4/`), and DL19's
  row in `engine/CONFORMANCE.md` carrying 047b's sentence "While raised it is raised at root and
  at each Interface whose scope holds its instance, never at an Agent (order 047b): ...".

Where 047 or 047b merged otherwise than its order says, apply the nearest sound reading of each
decision below to what merged, and say so.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`facb4ff4`, before orders 047 and 047b; find each place by the text quoted with it.

- `answer.v1` is a stub: in `invoke_inner` (`requests.rs:1150`, "C11's temporary stubs"), after
  the access decision, the repeat, the preflights and `available`, it reports unavailable under
  PND-009, and `reject_diagnostics` (`requests.rs:616`, the arm `"PND-009" => Some(format!(`)
  writes its message `not built yet, order 047c` (after 047). Root's Conduits from the core's
  `root-management`, `root-pool` and `root-fleet` templates list `answer.v1`
  (`spec/core/templates.yaml`), and so do `pool-manage` and `fleet-manage`.
- `access::evaluate_path` (`access.rs:711`) runs ACC-056's steps as order 046a ordered them: the
  rules on the caller and the operation alone, ACC-032, ACC-057 and SEC-007 (the check
  `if op == context.qualified("get-secret.v1")`, `:762`), then `routing` (`:765`, `fn routing` at
  `:514`), which gives ACC-033's target: `answer.v1` falls to its last arm, the Conduit's own
  target where it is not retired, so any target passes today.
- `access::may_inspect` (`access.rs:491`) holds rows 6, 6A and 7 of ACC-033 in its first lines:
  the owner of a Resource or Agent that is not a template, where that owner is not retired, and
  `management` otherwise.
- `explain.rs` holds `REFUSAL_CAUSES` (`:21`), each row checked against ACC-056's table in
  `spec/conventions/access.yaml` by the test `refusal_causes_match_spec_table_verbatim`
  (`engine/crates/conformance/src/checks.rs`); that table has PND-009's row, which
  `REFUSAL_CAUSES` lacks. `acc033_target` (`:350`) describes ACC-033's row for tier 1 and has no
  arm for `answer.v1`.
- `records::handles` (`records.rs:8`) lists the operations `records::execute` (`:213`) runs, and
  `records::preflight` (`records.rs:30`) gives their invalid and conflict answers as a `Rejection`
  (`explain.rs`), whose `diagnostics` may carry a specific diagnostic. `invoke_inner` returns an
  error for an operation whose implementation is the engine's and that neither `lifecycle` nor
  `records` handles (`requests.rs`, "engine operation belongs to its lifecycle or records
  transition").
- `purge.v1` (`records.rs`, the branch `v["operation"] == c.qualified("purge.v1")`) removes
  records only; `State` (`lib.rs:76`) holds no pending action.
- Retirement sets an instance's `state` to `retired` in `retirement.rs:221`
  (`object["state"] = json!("retired");`) and then writes the `object-change` record whose action
  is `retired` through `event`, whose last argument is merged into the record's `data`.
- A set-up.v1 invocation is written to `state.invocations` at `requests.rs:1388`
  (`let i = if let Some(i) = existing {`), and its `invocation-dispatched` record follows at
  `:1400`.

**When this order starts.** Orders 047 and 047b must be merged to `main` first. Before you change
anything, check from the repository root that you find what they give, and stop, change nothing
and report if any check prints otherwise:

```bash
grep -c 'fn setup_action_follows_the_latest_setup' engine/crates/core/src/pending.rs
grep -c 'fn actions_raised_by_scope' engine/crates/core/src/pending.rs
grep -c '"raise-setup-action"' engine/crates/core/src/records.rs
grep -c '"list-pending-actions"' engine/crates/core/src/records.rs
test -f conformance/conventions/setup/action-by-scope/case.yaml; echo "case $?"
grep -c '("answer.v1", "047c", "PND-009")' engine/crates/core/src/requests.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'counted in the `unanswered` key of its own results'
grep -c '^## Record' docs/work-orders/047-build-setup-action.md
grep -c '^## Record' docs/work-orders/047b-build-actions-raised-by-scope.md
```

They print `1`, `1`, `1`, `1`, `case 0`, `1`, `1`, `1` and `1`. Orders that land beside this one
(046b, 048a-1, 050 and others): whichever lands second keeps both changes and writes
`engine/STANDING.md` again with the tool. This order does not depend on S0b: it declares the
pending-action map itself.

**Who decided what.** PND-003, PND-006, PND-009 and PND-012, ACC-033's row 17 and ACC-056's order
decide what an answer is, who may give it, through which target, and what survives a purge.
Everything else here is the coordinator's, each open to the owner's strike ([ruling
48](../proposals/ruling-48/README.md)): the stand of decision 1 on the ID of a redispatched
setup's new action, which answers 047's decision 10, with its form, an answer that belongs to the
ending it answered, through `dispatched`; storing an expiry at a further setup or a retirement,
so that it stays answered; the state's map, its name and its form; reading "on the Interface's own
behalf" as an established Interface with no `serving`; reading PND-009's raised-at test for an
answer as the holder test of 047b's decision 1 without its time test, so that it holds for an
answered or expired action, as PND-009's last sentence asks; placing that test in ACC-056's step
(3), after SEC-007, for an unknown action too; routing an action whose subject no longer exists to
`management`; the answer's record, apart from the request's record (REC-002, PND-004): its
engine-named type `pending-action-answered`, which `record.v1` lets the engine name, its members,
and its place: after the records of the request it accompanies that are written by then (for
answer.v1, after `finish`'s `invocation-ended` and `request-ended`; for a setup's dispatch, right
after `invocation-dispatched`; for a retirement, right after the `object-change` whose action is
`retired`); the request record holds `pendingAction` only; for a retirement, the record that names
the action under PND-012 is the `object-change` record whose action is `retired`, written by the
retire.v1 invocation as it retires the subject, and not that invocation's `invocation-dispatched`,
which comes before the retirement is decided (the coordinator's reading of PND-012's 'its
record'); under DL8, extending increment 2's decision 7 (accepted by ruling 58, point 1) by one
documented type, since PND-004 keeps a record of an answer distinct from a record of a request,
that type as the seventh in DL8's row, the runner's list and the profile's `record_types` (decision 12); the diagnostics'
words of the invalid and the conflict; tier 1's words for row 17; the decisions' names; leaving to later orders the refusal of a Conduit opened listing `answer.v1` to a consumer
that is not an Interface, and withholding code and fingerprint values from answer records on wire
reads; and the case, its route, IDs and times. This order also adds the pending-action map that
plan 117 gives S0b to declare. That is the coordinator's choice, open to the owner's strike, and
S0b, when written, finds the map already declared. Apply them as written and report them under
`decisions` as the coordinator's. Rulings 80 to 82 and the targeted review's
[section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists) put nothing on
047c; nothing here touches sign-in.

**Read first:**

1. In `spec/`: PND-001, PND-002, PND-003, PND-004, PND-006, PND-009 and PND-012
   (`spec/promises/pending.yaml`); ACC-033 (its rows 6, 6A, 7 and 17) and ACC-056, with its table
   of causes (`spec/conventions/access.yaml`); REC-004 (`spec/conventions/records.yaml`); OPS-012
   and OPS-023 (`spec/conventions/operations.yaml`); DL19 (`spec/implementation-defined.yaml`);
   `answer.v1` (`spec/core/operations.yaml`); `pending-action.v1`, `refusal.v1` and `record.v1`
   (`spec/core/results.yaml`); the rows pending action, raised at and setup action of
   `spec/definitions.md`.
2. [Order 047](047-build-setup-action.md) and [order 047b](047b-build-actions-raised-by-scope.md),
   whole, and their Records.
3. Order 046a's decisions 1 (ACC-056's order), 2 and 10 (the portable-case rule), as merged.
4. The cases `setup-action-by-scope` and `e4-reconcile-setup-action-open`, and the e4 fixture's
   `documents.json`: the Fleets `group` and `elsewhere`, the Agents `a` (owner `group`) and `b`
   (owner `elsewhere`), and root's Conduits to each Fleet, made from the core's `root-fleet`
   template under FLEET-005 with the Fleet's name as local name, which list `set-up.v1`,
   `inspect.v1`, `retire.v1` and `answer.v1`.
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.35 to 0.5k changed lines of Rust with the unit test, one new case, and five
places in `engine/CONFORMANCE.md`.

## The decisions it applies

1. **Answers are stored, each for the ending it answered, and the stand on a redispatch.**
   - **The store.** Add to `State` (`lib.rs`) the member `pending_actions`, a
     `BTreeMap<String, Value>` keyed by action ID, with
     `#[serde(default, skip_serializing_if = "BTreeMap::is_empty")]` as `tombstones` has, so
     that a state without it reads as before and serializes unchanged. Each value is
     `{"action": A, "dispatched": D}`: `A` the answered action in `pending-action.v1`'s form, its
     evidence whole and its `answer` set; `D` the `dispatched` value (047's decision 1) that the
     action's set-up.v1 invocation held when the answer was stored, or `null` where it held none.
     The answer's members, in this order: `value`; `actor` and `invocation` where the answer has
     them; `at`. Storing an answer for an ID replaces any entry for it. Nothing removes an entry:
     a purge removes records and nothing else (REC-004; PND-012: answered state survives a
     purge).
   - **The action an ID names, at a time.** In `pending.rs`, add a function that gives, for an ID,
     the state and a time, the action the ID names with its answer, or none. Let `C` be the
     action that 047's computation gives the object of the set-up.v1 invocation whose ID is that
     ID (the invocation's `object`, resolved by ID), where `C`'s ID is that ID, and `D_C` that
     invocation's `dispatched` (or `null`). Then, in this order:
     1. where `C` exists and the store holds no entry for the ID, or one whose `dispatched` is not
        `D_C`: `C`, with the answer `{"value": "expired", "at": X}`, where `X` is its expiry,
        where the time is at or after `X` (047's time test false), and with no answer otherwise;
     2. otherwise, where the store holds an entry for the ID: its `action`, with its answer;
     3. otherwise none.
     Its name and signature are your choice. Where the time is not known, as in decisions 2 and
     3, a caller may ask for the action the ID names without its answer. Cases 1 and 2 choose the
     same action whatever the time, and only the answer of case 1 depends on it. Give the
     function an optional time, and leave `answer` as stored (case 2) or absent (case 1) when
     none is given.
   - **The stand on a redispatch** (047's decision 10). PND-001 gives a setup action "its setup
     invocation's ID", and OPS-012 keeps an invocation's ID across a redispatch; PND-006 says
     both that "A dismissal or expiry for that invocation stays answered and is not recomputed as
     unanswered" and that "A later unsuccessful setup makes a new action". The coordinator reads
     them together so: an answer belongs to the ending it answered, which `dispatched` names. The
     redispatch is a further set-up.v1 dispatch, so it answers the old action `retried` where that
     action is still unanswered and unexpired (decision 6), and otherwise leaves its answer as it
     is. When the redispatch ends failed or unknown, its ending makes a new action (047's
     reading), under the same ID, unanswered, with its expiry counted from the new ending: the
     old action's answer is not recomputed, since it is stored for the old `dispatched`, and the
     new action is not the old one answered again. While the redispatch is in progress, the ID
     names the old action with its stored answer (case 2 above). The ID thus names one action at
     a time, the latest ending's; once a redispatch has ended unsuccessfully, an `answer.v1` that
     names the ID answers the new action, and the old action's answer remains only in its answer
     record (decisions 5 to 7), until a purge. This reading is the coordinator's, open to the
     owner's strike. Report the stand as a question for the owner that says three things: it
     reads PND-006's "for that invocation ... not recomputed as unanswered" as "for that ending";
     the literal reading would leave the ID answered after a failed redispatch and raise nothing;
     and either a new ID per ending under PND-001, or a sentence in PND-006 on redispatch, would
     settle it.

2. **Who may answer: PND-009, in ACC-056's step (3).** In `pending.rs`, add a function that says
   whether an instance may answer an action ID, given the state, the established instance, the
   request's `serving` and the context. It is true only where all of these hold: the instance is
   an Interface and not a template; the request names no `serving` (it is the Interface's own
   request); the ID is a string that names an action (decision 1, whatever its answer); and that
   action is raised at the instance by 047b's holder test alone, without its time test: true at
   root, and at another Interface where its current scope holds one of the action's subjects,
   resolved by ID from the action the ID names (for an answered action, its stored subjects;
   PND-009: "For an action already answered, the raised-at test uses its retained subjects and
   the Interface's current scope"). Split 047b's function in two to reuse that holder test, the
   time test then the holder test, without changing what it gives. In `access::evaluate_path`,
   right after the SEC-007 check and before `routing`, where `op` is the core's `answer.v1` and
   the function gives false, return `Verdict::reason("refused", "PND-009")`. An unknown ID is
   refused here, before any invalid or conflict, and the refusal's tier 1 is the same whether the
   action exists or not (PND-009: "revealing nothing of whether it exists"). In `explain.rs`, add
   to `REFUSAL_CAUSES`, in its place in byte order of rule, PND-009's row of ACC-056's table, its
   text copied verbatim from `spec/conventions/access.yaml`: `Only an Interface acting on its own
   behalf may answer an action raised there, with the required Conduit; other consumers may not
   open a Conduit listing answer.` If 046a's tier-2 explanation of this refusal names anything of
   the action, report it.

3. **The target: ACC-033's row 17.** In `access.rs`, move the first lines of `may_inspect`, which
   choose its route by rows 6, 6A and 7, into one `pub(crate) fn` (state, object, context) that
   gives the target authorizing inspection of that object, and call it from `may_inspect`, whose
   result does not change. Add one more `pub(crate) fn` (state, action, context) that gives the
   ID of the target authorizing inspection of the action's subject: the action's first subject
   resolved by ID (a setup action has one; 047d adds the notice's rule), then the extracted fn,
   or `management` where the subject no longer exists in state. In `routing`, add an arm for
   `answer.v1` that calls it for the action the ID names and resolves that ID in `view`, as the
   other arms resolve theirs; decision 9's bullet calls the same fn. Decision 2 has already refused an ID that names no action. A Conduit to any
   other target then reports refused under ACC-033, as for every operation. In `explain.rs`'s
   `acc033_target`, add an arm for `answer.v1` that gives, without resolving any name, `the
   target that authorizes inspection of the action's subject`; the test
   `acc033_target_forms_do_not_resolve_hidden_owner_names` keeps passing.

4. **Invalid, then conflict.** In `records::preflight`, for `answer.v1`, at the step's evidence
   time:
   - where the kind of the action the ID names does not accept the answer through `answer.v1`:
     invalid under PND-009. The kinds and the answers `answer.v1` accepts are a table in
     `pending.rs` with one row, `setup` and `dismissed`; 047d and 049a-2 add theirs. The
     `Rejection` carries one specific diagnostic, set in the Rejection's `diagnostics`, as
     register.v1's preflight sets its own (records.rs, `rejection.diagnostics = Some(`),
     `{"ruleId": "PND-009", "level": "error", "message": {"text": M}}`, where `M` is
     `{tool} reports invalid under PND-009: answer.v1 answers a setup action only dismissed; a further set-up.v1 answers it retried, and retire.v1 or a cascade's retirement answers it retired.`
     and `{tool}` the request's tool (the route's `tool`);
   - otherwise, where that action has an answer (decision 1, `expired` included): conflict under
     PND-003, with the specific diagnostic whose text is
     `{tool} reports conflict under PND-003: action {ID} is already answered {value}; an answered or expired action takes no further answer.`
     where `{value}` is its answer's `value`.
   Invalid comes before conflict (OPS-023). No row is added to `REASON_CAUSES`.

5. **Dismissed, through `answer.v1`.** Remove the entry `("answer.v1", "047c", "PND-009")` from
   C11's stub table in `requests.rs`, keeping `sign-in.v1`'s, and remove the arm
   `"PND-009" => Some(format!(` from `reject_diagnostics`, which no reason now needs. Add
   `answer.v1` to `records::handles`, and a branch for it in `records::execute`, at the step's
   evidence time:
   - store the answer `{"value": V, "actor": the invocation's caller, "invocation": its ID, "at": the time}`
     for the action the ID names (decision 1), with the `dispatched` its set-up.v1 invocation
     holds, `V` the request's `answer`;
   - end the invocation through `requests::finish` with outcome `succeeded` and outputs `{}`
     (answer.v1 "has no outputs and dispatches no hook or model work");
   - then add to the `data` of the `invocation-ended` record that `finish` wrote for this
     invocation, in `t.records` and in `t.state.records`, as `link_access` updates a record in
     both, the member `pendingAction`, `[ID]`, and nothing else of the action: a request record
     holds action IDs only (REC-002, PND-012);
   - then write one record of the answer, apart from the request record (REC-002, PND-004),
     through `requests::record`: its type `pending-action-answered`, its `data`
     `{"pendingAction": [ID], "answer": A, "invocation": the answering invocation's ID}`, `A` the
     answered action with its full evidence, as `record.v1` says ("answer holds the recorded
     action and its full evidence (PND-012)"). Decisions 6 and 7 write the same record, with
     `invocation` where there is one;
   - push one decision named `answer-pending-action`, citing `PND-003`, `PND-006`, `PND-009` and
     `PND-012`, with `DL19`, and the facts `{"action": ID, "answer": V}`.

6. **Retried, by a further setup.** In `pending.rs`, add a function that a setup's dispatch and a
   retirement call, given the state, an object, the time, the value (`retried` or `retired`) and
   the answering invocation, before either changes the object or the invocation. Where the
   object's action `C` (decision 1) exists and the store holds no entry for its ID with `D_C`,
   it stores `C`, with `dispatched` `D_C`:
   - where the time is before its expiry, answered
     `{"value": V, "actor": the answering invocation's caller, "invocation": its ID, "at": the time}`,
     and returns the answered action;
   - otherwise answered `{"value": "expired", "at": X}`, with no actor or invocation, and
     returns none, so that the expiry stays answered once the redispatch drops the old ending
     (PND-006); an expiry is no request's answer, and no record names it.
   Otherwise it does nothing and returns none. In `requests.rs`, where a set-up.v1 invocation is
   written to `state.invocations` (`let i = if let Some(i) = existing {`), and at each other place
   where 047 sets `dispatched`, call it with `retried` and the new invocation, before the write,
   since a redispatch overwrites the old invocation and its `dispatched`, for the object: the
   instance that the request's `arguments.instance` names, resolved in state with
   `access::object`, as lifecycle's `named` resolves it (a fresh invocation has no `object` yet);
   at a redispatch, the old invocation's `object`. Since the function is called before the
   write, put `"pendingAction": [ID]`, only, into the `json!` that `record` receives for
   `invocation-dispatched` (requests.rs, `"invocation-dispatched",` after
   `link_access(t, &id);`) where the function returned an action, rather than editing the record
   afterwards; where it did, write decision 5's `pending-action-answered` record right after it, and push one decision
   named `answer-setup-retried`, citing `PND-003`, `PND-006` and `PND-012`, with `DL19`, and the
   facts `{"action": ID}`. A set-up.v1 request that is not dispatched, a repeat answered from its
   retained outcome among them, answers nothing. Report every place where a set-up.v1 invocation
   is written.

7. **Retired, by retirement.** In `retirement.rs`, right before `object["state"] =
   json!("retired");`, call decision 6's function for the object with `retired` and the
   retirement's invocation. Where it returns an action, pass the member `pendingAction`, `[ID]`,
   only, in the `extra` of the `event` that writes the `object-change` record whose action is
   `retired`, write decision 5's `pending-action-answered` record right after that record, and
   push one decision named `answer-setup-retired`, citing `PND-003`, `PND-006` and `PND-012`,
   with `DL19`, and the facts `{"action": ID}`. Report every other place where an instance's
   `state` becomes `retired` (a cascade's member, a reconciled retirement), and call it there too.
   A Conduit has no setup action.

8. **An answered action is not raised.** In 047b's function that says whether an action is raised
   at a holder at a time, an action that decision 1 gives an answer at that time is not raised;
   047b's lists, counts and `status.pending` follow. Nothing else in 047b's functions changes.

9. **A unit test pins the store.** In `pending.rs`'s tests, add one test,
   `stored_answers_follow_their_ending`, that builds `State` values by hand, as 047's and 047b's
   tests do, and asserts with `assert_eq!` each result below. Let T be `2026-11-09T12:30:00Z`,
   the objects those of 047b's test (root `r`, the Interface `v` with `c1` open to the Fleet `f`,
   the Agent `a1` of `f`, the Pool `p` and the Resource `x` of `p`), and `s1` a setup of `a1`
   ended `failed` at T with `dispatched` 1, as 047's first bullet builds one.
   - No entry: the action `s1` names at T+1s has no answer; at its expiry,
     `2026-11-16T12:30:00Z`, its answer is `{"value": "expired", "at": "2026-11-16T12:30:00Z"}`.
   - Dismissed by `r` through invocation `d1` at T+1h: the action `s1` names has that answer, at
     T+1h and at T+8d (it does not become expired); it is not raised at `r` or `v` at T+1h.
   - Decision 2's test: true for `r` and for `v` on `s1`, unanswered, dismissed and expired alike;
     false for `v` once `c1` is closed, for `a1`, for `r` with a `serving`, and for an ID that
     names nothing.
   - Decision 3's target, through its fn (state, action, context), for `s1`: `f`; with `f` retired, `management`; for an action whose
     subject is `x`, `p`; for a subject removed from state, `management`.
   - Retried: a further setup `s2` of `a1` dispatched at T+1h by `r`: `s1` answered `retried`,
     actor `r`, invocation `s2`; with `s2` then ended `failed` at T+2h, `s2` names a new action
     with no answer and `s1` still names its answered action.
   - Retired: `a1` retired by invocation `t1` at T+1h, with `s1` unanswered: `s1` answered
     `retired`; with `s1` dismissed before, it stays dismissed.
   - The stand: `s1` dismissed at T+1h, then redispatched at T+8d (`dispatched` 2, in progress):
     `s1` names the dismissed action; with the redispatch then ended `failed` at T+8d (`ended`
     set): `s1` names a new action with no answer and expiry `2026-11-24T12:30:00Z`, raised at
     `r` at T+8d+1s.
   - Expired, then redispatched at T+8d: `s1` is stored `expired`, with no actor; while the
     redispatch is in progress `s1` names that answer; once it ends `failed` at T+9d, `s1` names
     a new action with no answer and expiry `2026-11-25T12:30:00Z`.
   - After `state.records` is emptied, as a purge empties it: each answer above is unchanged.
   - The accepted answers: `setup` accepts `dismissed` and neither `retried`, `retired` nor
     `expired`.

   Write the values as `json!` values; members a test does not need may be left out.

10. **One portable case, through the e4 fixture.** Add
    `conformance/conventions/setup/action-answers/case.yaml`, ID `setup-action-answers`,
    `layers: [convention, interface]`, `promises: [PND-009, PND-003, PND-006, PND-012]`, title
    `Answer a failed setup's action dismissed, retried and retired, refuse or conflict otherwise, and keep the answer across a purge`,
    `profile: first-build` and `initial` `{"recipe": "empty"}`. Write it as JSON with two-space
    indentation and a final newline, its members in the order of `setup-action-by-scope`, as
    order 046a's decision 11 writes its cases. Its `files` are those of `setup-action-by-scope`.
    It must meet 046a's portable-case rule as it stands. Below, ROOT is root's ID,
    `00000000-0000-4000-8000-000000000005`; a new step's evidence is its time, `ids` the 100
    strings `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
    40000 + 100n + k, for k = 0 to 99, `revisions` the 16 strings `answers-` followed by the
    decimal of n, a `-` and the decimal of j, and `suffixes` the 16 strings `s` followed by the
    four-digit, zero-padded decimal of 100n + j, for j = 0 to 15, where n is the step's number; a copied step keeps its own evidence; `META/x` is the
    `_meta` key `x` under the prefix that `spec/names.yaml` sets, written as every case writes
    it; and `U` is the JSON pointer `/reply/_meta/` followed by that key for `unanswered`, its `/`
    written `~1`, as 047b's case writes it. "Root asks T with A" is an `invoke` step whose request
    is `{"identity": {"kind": "instance", "id": ROOT}, "tool": T, "arguments": A}`. "A run
    report" for an invocation `$I` is a `setup-report` step whose input is
    `{"kind": "setup-report", "evidence": E, "invocation": "$I", "report": R}`, as
    `e4-reconcile-setup-action-open`'s steps 3 and 4 write theirs. Its steps:
    1. `setup-action-by-scope`'s step 1, `input` and `expect` whole.
    2. `setup-action-by-scope`'s step 2, `input` and `expect` whole.
    3. `setup-action-by-scope`'s step 7, `input`, `expect` and `capture` (`setup`) whole.
    4. `setup-action-by-scope`'s step 8, `input` and `expect` whole.
    5. `setup-action-by-scope`'s step 9 `input`, whole. `expect`
       `{"reply": {"isError": true, "_meta": {"META/outcome": "failed"}}}`.
    6. Time `2026-11-13T12:00:00Z`: root asks `group.answer.v1` with
       `{"action": "00000000-0000-4000-8000-000000049999", "answer": "kept"}`. `expect`
       `{"reply": {"isError": true, "_meta": {"META/reason": "refused"}, "structuredContent": {"diagnostics": [{"ruleId": "PND-009"}]}}}`:
       an ID that names nothing is refused, before the answer that no setup accepts.
    7. Same time: root asks `management.answer.v1` with
       `{"action": "$setup", "answer": "dismissed"}`. `expect` refused as step 6, its `ruleId`
       `ACC-033`: `a`'s action is answered through `group`, which authorizes inspecting `a`.
    8. Same time: root asks `group.answer.v1` with `{"action": "$setup", "answer": "retried"}`.
       `expect` `{"reply": {"isError": true, "_meta": {"META/reason": "invalid"}, "structuredContent": {"diagnostics": [{"ruleId": "PND-009"}]}}}`.
    9. Same time: root asks `group.inspect.v1` with
       `{"object": {"kind": "Agent", "name": "a"}}`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"pending": [{"id": "$setup", "kind": "setup", "subject": [{"kind": "Agent", "name": "a"}], "expiry": "2026-11-19T12:00:00Z"}]}}}}`;
       `capture` `{"a": "/reply/structuredContent/id"}`.
    10. Same time: root asks `group.answer.v1` with
        `{"action": "$setup", "answer": "dismissed"}`. `expect`
        `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}, "absent": [U], "record_facts": [{"type": "invocation-ended", "data": {"pendingAction": ["$setup"]}}, {"type": "pending-action-answered", "data": {"pendingAction": ["$setup"], "answer": {"id": "$setup", "kind": "setup", "subject": [{"kind": "Agent", "name": "a", "id": "$a"}], "expiry": "2026-11-19T12:00:00Z", "answer": {"value": "dismissed", "actor": ROOT, "at": "2026-11-13T12:00:00Z"}}}}]}`,
        with ROOT written as root's ID.
    11. Same time: step 9's request. `expect`
        `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/status/pending", U]}`.
    12. Same time: step 10's request. `expect`
        `{"reply": {"isError": true, "_meta": {"META/reason": "conflict"}, "structuredContent": {"diagnostics": [{"ruleId": "PND-003"}]}}}`.
    13. Time `2026-11-14T12:00:00Z`: root asks `management.purge.v1` with
        `{"before": "2026-11-14T00:00:00Z"}`. `expect` `{"reply": {"isError": false}}`.
    14. Same time: step 10's request. `expect` step 12's: the answer survives the purge.
    15. Same time: step 9's request. `expect` step 11's.
    16. Time `2026-11-15T12:00:00Z`: root asks `group.set-up.v1` with
        `{"instance": {"kind": "Agent", "name": "a"}}`. `expect` `{"reply": {"pending": true}}`;
        `capture` `{"second": "/reply/invocation"}`. The dismissed action stays dismissed.
    17. Same time: a run report for `$second`, `R` `{"kind": "inspect-run", "state": "stopped"}`.
        `expect` `{"reply": {"pending": true}}`.
    18. Same time: a run report for `$second`, `R` `{"kind": "run", "outcome": "failed"}`.
        `expect` `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/unanswered": 1}}}`:
        a later unsuccessful setup makes a new action.
    19. Same time: step 16's request. `expect`
        `{"reply": {"pending": true}, "record_facts": [{"type": "invocation-dispatched", "data": {"pendingAction": ["$second"]}}, {"type": "pending-action-answered", "data": {"pendingAction": ["$second"], "answer": {"id": "$second", "kind": "setup", "answer": {"value": "retried", "actor": ROOT, "at": "2026-11-15T12:00:00Z"}}}}]}`;
        `capture` `{"third": "/reply/invocation"}`.
    20. Same time: a run report for `$third`, `R` as step 17's. `expect` step 17's.
    21. Same time: a run report for `$third`, `R` as step 18's. `expect` step 18's: the retried
        action is not counted.
    22. Same time: root asks `group.answer.v1` with
        `{"action": "$second", "answer": "dismissed"}`. `expect` step 12's.
    23. Same time: root asks `group.retire.v1` with
        `{"instance": {"kind": "Agent", "name": "a"}}`. `expect`
        `{"record_facts": [{"type": "object-change", "data": {"action": "retired", "pendingAction": ["$third"]}}, {"type": "pending-action-answered", "data": {"pendingAction": ["$third"], "answer": {"id": "$third", "kind": "setup", "answer": {"value": "retired", "actor": ROOT, "at": "2026-11-15T12:00:00Z"}}}}]}`.
    24. Same time: root asks `group.answer.v1` with
        `{"action": "$third", "answer": "dismissed"}`. `expect` step 12's.
    25. Time `2026-11-16T12:00:00Z`: root asks `elsewhere.set-up.v1` with
        `{"instance": {"kind": "Agent", "name": "b"}}`. `expect` `{"reply": {"pending": true}}`;
        `capture` `{"fourth": "/reply/invocation"}`.
    26. Same time: a run report for `$fourth`, `R` as step 17's. `expect` step 17's.
    27. Same time: a run report for `$fourth`, `R` as step 18's. `expect`
        `{"reply": {"isError": true, "_meta": {"META/outcome": "failed"}}}`.
    28. Time `2026-11-23T12:00:00Z`, the expiry of `$fourth`'s action: root asks
        `elsewhere.answer.v1` with `{"action": "$fourth", "answer": "dismissed"}`. `expect`
        step 12's: an expired action takes no answer.

    Steps 6 to 28 are new; the route is new and has not been run, since orders 047 and 047b are
    not yet built. Before running it, check that no ID repeats across the case's steps' `ids`.
    If 046a's runner check refuses the case all the same, if a step copied from
    `setup-action-by-scope` gives a reply other than that case states, if a setup of `a` or `b`
    asks for a report other than the two that steps 17 and 18 send, or if a step's reply differs
    from what it states, stop and report the error: do not change the expectations to pass it,
    and do not change the inputs beyond what this decision says. No expectation is generated
    from the engine's output. The coverage index entry holds `id`, `case`
    (`conventions/setup/action-answers/case.yaml`), `promises` and, where its neighbours in that
    piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write it in a
    `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
    `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it
    into `conformance/conventions/setup/index.yaml` in ID order and leaves `manifest.yaml` byte
    for byte as it was.

11. **The shell test.** In `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`
    (`engine/crates/shell/src/tests.rs`), remove the entry for `management.answer.v1` from the
    loop over the unbuilt operations, which keeps `system.sign-in.v1` alone, and make the loop's
    expected `ruleId` `PND-010`. After the loop, before the final assertion, send root's
    `management.answer.v1` with
    `{"action": "00000000-0000-4000-8000-000000009999", "answer": "dismissed"}` and assert that
    its `_meta` reason is `refused` and `structuredContent.diagnostics[0].ruleId` is `PND-009`.
    The final assertion, that no record is `invocation-dispatched`, stays. Nothing else in the
    file changes.

12. **`engine/CONFORMANCE.md`, DL8's record types and `engine/STANDING.md`.** Find each sentence below by its text
    read across line breaks, as 047 and 047b leave it:
    - In DL19's row in "Choices still required", the sentence "`answer.v1` answers unavailable
      under PND-009, with the text "not built yet, order 047c" LINK, and no other kind is held,
      so no other duration is chosen." becomes, in the same line:

      ```text
      `answer.v1` answers a setup action dismissed (order 047c) PLINK; a further set-up.v1 answers it retried and its retirement retired, and once its expiry passes it is expired; each answer is kept in the engine's state, which no purge touches, for the ending it answered, so that a redispatch that ends unsuccessfully makes a new action under the same invocation ID. No other kind is held, so no other duration is chosen.
      ```

      where `LINK` is the row's link to `requests.rs`, which goes, and `PLINK` a link to
      `pending.rs` in parentheses, written as `LINK` is but naming that file.
    - "The shell also passes answer.v1 and sign-in.v1 to the core, which answers each
      unavailable, naming the order that builds it (047c and 048a; OPS-005)." becomes "The shell
      also passes answer.v1 to the core, which answers pending actions with it (order 047c), and
      sign-in.v1, which the core answers unavailable, naming the order that builds it (048a;
      OPS-005)."
    - "`sign-in.v1` and `answer.v1` answer unavailable, naming orders 048a and 047c respectively
      (rows DL18 and DL19 in "Choices still required")." becomes "`sign-in.v1` answers
      unavailable, naming order 048a (row DL18 in "Choices still required")."
    - In the section "Refusals and reasons (order 046a)", "The unavailable stubs of `answer.v1`
      and `sign-in.v1` now carry diagnostics but remain unbuilt: orders 047c and 048a-2." becomes
      "The unavailable stub of `sign-in.v1` now carries a diagnostic but remains unbuilt: order
      048a-2."
    - In DL8's row "DL8: records" in "Implemented choices", "Six event types: access-decision,
      request-ended, invocation-dispatched, invocation-ended, provider-call, object-change."
      becomes "Seven event types: access-decision, request-ended, invocation-dispatched,
      invocation-ended, provider-call, object-change, pending-action-answered (order 047c, an
      answer to a pending action, apart from the request's records under PND-004 and REC-002)."
      Nothing else in the file changes. With it, add `pending-action-answered` to the runner's
      list in `engine/crates/conformance/src/checks.rs` (the six-type list in `run`) and to
      `record_types` in `conformance/profiles/first-build.yaml`: the runner fails any step whose
      records hold an undocumented type.
    - In `conformance/README.md`, the sentence "The first-build record vocabulary is
      `access-decision`, `request-ended`, `invocation-dispatched`, `invocation-ended`,
      `provider-call` and `object-change`." becomes "The first-build record vocabulary is
      `access-decision`, `request-ended`, `invocation-dispatched`, `invocation-ended`,
      `provider-call`, `object-change` and `pending-action-answered` (order 047c, an answer to a
      pending action, apart from the request's records under PND-004)." Nothing else in that file
      changes.
    - After the other decisions, run `python3 tools/promise_standing.py --write`, as AGENTS.md
      asks of a change to a case's `promises` and to a promise ID in the core's sources. Report
      the rows of PND-003, PND-006, PND-009 and PND-012 and the totals; the totals change only by
      what those rows change.

13. **The declaration.** `docs/work-orders/047c-declared.yaml` holds one `added` shape named
    `setup-action-answers` for that case. The coordinator wrote it with this order; check that it
    matches and change it only to make it so.

14. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or record
    changes. No kind other than setup is held or answered, no `pendingAction` key is set on a
    result, and nothing is acknowledged or kept: 047d, 048a-2 and 049a-2. No CLI command (051b),
    shell code or tool list changes. Not this order's, so report nothing about them: PND-009's
    other refusal, of an `open.v1` whose Conduit lists `answer.v1` to a consumer that is not an
    Interface, which ACC-056 places in its step (6) and the coordinator places in a later order
    beside PND-010's refusal of the same form; and withholding code and fingerprint values from
    answer records on wire reads (PND-004), which no kind this order holds has. Report as
    findings, for the coordinator: decision 1's stand, as a question for the owner; and any place
    where PND-003, PND-006 or PND-009 left you a choice this order does not settle.

## Scope

Change only `engine/crates/core/src/pending.rs` (decisions 1, 2, 4, 6, 8 and 9), `lib.rs` (the
state's map), `access.rs` (decision 2's check, decision 3's fn and `routing`'s arm), `explain.rs`
(the cause's row and `acc033_target`'s arm), `records.rs` (`handles`, `preflight` and `execute`
for `answer.v1`), `requests.rs` (the stub's entry and message, and decision 6's call),
`retirement.rs` (decision 7), `lifecycle.rs` only where decision 6 or 7 finds a set-up.v1
invocation written or an instance retired there, `engine/crates/shell/src/tests.rs` (decision
11), `engine/CONFORMANCE.md` (five places), `engine/crates/conformance/src/checks.rs` (the
six-type list in `run`, decision 12), `conformance/profiles/first-build.yaml` (`record_types`,
decision 12), `conformance/README.md` (that sentence, decision 12), `engine/README.md` (its sentence listing `State`'s
members gains `pending_actions`, and nothing else), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/setup/index.yaml` and the new
`conformance/conventions/setup/action-answers/case.yaml`. `docs/work-orders/047c-declared.yaml`
comes with this order; change it only as decision 13 says. `conformance/manifest.yaml` holds the
new entry only until the pieces tool moves it, and ends unchanged. Not `spec/`, `IN_FORCE.md`,
`tools/`, `packages/`, the shell's code, the experiments, the case schema,
`conformance/internal-cases.yaml`, `conformance/expected-failures.yaml`, any other case or
fixture, or any record: report any defect as a finding. `experiments/011-short-form-spike/`
changes only if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- pending::tests
cargo test --release -p conduit-core --lib -- explain::tests
cargo test --release -p conduit-conformance refusal_causes_match_spec_table_verbatim
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. setup-action-answers
cargo run --release -p conduit-conformance -- .. setup-action-
cargo run --release -p conduit-conformance -- .. e4-reconcile-setup-
cargo run --release -p conduit-conformance -- .. setup-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 3 passed and 0 failed: 047's test, 047b's and this order's. The pieces tool
prints `Nothing to move.` The runner commands end `1 cases passed`, `3 cases passed` (this case,
`setup-action-at-root` and `setup-action-by-scope`), `2 cases passed` with no expected failure,
and one more case than 047b's Record reports for `setup-` (report the count). The shell test
passes. If any other case, or any shell, CLI or experiment test, fails because `answer.v1` is no
longer a stub or because an answered action is no longer raised, stop and report it: do not
change it. The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decisions 5 and 7. From `engine/`, one at a time:

```bash
cp crates/core/src/requests.rs target/requests.rs.fixed
git show HEAD:engine/crates/core/src/requests.rs > crates/core/src/requests.rs
cargo run --release -p conduit-conformance -- .. setup-action-answers
cp target/requests.rs.fixed crates/core/src/requests.rs
cp crates/core/src/retirement.rs target/retirement.rs.fixed
git show HEAD:engine/crates/core/src/retirement.rs > crates/core/src/retirement.rs
cargo run --release -p conduit-conformance -- .. setup-action-answers
cp target/retirement.rs.fixed crates/core/src/retirement.rs
cargo run --release -p conduit-conformance -- .. setup-action-answers
```

The first two runner commands must each fail, printing `FAIL 0 cases passed; 1 failed`: the
first at step 6 with an error beginning `ACC-056: a refused reply holds`, because `HEAD`'s
`requests.rs` still gives PND-009 the stub's message; the second with an error at
step 23 that names the missing ordered record. After both restores the third ends
`1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older modification time, so
cargo would not rebuild the crate. Git ignores `target/`. If `HEAD`'s file does not build against
the rest of your change, revert that decision's lines by hand in the copy instead, and say so.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/047c-declared.yaml; echo "exit $?"
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/inventory engine/crates/shell/src/lib.rs spec IN_FORCE.md tools packages docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/047c-declared.yaml: 1 shapes.`,
then `Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.
`promise_standing.py --check`, `check_model_account.py` and `check_roadmap.py` each end
`Errors: 0`. The unit tests, run with the interpreter that has PyYAML, end `OK`; report the count.
The `git status` command prints nothing.

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
grep -c 'fn stored_answers_follow_their_ending' engine/crates/core/src/pending.rs
grep -c '"answer-pending-action"' engine/crates/core/src/records.rs
grep -c '"answer-setup-retried"' engine/crates/core/src/requests.rs
grep -c '"answer-setup-retired"' engine/crates/core/src/retirement.rs
grep -c '"pending-action-answered"' engine/crates/conformance/src/checks.rs
grep -c '"id": "setup-action-answers"' conformance/conventions/setup/index.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'answers a setup action dismissed (order 047c)'
grep -c '^| PND-009 | convention | must | 1 | 0 | outside | core, shell | case passes |$' engine/STANDING.md
grep -c '^| PND-012 | convention | must | 1 | 0 | outside | core | case passes |$' engine/STANDING.md
```

Each of these prints 0:

```bash
grep -c '"answer.v1", "047c"' engine/crates/core/src/requests.rs
cat engine/CONFORMANCE.md engine/crates/core/src/requests.rs engine/crates/shell/src/tests.rs | grep -c 'not built yet, order 047c\|"047c"'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'answer.v1 and sign-in.v1 to the core, which answers each'
git diff HEAD --stat -- conformance/expected-failures.yaml conformance/conventions/setup/action-at-root conformance/conventions/setup/action-by-scope | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the rows decision 12 names with the totals, the function names and signatures of
decisions 1 to 4 and 6, every place where a set-up.v1 invocation is written (decision 6) and where
an instance's `state` becomes `retired` (decision 7), and what tier 2 of decision 2's refusal
holds.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `047c-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
