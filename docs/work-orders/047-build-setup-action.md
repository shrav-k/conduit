# Work order 047 (E6a): a failed setup's action, computed and raised at root's inspection

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share: list in `vectors` the case you add and the case whose expected
failure you lift, with the promises each tests and whether it passes. This order has network
access only because the engine's tests bind loopback ports, which the sandbox refuses without it;
use it for nothing else, add no crate, and reach no other machine. Run one cargo command at a
time. Never print a whole large file, case, fixture or log: use `head`, `tail`, `grep`, `sed -n`
or a count. Never run `env`, `printenv`, `set` or `export -p`, never print an environment
variable's value, and never read or print a key, a token or any credential. Do not run `claude`,
`codex` or any client, and sign in nowhere.

## What and why

**Where it sits.** Batch 1's core chain is 046a, 046c, 047, 047b, 047c, 048a-2, 048b-join, 050 and
053, 047b and 047c coming from the cut below. [Spec batch 1's
plan](../proposals/spec-batch-1/README.md) (its entry 047 under "The work orders") gives 047, "the
pending-action core", all of this at once: `pending.rs` with the model, routing by scope and
disclosure; `answer.v1` in place of its stub, with answers limited by holdings; the setup kind
(computed) and the notice kind; expiry for every kind; the `unanswered` and `pendingAction` keys;
the list and inspect projections; about 14 vectors and 1 to 1.5k lines. [Order
117](117-plan-s2-s3.md) sizes it L, after 046c. That is several behaviors, and an order holds one
and stays small ([FR-264](../proposals/full-review/findings/P01-work-orders-lanes.md#fr-264)). So
the coordinator cuts 047 into four, each after the one before (see "Who decided what"):

- **047, this order:** one kind and one place. An instance's setup action (PND-006) is computed in
  `pending-action.v1`'s form (PND-001), with an expiry under DL19, and raised at root's own
  inspection of the instance, in `status.pending` (PND-002, for root), until it expires
  (PND-003).
- **047b:** where every action is raised: root and each Interface whose scope holds a subject,
  never an Agent; evidence disclosed only as inspection allows; `list.v1`'s `pending` output; and
  the `unanswered` key on an Interface's own results (PND-002, PND-004).
- **047c:** `answer.v1` in place of its stub (PND-009), with stored answers: `dismissed` for a
  setup, `retried` by a further setup and `retired` by retirement recorded as answers, a
  dismissal or expiry that stays answered, `conflict` for an answered or expired action, and
  answered state that survives a purge (PND-003, PND-006, PND-012). Its refusal of who may
  answer takes its place in ACC-056's order of refusing rules, which order
  [171a](171a-so-a-form-pools-refusals.md) wrote and 046a builds.
- **047d:** the notice kind (PND-007), stored with an engine ID allocated within retirement,
  with its expiry, the `pendingAction` key on the retirement's result, and `acknowledged` through
  `answer.v1`.

Sign-in as a kind of pending action, PND-005, `SignInUndone` and the holds are 048a-2's; CTR-013's
checks on a Type and the `.v2` fixture are 048a-1's. This order builds neither, and none of 047b
to 047d.

**What it builds.** PND-006: "An instance is not retired and its latest set-up.v1 invocation ended
failed or unknown, including one whose outcome a reconciliation has since set under LIF-028",
then "One setup action is computed, with that invocation's ID, the instance as subject, and
evidence naming the invocation, outcome, reason and diagnostics where present, parts that did not
succeed and unreconciled IDs. It has an expiry under DL19." PND-002 has an action raised, "listed
... in inspect.v1's status.pending for the subject", at root while it "is unanswered and has not
expired". Under [ruling 80](../proposals/ruling-80/README.md), point 4, a reconciliation that
records the setup as having occurred leaves the action open; order
[172b](172b-so-b-reconcile-records-stop-ends.md) wrote that into PND-006 and PND-003 and added
the case `e4-reconcile-setup-action-open`, an expected failure whose case "passes once the
engine computes setup actions". This order makes it pass. The S9 plan
([experiment 020's plan](../../experiments/020-s9-start-and-repair/plan.md), its row on PND-006's
evidence) counts on 047 for the setup action's IDs and evidence.

**What the engine already does, so that you do not redo it.**

- The core holds no pending action: `State` (`engine/crates/core/src/lib.rs:75`) has no such map,
  and this order adds none, since a setup action is computed, never stored (PND-012: "Computed
  setup actions add none").
- A set-up.v1 invocation keeps its instance's ID as `object` (`lifecycle.rs:228`), its parts in
  `setup.parts`, each with `name`, `outcome`, `effect`, `optional` and `invocation`
  (`lifecycle.rs:348`), and ends through `requests::finish` (`requests.rs:1445`), called from
  `lifecycle::settle`, `lifecycle::cancel`, `requests::cancel` and `requests::recover`. Order 046c
  gives `finish` a `cause` and keeps it on an invocation that ended failed or unknown as the member
  `cause`, and gives a part held back under POOL-013 its own `diagnostics`.
- A reconciliation (`requests.rs:1295-1319`) overwrites the reconciled invocation's `state` with
  its outcome and writes an `invocation-ended` record with `reconciled: true`. So the outcome the
  setup ended with is lost once it is reconciled; this order keeps it (decision 1).
- `inspect.v1` (`records.rs:348`, in `execute`) builds `status` from the object, and adds
  `status.unreconciled` (`:425`). It gives no `status.pending`. The contract's
  `status.pending` is a list of `pending-action.v1` (`spec/core/operations.yaml`, `inspect.v1`;
  `spec/core/results.yaml`, `pending-action.v1` and `evidence-item.v1`).
- `answer.v1` is one of C11's stubs (`requests.rs:1047-1058`): it reports unavailable under
  PND-009 with the text `not built yet, order 047`, which `engine/CONFORMANCE.md` (DL19's row,
  `:71`, and the line at `:434`) and the shell test
  `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`
  (`engine/crates/shell/src/tests.rs:1611-1616`) repeat. It stays a stub here, naming 047c.

Line numbers are those of `main` at `cbb49fc1`, before orders 046a and 046c; find each place by
the text quoted with it.

**When this order starts.** Orders 046a and 046c must be merged to `main` first. Stop, report and
change nothing unless `conformance/internal-cases.yaml`,
`engine/crates/conformance/src/portable.rs`,
`conformance/conventions/explain/outcome-causes/case.yaml` and
`engine/crates/core/src/explain.rs` all exist, and `requests::finish` takes a `cause`: only their work creates them. Orders 046b,
048a-1 and 050 may land beside this one; whichever lands second keeps both changes and writes
`engine/STANDING.md` again with the tool. This order does not depend on S0b: it adds no map to
`State`.

**Who decided what.** Everything here is the coordinator's, each open to the owner's strike ([ruling
48](../proposals/ruling-48/README.md)): the cut of 047 into 047, 047b, 047c and 047d (FR-264), which
order 117's entry records beside 047, with their rows in the work orders' README, in the change
that brings this order; DL19's duration for a setup action, seven days, and that
an action is raised while a step's time is before its expiry and not at it; keeping the time and
outcome an invocation first ended with, as the member `ended`, and the fallback for an invocation
that ended before this order; ranking set-up.v1 invocations by dispatch, through the member
`dispatched`, rather than by their place in `state.invocations`; a redispatched setup that ends
failed or unknown again raises an action under the same invocation ID, with an expiry counted from
its new ending (PND-006: a later unsuccessful setup makes a new action), a reading of PND-006 that
is the coordinator's, open to the owner's strike; the evidence items' words; raising
the action at root alone in this order, with an Interface's scope left to 047b; reading "root" as an
inspection whose caller and sender are both root; leaving `status.pending` out where nothing is
raised; the case, its route and its times; and the stub's new order tag. Apply them as written and
report them under `decisions` as the coordinator's. Rulings 80, 81 and 82 and the targeted review's
[section 6](../proposals/review-signin-2026-11-16/README.md#6-must-state-lists) put nothing on 047;
nothing here touches sign-in.

**Read first:**

1. In `spec/`: PND-001, PND-002, PND-003, PND-006 and PND-012 (`spec/promises/pending.yaml`);
   LIF-028 (`spec/conventions/lifecycle.yaml`); DL19 (`spec/implementation-defined.yaml`);
   `inspect.v1` and `answer.v1` (`spec/core/operations.yaml`); `pending-action.v1`,
   `evidence-item.v1` and `object.v1` (`spec/core/results.yaml`); the rows pending action,
   raised at and setup action of `spec/definitions.md`.
2. Order 172b's decision 5 and its Record, and the case
   `conformance/conventions/inventory/e4/reconcile-setup-action-open/`.
3. Orders 046a (decisions 10 and 11, the portable cases) and 046c (decision 1's `cause`), as
   merged.
4. The case `s1-whole` (`conformance/conventions/s1/whole/case.yaml`), steps 1 to 8.

**Size: S-M**: about 0.2 to 0.3k changed lines of Rust with the unit test, one new case, one
expected failure lifted, and four places in `engine/CONFORMANCE.md`.

## The decisions it applies

1. **An invocation keeps how it first ended.** In `requests::finish`, where the invocation has no
   member `ended`, set `ended` to `{"time": T, "outcome": O}`, where `T` is the evidence time of
   the step being decided (the `Allocation`'s evidence `time`, as written) and `O` the outcome
   `finish` gives. Where `ended` is already present, leave it: a later `finish` of the same
   dispatch, and a reconciliation, which does not call `finish`, change `state` but never
   `ended`. A redispatch under OPS-012 builds a fresh invocation object and starts without
   `ended`: do not carry `ended` over from `previous`, so the redispatch's own `finish` sets it
   again. Every invocation that `finish` ends gets it; nothing reads it but decision 2. If a
   set-up.v1 invocation can leave `in-progress` by a path that does not call `finish`, report it,
   and set `ended` there too in the same form.

   **A setup invocation keeps when it was dispatched.** In `requests`, where the invocation is
   written to `state.invocations` (at a first dispatch, pushed; at a redispatch, written at its
   old index), give each invocation whose `operation` is the core's `set-up.v1` the member
   `dispatched`, an integer: one more than the greatest `dispatched` held by any invocation in
   state, or 1 where none holds one. It is set at every dispatch and every redispatch, so a
   redispatch of an earlier setup ranks after every setup dispatched before it. Nothing reads it
   but decision 2.

2. **The setup action, computed.** Add `engine/crates/core/src/pending.rs` (`mod pending;` in
   `lib.rs`), with a function that takes the state and an object and returns the setup action of
   that object, whatever the time, or none; whether it is raised at a time is a separate test,
   below. Its name and signature are your choice; it needs no `Context` beyond the qualified name
   of the core's `set-up.v1`, which its caller may pass.
   - **Which invocation.** The latest set-up.v1 invocation of the object is, among the invocations
     whose `operation` is the core's `set-up.v1` and whose `object` is the object's `id`, the one
     with the greatest `dispatched` (decision 1). An invocation without the member (made before this
     order) ranks below every invocation that holds one, and among themselves such invocations rank
     by their place in `state.invocations`, the last the latest. There is an action only where the
     object's `state` is not `retired`, that invocation exists, and its first ending outcome is
     `failed` or `unknown`. Its first ending is its `ended`. Where it has no `ended` (it ended
     before this order), the first ending is the `time` and `data.outcome` of the latest record of
     type `invocation-ended` whose `data.invocation` is the invocation's ID and whose
     `data.reconciled` is not `true`; where there is none (a purge removed it) or the invocation is
     still `in-progress`, there is no action. A later setup that is in progress or ended succeeded
     is the latest, so it leaves no action; so is an earlier setup redispatched under OPS-012 after
     a later one, since its redispatch sets `dispatched` again (PND-006: a further set-up.v1 answers
     it retried; ruling 80, point 4: open until a later setup answers it); a reconciliation that set
     the outcome since leaves the action as it was (ruling 80, point 4).
   - **Its form** is `pending-action.v1`'s, members in this order:
     `{"id": ID, "kind": "setup", "subject": [{"kind": K, "name": N, "id": I}], "evidence": E, "expiry": X}`,
     where `ID` is the invocation's ID (PND-001: "A setup action uses its setup invocation's
     ID"), `K`, `N` and `I` are the object's `kind`, `name` and `id`, and there is no `answer`.
   - **Its expiry** `X` is the first ending's time plus seven days, parsed and written with
     `chrono` as RFC 3339 in UTC with `Z`, seconds shown and a fraction only where nonzero
     (`SecondsFormat::AutoSi`). For an ending at `2026-11-09T12:30:00Z` it is
     `2026-11-16T12:30:00Z`.
   - **Its evidence** `E` is a list of `evidence-item.v1` items, each `{"kind": "fact", "value":
     V, "where": W, "proves": P}`, in this order, the last three only where their value is not
     empty:
     1. `V` the invocation's ID; `W` `records`; `P`
        `the set-up.v1 invocation that did not succeed`.
     2. `V` the first ending's outcome; `W` `records`; `P` `the outcome that setup ended with`.
     3. `V` the invocation's `cause` (order 046c), where it is a non-empty list; `W` `records`;
        `P` `the diagnostics that setup ended with`.
     4. `V` the list, in the order of `setup.parts`, of each part whose `outcome` is not
        `succeeded`, as an object of its `name`, then its `outcome`, `reason` and `diagnostics`
        where the part holds them; `W` `status.setup`; `P`
        `the parts of that setup that did not succeed`.
     5. `V` the IDs, without repeats, of the invocations among the setup itself and its parts'
        `invocation` values (strings only, in the order of `setup.parts`, after the setup's own)
        whose `state` is now `unknown`; `W` `status.unreconciled`; `P`
        `the invocations of that setup whose outcome is still unknown`.

     A set-up.v1 invocation of this build holds no `reason` of its own; a part's reason is in
     item 4. Every item is a fact the engine observed (PND-004); none is a code or a fingerprint.
   - **Raised until it expires.** A second function says whether an action is raised at a time:
     where the time is before the expiry, compared as instants, it is raised; at the expiry and
     after, it is not (PND-003: once its expiry passes, the answer is expired and nothing is
     dispatched; this order stores no answer and dispatches nothing).

3. **Raised at root's inspection.** In `records::execute`, in the `inspect.v1` branch, right after
   `status["unreconciled"]` is set: where the object is not a template (it has no `spec`), the
   inspection's invocation has `caller` and `sender` both equal to root's ID, and decision 2 gives
   an action that is raised at the step's evidence time, set `status["pending"]` to a list of
   that one action, and push one decision named `raise-setup-action`, citing `PND-001`,
   `PND-002`, `PND-003` and `PND-006`, with `DL19`, and the facts `{"action": ID}`. Root is the
   object whose `from` is the core's `root`, as `access.rs` finds it (`view.management("root")`);
   reuse or expose that lookup rather than writing another. Otherwise leave `status.pending` out:
   an Agent's inspection carries no pending action (PND-002), and an Interface's scope is 047b's.
   Nothing else in the inspection changes. No case asserts the non-root branch: the verifier
   reviews it in the code.

4. **A unit test pins the computation.** In a `#[cfg(test)] mod tests` in `pending.rs`, add one
   test, `setup_action_follows_the_latest_setup`, that builds `State` values by hand and asserts
   with `assert_eq!` the whole action, or its absence, for each of these. Let T be
   `2026-11-09T12:30:00Z` and the object `{"id": "o1", "kind": "Resource", "name": "shared", "state": "registered"}`.
   - A setup `s1` ended `failed` at T (`ended` set), its parts `[{name: run, outcome: failed,
     effect: false, optional: false, invocation: r1}]`, its `cause` one diagnostic, and `r1` an
     invocation in state `failed`: the action with evidence items 1 to 4, item 4's value
     `[{"name": "run", "outcome": "failed"}]`, and expiry `2026-11-16T12:30:00Z`; raised at
     `2026-11-16T12:29:59Z`, not raised at `2026-11-16T12:30:00Z`.
   - `s1` ended `unknown` at T, its `state` still `unknown`, its part `run` `unknown` with
     invocation `r1` in state `unknown`: item 2's value `unknown`, item 5's value `["s1", "r1"]`.
   - The same, then `s1`'s `state` set to `succeeded` as a reconciliation sets it: the action
     stays, item 2's value `unknown`, item 5's value `["r1"]`.
   - `s1` as in the first bullet, then a later setup `s2` of the object in state `in-progress`:
     no action. With `s2` ended `succeeded` instead: no action.
   - `s1` as in the first bullet with `dispatched` 1, then a later setup `s2` of the object
     ended `failed` with `dispatched` 2, then `s1` redispatched: `s1` in state `in-progress`,
     with `dispatched` 3 and no `ended`, at its old place before `s2` in `state.invocations`: no
     action. With `s1` then ended `succeeded` (`ended` set, outcome `succeeded`): no action.
   - s1 failed at T, then redispatched and ended failed at T+8d, gives the action with expiry
     T+15d: `s1` as in the first bullet with `dispatched` 1, then redispatched and ended `failed`
     again at `2026-11-17T12:30:00Z` (`dispatched` 2, `ended` that time and outcome): the action
     with ID `s1` and expiry `2026-11-24T12:30:00Z`.
   - The first bullet with the object's `state` `retired`: no action.
   - `s1` ended `failed` with no `ended`, and one record
     `{"type": "invocation-ended", "time": "2026-11-10T08:00:00Z", "data": {"invocation": "s1", "outcome": "failed"}}`:
     the action, expiry `2026-11-17T08:00:00Z`, item 2's value `failed`. With no such record:
     no action.

   Write the values as `json!` values; members a test does not need may be left out.

5. **One portable case, through root's own inspection.** Add
   `conformance/conventions/setup/action-at-root/case.yaml`, ID `setup-action-at-root`, `layers:
   [convention, interface]`, `promises: [PND-006, PND-001, PND-002, PND-003]`, title
   `Raise a failed setup's action at root's inspection of its instance, until it expires`,
   `profile: first-build` and `initial` `{"recipe": "empty"}`. Write it as JSON with two-space
   indentation and a final newline, its members `id`, `layers`, `promises`, `title`, `profile`,
   `initial`, `files` and `steps`, in that order, as order 046a's decision 11 writes its cases.
   Its `files` are `s1-whole`'s, whole: the new directory is at the same depth, so every path
   holds. It must meet 046a's portable-case rule as it stands. Its steps, against `s1-whole`'s:
   1. `s1-whole`'s step 1 `input`, whole. `expect`: its `reply`, `external_intents` and
      `required_promises` as `s1-whole` states them, `state` `{"started": true}` and
      `"record_facts": []`, placed as 046a's `explain-refusal-tiers` step 1 places it; not its
      `records` or `absent`. No `capture`. The case schema requires `records` or `record_facts`
      in the `expect` of these input kinds.
   2. `s1-whole`'s step 2, `input` and `expect` whole.
   3. `s1-whole`'s step 3 `input`, whole. `expect`: its `reply`, `external_intents` and
      `required_promises`, `state` `{"started": true}` and `"record_facts": []`, placed as in
      step 1, which the case schema requires for this input kind too; not its `records`,
      `object_count` or `object_facts`.
   4. to 7. `s1-whole`'s steps 4 to 7, each `input` whole and step 6's `capture` (`shared`),
      `expect` only its `reply`.
   8. `s1-whole`'s step 8 `input`, whole except that `report.outcome` is `failed`. `expect`: its
      `reply` with `META/outcome` `failed` for `succeeded`, where `META/outcome` is the `_meta`
      key `outcome` under the prefix that `spec/names.yaml` sets, written as every case writes
      it; nothing else.
   9. `kind: invoke`, evidence time `2026-11-16T12:29:59Z`, `ids` the 30 strings
      `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of 20000 + k,
      for k = 0 to 29, and the request
      `{"identity": {"kind": "instance", "id": "00000000-0000-4000-8000-000000000005"}, "tool": "root-workspace.inspect.v1", "arguments": {"object": {"kind": "Resource", "name": "shared"}}}`.
      `expect`:
      `{"reply": {"isError": false, "structuredContent": {"status": {"pending": [{"id": "$shared", "kind": "setup", "subject": [{"kind": "Resource", "name": "shared"}], "expiry": "2026-11-16T12:30:00Z"}]}}}}`.
   10. The same request, evidence time `2026-11-16T12:30:00Z`, `ids` as step 9's with 20100 + k.
       `expect` `{"reply": {"isError": false}, "absent": ["/reply/structuredContent/status/pending"]}`.

   The case asserts no evidence item: decision 4 pins them, and order 046c's `cause` text is
   not this order's to assert. If 046a's runner check refuses the case all the same, or a step's
   reply differs from what it states, stop and report the error: do not change the expectations
   to pass it. No expectation is generated from the engine's output. The coverage index entry
   holds `id`, `case` (`conventions/setup/action-at-root/case.yaml`), `promises` and, where its
   neighbours in that piece hold it, `safety_rows: []`, in the members and order its neighbours
   use. Write it in a `cases` list in `conformance/manifest.yaml`, then, from `engine/`, run
   `cargo run --release -p conduit-conformance --example pieces` with no flag, which moves it into
   `conformance/conventions/setup/index.yaml` in ID order and leaves `manifest.yaml` byte for byte
   as it was.

6. **The expected failure lifted.** In `conformance/expected-failures.yaml`, remove the entry whose
   `id` is `e4-reconcile-setup-action-open`, and nothing else; the case itself does not change.
   Its step 6, root's inspection after the reconciliation, now finds the action.

7. **The stub names 047c.** In C11's stub table in `requests.rs`, `("answer.v1", "047",
   "PND-009")` becomes `("answer.v1", "047c", "PND-009")`. In the shell test
   `first_start_presents_final_lists_and_unbuilt_operations_name_their_orders`, the order tag
   `"047"` beside `management.answer.v1` becomes `"047c"`. Nothing else in either changes.

8. **`engine/CONFORMANCE.md`.**
   - DL19's row in "Choices still required" becomes, as one line:

     ```text
     | DL19 | Partly built. A setup action (PND-006) expires seven days after its set-up.v1 invocation first ended, counted from the time of the step that ended it, and is raised while a step's time is before its expiry and not from then on. A setup that ended before order 047 kept no end time: its expiry counts from the last `invocation-ended` record of that invocation that no reconciliation wrote, and where none remains, as after a purge, it is not raised. It is raised only at root's own inspection of its instance, in `status.pending`; an Interface's scope, `list.v1`'s `pending` output and the `unanswered` key come with order 047b. `answer.v1` answers unavailable under PND-009, with the text "not built yet, order 047c" LINK, and no other kind is held, so no other duration is chosen. |
     ```

     where `LINK` is the row's link to `requests.rs` as it stands today, kept as it is.

   - In the line that reads "`sign-in.v1` and `answer.v1` answer unavailable, naming orders 048a
     and 047 respectively", `047` becomes `047c`.
   - In the sentence that reads "The shell also passes answer.v1 and sign-in.v1 to the core,
     which answers each unavailable, naming the order that builds it (047 and 048a; OPS-005).",
     `047` becomes `047c`; the rest is unchanged.
   - In the section "Refusals and reasons (order 046a)" that order 046a adds, where it lists as
     unfinished the stubs of `answer.v1` and `sign-in.v1` with their orders (`047, 048a-2`),
     `047` becomes `047c`; the rest is unchanged. Nothing else in the file changes.
   - `engine/STANDING.md`: after the other decisions, run `python3 tools/promise_standing.py
     --write`, as AGENTS.md asks of a change to a case's `promises`, to the expected failures and
     to a promise ID in the core's sources. The rows of PND-001, PND-002, PND-003 and PND-006
     become `case passes` with the source `core`, and the totals change only by what those rows
     change; report each row and the totals.

9. **The declaration.** `docs/work-orders/047-declared.yaml` holds one `added` shape named
   `setup-action-at-root` for that case. The coordinator wrote it with this order; check that it
   matches and change it only to make it so.

10. **What this order does not do.** No promise, schema, definition, `IN_FORCE.md` line or record
    changes. No pending action is stored, answered, listed by `list.v1` or counted in an
    `unanswered` key, and no scoped Interface sees one: 047b to 047d. Ruling 80's point 4 is
    built as PND-006 states it; order 172b's Record left a follow-up, that the evidence of an
    action whose setup a reconciliation set could name that reconciliation's invocation and its
    `checked` text, which no promise states yet: do not add it. Report as a finding for 047c
    that PND-001 gives the new action of a redispatched setup (decision 2) the ID of the old,
    answered or expired, one.

## Scope

Change only `engine/crates/core/src/pending.rs` (new), `lib.rs` (the module line, and the root
lookup if decision 3 needs it exposed), `requests.rs` (`finish`, the member `dispatched` where a set-up.v1 invocation is written, and
the stub's tag),
`records.rs` (the inspection), `access.rs` only to expose the root lookup,
`engine/crates/shell/src/tests.rs` (one tag), `engine/CONFORMANCE.md` (four places),
`engine/STANDING.md`, written only by the tool, `conformance/expected-failures.yaml` (one entry
removed), `conformance/conventions/setup/index.yaml` and the new
`conformance/conventions/setup/action-at-root/case.yaml`. `docs/work-orders/047-declared.yaml`
comes with this order; change it only as decision 9 says. `conformance/manifest.yaml` holds the
new entry only until the pieces tool moves it, and ends unchanged. Not `spec/`, `IN_FORCE.md`,
`tools/`, `packages/`, the experiments, the case schema, `conformance/internal-cases.yaml`, any
other case, or any record: report any defect as a finding. `experiments/011-short-form-spike/`
changes only if its tests need it, since it depends on the core; report why.

## Done when

From `engine/`, one command at a time, each passes:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test --release -p conduit-core --lib -- pending::tests::setup_action
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. setup-action-at-root
cargo run --release -p conduit-conformance -- .. e4-reconcile-setup-
cargo run --release -p conduit-conformance -- .. setup-
cargo test --release -p conduit-shell first_start_presents_final_lists
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The third reports 1 passed and 0 failed. The pieces tool prints `Nothing to move.` The runner
commands end `1 cases passed`, `2 cases passed` with no expected failure, and `40 cases passed`
(the 39 cases of `conventions/setup/` at `cbb49fc1` and this one; if an order since added one,
report the count). The shell test passes. The threads are 4 because at 24 the runner ran out of
memory (order 071's Record).

Next, show that both cases fail without decision 3. From `engine/`, one at a time:

```bash
cp crates/core/src/records.rs target/records.rs.fixed
git show HEAD:engine/crates/core/src/records.rs > crates/core/src/records.rs
cargo run --release -p conduit-conformance -- .. setup-action-at-root
cargo run --release -p conduit-conformance -- .. e4-reconcile-setup-action-open
cp target/records.rs.fixed crates/core/src/records.rs
cargo run --release -p conduit-conformance -- .. setup-action-at-root
cargo run --release -p conduit-conformance -- .. e4-reconcile-setup-action-open
```

The first two runner commands must each fail, printing `FAIL 0 cases passed; 1 failed`, with an
error at step 9 for the first and at step 6 for the second, each beginning
`/reply/structuredContent/status/pending: missing required field` after its step; after the
restore each ends `1 cases passed`. Restore with `cp`, not `mv`: a moved file keeps its older
modification time, so cargo would not rebuild the crate. Git ignores `target/`. If `HEAD`'s
`records.rs` does not build against the rest of your change, revert decision 3's lines by hand
in the copy instead, and say so.

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
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/047-declared.yaml; echo "exit $?"
git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/conventions/inventory spec IN_FORCE.md tools packages docs/implementation docs/proposals
git diff --check
```

`vector_delta.py` prints `Declarations read from docs/work-orders/047-declared.yaml: 1 shapes.`,
then `Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`; if it counts the
lifted expected failure as a change, report its lines and stop. `promise_standing.py --check`,
`check_model_account.py` and `check_roadmap.py` each end `Errors: 0`. The unit tests, run with
the interpreter that has PyYAML, end `OK`; report the count. The `git status` command prints
nothing.

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
grep -c '("answer.v1", "047c", "PND-009")' engine/crates/core/src/requests.rs
grep -c 'not built yet, order 047c' engine/CONFORMANCE.md
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'naming orders 048a and 047c respectively'
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'naming the order that builds it (047c and 048a; OPS-005)'
grep -c '^| DL19 | Partly built. A setup action (PND-006) expires seven days' engine/CONFORMANCE.md
grep -c '"raise-setup-action"' engine/crates/core/src/records.rs
grep -c '^mod pending;' engine/crates/core/src/lib.rs
grep -c '"id": "setup-action-at-root"' conformance/conventions/setup/index.yaml
grep -c '^| PND-006 | convention | must | 2 | 0 | outside | core | case passes |$' engine/STANDING.md
```

Each of these prints 0:

```bash
grep -c 'e4-reconcile-setup-action-open' conformance/expected-failures.yaml
grep -c 'not built yet, order 047"' engine/CONFORMANCE.md
grep -c '"047",' engine/crates/shell/src/tests.rs
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -Ec '\(047( and|,) 048a'
```

`git status --short` lists only the files in scope. Do not stage anything. In your report, mark
each choice you made yourself as your own, give the last line of every check, each exit status and
count above, the rows decision 8 names with the totals, the function names and signatures of
decision 2, and every path by which a set-up.v1 invocation leaves `in-progress` (decision 1).

`git diff --check` and `check_private_paths.py` read tracked files only, so the admin then stages
the new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`.
Before the merge the admin runs the local gate that [ruling 80](../proposals/ruling-80/README.md),
point 10, has each merge wait on, `tools/gate.py`; CI runs on the pull request as well, and its
`docs` job reads `047-declared.yaml` through `--declared-changed`, and the merge does not wait
for it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).
