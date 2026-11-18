# Work order 046b (E5b): tier 2 for a caller that may inspect, and closed Conduits from tombstones

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. List in `vectors` the case you add, with the promises it tests and
whether it passes. This order has network access only because the engine's tests bind loopback
ports, which the sandbox refuses without it. Use it for nothing else, add no crate, and reach no
other machine. Run one cargo command at a time. Never print a whole large file, case, fixture or
log: use `head`, `tail`, `grep`, `sed -n`, `wc` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

**Where it sits.** Batch 1's plan has one order 046, "refusals and explicitness"
([the plan](../proposals/spec-batch-1/README.md), its entry 046 under "The work orders"), whose
build lists "the tier decision; the tier-2 list; closed Conduits from tombstones".
[Order 117](117-plan-s2-s3.md) (lines 81 to 87) split it: "046b: closed Conduits from tombstones.
M. After 042 and 046a. Order 046a, as written, moves tier 2 for callers other than root to 046b."
[Order 046a](046a-build-refusal-tiers-and-diagnostics.md), merged as #345, says so in "What it
leaves, and to whom": "Tier 2 for callers other than root ... and closed Conduits from tombstones
in tier 2: order 046b." Its decision 2 ends "Closed Conduits from tombstones are 046b's", and
`engine/CONFORMANCE.md`'s section "Refusals and reasons (order 046a)" lists both as unfinished,
for this order. [Order 046c](046c-build-outcome-diagnostics.md) is merged too (#350). Order 042,
merged, keeps the tombstones this order reads.

**What it builds.** One behavior: tier 2 of a refusal, as ACC-056 states who gets it and what it
holds. ACC-056's `then`: "Tier 2 is always given to root. Another caller gets it only where it may
inspect the authorizing target under ACC-033 and every object the request names, through open
Conduits with no filter. For an operation that ACC-033 does not route, the authorizing target is
the request's target; for a request that a Pool or a Fleet authorizes, no caller but root and a
holder of `inspect.v1` on `management` meets that condition. Tier 2 adds the resolved target and
the caller's Conduits to it, each with its name, local name, state and operations, or states that
the named object does not exist." Its observable: "The Conduits in tier 2 include open ones,
registered ones not yet open and closed ones whose tombstones remain (LIF-040)." `refusal.v1`
(`spec/core/results.yaml`) already allows a Conduit's `state` `closed`. Ruling 52, point 1, says the
same to the reader of [IN_FORCE.md](../../IN_FORCE.md) (its line on ACC-056, about line 150): "A
caller that may inspect the target is also shown which Conduits from it to the target exist, open
or closed, and what each lists, or that the target does not exist; root is always shown them."
Today the engine gives tier 1 to every caller but root, and its tier 2 lists only Conduits that are
still objects, so a Conduit that was closed, which LIF-040 turns into a tombstone, is never shown.
Both halves are one behavior: who sees tier 2, and what tier 2 holds about the caller's Conduits.

**What 046a brought, which this order builds on.** As merged:

- `engine/crates/core/src/explain.rs`: `diagnostic(state, route, c, reason, rule)`, which builds
  the one error diagnostic of every reason, `properties.refusal` among it; `tier(state, sender, c)`,
  which gives 2 where the sender is root and 1 otherwise; `access_route`, which resolves a tool's
  spelling through the sending instance's live Conduits; `ids`, the route of the inputs that name a
  request by IDs; and `acc033_target`, which describes ACC-033's row in tier-1 words.
- `refusal.v1`'s fields as 046a's decision 2 fills them: `tier`; `operation`; `named` (`tool`,
  `conduit` where the sending instance holds a live Conduit of the tool's local name, and
  `objects`); `cause`; and, at tier 2, `target`, `exists` and `conduits`, where `conduits` holds the
  sending instance's Conduits to `target` that are not retired and have no `spec`, each
  `{name, as, state, operations}` with `state` `open` or `registered`.
- Tier 2 for root, with the records of 046a's decision 5: a refusal's records hold `refusalTier`,
  computed at five sites by calling `explain::tier` with the sender alone (`access.rs`, the
  access-decision data; `lib.rs`, `end_request`; `lifecycle.rs`, a refused setup part;
  `requests.rs`, twice). The diagnostic computes its own tier inside `diagnostic`.
- The case `explain-refusal-tiers`, whose step 2 asserts root's tier 2, and the portable-case rule
  of 046a's decision 10, which this order's case follows.

**What it leaves, and to whom.** Nothing of 046a's other unfinished items is this order's: F2's
build and vectors with F3's other vectors (the admin's order, not yet numbered), the https fetch
(046d), the -32602 form of a call with no established instance (046e), and the stubs of
`answer.v1` and `sign-in.v1` (047c, 048a-2). Pending actions, their scope and their disclosure are
orders 047 and 047b's: this order changes nothing in `records.rs`, `inspect.v1` or `list.v1`.
Tier 1 does not change: its `operation`, `named` and `cause`, and every message, stay as 046a
builds them.

**Who decided what.** ACC-056, ACC-033, LIF-040 and `refusal.v1` decide who gets tier 2 and what it
holds. Everything else is the coordinator's, each open to the owner's strike
([ruling 48](../proposals/ruling-48/README.md)); apply them as written, and report them under
`decisions` as the coordinator's:

- order 046a's move of tier 2 for callers other than root into this order, beside order 117's
  closed Conduits from tombstones (FR-264: one behavior, as "What it builds" says);
