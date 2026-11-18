# Work order 047b (E6b): where pending actions are raised: scoped Interfaces, disclosure, `list.v1`'s `pending` output and the `unanswered` key

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
an order holds one behavior and stays small). This is the second: "047b, where actions are raised:
root and each Interface whose scope holds a subject, evidence disclosed only as inspection allows,
`list.v1`'s `pending` output and the `unanswered` key". [Order 047](047-build-setup-action.md)
computes a failed setup's action and raises it at root's own inspection alone. 047c builds
`answer.v1` with stored answers; 047d the notice kind; 048a-2 sign-in as a kind of pending action,
PND-005 and `SignInUndone`. This order builds none of them.

**What it builds.** One behavior: the place where an action is raised. PND-002: an action that "is
unanswered and has not expired" is "raised at root and every Interface whose scope contains a
subject: the Pool or Fleet itself, the owner of an instance, or an owner of either end of a
Conduit. Scope is computed from the Interface's open inspection Conduits (definitions.md). It is
never raised at an Agent." Its observable says what raised means: "listed in list.v1's pending
output and in inspect.v1's status.pending for the subject, for such an Interface. Each result of
that Interface's own request carries `META/unanswered`, the number raised
there, only when nonzero. Another Interface sees only subjects and evidence about objects it may
inspect, with no count of the rest. Root sees all subject objects. Codes and fingerprints remain
withheld under PND-004 even from root's tool results. Agents' list and inspect results carry no
pending actions." (The quote is shortened so: it writes the key as `META/unanswered`, the `_meta`
key `unanswered` under the prefix that `spec/names.yaml` sets, as decision 8 writes such keys.)
PND-004 adds: "Other evidence is disclosed only to Interfaces at which the action
is raised, limited by inspection under PND-002." The only kind the engine holds after 047 is the
computed setup action, so this order raises that kind everywhere PND-002 puts it, through rules
written for any kind, which 047c and 047d then use for theirs. [Spec batch 1's
plan](../proposals/spec-batch-1/areas/pending-actions.md) lists the vectors `raised-scoped` and
`unanswered-hint` (its cases 15 and 23); this order's one case covers both for the setup kind.

**What order 047 brings, which this order assumes.** Order 047 is written on `main` and not yet
built; this order starts only once it is merged (below). From 047, as its Record names them:

