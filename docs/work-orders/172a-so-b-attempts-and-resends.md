# Work order 172a: SO-B, part one: a new attempt's failed, deduplication by invocation, and resends

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share: list in `vectors` every case you add or change, with the promises it
tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it; use it for nothing else, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, case, fixture or log:
use `head`, `tail`, `grep`, `sed -n` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

The full review of 2026-11-14 planned SO-B, order 21 of
[its orders](../proposals/full-review/orders.md), as one of "the two specification orders the
build must not overtake".
[Ruling 78](../proposals/ruling-78/README.md#the-decisions) changed it. Point 6 (the owner chose C
to question 6) dropped the fence and the record "did not occur". So SO-B's narrowing of the
fence's evidence ([FR-022](../proposals/full-review/findings/L03-operations.md#fr-022) =
[FR-180](../proposals/full-review/findings/X03-model-checking.md#fr-180),
[FR-217](../proposals/full-review/findings/X07-retries-fence.md#fr-217) and
[FR-218](../proposals/full-review/findings/X07-retries-fence.md#fr-218), the coordinator's F12) is
withdrawn, as orders.md says it would be under B or C. Taking the fence out of `spec/` is SO-E's
(review order 52), and so is [FR-548](../proposals/full-review/findings/GL2-vectors.md#fr-548), a
vector that records "did not occur" after a late success. What SO-B keeps is split in two, so that
each order holds one concern and converges in two rounds: this order, on attempts and on what is
sent again, and order 172b, on what a reconciliation of a setup or an open, and a stop, confirm.

This order closes three unsafe faults.

1. **A later attempt's failed while an earlier attempt may still land**
   ([FR-181](../proposals/full-review/findings/X03-model-checking.md#fr-181) =
   [FR-220](../proposals/full-review/findings/X07-retries-fence.md#fr-220)). OPS-012 dispatches a
   new attempt for a repeat of an unknown invocation where the operation declares
   `deduplicates: true` and the implementation is not plain, and nothing then keeps failed from
   standing. With the example as FR-220's verifier corrected it: attempt 1 is delayed and the
   engine stops waiting, so the outcome is unknown; the caller repeats and attempt 2 is dispatched;
   attempt 2 arrives first and is answered failed, truthfully, because a precondition is not met
   yet; attempt 1 then lands, the precondition now met, and applies the effect. The engine reports
   failed for an invocation whose effect occurred, against OPS-003, and a caller that trusts it acts
   again under a new key. Order 160 reran MC2 under ruling 78
   ([experiment 012](../../experiments/012-model-checking/README.md), the section "MC2 rerun under
   ruling 78: model and pre-registration" and its dated results of 2026-11-15): the variant
   `DedupFailedSettles`, which reads OPS-012 as written, refutes W in its reordering part in 8
   actions, which is gap MC2-004; `Deduplicating` and `DiscardLate`, which apply F13, hold O and W
   in every part. Each of MC2's parts allows one fault. Ruling 78 point 6 rules F13: "Where a
   later attempt is answered failed while an earlier attempt of the same invocation has no answer,
   the invocation stays unknown, and no attempt number is sent." A run in which an earlier
   attempt's late answer, stating succeeded, reaches the engine before the new attempt's failed
   answer needs two reorders and is outside MC2's bounds, and F13 as ruling 78 words it leaves
   failed there. [Ruling 80](../proposals/ruling-80/README.md) rules that run: failed stands only
   where no earlier attempt's answer said succeeded; otherwise the invocation stays unknown, and a
   late answer still changes no outcome. Ruling 80 also rules that F13 does not apply to a call
   that changes nothing, the class that ruling 78 point 7 names, so that such a call is never
   unknown; the order that writes point 7, SO-E (review order 52), adds that exception. This order
   writes F13 so that the exception has its place (decision 1). Ruling 80's own commit (#318)
   already states the exception in [IN_FORCE.md](../../IN_FORCE.md) as spec pending for SO-E,
   under 78 point 7's line, and holds F13, as ruling 80 refines it, as spec pending in topic 7;
   this order turns the second into a pointer to `spec/` and keeps the first spec pending
   (decision 6). orders.md had SO-B take FR-220 as the coordinator's under ruling 71 point 4; it
   is now the owner's, and only how this order writes it is the coordinator's. Since
   [order 162](162-dispatch-deadline-cancels.md) the shell sends `notifications/cancelled` when a
   call passes its deadline, before it reports the call missing; the protocol lets a server ignore
   that, so such an attempt still has no answer.
2. **An implementation told to deduplicate by the key**
   ([FR-221](../proposals/full-review/findings/X07-retries-fence.md#fr-221) =
   [FR-253](../proposals/full-review/findings/X10-queued-and-kept.md#fr-253)). CTR-002
   (`spec/promises/contracts.yaml:43-44`) and the `deduplicates` row of `spec/definitions.md`
   (line 217) say that a deduplicating implementation "recognizes an idempotency key that it has
   already been sent". OPS-008 scopes a key to the caller, the target and the sending instance, and
   IFC-006 sends no sending instance. So an Agent's own request and a Resource's onward request for
   it, or two callers that chose one key string, reach one implementation with one key, and an
   implementation that follows CTR-002 loses one effect and answers it succeeded, perhaps with the
   other's outputs. Only the engine's ID for the invocation, which every attempt carries in
   `io.github.shrav-k.conduit/invocation`, matches OPS-008's scope; the package of
   [order 143](143-s4-srv1-effect.md) already keys on it. This is the coordinator's fix F14 in
   [the decisions](../proposals/full-review/decisions.md#operations-outcomes-and-reconciliation)
   (rulings 37 and 46 point 4), logged for the owner to strike.
3. **A resend after a revision error**
   ([FR-033](../proposals/full-review/findings/L04-interface.md#fr-033)). IFC-046 sends a call once
   more after the protocol's error for an unsupported revision whatever the operation
   (`spec/interface/implementation.yaml:1284-1286`), though nothing in `spec/` says that error
   proves the server has not acted, and after -32020 the same promise sends again only a call that
   changes nothing. IFC-003's observable (lines 79 to 81) and IMP-007's ("A call that the
   implementation rejected before acting on it", `spec/conventions/implicit.yaml:71-72`) repeat the
   claim. The coordinator's fix F46
   ([the decisions](../proposals/full-review/decisions.md#the-interface)): after either error only
   a call that changes nothing is sent again, and any other ends unknown, as decision 8a settled
   for -32020. "A call that changes nothing" is the class that ruling 78 point 7 names: an
   operation whose contract states no `effect`, that is not a control and that starts no model
   work. The rest of point 7, what such a call ends with when no answer comes (OPS-007, IFC-010,
   IFC-016, OPS-027 and OPS-034's first case), is SO-E's. A revision error is an answer, so point 7
   does not change what it ends with. The first build's core sends a call again only after -32020,
   and only for that class (`engine/crates/core/src/requests.rs:1523-1527`;
   `engine/CONFORMANCE.md`, row "DL2: attempts and late results": "no general effectful resend"),
   so this fix changes no case. It adds none either: `spec/` names no code for the revision error,
   so a case could not tell it from any other JSON-RPC error.

The cases follow [rule 8 of "Writing promises"](../../CONTRIBUTING.md#writing-promises), which
[order 163](163-a24-at-entry.md) put in force. A change that adds no promise ID is exempt, but each
case this order adds can fail, and the two the first build does not pass yet are listed in
[conformance/expected-failures.yaml](../../conformance/README.md#expected-failures). The engine
follows in review order 22, which removes those entries. One of the two tests ruling 80's clause
directly. When a late answer reaches it, the first build marks that attempt as answered
(`fence_attempt` in `engine/crates/core/src/requests.rs`) and keeps nothing of what the answer
stated, so an engine that read "has an answer" off that mark alone would leave failed standing in
the very run that ruling 80 rules. The case keeps review order 22 from doing so.

**Who decided what.** F13 is the owner's (78 point 6), and so is what ruling 80 adds to it: its
clause for an earlier attempt answered succeeded, and its exception for a call that changes
nothing, which the order that writes 78 point 7 adds. F14 and F46 are the coordinator's, logged
for the owner to strike. Where a decision below marks a reading as the coordinator's, apply it as
written and report it under `decisions` as the coordinator's; you write no record.

**What it does not decide.** This order changes nothing on sign-in, and it does not decide whether
any package's hooks carry Conduit's keys or are plain. Decision 3 changes only how an
implementation that carries the keys recognizes a hook's new attempt; OPS-028 keeps its sentence
that a plain implementation receives no idempotency key, under IFC-007. Write nothing on sign-in,
on which keys a package's hooks carry, or on a connection over which a person completes a sign-in.

**Before you start.** Order 160 (MC2's rerun) is on `main` at `d3140289`; check that your tree
holds it: `grep -n "standing: 'gap: MC2-004'" experiments/012-model-checking/account.yaml` prints
one line. If it prints nothing, stop and report that. Ruling 80's record, which this order links,
is on `main` since #318, with the lines its commit wrote into `IN_FORCE.md`:
`ls docs/proposals/ruling-80/README.md` lists it, and this prints 1:
`grep -c 'Spec pending, ruling 80, SO-E\*: F13 does not apply to such a call' IN_FORCE.md`. If
either fails, stop and report that. Orders 171a and 171b (SO-A, review order 19) and 172b may land
before this one, and none of them changes a promise this order changes. 171a changes other text
of `spec/conventions/operations.yaml`, `spec/interface/implementation.yaml`,
`spec/definitions.md` and `spec/README.md`; 171b other text of
`spec/interface/implementation.yaml`, `spec/definitions.md`, `spec/README.md` and `IN_FORCE.md`,
and an entry of `conformance/conventions/requests/index.yaml`; 172b other rows of
`spec/definitions.md` and another line of `IN_FORCE.md`. Each also changes other rows of the
account, the list in `conformance/expected-failures.yaml` and `engine/STANDING.md`. Whichever
merges later rebases, keeps both sides' entries in each list in byte order of `id`, and writes
`engine/STANDING.md` again with the tool (decision 11). Where a line has moved, find it by the
words quoted.

**Size: M.** Text in five files under `spec/`, three cases copied from one, three rows of the
account, and the generated standing table.

## The decisions it applies

Line numbers are those of `main` at `d4c8cf8b`. Folded `>-` text keeps its four-space indent and
lines under 97 characters; rewrap only as far as a change needs. Everything a decision does not
name stays word for word ("Writing promises", rules 1 and 7).

1. **F13 in OPS-012** (78 point 6), `spec/conventions/operations.yaml`, lines 273 to 291.
   - `then` keeps its two sentences (lines 281 to 283) and gains after them: "Where a new attempt
     is answered failed while an earlier attempt of the invocation has no answer or was answered
     succeeded, the invocation's outcome is unknown, not failed."
   - `observable` keeps its text (lines 285 to 290) and gains at its end:

     > An attempt has an answer once a response that ends its call has reached the engine: a
     > result, an error or a JSON-RPC error. Creating a task under IFC-016 is not that answer,
     > which is what the task ended with under IFC-010; an `input_required` result is none; and
     > an error after which the call is sent once more under IFC-046 is none until the call sent
     > again is answered. A cancellation that the engine sends, under OPS-027 or when a call
     > passes the time that DL10 allows it, is no answer. Whether an answer that reaches the
     > engine after it stopped waiting is kept is DL2: one that is not kept with what it stated
     > is none here, and a late answer changes no outcome, under OPS-013. An earlier attempt with
     > no answer may still reach the implementation and apply the effect after a new attempt was
     > answered failed, which would make failed false under OPS-003. No attempt number is sent
     > under IFC-006, so the implementation cannot tell such a late attempt from a new one: the
     > outcome is unknown instead, and a further repeat is dispatched as a new attempt under this
     > promise. An earlier attempt answered succeeded under IFC-009 has applied the effect, which
     > would make failed false as well, so the outcome is unknown there too, and not succeeded,
     > since a late answer changes no outcome. An engine that does not keep a late answer counts
     > that attempt as having none, with the same outcome.

   - *Coordinator's readings.* F13 goes in `then`, since it changes a reported outcome. "An
     answer" is the reading of what ends a call that OPS-014's observable holds today (lines 352 to
     358), without the fence, so it survives when SO-E takes the fence out. An engine that discards
     a late answer under DL2 counts that attempt as unanswered; MC2's `DiscardLate` variant checks
     that reading within its bound of one fault in each part, with the same verdicts as
     `Deduplicating`. Ruling 80's clause, an earlier attempt answered succeeded, lies outside MC2's
     bounds; under either reading of DL2 such a run ends unknown, by that clause where the late
     answer is kept and by the clause for an attempt with no answer where it is not. An answer
     kept without what it stated, as a mark that the attempt was answered, counts as none, so
     that an engine cannot keep the mark and lose ruling 80's clause; the case
     `requests-repeat-failed-earlier-succeeded` (decision 7) tests that run.
   - *Coordinator's reading: where the exception goes.* This order writes F13 in OPS-012's `then`
     and observable, in OPS-024's `then` and observable (decision 2) and in the row "attempt, new
     attempt" (decision 4), and those are the places where the order that writes 78 point 7 adds
     ruling 80's exception, so that a call that changes nothing is never unknown. Write no
     exception: `IN_FORCE.md` keeps it spec pending for SO-E (decision 6). Until that order, this
     text leaves unknown a call that changes nothing, sent again under OPS-024 after it
     succeeded, whose new attempt is answered failed, since its earlier attempt was answered
     succeeded; the exception ends that.
2. **F13 in OPS-024.** Its `then` (lines 521 and 522) keeps its two sentences and gains: "Where
   the new attempt is answered failed while an earlier attempt of the invocation has no answer or
   was answered succeeded, the invocation's outcome is unknown, not failed, under OPS-012." In its
   `observable` (lines 526 to 528), "afterwards the outcome of the new attempt." becomes
   "afterwards the outcome of the new attempt, or unknown where it is answered failed while an
   earlier attempt has no answer or was answered succeeded." The fence's sentence that opens the
   observable (lines 524 to 526) stays: it is SO-E's to remove under 78 point 6. F13 names OPS-024
   in
   [the decisions](../proposals/full-review/decisions.md#operations-outcomes-and-reconciliation),
   as FR-220 does.
3. **F14 in CTR-002, IFC-006 and OPS-028.**
   - CTR-002's `then` (`spec/promises/contracts.yaml:43-44`): "`deduplicates`, the mark of an
     operation whose implementation recognizes an idempotency key that it has already been sent and
     applies the effect at most once for it;" becomes "`deduplicates`, the mark of an operation
     whose implementation recognizes an invocation that it has already been sent, by the engine's
     ID for it, which every attempt of it carries in `io.github.shrav-k.conduit/invocation` under
     IFC-006, and applies the effect at most once for it;". The rest of CTR-002, its mention of
     handles included, does not change.
   - IFC-006's `observable` (`spec/interface/implementation.yaml:139-156`) gains, after its first
     sentence ("... and the revision as REV-001 gives it."): "Every attempt of one invocation
     carries the same `io.github.shrav-k.conduit/invocation`, and no other invocation carries it,
     so an implementation that deduplicates under CTR-002 recognizes an invocation by it.
     `io.github.shrav-k.conduit/idempotencyKey` is unique only within the scope that OPS-008 gives
     it, and the instance that sent the request is not sent, so an implementation tells one
     invocation from another by the invocation's ID and never by the idempotency key alone. No
     attempt number is sent: Conduit's keys do not tell one attempt of an invocation from
     another."
   - OPS-028's `observable` (`spec/conventions/operations.yaml:664-666`): the sentence "An
     implementation therefore recognizes the new attempt of a hook that failed, which a further
     setup or retirement dispatches under OPS-021 and OPS-024." becomes "An implementation
     recognizes the new attempt of a hook that failed, which a further setup or retirement
     dispatches under OPS-021 and OPS-024, by `io.github.shrav-k.conduit/invocation`, which every
     attempt of the hook's invocation carries under IFC-006, as CTR-002 has a deduplicating
     implementation recognize an invocation." Its sentence on an implementation marked `plain`
     (lines 670 to 672) stays.
   - The core's notes on `deduplicates` (`spec/core/README.md:224-227` and the head comment of
     `spec/core/operations.yaml`, lines 39 to 45) speak of the engine recognizing its own work,
     whose key OPS-008 already scopes, and do not change. Write names such as
     `io.github.shrav-k.conduit/invocation` exactly as IFC-006 writes them;
     `tools/set_names.py --check` checks them.
4. **The definitions**, `spec/definitions.md`.
   - The row "attempt, new attempt" (line 215): "A new attempt is a further dispatch of the same
     invocation, with the same idempotency key." becomes "A new attempt is a further dispatch of
     the same invocation, with the same idempotency key and the same invocation ID (IFC-006)." After
     the sentence that ends "and the implementation is not plain (OPS-012).", add: "Where a new
     attempt is answered failed while an earlier attempt of the invocation was answered succeeded
     or has no answer, that is, no response that ends its call has reached the engine, the
     invocation's outcome is unknown, since the earlier attempt has applied the effect or may still
     apply it (OPS-012, OPS-024)." The row's fence text stays for SO-E.
   - The row "`deduplicates`" (line 217): "is one whose implementation recognizes an idempotency
     key that it has already been sent and applies the effect at most once for it (CTR-002)."
     becomes "is one whose implementation recognizes an invocation that it has already been sent,
     by the engine's ID for it, which every attempt carries in
     `io.github.shrav-k.conduit/invocation`, and applies the effect at most once for it; it never
     uses the idempotency key alone, which is unique only within the scope that OPS-008 gives it
     (CTR-002, IFC-006)."
5. **F46 in IFC-046, IFC-003 and IMP-007.**
   - IFC-046's `then` (`spec/interface/implementation.yaml:1281-1286`) becomes: "After -32020, or
     after the protocol's error for a revision that the implementation does not support, the engine
     sends the call once more only for an operation whose contract states no `effect`, that is not
     a control and starts no model work; any other invocation it ends unknown, and it sends nothing
     more. After -32020 it first reads the implementation's `tools/list` again, and after an error
     for the revision it sends the call at a revision that both support, where there is one. The
     two are one call, and what it ends with is read from the response to the second."
   - IFC-046's `observable`: its first sentence (lines 1288 and 1289), "The protocol's -32020 gives
     no guarantee that a plain server has not acted, so an effectful call is never repeated after
     that code.", becomes "Neither the protocol's -32020 nor its error for an unsupported version
     guarantees that a server has not acted, so a call of an operation outside that class is never
     sent again after either." In lines 1293 and 1294, "the safety condition above governs the
     resend after -32020" becomes "the safety condition above governs the resend after both". The
     rest stays, the sentence on no common revision included.
   - IFC-003's `observable` (lines 79 to 81) becomes: "A request for an operation whose contract
     states no `effect`, that is not a control and starts no model work, and that the
     implementation rejects for its revision with the protocol's JSON-RPC error for an unsupported
     version, is sent again at a revision both support, under IFC-046, and is one invocation, not
     two. Any other request so rejected ends unknown, under IFC-046."
   - IMP-007's `observable` (`spec/conventions/implicit.yaml:71-72`): "A call that the
     implementation rejected before acting on it, and that the engine sends once more under
     IFC-046, is part of the same dispatch, not a second." becomes "A call that the engine sends
     once more under IFC-046, which it does only for an operation whose contract states no
     `effect`, that is not a control and starts no model work, is part of the same dispatch, not a
     second."
   - IFC-001 (its last sentence, on -32020, lines 49 to 51), IFC-010 (lines 229 and 230, 248 and
     249) and OPS-007 (`spec/conventions/operations.yaml:115-116`, 137 and 138) do not change: each
     sends the call again only "under IFC-046", which IFC-046 now limits, and ruling 78 point 7's
     changes to IFC-010 and OPS-007 are SO-E's.
6. **IN_FORCE.md and the safety traces.**
   - `IN_FORCE.md`, topic 7. Ruling 80's commit rewrote the F13 bullet, now line 161, under the
     group that begins "*Spec pending, rulings 78 and 80*" (line 156), and added a bullet for SO-E
     under 78 point 7's line, now line 163; find each by the words quoted. The bullet that begins
     "Where a later attempt is answered failed, failed stands only where" becomes:
     `  - **In spec/**: where a later attempt is answered failed, failed stands only where every earlier attempt of the same invocation has an answer and none of those answers said succeeded; otherwise the invocation stays unknown, and a late answer still changes no outcome. No attempt number is sent: OPS-012, OPS-024, IFC-006, OPS-013 (78 point 6, F13, as 80 point 2 refines it). A call that changes nothing is exempt, spec pending below (80 point 3).`
     It stays where it is, an In-spec line in the spec-pending group, as topics 4 to 6 nest
     theirs. In the bullet that begins "*Spec pending, ruling 80, SO-E*: F13 does not apply to
     such a call", "adds this exception to F13 (80 point 3)." becomes "adds this exception to F13;
     OPS-012 and OPS-024 say otherwise today (80 point 3)." The rest of it stays, and it stays
     spec pending: the exception is SO-E's to write. The group's first line stays spec pending:
     its other points are not written yet. Put no date in either line; `check_rulings.py` refuses
     one.
   - `conformance/s1-safety-cases.md`, lines 72 to 74: "IFC-046's wire resend is within one
     attempt: after -32020 only that same class of operation is sent again, and a revision error
     allows a resend only at a revision both sides support." becomes "IFC-046's wire resend is
     within one attempt: after -32020 or a revision error only that same class of operation is sent
     again, after a revision error at a revision both sides support." Rewrap within the paragraph's
     width.
   - `spec/README.md`'s figures of coverage by vectors are those recorded on 2026-11-10, as its
     Contents says, and do not change, as orders 171b and 172b leave theirs.
7. **Three cases**, each copied from `conformance/conventions/requests/aware-unknown/` (`case.yaml`
   and `requests.json`): an aware implementation, a write whose contract the fixture marks
   `deduplicates: true`, and the key `key`. Make the copies with `cp` and the edits with a script
   you run and do not commit. Steps are zero-based here, as JSON Pointers; the runner and
   `tools/vector_delta.py` print step N + 1. Each new step's evidence holds fresh IDs that the case
   uses nowhere else. The outcomes, the dispatches and the records follow `spec/`; for the steps
   whose behavior this order does not change, the first build's output may settle what `spec/`
   leaves open, such as which promises a decision cites.
   - **`requests-repeat-failed-earlier-unanswered`**, in
     `conformance/conventions/requests/repeat-failed-earlier-unanswered/`, with the title "A new
     attempt answered failed while an earlier attempt has no answer leaves the invocation unknown"
     and the promises `OPS-012` and `IFC-006`.
     - `/steps/2`: attempt 1 completes with `{"kind": "missing"}`, so it has no answer, and the
       outcome is unknown under OPS-007. Its `expect` does not change: what aware-unknown's step 2
       expects for a result that states unknown is what `requests-unknown-write`'s step 2 expects
       for a missing completion, apart from each case's own register invocation.
     - `/steps/3`: unchanged, except that the expected dispatch's `call.params` also holds
       `"_meta": {"io.github.shrav-k.conduit/invocation": "$i", "io.github.shrav-k.conduit/idempotencyKey": "key"}`.
       A map is a projection, so this asserts that attempt 2 carries attempt 1's invocation and key
       (IFC-006; F14).
     - `/steps/4`, new: `complete` for `$i`, attempt 2, with the completion that step 2 of
       `requests-aware-failed` sends (an aware result that states failed). Expect `reply._meta` to
       hold `io.github.shrav-k.conduit/outcome` `unknown` and `io.github.shrav-k.conduit/invocation`
       `$i`, and `reply.isError` true, with no `content` in the projection, whose wording this
       order does not decide; `state.invocations` as this case's own `/steps/2` lists them, the
       register invocation's ID included, with `$i` `unknown` at attempt 2 (each case has its own
       register ID, and `requests-aware-failed`'s is not this case's); empty `external_intents`;
       the two records, an invocation's end and a request's end, that that step expects, with
       this step's IDs; its `absent` list; and `required_promises` `["OPS-012"]`.
   - **`requests-repeat-failed-earlier-succeeded`**, in
     `conformance/conventions/requests/repeat-failed-earlier-succeeded/`, with the title "A new
     attempt answered failed after an earlier attempt's late answer said succeeded leaves the
     invocation unknown" and the promise `OPS-012`. Copy `requests-repeat-failed-earlier-unanswered`
     once it is written: its steps 0 to 3 as they are, and its `/steps/4` as `/steps/5`. Between
     them, a new `/steps/4`: `complete` for `$i`, attempt 1, with an aware result that states
     succeeded,
     `{"content": [], "isError": false, "_meta": {"io.github.shrav-k.conduit/outcome": "succeeded"}}`,
     so that attempt 1's late answer says succeeded while attempt 2 is in progress. Expect the
     reply `{"ignored": true}`, as step 4 of `requests-reconcile-fenced` expects for a late
     answer; empty `external_intents` and `records`; `required_promises` `["OPS-013"]`; and
     `state.invocations` as this case's `/steps/2` lists them, with `$i` `in-progress` at
     attempt 2. `/steps/5` expects the outcome unknown, as the unanswered case's `/steps/4` does:
     by ruling 80's clause where the engine keeps that late answer with what it stated, and by
     the clause for an attempt with no answer where it does not (decision 1).
   - **`requests-repeat-failed-earlier-answered`**, in
     `conformance/conventions/requests/repeat-failed-earlier-answered/`, with the title "A new
     attempt answered failed after every earlier attempt was answered, none succeeded, ends
     failed" and the promise `OPS-012`. Steps 0 to 3 are aware-unknown's: attempt 1 is answered,
     with a result that states unknown. `/steps/4`, new: the same completion of attempt 2,
     expecting the outcome `failed`, `$i` `failed` at attempt 2, and the rest as step 2 of
     `requests-aware-failed` expects it, except that `state.invocations` lists this case's own
     register invocation, as its `/steps/2` does, and the records take this step's IDs. It shows
     that F13 leaves alone a failed that every earlier answer allows; the first build passes it.
   - Add the three entries to `conformance/conventions/requests/index.yaml`, in byte order of
     `id`, each with its `case` path written as the other entries write theirs, the same
     `promises` as its case, and `"safety_rows": []`. The manifest and the case schema do not
     change.
8. **The expected failures.** Add to `conformance/expected-failures.yaml`, in byte order of `id`
   and in the form "Expected failures" in `conformance/README.md` gives, two entries:
   - `id` `requests-repeat-failed-earlier-succeeded`, `reason` "OPS-012, F13 of ruling 78 point 6
     as ruling 80 point 2 refines it: the first build records failed when a new attempt is
     answered failed after an earlier attempt's late answer said succeeded; review order 22, the
     engine following SO-B, makes it pass and removes this entry";
   - `id` `requests-repeat-failed-earlier-unanswered`, `reason` "OPS-012, F13 of ruling 78 point
     6: the first build records failed when a new attempt is answered failed while an earlier
     attempt has no answer; review order 22, the engine following SO-B, makes it pass and removes
     this entry".
9. **The account**,
   [the model-checking account](../../experiments/012-model-checking/account.yaml).
   `tools/check_model_account.py` requires that a row with a model-run standing whose promise's
   text changed become `text changed, not rerun`, with `was` set to the standing it had and
   `model`, `read` and `digest` kept as they are ([order 153](153-model-account-checked.md),
   decision 7, as [order 160](160-mc2-rerun-077.md) applies it). Three rows change, each with one
   sentence appended to its `note`, and no other:
   - OPS-012, from `gap: MC2-004`: "Work order 172a wrote into OPS-012's then F13 (ruling 78 point
     6), the rule under which work order 160's Deduplicating and DiscardLate variants hold O and W
     while DedupFailedSettles reads the text before it, and ruling 80's clause for an earlier
     attempt answered succeeded, which needs two reorders and so lies outside MC2's bounds; the
     row waits for a rerun that reads this text."
   - OPS-028, from `contradiction: MC2-001`: "Work order 172a changed its observable: an
     implementation recognizes a hook's new attempt by the invocation's ID (F14)."
   - IMP-007, from `searched`: "Work order 172a changed its observable: only a call that changes
     nothing is sent once more under IFC-046 (F46)."

   OPS-024 is `text changed, not rerun` already and keeps its row. CTR-002, IFC-003, IFC-006 and
   IFC-046 are not convention-layer promises and have no row. Keep every other byte of the file.
10. **The declaration**, `docs/work-orders/172a-declared.yaml`, in the form of
    `docs/work-orders/081-declared.yaml`: one `added` declaration for each of the three cases, and
    nothing else.
11. **The standing table**, `engine/STANDING.md`, which `tools/promise_standing.py` generates
    ([order 185](185-promise-standing-generated.md)) from the coverage index and the expected
    failures among its inputs, so decisions 7 and 8 change it. Once they are done, run
    `python3 tools/promise_standing.py --write`; never edit the file by hand. From its state on
    your starting tree, these rows change: IFC-006 (Cases 0 to 1, Expected failures 0 to 1,
    Standing `no case, no reason` to `expected failure`) and OPS-012 (Cases up 3, Expected
    failures up 2), and, in Totals, `expected failure` up 1 and `no case, no reason` down 1, each
    in the columns interface and all. On `main` at `d4c8cf8b` that is OPS-012 21 to 24,
    `expected failure` 0 to 1, and `no case, no reason` 24 to 23 and 153 to 152; orders 171a,
    171b and 172b, where they landed first, move the column all as well. Report any other change
    with its rows.

## Scope

Change only `spec/conventions/operations.yaml` (OPS-012, OPS-024, OPS-028),
`spec/conventions/implicit.yaml` (IMP-007), `spec/interface/implementation.yaml` (IFC-003,
IFC-006, IFC-046), `spec/promises/contracts.yaml` (CTR-002), `spec/definitions.md` (two rows),
`IN_FORCE.md` (two bullets of topic 7),
`conformance/s1-safety-cases.md` (lines 72 to 74), `conformance/conventions/requests/index.yaml`,
`conformance/expected-failures.yaml`, `experiments/012-model-checking/account.yaml` and
`engine/STANDING.md`, written only by the tool (decision 11), and create
`conformance/conventions/requests/repeat-failed-earlier-unanswered/`,
`conformance/conventions/requests/repeat-failed-earlier-succeeded/`,
`conformance/conventions/requests/repeat-failed-earlier-answered/` and
`docs/work-orders/172a-declared.yaml`. Not `engine/` apart from `engine/STANDING.md`, the
experiment's code or README, `tools/`, `conformance/manifest.yaml` or the case schema.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_roadmap.py
python3 tools/check_private_paths.py
python3 tools/check_model_account.py
/usr/bin/python3 tools/check_model_account.py --history
python3 tools/promise_standing.py --check
python3 tools/vector_delta.py HEAD --declared docs/work-orders/172a-declared.yaml
git diff --check
```

`vector_delta.py` reads the declaration with PyYAML, so where `python3` lacks it, run it with
`/usr/bin/python3`; it must end `Vector delta: 3 changes; 0 flips; 0 undeclared; 0 errors.`
`promise_standing.py --check` ends `Errors: 0`; with decisions 7 and 8 done and before decision
11's `--write`, it fails, naming `engine/STANDING.md`. `check_model_account.py` ends `Errors: 0`,
and from the counts it printed on your starting tree: `searched` falls by 1,
`contradiction: MC2-001` by 1, `gap: MC2-004` from 1 to no line, and `text changed, not rerun`
rises by 3; under "Text changed, not rerun by was", `searched`, `contradiction: MC2-001` and
`gap: MC2-004` each rise by 1. Report any other count with its rows.

From `engine/`, one at a time:

```bash
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. requests-repeat-failed-
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

The pieces tool prints `Nothing to move.` The runner command prints, in this order, `PASS` for
`requests-repeat-failed-earlier-answered`;
`EXPECTED FAILURE requests-repeat-failed-earlier-succeeded (OPS-012): ` followed by its reason;
and `EXPECTED FAILURE requests-repeat-failed-earlier-unanswered (OPS-012, IFC-006): ` followed by
its reason. It ends `1 cases passed; 2 expected failures`. Each expected failure's error must be at
the case's last step, the completion of attempt 2 (step 6 of the succeeded case and step 5 of the
unanswered case, as the runner counts), and on its outcome: failed where the case expects unknown,
or OPS-012 not cited. The runner checks a step's cited promises before the rest of it, and its
message for one not cited, `uncited expected promise "OPS-012"; reply: ...`, names no step: the
reply it shows states the outcome failed, which no earlier step of either case replies (the
succeeded case's step 5 replies `{"ignored": true}`), and so marks the last step. An error at an
earlier step means the case is wrong, since those steps test what does not change: fix the case,
never the list. Report each error's first line. The threads are 4 because at 24 the runner ran out
of memory (order 071's Record).

From `experiments/012-model-checking/`, one at a time:

```bash
CARGO_BUILD_JOBS=3 cargo test --release operations::tests::rerun_matches_registration
CARGO_BUILD_JOBS=3 cargo test --release
```

Both pass, with nothing in the experiment's code changed: MC2 reproduces its registered table, in
which `Deduplicating` and `DiscardLate` apply F13 as ruling 78 words it, which within MC2's bounds
is OPS-012 as this order writes it, and the text before it is `DedupFailedSettles`. In your report,
compare OPS-012's new `then` and observable with the rule that the README's pre-registration of the
rerun gives ("F13 leaves unknown when a failed current attempt has an earlier unanswered attempt;
otherwise it records failed"). That rule has no clause for an earlier attempt answered succeeded,
which ruling 80 adds and MC2's bounds do not reach; quote that difference and any other.

Each of these prints 0. The `tr` checks here and below read the file with its line breaks as
spaces, so a rewrap hides nothing:

```bash
tr -s ' \n' '  ' < spec/promises/contracts.yaml | grep -c 'recognizes an idempotency key'
tr -s ' \n' '  ' < spec/definitions.md | grep -c 'recognizes an idempotency key'
tr -s ' \n' '  ' < spec/conventions/implicit.yaml | grep -c 'rejected before acting on it'
tr -s ' \n' '  ' < conformance/s1-safety-cases.md | grep -c 'allows a resend only at a revision'
grep -c 'Order 172a writes this into OPS-012 and OPS-024' IN_FORCE.md
```

Each of these prints 1:

```bash
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c 'governs the resend after both'
tr -s ' \n' '  ' < spec/interface/implementation.yaml | grep -c "No attempt number is sent: no key of Conduit's says which attempt"
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -c 'No attempt number is sent under IFC-006'
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -c 'one that is not kept with what it stated is none here'
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -c 'or unknown where it is answered failed while an earlier attempt has no answer'
grep -c '^  - \*\*In spec/\*\*: where a later attempt is answered failed, failed stands only where' IN_FORCE.md
grep -c 'A call that changes nothing is exempt, spec pending below (80 point 3)' IN_FORCE.md
grep -c 'adds this exception to F13; OPS-012 and OPS-024 say otherwise today' IN_FORCE.md
```

This prints 2, one for OPS-012's `then` and one for OPS-024's:

```bash
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -o 'has no answer or was answered succeeded, the invocation' | grep -c .
```

`git status --short` lists only the files in scope. Do not stage anything. Report each choice you
made yourself and every check's last line. `git diff --check` and `check_private_paths.py` read
tracked files only, so the admin then stages the new files and runs
`python3 tools/check_private_paths.py` and `git diff --cached --check`. Before the merge the admin
runs the local gate that [ruling 80](../proposals/ruling-80/README.md), point 10, has each merge
wait on, the engine's tests, its doctests and the experiment's tests among what it runs; CI runs
on the pull request as well, and the merge does not wait for it. A fresh Codex verifier and a
fresh read-only Opus reader read the result before it merges
([ruling 79](../proposals/ruling-79/README.md), point 2).

## Record

Run on 2026-11-16, from about 20:39 to 21:04 UTC, on the Mac from `main` `55d2b85a`: the worker
(1,253 s) and a fresh Codex verifier (233 s), which found no must or should in one round. The
verifier reran the repository's checks, and with binaries already built the three cases and MC2's
registration test, but not the full Cargo suites. OPS-012's `then` and observable now carry F13 as
ruling 80, point 2, refines it: a new attempt answered failed while an earlier attempt of the
invocation has no answer or was answered succeeded leaves the outcome unknown, not failed. The
observable says what counts as an answer, that one not kept with what it stated counts as none, and
that no attempt number is sent. OPS-024's `then` and observable say the same. CTR-002, IFC-006,
OPS-028 and the row `deduplicates` have a deduplicating implementation recognize an invocation by
`io.github.shrav-k.conduit/invocation`, never by the idempotency key alone (F14). IFC-046, IFC-003
and IMP-007 send a call once more after -32020 or a revision error only for a call that changes
nothing, and end any other unknown (F46). In `IN_FORCE.md`, topic 7's F13 bullet is now an In-spec
line, and ruling 80, point 3's exception stays spec pending for SO-E;
`conformance/s1-safety-cases.md` follows F46. Of the three cases copied from
`requests-aware-unknown`, `requests-repeat-failed-earlier-answered` passes, and
`requests-repeat-failed-earlier-succeeded` and `requests-repeat-failed-earlier-unanswered` are
expected failures until review order 22. Each fails at attempt 2's completion, step 6 and step 5 as
the runner counts, with `uncited expected promise "OPS-012"` on a reply that states failed. The
account's rows OPS-012, OPS-028 and IMP-007 became `text changed, not rerun`: `searched` 7 to 6,
`contradiction: MC2-001` 2 to 1, `gap: MC2-004` gone, and `text changed, not rerun` 28 to 31, with
`model`, `read` and `digest` kept. Against the README's pre-registered rule for the rerun, the
worker reported that OPS-012 adds "or was answered succeeded", ruling 80's run, which needs two
reorders and lies outside MC2's bounds. Its observable also says what is an answer, which answers
count as none, and that no attempt number is sent; the outcome rule differs in nothing else.

A fresh read-only Opus reader then verified the order at `ab0b8d1f` (ruling 79, point 2). It found
with `git merge-tree` that a rebase onto `main` `ecabe372` does not conflict, and that on the
merged tree the Python checks end `Errors: 0` and `engine/STANDING.md` needs no regeneration. Under
`env -i` it ran `check_docs`, `check_rulings`, `set_names.py --check`, `check_schemas`,
`check_roadmap`, `check_model_account` (with and without `--history`), `promise_standing.py
--check` and `check_private_paths` (each `Errors: 0`), `vector_delta.py origin/main HEAD
--declared-changed` (3 changes; 0 flips; 0 undeclared; 0 errors) and `git diff --check`. In
`engine/` it ran fmt, clippy, the pieces tool, the runner (`1 cases passed; 2 expected failures`)
and the engine's tests, and in the experiment the 62 tests, which the Codex verifier had not run.
It found decisions 1 to 11 applied word for word, and the change in the 18 files of Scope. On
scratch copies, each failing case passed once cut back by its last step, so the only failure is at
attempt 2's completion, and `main`'s `engine/STANDING.md` failed `promise_standing.py --check`. It
noted that the answered case lists OPS-012 though no step requires it, as decision 7 says, and can
still fail on an engine that applied F13 too widely, and that the order settles nothing on the Open
line of `IN_FORCE.md`'s topic 9 and writes nothing on sign-in. It found no must and three shoulds,
all applied:

- The row "attempt, new attempt" in `spec/definitions.md` defined "has no answer" more narrowly
  than OPS-012's observable, which also counts as none a late answer not kept with what it stated.
  The row now reads "no response that ends its call has reached the engine and been kept with what
  it stated"; decision 4, which quotes the old words, is left as written.
- IFC-006's "Conduit's keys do not tell one attempt of an invocation from another" was false where
  attempts go through different Conduits (OPS-012) or take a new caller (OPS-028). It now reads "No
  attempt number is sent: no key of Conduit's says which attempt of an invocation a call is, or
  which of two attempts was sent first." The grep of "Done when" above follows the new words;
  decision 3, which quotes the old sentence, is left as written.
- IFC-006's only assertion lay in an expected-failure case, which the runner excuses whatever step
  fails. Step 3 of `requests-repeat-failed-earlier-answered` now asserts the same `_meta`
  projection, the invocation `$i` and the key `key`, and the case and its index entry list IFC-006.
  The first build passes the case, so in `engine/STANDING.md`, written again by the tool, IFC-006
  reads 2 cases, 1 expected failure, `case passes`. From `main`, in the columns interface and all,
  Totals move `case passes` 22 to 23 and 292 to 293 and `no case, no reason` 24 to 23 and 153 to
  152, and `expected failure` stays 0, where decision 11 had it rise by 1. Decision 7, which gives
  the case only OPS-012, is left as written.

The fixer first rebased the branch onto `main` `ecabe372`, five commits past `55d2b85a`, among them
ruling 81's, which changed other lines of `IN_FORCE.md`; there was no conflict. After the fixes,
under `env -i`, `check_docs`, `check_rulings`, `set_names.py --check`, `check_schemas`,
`check_roadmap`, `check_model_account` (with and without `--history`), `promise_standing.py
--check` and `check_private_paths` each end `Errors: 0`, and `git diff --check origin/main...HEAD`
is clean. `vector_delta.py origin/main HEAD --declared-changed`, and the form of "Done when" with
`origin/main` in place of `HEAD`, each end `Vector delta: 3 changes; 0 flips; 0 undeclared; 0
errors.` The form with `HEAD` compares `HEAD` with the working tree; with the worker's output
committed, it measured only the fix's uncommitted changes to the answered case, as 3 undeclared.
Every grep of "Done when" prints its value, and the old words of IFC-006 print 0. The model
account's counts are those above. In `engine/`, `cargo fmt --check` and `cargo clippy --all-targets
-- -D warnings` pass, and the pieces tool prints `Nothing to move.` The runner prints `PASS` for
`requests-repeat-failed-earlier-answered`, now with OPS-012 and IFC-006, then the two expected
failures, each with its first line as before, and ends `1 cases passed; 2 expected failures`. With
that case's key set to `other` and then restored, it failed with `step 4:
/external_intents/0/call/params/_meta/io.github.shrav-k.conduit/idempotencyKey: expected "other",
found "key"`. The vectors test passes (2 tests) and `CONDUIT_VECTOR_THREADS=4 cargo test --release`
passes, 242 tests in 28 suites. In the experiment, `rerun_matches_registration` and the 62 tests
pass.