- reading ACC-056's condition through the engine's existing inspection routes: "may inspect X
  under ACC-033" is ACC-033's rows 6, 6A and 7 for X, as `access::may_inspect` computes them, and an
  object that does not exist is inspected through the first-start `management` instance itself
  (ACC-033's observable: "Authority to inspect it or act on it by a stale reference is
  `management`'s alone"), not through the route `may_inspect` would choose for the `management`
  Resource. IN_FORCE.md's words, "a caller that may inspect the target", are shorter than
  ACC-056's; where they differ, `spec/` governs ([ruling 67](../proposals/ruling-67/README.md));
- the authorizing target is what `access::routing` gives for the Conduit the explanation resolves,
  as `access::evaluate_path` uses it, plus the live owner of a registration's rows 1, 1A and 3;
  where the explanation resolves no Conduit and no tombstone (decision 2), there is no authorizing
  target and a caller other than root gets tier 1. ACC-056 says "only where", so this discloses
  less than the promise allows and nothing it forbids, as 046a reasoned for its own interim;
- where the sending instance holds no live Conduit of the tool's local name but a tombstone of one,
  tier 2's `target` and `conduits` are read through that tombstone, while `exists` keeps 046a's
  rule (false, since no such Conduit exists) and tier 1 keeps the spelling: ACC-056's last paragraph
  forbids disclosing "a past tool list or hidden object ... to reconstruct the name" at tier 1, and
  tier 2 goes only to a caller that LIF-040 lets see the tombstone;
- a caller other than root is shown a closed Conduit only where `inventory::visible` (LIF-040's
  visibility) gives true for it; root is shown every one ("root is always shown them");
- the order of `conduits`, live ones first as now, then closed ones by tombstone `time` and then
  tombstone ID;
- ACC-056's "caller" stays the sending instance, as 046a's decision 2 reads it;
- the case, its route through order 042's e4 fixture, its IDs, times and promises, and the size.

**When this order starts.** Orders 046a and 046c must be merged to `main`. Before you change
anything, from the repository root, run:

```bash
grep -c 'pub const REFUSAL_CAUSES' engine/crates/core/src/explain.rs
grep -c 'pub const OUTCOME_CAUSES' engine/crates/core/src/explain.rs
test -f conformance/conventions/explain/outcome-causes/case.yaml; echo "case $?"
grep -c 'pub tombstones: BTreeMap<String, Value>' engine/crates/core/src/lib.rs
grep -c 'pub(crate) fn visible' engine/crates/core/src/inventory.rs
grep -c 'closed Conduits from tombstones: order 046b' engine/CONFORMANCE.md
```

They print `1`, `1`, `case 0`, `1`, `1` and `1`. If any prints otherwise, stop, change nothing and
report it. Orders 047, 047b, 048a-2 and 050 may land beside this one; whichever lands second keeps
both changes and writes `engine/STANDING.md` again with the tool. This order does not depend on S0b.

**Read first:**

1. In `spec/`: ACC-056 and ACC-033 (`spec/conventions/access.yaml`), LIF-040 and LIF-013
   (`spec/conventions/lifecycle.yaml`), `refusal.v1` and `object.v1` (`spec/core/results.yaml`), and
   the rows filter, requirement and `management` of `spec/definitions.md`.
2. Order 046a whole, with its Record, above all decisions 2 and 5, and order 046c's "Who decided
   what".
3. Under `engine/crates/core/src/`:
   - `explain.rs`: `tier`, `access_route`, `ids`, `diagnostic` (its tier-2 block) and the tests
     `tiers_and_ids_conduit_follow_sender` and `tier_two_exists_resolves_ids_and_typed_names`, with
     the test `fixture`;
   - `access.rs`: `open`, `requirements`, `bound`, `current_path`, `may_inspect`, `routing` and the
     ACC-033 checks of `evaluate_path` that follow its call (registration's owner and
     `register-replacement.v1`'s Pool), and the access-decision data that sets `refusalTier`;
   - `inventory.rs`: `make` (what a tombstone keeps), `endpoint` and `visible`;
   - `lifecycle.rs`: `close.v1`'s branch, which makes the tombstone and removes the Conduit, and the
     refused setup part that sets `refusalTier`;
   - `lib.rs`: `end_request`, and the refusing sites of `Submit` and `Request`;
   - `requests.rs`: the two sites that set `refusalTier`;
   - `registration.rs`: its three refusals.
4. The cases `explain-refusal-tiers` (`conformance/conventions/explain/refusal-tiers/`),
   `e4-inventory-from-first-start`, `e4-inventory-scope` and `e4-as-after-close`
   (`conformance/conventions/inventory/e4/`), and that directory's `documents.json`: the Interface
   `viewer`, the Pool `store`, the Resource `shared` (owner `store`), and the Conduits `viewer-pool`
   (local name `pool`, from `viewer` to `store`, listing `inspect.v1`, `list.v1`, `reconcile.v1` and
   `set-up.v1`) and `viewer-shared` (no `as`, so its local name is its name, from `viewer` to
   `shared`, listing the fixture's `ping.v1`).
5. `conformance/README.md`, its sections "Assertions" and "Portable cases".

Two shorthands, as order 046a's: `CORE#name` is a core operation written as the cases write one,
with the core prefix that `conformance/README.md` defines; `META/reason` is the `_meta` key
`reason` under the prefix that `spec/names.yaml` sets, written as every case writes it. Line
numbers are not given: find each place by its name.

**Size: S-M**: about 0.2 to 0.35k changed lines of Rust with tests, one new case, and one section of
`engine/CONFORMANCE.md`.

## The decisions it applies

1. **Who gets tier 2** (ACC-056). In `explain.rs`, `tier` takes the route that the refusing site
   gives `diagnostic` (`tier(state, route, c)`), and gives:
   - 2 where the route's `consumer`, the sending instance, is root, as now;
   - 1 where the route has no `consumer`, as now (a tools/list refused before any instance is
     established among them);
   - for any other sending instance, Interface or Agent alike (ACC-056: "Another caller"), 2 only
     where both hold, and 1 otherwise:
     - **the authorizing target.** There is one, and the sending instance may inspect it. The
       authorizing target is what `access::routing` gives as its first value for the route's Conduit
       (decision 2 says which Conduit), the route's `operation` (decision 2 says which operation
       where the Conduit is read through a tombstone) and its `arguments`. For a route that
       `explain::ids` builds whose `arguments` hold `object`, call `routing` with arguments in which
       `instance` also holds that value, for that call alone. A route that `explain::ids` built is
       one with a `conduit` member and no `tool` member. `ids` builds such a route, and so does
       `requests.rs`'s IFC-026 rejection of a provider's refusal at dispatch. Treat that route
       alike: the copy changes `routing` only for `set-up.v1`, `retire.v1` and
       `register-replacement.v1`, which never reach that rejection. `named.objects` and `exists` are
       unchanged. For an operation that ACC-033 does not route, `routing` already gives the
       Conduit's target that is not retired, as ACC-056 asks. For `register.v1`, each live Pool or
       Fleet that a document in the route's `arguments.documents` names as its owner, found as
       `evaluate_path`'s registration check finds it, is an authorizing target too, and for
       `register-replacement.v1` the live Pool that `routing` gives as its second value: ACC-033's
       rows 1, 1A and 3 name two targets, and the sending instance must be able to inspect each.
       Where the route resolves no Conduit and decision 2 finds no tombstone, as a refused setup
       part's route (`lifecycle.rs`) or an unknown tool, there is no authorizing target, and the
       tier is 1;
     - **every named object.** For each value of the route's `arguments` `object`, `instance`,
       `conduit` and `node` that 046a's decision 2 puts in `named.objects`, resolved as the tier-2
       `exists` check resolves it now, the sending instance may inspect the object it resolves to.
       Where it resolves to none, the condition is an open Conduit from the sending instance to
       the first-start `management` instance, whose target is not retired, that lists the core's
       `inspect.v1` and that holds no filter: the same test as the new function below applies to
       its route, with the route `management` itself. This is not the new function applied to the
       `management` Resource, whose route is its owning Pool `system` in every state a first start
       makes.
   - "May inspect X" is a new `pub(crate) fn` in `access.rs`, beside `may_inspect` and taking the
     same arguments: it chooses the route that `may_inspect` chooses (X's owning Pool or Fleet where
     X is a Resource or Agent with no `spec` and a live owner, `management` otherwise) and returns
     true only where the actor is the consumer of an open Conduit (`access::open`) to that route,
     whose target is not retired, that lists the core's `inspect.v1` and whose `requirements` hold
     no filter: no requirement outside the core package, the test `access::evaluate_path` applies
     when it collects a Conduit's filters (ACC-056: "through open Conduits with no filter"). Move
     the shared route choice into one place that both functions call; `may_inspect` keeps every
     result it gives today.
   - **Records follow the diagnostic.** Each of the five sites that sets `refusalTier` passes
     `tier` the route that the same refusal gives `diagnostic`, building it as that site builds the
     diagnostic's route where it sets the record first (`lib.rs`'s `end_request` among them, which
     takes the route or the tier from its caller). A record's `refusalTier` and the diagnostic's
     `tier` are then equal for every refusal, and neither reads the other (046a's decision 5). No
     record holds a tier-2 member.
   - `diagnostic` takes its tier from the same function. A refused tools/list stays at tier 1.
   - **Errors.** `routing` and the inspection functions return `Result`, so `tier` returns
     `Result<u8>`, and each site that calls it propagates the error with `?`, as `diagnostic`
     already does. No error becomes tier 1.
   - **Registration's ACC-054 refusal.** Its route, which `registration.rs` builds as
     `ids(request, {})`, gains `documents` as the ACC-001, ACC-033 and ACC-057 sites' routes hold
     them (`{"documents": project.documents}`), so that its tier condition reads the live owners of
     rows 1 and 1A. `named.objects` stays empty, and no tier-1 field or message changes by it; if
     one does, stop and report it.

2. **What tier 2 holds: the target and the Conduits, closed ones included** (ACC-056's observable,
   LIF-040). In `diagnostic`'s tier-2 block, for every sending instance that gets tier 2, root
   included:
   - **The Conduit the route resolves.** As now, the live Conduit of the route (`conduit`), found
     by `access_route` or given by an input that names a request by IDs. Where there is none, and
     the route has a `consumer` and a `tool` whose text before its first dot is L, the tombstone of
     `state.tombstones` whose `object.kind` is `Conduit`, whose `consumer.id` is the route's
     `consumer` and whose `localName` is L; for an input that names a request by IDs whose
     `request.conduit` names no live Conduit, the tombstone whose `object.id` is that ID and whose
     `consumer.id` is the route's `consumer`. Where several match, take the one with the latest
     `time`, then the smallest tombstone ID. For a caller other than root, a tombstone counts only
     where `inventory::visible` gives true for the sending instance. The authorizing target of
     decision 1 is read through this tombstone where it is the one found: call `routing` with a
     Conduit value that holds only `target`, the kept target's `id` string alone, so that `routing`
     reads it as it reads a live Conduit's and a later object that reuses the name never resolves
     in its place (LIF-040). For that `routing` call alone, the operation is the tombstone's listed
     operation whose name after `#` is the tool's text after its first dot, where it lists one, and
     the spelling otherwise; tier 1's `operation` keeps the spelling, as "Nothing else moves"
     says. Where no listed operation matches and the core holds an Operation of that name, use
     `c.qualified(name)`, for that `routing` call alone, as `access_route` resolves a live
     Conduit's core operation.
   - **`target`.** From the live Conduit's target, as now; from a tombstone, the kept target's
     `kind`, `name` and `id`. Omitted where neither gives one, as now.
   - **`exists`.** Unchanged: false where the route resolves no live Conduit, or where a named
     object does not exist; true otherwise. A closed Conduit is no Conduit, so a request through
     one has `exists` false and a `target`: the named Conduit no longer exists, and tier 2 says what
     it was.
   - **`conduits`.** First, as now, the live Conduits of the sending instance to `target`, in the
     order of `state.objects`, with `state` `open` or `registered`. Then one entry for each
     tombstone whose `object.kind` is `Conduit`, whose `consumer.id` is the sending instance and
     whose `target.id` is `target`'s ID, and, for a caller other than root, for which
     `inventory::visible` gives true for the sending instance: `{"name": object.name, "as":
     localName, "state": "closed", "operations": operations}`, ordered by `time`, then by
     tombstone ID. An empty list where there is no `target`, as now.
   - **Nothing else moves.** `operation`, `named` (`conduit` included, which stays omitted where no
     live Conduit has that local name), `cause` and the message are 046a's and do not change: a
     tombstone never reconstructs a tier-1 field. `access_route` and `acc033_target` do not change.

3. **One portable case**, `explain-tier-two-inspectors`, in
   `conformance/conventions/explain/tier-two-inspectors/case.yaml`, written as order 046a's
   decision 11 writes its cases (JSON with two-space indentation and a final newline; members `id`,
   `layers`, `promises`, `title`, `profile`, `initial`, `files` and `steps`, in that order;
   `layers` `["convention", "interface"]`; `profile` `first-build`; `initial`
   `{"recipe": "empty"}`), with promises `["ACC-056", "ACC-001"]` and title
   `A caller that may inspect the target gets tier 2, closed Conduits included`. Its `files` are
   `{"fixture": "../../inventory/e4/fixture.json", "manifest": "../../inventory/e4/manifest.json",
   "documents": "../../inventory/e4/documents.json"}`. It follows 046a's portable-case rule, and no
   order adds it to `conformance/internal-cases.yaml`. Each step's members are `input`, `expect`
   and, where given, `capture`, in that order; each `expect` holds its members in the order
   written below.

   An `invoke` step here has `evidence` `{"time": T, "ids": IDS}`, where IDS is the 30 strings
   `00000000-0000-4000-8000-` followed by the twelve-digit, zero-padded decimal of
   46000 + 100n + k, for k = 0 to 29, n being the step's number; and the request
   `{"identity": {"kind": "instance", "id": WHO}, "tool": TOOL, "arguments": ARGS}`. ROOT is
   `00000000-0000-4000-8000-000000000005`. P is `/reply/error/data/diagnostics/0/properties/refusal/`.
   1. `e4-inventory-from-first-start`'s step 1 `input`, whole. `expect`: `{"reply": {"started":
      true}, "external_intents": [], "required_promises": ["FST-001"], "state": {"started": true},
      "record_facts": []}`.
   2. `e4-inventory-scope`'s step 1 `input` and `expect`, whole: root registers the e4 fixture.
   3. T `2026-11-11T13:00:00Z`; WHO ROOT; TOOL `management.inspect.v1`; ARGS
      `{"object": {"kind": "Interface", "name": "viewer"}}`. `expect` `{"reply": {"isError":
      false}}`; `capture` `{"viewer": "/reply/structuredContent/id"}`.
   4. T `2026-11-11T13:00:01Z`; WHO ROOT; TOOL `access.open.v1`; ARGS
      `{"conduit": "viewer-shared"}`. `expect` `{"reply": {"isError": false}}`.
   5. T `2026-11-11T13:00:02Z`; WHO ROOT; TOOL `access.close.v1`; ARGS
      `{"conduit": "viewer-shared"}`. `expect` `{"reply": {"isError": false}}`. LIF-040 makes the
      Conduit's tombstone.
   6. T `2026-11-11T13:00:03Z`; WHO `$viewer`; TOOL `viewer-shared.ping.v1`; ARGS `{}`. `viewer`
      holds no Conduit through which it may inspect `shared`, so tier 1. `expect`:
      `external_intents` `[]`; `record_facts` `[{"type": "request-ended", "data": {"reason":
      "refused", "ruleId": "ACC-001", "refusalTier": 1}}]`; `reply` `{"error": {"code": -32602,
      "data": {META/reason: "refused", "diagnostics": [{"ruleId": "ACC-001", "level": "error",
      "properties": {"refusal": {"tier": 1, "operation": "viewer-shared.ping.v1", "named":
      {"tool": "viewer-shared.ping.v1"}}}}]}}}`; `absent` the three pointers P followed by `target`,
      `exists` and `conduits`.
   7. T `2026-11-11T13:00:04Z`; WHO ROOT; TOOL `access.open.v1`; ARGS
      `{"conduit": "viewer-pool"}`. `expect` `{"reply": {"isError": false}}`. `viewer` may now
      inspect `shared` through its owning Pool `store`.
   8. T `2026-11-11T13:00:05Z`; WHO `$viewer`; TOOL `viewer-shared.ping.v1`; ARGS `{}`: step 6's
      request again. `ping.v1` is not an operation ACC-033 routes, so the authorizing target is the
      request's target, `shared`, read through the tombstone, and the request names no object.
      `expect`: `external_intents` `[]`; `record_facts` as step 6's with `refusalTier` 2; `reply`
      `{"error": {"code": -32602, "data": {META/reason: "refused", "diagnostics": [{"ruleId":
      "ACC-001", "level": "error", "properties": {"refusal": {"tier": 2, "operation":
      "viewer-shared.ping.v1", "named": {"tool": "viewer-shared.ping.v1"}, "target": {"kind":
      "Resource", "name": "shared"}, "exists": false, "conduits": [{"name": "viewer-shared", "as":
      "viewer-shared", "state": "closed"}]}}}]}}}`.
   9. T `2026-11-11T13:00:06Z`; WHO `$viewer`; TOOL `pool.retire.v1`; ARGS
      `{"instance": {"kind": "Resource", "name": "shared"}}`. `viewer-pool` does not list
      `retire.v1`, so ACC-001 refuses; ACC-033's row 4 makes `store` the authorizing target, and
      inspecting a Pool goes through `management` (row 7), on which `viewer` holds no open
      Conduit listing `inspect.v1`, so
      tier 1, though `viewer` may inspect `shared` itself. `expect`: `external_intents` `[]`;
      `record_facts` as step 6's; `reply` `{"error": {"code": -32602, "data": {META/reason:
      "refused", "diagnostics": [{"ruleId": "ACC-001", "level": "error", "properties": {"refusal":
      {"tier": 1, "operation": "CORE#retire.v1", "named": {"tool": "pool.retire.v1", "conduit":
      "pool", "objects": [{"kind": "Resource", "name": "shared"}]}}}}]}}}`; `absent` as step 6's.

   The coordinator ran this case before dispatch, on a scratch copy of `main` at `facb4ff4`: steps 1
   to 7 and 9 passed as written, and step 8 failed with
   `missing ordered record {"data":{"reason":"refused","refusalTier":2,"ruleId":"ACC-001"},"type":"request-ended"}`;
   with step 8 rewritten to tier 1 the whole case passed, so main gives step 8 tier 1 today. If any
   step but step 8 fails on your build, or step 8 fails otherwise than this order's change explains,
   stop and report: do not change an expectation to pass it, and never write one from the engine's
   output.

   Index it in `conformance/conventions/explain/index.yaml` as orders 046a and 046c indexed theirs:
   write its entry (`id`, `case` `conventions/explain/tier-two-inspectors/case.yaml`, `promises` and
   `safety_rows: []`, in that order) in a `cases` list in `conformance/manifest.yaml`, then run,
   from `engine/`, `cargo run --release -p conduit-conformance --example pieces` with no flag. It
   adds the entry to the piece and leaves `manifest.yaml` byte for byte as it was. Change no
   `s1_coverage` row.

4. **The standing table.** Once the case and the index are in place, run
   `python3 tools/promise_standing.py --write` from the repository root. It rewrites
   `engine/STANDING.md`. Never edit that file by hand. Report the rows of ACC-056 and ACC-001.

5. **Unit tests**, in the core, in `explain.rs` or beside the code they test, each building `State`
   by hand as the existing `fixture` does:
   - a Resource `x` owned by a Pool `p`, and an Interface `v` with a Conduit to `x` (local name
     `x`) and a Conduit to `p` listing `inspect.v1`, refused under ACC-001 for an operation that
     ACC-033 does not route: tier 2, its `target` `x` and its `conduits` the Conduit to `x`; the
     same with the Conduit to `p` not open, or listing no `inspect.v1`, or applying a filter: tier
     1; the same with an Agent in `v`'s place: tier 2;
   - a named object `v` may not inspect: tier 1; a named object that does not exist, where `v` may
     inspect `management`: tier 2, `exists` false; where it may not: tier 1; and the same with
     `management` given a live owner Pool, so that the two readings differ: a Conduit from `v` to
     `management` itself listing `inspect.v1` gives tier 2, and one to that owner Pool alone gives
     tier 1;
   - a request that a Pool authorizes (a `retire.v1` of `x` through a Conduit to `p` that does not
     list it): tier 1 where `v` may inspect `p`'s members but holds no `inspect.v1` on `management`,
     and tier 2 where it holds one;
   - a non-root sender, with `management` owned by a live Pool `s`, refused through
     `Input::Request` for `CORE#retire.v1` of a Resource owned by `s`, that holds `inspect.v1` on
     `s` but none on `management`: tier 1; with `inspect.v1` on `management` as well: tier 2;
   - in the state the test of a named object that does not exist builds, `management` with a live
     owner Pool `sys`: a `register.v1` with a document owned by a live Pool `p`, refused under
     ACC-033. A sender with `inspect.v1` on `sys` alone gets tier 1, and one with `inspect.v1` on
     `sys` and on `management` gets tier 2;
   - the records: for a non-root sender that gets tier 2, the access-decision and request-ended
     records of an `invoke` refusal, and the request-ended record of an input that names a request
     by IDs (`end_request`), each hold `refusalTier` 2, equal to the diagnostic's `tier`, and no
     tier-2 member; and the same check through `access::decide` (the `access` input) for a
     non-root sender that gets tier 2: its access-decision and request-ended records hold
     `refusalTier` 2;
   - closed Conduits: root, with a tombstone of its Conduit of local name `y` to `x` and no live
     Conduit `y`, refused for the tool `y.op`: `target` `x` from the tombstone, `exists` false,
     `conduits` that one entry with `state` `closed`, the tombstone's name, local name and
     operations; with a live Conduit to `x` as well, the live entry first; two tombstones to `x`,
     ordered by `time`; a tombstone of another consumer's Conduit with local name `y` is not used;
     and the tier-1 fields of that refusal to a non-root sender (`operation` the spelling, `named`
     without `conduit`) equal those it gets today;
   - a closed Conduit's core operation, in the same state, `management` with a live owner Pool
     `sys`: `v` holds open Conduits to `management` and to `p`, each listing `inspect.v1`, and none
     to `sys`. A tombstone of `v`'s Conduit to `management` with local name `m` lists
     `CORE#retire.v1`. The tool `m.retire.v1` with `instance` `x` (owner `p`): tier 2. The
     spelling reading gives tier 1;
   - visibility: `v` has a live Conduit to `x` (owner `p`) and `inspect.v1` on `p`, and nothing
     on `management`. A tombstone of another of `v`'s Conduits to `x` keeps a target owner that
     is a live Pool `q`, and `v` holds nothing on `q`, so `inventory::visible` gives false for it.
     The tombstone's entry is absent from `conduits`, and the live entry is present;
   - the new inspection function gives what `may_inspect` gives on every case the tests above
     build where no filter applies.

   An existing test whose assertion this order changes may change that assertion alone. Report
   each such test. `tiers_and_ids_conduit_follow_sender`'s non-root senders hold no `inspect.v1`,
   so its tiers stay as they are.