- `engine/crates/core/src/pending.rs`, with `mod pending;` in `lib.rs`, holding 047's two
  functions: the setup action of an object, computed from the state whatever the time, in
  `pending-action.v1`'s form `{"id", "kind": "setup", "subject": [{"kind", "name", "id"}],
  "evidence", "expiry"}`, every evidence item of kind `fact` (047's decision 2); and the test
  whether an action is raised at a time, true before its expiry and false at it and after. Their
  names are those 047's Record gives. Its unit test `setup_action_follows_the_latest_setup`.
- The members `ended` and `dispatched` on set-up.v1 invocations (047's decision 1), and
  `setup_cause`, the diagnostics a failed or unknown setup first ended with, which `finish` sets
  where they exist and a reconciliation moves there from `cause` (047's Record); only those two
  functions read them.
- In `records::execute`'s `inspect.v1` branch, right after `status["unreconciled"]` is set,
  `status["pending"]`, a list of the object's one raised setup action, set where the object is not
  a template and the inspection's caller and sender are both root's ID, with the decision
  `raise-setup-action` (047's decision 3); `status.pending` left out otherwise. Root is the object
  whose `from` is the core's `root`, through the lookup that 047 reused or exposed. Where a Type
  records outputs, the record writer in `requests.rs` removes `status.pending` from an
  `inspect.v1` invocation's recorded outputs (047's Record).
- The portable case `setup-action-at-root` in `conformance/conventions/setup/`, the expected
  failure `e4-reconcile-setup-action-open` lifted, `answer.v1`'s stub naming 047c, and DL19's row
  in `engine/CONFORMANCE.md` beginning `| DL19 | Partly built. A setup action (PND-006) expires
  seven days`, whose sentence "It is raised only at root's own inspection of its instance, in
  `status.pending`; an Interface's scope, `list.v1`'s `pending` output and the `unanswered` key
  come with order 047b." this order replaces (decision 8).

Where 047 merged otherwise than its order says, apply the nearest sound reading of each decision
below to what merged, and say so.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`408d5f17`, before order 047; find each place by the text quoted with it.

- Scope is computed in one place, `inventory::read` (`engine/crates/core/src/inventory.rs:373-391`:
  the open test `if !access::open(p) { continue; }`, then the block that begins
  `if consumed && let Some(target) = access::object(s, &p["target"])`, where `consumed`, computed
  above at `:346` as `access::id(&p["consumer"]) == &o["id"]`, says the holder is the consumer): an
  open Conduit (`access::open`: kind `Conduit`, `open` true, `state` `ready`) consumed by the
  holder, whose target is a Pool or a Fleet whose name is not `system`, and whose `operations`
  hold the core's `inspect.v1`. The condition sits inline in `read`, mixed with building the
  inventory's entries; it builds the inspection's `inventory.scope` and returns no set another
  module can use.
- `access::may_inspect(state, actor, item, context)` (`access.rs:491`) says whether an actor may
  inspect an object: through the object's owner where it is a Resource or an Agent with a
  non-retired owner, otherwise through `management`, by an open Conduit listing `inspect.v1`.
  `list.v1` already filters its `objects` with it.
- `list.v1` (`records.rs:247`, the branch `v["operation"] == c.qualified("list.v1")`) gives
  `objects`, `packages` and `tombstones`, and no `pending`; its contract's `outputSchema`
  (`spec/core/operations.yaml`, `list.v1`) already declares `pending`, a list of
  `pending-action.v1`.
- A tool result is built by `envelope` (`requests.rs:111`), whose `_meta` holds the keys that
  `mark(c, name)` (`requests.rs:22`) qualifies with the prefix that `spec/names.yaml` sets: the
  outcome or the reason, and the invocation's ID where there is one. `requests::finish` sets the
  transition's `reply` to the ended invocation's result and pushes the same value to
  `request_replies` (`requests.rs:1567-1583`), for callers that wait on it. `lib::step_owned`
  (`lib.rs:327`) runs `explain::first_text` on `reply` and on each of `request_replies`, once all
  is final. The shell sends both to their callers unchanged (`engine/crates/shell/src/lib.rs`,
  the loop `for ended in &t.request_replies` and the `ReplyCell` that holds `t.reply`).
- No result carries an `unanswered` key, and no list carries pending actions.

**When this order starts.** Order 047 must be merged to `main` first. Before you change anything,
check from the repository root that you find what 047 gives, and stop, change nothing and report
if any check prints otherwise:

```bash
test -f engine/crates/core/src/pending.rs; echo "pending $?"
grep -c '^mod pending;' engine/crates/core/src/lib.rs
grep -c 'fn setup_action_follows_the_latest_setup' engine/crates/core/src/pending.rs
grep -c '"raise-setup-action"' engine/crates/core/src/records.rs
test -f conformance/conventions/setup/action-at-root/case.yaml; echo "case $?"
grep -c 'e4-reconcile-setup-action-open' conformance/expected-failures.yaml
grep -c '("answer.v1", "047c", "PND-009")' engine/crates/core/src/requests.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'come with order 047b'
grep -c '^## Record' docs/work-orders/047-build-setup-action.md
```

They print `pending 0`, `1`, `1`, `1`, `case 0`, `0`, `1`, `1` and `1`. Orders that land beside
this one (046b, 048a-1, 050 and others): whichever lands second keeps both changes and writes
`engine/STANDING.md` again with the tool.

**Who decided what.** PND-002, PND-004 and the definitions decide where an action is raised, what
raised means and who sees what. Everything else here is the coordinator's, each open to the owner's
strike ([ruling 48](../proposals/ruling-48/README.md)): reading "an Interface's own request" as one
whose caller and sender are both that Interface, and finding a result's request by the invocation
its `_meta` names, or else by the identity of the request the step decided, or, for a non-invoke
input that carries a `request` with no invocation, by that request's `caller` and `sender`; counting
at the time of the step that gives the result; that an Agent's scope over its own Fleet raises
nothing, as PND-002 says, and that an Interface template is raised at nothing; reading "an owner of
either end of a Conduit" as the owner of the Conduit's consumer or target; that a non-root
Interface's view keeps an action's evidence only where it may inspect every subject, and otherwise
none, since no evidence item names the object it is about; that an action whose view keeps no
subject is neither shown nor counted at that Interface; that the views withhold code and fingerprint
items at every Interface, root included, for every kind; that `list.v1`'s `pending` holds every
action raised at the caller, whatever Pool, Fleet or management the list goes through, and that
`list.v1`'s `kind` argument narrows `objects` only, so that `pending` is the same with or without
it; leaving `pending` and the key out where nothing is raised, as 047 leaves out `status.pending`;
removing `pending` from `list.v1`'s recorded outputs where a Type records outputs, as 047's Record
removes `status.pending` from `inspect.v1`'s; ordering actions by ID; the decisions' names; the
case, its route through order 042's e4 fixture, its IDs and its times; tagging the case with PND-002
alone, since no step asserts PND-004's disclosure, which decision 7's unit test pins, so that
PND-004's row stays `no case` with source `core`; and DL19's new sentence. Apply them as written and
report them under `decisions` as the coordinator's. Rulings 80 to 82 and the targeted review's
[section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists) put nothing on 047b;
nothing here touches sign-in.

**Read first:**

1. In `spec/`: PND-001, PND-002, PND-003, PND-004, PND-006 and PND-012
   (`spec/promises/pending.yaml`); `list.v1` and `inspect.v1` (`spec/core/operations.yaml`);
   `pending-action.v1`, `evidence-item.v1` and `object.v1` (`spec/core/results.yaml`); IFC-025
   (`spec/interface/implementation.yaml`); the rows scope, raised at and inspect of
   `spec/definitions.md`.
2. [Order 047](047-build-setup-action.md), whole, and its Record.
3. The cases `e4-inventory-from-first-start`, `e4-inventory-scope`,
   `e4-reconcile-setup-action-open` and `e4-reconcile-sender-fleet`, under
   `conformance/conventions/inventory/e4/`, and that directory's `documents.json`: the Interface
   `viewer`, the Agent `a` of the Fleet `group`, and the Conduits `viewer-pool` (local name
   `pool`, to the Pool `store`), `viewer-fleet` (`fleet`, to `group`) and `a-self` (`own`, from
   `a` to `group`), each listing `inspect.v1` and `list.v1`.
4. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: S-M**: about 0.25 to 0.4k changed lines of Rust with the unit test, one new case, and one
sentence in `engine/CONFORMANCE.md`.

## The decisions it applies

1. **Scope and where an action is raised.** In `pending.rs`, add a function that says whether an
   action is raised at a holder at a time, for an action in `pending-action.v1`'s form, whatever
   its kind. It is false where 047's test says the action is not raised at that time. Otherwise:
   - where the holder is not an Interface, or is a template (it has `spec`), false: an Agent's
     scope over its own Fleet raises nothing (PND-002: "never raised at an Agent");
   - where the holder is root, true;
   - otherwise, true where the holder's scope holds a subject, and false where it holds none. The
     holder's scope is the set of Pools and Fleets that `inventory::read`'s test above finds for
     it: move that test (`inventory.rs:373-391`) out of `read` into one `pub(crate) fn` in
     `inventory.rs` (holder, Conduit, state, context -> bool), which returns true only where
     `access::open(conduit)` holds, the Conduit's consumer is the holder, and the rest of the
     block's condition holds. Call it from `pending.rs` and from `read`: `read` keeps its
     `continue`, which the administrators block after it also relies on, and calls the fn in
     place of the scope block's condition; `read`'s output is unchanged. Write no second copy of
     the test. A subject, resolved by its `id`
     (`access::object_by_id`; a subject no longer in state counts for nothing), is held where it
     is itself a Pool or Fleet in the scope, where its `owner` is in the scope, or, for a
     Conduit, where the `owner` of its `consumer` or of its `target` is in the scope.

2. **What an Interface sees: the view.** Add a function that gives an action's view at a holder,
   or none. At root, the view is the action with every evidence item whose `kind` is `code` or
   `fingerprint` removed (PND-004: "Codes and fingerprints remain withheld under PND-004 even from
   root's tool results"). At another Interface:
   - its `subject` keeps, in order, only the subjects that the holder may inspect, by
     `access::may_inspect` on the subject's object; where none is kept, there is no view;
   - its `evidence` is the action's, with code and fingerprint items removed, where every subject
     was kept, and `[]` otherwise: no evidence item names the object it is about, so evidence is
     shown only where the holder may inspect everything the action is about. Every item of a
     setup action is about its one subject: each comes from that instance's own `status.setup`,
     `status.unreconciled` or records, which the holder's inspection of the instance shows;
   - every other member is kept as the action holds it.

   No view says how many subjects or items it left out (PND-002: "with no count of the rest").

3. **The actions held, and the count.** Add a function that gives every action the state holds:
   for each object with no `spec`, its setup action where 047's computation gives one, in byte
   order of action ID; no stored kind exists yet (047c and 047d add them here). Add a function
   that gives, for a holder at a time, the views of the actions raised there that have a view,
   in the same order; the number raised at a holder is their count. An action raised at a holder
   whose view is none is neither listed nor counted there.

4. **`status.pending` at each Interface's own inspection.** In the `inspect.v1` branch, replace
   047's test of root by this: where the object is not a template and the inspection's
   invocation has `caller` and `sender` equal to one Interface's ID, set `status["pending"]` to
   the views at that Interface, at the step's evidence time, of the actions whose view's subjects
   include the inspected object's ID, in decision 3's order; leave it out where there are none.
   Push 047's decision `raise-setup-action` for each action listed, as 047 pushes it, its
   promises gaining `PND-004`. An Agent's inspection, or an inspection whose caller and sender
   differ, carries no `status.pending`. At root the result is as 047 gives it.

5. **`list.v1`'s `pending` output.** In the `list.v1` branch, where the invocation's `caller` and
   `sender` are one Interface's ID, set `outputs["pending"]` to decision 3's views at that
   Interface at the step's evidence time, whatever the list's container: every action raised at
   the caller, through any Pool, Fleet or management. The `kind` argument narrows `objects` only:
   `pending` is the same with or without it. This reading is the coordinator's, open to the
   owner's strike, since `list.v1`'s description says only "kind optionally narrows object
   kinds"; report that silence as a finding (decision 11). Leave `pending` out where there are
   none, and for every other caller. Where it is present, push one decision named
   `list-pending-actions`, citing `PND-001`, `PND-002`, `PND-003` and `PND-004`, with `DL19`, and
   the facts `{"actions": [IDs]}`. `objects`, `packages` and `tombstones` do not change. Where a
   Type records outputs, the record writer in `requests.rs` removes `pending` from a `list.v1`
   invocation's recorded outputs, as 047 removes `status.pending` from `inspect.v1`'s, so that no
   action or evidence reaches the shared records.

6. **The `unanswered` key.** After `step_body` gives the transition, beside `explain::first_text` in
   `step_owned` or at the end of `step_body`, mark each tool result the transition gives: `reply`,
   and the `reply` of each of `request_replies`. A tool result is an object holding the member
   `isError`; a `{"pending": true}` reply, a JSON-RPC error and every other reply are not, and are
   left as they are. A tool result's request is the invocation in the transition's state whose ID it
   carries (the entry's `invocation`, or the `_meta` key `invocation` of `reply`); where it carries
   none, as a request that was not dispatched, it is the request of the step's `invoke` input, if
   the step is one, or else the `request` that the step's input carries with a `caller` and a
   `sender`, if it carries one (as an access or registration input may). `Input::Request`'s refused
   lifecycle request is not such a case today: its reply (`lib.rs:674-701`) holds no `isError`, so
   it is not a tool result and gets no key. The result is the Interface's own where that
   invocation's `caller` and `sender` are both one Interface's ID; for the request of an `invoke`
   input with no invocation, where its identity is an Interface's ID and it names no `serving`; and
   for another input's `request` with no invocation, where that request's `caller` and `sender` are
   both one Interface's ID. For such a result, where decision 3's count at that Interface is not
   zero, set the `_meta` key `unanswered` to that count, an integer. Count over the transition's
   state (`transition.state`), the state after the step, at the evidence time of the step's input.
   Read the input's evidence time and request before `step_body` consumes the input. Write the key's
   qualification in `pending.rs`, or in `lib.rs` beside the marking, by the same rule that `mark`
   (`requests.rs:22`) applies: `format!("{}unanswered", context.meta_prefix)` (`Context`'s
   `meta_prefix`, `lib.rs:68`). Do not change `requests.rs` for the key, since the Scope admits it
   only for decision 5's removal, and do not expose `mark`. Set nothing otherwise, and never on an
   Agent's result. A result whose invocation the state no longer holds, or a step whose input
   carries no evidence time, gets no key: report each path by which a tool result reaches either
   case. Report also each non-invoke input whose tool result is found through the `request` it
   carries, and whether that result got the key. Push no decision for the key. Nothing else in any
   reply changes.

7. **A unit test pins the rules.** In `pending.rs`'s tests, add one test,
   `actions_raised_by_scope`, that builds `State` values by hand, as 047's test does, and asserts
   with `assert_eq!` each result below. Build the `Context` that `access::may_inspect` needs as
   an existing unit test of the core builds one, or pass the qualified names the functions need.
   Let T be `2026-11-09T12:30:00Z`. The objects: root `r` (an Interface whose `from` is the core's
   `root`), `management` (a Resource whose `from` is the core's `management`), the Fleet `f`,
   the Pool `p`, the Interface `v`, the Agents `a1` and `a2` (owner `f`) and the Resource `x`
   (owner `p`); `a1` and `x` each with a setup ended `failed` at T, as 047's first bullet builds
   one. The Conduits, each with `operations` holding the core's `inspect.v1` unless stated: `c1`
   from `v` to `f`, open; `c2` from `a2` to `f`, open; `c3` from `v` to `p`, closed (`open`
   false); `c4` from `v` to `p`, open, listing only the core's `list.v1`. Open means kind
   `Conduit`, `open` true and `state` `ready`.
   - At `2026-11-09T12:30:01Z`: at `r`, both actions, `a1`'s and `x`'s, in ID order, each view
     equal to its action; at `v`, `a1`'s alone, its view equal to the action, count 1; at `a2`,
     none; at an Interface with no Conduit, none.
   - With `c3` open: at `v`, both, count 2. At `a1`'s expiry, `2026-11-16T12:30:00Z`: at `r` and
     at `v`, neither.
   - At `2026-11-09T12:30:01Z`, as the first bullet is: a hand-made action `{"id": "n1", "kind":
     "notice", "subject": [a1, x], "evidence": [a code item, a fingerprint item, a fact item],
     "expiry": "2026-11-16T12:30:00Z"}`, subjects in resolved form: at `r`, raised, its view both
     subjects and the fact item alone; at `v` with `c3` closed, raised (its scope holds `a1`'s
     owner), its view subject `[a1]` and evidence `[]`; at `v` with `c3` open, both subjects and the
     fact item alone.
   - At `2026-11-09T12:30:01Z`, as the first bullet is: a hand-made action of kind `notice` whose
     one subject is a Conduit from `a2` to `x`: at `v`, raised (`a2`'s owner `f` is in its scope),
     with no view, since inspecting a Conduit goes through `management`, on which `v` holds no
     Conduit; at `r`, its view the action whole.

   Write the values as `json!` values; members a test does not need may be left out.

8. **One portable case, through the e4 fixture.** Add
   `conformance/conventions/setup/action-by-scope/case.yaml`, ID `setup-action-by-scope`,
   `layers: [convention, interface]`, `promises: [PND-002]`, title
   `Raise a failed setup's action at root and each Interface whose scope holds its instance, never at an Agent`,
   `profile: first-build` and `initial` `{"recipe": "empty"}`. Write it as JSON with two-space
   indentation and a final newline, its members in the order of `setup-action-at-root`, as order
   046a's decision 11 writes its cases. Its `files` are `fixture`, `manifest` and `documents`,
   the paths of `conventions/inventory/e4/`'s `fixture.json`, `manifest.json` and
   `documents.json` from the new directory (`../../inventory/e4/fixture.json` and so on). It
   must meet 046a's portable-case rule as it stands. Below, an `invoke` step's evidence is its
   time and `ids` the 30 strings `00000000-0000-4000-8000-` followed by the twelve-digit,
   zero-padded decimal of 30000 + 100n + k, for k = 0 to 29, where n is the step's number;
   `META/x` is the `_meta` key `x` under the prefix that `spec/names.yaml` sets, written as every
   case writes it; and `U` is the JSON pointer `/reply/_meta/` followed by that key for
   `unanswered`, its `/` written `~1`, as `conventions/explain/agent-results` writes its capture
   pointer under `_meta`. Root's ID is `00000000-0000-4000-8000-000000000005`. Its steps:
   1. `e4-inventory-from-first-start`'s step 1 `input`, whole. `expect`: its `reply`,
      `external_intents` and `required_promises` as that case states them, `state`
      `{"started": true}` and `"record_facts": []`, placed as 046a's `explain-refusal-tiers` step
      1 places it; not its `records` or `absent`.
   2. `e4-reconcile-setup-action-open`'s step 1 `input` and `expect`, whole; not its `capture`,
      whose `/objectIds` pointers a portable case may not hold.
   3. `kind: invoke`, time `2026-11-11T12:00:00Z`, root's request
      `{"identity": {"kind": "instance", "id": ROOT}, "tool": "management.inspect.v1", "arguments": {"object": {"kind": "Interface", "name": "viewer"}}}`
      with ROOT root's ID. `expect` `{"reply": {"isError": false}}`; `capture`
      `{"viewer": "/reply/structuredContent/id"}`.
   4. `kind: invoke`, time `2026-11-11T12:00:00Z`, root's request through the tool
      `group.inspect.v1` with the arguments `{"object": {"kind": "Agent", "name": "a"}}`, as
      `e4-inventory-scope`'s step 7 does: an Agent's inspection goes through its owning Fleet, and
      the engine refuses it through `management` (ACC-033). `expect`
      `{"reply": {"isError": false}, "absent": [U]}`; `capture` `{"a": "/reply/structuredContent/id"}`.
   5. `e4-inventory-scope`'s step 2 `input` (root opens `viewer-pool`) and `expect`, whole.
   6. `e4-inventory-scope`'s step 4 `input` (root opens `a-self`) and `expect`, whole.
   7. `e4-reconcile-setup-action-open`'s step 2 `input` (root's `group.set-up.v1` of `a`), whole.
      `expect` `{"reply": {"pending": true}}`; `capture` `{"setup": "/reply/invocation"}`.
   8. Its step 3 `input` (`inspect-run`, `stopped`), whole. `expect` `{"reply": {"pending": true}}`.
   9. Its step 4 `input`, whole except that `report.outcome` is `failed`. `expect`
      `{"reply": {"isError": true, "_meta": {"META/outcome": "failed", "META/unanswered": 1}}}`:
      the setup's own result to root counts the action it leaves.
   10. `kind: invoke`, time `2026-11-12T12:00:00Z`, the request
       `{"identity": {"kind": "instance", "id": "$viewer"}, "tool": "pool.list.v1", "arguments": {}}`.
       `expect` `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/pending", U]}`:
       `viewer`'s scope holds `store` alone, not `a`'s owner.
   11. `e4-inventory-scope`'s step 3 `input` (root opens `viewer-fleet`), whole except that its
       evidence `time` is `2026-11-12T12:00:00Z`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}}}`.
   12. `kind: invoke`, time `2026-11-12T12:00:00Z`, `$viewer`'s request through `fleet.inspect.v1`
       with the arguments `{"object": {"kind": "Agent", "name": "a"}}`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"status": {"pending": [{"id": "$setup", "kind": "setup", "subject": [{"kind": "Agent", "name": "a", "id": "$a"}], "expiry": "2026-11-19T12:00:00Z"}]}}}}`.
   13. `kind: invoke`, time `2026-11-12T12:00:00Z`, `$viewer`'s request through `pool.list.v1`
       with the arguments `{}`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"pending": [{"id": "$setup", "kind": "setup", "subject": [{"kind": "Agent", "name": "a", "id": "$a"}], "expiry": "2026-11-19T12:00:00Z"}]}}}`:
       listed through `store` though its subject is `group`'s (decision 5).
   14. `kind: invoke`, time `2026-11-12T12:00:00Z`, the request
       `{"identity": {"kind": "instance", "id": "$a"}, "tool": "own.list.v1", "arguments": {}}`.
       `expect` `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/pending", U]}`.
   15. `kind: invoke`, time `2026-11-12T12:00:00Z`, `$a`'s request through `own.inspect.v1` with
       the arguments `{"object": {"kind": "Agent", "name": "a"}}`. `expect`
       `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/status/pending", U]}`.
   16. `kind: invoke`, time `2026-11-19T12:00:00Z`, `$viewer`'s request through `fleet.list.v1`
       with the arguments `{}`. `expect`
       `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/pending", U]}`: the
       action has expired.

   This route is new and has not been run: every e4 case today starts from the snapshot
   `conformance/fixtures/root-system-state.json`, which differs from a first start (it holds
   `root-system` and lacks first start's `root-pool-abc03`), and no case registers the e4 fixture
   after a first start. The admin could not run steps 1, 2 and 7 to 9 before dispatch, since they
   need order 047 merged; the stop clause below is expected to be the check.

   Before running it, check that no ID repeats across the case's steps' `ids`. The case asserts no
   evidence item: decision 7 pins the views, and so PND-004's disclosure. If 046a's runner check refuses the case all the same,
   if a step that this case takes whole from another case gives a reply other than that case
   states, or if a step's reply differs from what it states, stop and report the error: do not
   change the expectations to pass it, and do not change the inputs beyond what this decision
   says. No expectation is generated from the engine's output. The coverage index entry holds
   `id`, `case` (`conventions/setup/action-by-scope/case.yaml`), `promises` and, where its
   neighbours in that piece hold it, `safety_rows: []`, in the members and order its neighbours
   use. Write it in a `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it into
   `conformance/conventions/setup/index.yaml` in ID order and leaves `manifest.yaml` byte for byte
   as it was.

9. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.**
   - In DL19's row in "Choices still required", the sentence that 047 wrote, "It is raised only at
     root's own inspection of its instance, in `status.pending`; an Interface's scope,
     `list.v1`'s `pending` output and the `unanswered` key come with order 047b.", becomes, in the
     same line:

     ```text
     While raised it is raised at root and at each Interface whose scope holds its instance, never at an Agent (order 047b): it is listed in `status.pending` of that Interface's own inspection of the instance and in the `pending` output of its own `list.v1` through any Pool, Fleet or management, and counted in the `unanswered` key of its own results; another Interface sees its evidence only where it may inspect the instance, and no Interface sees a code or fingerprint item.
     ```

     Nothing else in the row or the file changes.
   - After the other decisions, run `python3 tools/promise_standing.py --write`, as AGENTS.md
     asks of a change to a case's `promises` and to a promise ID in the core's sources. Report the
     rows of PND-002 and PND-004 and the totals; the totals change only by what those rows
     change.

10. **The declaration.** `docs/work-orders/047b-declared.yaml` holds one `added` shape named
    `setup-action-by-scope` for that case. The coordinator wrote it with this order; check that it
    matches and change it only to make it so.

11. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or record
    changes. No pending action is stored or answered, `answer.v1` stays the stub that names 047c,
    no `pendingAction` key is set, and no kind other than setup is held: 047c, 047d and 048a-2.
    No CLI command (051b), shell code, tool list or record changes; peer delivery is batch 2's
    (PND-002). Report as findings, for the coordinator: that `list.v1`'s description does not say
    which actions a list through a Pool or a Fleet shows, nor whether `kind`, which "optionally
    narrows object kinds", narrows `pending` (decision 5's readings); that
    `evidence-item.v1` names no object an item is about, which 047d's notice, with its several
    subjects, needs for PND-002's disclosure (decision 2); and that PND-002's scope can raise an
    action whose subjects the Interface may not inspect, as a Conduit's (decision 7's last
    bullet).

## Scope

Change only `engine/crates/core/src/pending.rs` (decisions 1 to 3 and 7), `records.rs` (the
inspection and `list.v1`), `lib.rs` (decision 6), `inventory.rs` only to move decision 1's scope
test into its one `pub(crate) fn` and call it from `read`, and `access.rs` only to expose a lookup
that decision 1 or 6 reuses, each without changing what it gives, `requests.rs` only for decision
5's removal of `pending` from `list.v1`'s recorded outputs,
`engine/CONFORMANCE.md` (one sentence), `engine/STANDING.md`, written only by the tool,
`conformance/conventions/setup/index.yaml` and the new
`conformance/conventions/setup/action-by-scope/case.yaml`. `docs/work-orders/047b-declared.yaml`
comes with this order; change it only as decision 10 says. `conformance/manifest.yaml` holds the
new entry only until the pieces tool moves it, and ends unchanged. Not `spec/`, `IN_FORCE.md`,
`tools/`, `packages/`, the shell, the experiments, the case schema,
`conformance/internal-cases.yaml`, `conformance/expected-failures.yaml`, any other case or
fixture, or any record: report any defect as a finding. `experiments/011-short-form-spike/`
changes only if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- pending::tests
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. setup-action-by-scope
cargo run --release -p conduit-conformance -- .. setup-action-
cargo run --release -p conduit-conformance -- .. e4-reconcile-setup-
cargo run --release -p conduit-conformance -- .. setup-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 2 passed and 0 failed: 047's test and this order's. The pieces tool prints
`Nothing to move.` The runner commands end `1 cases passed`, `2 cases passed` (this case and
`setup-action-at-root`), `2 cases passed` with no expected failure, and one more case than 047's
Record reports for `setup-` (41 if 047 added its case to the 39 of `conventions/setup/` at
`cbb49fc1`; report the count). The shell test passes. If any other case, or any shell, CLI or
experiment test, fails because a result now carries the key or a list its `pending`, stop and
report it: do not change it. The threads are 4 because at 24 the runner ran out of memory (order
071's Record).

Next, show that the case fails without decisions 4 and 6. From `engine/`, one at a time:

```bash
cp crates/core/src/records.rs target/records.rs.fixed
git show HEAD:engine/crates/core/src/records.rs > crates/core/src/records.rs
cargo run --release -p conduit-conformance -- .. setup-action-by-scope
cp target/records.rs.fixed crates/core/src/records.rs
cp crates/core/src/lib.rs target/lib.rs.fixed
git show HEAD:engine/crates/core/src/lib.rs > crates/core/src/lib.rs
cargo run --release -p conduit-conformance -- .. setup-action-by-scope
cp target/lib.rs.fixed crates/core/src/lib.rs
cargo run --release -p conduit-conformance -- .. setup-action-by-scope
```

The first two runner commands must each fail, printing `FAIL 0 cases passed; 1 failed`: the
first with an error at step 12 beginning `/reply/structuredContent/status/pending: missing
required field`, since 047's inspection raises the action at root alone; the second with an error
at step 9 that names the `unanswered` key. After both restores the third ends `1 cases passed`.
Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild the crate. Git ignores `target/`. If `HEAD`'s file does not build against the rest of
your change, revert that decision's lines by hand in the copy instead, and say so.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/047b-declared.yaml; echo "exit $?"
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/inventory engine/crates/shell spec IN_FORCE.md tools packages docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/047b-declared.yaml: 1 shapes.`,
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
grep -c 'fn actions_raised_by_scope' engine/crates/core/src/pending.rs
grep -c '"list-pending-actions"' engine/crates/core/src/records.rs
grep -c '"id": "setup-action-by-scope"' conformance/conventions/setup/index.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'counted in the `unanswered` key of its own results'
grep -c '^| PND-002 | convention | must | 2 | 0 | outside | core | case passes |$' engine/STANDING.md
grep -c '^| PND-004 | interface | must | 0 | 0 | outside | core | no case, no reason |$' engine/STANDING.md
grep -c '("answer.v1", "047c", "PND-009")' engine/crates/core/src/requests.rs
```

Each of these prints 0:

```bash
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'come with order 047b'
git diff HEAD --stat -- conformance/expected-failures.yaml conformance/conventions/setup/action-at-root | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the rows decision 9 names with the totals, the function names and signatures of
decisions 1 to 3, where decision 6's marking runs, and every path by which a tool result gets no
key under decision 6's last sentences, with each non-invoke input whose result decision 6 finds
through the `request` it carries.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `047b-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.

## Record

Run on 2026-11-18, from about 09:12 to 10:22 UTC, on the Mac from `main` `3aafbd93` (order 047's
merge), in two rounds: the worker (1952 s) and a fresh Codex verifier (498 s), then the worker again
(1389 s) and a second fresh Codex verifier (363 s). The change is ten files of Scope. `pending.rs`
gains `is_raised_for`, `scope_contains_subject`, `action_view(action: &Value, holder: &Value, state:
&State, context: &Context) -> Result<Option<Value>>`, `held_actions(state: &State, context:
&Context) -> Vec<Value>`, `pub(crate) fn raised_views(holder: &Value, state: &State, time: &str,
context: &Context) -> Result<Vec<Value>>` and `pub(crate) fn own_interface`, with the test
`actions_raised_by_scope`; `inventory.rs` gains `pub(crate) fn is_scope_conduit(holder: &Value,
conduit: &Value, state: &State, context: &Context) -> bool`, which `read` calls; `access.rs` exposes
its `management` lookup; `records.rs` sets `status.pending` at each Interface's own inspection and
`list.v1`'s `pending` output with the decision `list-pending-actions`; `requests.rs` removes
`pending` from `list.v1`'s recorded outputs; `lib.rs`'s `step_owned` marks the `unanswered` key
after `step_body` and before `explain::first_text`. The case `setup-action-by-scope` has 16 steps
and 798 unique allocation IDs; with it come its index entry, the DL19 sentence as the order gives it
and `STANDING.md`, where PND-002's row counts 2 cases and PND-004's is named in the core, 325
promises named in engine source. The worker's own choices: the exposed `access::management` lookup
for root, and `own_interface` shared by the list, the inspection and the marking. It reported
decision 11's three findings for the coordinator: list.v1's description does not say which actions a
list through a Pool or a Fleet shows, nor whether `kind` narrows `pending`; `evidence-item.v1` names
no object an item is about, which 047d's notice needs; and PND-002's scope can raise an action about
a Conduit that the Interface may not inspect, which the build neither lists nor counts. The first
round's release suite failed one test, order 047's expiry repeat test, which compared whole replies.
The first verifier found four musts: that test (decision 6's live count breaks its whole-reply
equality); a retained `list.v1` reply that shows pending subjects and evidence after the Interface
loses inspection scope; a first-attempt filter refusal of an Interface's own request, which got no
key since the refusal removes the invocation; and PND-012's record gap. The coordinator's round-2
prompt forwarded the four fixes as the verifier wrote them. Round 2 compared the expiry test's
`structuredContent` and checked the key on each reply; added a rewrite of retained `list.v1` replies
on the repeat path in `requests.rs`, with `restrict_retained_list` in `pending.rs` and a test;
captured the caller from the pre-step state for a completion (`completion_caller` and
`primary_caller` in `lib.rs`), with a test in `requests.rs`; and left PND-012's record to the
coordinator. The release suite then passed. The second verifier found five musts and a should: that
the replay rewrite checks no expiry, that retained inspections still replay `status.pending`, that
the replay goes beyond the Scope and OPS-009, that decision 6's text conflicts with the build, and
PND-012's gap; its should asked for a replay test that checks inspection authority apart from scope.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on a scratch copy at the
branch with `HEAD` reset to `3aafbd93` and `RUST_TEST_THREADS=4`. Every check of "Done when" passed
as worded: `pending::tests` 2 passed, `Nothing to move.`, the runner's 1, 2, 2 and 41 cases, the
shell test, the vectors and the full release suite; both negative controls failed as worded, at step
12 and at step 9, and the restored run passed; the spike reported 9 passed; each Python check ended
`Errors: 0`; the tool tests ran 129, OK; and `vector_delta.py` reported 1 shapes, 1 changes, 0
flips, 0 undeclared and 0 errors. The case matched decision 8 step by step and the DL19 sentence was
exact. It found one must and three shoulds:

- R1: round 2's rewrite of retained `list.v1` replies on the OPS-009 repeat path lies outside the
  Scope, which admits `requests.rs` only for decision 5's removal; no decision asks for it; it
  contradicts 047's repeat rule in DL19; it leaves list and inspection repeats inconsistent; and it
  checks no expiry.
- R2: decision 6 says a result whose invocation the state no longer holds gets no key, but the build
  captures the caller from the pre-step state (`completion_caller` and `primary_caller`) and sets
  the key on a first-attempt filter refusal; the coordinator's round-2 prompt directed this, the
  order's text still says otherwise, and the new test sits in `requests.rs`, outside the Scope.
- R3: the edit of order 047's expiry repeat test in `requests.rs` is sound but not authorized by the
  order, whose stop clause does not name core unit tests.
- R4: two conditions decision 1 does not state, unreported as own choices: `is_raised_for` returned
  false for an action with an `answer` member, and `is_scope_conduit` required the Conduit to have
  no `spec`. Neither changed a result.

The fixer rebased the branch onto `origin/main` at `f72c2bc3` (orders 046b and 047c, documents only)
with the work-order identity, without conflict, and applied each finding:

- R1: reverted the repeat-path block in `requests.rs`, `restrict_retained_list` in `pending.rs`, the
  test `retained_list_withholds_pending_after_inspection_scope_closes` and the then-unused `pending`
  import. A retained list again returns its original outputs, as an inspection repeat does under
  DL19. Finding for the owner: OPS-009 has a repeat return the outputs the invocation returned,
  while PND-002 and PND-004 limit pending subjects and evidence to what the Interface may inspect
  and to actions still raised, so a keyed repeat of `list.v1` or `inspect.v1` can show an action and
  its evidence after the Interface's inspection scope has closed or the action has expired. Which
  rule wins for retained pending outputs is the owner's question.
- R2: kept. Decision 6's sentence on a missing invocation is replaced: a first-attempt filter
  refusal of an Interface's own request is attributed through the pre-step invocation, matched by ID
  and attempt. The Scope was widened for the `requests.rs` test
  `first_filter_refusal_counts_actions_for_the_waiting_interface`, which keeps the delegated-refusal
  assertion.
- R3: kept, as a Scope widening by the coordinator, made because decision 6's live count breaks
  whole-reply equality in the expiry repeat test: the test now compares `structuredContent`, checks
  `unanswered` 1 on the original and no key on the repeat at expiry.
- R4: dropped `action.get("answer").is_some() ||` from `is_raised_for`, leaving stored answers to
  047c, and `&& conduit.get("spec").is_none()` from `is_scope_conduit`, since `read` already skips
  Conduits with a `spec`.

PND-012's record gap, that the record of an inspection that raises an action does not name the
action's ID, was in place after 047 and stays with the coordinator. The fixer then ran, from
`engine/` with `RUST_TEST_THREADS=4`, each check of "Done when" on the files of this commit before
it was made: `cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` (exit 0),
`pending::tests` (2 passed, 0 failed), the pieces tool (`Nothing to move.`), the runner (`1 cases
passed`, `2 cases passed`, `2 cases passed`, `41 cases passed`), the shell test (1 passed), the
vectors (2 passed) and the full release suite (exit 0: 322 passed, 0 failed, 4 ignored). The
negative controls, with `origin/main`'s `records.rs` and then its `lib.rs`, each printed `FAIL 0
cases passed; 1 failed`, the first at step 12 with `/reply/structuredContent/status/pending: missing
required field` and the second at step 9 with an error naming the `unanswered` key; after both
restores the case ended `1 cases passed`. The spike reported `test result: ok. 9 passed; 0 failed`.
From the root, each Python check ended `Errors: 0`, `/usr/bin/python3 -m unittest discover -s
tools/tests` ran 129, OK, and `vector_delta.py origin/main WORKTREE --declared
docs/work-orders/047b-declared.yaml` printed `1 shapes.` and `Vector delta: 1 changes; 0 flips; 0
undeclared; 0 errors.`, exit 0 (against `HEAD`, which holds the case, 0 changes). The order's greps
printed seven 1s and two 0s, the protected paths' `git status` printed nothing, and `git diff
--check`, `git diff --check origin/main...HEAD`, `check_docs` and `check_private_paths` were clean.

A second fresh read-only Opus reader read the branch at `5d2596d3`, with `HEAD` soft-reset to
`f72c2bc3`; every check of "Done when" passed as worded and each fix held. It found no must and
three shoulds, applied at the merge: the round durations and the second verifier's counts above,
decision 11's three findings above, and DL19's repeat sentence, which after R1's revert named
inspections alone although a keyed repeat of `list.v1` returns its retained `pending` output. As the
coordinator's amendment, open to the owner's strike, DL19 now reads "returns its original
inspection or `list.v1` outputs while retained, including any `status.pending` or `pending`,", so
the row declares the limit the engine has; R1's question for the owner stands. As the
coordinator's acceptance, open to the owner's strike, the coordinator also accepts the fixer's R2,
decision 6's sentence on a missing invocation replaced as written above, and R3, the Scope widened
for the expiry repeat test. After the DL19 change, the order's checks that read `CONFORMANCE.md`
were run again: the `tr` grep for the `unanswered` key printed 1 and the one for `come with order
047b` printed 0, and `promise_standing.py --check`, `check_docs.py` and `check_private_paths.py`
each ended `Errors: 0`. The gate's run on this order's squash, which this merge waits on, is the
coordinator's at the merge.

At the merge the coordinator applied the last reader's should, if any remained, and added a check
line to 047b-declared.yaml, which reached main with this order's text, so that the gate's vector
check reads it. The gate runs on the squash, and the merge waits on its PASS.
