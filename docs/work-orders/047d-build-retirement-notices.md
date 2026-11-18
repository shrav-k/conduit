# Work order 047d (E6d): the notice kind: one per Fleet that a Resource's retirement cuts off, its `pendingAction` key, and `acknowledged`

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
an order holds one behavior and stays small). This is the fourth: "047d: the notice kind
(PND-007), stored with an engine ID allocated within retirement, with its expiry, the
`pendingAction` key on the retirement's result, and `acknowledged` through `answer.v1`."
[Order 047](047-build-setup-action.md) computes a failed setup's action and raises it at root;
[order 047b](047b-build-actions-raised-by-scope.md) raises an action wherever PND-002 puts it;
[order 047c](047c-build-stored-answers.md) builds `answer.v1` with stored answers for the setup
kind. Sign-in as a kind of pending action, PND-005 and `SignInUndone` are order 048a-2's and the
four orders after it; enrollment's kinds are 049a-2's. This order builds none of them, and no
kind but the notice.

**What it builds.** One behavior: the notice. PND-007: "A Resource's retirement sent by an
instance other than root closes under CON-008 a Conduit consumed by an Agent of a Fleet other
than the sender's", then "One notice is raised per such Fleet. If the sender is not an Agent,
every Fleet counts as other. Its subjects are the Resource and that Fleet's closed Conduits,
retained by their resolved identities. Its evidence gives the Resource, owning Pool, sender,
caller, invocation and closed Conduits. It is stored with an engine ID allocated within
retirement and an expiry under DL19." Its observable: "It is raised at root and Interfaces whose
scope contains that Fleet. answer.v1 answers acknowledged. The retirement result names each
notice in `META/pendingAction`. Raising a notice closes nothing beyond CON-008 and confers no
permission. Evidence is disclosed under PND-002 and PND-004." (The quote writes the key as
`META/pendingAction`, the `_meta` key `pendingAction` under the prefix that `spec/names.yaml`
sets, as decision 7 writes such keys.) PND-009 adds that `answer.v1` goes "to the target that
authorizes inspection of the subject (ACC-033): ... the owning Fleet for an Agent or a notice's
Fleet", and "Where subjects have different inspection targets, authorization follows the subject
the kind defines for answering: the notice's Fleet under PND-007". PND-001: "stored kinds receive
an engine ID when raised ... Where there is a raising invocation, the action retains its
attribution to that invocation." PND-012: "A request raises or answers an action. Its record
names the action's ID", and "Raising adds no record type of its own". [Spec batch 1's
plan](../proposals/spec-batch-1/areas/pending-actions.md) lists the vectors `notice-other-fleet`
and `notice-none` (its cases 20 and 21): this order's case covers the first, with an Interface as
the sender, and its unit test the second and the sender's own Fleet.