6. **`engine/CONFORMANCE.md`.** In the section "Refusals and reasons (order 046a)", the sentence
   "Each refusal holds one `refusal.v1` explanation: tier 1 for every sender, tier 2 for root."
   becomes "Each refusal holds one `refusal.v1` explanation: tier 1 for every sender, and tier 2 for
   root and for a sender that may inspect the authorizing target and every named object (order
   046b).", and its unfinished item "Tier 2 for callers other than root, and closed Conduits from
   tombstones: order 046b." goes. After the section "Outcomes the engine decides (order 046c)", add
   a section "Tier 2 for inspectors and closed Conduits (order 046b)": what decisions 1 and 2 built,
   in a few sentences, and that a caller other than root whose request resolves no Conduit and no
   tombstone gets tier 1, since no authorizing target is found (the coordinator's reading). Nothing
   else in the file changes.

7. **The declaration.** `docs/work-orders/046b-declared.yaml` holds one `added` shape for the case.
   The coordinator wrote it with this order; check that it matches, and change it only to make it so.

8. **Before you finish, check:**
   - every refusal whose sender is not root has tier 2 only where decision 1's condition holds,
     and its records' `refusalTier` equals its diagnostic's `tier`;
   - no tier-1 refusal holds `target`, `exists` or `conduits`, no tier-1 field reads a tombstone,
     and no message or record holds a tier-2 fact;
   - every existing case passes unchanged: if one fails because a sender now gets tier 2, or a
     tier-2 list now holds a closed Conduit, stop and report the case and its step; do not change it.

9. **Findings to report, for SO-H's sweep.** ACC-056 does not say whether tier 2's resolved target
   may be read through the tombstone of a closed Conduit that the request names by its local name,
   nor what `exists` states then (decision 2's reading); and it does not say which targets the
   condition reads where ACC-033's row names two (rows 1, 1A and 3; decision 1's reading). Report
   both, and any other gap you meet, as findings against the specification.

## Scope

Change:

- `engine/crates/core/src/explain.rs` (decisions 1, 2 and 5);
- `engine/crates/core/src/access.rs`: the new inspection function and the route choice it shares
  with `may_inspect` (decision 1), and the access-decision record's tier, and a `pub(crate)` on
  `routing` if `explain.rs` calls it, without changing what `routing` or `may_inspect` give;
- at the sites that set `refusalTier` or pass a route to `explain::tier`, only to pass the route:
  `lib.rs`, `lifecycle.rs` and `requests.rs`; `registration.rs` only where a call into `lib.rs`'s
  `end_request` needs it, and the ACC-054 refusal's route (decision 1);
- `engine/CONFORMANCE.md` (decision 6), and `engine/STANDING.md` through the tool alone;
- `conformance/conventions/explain/tier-two-inspectors/case.yaml` (new) and its entry in
  `conformance/conventions/explain/index.yaml`;
- `docs/work-orders/046b-declared.yaml`, only as decision 7 allows;
- `experiments/011-short-form-spike/`, only if its tests need it: it depends on the core.

Report any other core file you change, and why. Do not change:

- `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, or any record (`docs/proposals/`,
  `docs/AUDIT.md`, `ROADMAP.md`, `docs/implementation/`): report any defect as a finding;
- `conformance/case.schema.json`, `conformance/case-schema/`, `conformance/internal-cases.yaml`,
  `conformance/expected-failures.yaml` and any other case or fixture;
- `conformance/manifest.yaml`, which ends byte for byte as it was;
- `inventory.rs` (this order calls `visible`; it changes nothing there), `records.rs`, the shell and
  the conformance crate;
- tier 1's fields, every message, `access_route`, `acc033_target`, the order of refusing rules, and
  the outcome diagnostics of order 046c.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_docs.py
python3 tools/check_rulings.py
python3 tools/set_names.py --check
python3 tools/check_schemas.py
python3 tools/check_roadmap.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
python3 tools/check_private_paths.py
python3 -m unittest discover -s tools/tests
git diff --check
/usr/bin/python3 tools/vector_delta.py HEAD WORKTREE --declared docs/work-orders/046b-declared.yaml; echo "exit $?"
```

Run the Python checks that need PyYAML with `/usr/bin/python3` where `python3` lacks it.
`check_model_account.py`, `check_roadmap.py` and `promise_standing.py --check` must end
`Errors: 0`; the last fails until decision 4's `--write` has run. `vector_delta.py` prints
`Declarations read from docs/work-orders/046b-declared.yaml: 1 shapes.`, then
`Vector delta: 1 changes; 0 flips; 0 undeclared; 0 errors.` and `exit 0`.

Each of these prints 1. The `tr` check reads the file with its line breaks as spaces, so a rewrap
hides nothing:

```bash
grep -c '"explain-tier-two-inspectors"' conformance/conventions/explain/index.yaml
grep -c '^## Tier 2 for inspectors and closed Conduits (order 046b)' engine/CONFORMANCE.md
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'tier 2 for root and for a sender that may inspect the authorizing target and every named object (order 046b)'
```

Each of these prints 0:

```bash
grep -c 'explain-' conformance/internal-cases.yaml
tr -s ' \n' '  ' < engine/CONFORMANCE.md | grep -c 'closed Conduits from tombstones: order 046b'
```

This prints nothing: `git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/internal-cases.yaml conformance/expected-failures.yaml conformance/conventions/inventory spec IN_FORCE.md tools packages docs/implementation docs/proposals engine/crates/shell engine/crates/conformance`.

From `engine/`, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. explain-tier-two-inspectors
cargo run --release -p conduit-conformance -- .. explain-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

What each must show:

- The pieces tool prints `Nothing to move.`
- The first runner command prints `PASS explain-tier-two-inspectors (9 steps; ACC-056, ACC-001)`
  and ends `1 cases passed`; the second prints a `PASS` line for each of the five `explain-` cases
  and ends `5 cases passed`.
- The two test commands pass. Report the totals passed, failed and ignored of the full run. If a
  shell test fails, rerun it alone; rerun a timing-sensitive one with `-- --test-threads=2`. The
  threads are 4 because at 24 the runner ran out of memory (order 071's Record).

Then show that the case can fail. From `engine/`, one at a time:

1. `cp crates/core/src/explain.rs target/explain.rs.046b`
2. Make `tier` give 1 to every sending instance that is not root, as before this order. Run
   `cargo run --release -p conduit-conformance -- .. explain-tier-two-inspectors`. It must fail,
   printing a line that holds `missing ordered record` and `"refusalTier":2`.
3. `cp target/explain.rs.046b crates/core/src/explain.rs`
4. Make tier 2 list no Conduit from a tombstone, leaving decision 2's target as it is. Run the same
   command. It must fail, printing a line that holds `conduits`.
5. `cp target/explain.rs.046b crates/core/src/explain.rs`, and run the same command a last time.
   It must pass.

Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild. Then `cmp crates/core/src/explain.rs target/explain.rs.046b` exits 0. Git ignores
`target/`. If decision 2's tombstone lookup does not live in `explain.rs`, make step 4's change
where it lives, with its own copy and `cmp`, and say so.

In `experiments/011-short-form-spike/`: `cargo test --release`. Its `tests/spike.rs` run reports
`test result: ok. 9 passed; 0 failed`.

Do not stage anything. In your report:

- the case added, by path, and the count `vector_delta.py` gave;
- the signatures of `tier` and of the new inspection function, and each site whose call you
  changed;
- each test of decision 5 by name, and each existing test whose assertion you changed;
- the rows of ACC-056 and ACC-001 in `engine/STANDING.md` after decision 4;
- the line each run that had to fail printed;
- every choice you made yourself, marked as yours;
- decision 9's findings, and every other finding against the specification;
- the last line of every check.

`git diff --check` and `check_private_paths.py` read tracked files only. The admin then stages the
new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. Before
the merge the admin runs `python3 tools/gate.py`, and merges on its PASS
([ruling 80](../proposals/ruling-80/README.md)); GitHub's CI runs on the pull request too, and its
`docs` job reads `046b-declared.yaml` through `--declared-changed`, and the merge does not wait for
it. A fresh Codex verifier and a fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2). The admin writes this order's Record at
the foot of this file when it merges.

## Record

Run on 2026-11-18, from about 09:15 to 10:19 UTC, on the Mac from `main` `f72c2bc3` (the merge of
orders 046b and 047c, #354), in two rounds: the worker (about 1761 s) and a fresh Codex verifier
(about 525 s), then the worker again (about 1125 s) and a second fresh Codex verifier (about 434 s).
The change is the files of Scope: `explain.rs`, `access.rs`, `lib.rs`, `lifecycle.rs`,
`registration.rs` and `requests.rs` in the core, `engine/CONFORMANCE.md`, `engine/STANDING.md`
through the tool, and the new case with its index entry; no other core file, and
`experiments/011-short-form-spike/` needed no change. `tier(state: &State, route: &Value, c:
&Context) -> Result<u8>` applies decision 1's condition, and `access::may_inspect_unfiltered`, with
the same arguments as `may_inspect`, shares its route choice. `access::may_inspect_missing(state,
actor, context) -> Result<bool>` applies the same unfiltered test to `management` itself for a named
object that does not exist, and `PublicationView::fixed` became `pub(crate)` so that `tier` finds a
registration owner as `evaluate_path` does. The route reaches `tier` from `diagnostic` and from the
five record sites: `access::decide`, `lib.rs`'s `end_request` (from Submit, both branches of an
input that names a request by IDs, and registration), the refused setup part in `lifecycle.rs`, and
`requests.rs`'s `invoke_inner` and `reject_diagnostics`. Registration's ACC-054 routes hold the
submitted documents. Decision 2's tier-2 block lists live Conduits first, then visible closed ones
by time and tombstone ID. The rows of `engine/STANDING.md` are `| ACC-056 | convention | must | 4 |
0 | outside | core | case passes |` and `| ACC-001 | convention | must | 8 | 0 | cases | core | case
passes |`. The worker's own choices: an ID-only target for the Conduit read through a tombstone;
tombstone times sorted as instants with the existing `chrono`, an unparseable time by its text
before the valid ones; one resolver of named objects shared by the tier and `exists`; one route,
`rejection_route` in `requests.rs`, for the access record's tier and the diagnostic where the route
has no operation; and non-root refusals of `sign-in.v1`, `answer.v1`, `enroll-machine.v1`,
`admit-machine.v1` and `refuse-machine.v1` kept at tier 1, since `access::routing` does not yet
route them as ACC-033 does. The Rust diff, 848 insertions and 140 deletions in six files after the
fixes, most of them tests in `explain.rs`, exceeds the order's estimate of 0.2 to 0.35k lines; the
second verifier noted it as a should, and nothing was trimmed.

The first verifier found four musts and three shoulds. Musts: the refused setup part's route took
`v["caller"]` as `consumer`; an ID-based refusal could use another instance's live Conduit to reach
tier 2 and disclose its target; operations ACC-033 routes that `routing` does not yet implement took
the Conduit's target as the authorizing target; and `046b-declared.yaml`, unchanged in bytes from
the base, was not read by CI's `--declared-changed`. Shoulds: the ordering test passed without the
sort, the parity of the two inspection functions was not asserted in every unfiltered fixture, and
the worker report carried a private path. Round 2 used `v["sender"]` in the setup part's route, with
the test `setup_part_without_request_uses_the_sender_for_disclosure`; required the Conduit of tier
eligibility and of tier 2's details to be the sender's (`sender_live_path`), with
`id_refusal_cannot_use_a_foreign_live_conduit_for_disclosure`; kept the five operations above at
tier 1, with `unimplemented_authorizing_routes_keep_non_root_refusals_at_tier_one`; added
`operation: add` to the declaration; reversed the ordering fixture's IDs against their times; added
the parity assertions; and redacted the report. The second verifier found two musts and two shoulds:
an owner given as a Pool ID to `Input::Submit` skipped the owner check; a `replace.v1` names its
template outside the four keys of `named.objects`; the registration test built its route by hand;
and the diff's size.

A fresh read-only Opus reader then verified the order (ruling 79, point 2) on a scratch clone at
`d2484c90`, soft-reset to the base. Every check of "Done when" passed as worded: the case with 9
steps, the five `explain-` cases, the full release run (332 passed, 0 failed, 4 ignored), both
mutations failing as required and the restored file passing with `cmp` exit 0, the spike's 9 tests,
and `vector_delta.py` with main's declared text (`1 changes; 0 flips; 0 undeclared; 0 errors`, exit
0). It re-rated the second verifier's two musts as shoulds, since both follow the order's wording,
and found one must and five shoulds:

