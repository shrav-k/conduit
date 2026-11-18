# Work order 046a (E5a): one deciding rule, and a diagnostic on every reason

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the build orders share. List in `vectors` every case you add or change, with the promises
it tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it. Use it for nothing else, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, fixture or log: use
`head`, `tail`, `grep`, `sed -n`, `wc` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

**Where it sits.** This is the head of batch 1's core chain (046a, 046c, 047, 048a-2, 048b-join,
050, 053). Batch 1's plan has one order 046, "refusals and explicitness"
([the plan](../proposals/spec-batch-1/README.md), its entry 046 under "The work orders").
[Order 117](117-plan-s2-s3.md) (lines 81 to 83) split it in two, and the coordinator splits 046a
once more (see "Who decided what"):

- **046a, this order:** the refusal tiers, a diagnostic on every reason, the runner's assertions
  and the CLI's tier 2;
- **[046c](046c-build-outcome-diagnostics.md), after this order and before 047:** the outcomes
  the engine decides name their promise, which was this order's decision 6;
- **046b, after 042 and this order:** closed Conduits from tombstones, in tier 2.

The full review's queue ([orders.md](../proposals/full-review/orders.md), line 100) adds four
things to 046a. This order keeps three of them whole and the half of FR-547 that goes with them:

- the vector rule ([FR-350](../proposals/full-review/findings/P07-engine-build.md#fr-350); D36 of
  [the decisions](../proposals/full-review/decisions.md#the-pipeline-and-verification)): from this
  order on, a new case sends only what a client or a provider can send, and asserts only what a
  client can see, with a runner check that rides in this order;
- IFC-050 for every reason: a reason carries an error diagnostic with its deciding rule, and a
  refusal names one rule with ACC-056's tiers. IFC-050's other, unbuilt half
  ([FR-353](../proposals/full-review/findings/P07-engine-build.md#fr-353); D37), an outcome the
  engine decides as failed or unknown carrying a diagnostic naming its cause, goes whole to
  order 046c;
- [FR-547](../proposals/full-review/findings/GL2-vectors.md#fr-547) (F59): the cases' negative
  assertions on the old error form beside a reason, and the runner's allow-list, which fail an
  engine that keeps IFC-025, IFC-050 and ACC-056. Those beside an outcome the engine decides go
  with order 046c, which gives that outcome its diagnostic;
- [FR-554](../proposals/full-review/findings/GL2-vectors.md#fr-554): `list.v1`'s results compared
  as sets, since OPS-036 states no order.

**What SO-A left to it.** [Order 171a](171a-so-a-form-pools-refusals.md) (SO-A, part 1) wrote
ACC-056's order of refusing rules, its tiers and `refusal.v1`'s non-empty `cause` into `spec/`.
Its case `access-close-root-outside-bound` is an expected failure "until order 046a builds that
order". Its Record notes that the build's order differs from ACC-056's in more places than the
bound against ACC-024: ACC-033's routing comes before ACC-032 and ACC-039, and ACC-054 before the
owner check of a registration under ACC-033. Its section "Cases" says that F2's vectors and F3's
other vectors come with 046a. F2's build and vectors stay out of 046a all the same: they go to a
separate order of the admin's, not yet numbered, because F2 needs ruling 80's exemption of a
bounded holder's own repeat ([ruling 80](../proposals/ruling-80/README.md), point 1), which
touches the repeat index in `requests.rs`. F3's other vectors go with them ("What it leaves",
below). [Order 171b](171b-sign-in-return-and-holds.md) (SO-A, part 2) and
SO-B ([172a](172a-so-b-attempts-and-resends.md), [172b](172b-so-b-reconcile-records-stop-ends.md))
are merged. Their Records leave nothing to 046a: SO-B's expected failures wait for the full
review's order 22, and 171b's for 048a-1 and 048a-2. The targeted review of 0.2.0's sign-in
surface ([its record](../proposals/review-signin-2026-11-16/README.md), section 6) has no
must-state list for 046a, and [ruling 82](../proposals/ruling-82/README.md) changes nothing here.

**What it builds.** One behavior, IFC-050 for reasons and ACC-056 for refusals: every result
that ends with a reason says which promise decided it and why, and a refusal names one rule, the
first in ACC-056's order, with its tier-1 explanation for every caller and its tier-2 explanation
for root. The runner then checks this on every step, the old negative assertions beside a reason
go, and new cases follow the vector rule. An outcome that the engine decides as failed or unknown
is not this order's: it keeps the diagnostics it has today until order 046c (decision 6).

**What it leaves, and to whom.** Each of these is a behavior of its own (FR-264: an order holds
one behavior and stays small):

- **Tier 2 for callers other than root** (ACC-056: "Another caller gets it only where it may
  inspect the authorizing target under ACC-033 and every object the request names, through open
  Conduits with no filter"), and **closed Conduits from tombstones** in tier 2: order 046b. Order
  117 gives 046a the refusal tiers and 046b only closed Conduits from tombstones, and batch 1's
  plan lists the tier-2 list in 046; this order moves the tier-2 condition for callers other
  than root to 046b (the coordinator's reading, for FR-264, open to the owner's strike). Ruling
  52 (`IN_FORCE.md`) shows such a caller tier 2, so until 046b merges that ruled behavior is
  unbuilt. Here, every caller but root gets tier 1. ACC-056 says "only where", so tier 1 to such
  a caller discloses less than the promise allows and nothing it forbids. `engine/CONFORMANCE.md`
  says so.
- **The outcomes the engine decides**, which were this order's decision 6: order
  [046c](046c-build-outcome-diagnostics.md), "the outcomes the engine decides name their
  promise", after this order and before 047. It carries that decision whole, with its vectors,
  its done-whens, the rule each site names, the steps of the FR-547 edit that stand beside such
  an outcome, and the steps of `explain-agent-results` that asserted an outcome's diagnostic.
  Decision 6 below says what this order does in the meantime.
- **F2's build**: a bounded holder that names a Conduit that does not exist is refused under
  ACC-029, except a repeat of its own request answered from its stored outcome
  ([ruling 80](../proposals/ruling-80/README.md), point 1). The engine still answers invalid under
  OPS-029. That is a change to which reason a request gets, with ruling 80's exemption, which
  touches the repeat index in `requests.rs`. Because it needs that exemption, F2's build and its
  vectors stay out of 046a, in a separate order of the admin's, not yet numbered: the queue's
  row 19 and order 171a's "Cases" give F2's vectors to 046a, and this order moves them, with
  F2's build, to that order (the coordinator's decision, for FR-264). The admin numbers it and
  records it in the queue ([orders.md](../proposals/full-review/orders.md)) when this order
  merges. This order changes nothing there.
- **F3's other vectors at the public boundary.** Order 171a's "Cases" and the queue's row 19
  give them to 046a. The reorderings of decision 1 other than the bound before ACC-024 need a
  bounded holder, a retired sender, an Agent sender or a non-root registrant. Some of these can
  be reached by requests: a non-root registrant and an Agent sender through `register-project`,
  as `explain-agent-results` reaches Agents. But each reordering needs a case of its own built on
  a state the size of `s1-whole`'s, and F2's vectors need such a state too. So this order moves
  F3's other vectors, with F2's, to the same order of the admin's, not yet numbered (the
  coordinator's choice, for FR-264, open to the owner's strike), and shows the reorderings here
  by core unit tests (decision 13). `access-close-root-outside-bound` shows the bound before
  ACC-024. Order 117's line for 046a and 046b records these moves, with tier 2 for callers other
  than root to 046b and the outcomes the engine decides to 046c, and the admin records them in
  the queue when this order merges.
- **The https fetch** that answers invalid citing PKG-023 (FR-353, "Cited, but the promise does
  not say it"). D37 reads: "An https fetch answers as unbuilt instead of citing PKG-023." That
  interim answer changes which answer a request gets, a behavior of its own, and goes to order
  046d, which the admin records in the queue when this order merges. PSD3 builds https itself,
  as FR-353's "Waits" says. Until 046d merges, the reply keeps its PKG-023 diagnostic unchanged
  (decision 3). The stubs of `answer.v1` and `sign-in.v1`,
  which answer unavailable citing PND-009 and PND-010, go to 047 and 048a-2. Their answers gain
  the diagnostic of decision 3 here, and keep their rule and their text (decision 3).
- **The -32602 form of a call from a client with no established instance.** IFC-032's
  observable answers each call of such a client "as IFC-012 states for a tool that is not
  presented", and IFC-025's agrees: the protocol's error, the reason in its `data`, no tool
  result. The engine answers it as a tool result with META/reason, and `records-expected-id`
  step 10, an IFC-022 refusal, pins that form. Changing the wire form changes that case, so it
  is a behavior of its own and goes to order 046e, which the admin records in the queue when
  this order merges (the coordinator's choice, for FR-264). Here such a refusal gains decision
  2's tier-1 diagnostic in its tool result.
- **Diagnostic messages matched by `message_contains`** instead of equality
  ([FR-552](../proposals/full-review/findings/GL2-vectors.md#fr-552), F60): the review's order 56.

**Who decided what.** The exemption of a bounded holder's own repeat is the owner's (ruling 80,
point 1). The rest is the coordinator's, under rulings 37, 46 point 4 and 52, each open to the
owner's strike ([ruling 48](../proposals/ruling-48/README.md)):

- splitting the outcomes the engine decides (FR-353's half of IFC-050) out of this order into
  order 046c, which comes after it and before 047 (FR-264: an order holds one behavior);
- keeping F2's build and vectors, and F3's other vectors with them, out of this order, in a
  separate order of the admin's not yet numbered, because F2 needs ruling 80's repeat
  exemption (the coordinator confirms that F3's other vectors go with F2's, open to the owner's
  strike);
- two readings: tier 2 for root alone here, the condition for other callers left to 046b; and
  D36's exception for the steps that build a case's state (decision 10), which lets a portable
  case keep the `state`, `stateUnchanged` and `required_promises` that the case schema requires
  of the input kinds that build state;
- reading ACC-056's "caller" as the sending instance for the tier and for tier 2's Conduits
  (decision 2);
- IFC-036 as the rule of the shell's own errors (decision 4): IFC-036's `then` has the engine
  answer such a request with the protocol's error and, where it could establish the sender,
  report invalid with the reason in the error's `data`, so IFC-036 is the rule that decides it;
- moving the https fetch to 046d and the -32602 form of a call with no established instance to
  046e, and the other decisions below, the wording of each cause among them.

Apply them as written, and report them under `decisions` as the coordinator's.

**Orders in flight.** Build on `main` as it is when you start. Order 046c, written beside this
one, starts only once this order is merged, and comes before 047. Order 048a-1, written beside
this one, comes after it: it starts only once this order's work is merged, and it carries CTR-013's
checks on a Type's sign-in and the signIn fixture Type, not IFC-016's detection, which order
048a-2 takes. Order 050 is written beside both. Whichever of this order and order 050 lands
second keeps both changes and writes `engine/STANDING.md` again with the tool. This order does
not depend on S0b.

**Read first:**

1. In `spec/`: ACC-056 (`spec/conventions/access.yaml`, with its table of causes), IFC-050,
   IFC-025, IFC-012 and IFC-036 (`spec/interface/implementation.yaml`), OPS-005 and OPS-023
   (`spec/conventions/operations.yaml`), `refusal.v1`, `diagnostic.v1` and `record.v1`
   (`spec/core/results.yaml`), and `list.v1` (`spec/core/operations.yaml`).
2. The findings above with their verifiers' corrections, which govern, and D36, D37 and F59.
3. Under `engine/crates/`:
   - `core/src/access.rs`: `established` (line 290), `list` (line 346), `routing` (line 506),
     `Verdict` (line 429), `evaluate` (line 600), `evaluate_path` (line 697) and `decide`
     (line 1056);
   - `core/src/schemas.rs`: `publication_omission` (line 958) and `publish` (line 987), to read:
     this order changes nothing there;
   - `core/src/requests.rs`: `envelope` (line 112), `reject` (line 557, with its -32602 branch at
     line 584), `finish` (line 1445), to read, and `complete` (line 1507, its implementation's
     reason under IFC-026 at lines 1620 to 1647);
   - `core/src/lifecycle.rs`: a setup's parts (lines 575 to 700);
   - `core/src/cascade.rs`: lines 274 to 279, which write the cascade's LIF-021 diagnostics after
     `requests::finish`, to read (decision 4);
   - `core/src/registration.rs`: `register` (line 1538), `refuse_root_registration` (line 1605)
     and `prepare` (line 1625);
   - `core/src/lib.rs`: the inputs `Submit` (line 551) and `Request` (line 644), and
     `authorized_path` (line 876);
   - `core/src/archives.rs`: the three replies with an empty `diagnostics` (lines 143, 161
     and 174);
   - `shell/src/server.rs`: the errors it makes itself with a reason (lines 442 to 445, 632 and
     695 to 706);
   - `conformance/src/`: `lib.rs` (`run_case`, line 534, and how it reads the expected
     failures, line 284), `checks.rs`, `expect.rs`, `expected.rs` and `access.rs` (lines 52
     to 63).
4. In `conformance/README.md`: "Assertions" (lines 74 to 87), the access fixture's paragraph
   (lines 476 to 483), "Expected failures" and "Reviewing vector changes".
5. The cases `s1-whole` (`conformance/conventions/s1/whole/`), `list-from-first-start`
   (`conformance/conventions/list/from-first-start/`), `archives-unavailable`
   (`conformance/language/archives/unavailable/`) and `access-close-root-outside-bound`.

Line numbers are those of `main` at `9af1c29f`. If a line has moved, find it by its words.

Two shorthands below. `CORE#name` is a core operation written as the cases write one, with the
core prefix that `conformance/README.md` defines (line 62). `META/reason` is the `_meta` key
`reason` under the prefix that `spec/names.yaml` sets, written as every case writes it.

**Size: L**, as order 117 sizes 046a: about 0.7 to 1.0k changed lines of Rust with tests, now
that decision 6's part is order 046c's, three new cases, one mechanical edit of 34 cases, and one
listed hand edit of three steps of one of them.

## What to do

1. **ACC-056's order of refusing rules** (F3; ACC-056's `then`). Reorder the refusing checks of
   `access::evaluate` and `access::evaluate_path` so that, for one request, they run in this
   order. Move each check whole, with its condition and rule unchanged, except where a step below
   says otherwise. The checks that report invalid, conflict or unavailable keep their places
   after every refusing check, as OPS-023 orders them.
   1. Unchanged at the head: the unreadable request (invalid, OPS-023), ACC-037, `established`
      (IFC-032, IFC-022), the serving chain (IFC-028, ACC-019, ACC-034, ACC-039) and IFC-021.
   2. **Moved:** the check of a retired sender that does not serve its own retirement (ACC-039,
      now at line 731 in `evaluate_path`) moves into `evaluate`, right after IFC-021 and before
      the tool is selected. It reads only `instance`, `request.serving` and `state.invocations`.
   3. ACC-001, where no Conduit gives the tool. Then, **moved from `evaluate_path`**, ACC-001 where
      the contract does not resolve (`SCH-015: unresolved`). Then, **moved earlier**, ACC-001
      where the Conduit gives no tool: call `schemas::publish` at this point, and refuse under
      ACC-001 only where it answers from its own first branch, the Conduit not open, not ready or
      not listing the operation (`schemas.rs`, about line 1000, the decision citing ACC-001). Do
      not refuse under ACC-001 for an omission that `publication_omission` gives: the one under
      SEC-007 (every `get-secret.v1` request, `schemas.rs`, line 963) is decided by SEC-007 at
      step 4, and the one under NAM-023 is the unresolved contract already refused above. The
      invalid answer under TPL-013 stays where it is now, after every refusing check.
   4. The rules on the caller and the operation alone, in this order: ACC-032 on an operation
      reserved to root (**moved** before ACC-033's routing); ACC-057 (**moved** before ACC-033's
      routing); SEC-007 (**moved** from `evaluate` to here, after the checks of ACC-001 above).
      SEC-007 decides a direct request for a Conduit's `get-secret.v1` tool wherever the Conduit
      is open, ready and lists the operation (ACC-056, step (3); SEC-007's row of its table).
   5. ACC-033: the routing check, then registration's checks of the owner's Pool or Fleet, then
      `register-replacement.v1`'s check of the Pool, as now.
   6. The bound: for `open.v1` and `close.v1` naming a Conduit, ACC-029 and ACC-030 (**moved**
      before ACC-024, ACC-054, SEC-008 and ACC-032's clause on a Conduit to be opened); then
      ACC-053 for `lift.v1` and ACC-055 for `reconcile.v1`, as now.
   7. The rules that read the named object: registration's ACC-054 on a document whose consumer
      is root (**moved** after ACC-033's checks of step 5); LIF-034, both checks; and, for
      `open.v1` and `close.v1`, ACC-024, then ACC-054, SEC-008 and ACC-032 on the Conduit to be
      opened, as now among themselves.

   In `registration::register`, split the check `!permitted || !pool_authority` (line 1582) in
   two. Where `permitted` is false, refuse under ACC-001. Where it is true, first refuse under
   ACC-057 where the sender is not root on its own behalf and a document is an Agent made from a
   template (the condition `prepare` checks at line 1640), and then under ACC-033 where
   `pool_authority` is false. Keep the ACC-057 check in `prepare`, which replacement also uses.
   In `lib.rs`'s `Submit` (line 593), split `!management_authority || !pool_authority` the same
   way: ACC-001 where `management_authority` is false, ACC-033 where only `pool_authority` is.
   No existing step expects a rule that this split changes: `register-agent-non-root` expects
   ACC-057 and `first-start-protected-without-pool` ACC-033, and both keep them.

   In `lib.rs`'s `Request` (line 644), which now calls `authorized_path` and ignores its result,
   refuse under ACC-001 where `authorized_path` gives false, before LIF-034's check, since
   ACC-056 puts ACC-001 before LIF-034. Where it gives true, LIF-034 refuses as now. The cases
   that use this input (`first-start-lifecycle-protected`, `first-start-lifecycle-pool-target`)
   send through root's own Conduits; if a step of theirs now fails because its rule changed,
   treat it as below.

   Then remove the entry of `access-close-root-outside-bound` from
   `conformance/expected-failures.yaml`, keeping every other entry in byte order. If any existing
   step now fails because its expected rule changed, change nothing in that case and nothing in
   `expected-failures.yaml` for it, and stop: report the case, its step and both rules as a
   blocking finding, with the work done so far, and do not go on to "Done when". The admin then
   decides whether the order or the case changes.

2. **The refusal's diagnostic** (ACC-056, `refusal.v1`, IFC-050). In a new module
   `engine/crates/core/src/explain.rs`, build the one error diagnostic of every refusal the core
   reports, whoever decides it. Every refusing site calls it: `access::decide`, `requests::reject`
   with reason refused (IFC-026's refusal by an implementation included), `registration`'s three
   refusals, `lib.rs`'s `Submit` and `Request`, `access::list`'s refusal of a tools/list
   (decision 3), and a setup's part refused at `lifecycle.rs`, line 661, whose rule is ACC-001.
   A setup's part whose child request `requests::invoke_part` refuses with the -32602 error of
   decision 4 (ACC-001, or TPL-013's invalid) takes that error's `error.data.diagnostics` into
   that part's `diagnostics` in the setup's parts (the invocation's `setup.parts` and the
   object's `status.setup.parts`), and its reason from `error.data`'s META/reason, where it reads
   only `structuredContent.diagnostics` and `_meta` today (`lifecycle.rs`, lines 642 to 652).
   The diagnostic is
   `{"ruleId": RULE, "level": "error", "message": {"text": MESSAGE}, "properties": {"refusal": R}}`:
   - `RULE` is the refusing rule.
   - **The sending instance and the named Conduit.** Below, the sending instance is the instance
     that `established` gives. The inputs that name a request by IDs (`submit-files`, `request`,
     `register-project`, `registration-orders`) establish none: they carry `request.sender`,
     `request.caller` and `request.conduit` (`lib.rs`, lines 551 to 683; `registration.rs`,
     lines 1538 to 1593). For them the sending instance is `request.sender`, and the named
     Conduit is `request.conduit`. So the tier is 2 where `request.sender` is root and 1
     otherwise, and `named.conduit` and tier 2's members read the Conduit `request.conduit`.
   - **ACC-056's "caller".** ACC-056 says "Tier 2 is always given to root", and that tier 2 adds
     "the caller's Conduits". For an onward request the caller is the caller of the invocation
     being served, and the sending instance "is not the caller" (`spec/definitions.md`, "caller"
     and "the instance that sent a request"). This order reads ACC-056's "caller" as the sending
     instance, for the tier and for tier 2's `conduits`: the reply goes to the sender, so this is
     the reading that never shows tier 2 to an instance that is not root. It is the
     coordinator's reading, open to the owner's strike. Report it as a finding against ACC-056's
     wording, for SO-H's sweep.
   - `R` is `refusal.v1`:
     - `tier`: 2 where the sending instance is root, 1 otherwise. A request with no sending
       instance (none established, and no `request.sender`) gets 1.
     - `operation`: the qualified operation, where the input names the operation by ID, or where
       the request's tool identifies one: the sending instance holds a
       Conduit, not retired, whose `local_name` is the tool's text before its first dot, and the
       contract of that Conduit's target resolves and has an operation whose name is the text
       after that dot, whether or not the Conduit is open or lists the operation (ACC-056's
       `then` gives tier 1 the qualified operation on every refusal). Otherwise, where the
       spelling cannot identify an operation or a local Conduit name (ACC-056's observable), the
       tool's spelling as the request wrote it. For an onward request, the Conduit and its local
       name belong to that instance, not to the served invocation's caller.
     - `named.tool`: the tool as the request named it. For the inputs that name a request by IDs
       (`register-project`, `registration-orders`, `submit-files`, `request`), it is the tool that
       the named Conduit gives the operation: its `local_name`, a dot, and the operation's name
       after `#`. An implementation's refusal under IFC-026 takes it the same way, from the
       invocation's `conduit` and `operation`, since an invocation keeps no tool name.
     - `named.conduit`: the text of the tool before its first dot, where the sending instance
       holds a Conduit that is not retired and has that `local_name`; for the inputs that name a
       request by IDs, the `local_name` of `request.conduit`; omitted otherwise.
     - `named.objects`: the values of the request's arguments `object`, `instance`, `conduit` and
       `node`, in that order, each that is present and copied unchanged; omitted where none is.
       For IFC-026 they come from the invocation's `arguments`.
     - `cause`: the text of the refusing rule's row in ACC-056's table, verbatim, except for
       ACC-033 (below). The core refuses under 21 rules, each with a row: ACC-001, ACC-019,
       ACC-024, ACC-029, ACC-030, ACC-032, ACC-033, ACC-034, ACC-039, ACC-053, ACC-054, ACC-055,
       ACC-057, IFC-021, IFC-022, IFC-026, IFC-028, IFC-032, LIF-034, SEC-007 and SEC-008. Keep
       their rows, ACC-033's included, verbatim in one public table,
       `pub const REFUSAL_CAUSES: &[(&str, &str)]` in `conduit_core::explain`.
     - ACC-033's row is an instruction, not a cause: "The request needs the target described by
       its first fitting row. Tier 1 names it only if the request itself did, otherwise it
       describes that row without resolving an owner name." So for ACC-033 alone, `explain`
       does not copy the row; `cause` is `The request needs an open Conduit listing the
       operation to {D}.`, where `D` describes the authorizing target that ACC-033's first
       fitting row gives, in tier-1 terms:
       - `management` or `access`, where that target is the first-start instance of that name;
       - `the named {kind} {name}`, where that target is an object the request itself names,
         with its kind and name as the request wrote them;
       - `the owning Pool of the named Resource` or `the owning Fleet of the named Agent`, where
         that target is the owner of a named instance, which the request did not name;
       - for registration's refusal of the owner's Pool or Fleet (rows 1 and 1A),
         `management and the owning {Pool or Fleet} {name}`, with the owner's name as the
         submitted document states it;
       - `the named instance itself`, for a control (row 11).
       Where none of these fits, `D` is `the target of ACC-033's first fitting row`, and the
       worker reports the request. Build `D` from what the refusing site already holds; change
       nothing in `access::routing`.
     - Tier 2 adds three members. `target`: the `object.v1` (`kind`, `name`, `id`) of the target
       of the Conduit of the sending instance that `named.conduit` names (for the inputs that
       name a request by IDs, `request.conduit`), omitted where there is none.
       `exists`: false where there is no such Conduit, or where an object that `named.objects`
       references does not exist; true otherwise. `conduits`: every Conduit that is not retired,
       has no `spec`, has the sending instance as its consumer and
       `target`'s ID as its target, in the order of `state.objects`, each as
       `{name, as, state, operations}`, where `as` is its `local_name` and `state` is `open`
       where its `open` is true and `registered` otherwise; an empty list where there is no
       `target`. Closed Conduits from tombstones are 046b's.
   - `MESSAGE` is `"{named.tool} is refused under {RULE}: {cause}"`, and, where `named.tool` is
     omitted, `"{operation} is refused under {RULE}: {cause}"`. It holds no tier-2 fact.
   - **Two refusals that name no tool.**
     - A setup's part refused at `lifecycle.rs`, line 661, where `request_for` gives none, has
       no request tool, no request IDs and no Conduit. Its `operation` is the part's qualified
       operation (`CORE#open.v1` for a Conduit entry, `CORE#set-up.v1` otherwise, the `op` the
       code holds); `named.tool` and `named.conduit` are omitted; `named.objects` comes from the
       part's `args` as above; the tier is that of the setup invocation's `caller`. The
       diagnostic goes in that part's `diagnostics` in the setup's parts (the invocation's
       `setup.parts` and the object's `status.setup.parts`). The setup's reply holds no parts,
       and a setup that does not succeed returns none of the operation's outputs (IFC-025):
       add no parts to it.
     - A tools/list that `access::list` refuses (IFC-032 or IFC-022, from `established`) has
       `operation` `tools/list`, `named` `{}` and tier 1, since no instance is established. Its
       reply holds the diagnostic in `diagnostics`.
   - Add no field to the core's `Decision` (FR-350's verifier: the public form is the
     diagnostic).

3. **A diagnostic on every other reason** (IFC-050, OPS-005). Wherever the core gives the reason
   invalid, conflict or unavailable, the reply holds at least one error diagnostic whose `ruleId`
   is the deciding rule, at the reply's diagnostic place (decision 4).
   - A reply that already holds such a diagnostic with a specific message keeps it unchanged:
     those of OPS-029 (`requests.rs`, line 580), LIF-037, OPS-030, PKG-023, FST-006, CON-004 and
     every registration and loader fault.
   - The two generic messages, "The request is invalid." (`access.rs`, line 1080) and
     "The request reports {status}." (`requests.rs`, line 148), become
     `"{subject} reports {reason} under {RULE}: {cause}"`, and so does every diagnostic this
     decision adds. `subject` is the request's tool where the input names one, the qualified
     operation where it names one by ID, and `install of {address} {version}` for
     `install-archive`.
   - **The stubs of `answer.v1` and `sign-in.v1`** (`requests.rs`, lines 1046 to 1058). Their
     rewrite of `content` after `reject` goes. `reject` gives each decision 3's diagnostic, with
     `ruleId` PND-009 or PND-010 and the specific message
     `"{subject} reports unavailable under {RULE}: not built yet, order {order}."`, where `order`
     is the order the code names now (`047`, `048a`); it is kept as OPS-029's specific message is,
     and takes no cause from a table. `explain::first_text` (decision 4) then writes `content[0]`
     from it. The shell test that checks the stubs (`crates/shell/src/tests.rs`, about line 1632)
     keeps its assertion that `content[0].text` contains `not built yet, order {order}`, and
     gains one that `structuredContent.diagnostics[0].ruleId` is PND-009 or PND-010.
   - Add one where none is today: in `access::decide` for unavailable (ACC-037, CON-024, NAM-026,
     ACC-015); in `requests::envelope` for refused and unavailable; in `access::list`, whose
     answer to a tools/list with a reason holds no diagnostic today (IFC-050 says every reason
     the engine gives a consumer carries one): decision 2's diagnostic for refused, and this
     decision's, with `subject` `tools/list`, for unavailable under ACC-037; and in
     `archives.rs`, whose three empty lists become one diagnostic each, PKG-029 (line 143),
     PKG-025 (line 161) and PKG-024 (line 174).
   - `cause` for a reason other than refused comes from a second table,
     `pub const REASON_CAUSES: &[(&str, &str)]` in `conduit_core::explain`. Give it one entry for
     each rule the core gives as invalid, conflict or unavailable without a specific message:
     ACC-015, ACC-037, CON-024, CON-031, FST-006, IFC-026, IFC-036, LIF-009, LIF-015, LIF-017,
     LIF-020, LIF-032, LIF-041, NAM-026, OPS-023, OPS-029, OPS-031, OPS-032, OPS-034, PKG-024,
     PKG-025, PKG-029, PLC-011, POOL-004, PRV-020, REV-002, REV-011, REV-013,
     REV-014, SCH-001, SUB-009 and TPL-013. IFC-026 is there because an implementation's reason
     under it may be unavailable as well as refused (`requests::complete`, lines 1620 to 1647;
     the cases `setup-review-hook-unavailable` and `setup-review-hook-unavailable-no-effect`).
   - `explain` picks the table by reason: refused reads `REFUSAL_CAUSES`, every other reason
     reads `REASON_CAUSES`, so the same rule may have an entry in each.
     Each cause is one sentence in the engine's own words, ending with a period, that states what
     the rule requires, and names no object and no state. Wording is the engine's choice
     (IFC-050's observable). If a test or a case reaches a rule that neither table holds, add its
     cause in the same form and report it.
   - A cause that neither table holds is an error of the build, never a silent default:
     `explain` returns `Err` naming the rule.

4. **The wire forms** (IFC-025, IFC-012, IFC-036).
   - **The diagnostic's place.** A reply with a top-level `reason` (the `access` input,
     registration, `install-archive`, `submit-files`, `request`) holds it in `diagnostics`. A tool
     result holds it in `structuredContent.diagnostics`, for every reason, and for a failed or
     unknown outcome that carries one: today only the cascade's (decision 6), and order 046c's
     once it merges. The protocol error of IFC-012 holds it in
     `error.data.diagnostics`.
   - **The first text block** (IFC-025): where `isError` is true, `content[0]` is
     `{"type": "text", "text": T}`. `T` is the reason or outcome, then, where the result holds
     error diagnostics, a colon, a space and each error diagnostic's message, joined by one space.
     With none, `T` is the reason or outcome alone. A plain implementation's content follows the
     block, as now.
   - **`T` is computed once, from the reply's final diagnostics.** Several sites add, rewrite or
     replace a diagnostic after `envelope` or `reject` has built the reply: OPS-029's message
     (`requests.rs`, line 580), LIF-037's (about line 593), LIF-041's inventory diagnostic and
     `retirement::blocking_diagnostic` (about lines 911 to 925), the records preflight's
     diagnostics (about line 943), OPS-030's (about lines 966 to 982) and `cascade.rs`'s
     (lines 274 to 276). So `envelope` does not write `T`. One function,
     `explain::first_text`, writes `content[0]` from the error diagnostics the reply finally
     holds; the core calls it on the transition's reply and on each stored request reply once
     the input's handling has ended, before the transition returns. No other site writes
     `content[0].text`: OPS-030's hand rewrite of it goes, since its message gives the same `T`.
   - **-32602** (IFC-012; `requests.rs`, lines 584 to 586). For ACC-001 and TPL-013 the error's
     `message` becomes `"No tool named {tool} is presented to this instance. {MESSAGE}"`, where
     `MESSAGE` is the diagnostic's message. Its `data` keeps META/reason and gains `diagnostics`,
     holding the refusal diagnostic of decision 2 for ACC-001, and for TPL-013 the invalid
     diagnostic of decision 3. A tools/call refused under IFC-032 or IFC-022 keeps its tool
     result here, with decision 2's diagnostic in `structuredContent.diagnostics`: its -32602
     form is order 046e's (decision 14).
   - **The shell's own errors** (IFC-036). In `shell/src/server.rs`, each error that the shell
     makes with META/reason in its `data` gains `diagnostics` there, holding one diagnostic
     `{"ruleId": "IFC-036", "level": "error", "message": {"text": M}}`, where `M` is the error's
     own `message`. These are the invalid parameters for `serving` and `idempotencyKey`
     (lines 442 to 445), -32021 for an undeclared extension (line 632), and the protocol errors to
     which the shell adds the reason invalid (lines 695 to 706), where `data` holds no
     `diagnostics` yet.
   - **The CLI's tier 2.** `conduit call` prints the tool result as the server returns it
     (`shell/src/main.rs`, line 155), so root sees tier 2 with no change to the CLI. Change
     nothing in `main.rs`; the shell test of decision 13 shows it.

5. **Records: the rule and the tier** (`record.v1`: "A reason records ruleId; a refusal records
   refusalTier without its tier-2 list"). Every record of type `access-decision` or
   `request-ended` whose `data` holds `reason` also holds `ruleId`, the deciding rule, and, where
   the reason is refused, `refusalTier`, the refusal's tier. The site that decides takes both from
   the rule and tier it holds and passes to `explain`, not by reading them back from the
   diagnostic it built, so a record's `ruleId` never depends on the diagnostic's. No record holds
   a tier-2 member. This includes the two records of a refused setup part (`lifecycle.rs`, lines 664 to
   670).

6. **Outcomes the engine decides: order 046c.** This decision, an outcome that the engine
   decides as failed or unknown naming its deciding promise and cause (FR-353's half of IFC-050;
   D37), with its table of sites, the repeat that carries a stored cause, and a held-back
   part's POOL-013 diagnostic, moved whole to [order 046c](046c-build-outcome-diagnostics.md)
   (see "Who decided what"). Here, change nothing in how an outcome ends: `requests::finish`
   keeps its signature, no site gains a cause, and an outcome's reply holds the diagnostics it
   holds today.
   Decision 4's first text block applies to an outcome as to a reason, so an outcome whose reply
   holds no error diagnostic keeps `T` as the outcome alone, and the cascade's failed reply,
   whose LIF-021 diagnostics `cascade.rs` writes after `requests::finish` (lines 274 to 279),
   states them in `T`.

7. **The runner checks every step** (IFC-050, ACC-056).
   - The checks below run in `Checks::run` (`crates/conformance/src/checks.rs`) before its
     existing validation of records (`record.v1: `) and of diagnostics (`diagnostic.v1: `), each
     of which returns at its first fault, so that where one of them fails, its fault is the one
     the step reports.
   - In `crates/conformance/src/checks.rs`, for each step whose input kind is `access-fixture`,
     `invoke`, `request`, `request-fixture`, `lifecycle-fixture`, `submit-files`,
     `register-project`, `registration-orders`, `install-archive`, `resolve-project`, `complete`,
     `cancel`, `setup-report` or `list-tools`, read the actual reply's reason: its top-level
     `reason`, its META/reason, or the META/reason key of `error.data`. Where it has one, with `{pointer}` the
     diagnostic place of decision 4 written from the step's result (`/reply/diagnostics`,
     `/reply/structuredContent/diagnostics` or `/reply/error/data/diagnostics`):
     - its diagnostic place of decision 4 holds at least one diagnostic of level `error` whose
       `ruleId` matches `^[A-Z]+-[0-9]{3}$`; otherwise the step fails with
       `IFC-050: a reply with reason {reason} holds no error diagnostic with a ruleId at {pointer}`;
     - where the reason is refused, that place holds exactly one diagnostic, with
       `properties.refusal`; otherwise
       `ACC-056: a refused reply holds {n} diagnostics at {pointer}; it must hold exactly one`;
     - each diagnostic there validates against `diagnostic.v1`, and each `properties.refusal`
       against `refusal.v1`, read through `spec.contract_validator` as the three validators there
       are; a failure begins `refusal.v1: `.
   - For each of those steps whose actual reply is a tool result with `isError` true, whether it
     ends with a reason or an outcome, `content[0]` is `{"type": "text", "text": T}`, with `T`
     computed as decision 4 says from the error diagnostics that the actual reply holds;
     otherwise the step fails with `IFC-025: the first text block is {actual}; it must be {T}`.
     This is the check that fails where a site rewrites a diagnostic after `T` was written.
   - `schema-case`, `schema-admission`, `schema-evaluation`, `validate-values` and the loader's
     inputs answer no request, and are not checked.
   - In `crates/conformance/src/access.rs`, lines 52 to 63, the allowed members of a refused or
     unavailable reply become `reason`, `restriction` and `diagnostics`, and the error message
     stays "refusal/unavailability disclosed extra fields".

8. **FR-547: the negative assertions beside a reason go** (F59; FR-547's least change). From
   the repository root, run this once, before decision 10 writes its list and before any new case
   exists. It changes only the steps whose expected reply ends with a reason: a top-level
   `reason`, or META/reason in `_meta`. It changes 34 cases and rewrites each in the form every
   one of them has now, JSON with two-space indentation and a final newline. It reads the `_meta`
   prefix from `spec/names.yaml`:

   ```bash
   python3 - <<'EOF'
   import json, pathlib
   PREFIX = next(line.split(":", 1)[1].strip()
                 for line in pathlib.Path("spec/names.yaml").read_text().splitlines()
                 if line.startswith("metaPrefix:"))
   REASONS = {"refused", "invalid", "conflict", "unavailable"}
   for path in sorted(pathlib.Path("conformance").rglob("case.yaml")):
       case = json.loads(path.read_text())
       before = json.dumps(case, indent=2, ensure_ascii=False) + "\n"
       for step in case["steps"]:
           expect = step["expect"]
           reply = expect.get("reply")
           if not isinstance(reply, dict):
               continue
           meta = reply.get("_meta")
           meta = meta if isinstance(meta, dict) else {}
           if "reason" not in reply and PREFIX + "reason" not in meta:
               continue
           content = reply.get("content")
           if (isinstance(content, list) and content and isinstance(content[0], dict)
                   and content[0].get("text") in REASONS):
               del content[0]["text"]
           absent = expect.get("absent")
           if isinstance(absent, list):
               count = len(absent)
               if PREFIX + "reason" in meta and "/reply/structuredContent" in absent:
                   absent.remove("/reply/structuredContent")
               if "reason" in reply and "/reply/diagnostics" in absent:
                   absent.remove("/reply/diagnostics")
               if count and not absent:
                   del expect["absent"]
           if "reason" in reply and reply.get("diagnostics") == []:
               del reply["diagnostics"]
       after = json.dumps(case, indent=2, ensure_ascii=False) + "\n"
       if after != before:
           path.write_text(after)
   EOF
   ```

   A tool result's step loses `/reply/structuredContent` from `absent` where its `_meta` holds
   META/reason: 15 steps in 7 cases. Two of them, `records-expected-id` steps 6 and 8, state
   `isError` true. The other 13, in 6 cases (`root-consumer-open` step 1,
   `expected-id-access-order` step 1, `root-consumer-register-invalid` step 4,
   `root-consumer-onward` steps 1 and 2, `root-consumer-register` steps 2 and 3, and
   `root-consumer-register-onward-invalid` steps 1 to 6), expect a refused tool result without
   stating `isError`. Decision 4 puts the diagnostic of each in `structuredContent.diagnostics`.

   **The steps that end with an outcome stay as they are.** The script changes no step whose
   expected reply ends with an outcome, failed or unknown, whether the engine or an
   implementation decided it: decision 6 leaves every outcome's reply as it is, so their bare
   word and their absent `/reply/structuredContent` still hold, and a build that adds a
   diagnostic to an outcome fails them. At `9af1c29f` there are 42 such steps that the old form
   pins, in 34 cases. Order 046c removes those assertions on the 32 steps whose outcome the
   engine decides, when it gives them a diagnostic, and keeps them on the ten whose outcome an
   implementation stated or IFC-010 gives as a plain read's own failure.

   Each case keeps every reason and outcome it asserts. At `9af1c29f` the script changes 25 steps
   in 12 cases that pin the first text block to a bare reason, removes `/reply/structuredContent`
   from `absent` in 15 steps of 7 cases, `/reply/diagnostics` in 35 steps of 11 cases, and an
   empty `diagnostics` in 7 steps of 7 cases: 34 cases in all. The positive assertions are
   decision 7's check, on every step of every case. Edit none of these cases by hand, except
   as the next paragraph lists. If `main` has moved and the counts differ, report the counts you
   see.

   **C9's tier 1, one listed hand edit.** The plan gives 046 C9's tier-1 assertions ("In 040 it
   asserts the refused reason as today; 046 adds the tier-1 assertions"; "the runner: the new
   assertions, including protected's tier 1"), and order 117 gives 046a the runner's
   assertions. After the script, edit `first-start-protected-without-pool`
   (`conformance/language/first-start/protected-without-pool/case.yaml`), steps 1 to 3, an
   Interface that is not root refused under ACC-033, and nothing else in it. In each of the three
   steps, the expected `reply` becomes `{"reason": "refused", "diagnostics": [{"ruleId":
   "ACC-033", "level": "error", "properties": {"refusal": {"tier": 1}}}]}`, and `expect` gains
   `record_facts` `[{"type": "request-ended", "data": {"ruleId": "ACC-033", "refusalTier": 1}}]`,
   placed right before `records`. Write the file in the same form as the script does.

9. **FR-554: `list.v1`'s results as sets.** In `crates/conformance/src/expect.rs`, before the
   projection, for a step whose input is `invoke` and whose request's `tool` ends in `.list.v1`:
   for each member of the expected `reply.structuredContent` whose value is an array, and whose
   actual counterpart is an array of the same length, find a one-to-one matching in which each
   expected item projects onto its actual item, by augmenting paths. Where one exists, reorder the
   actual array into the expected order; where none exists, leave it as it is, so that the
   ordinary projection reports the failure. Nothing else is compared differently. In
   `conformance/README.md`, line 77, "Arrays have exact cardinality and order" gains ", except the
   arrays of a `list.v1` result, which are compared without regard to order, since OPS-036 states
   none". Change no case.

10. **D36: the vector rule, with a check** (FR-350, rules 1 and 2, as its verifier corrected them).
    - **The internal cases.** Before decision 11 adds any case, write
      `conformance/internal-cases.yaml`, which lists every case in the tree, by running from the
      repository root:

      ```bash
      python3 - <<'EOF'
      import json, pathlib
      ids = sorted({json.loads(p.read_text())["id"] for p in pathlib.Path("conformance").rglob("case.yaml")})
      pathlib.Path("conformance/internal-cases.yaml").write_text(
          json.dumps({"cases": ids, "exceptions": []}, indent=2) + "\n")
      EOF
      ```

      At `9af1c29f` it lists 510 IDs. The runner reads it before any case runs, as it reads the
      expected failures, in a new module `crates/conformance/src/portable.rs`: the root is an
      object with only `cases` and `exceptions`. `cases` is a list of strings, in byte order,
      with no duplicates, each the ID of a case in the tree. `exceptions` is a list, in byte
      order of `case`, of objects with only `case`, `initial`, `steps`, `state` and `why`
      (below). Each fault begins `conformance/internal-cases.yaml: ` and says what would fix it,
      as `expected.rs` words its own. No later order adds an ID to `cases`.
    - **D36's exception.** D36 reads: "No new fixture patches state, except recovery, crash and
      restore states", which client requests cannot reach (FR-350's verifier). A later order
      whose new case needs such a state adds one item to `exceptions`, with a stated reason:
      `case`, the ID of a case in the tree that `cases` does not hold; `initial`, true where
      the case may start from a snapshot recipe other than `{"recipe": "empty"}`; `steps`, the
      numbers, from 1, ascending and unique, of the steps that may use `lifecycle-fixture` or
      `request-fixture`, each within the case; `state`, one of `recovery`, `crash` and
      `restore`; and `why`, a non-empty sentence saying why requests cannot reach the state. An
      item lifts only those two rules, for that case and those steps; every other rule below
      holds. This order adds no item.
    - **A portable case** is one that the list does not hold. Before its steps run, the runner
      fails it, with an error that begins `portable case: `, where:
      - its `initial` is not `{"recipe": "empty"}`, unless its item of `exceptions` states
        `initial` true;
      - a step's input kind is not one of these, which a client or provider can send or a
        client can read: `first-start`, `install-archive`, `resolve-project`,
        `register-project`, `registration-orders`, `submit-files`, `invoke`, `complete`,
        `setup-report`, `cancel`, `restore-state`, `list-tools`, `export-records`, `load` and
        `read` (a document a client submits, and the loader's verdict on it). Excluded, since
        they patch state, call the engine's own functions on an internal fixture, or stand for
        its own timer: `access-fixture`, `request-fixture`, `lifecycle-fixture`, `request`,
        `expire-outputs`, `schema-case`, `schema-admission`, `schema-evaluation` and
        `validate-values`. A step that its case's item of `exceptions` lists may also use
        `lifecycle-fixture` or `request-fixture`. A kind that a later order adds to the case
        schema is excluded until an order adds it here;
      - a step's `expect` holds `object_facts`, `invocation_facts`, `object_count`,
        `absent_objects`, `tombstone_count`, `tombstone_facts`, `tombstones`,
        `absent_tombstones`, `objectIds`, `inventory_facts` or `stored_record_facts`;
      - a step's `expect` holds `state`, `stateUnchanged` or `required_promises`, unless its input
        kind is one for which the case schema requires them (`first-start`, `submit-files`,
        `register-project`, `registration-orders`, `install-archive`, `resolve-project`): those
        steps build the state a case needs, and keep only what the schema asks;
      - an `absent` pointer does not begin `/reply`, `/records` or `/external_intents`, or a
        `capture` pointer does not begin `/reply` or `/records`.
    - After each step of a portable case runs, where its actual reply has a reason (decision 7),
      the step's expected reply must state that reason at the same place and hold, at the
      diagnostic place of decision 4, an array whose first item states `ruleId`. Otherwise the
      step fails with `portable case: step {n} ends with reason {reason} but does not assert it
      and its deciding ruleId`.
    - The case schema does not change: it cannot see the list, and its required members stay as
      they are.
    - In `conformance/README.md`, add a section "Portable cases" after "Expected failures", which
      states the list, the rule with its allowed and excluded input kinds, its exception for the
      steps that build state, D36's exception for recovery, crash and restore states with the
      form of an `exceptions` item, and that the existing cases stay as they are (D36).

11. **Three portable cases**, under `conformance/conventions/explain/`, in
    `conformance/conventions/explain/refusal-tiers/`, `reason-diagnostic/` and `agent-results/`,
    each holding `case.yaml`; a case's ID is the path after the layer, as
    `conventions/list/from-first-start` gives `list-from-first-start`. Each is written as the
    existing cases are (JSON with two-space indentation and a final newline; members `id`,
    `layers`, `promises`, `title`, `profile`, `initial`, `files` and `steps`, in that order;
    `layers` `["convention", "interface"]`; `profile` `first-build`; `initial`
    `{"recipe": "empty"}`). They follow decision 10. A new step is one whose input copies no
    existing step's input or evidence: steps 2 and 3 of `explain-refusal-tiers` and step 3 of
    `explain-reason-diagnostic`. Every other step copies an existing step's input whole, its
    evidence with it. One counter runs across the three cases, in the order they are
    listed below, over the new steps alone: the first new step's evidence has the time
    `2026-11-09T13:00:00Z` and each next one a second later (13:00:00Z, 13:00:01Z and
    13:00:02Z), and IDs of the form `00000000-0000-4000-8046-a0000000NNNN`, four for each new
    step, numbered from `0001` on the same counter (`0001` to `0004` for the first, up to `0009`
    to `0012` for the third). A copied step keeps its own evidence. Every `required_promises` that a building
    step keeps holds only the first promise of the step it copies.

    **`explain-refusal-tiers`** (promises ACC-056, IFC-050, IFC-025, IFC-012, ACC-001, LIF-034;
    title "A refusal names one rule, with tier 2 for root"). Tier 1 is asserted by
    `explain-agent-results` step 14. A call from a client with no established instance, refused
    under IFC-032, gets no step here: decision 14 says why and which order takes it.
    `files` is `{}`.
    - Step 1 builds the state: `list-from-first-start`'s step 1 input, whole, with
      `expect` `{"reply": {"started": true}, "external_intents": [], "required_promises":
      ["FST-001"], "state": {"started": true}, "record_facts": []}` and no `capture`.
    - Step 2, root (`00000000-0000-4000-8000-000000000005`) asks `management.retire.v1` with
      `{"instance": {"kind": "Pool", "name": "system"}}`. Expect `external_intents` `[]`,
      `record_facts` `[{"type": "request-ended", "data": {"reason": "refused", "ruleId":
      "LIF-034", "refusalTier": 2}}]`, and a reply: META/reason `refused`, `isError` true, and
      `structuredContent.diagnostics` `[{"ruleId": "LIF-034", "level": "error", "properties":
      {"refusal": {"tier": 2, "operation": "CORE#retire.v1", "named": {"tool":
      "management.retire.v1", "conduit": "management", "objects": [{"kind": "Pool", "name":
      "system"}]}, "target": {"kind": "Resource", "name": "management", "id":
      "00000000-0000-4000-8000-000000000003"}, "exists": true, "conduits": [{"as": "management",
      "state": "open"}]}}}]`.
    - Step 3, root asks `nope.list.v1` with `{}`. Expect `external_intents` `[]` and a reply
      `{"error": {"code": -32602, "data": {META/reason: "refused", "diagnostics": [{"ruleId":
      "ACC-001", "level": "error", "properties": {"refusal": {"tier": 2, "operation":
      "nope.list.v1", "named": {"tool": "nope.list.v1"}, "exists": false, "conduits": []}}}]}}}`.

    **`explain-reason-diagnostic`** (promises IFC-050, IFC-025, OPS-005, PKG-024, OPS-029; title
    "Every reason carries its deciding diagnostic"). `files` is `archives-unavailable`'s, each
    path rewritten relative to the new case's directory.
    - Step 1: `archives-unavailable`'s step 1, input and `expect` whole, with its reply's
      `diagnostics` `[{"ruleId": "PKG-024", "level": "error"}]` in place of the member that
      decision 8 removed.
    - Step 2 builds the state, as step 1 of `explain-refusal-tiers`.
    - Step 3, root asks `management.inspect.v1` with `{"object": {"kind": "Resource", "name":
      "missing"}}`. Expect `external_intents` `[]`, `record_facts` `[{"type": "request-ended",
      "data": {"reason": "invalid", "ruleId": "OPS-029"}}]`, and a reply: META/reason
      `invalid`, `isError` true, and `structuredContent.diagnostics` `[{"ruleId": "OPS-029",
      "level": "error"}]`.

    **`explain-agent-results`** (promises ACC-056, IFC-050, IFC-012, ACC-001; title "An Agent's
    refusal is tier 1"). Its steps that asserted an outcome's diagnostic under IFC-010 are order
    046c's, in a case of its own (decision 6).
    `files` is `s1-whole`'s, each path rewritten relative to the new case's directory.
    - Steps 1 to 13 build the state: `s1-whole`'s steps 1 to 13, each input and `capture` whole,
      except step 1's `capture`, which reads `/state` and is dropped. Cut each `expect` to the
      members decision 10 allows its kind, keeping them as `s1-whole` has them, with three
      exceptions: step 1's `state` is `{"started": true}`, not `s1-whole`'s whole internal state
      (D36: a new case asserts only what a client can see); step 1's `absent`
      (`/state/objects/6/bound`, `/state/objects/6/requirements`, which decision 10 refuses) is
      dropped; and every `required_promises` kept holds only the first promise of `s1-whole`'s
      step, as the general rule above says for every building step. Step 3's `state` is kept as
      it is, `{"started": true}`. Where a kept expectation uses a symbol of step 1's dropped
      `capture`, write the ID that `s1-whole`'s step 1 `state` gives that object.
    - Step 14: `s1-whole`'s step 20 input whole, agent-a asking `project.file.write.v1`, which
      its Conduit does not list. Expect `external_intents` `[]`, `record_facts` `[{"type":
      "request-ended", "data": {"reason": "refused", "ruleId": "ACC-001", "refusalTier": 1}}]`,
      and a reply `{"error": {"code": -32602, "data": {META/reason: "refused", "diagnostics":
      [{"ruleId": "ACC-001", "level": "error", "properties": {"refusal": {"tier": 1,
      "operation": "example.org/s1/filesystem#file.write.v1", "named": {"tool":
      "project.file.write.v1", "conduit": "project"}}}}]}}}`. The operation is the qualified one:
      agent-a holds the Conduit `project`, whose target's contract has `file.write.v1`
      (decision 2).

    Index each case in `conformance/conventions/explain/index.yaml` with `id`, `case` (its path,
    such as `conventions/explain/refusal-tiers/case.yaml`, written as the other pieces write it),
    `promises`
    and `safety_rows: []`, in that order, as order 171a did: write the entries in a `cases` list in
    `conformance/manifest.yaml`, then run, from `engine/`, `cargo run --release -p
    conduit-conformance --example pieces` with no flag. It makes the new piece and leaves
    `manifest.yaml` byte for byte as it was. Change no `s1_coverage` row.

    If a step expects what the build does not give, check the step against this order and the
    specification first. Fix the step only where it is wrong against them; otherwise fix the
    build. Never write an expectation from the engine's output.

12. **The standing table.** Once the cases, the index and the expected failures are in place, run
    `python3 tools/promise_standing.py --write` from the repository root. It rewrites
    `engine/STANDING.md`. Never edit that file by hand.

13. **Unit tests.**
    - In the core, in `explain.rs` or beside the code they test:
      - one test for each moved check of decision 1, each with a request that two rules refuse,
        expecting the earlier: ACC-039 (a retired sender) before ACC-001; SEC-007 decides a
        `get-secret.v1` request through an open, ready Conduit that lists it; a `get-secret.v1`
        request through a Conduit that is open and lists `get-secret.v1` but is not ready is
        refused under ACC-001, not SEC-007 (on `main`, `evaluate` selects that Conduit and
        refuses under SEC-007, so this test fails before the move); ACC-032 (a reserved
        operation) before ACC-033's routing; ACC-057 before ACC-033's routing; ACC-029 before ACC-054 and before SEC-008 on an open;
        registration's ACC-033 before ACC-054; in `registration::register`, ACC-001, then
        ACC-057, then ACC-033; and, for `lib.rs`'s `Request`, ACC-001 where `authorized_path`
        gives false, before LIF-034;
      - the tiers: root gets 2, an Agent and another Interface get 1, a request with no
        established instance gets 1, and a tier-1 refusal holds no `target`, `exists` or
        `conduits`; for an input that names a request by IDs, the tier is 2 where
        `request.sender` is root and 1 otherwise, and `named.conduit`, `target` and `conduits`
        read `request.conduit` (decision 2);
      - `operation`: the qualified operation where the sending instance holds a Conduit of the
        tool's local name whose target's contract has the operation, whether or not that Conduit
        is open or lists it, and the spelling where no such Conduit or operation exists;
      - `named`: `conduit` omitted where the sending instance has no Conduit of that local
        name, and, for an onward request, taken from the sending instance's Conduits, not the
        served invocation's caller's; and `objects` in the order of decision 2;
      - ACC-033's cause: one test for each form of `D` in decision 2, and none of them names an
        owner the request did not name;
      - the two refusals that name no tool: a refused setup part's diagnostic, in the part's
        `diagnostics`, with `operation` the part's qualified operation and no `named.tool`; and a
        refused tools/list's, with `operation` `tools/list`, `named` `{}` and tier 1;
      - a setup part whose child request `invoke_part` refuses with the -32602 error under
        ACC-001: the part's `diagnostics` hold that error's ACC-001 diagnostic, and its reason is
        refused;
      - `explain` picks the table by reason: IFC-026 refused takes `REFUSAL_CAUSES`'s cause,
        and IFC-026 unavailable `REASON_CAUSES`';
      - every entry of the two tables is one non-empty sentence ending with a period, and
        `explain` returns `Err` for a rule in neither table;
      - an outcome, decided by the engine or stated by an implementation, whose reply holds no
        diagnostic and whose first text block is the outcome alone, as decision 6 leaves it;
      - the records of decision 5;
      - the -32602 error's message and `data`.
    - In the conformance crate:
      - a test that, for each `(rule, cause)` of `REFUSAL_CAUSES`, the line
        `    | {rule} | {cause} |` occurs in `spec/conventions/access.yaml`;
      - each fault of decision 7's check, of `internal-cases.yaml`'s reading and of decision 10's
        rule, each giving its stated message, the IFC-025 fault of a stale first text block
        among them;
      - D36's exception: a case that an `exceptions` item names, with a `lifecycle-fixture` step
        that the item lists, passes the rule; the same step, unlisted, fails it; and an item
        whose `state` is not one of the three fails the reading;
      - decision 9: a `list.v1` reply whose expected items are reversed passes, and one with an
        item missing fails.
    - In the shell, in `crates/shell/src/server_tests.rs`, beside the tests that call a tool:
      - `root_refusal_carries_tier_two`: root calls `management.retire.v1` for the Pool `system`
        through the server; the result's `structuredContent.diagnostics[0].properties.refusal`
        has `tier` 2 and one `conduits` entry, whose `as` is `management`;
      - `shell_rejection_carries_diagnostic`: a call whose `serving` is an empty string gets the
        invalid parameters error, whose `data.diagnostics[0].ruleId` is IFC-036.
    - A shell or core test that asserts a reply, record or message this order changes may change
      that assertion alone. Report each such test.

14. **`engine/CONFORMANCE.md` and `conformance/README.md`.**
    - In `engine/CONFORMANCE.md`, add a section "Refusals and reasons (order 046a)": what this
      order built (decisions 1 to 5 and 7 to 11), and, as unfinished, each with its order: tier
      2 for callers other than root and closed Conduits from tombstones (046b); an outcome the
      engine decides as failed or unknown, which carries no diagnostic naming its cause yet
      (046c); a bounded holder's request naming no Conduit, still invalid under OPS-029, with
      F2's vectors and F3's other vectors (an order of the admin's, not yet numbered); the https
      fetch citing PKG-023, which answers as unbuilt under D37 (046d) until https is built
      (PSD3); a tools/call from a client with no established instance, refused
      under IFC-032 or IFC-022, still answered as a tool result where IFC-032's and IFC-025's
      observables answer it as IFC-012 states for a tool that is not presented, with the -32602
      error and the reason in its `data` (046e; `records-expected-id` step 10 pins the tool
      result today); the
      stubs of `answer.v1` and `sign-in.v1` (047, 048a-2).
    - In `conformance/README.md`: the sentence of the access fixture's paragraph (line 482),
      "that refused/unavailable replies contain only a reason and an optional restriction",
      gains "and the deciding diagnostic"; the record vocabulary (line 97) says that a reason's
      records hold `ruleId`, and a refusal's `refusalTier`; a sentence under "Assertions" states
      decision 7's check, the first text block's included; and decisions 9 and 10 as they say.

15. **The declaration.** `docs/work-orders/046a-declared.yaml` is written: five shapes, one for
    the three new cases, one for decision 8's hand edit of `first-start-protected-without-pool`,
    and three for its script's edits, each with its cases listed. Change it
    only where `main` has moved and the script of decision 8 changes other cases, and then only
    their lists, and report it.

16. **Before you finish, check:**
    - every refused request in the vectors names one rule, the first in ACC-056's order;
    - no tier-1 refusal holds `target`, `exists` or `conduits`, and no message or record holds a
      tier-2 fact;
    - every reason in every vector carries its deciding diagnostic (decision 7's check passes on
      all cases);
    - no outcome gains a diagnostic: that is order 046c's (decision 6).

## Scope

Change:

- `engine/crates/core/`: the new `src/explain.rs`, and `access.rs`, `requests.rs`, `lifecycle.rs`,
  `registration.rs`, `lib.rs` and `archives.rs` at the sites the decisions name; report any
  other core file you change, and why;
- `engine/crates/conformance/`: `checks.rs`, `expect.rs`, `access.rs`, `lib.rs`, the new
  `src/portable.rs`, and their tests;
- `engine/crates/shell/src/server.rs` at decision 4's sites, and the shell's tests;
- `engine/CONFORMANCE.md`, and `engine/STANDING.md` through the tool alone;
- `conformance/`: the 34 cases of decision 8, through its script, and steps 1 to 3 of
  `first-start-protected-without-pool` by decision 8's listed hand edit alone; the new
  `conventions/explain/` with its three cases and its `index.yaml`; the new `internal-cases.yaml`;
  `expected-failures.yaml`, one entry removed; `README.md`;
- `docs/work-orders/046a-declared.yaml`, only as decision 15 allows;
- `experiments/011-short-form-spike/`, only if its tests need it: it depends on the core;
- `engine/crates/core/src/cascade.rs`, by the fixer's amendment alone (see the Record): `ruleId`
  in the data of a held member's two records, and a test of it.

Do not change:

- `spec/`, `IN_FORCE.md`, `tools/`, `packages/`, or any record (`docs/proposals/`,
  `docs/AUDIT.md`, `ROADMAP.md`, `docs/implementation/`): report any defect as a finding;
- `conformance/case.schema.json` and `conformance/case-schema/`;
- `conformance/manifest.yaml`, which ends byte for byte as it was;
- any case outside decisions 8 and 11, by hand or by tool;
- the shell's `main.rs`, `runner.rs` and `lib.rs`;
- `access::routing`, the repeat index in `requests.rs`, and OPS-029's invalid answer (F2's
  order, not yet numbered);
- how an outcome ends: `requests::finish`'s signature, `retirement.rs`, `cascade.rs` but for the
  amendment above, and the outcome sites of `requests.rs` and `lifecycle.rs` (order 046c's).

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
python3 tools/vector_delta.py HEAD --declared docs/work-orders/046a-declared.yaml
```

Run the Python checks that need PyYAML with `/usr/bin/python3` where `python3` lacks it.
`check_model_account.py` and `promise_standing.py --check` must end `Errors: 0`; the second
fails until decision 12's `--write` has run. At `9af1c29f`, `vector_delta.py` must end
`Vector delta: 195 changes; 0 flips; 0 undeclared; 0 errors.`: 174 from decision 8's script, 18
from its hand edit and the three new cases. Where `main` has moved,
`0 flips; 0 undeclared; 0 errors.` must hold, and report the count.

This prints `0 0 0 0 42`: the first four are the old form left beside a reason, and the last is
the steps that end with an outcome and still assert `/reply/structuredContent` absent, which
decision 8 leaves for order 046c. Where `main` has moved and the last differs, report it.

```bash
python3 - <<'EOF'
import json, pathlib
P = next(l.split(":", 1)[1].strip() for l in pathlib.Path("spec/names.yaml").read_text().splitlines()
         if l.startswith("metaPrefix:"))
W = {"refused", "invalid", "conflict", "unavailable"}
a = b = c = d = o = 0
for p in pathlib.Path("conformance").rglob("case.yaml"):
    for s in json.loads(p.read_text())["steps"]:
        e = s["expect"]; r = e.get("reply"); ab = e.get("absent") or []
        if not isinstance(r, dict):
            continue
        m = r.get("_meta") if isinstance(r.get("_meta"), dict) else {}
        if m.get(P + "outcome") in ("failed", "unknown"):
            o += "/reply/structuredContent" in ab
        if "reason" not in r and P + "reason" not in m:
            continue
        ct = r.get("content")
        a += isinstance(ct, list) and bool(ct) and isinstance(ct[0], dict) and ct[0].get("text") in W
        b += P + "reason" in m and "/reply/structuredContent" in ab
        c += "reason" in r and "/reply/diagnostics" in ab
        d += "reason" in r and r.get("diagnostics") == []
print(a, b, c, d, o)
EOF
```

Each of these prints 1:

```bash
grep -c '"access-close-root-outside-bound"' conformance/internal-cases.yaml
grep -c 'pub const REFUSAL_CAUSES' engine/crates/core/src/explain.rs
grep -c '^## Portable cases' conformance/README.md
grep -c 'Refusals and reasons (order 046a)' engine/CONFORMANCE.md
```

Each of these prints 0:

```bash
grep -c 'access-close-root-outside-bound' conformance/expected-failures.yaml
grep -c 'explain-' conformance/internal-cases.yaml
grep -c 'The tool is not presented.' engine/crates/core/src/requests.rs
grep -c -e 'The request is invalid.' -e 'The request reports' engine/crates/core/src/access.rs engine/crates/core/src/requests.rs | awk -F: '{s+=$2} END {print s}'
```

This prints nothing: `git status --short -- conformance/manifest.yaml conformance/case.schema.json conformance/case-schema spec tools packages`.

From `engine/`, one at a time:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. explain-
cargo run --release -p conduit-conformance -- .. access-close-root-outside-bound
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

What each must show:

- The pieces tool prints `Nothing to move.`
- The `explain-` run prints a `PASS` line for each of the three cases and ends `3 cases passed`.
- The `access-close-root-outside-bound` run prints
  `PASS access-close-root-outside-bound (2 steps; ACC-029, ACC-056)` and ends `1 cases passed`.
- The two test commands pass. Report the totals passed, failed and ignored of the full run. If a
  shell test fails, rerun it alone; rerun a timing-sensitive one with `-- --test-threads=2`.

Then show that decision 7's check and decision 1's order can fail. From `engine/`, one at a time:

1. `cp crates/core/src/explain.rs target/explain.rs.046a`
2. Make the function that builds the refusal diagnostic of decision 2 return a diagnostic with no
   `ruleId`. Run `cargo run --release -p conduit-conformance -- .. access-bounds`. It must fail,
   printing a line that holds
   `IFC-050: a reply with reason refused holds no error diagnostic with a ruleId at /reply/diagnostics`.
3. `cp target/explain.rs.046a crates/core/src/explain.rs`
4. `cp crates/core/src/access.rs target/access.rs.046a`; move the bound's check of decision 1,
   step 6, back after ACC-024; run the `access-close-root-outside-bound` command. It must fail,
   printing `uncited expected promise "ACC-029"`.
5. `cp target/access.rs.046a crates/core/src/access.rs`, and run both commands a last time. Each
   must pass.

Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild. Then `cmp crates/core/src/explain.rs target/explain.rs.046a` and
`cmp crates/core/src/access.rs target/access.rs.046a` each exit 0.

In `experiments/011-short-form-spike/`: `cargo test --release`.

Do not stage anything. In your report:

- the cases added and changed, by path, and the counts that decision 8's script and
  `vector_delta.py` gave;
- each test of decision 13 by name, and each existing test whose assertion you changed;
- the causes you wrote for `REASON_CAUSES`, and any rule you added to a table;
- the line each run that had to fail printed;
- every choice you made yourself, marked as yours;
- every finding against the specification, among them the finding against ACC-056's word
  "caller" that decision 2 asks for, for SO-H's sweep;
- the last line of every check.

`git diff --check` and `check_private_paths.py` read tracked files only. The admin then stages the
new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. Before
the merge the admin runs `python3 tools/gate.py`, and merges on its PASS
([ruling 80](../proposals/ruling-80/README.md)); GitHub's CI runs on the pull request too, and the
merge does not wait for it.

## Record

Run on 2026-11-17, from about 09:52 to 12:53 UTC, on the Mac from `main` `cbb49fc1` (#342, which
added this order), in three rounds: the worker (6236 s) and a fresh Codex verifier (574 s), the
worker again (1704 s) and a second fresh verifier (418 s), then the worker a third time (1457 s)
and a third fresh verifier (448 s).

The core gains `explain.rs`, with decision 2's 21 refusal rows verbatim as `REFUSAL_CAUSES` and 40
`REASON_CAUSES` entries, whose wording is the worker's (beyond the seed list: CON-025, INS-002,
LIF-029, OBJ-025, OPS-010, OPS-030 and OPS-033, and POOL-003 from round 2). A refusal carries one
diagnostic, at tier 2 where root sends and tier 1 for every other sender; `access.rs` checks the
bound before ACC-024 (decision 1) and splits registration and submission authority; every other
reason gains a diagnostic, and specific fault messages are kept; reason records carry `ruleId`, and
refusal records `refusalTier`; the shell's `server.rs` answers IFC-036 with a diagnostic. The
conformance runner checks every request step's reason, refusal and first text block (decision 7),
matches `list.v1` arrays as sets (decision 9), and gains `portable.rs` and the frozen
`internal-cases.yaml` of 510 cases (decision 10). Decision 8's script changed 34 cases (25 text
assertions, 15 `structuredContent` absences, 35 `diagnostics` absences and 7 empty lists), and its
hand edit steps 1 to 3 of `first-start-protected-without-pool`; the three `explain-` cases are new,
and `access-close-root-outside-bound` left `expected-failures.yaml`. `vector_delta.py` gave
`195 changes; 0 flips; 0 undeclared; 0 errors.` `engine/CONFORMANCE.md`, `conformance/README.md` and
`engine/STANDING.md`, through the tool, follow; decision 13's 32 tests were added. The worker's own
choices include a private carrier for a registration fault's rule, and, of several registration
faults, recording the first error of the chosen reason class. Its finding for SO-H's sweep: ACC-056
says "caller", where an onward request's caller is not its sending instance; the build follows the
coordinator's reading, the sending instance.

The first verifier found five musts: POOL-003 and replacement faults such as PLC-018 had no cause,
so replacing a live Pool entry gave a core error; tier 2's `exists` read an ID as a Conduit name; a
replacement refused for the named Pool's authority, and a reconciliation through an owning Pool or
Fleet, were described as needing management; and trial-based registration and installation
records took their rule from the serialized diagnostics. Round 2 fixed the five, with POOL-003's
cause and the deciding rule carried apart from the diagnostics, which also changed `records.rs`
(reported under the scope's allowance), and eight new tests. The second verifier found three musts
and a should: tier 2 read a bare node name only as an ID; setup and retirement under a retired
owner were described as needing management, a fallback ACC-033 gives inspection alone; install.v1's
generic PKG-024, PKG-025 and PKG-029 diagnostics took install-archive's subject; and the
registration precedence test never made ACC-057 and ACC-033 both refuse. Round 3 fixed the four,
with three new tests and two strengthened. The third verifier found two musts, which the run left
open: R1 and R2 below.

A fresh read-only Opus reader then verified the order (ruling 79, point 2), on a scratch clone of
the branch before its rebase. It ran every check of "Done when": each Python check ended
`Errors: 0`, the tool tests ran 123, OK, and `vector_delta.py` ended
`195 changes; 0 flips; 0 undeclared; 0 errors.` against both the base and `origin/main`; the count
script printed `0 0 0 0 42` and the greps `1 1 1 1` and `0 0 0 0`; fmt and clippy were clean, the
pieces tool printed `Nothing to move.`, the `explain-` run passed 3 cases and
`access-close-root-outside-bound` passed (2 steps; ACC-029, ACC-056); the vectors passed 2, the
full run 285 passed, 0 failed and 4 ignored, and the spike 9. Both runs that must fail failed with
the stated lines and were restored by `cp` and `cmp`. Decision 8's script, rerun on the base,
matched the branch byte for byte in the 34 cases but the listed hand edit. A probe test showed R1.
It found one must and one should:

- R1 (must): `explain::access_route` took the operation from the first scope, in the environment's
  key order, that held an Operation of the tool's name, so where a package's operation shares an
  export name with the core's, which `spec/interface/implementation.yaml` permits, a refusal named
  the core's operation; `access::decide` always recomputed it that way, even after selection.
- R2 (should): a held cascade member's access-decision and request-ended records held `reason`
  without `ruleId`, against decision 5, though `lifecycle::preflight` hands over the rule (REV-014,
  for example); the scope kept `cascade.rs` for order 046c, and no report named the conflict.

The fixer rebased the branch onto `origin/main` at `5eb71667` (#343) with the work-order identity,
without conflict, and applied each finding:

- R1: `access_route` takes the found Conduit's own listed operation whose name after `#` is the
  tool's after its first dot; otherwise it resolves `#{name}` in the target's own contract scope,
  then the core's, and never follows the environment's order. `access::decide` keeps the operation
  of the verdict's route where selection found one. The test
  `operation_identification_names_the_package_operation_on_a_name_collision` gives a package scope
  `example.org/p` a `list.v1` beside the core's, and checks that `access_route`, an
  `access::decide` refused after selection (ACC-032) and an invocation refused before it (ACC-001,
  a closed Conduit listing nothing) each name `example.org/p#list.v1`. With the old order restored
  on a scratch copy, it failed, naming `test.core#list.v1`.
- R2: of the reader's two ways, the fixer took the amendment, for the coordinator to confirm at the
  merge: Scope now allows one metadata change in `cascade.rs`, `"ruleId":rule` in the data of both
  records, and its test. The test `held_member_records_carry_the_rule` holds a member under REV-014
  and checks that both records carry `ruleId` REV-014 and `dispatched` false, that the decision is
  `hold-cascade-member` citing REV-014 and LIF-021, and that no child invocation is made. Without
  the change it failed. The cascade's outcome sites, order 046c's, are unchanged.

The fixer then ran, on the files of this commit before it was made: every Python check of "Done
when" (`Errors: 0` each), the tool tests (123, OK), `git diff --check origin/main...HEAD` and
`git diff --check` (clean), `vector_delta.py HEAD` (`0 changes; 0 flips; 0 undeclared; 0 errors.`)
and `vector_delta.py origin/main` (`195 changes; 0 flips; 0 undeclared; 0 errors.`), the count
script (`0 0 0 0 42`), the greps (`1 1 1 1` and `0 0 0 0`), and the empty `git status` of the
protected paths. From `engine/`: `cargo fmt --check` and clippy (clean), pieces
(`Nothing to move.`), the `explain-` run (3 cases passed), `access-close-root-outside-bound`
(`PASS access-close-root-outside-bound (2 steps; ACC-029, ACC-056)`), the vectors (2 passed) and
the full run (287 passed, 0 failed, 4 ignored). Both runs that must fail failed with the stated
lines, and after `cp` both `cmp` exited 0 and both cases passed; the spike passed 9. The gate's run
on this order's squash, which the merge waits on, is the coordinator's.

A second fresh Opus reader then verified the branch at the fixer's commit, on a scratch clone. It
reran every check of "Done when", with the same results as the fixer's above: each Python check
`Errors: 0`, the tool tests 123, OK, `git diff --check` clean, `vector_delta.py` 0 changes against
HEAD and 195 against `origin/main`, the count script `0 0 0 0 42`, the greps `1 1 1 1` and
`0 0 0 0`, fmt and clippy clean, pieces `Nothing to move.`, the `explain-` run 3 cases passed,
`access-close-root-outside-bound` passed, the vectors 2 passed, the full run 287 passed, 0 failed
and 4 ignored, both runs that must fail failing with the stated lines and restored by `cp` and
`cmp`, and the spike 9. It found R2's fix correct and in scope. It found one must and one should:

- R3 (must): R1's fix narrowed the lookup too far. After the Conduit's listed operations it tried
  only `#{name}` in the target's contract scope, then the core, but a Type may offer a Capability
  whose operations live in a pinned dependency package, as the in-tree
  `scenario-s2-linear-adapter` does. Such an operation, not listed by the Conduit, was then named
  by the tool's spelling, not its qualified name, against decision 2. A probe test showed it: the
  old code passed and the new failed.
- R4 (should): the summary above said 39 `REASON_CAUSES` entries, round 1's count; the table holds
  40, with round 2's POOL-003.

The fixer applied each finding, then rebased the branch onto `origin/main` at `93415960` (#344,
which adds only order 200's file) with the work-order identity, without conflict:

- R3: between the Conduit's listed operations and the `#{name}` lookup, `access_route` now
  resolves the Conduit's own Capability through the target, and of its operations whose name after
  `#` is the tool's, qualifies each by the Capability's scope's address where its address is
  empty, and takes the first that resolves through the target to an Operation; it still never
  follows the environment's order. The test
  `operation_identification_names_a_dependency_operation_the_conduit_does_not_list` builds a scope
  `example.org/adapter` holding a Capability `cap.v1` of `example.org/dep#fetch.v1`, pinned to a
  scope `example.org/dep` that exports `fetch.v1`, and a closed Conduit with that Capability that
  lists no operation; `access_route` names `example.org/dep#fetch.v1`. With the new lookup disabled
  on a scratch copy, it failed, naming `files.fetch.v1`; restored, `cmp` exited 0. The collision
  test still passes.
- R4: the summary now says 40 entries and names POOL-003 from round 2.

The fixer then ran, on the files of this commit before the rebase: every Python check of "Done
when" (`Errors: 0` each), the tool tests (123, OK), `git diff --check origin/main...HEAD` and
`git diff --check` (clean), `vector_delta.py HEAD` (`0 changes; 0 flips; 0 undeclared; 0 errors.`)
and `vector_delta.py origin/main` (`195 changes; 0 flips; 0 undeclared; 0 errors.`), the count
script (`0 0 0 0 42`), the greps (`1 1 1 1` and `0 0 0 0`), and the empty `git status` of the
protected paths. From `engine/`: `cargo fmt --check` and clippy (clean), pieces
(`Nothing to move.`), the `explain-` run (3 cases passed), `access-close-root-outside-bound`
(`PASS access-close-root-outside-bound (2 steps; ACC-029, ACC-056)`), the vectors (2 passed) and
the full run (288 passed, 0 failed, 4 ignored, one more than before for R3's test). Both runs that
must fail failed with the stated lines, and after `cp` both `cmp` exited 0 and both cases passed;
the spike passed 9. After the rebase, `check_docs.py` and `check_private_paths.py` ended
`Errors: 0`, `git diff --check origin/main...HEAD` was clean, and `vector_delta.py origin/main`
ended `195 changes; 0 flips; 0 undeclared; 0 errors.` The gate's run on this order's squash, which
the merge waits on, is the coordinator's.

At the merge the coordinator confirmed the fixer's choice of the first reader's option (a): the one
metadata-only change in `cascade.rs` that gives a held member's access-decision and request-ended
records their `ruleId`, so that decision 5 holds for every record that carries a reason. The gate
runs on the squash, and the merge waits on its PASS.