**What orders 047, 047b and 047c bring, which this order assumes.** Orders 047 and 047b are
merged (#353 and #355). Order 047c is written on `main` and dispatched, not yet merged; this order
starts only once it is merged (below). From 047 and 047b, as their Records name them, in
`engine/crates/core/src/pending.rs`:

- `setup_action` and `is_raised_at(action, time)`, true before the action's expiry and false at it
  and after; the members `ended`, `dispatched` and `setup_cause` on set-up.v1 invocations.
- `is_raised_for`, which tests the time, then the holder: false where it is not an Interface or is
  a template, true at root, and otherwise `scope_contains_subject`, which resolves each subject by
  its `id` (a subject no longer in state counts for nothing) and tests it against the targets of
  the holder's scope Conduits (`inventory::is_scope_conduit`).
- `action_view`, which at root removes `code` and `fingerprint` items, and at another Interface
  keeps the subjects it may inspect by `access::may_inspect`, gives no view where it keeps none,
  and keeps the evidence, less those items, only where it kept every subject.
- `held_actions`, the computed setup actions in byte order of ID, which 047b's decision 3 says
  "no stored kind exists yet (047c and 047d add them here)"; `raised_views`, whose number is the
  count; `own_interface`.
- In `records::execute`: `status.pending` at an Interface's own inspection, the views whose
  subjects include the inspected object's ID, with one decision `raise-setup-action` for each
  action listed (`records.rs:415-436` at `f414e438`); and `list.v1`'s `pending` with the decision
  `list-pending-actions`. In `lib.rs`'s `step_owned`, the `_meta` key `unanswered` on each result
  of an Interface's own request.

From 047c, as its order states them; its Record is not yet written, so each is an assumption
this order names:

- **A1.** `State`'s member `pending_actions`, a `BTreeMap<String, Value>` keyed by action ID, each
  value `{"action": A, "dispatched": D}`, which no purge touches.
- **A2.** A function in `pending.rs` that gives the action an ID names, with its answer, at an
  optional time: a computed setup action first, and otherwise the stored entry's `action` with
  its answer.
- **A3.** A function that says whether an instance may answer an action ID (its decision 2),
  called in `access::evaluate_path` right after SEC-007's check, which reuses 047b's holder test
  without its time test, 047b's function split in two to allow it.
- **A4.** A `pub(crate) fn` in `access.rs` (state, action, context) that gives the ID of the
  target authorizing inspection of the action's subject, "the action's first subject resolved by
  ID (a setup action has one; 047d adds the notice's rule)", or `management` where the subject no
  longer exists, which `routing`'s arm for `answer.v1` calls.
- **A5.** In `pending.rs`, the table of the kinds and the answers `answer.v1` accepts, one row,
  `setup` and `dismissed`, "047d and 049a-2 add theirs"; in `records::preflight`, invalid under
  PND-009 with a specific diagnostic whose text names the kind's accepted answer, then conflict
  under PND-003 for an action that has an answer.
- **A6.** `answer.v1`'s branch in `records::execute`: it stores the answer
  `{"value", "actor", "invocation", "at"}`, ends the invocation succeeded with outputs `{}`, adds
  `"pendingAction": [ID]` to its `invocation-ended` record, writes one `pending-action-answered`
  record whose `data` is `{"pendingAction": [ID], "answer": A, "invocation": I}`, and pushes the
  decision `answer-pending-action`.
- **A7.** In `retirement::execute`, right before `object["state"] = json!("retired");`, the call
  of 047c's decision 6 function that answers the instance's setup action `retired`, and, where it
  answered one, `"pendingAction": [ID]` in the `extra` of the `object-change` record whose action
  is `retired`, with its `pending-action-answered` record right after that record; the same at
  each other place 047c's Record names for a retirement.
- **A8.** An action that A2 gives an answer at a time is not raised (its decision 8).
- **A9.** The case `setup-action-answers`; DL19's row in `engine/CONFORMANCE.md` ending
  "No other kind is held, so no other duration is chosen."; and the record type
  `pending-action-answered` in DL8's row, the runner's list and the profile's `record_types`.

Order 048a-2, written on `main` (#357), stores the sign-in kind as an entry
`{"action": A, "dispatched": null, "raisedBy": I}` and extends A2 and `held_actions` for stored
kinds (its decision 4), and makes retirement's `void` store keep the entry's other members (its
decision 7). It may merge before or after this order. Decision 5 below states those extensions for
the notice too: where 048a-2 has merged first, they exist, so reuse them and write no second copy.
Where 047c or 048a-2 merged otherwise than stated here, apply the nearest sound reading of each
decision below to what merged, and say so.

**What the engine already does, so that you do not redo it.** Line numbers are those of `main` at
`f414e438`, before order 047c; find each place by the text quoted with it.

- `retirement::execute` (`retirement.rs:169`) retires an instance: past the `unregister` branch, a
  first retirement makes its tombstone (`let tombstone = if o["state"] != "retired" {`, `:207`;
  a further one is a retry under LIF-040), sets `object["state"] = json!("retired");` (`:221`),
  writes the `object-change` record whose action is `retired` through `event` (`:228`), whose last
  argument is merged into the record's `data`, and then calls
  `close_ends(t, a, c, i, &o["id"], false)?;` (`:237`).
- `close_ends` (`retirement.rs:100`) closes, under CON-008, every Conduit with no `spec` whose
  `target` is the instance (and, where its last argument is true, whose `consumer` is). For each,
  in the order of `state.objects`, it makes the Conduit's tombstone, writes
  `status.closedConduits[ID] = {"name", "capability"}` on the consumer, writes the `object-change`
  records `closed` and `removed`, and removes the Conduit from state (CON-008: "A closed Conduit
  is removed, under LIF-035"). After it no Conduit it closed can be inspected (OPS-029).
- A cascade's member is retired by a retire.v1 invocation whose `sender` and `caller` are the
  cascade's and which holds `cascade_parent`, through `retirement::execute`
  (`cascade.rs`, `retire_part`, the call `crate::retirement::execute(t, a, c, index, object)?;`).
- `requests::finish` (`requests.rs:1596`) ends an invocation: it builds the reply with `envelope`
  (`:114`), whose `_meta` keys `mark(c, name)` (`:24`) qualifies, sets it as `t.reply` and pushes
  it to `request_replies`.
- `access::may_inspect` (`access.rs:501`) inspects a Resource or Agent through its owner where
  that owner is not retired, and anything else, a Conduit included, through `management`.
  `access::management(state, "root", context)` (`:457`) finds root.
- An `Allocation` (`lib.rs:275`) gives the step's next engine ID with `next`; a record takes its
  ID the same way.

**When this order starts.** Order 047c must be merged to `main` first. Before you change
anything, check from the repository root that you find what it gives, and stop, change nothing
and report if any check prints otherwise:

```bash
grep -c 'fn stored_answers_follow_their_ending' engine/crates/core/src/pending.rs
grep -c 'fn actions_raised_by_scope' engine/crates/core/src/pending.rs
grep -c 'pending_actions' engine/crates/core/src/lib.rs
grep -c '"answer-pending-action"' engine/crates/core/src/records.rs
grep -c '"answer-setup-retired"' engine/crates/core/src/retirement.rs
grep -c '"pending-action-answered"' engine/crates/conformance/src/checks.rs
test -f conformance/conventions/setup/action-answers/case.yaml; echo "case $?"
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'No other kind is held, so no other duration is chosen.'
grep -c '^## Record' docs/work-orders/047c-build-stored-answers.md
test -f engine/crates/core/src/sign_in.rs; echo "sign-in $?"
```

They print `1`, `1`, a number other than `0`, `1`, `1`, `1`, `case 0`, `1` and `1`. The last says
whether order 048a-2 has merged (`sign-in 0`) or not (`sign-in 1`): report which, and where it has,
the names of its extensions that decision 5 reuses. Orders that land beside this one (046b,
048a-2, 049a-1, 050 and others): whichever lands second keeps both changes and writes
`engine/STANDING.md` again with the tool.

**Who decided what.** PND-001, PND-002, PND-003, PND-004, PND-007, PND-009 and PND-012, CON-008
and ACC-033's row 17 decide when a notice is raised, what it holds, where it is raised, who
answers it, through which target, and what its record names. Everything else here is the
coordinator's, each open to the owner's strike ([ruling 48](../proposals/ruling-48/README.md)):
the stand of decision 4 on the finding that order 047b's Record names, that `evidence-item.v1`
names no object an item is about; counting every Conduit that
`close_ends` closes at the Resource's retirement, open or not, and raising nothing at a retry
under LIF-040 or at an unregistration; the order of the notices and of their subjects; the stored
entry's members `fleet` and `about`, and `raisedBy` as 048a-2 names it; the evidence items' words
and what each is about; DL19's duration for a notice, seven days from the step that raised it;
naming the notices in the `object-change` record whose action is `retired`, after any IDs that
record's `pendingAction` already holds, as the retirement's record under PND-012, as 047c names
that record for a retirement's answer; the invocation member `notices`, and naming a cascade
member's notices in the cascade's result too; the key's value, a list of IDs, set on every result
that ends the retire.v1 invocation, whatever its outcome and whoever its caller; the target
`management` where the notice's Fleet is retired or gone, as ACC-033's row 6A falls back; the
invalid's words; the decisions' names; the case, its route, IDs and times; and DL19's sentence.
Apply them as written and report them under `decisions` as the coordinator's. Rulings 80 to 82
and the targeted review's
[section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists) put nothing on
047d; nothing here touches sign-in.

**Read first:**

1. In `spec/`: PND-001, PND-002, PND-003, PND-004, PND-007, PND-009 and PND-012
   (`spec/promises/pending.yaml`); CON-008 (`spec/promises/conduits.yaml`); ACC-033, its rows 4,
   6, 6A, 7 and 17 (`spec/conventions/access.yaml`); DL19 (`spec/implementation-defined.yaml`);
   `answer.v1`, `retire.v1`, `inspect.v1` and `list.v1` (`spec/core/operations.yaml`);
   `pending-action.v1` and `evidence-item.v1` (`spec/core/results.yaml`); the rows pending
   action, raised at, notice, scope and inspect of `spec/definitions.md`.
2. Orders [047](047-build-setup-action.md), [047b](047b-build-actions-raised-by-scope.md) and
   [047c](047c-build-stored-answers.md), whole, and their Records; order
   [048a2](048a2-build-sign-in-lapse-and-conflict.md), its decisions 4 and 7.
3. The cases `setup-action-by-scope`, `setup-action-answers`, `e4-closed-with-target` and
   `e4-provenance-setup-parts`, under `conformance/conventions/`, and the e4 fixture's
   `documents.json` (`conformance/conventions/inventory/e4/`): the Pool `store`, the Fleet
   `group`, the Agent `a` of `group`, the Resource `shared` of `store`, the Interfaces `viewer`
   and `declared` (from `declared-person`, whose Conduit `slot` to `store` comes from the core's
   `root-pool` template, which lists `retire.v1`, `list.v1`, `inspect.v1` and `answer.v1`), and
   the Conduits `to-shared` (`a` to `shared`), `viewer-shared` (`viewer` to `shared`),
   `viewer-pool` (local name `pool`) and `viewer-fleet` (local name `fleet`).
4. `conformance/README.md`, its sections "Assertions" and "Portable cases".

**Size: M**: about 0.35 to 0.5k changed lines of Rust with the unit tests, one new case, and one
place in `engine/CONFORMANCE.md`.

## The decisions it applies

1. **Where a notice is raised.** In `pending.rs`, add a function that raises the notices of a
   Resource's retirement, given the state, the Resource, the retirement's invocation, the step's
   time, the `Allocation` and the context, and gives the notices it raised, in order. It raises
   none where the Resource is a template (it has `spec`), or where the invocation's `sender` is
   root's ID. Otherwise:
   - The Conduits are those that `close_ends` closes at this retirement, found by `close_ends`'s
     own test with its last argument false: move that test into one `fn` that both call, without
     changing what `close_ends` does, rather than writing a second copy. Every such Conduit
     counts, open or not, since CON-008 closes and removes each.
   - A Conduit counts for the Fleet that owns its consumer, where that consumer is an Agent (by
     `kind`, resolved by ID, a template excluded) whose `owner` is a Fleet; a Conduit whose
     consumer is not an Agent counts for none.
   - The sender's Fleet is the `owner` of the sender where the sender is an Agent, and none
     otherwise ("If the sender is not an Agent, every Fleet counts as other"). A Fleet that is the
     sender's raises nothing.
   - One notice is raised for each other Fleet with at least one Conduit, in the order in which
     each Fleet's first Conduit comes among the Conduits; each takes its ID from `a.next` at its
     turn.

   In `retirement::execute`, past the `unregister` branch, on a first retirement only (where
   `o["state"]` is not `retired`, as the tombstone's test reads it) and for a Resource only, call
   it right before `object["state"] = json!("retired");`, after 047c's call (A7) and, where
   048a-2 has merged, after its own, so that the notices take their IDs before the
   `object-change` record whose action is `retired` takes its own. Report every other place
   where a Resource's Conduits close at its retirement, and call it there too; a cascade's
   member is retired through `retirement::execute`, so it raises its notices there.

2. **The notice and its entry.** Each notice is `pending-action.v1`'s form, members in this order:
   `{"id": ID, "kind": "notice", "subject": S, "evidence": E, "expiry": X}`, with no `answer`:
   - `S` is the Resource, then each of that Fleet's Conduits in the order of decision 1, each
     `{"kind": K, "name": N, "id": I}` from its object as it stood before the retirement (PND-007:
     "retained by their resolved identities").
   - `X` is the step's evidence time plus seven days, written as 047's expiry is (RFC 3339 in UTC
     with `Z`, `SecondsFormat::AutoSi`).
   - `E` is a list of `evidence-item.v1` items, each `{"kind": "fact", "value": V, "where": W,
     "proves": P}`, in this order:
     1. `V` the Resource's ID; `W` `records`; `P` `the Resource whose retirement closed these Conduits`.
     2. `V` the ID of the Resource's owning Pool, where it has an `owner`; `W` `declared`; `P`
        `the Pool that owns that Resource`.
     3. `V` the invocation's `sender`; `W` `records`; `P` `the instance that sent the retirement`.
     4. `V` the invocation's `caller`; `W` `records`; `P` `the caller on whose behalf the retirement was sent`.
     5. `V` the invocation's ID; `W` `records`; `P` `the retirement invocation that closed these Conduits`.
     6. One for each of that Fleet's Conduits, in the order of `S`: `V`
        `{"id": I, "name": N, "consumer": C}`, `C` the consumer's ID; `W` `status.closedConduits`;
        `P` `a Conduit of an Agent of this Fleet that the retirement closed`.

     Every item is a fact the engine observed (PND-004); none is a code or a fingerprint.

   Store each notice in 047c's `pending_actions` (A1), keyed by its ID, as the entry
   `{"action": A, "dispatched": null, "raisedBy": R, "fleet": F, "about": B}`: `A` the notice, `R`
   the retirement's invocation ID (PND-001: "the action retains its attribution to that
   invocation"), `F` the Fleet's ID (PND-009: "the notice's Fleet"), and `B`
   `{"subject": [..], "evidence": [..]}`, two lists of IDs as long as `S` and `E`, each giving the
   object that subject or item is about (decision 4): the Resource's ID for the Resource and for
   items 1 to 5, and the consumer's ID for each Conduit and its item 6. Nothing removes an entry:
   a purge removes records alone (PND-012).

3. **Its record and the result's key.** Where decision 1 raised notices:
   - add their IDs, in order, to the `pendingAction` that the `extra` of the `event` writing the
     `object-change` record whose action is `retired` carries: after the IDs that A7 (and 048a-2,
     where merged) put there, or alone, so that the retirement's record names each (PND-012:
     "Its record names the action's ID"). Raising writes no record of its own (PND-012: "Raising
     adds no record type of its own");
   - set the member `notices`, the list of their IDs, on the retirement's invocation, and, where
     that invocation holds `cascade_parent`, append them to that parent invocation's `notices`
     too, so that a cascade's own result names its members' notices;
   - push one decision named `raise-notice`, citing `PND-001`, `PND-007` and `PND-012`, with
     `DL19`, and the facts `{"actions": [IDs], "resource": the Resource's ID}`.

   In `requests::finish`, where the invocation holds a non-empty `notices`, set the reply's `_meta`
   key `pendingAction`, qualified by `mark(c, "pendingAction")`, to that list, before the reply is
   pushed to `request_replies`, whatever the outcome (PND-007: "The retirement result names each
   notice"). Set it nowhere else. Report every other path by which a retire.v1 invocation, or a
   cascade, gives its caller a result without `finish`, and whether a repeat under OPS-009
   replays the key; do not change them.

4. **Raised at the notice's Fleet, and what each Interface sees: the stand on `evidence-item.v1`.**
   - **Where.** In the holder test that A3 split from 047b's `is_raised_for`, for an action whose
     `kind` is `notice`: true at root; and at another Interface (not a template) where one of its
     scope Conduits (`inventory::is_scope_conduit`) has as target the entry's `fleet`, or where
     047b's subject test (`scope_contains_subject`) holds; false otherwise. PND-007 says "It is
     raised at root and Interfaces whose scope contains that Fleet". PND-002, and the ruling in
     force (ruling 66 point 3: "A pending action is raised at root and at every Interface whose
     scope holds its subject"), also raise it where a scope holds the Resource's Pool, through the
     Resource. The two rules are joined. Since 047b's subject test counts for nothing a subject no
     longer in state, the Pool's half applies only while the Resource is in state, that is, while
     its shutdown is unconfirmed. That holder test is also A3's test of who may answer: a notice
     already answered keeps its `fleet` and is tested against the Interface's current scope
     (PND-009: "the raised-at test uses its retained subjects and the Interface's current scope").
   - **The stand.** PND-002: "Another Interface sees only subjects and evidence about objects it
     may inspect, with no count of the rest." Order 047b's Record reports that `evidence-item.v1`
     names no object an item is about, which the notice, with its several subjects, needs; and a
     notice's Conduit subjects are removed by CON-008 before anyone can inspect them. The
     coordinator takes this stand within the specification as written: the promise asks the
     engine to know what each subject and item is about, not the wire form to say it. So the
     engine keeps it in its own state, as decision 2's `about`, and `pending-action.v1` and
     `evidence-item.v1` do not change. A subject or item is about the object whose inspection
     shows it. A live subject is about itself. A closed Conduit survives in state in its
     consumer's `status.closedConduits`, under its ID, which that consumer's inspection shows
     (ACC-033 row 6A: through the owning Fleet), and as its tombstone, which LIF-040 shows to
     whoever may inspect either end as it stood; this order takes the consumer, so a closed
     Conduit and its item 6 are about the consumer. Items 1 to 5 come from the Resource's own
     records and declaration, which its inspection shows, so they are about the Resource. A
     completed retirement removes the Resource, so after it no Interface but root sees the
     Resource subject or items 1 to 5, even one that holds both the Pool and the Fleet.
   - **The view.** In 047b's `action_view`, for an entry that holds `about`, at an Interface other
     than root: keep, in order, each subject whose `about` object exists in state and that the
     holder may inspect (`access::may_inspect`), and no view where none is kept; keep each
     evidence item whose `about` object exists in state and that the holder may inspect; remove
     `code` and `fingerprint` items as 047b does. At root the view is 047b's. An action whose
     entry holds no `about`, the computed setup action and 048a-2's sign-in among them, keeps
     047b's view unchanged. No view says what it left out.

   So an Interface whose scope holds only the Fleet sees the notice with that Fleet's closed
   Conduits and their items, and the Resource and items 1 to 5 only where it may also inspect the
   Resource while the Resource is in state; the count and lists follow 047b's decision 3.
   `status.pending` lists a notice at the Resource's inspection, where the view keeps it, only
   while the Resource's shutdown is unconfirmed; after the Resource's removal the notice is
   listed only in `list.v1`.

5. **Held, expired and answered.** In `pending.rs`, and where 048a-2 has merged, through its
   functions:
   - **Every action the state holds** (047b's `held_actions`): add each stored entry whose
     action's `kind` is not `setup`, in the same byte order of ID as the computed actions.
   - **The action an ID names** (A2): for a stored entry whose action has no answer, where a time
     is given and is at or after its expiry, the answer is `{"value": "expired", "at": X}`;
     otherwise as A2 gives it.
   - **Storing an answer** (A6): Where 047c stores answer.v1's answer in `records::execute`, move
     that store into one `pub(crate) fn` in `pending.rs` (state, action ID, answer), which
     `records::execute`'s answer.v1 branch calls; for a stored entry it sets the answer on the
     entry's `action` and keeps every other member, and for a setup it stores as 047c does. Where
     047c or 048a-2 already has such a fn, extend it. A notice's other members are `raisedBy`,
     `fleet` and `about`.
   - **The accepted answers** (A5): add the row `notice` and `acknowledged`. For a notice, the
     invalid's specific diagnostic text is
     `{tool} reports invalid under PND-009: answer.v1 answers a notice only acknowledged.`; the
     setup kind keeps 047c's text.
   - **The target** (A4): for an action whose `kind` is `notice`, the entry's `fleet` where that
     Fleet is in state and not retired, and `management` otherwise (PND-009: "the owning Fleet for
     ... a notice's Fleet ... with the inspection fallback for a retired owner"); not its first
     subject. Other kinds keep 047c's rule.
   - **At the inspection** (047b's decision 4): for each listed action whose `kind` is `notice`,
     push a decision named `show-notice`, citing `PND-001`, `PND-002`, `PND-004` and `PND-007`,
     with `DL19`, and the facts `{"action": ID}`, in place of `raise-setup-action`; other kinds
     keep theirs.

   A notice is answered only through `answer.v1`: no setup, retirement or expiry answers it but
   its own expiry, and 047c's decision 6 function, which takes a computed setup action, does not
   touch it.

6. **Unit tests pin the notice.** In `pending.rs`'s tests, add one test,
   `notices_follow_their_fleet`, that builds `State` values by hand, as 047b's and 047c's tests
   do, and an `Allocation` from an `Evidence` whose `ids` are `n1` to `n9`, and asserts with
   `assert_eq!` each result below. Let T be `2026-11-12T12:00:00Z`. The objects: root `r`,
   `management`, the Fleets `f` and `g`, the Pool `p`, the Resource `x` (owner `p`), the Agents
   `a1` and `a2` (owner `f`) and `b1` (owner `g`), the Interfaces `v` (an open Conduit to `f`
   listing the core's `inspect.v1`), `w` (an open Conduit to `p` listing it) and `u`; and the
   Conduits, with no `spec`, `k1` (`a1` to `x`), `k2` (`a2` to `x`, not open), `k3` (`b1` to
   `x`) and `k4` (`v` to `x`), in that order in `state.objects`. Raise for `x` at T through a
   retirement invocation `t1`:
   - With `t1`'s `sender` and `caller` `u`: two notices, `n1` for `f` with subjects `[x, k1, k2]`
     and `n2` for `g` with `[x, k3]`; `n1` whole, with seven evidence items (decision 2's five,
     then one for `k1` and one for `k2`) and expiry `2026-11-19T12:00:00Z`; and the entry of
     `n1`, with `raisedBy` `t1`, `fleet` `f` and its `about`.
   - Each on a fresh copy of the state: with sender `a1`, `n1` for `g` alone; with sender `r`,
     none, and the `Allocation`'s cursor unmoved; with `x` a template, none.
   - After the raise with sender `u`, at T+1s: `n1` is raised at `r`, its view whole, and at `v`,
     its view subjects `[k1, k2]` and evidence the two Conduit items alone; it is raised at `w`,
     its view subjects `[x]` and evidence items 1 to 5; it is not raised at `u`. The count at `v`
     is 1, since `n2` is not raised there; the count at `w` is 2, since `n2` is raised there through
     `x` too. With `v` also holding an open Conduit listing the core's `inspect.v1` to `p`: its view
     at `v` is whole. At `n1`'s expiry it is raised at none of them, and the action `n1` names has
     the answer `{"value": "expired", "at": "2026-11-19T12:00:00Z"}`. Once `x` is removed from
     state, `n1` is raised at neither `w` nor A3: it is not raised at `w`, and A3's test is false
     for `w`.
   - A3's test is true for `r`, `v` and `w` on `n1`, unanswered and expired alike, and false for
     `u`; A4's target for `n1` is `f`, and `management` once `f` is retired.
   - `n1` answered `acknowledged` by `r` through invocation `d1` at T+1h, the test answering it
     through decision 5's store fn: the action `n1` names
     has that answer at T+8d; its entry keeps `fleet` and `about`; it is not raised at `r` or `v`
     at T+1h; after `state.records` is emptied, as a purge empties it, the answer is unchanged.
   - The accepted answers: `notice` accepts `acknowledged`, and neither `dismissed` nor
     `expired`; `setup` still accepts `dismissed` alone.

   Write the values as `json!` values; members a test does not need may be left out.

   In `retirement.rs`'s tests, add one test, `retirement_names_its_notices`, built as its
   `fixture` builds a `Transition`. It retires a Resource through `execute` with a non-root sender
   and an Agent consumer of another Fleet, and asserts four things: the `object-change` record
   whose action is `retired` has `pendingAction` equal to the notice IDs; the invocation's
   `notices` holds them; a `raise-notice` decision is pushed with facts
   `{"actions": [IDs], "resource": ID}`; and, with `cascade_parent` set, the parent invocation's
   `notices` holds them too.

7. **One portable case, through the e4 fixture.** Add
   `conformance/conventions/setup/retirement-notice/case.yaml`, ID `setup-retirement-notice`,
   `layers: [convention, interface]`, `promises: [PND-007, PND-002, PND-009, PND-003]`, title
   `Raise a notice for each other Fleet whose Conduit a Resource's retirement closes, name it in the result, and answer it acknowledged`,
   `profile: first-build` and `initial` `{"recipe": "empty"}`. Write it as JSON with two-space
   indentation and a final newline, its members in the order of `setup-action-answers`, as order
   046a's decision 11 writes its cases. Its `files` are those of `setup-action-by-scope`. It must
   meet 046a's portable-case rule as it stands. Below, ROOT is root's ID,
   `00000000-0000-4000-8000-000000000005`; a new step's evidence is the time
   `2026-11-12T12:00:00Z`, `ids` the 100 strings `00000000-0000-4000-8000-` followed by the
   twelve-digit, zero-padded decimal of 50000 + 100n + k, for k = 0 to 99, `revisions` the 16
   strings `notice-` followed by the decimal of n, a `-` and the decimal of j, and `suffixes` the
   16 strings `s` followed by the four-digit, zero-padded decimal of 100n + j, for j = 0 to 15,
   where n is the step's number; a copied step keeps its own evidence. `META/x` is the `_meta`
   key `x` under the prefix that `spec/names.yaml` sets, written as every case writes it; `U` is
   the JSON pointer `/reply/_meta/` followed by that key for `unanswered`, its `/` written `~1`,
   as 047b's case writes it; and `N0` and `N1` are the JSON pointers `/reply/_meta/` followed by
   that key for `pendingAction`, its `/` written `~1`, then `/0` and `/1`. "X asks T with A" is
   an `invoke` step whose request is
   `{"identity": {"kind": "instance", "id": X}, "tool": T, "arguments": A}`; "root asks" is ROOT's.
   REFUSED(R, RULE) is
   `{"reply": {"isError": true, "_meta": {"META/reason": R}, "structuredContent": {"diagnostics": [{"ruleId": RULE}]}}}`.
   Its steps:
   1. `setup-action-by-scope`'s step 1, `input` and `expect` whole.
   2. `setup-action-by-scope`'s step 2, `input` and `expect` whole.
   3. `setup-action-by-scope`'s step 3, `input`, `expect` and `capture` (`viewer`) whole.
   4. Root asks `management.inspect.v1` with
      `{"object": {"kind": "Interface", "name": "declared"}}`. `expect`
      `{"reply": {"isError": false}}`; `capture` `{"declared": "/reply/structuredContent/id"}`.
   5. Root asks `access.open.v1` with `{"conduit": "to-shared"}`. `expect`
      `{"reply": {"isError": false}}`.
   6. Root asks `access.open.v1` with `{"conduit": "viewer-fleet"}`. `expect` step 5's.
   7. Root asks `management.set-up.v1` with `{"instance": {"kind": "Interface", "name": "declared"}}`,
      as `e4-provenance-setup-parts`'s step 4 does: it opens `declared`'s Conduit `slot` to
      `store`. `expect` step 5's.
   8. `$declared` asks `slot.retire.v1` with `{"instance": {"kind": "Resource", "name": "shared"}}`.
      `expect` `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}, "absent": [N1, U]}`;
      `capture` `{"notice": N0}`: one notice, for `group`, whose Agent `a` consumed `to-shared`;
      `viewer-shared`'s consumer is no Agent, and `declared`'s scope does not hold `group`.
   9. `$viewer` asks `fleet.list.v1` with `{}`. `expect`
      `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"pending": [{"id": "$notice", "kind": "notice", "subject": [{"kind": "Conduit", "name": "to-shared"}], "expiry": "2026-11-19T12:00:00Z"}]}}}`:
      `viewer` may inspect `a` through `group`.
   10. `$declared` asks `slot.list.v1` with `{}`. `expect`
       `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/pending", U]}`:
       `shared` is removed, and `declared`'s scope does not hold `group` (decision 4).
   11. Root asks `group.list.v1` with `{}`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/unanswered": 1}, "structuredContent": {"pending": [{"id": "$notice", "kind": "notice", "subject": [{"kind": "Resource", "name": "shared"}, {"kind": "Conduit", "name": "to-shared"}]}]}}}`:
       root's view keeps the retained subjects whether or not they are still in state.
   12. Root asks `access.open.v1` with `{"conduit": "viewer-pool"}`. `expect` step 5's.
   13. `$viewer` asks `pool.list.v1` with `{}`. `expect` step 9's: a removed Resource fails
       decision 4's "exists in state" test, though `viewer` may now inspect its Pool.
   14. Root asks `store.answer.v1` with `{"action": "$notice", "answer": "acknowledged"}`.
       `expect` REFUSED(`refused`, `ACC-033`): a notice is answered through its Fleet.
   15. Root asks `group.answer.v1` with `{"action": "$notice", "answer": "dismissed"}`. `expect`
       REFUSED(`invalid`, `PND-009`).
   16. Root asks `group.answer.v1` with `{"action": "$notice", "answer": "acknowledged"}`. `expect`
       `{"reply": {"isError": false, "_meta": {"META/outcome": "succeeded"}}, "absent": [U], "record_facts": [{"type": "invocation-ended", "data": {"pendingAction": ["$notice"]}}, {"type": "pending-action-answered", "data": {"pendingAction": ["$notice"], "answer": {"id": "$notice", "kind": "notice", "answer": {"value": "acknowledged", "actor": ROOT, "at": "2026-11-12T12:00:00Z"}}}}]}`,
       with ROOT written as root's ID.
   17. Step 9's request. `expect`
       `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/pending", U]}`.
   18. Step 16's request. `expect` REFUSED(`conflict`, `PND-003`).

   Steps 4 to 18 are new; the route is new and has not been run, since order 047c is not yet
   built and no case retires a Resource after a first start through a Conduit that an
   Interface's setup opened. No case asserts the notice's ID in the `object-change` record: the
   step that raises it cannot name the ID it allocates, and a portable case may not assert
   stored records, so decision 6's `retirement_names_its_notices` pins decision 3's record. Before
   running it, check that no ID repeats across the case's steps' `ids`. If 046a's runner check refuses the
   case all the same, if a step copied from `setup-action-by-scope` gives a reply other than that
   case states, if `declared`'s setup opens no Conduit whose local name is `slot` or asks for any
   report, if the retirement of `shared` does not end at step 8, or if a step's reply differs from
   what it states, stop and report the error: do not change the expectations to pass it, and do
   not change the inputs beyond what this decision says. No expectation is generated from the
   engine's output. The coverage index entry holds `id`, `case`
   (`conventions/setup/retirement-notice/case.yaml`), `promises` and, where its neighbours in that
   piece hold it, `safety_rows: []`, in the members and order its neighbours use. Write it in a
   `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it into
   `conformance/conventions/setup/index.yaml` in ID order and leaves `manifest.yaml` byte for byte
   as it was.

8. **`engine/CONFORMANCE.md` and `engine/STANDING.md`.**
   - In DL19's row in "Choices still required", found by its text read across line breaks as
     047c leaves it, insert right before the sentence "No other kind is held, so no other
     duration is chosen.", in the same line:

     ```text
     A notice (PND-007, order 047d) expires seven days after the step of the retirement that raised it. One is raised for each Fleet, other than the sender's where the sender is an Agent, whose Agent consumed a Conduit that a Resource's retirement sent by an instance other than root closed; it is stored with an engine ID, named in the `pendingAction` key of the retirement's result and in the `pendingAction` of its `object-change` record, raised at root, at each Interface whose scope holds that Fleet, and, while the Resource's shutdown is unconfirmed, at each Interface whose scope holds the Resource, and answered `acknowledged` through `answer.v1` with a Conduit to that Fleet, or to `management` where that Fleet is retired or gone. At an Interface other than root it shows a closed Conduit and its evidence where that Interface may inspect the Conduit's consumer, and the Resource and its evidence where it may inspect the Resource while the Resource's shutdown is unconfirmed.
     ```

     Where 048a-2 has merged first, its sign-in sentence already stands before that sentence: put
     this text after it, still before "No other kind is held, so no other duration is chosen.",
     and say so. Nothing else in the file changes.
   - After the other decisions, run `python3 tools/promise_standing.py --write`, as AGENTS.md asks
     of a change to a case's `promises` and to a promise ID in the core's sources. Report the rows
     of PND-002, PND-003, PND-007 and PND-009 and the totals; the totals change only by what those
     rows change.

9. **The declaration.** `docs/work-orders/047d-declared.yaml` holds one `added` shape named
   `setup-retirement-notice` for that case. The coordinator wrote it with this order; check that
   it matches and change it only to make it so.

10. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or record
    type changes, and `pending-action.v1` and `evidence-item.v1` carry no member for decision 2's
    `fleet` or `about`. No kind but the notice is added: sign-in (048a-2 and after) and
    enrollment (049a-2) are not this order's, and no request is held. No CLI command (051b), shell
    code, tool list or record type changes, and nothing is relayed or delivered to a peer (batch
    2, PND-002). Report as findings, for the coordinator:
    - the stand of decision 4, as a question for the owner that says four things: it reads
      PND-002's "subjects and evidence about objects it may inspect" as each subject and item
      being about the object whose inspection shows it, a removed Conduit's being its consumer;
      without that reading an Interface whose scope holds only the notice's Fleet, which PND-007
      raises it at, may inspect none of its subjects, so it would see nothing; after a completed
      retirement only root sees the Resource and items 1 to 5, and LIF-040's tombstone
      visibility (the Resource through its kept owning Pool; a Conduit through either end as it
      stood) is the alternative reading the spec offers; and a sentence in PND-007 naming what
      each subject and evidence item is about, or a member of `evidence-item.v1` naming it, would
      settle it;
    - any place where PND-001, PND-003, PND-007, PND-009 or PND-012 left you a choice this order
      does not settle.

## Scope

Change only `engine/crates/core/src/pending.rs` (decisions 1, 2, 4, 5 and 6), `retirement.rs`
(decision 1's call and its shared test, decision 3's record, member and decision, and decision
6's test in its tests),
`requests.rs` (decision 3's key in `finish`, and nothing else), `records.rs` (decision 5's
inspection decision, the invalid's text by kind where 047c writes it, and the answer.v1 branch's
call of decision 5's store), `access.rs` (decision
5's arm of A4's fn), `cascade.rs` or `lifecycle.rs` only where decision 1 finds another place where
a Resource's Conduits close at its retirement, `engine/CONFORMANCE.md` (one place),
`engine/STANDING.md`, written only by the tool, `conformance/conventions/setup/index.yaml` and the
new `conformance/conventions/setup/retirement-notice/case.yaml`.
`docs/work-orders/047d-declared.yaml` comes with this order; change it only as decision 9 says.
`conformance/manifest.yaml` holds the new entry only until the pieces tool moves it, and ends
unchanged. Not `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, `lib.rs`, the shell, the
conformance runner, the profiles, the experiments, the case schema,
`conformance/internal-cases.yaml`, `conformance/expected-failures.yaml`, any other case or
fixture, or any record: report any defect as a finding. `experiments/011-short-form-spike/`
changes only if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- pending::tests
cargo test --release -p conduit-core --lib -- retirement::tests
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. setup-retirement-notice
cargo run --release -p conduit-conformance -- .. setup-action-
cargo run --release -p conduit-conformance -- .. e4-closed-with-target
cargo run --release -p conduit-conformance -- .. setup-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 4 passed and 0 failed: 047's test, 047b's, 047c's and this order's (5 where
048a-2 has merged, with its own). The fourth passes, with decision 6's
`retirement_names_its_notices` among its tests; report the count. The pieces tool prints `Nothing to
move.` The runner commands end `1 cases passed`, `3 cases passed` (`setup-action-at-root`,
`setup-action-by-scope` and `setup-action-answers`), `1 cases passed` (root's retirement of
`shared`, which raises no notice), and one more case than 047c's Record reports for `setup-`
(report the count). The shell test passes. If any other case, or any core, shell, CLI or
experiment test outside `pending::tests` and `retirement::tests`, fails because a retirement's
result now carries the key or a list or inspection a notice, stop and report it: do not change
it. The threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Next, show that the case fails without decisions 1 and 3's place in the retirement. From
`engine/`, one at a time:

```bash
cp crates/core/src/retirement.rs target/retirement.rs.fixed
git show HEAD:engine/crates/core/src/retirement.rs > crates/core/src/retirement.rs
cargo run --release -p conduit-conformance -- .. setup-retirement-notice
cp target/retirement.rs.fixed crates/core/src/retirement.rs
cargo run --release -p conduit-conformance -- .. setup-retirement-notice
```

The first runner command must fail, printing `FAIL 0 cases passed; 1 failed`, with an error at
step 8 that names the missing capture or the `pendingAction` key; after the restore the second
ends `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older modification
time, so cargo would not rebuild the crate. Git ignores `target/`. If `HEAD`'s file does not build
against the rest of your change, revert decision 1's call and decision 3's lines by hand in the
copy instead, and say so.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/047d-declared.yaml; echo "exit $?"
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/inventory conformance/profiles engine/crates/shell engine/crates/conformance engine/crates/core/src/lib.rs spec IN_FORCE.md tools packages docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/047d-declared.yaml: 1 shapes.`,
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

Each of these prints 1. The `tr` checks read the file with its line breaks as spaces, so a rewrap
hides nothing; the `sed` check reads `retirement.rs` only up to its `mod tests {` line, so
decision 6's test, which names `raise-notice` too, is not counted:

```bash
grep -c 'fn notices_follow_their_fleet' engine/crates/core/src/pending.rs
sed '/^mod tests {/,$d' engine/crates/core/src/retirement.rs | grep -c '"raise-notice"'
grep -c 'fn retirement_names_its_notices' engine/crates/core/src/retirement.rs
grep -c '"show-notice"' engine/crates/core/src/records.rs
grep -c '"id": "setup-retirement-notice"' conformance/conventions/setup/index.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'A notice (PND-007, order 047d) expires seven days'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'No other kind is held, so no other duration is chosen.'
grep -c '^| PND-007 | convention | must | 1 | 0 | outside | core | case passes |$' engine/STANDING.md
```

Each of these prints 0:

```bash
git diff HEAD --stat -- conformance/expected-failures.yaml conformance/conventions/setup/action-at-root conformance/conventions/setup/action-by-scope conformance/conventions/setup/action-answers | wc -l
git diff HEAD -- spec/core/results.yaml | wc -l
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the rows decision 8 names with the totals, the function names and signatures of
decisions 1, 4 and 5, which of 048a-2's functions you reused, every place where a Resource's
Conduits close at its retirement (decision 1), and every path decision 3 asks for.

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `047d-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.