- r1 (must): round 2's filter of the route's Conduit ran for root too, so where root's request named
  another instance's live Conduit by ID, tier 2 omitted `target` and said `exists` false, against
  decision 2's "as now".
- r2: the declaration gained `operation: add` only so that CI's `--declared-changed` would see a
  diff; main's text already matches, and the coordinator's check line at the merge covers CI.
- r3: a refused `replace.v1` names its template through `arguments.template`, outside the four keys
  decision 1 fixes, so a sender may get tier 2 without inspecting the template (ACC-056: "every
  object the request names").
- r4: `tier` resolved a registration owner by name alone, where Submit's ACC-033 check finds a Pool
  by name or ID, so an owner given by ID skipped its inspection check.
- r5: round 2's change of the setup part's `consumer` from `v["caller"]` to `v["sender"]` changes an
  observable and was reported only inside a decision note.
- r6: the registration test built its route by hand, and the named-object test never removed
  `management`'s owner Pool.

The fixer found the branch already on `origin/main` at `f72c2bc3`, so the rebase with the work-order
identity changed nothing, and applied each finding:

- r1: in `diagnostic`'s tier-2 block the filter is
  `is_root(state, sender, c) || sender_live_path(path, sender)`, so root's `target` and `exists`
  come from the route's live Conduit, as in order 046a; `conduits` stays the sender's own.
  `id_refusal_cannot_use_a_foreign_live_conduit_for_disclosure` now expects root's `target`
  `{"kind":"Resource","name":"x","id":"x"}`, `exists` true and `conduits` `[]`.
- r2: `046b-declared.yaml` is main's text again, unchanged in the diff.
- r3: no code change, the coordinator's choice left open: a finding against ACC-056 for SO-H's
  sweep, below.
- r4: `tier`'s register.v1 loop finds the owner by name, then by ID (`access::object_by_id`), which
  can only lower a tier. The new test `registration_owner_given_by_id_needs_inspection_of_that_pool`
  gives the Pool `p` the name `pool-p` and drives Submit with owner `p`: ACC-033 decides, and the
  diagnostic and both records hold tier 1.
- r5: no code change. Round 2's `consumer` `v["sender"]` in the refused setup part's route
  (`lifecycle.rs`) is a correction of order 046a's route, made under the reading that ACC-056's
  caller is the sending instance; a setup sent by a non-root instance with root as caller now gets
  tier 1, not 2. `setup_part_without_request_uses_the_sender_for_disclosure` covers it. As the
  coordinator's acceptance, open to the owner's strike, the setup part's consumer is read as the
  sending instance, and the change stands.
- r6: `tier_two_checks_management_and_the_live_registration_owner` drives `Input::Submit` through
  `crate::step` with a Conduit `v-register` to `management` listing `register.v1` and none to `p`,
  and asserts that ACC-033 decides and that the diagnostic and both records go from tier 1 to tier 2
  once `v` holds `inspect.v1` on `management`.
  `tier_two_checks_named_objects_and_uses_management_itself_for_missing_objects` gains a variant
  without `management`'s owner: a missing object gives tier 1 with no inspection, and tier 2 with
  `exists` false with `inspect.v1` on `management`.

Findings against the specification, for SO-H's sweep (decision 9 and others): ACC-056 does not say
whether tier 2's target may be read through the tombstone of a closed Conduit named by its local
name, nor what `exists` states then (the build gives the kept target and `exists` false); it names
one authorizing target where ACC-033's rows 1, 1A and 3 name two (the build requires inspection of
each); it says caller where the definitions distinguish the caller from the sending instance of an
onward request (the build uses the sending instance); the tier-1 echo of an ID-based refusal can
derive a tool spelling and local name from a Conduit ID that is not the sender's, though
`refusal.v1` describes `named` as an echo of the request (kept, as this order requires); and
ACC-056's "every object the request names" reaches a `replace.v1`'s template, which `named.objects`
and the tier do not read (r3).

With each fix taken out on the fixer's tree, its test failed: r1's filter without root, and r4's
lookup by ID. The fixer then ran, on the files of this commit before it was made:
`cargo fmt --check` and `cargo clippy --all-targets -- -D warnings` (exit 0), the pieces example
(`Nothing to move.`, `manifest.yaml` unchanged), the case
(`PASS explain-tier-two-inspectors (9 steps; ACC-056, ACC-001)`, `1 cases passed`), the `explain-`
cases (five PASS lines, `5 cases passed`), the vectors test (2 passed) and the full release run (333
passed, 0 failed, 4 ignored), each with `CONDUIT_VECTOR_THREADS=4` and `RUST_TEST_THREADS=4` where
cargo tests; both mutations again (a line holding `missing ordered record` and `"refusalTier":2`,
and step 8's `conduits: expected 1 items, found 0`), the restored file passing with `cmp` exit 0;
the spike (`test result: ok. 9 passed; 0 failed`); the order's eight Python checks (Errors: 0 each)
and the tool tests (OK); `vector_delta.py` from `origin/main` to the worktree with the declaration
(`1 shapes`, `1 changes; 0 flips; 0 undeclared; 0 errors`); the order's greps (1, 1, 1, 0, 0) and
its `git status` on the forbidden paths (nothing); and `git diff --check` and
`git diff --check origin/main...HEAD` (clean). The gate's run on this order's squash, which this
merge waits on, is the coordinator's at the merge.

At the merge the coordinator applied the last reader's should, if any remained, and added a check
line to 046b-declared.yaml, which reached main with this order's text, so that the gate's vector
check reads it. The gate runs on the squash, and the merge waits on its PASS.
