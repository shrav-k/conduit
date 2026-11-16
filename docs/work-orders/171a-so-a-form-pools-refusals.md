# Work order 171a: SO-A, part 1: what an observable binds, what a Pool in a bound covers, and which rule a refusal names

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema the orders share. List in `vectors` every case you add or change, with the promises it
tests and whether it passes. This order has network access only because the engine's tests bind
loopback ports, which the sandbox refuses without it. Use it for nothing else, and reach no other
machine. Run one cargo command at a time. Never print a whole large file, fixture or log: use
`head`, `tail`, `grep`, `sed -n`, `wc` or a count. Never run `env`, `printenv`, `set` or
`export -p`, never print an environment variable's value, and never read or print a key, a token
or any credential.

## What and why

The full review of 2026-11-14 found text in `spec/` that batch 1's orders 046a, 047 and 048a-2
would otherwise be written on. Its order 19 (SO-A in
[its orders](../proposals/full-review/orders.md)) is one of "the two specification orders the
build must not overtake", and it carries five unsafe fixes. Row 19 sizes SO-A as M: text and two
cases. With the refusal order and the copies it aligns, that is too much for one order to
converge in two rounds. So SO-A is cut in two:

- **This order** covers the form of a promise and `access`. It settles what an observable binds,
  what a Pool named in a bound covers, what a bounded caller learns, and which rule a refusal
  names.
- **Order 171b**, which follows this one, covers sign-in: a return that is kept nowhere, and the
  sign-in holds.

Both parts land before 046a, 047 and 048a-2 are written (row 19's place). Ruling 78, point 1, part
2, keeps sign-in before the pin, so neither part moves after it.

1. **Whether an observable binds**
   ([FR-140](../proposals/full-review/findings/L13-framing.md#fr-140) =
   [FR-153](../proposals/full-review/findings/X01-catalogue-layers.md#fr-153), with
   [FR-441](../proposals/full-review/findings/E05-falsifier.md#fr-441); unsafe).
   - The form in `spec/README.md` (lines 292 and 293) calls `then` "the single obligation" and
     `observable` "what can be checked". Nothing says whether an observable binds, and rules stand
     in observables alone.
   - FR-140's verifier found the unsafe case. The rule that a browser login's return is kept
     nowhere stands only in the observables of PND-013 and IFC-049. If only `then` binds, an
     engine may record an accepted authorization code.
   - Fix F122 of the coordinator's decisions
     ([F122](../proposals/full-review/decisions.md#the-form-the-layers-definitions-and-the-limits))
     says what an observable binds, and that the `then` governs where the two differ.
   - F122 names one clash that its sentence would make binding, and fixes it in the same change:
     ACC-024's observable says a protected close reports refused "under ACC-012", while LIF-022,
     LIF-034 and the build say ACC-024. FR-140's verifier, correction 3, warns that the sentence
     makes binding every observable that conflicts with another promise's `then`, and names
     INS-016's ([FR-147](../proposals/full-review/findings/L13-framing.md#fr-147)) as one.
   - This order writes F122's sentence and fixes the ACC-024 clash. F122's other half gives the
     return a promise and a case of its own. Order 171b writes it, beside the sign-in promises it
     rewrites. PND-013's rule binds through the form as soon as this order merges.
   - Two things stay with SO-H, the review's order 55, and not with this order:
     - its sweep of the observables, which settles each observable that differs from another
       promise's `then`, INS-016's among them;
     - the writing rules that ruling 78, point 17, accepted (F124 and F127).
   - [FR-549](../proposals/full-review/findings/GL2-vectors.md#fr-549) lists the vectors that rest
     on sentences found only in observables. They join that sweep, and this order changes none of
     them.
   - FR-441's verifier held that a move that makes an observable sentence binding is not the
     coordinator's to make. F122 decides it for the form. The build and the vectors already read
     the form that way. The owner accepted question 17's recommendation that the language "say
     plainly which text binds". F122 stays on the review list.
2. **A Pool named in a bound's `targets`**
   ([FR-001](../proposals/full-review/findings/L01-access.md#fr-001) =
   [FR-121](../proposals/full-review/findings/L11-core-rest.md#fr-121); unsafe; fix F1 of
   [the decisions](../proposals/full-review/decisions.md#access-and-refusals)).
   - `bound.v1`'s `targets` accepts a Pool, and ACC-029 says only what a Fleet there covers.
   - Under one reading, a Pool in `targets` covers its Resources. Under the other, ACC-030's
     `then` as written still lets a holder of `{targets: [shared]}` open a Conduit to `access`
     with `{pools: [shared]}`, which covers every Resource of `shared`. Only the invariant in
     ACC-030's observable (lines 536 and 537: "its bound covers nothing that the bound of the
     actor that opened it does not cover") forbids that.
   - The first build takes the safe reading. It checks a new bound's `pools` against the old
     bound's `pools` (`engine/crates/core/src/access.rs`: `covers` from line 88,
     `subset_in_view` from line 114).
   - F1 writes that reading down, and both verifiers list its copies. FR-001's verifier found no
     vector that tests a Resource of a Pool that a bound names only in `targets`. This order adds
     one, and the build passes it.
3. **What a bounded caller learns**
   ([FR-002](../proposals/full-review/findings/L01-access.md#fr-002),
   [FR-209](../proposals/full-review/findings/X06-authority-access.md#fr-209),
   [FR-003](../proposals/full-review/findings/L01-access.md#fr-003),
   [FR-211](../proposals/full-review/findings/X06-authority-access.md#fr-211); fix F2).
   - ACC-056 says tier 1 depends on no object's existence, owner or state. Yet it gives every
     caller the refusing rule.
   - Under OPS-029, a bounded holder that asks `open.v1` for a Conduit that does not exist is told
     invalid. Under ACC-053, a bounded holder that asks to lift is refused and "learns nothing".
   - Ruling 52, point 1 (IN_FORCE.md, line 149), says: "a target that does not exist gets the same
     answer where the caller has no authority over it".
   - F2 refuses such a request under ACC-029. It restates the tier-1 rule so that tier 1 depends
     on nothing outside the caller's bound or inspection rights. It also says who receives tier
     2, as FR-211's verifier narrowed it.
   - [Ruling 80](../proposals/ruling-80/README.md), point 1, exempts one request from F2's
     refusal. The owner chose to exempt a bounded holder's repeat of its own request, which is
     answered from its stored outcome as it is today: for example, a repeat of a close that
     succeeded and removed the Conduit it names. F2 keeps refusing every other request about a
     Conduit that does not exist.
   - A repeat that OPS-012 or OPS-024 dispatches as a new attempt is not answered from a stored
     outcome. Ruling 80 words the exemption as an answer from the stored outcome, so decision 4
     leaves such a repeat to F2, which refuses it. That reading is the coordinator's, not the
     owner's.
4. **Which rule a refusal names**
   ([FR-004](../proposals/full-review/findings/L01-access.md#fr-004),
   [FR-126](../proposals/full-review/findings/L11-core-rest.md#fr-126),
   [FR-158](../proposals/full-review/findings/X01-catalogue-layers.md#fr-158),
   [FR-210](../proposals/full-review/findings/X06-authority-access.md#fr-210),
   [FR-038](../proposals/full-review/findings/L04-interface.md#fr-038); fix F3).
   - A refusal's ruleId is observable (IFC-050) and recorded (REC-001). Yet no text orders the
     rules when more than one refuses.
   - Six copies give a closed Conduit's refusal to ACC-011. ACC-011's own observable and IFC-012
     give it to ACC-001.
   - The build checks in code order (`access.rs` from line 709). So a bounded caller that asks to
     close root's Conduit to a target outside its bound is told ACC-024, and learns that the
     Conduit is root's (FR-210's verifier).
   - F3 states one order and puts the bound before the rules that read the named object. It also
     aligns the copies, and makes a refusal's `cause` a non-empty sentence in the engine's own
     words. With one rule per refusal, DL9 no longer describes a refusal's choice (FR-126's
     verifier).
   - F3 also says that "one vector's last step then expects ACC-029". That is the last step of
     `access-bounds`, which FR-210 cites. That step does not fit F3's premise, though:
     - in its observation 16, `c610`'s bound names Pool `workspace`
       (`00000000-0000-4000-8000-000000000117`) in `pools`;
     - `c611`'s target is Resource `shared` (`…0119`), which `workspace` owns;
     - `c611` lists only `file.read.v1`, which the bound names;
     - the same case's step 2 opens a Conduit to `shared` under the same bound, and it is
       permitted.
   - So the bound covers that close, and ACC-024 decides it under F3's order too. This order
     leaves the step as it is. It writes F3's vector as a new case of FR-210's own situation:
     root's Conduit to a target outside the bound.

**Who decided what.** F1, F2, F3 and F122 are the coordinator's fixes. They are made under rulings
37 and 46 point 4, and F2 and F3 also under ruling 52. Each is logged for the owner to strike
(ruling 48), and [ruling 78](../proposals/ruling-78/README.md#the-decisions) struck none.
[Ruling 80](../proposals/ruling-80/README.md), point 1, narrows F2: a bounded holder's repeat of
its own request is answered from its stored outcome, and is not refused under ACC-029. That
exemption is the owner's, and decision 4 writes it. That a repeat dispatched as a new attempt is
not exempt is the coordinator's reading of ruling 80's words, open to the owner's strike like the
rest of F2.

F3 names its steps and a few rules in them, and FR-210 names the rest of its list. Decision 5
also places IFC-016, IMP-008, SEC-007, PND-009 and PND-010, which neither of them names. Each is
placed by what its own `then` reads. By the same rule, ACC-032's clause on opening a Conduit that
lists a reserved operation goes with the rules that read the named object, and not with its
clause on asking for a reserved operation. That placement is this order's application of F3,
and so the coordinator's. Apply it as written, and report it under `decisions` as the
coordinator's.

**What this order leaves to the review of sign-in.** A targeted review of the sign-in and home
surface that the Agent packages 0.2.0 will freeze is to follow. It owns, among others,
whether the hooks are keyed or plain, the connection Conduit manages for a Claude sign-in (ruling
80, point 8), how Codex's browser is kept closed, the device login, F95 to F97, and the reason a
second `sign-in.v1` of one instance reports (F85's conflict or PND-010's unavailable). This order
writes nothing on any of them:

- change nothing in `spec/promises/pending.yaml`, IFC-007, IFC-016, IFC-045, IFC-049, CTR-013's
  text, `spec/promises/contracts.yaml`, the core operations of sign-in or the limits "Pending
  actions, sign-in and the network" in `spec/README.md`;
- ACC-056's order (decision 5) ranks only the rules that report refused. It names PND-009 and
  PND-010 only where they refuse who may answer or sign in, and an open of a Conduit that lists
  `answer.v1` or `sign-in.v1`. It says nothing of a second sign-in's reason;
- decision 6's CTR-013 row only gives the cause of a refusal that CTR-013 states today.

**IN_FORCE.md's lines in these files.** This order writes one line of IN_FORCE.md, line 135, the
*spec pending* line of F2 that names this order (decision 15). It writes none of the other lines
that mark a change to these files *spec pending*, and each stays marked:

| IN_FORCE.md line | What it rules | Written by |
| --- | --- | --- |
| 45 | rule 1, a rule on observables and the `vectors` field (78 point 17, F124, F127) | SO-H |
| 44 | the convention layer's words (78 point 15) | SO-H |
| 21, 102 | `install.v1` for root alone: ACC-032, ACC-033, CTR-013 (78 point 10) | another order |
| 164, 165 | the store fails closed, naming ACC-037 (78 point 17, F15) | SO-F |
| 232 | a run inside the engine's domain of trust: ACC-027 (78 point 14) | another order |
| 156, 158 | reconciliation, ACC-044 among the promises (78 point 5) | SO-B (172b), SO-E |
| 161 | F13 after a late "succeeded", into OPS-012 and OPS-024 (80 point 2) | SO-B (172a) |
| 163 | F13 does not reach a call that changes nothing (80 point 3) | SO-E |
| 216 | the connection Conduit manages for a Claude sign-in (80 point 8) | order 050 |

F3 puts CTR-013 among the rules on the caller alone, and its clause on a root-only input still
refuses today. So decisions 5 and 6 name CTR-013. The order that writes 78 point 10 removes that
clause, and with it CTR-013 from ACC-056's step 3 and table. If that order has merged before you
start, CTR-013 no longer refuses: leave it out of decisions 5 and 6, and report that you did.

**Cases.** This order adds no promise ID, so
[CONTRIBUTING.md, rule 8](../../CONTRIBUTING.md#writing-promises) asks it for no new case. It adds
two anyway:

- the vector that F1 needs, which row 19 names. The build passes it, and "Done when" shows that it
  can fail.
- F3's one vector, which the build does not pass yet. It is listed in
  [conformance/expected-failures.yaml](../../conformance/README.md#expected-failures), as rule 8
  says. Order 163 made that list.

F2's vectors and F3's other vectors come with order 046a, which builds the refusal tiers, as row
19's size cell says. This order changes no existing case.

**Orders in flight.** Build on `main` as it is when you start.

- Order 171b follows this one. It changes other promises in `spec/README.md`,
  `spec/definitions.md` and `spec/interface/implementation.yaml`, and it also changes
  IN_FORCE.md, the model-checking account, `conformance/expected-failures.yaml` and
  `engine/STANDING.md`.
- Orders 172a and 172b (SO-B), written beside this one, change other promises in several of the
  same files. 172a changes OPS-012, OPS-024 and OPS-028 in `spec/conventions/operations.yaml`,
  and parts of `spec/definitions.md` and `spec/interface/implementation.yaml`.
  172b changes LIF-003, LIF-024 and LIF-028 in `spec/conventions/lifecycle.yaml`, ACC-044, the
  descriptions of `reconcile.v1` and `stop.v1` in `spec/core/operations.yaml`, and parts of
  `spec/definitions.md` and `spec/README.md`. Both change lines of IN_FORCE.md, the account and
  `conformance/expected-failures.yaml`.

Whichever of these lands second keeps both changes, and writes `engine/STANDING.md` again with
the tool. Order 160 has merged: decision 12 reads the account rows it rewrote.

**Size: M.** About 125 changed lines of `spec/` in ten files, one line of IN_FORCE.md, two new
cases with their fixtures, five rows of the account, and the generated `engine/STANDING.md`.

Read first:

- the findings linked above, each with its verifier's corrections, which govern;
- fixes F1 to F3 and F122 in [decisions.md](../proposals/full-review/decisions.md);
- [ruling 80](../proposals/ruling-80/README.md), point 1;
- IN_FORCE.md, lines 44, 45, 135 and 147 to 150;
- `spec/README.md`, "The form of a promise" (lines 282 to 309);
- in `spec/conventions/access.yaml`: ACC-006, ACC-011, ACC-012, ACC-024, ACC-029, ACC-030,
  ACC-032, ACC-034, ACC-053 and ACC-056;
- OPS-009, OPS-012, OPS-023, OPS-024, OPS-029 and OPS-032 in
  `spec/conventions/operations.yaml`, and LIF-028, which shows that `open.v1` and `close.v1` can
  end unknown;
- IFC-012, IFC-013 and IFC-050 in `spec/interface/implementation.yaml`;
- `refusal.v1` in `spec/core/results.yaml`, `bound.v1` in `spec/core/requirements.yaml`, and
  `open.v1` and `close.v1` in `spec/core/operations.yaml`, whose descriptions IFC-033 puts in
  each tool's description;
- in AGENTS.md, what `promise_standing.py` needs;
- in `conformance/README.md`, "Running", "The coverage index and the case schema in pieces",
  "Expected failures" and "Reviewing vector changes";
- the case `access-bounds`, its steps 1 to 5, 13 and 18, and its observations 0 to 3, 11 and 16;
- decision 7 of [order 153](153-model-account-checked.md), and how
  [order 157](157-own-tools-reported.md) added cases through the pieces tool.

## The decisions it applies

Line numbers are those of `main` at `d4c8cf8b`, where `spec/`, `engine/` and the cases are as
they were at `260f3cba`. If a line has moved, find it by its words. Follow CONTRIBUTING.md,
"Writing promises":

- Rewrap only as far as a change needs, at the field's four-space indent, with every line under
  97 characters (rule 1). ACC-056's observable is a literal block (`|-`): keep its lines as
  lines.
- Keep everything that a decision does not name (rule 7).
- `spec/` carries no change narrative.

1. **The form says what an observable binds** (F122). In `spec/README.md`, line 293, the
   `observable` row becomes:

   ```text
   | `observable` | What can be checked to tell whether the promise was kept. What it says is made, returned, rejected or never done binds as `then` does. A comparison with another system, or a reason given, explains and binds nothing. Where `observable` and `then` differ, `then` governs. |
   ```

   Nothing else in the section changes. Rule 1, a rule on what an observable may say, and the
   `vectors` row belong to SO-H (IN_FORCE.md, line 45).

   F122's own words are "a sentence that names another system". This order writes FR-140's
   words, "a comparison with another system", so that a sentence that cites an RFC for what the
   engine does still binds. PND-013's rebuilding of a redirect value is one such sentence.
2. **ACC-024's refusal is ACC-024's** (F122's named clash; FR-158, situation 1; FR-004's
   verifier). In `spec/conventions/access.yaml`, line 428, ACC-024's observable changes "reports
   refused under ACC-012" to "reports refused under ACC-024". LIF-022 (lifecycle.yaml, line 471)
   and LIF-034 (line 712) already say so, and so does the build.
3. **A Pool named in `targets` covers that Pool alone** (F1).
   - **ACC-029's `then`** (lines 479 to 482) becomes: "The bound is a restriction, which the
     engine applies itself. It names Pools, in `pools`, individual targets, in `targets`, and
     operations. Naming a Pool in `pools` covers the Pool itself and every Resource that Pool
     owns, including those instantiated after the Conduit that carries the bound was opened. An
     individual target is covered when `targets` names it. A Pool named in `targets` is an
     individual target: it covers that Pool alone and none of its Resources, as a Fleet named
     there covers that Fleet alone." Decision 4 then appends two sentences.
   - **ACC-029's observable.**
     - Lines 484 to 486: "whose bound names a Pool can open" becomes "whose bound names a Pool in
       `pools` can open". "unless the bound names that Resource or its Pool" becomes "unless the
       bound names that Resource, or names its Pool in `pools`".
     - Lines 508 and 509: "naming its Pool covers the replacement" becomes "naming its Pool in
       `pools` covers the replacement".
   - **ACC-030's `then`** (lines 521 to 523). The sentence becomes: "A new bound is within an old
     one when every Pool that its `pools` names is one that the old one's `pools` names, every
     individual target that its `targets` names is covered by the old one, as ACC-029 states, and
     every operation it names is one the old one names." "This is the whole condition" stays.
   - **ACC-030's observable** (line 535). "and may name a Pool only if the old one names that
     Pool" becomes "and may name a Pool in `pools` only if the old one names that Pool in
     `pools`".
   - **The copies:**
     - `spec/README.md`, lines 464 and 465. The bullet's first sentence becomes: "A bound that
       names a Pool in `pools` covers every Resource that the Pool owns, those instantiated after
       the Conduit that carries the bound was opened among them, and a Pool named in `targets`
       covers that Pool alone (ACC-029)."
     - `spec/definitions.md`, line 32. "a Fleet in `targets` covers that Fleet alone (ACC-029)"
       becomes "a Pool or a Fleet in `targets` covers itself alone (ACC-029)".
     - `spec/definitions.md`, line 186, three changes:
       - "Naming a Pool covers the Pool itself and every Resource it owns, whenever instantiated
         (ACC-029)." becomes "Naming a Pool in `pools` covers the Pool itself and every Resource
         it owns, whenever instantiated (ACC-029)."
       - "One bound is within another when every Pool and every target it names is covered by
         the other, and every operation it names is one the other names (ACC-030)." becomes "One
         bound is within another when every Pool its `pools` names is one the other's `pools`
         names, every target its `targets` names is covered by the other, and every operation it
         names is one the other names (ACC-030)."
       - Near the row's end, "a Fleet in `targets` covers that Fleet alone (ACC-029)" becomes "a
         Pool or a Fleet in `targets` covers itself alone (ACC-029)", as on line 32.
     - `spec/core/requirements.yaml`, lines 52 and 53, `bound.v1`'s description. "Naming a Pool
       covers the Pool itself and every Resource it owns." becomes "Naming a Pool in `pools`
       covers the Pool itself and every Resource it owns; a Pool named in `targets` covers that
       Pool alone."
4. **A bounded holder that names no Conduit is refused under ACC-029, unless it repeats its own
   request** (F2, first part, as ruling 80, point 1, narrows it; FR-003 and FR-209).
   - **ACC-029's `then`** gains two last sentences: "A request to open or to close that is made
     through such a Conduit and names a Conduit that does not exist is not covered by its bound:
     it reports refused under this promise, with the bound in
     `io.github.shrav-k.conduit/restriction`, as one whose target the bound does not cover does.
     This does not hold for a repeat of the consumer's own request, as OPS-009 describes a match,
     that neither OPS-012 nor OPS-024 dispatches as a new attempt: the repeat is answered from
     the stored outcome of the invocation it repeats, as OPS-009 and OPS-032 state."
   - The second sentence is ruling 80's exemption, and it exempts nothing else. A request that
     repeats no earlier request of the consumer's is refused as the first sentence says, and a
     repeat is still decided anew under ACC-036.
   - A repeat of an invocation whose outcome is unknown or failed is dispatched as a new attempt
     under OPS-012 (`open.v1` and `close.v1` declare `deduplicates: true`) or OPS-024. LIF-028
     shows that either can end unknown. That attempt is not answered from a stored outcome, so
     the exemption leaves it out, and the first sentence refuses it. This is the coordinator's
     reading (see "Who decided what").
   - **OPS-029's observable** (`spec/conventions/operations.yaml`, lines 702 to 705). "A bound is
     judged against what exists, and does not make such a request report refused: a request for
     `open.v1` that names no Conduit reports invalid whatever the bound of the actor's Conduit to
     `access`." becomes "A request for `open.v1` or `close.v1` that names no Conduit reports
     invalid where the actor's Conduit to `access` carries no bound, and refused under ACC-029
     where it carries one, as ACC-053 refuses a bounded holder's `lift.v1`: a bounded holder
     learns nothing of a name outside its bound. A repeat of the actor's own request, as OPS-009
     describes a match, that neither OPS-012 nor OPS-024 dispatches as a new attempt is answered
     from the stored outcome of the invocation it repeats, as OPS-009 and OPS-032 state, whether
     or not that Conduit carries a bound." OPS-029's `when` and `then` do not change. OPS-023
     already puts refused before invalid, as it does for `lift.v1`.
   - **The copies** in `access.yaml`:
     - ACC-006's observable, line 126. After "and reports invalid under OPS-029," insert "or
       refused under ACC-029 where the actor's Conduit to `access` carries a bound,".
     - ACC-011's observable, lines 195 and 196. "reports invalid under OPS-029, because it no
       longer exists." becomes "reports invalid under OPS-029, because it no longer exists, or,
       to open or close it, refused under ACC-029 where the actor's Conduit to `access` carries
       a bound."
     - ACC-012's observable, line 219. After "so a request to close it reports invalid under
       OPS-029" insert ", or refused under ACC-029 where the actor's Conduit to `access` carries
       a bound". The clause after it, "and a repeat of the request that closed it is answered as
       OPS-009 and OPS-032 state", stays as it is: under ruling 80 it holds for a bounded holder
       too.
   - **The copies in the core package.** In `spec/core/operations.yaml`, the descriptions of
     `open.v1` (lines 764 and 765) and `close.v1` (line 803) each say "reports invalid,
     whatever the bound." Each becomes "reports invalid, or refused where the actor's own
     Conduit to `access` applies a bound (ACC-029)." IFC-033 puts these descriptions in the
     tools an Agent's model reads. Rewrap only the lines the change touches, each under 97
     characters, and change nothing else in the file.
   - **Copies this order leaves.** Three sentences have a request that names a closed Conduit
     report invalid:
     - CON-008's observable (`spec/promises/conduits.yaml`, lines 117 to 119);
     - CON-031's observable (lines 397 to 399);
     - `spec/core/README.md`, lines 323 and 324.

     They stay as they are. OPS-023 already gives a bounded holder refused first, and the sweep
     of observables is SO-H's.
   - **Others.** Run `git grep -n -e 'reports invalid under OPS-029' -e 'names no Conduit' -e
     'whatever the bound' -- spec`. Report any other sentence that has `open.v1` or `close.v1` of
     `access` report invalid for a Conduit that does not exist, and change none.
5. **ACC-056's `then`: one deciding rule, a cause in the engine's words, and the two tiers** (F3;
   F2, second part; FR-211 as its verifier narrowed it). Replace ACC-056's `then` (lines 915 to
   925) with this text, rewrapped under 97 characters. Its line 918 is 126 characters today.

   ```text
   Its result carries one error diagnostic, whose ruleId is the refusing promise and whose
   properties.refusal has the form refusal.v1. Where more than one rule would refuse the
   request, the refusing promise is the first that refuses in this order: (1) who is asking:
   IFC-032 and IFC-022, where the request establishes no instance, then the sender and the
   invocations it serves, under IFC-021, IFC-028, IFC-016, ACC-019, ACC-034, ACC-039 and
   IMP-008; (2) ACC-001; (3) the rules on the caller and the operation it asks for alone:
   ACC-032 on an operation reserved to root, ACC-057, CTR-013 and SEC-007, and PND-009 and
   PND-010 on who may answer or sign in; (4) ACC-033; (5) the bound: ACC-029, ACC-030, ACC-053
   and ACC-055; (6) the rules that read the object the request names: ACC-054, SEC-008, ACC-024
   and LIF-034, and ACC-032, PND-009 and PND-010 on a Conduit to be opened that lists an
   operation they restrict; (7) last, the filter: ACC-008. A promise that this order does not
   name restates a refusal and is never the refusing promise. Tier 1 is given to every caller,
   an Agent's model among them: the qualified operation, the tool and local Conduit name and
   object references as the request named them, and a cause: a non-empty sentence, in the
   engine's own words, that conveys the refusing rule's row below. The no-open-Conduit
   statement belongs to ACC-001. Nothing in tier 1 depends on a fact outside both what the
   caller may inspect and what the bound of its Conduit to `access` covers, where the request
   is made through one; one with no bound covers every target (ACC-029). Within them, tier 1
   may depend on the named Conduit and on the chain of invocations the caller serves. Tier 2
   is always given to root. Another caller gets it only where it may inspect the authorizing
   target under ACC-033 and every object the request names, through open Conduits with no
   filter. For an operation that ACC-033 does not route, the authorizing target is the
   request's target; for a request that a Pool or a Fleet authorizes, no caller but root and a
   holder of `inspect.v1` on `management` meets that condition. Tier 2 adds the resolved target
   and the caller's Conduits to it, each with its name, local name, state and operations, or
   states that the named object does not exist.
   ```

   - ACC-056's `when` does not change.
   - In its observable, lines 929 and 930, "Without that inspection authority the diagnostic is
     identical whether a name exists or not, apart from the references the caller wrote." becomes
     "Without that inspection authority the diagnostic is identical whether a name exists or not,
     apart from the references the caller wrote, except where the `then` lets tier 1 depend on a
     named object within the caller's bound."
   - Decision 6 adds a row. No other part of the observable changes. Its sentence "Promises that
     restate a refusal reuse the deciding access rule's cause; the ruleId remains the deciding
     rule." stays.

   Under this order the rows of promises that restate a refusal are never a ruleId. Among them are
   ACC-006, ACC-011, ACC-012, IFC-012, IFC-013, IFC-019, IFC-026, IFC-027, IFC-033, ACC-028 and
   REC-004. That settles FR-158's situations 2 and 3, and FR-038's case, without touching their
   rows.
6. **CTR-013's row** (F3; FR-004). In ACC-056's table, between the rows of CON-031 and FLEET-004,
   add:

   ```text
       | CTR-013 | Only root may confirm a shared client ID in install.v1's confirmClientIds. |
   ```

   No other row changes.
7. **The copies that give a closed Conduit's refusal to ACC-011 give it to ACC-001** (F3; FR-004,
   FR-038). ACC-011's own observable already says so. "under ACC-011" becomes "under ACC-001" in
   six places:
   - ACC-034's observable, `access.yaml` line 689 ("it reports refused, under ACC-011");
   - OPS-009, `operations.yaml` line 242;
   - OPS-032, lines 795 and 796 ("reports refused" ends line 795);
   - OPS-034, lines 860 and 861 (the same);
   - LIF-026, `lifecycle.yaml` lines 563 and 564;
   - IFC-013, `spec/interface/implementation.yaml` line 345 ("reports refused under ACC-011, and
     is answered as IFC-012 states").

   CON-014's "What its later requests through the closed Conduit report is ACC-011"
   (`conduits.yaml`, line 193) names the promise that says what they report, not a ruleId. It
   stays.
8. **`refusal.v1`'s cause** (F3; [FR-127](../proposals/full-review/findings/L11-core-rest.md#fr-127)
   as its verifier narrowed it). In `spec/core/results.yaml`:
   - Lines 617 to 619, the description, become "The two tiers of ACC-056. named echoes only the
     request; cause is a non-empty sentence, in the engine's own words, that conveys the refusing
     rule's row of ACC-056. target, exists and conduits are tier-2-only. An unknown tool need not
     be decomposable into an operation; its spelling is preserved in named.tool."
   - Lines 640 and 641: `cause` gains `minLength: 1` under its `type: string`.
   - `operation` does not change.
9. **DL9** (F3; FR-126 with its verifier). In `spec/implementation-defined.yaml`, line 75, DL9's
   `document` becomes "The order in which several failed checks that give the same reason, other
   than refused, are reported. A refused request names one rule, the first in ACC-056's order."
10. **The Pool-target case** (F1's vector; FR-001's "one vector for each sentence").
    - **Where it goes.** Add `conformance/conventions/access/pool-target/`, holding `case.yaml`
      (the case `access-pool-target`) and `observations.json`, each written as `access-bounds`
      writes its own: JSON with two-space indentation and a final newline.
      - The case's members are those of `access-bounds`, in its order: `id`, `layers`,
        `promises`, `title`, `profile`, `initial`, `files` and `steps`.
      - Copy `access-bounds`' `layers`, `profile`, `initial` and `files`, with `observations`
        pointing at the new file. Its `promises` are `["ACC-029", "ACC-030"]`, and its `title`
        is "A Pool named in a bound's targets covers that Pool alone".
      - Its first step is `access-bounds`' first step, the `register-project` step, unchanged.
        After it come three `access-fixture` steps, indices 0 to 2 of the new file, which the
        runner counts as steps 2 to 4.
      - Use only input kinds and expectations that exist. Add nothing under
        `conformance/case-schema/`, and leave `conformance/case.schema.json` unchanged.
    - **Each observation** has a `patches` list and a `call`:
      - the ten `set` patches that every observation of `access-bounds` has;
      - two `append` patches. The first is `c610`, as in `access-bounds`' observation 0, whose
        bound becomes `{"pools": [], "targets": [{"kind": "Pool", "id":
        "00000000-0000-4000-8000-000000000117"}], "operations":
        ["example.org/s1/filesystem#file.read.v1"]}`. The second is `c611`, registered and not
        open, whose consumer is Agent `agent-a` (`…0109`), as each step below gives it. The Pool
        is `workspace` (`…0117`), and it owns the Resource `shared` (`…0119`);
      - the `call` of the `access-bounds` observation named in the step below, copied whole with
        its `evidence`: a call from `…0109`, `c610.open.v1` naming `c611`. Each step's records
        are then the records of the `access-bounds` step it copies, unchanged.
    - **Step 2: `c611` to the Pool.** `c611` is the one of `access-bounds`' observation 3: it
      targets `…0117`, lists `file.read.v1` and has the capability `@core#pool-management.v1`.
      The call is observation 3's. Expect exactly what that case's step 5 expects: `permitted`
      true with the request echoed, `stateUnchanged`, no external intent, its `absent`, its
      records, and `required_promises` `["OPS-023", "IFC-025", "ACC-002", "ACC-003",
      "ACC-036"]`.
    - **Step 3: `c611` to the Resource.** `c611` is the one of observation 0: it targets
      `…0119`, lists `file.read.v1` and has the capability
      `example.org/s1/filesystem#directory.v1`. The call is observation 1's. Expect what
      `access-bounds`' step 3 expects, with one change: `reply.restriction.arguments` holds
      `c610`'s bound above. So:
      - reason refused, with `restriction` holding `c610`'s bound;
      - `stateUnchanged`, no external intent, and its `absent` and records;
      - `required_promises` `["OPS-023", "IFC-025", "ACC-029"]`.
    - **Step 4: `c611` to `access`.** `c611` is the one of observation 11, with its bound's Pool
      changed: it targets `access` (`…0004`), lists `@core#open.v1`, has the capability
      `@core#access-control.v1` and carries the bound `{"pools": [{"kind": "Pool", "id":
      "00000000-0000-4000-8000-000000000117"}], "targets": [], "operations":
      ["example.org/s1/filesystem#file.read.v1"]}`. The call is observation 11's. Expect what
      `access-bounds`' step 13 expects, with the same one change as step 3: reason refused, with
      `restriction` holding `c610`'s bound, and `required_promises` `["OPS-023", "IFC-025",
      "ACC-030"]`.
    - **Its index entry.** Give it one entry in `conformance/conventions/access/index.yaml`:
      `id`, `case`, `promises` and `safety_rows: []`, in that order. Add the entry as order 157
      did:
      - write it in a `cases` list in `conformance/manifest.yaml`;
      - then, from `engine/`, run `cargo run --release -p conduit-conformance --example pieces`
        with no flag.

      The tool moves the entry into its piece and leaves `manifest.yaml` byte for byte as it was.
    - The build keeps both sentences already, so the case passes. "Done when" shows that it can
      fail.
11. **F3's vector: the bound decides before ACC-024** (F3; FR-210, situation ii, as its verifier
    corrected it).
    - **Where it goes.** Add `conformance/conventions/access/close-root-outside-bound/`, holding
      `case.yaml` (the case `access-close-root-outside-bound`) and `observations.json`, written
      as in decision 10.
      - Copy `layers`, `profile`, `initial`, `files` and the first step from `access-bounds`, as
        in decision 10, with the members in the same order.
      - Its `promises` are `["ACC-029", "ACC-056"]`, and its `title` is "The bound decides
        before the rule that protects root's Conduits".
      - It has one more step, an `access-fixture` step reading index 0, which the runner counts
        as step 2.
    - **The observation** is `access-bounds`' observation 16, its call and evidence included, with
      one change. `c611`'s `target` becomes `management` (`…0003`) and its `capability`
      `@core#object-management.v1`, as in observation 2. Pool `system` owns `management`, so
      `c610`'s bound does not cover it. `c611`'s consumer stays root (`…0005`), and agent-a still
      asks `c610.close.v1` for it.
    - **What step 2 expects.** What `access-bounds`' step 18 expects, with two changes:
      - `reply` also holds `restriction` with `c610`'s bound, as step 3 of that case has it;
      - `required_promises` is `["OPS-023", "IFC-025", "ACC-029"]`.

      Under ACC-056's order the bound refuses first. The build checks ACC-024 first, so the case
      fails at step 2.
    - **Its index entry,** in `conformance/conventions/access/index.yaml`, goes in as in
      decision 10.
    - **The expected failure.** Add this entry to `conformance/expected-failures.yaml`, as the
      file's rules state (serde_json's pretty printing, a final newline, entries sorted by `id`
      in byte order). Keep any entry another order has added.
      - `"id": "access-close-root-outside-bound"`;
      - `"reason": "The first build checks ACC-024 before the bound; ACC-056's order of refusing
        rules (work order 171a) makes ACC-029 decide. Order 046a builds that order."`
    - **Other steps.** Change no existing case. If you notice a step elsewhere whose expected rule
      decision 5 changes, change nothing there, and report it with its case and step.
12. **The model-checking account.** Five promises that have a model-run standing change text:
    - ACC-011 (decision 4);
    - ACC-034 (decision 7);
    - OPS-009, OPS-032 and LIF-026 (decision 7).

    In `experiments/012-model-checking/account.yaml`, each of those rows becomes
    `standing: text changed, not rerun`, as order 153's decision 7 marks one:
    - `was` holds its old standing: `holds, not graded` for ACC-011 and LIF-026, `restated`
      for OPS-009 and OPS-032, as order 160 graded them, and `property over-broad: MC3-002`
      for ACC-034, quoted as its standing is now;
    - `model`, `read`, `digest` and `note` stay as they are.

    The row of OPS-034 is already `text changed, not rerun` and stays as it is. The static and
    not-modelled rows have no digest and do not change. If `tools/check_model_account.py` names
    any other row, mark it the same way and report it.
13. **The declaration.** Write `docs/work-orders/171a-declared.yaml` in the form of
    `docs/work-orders/081-declared.yaml`, with two declarations and nothing more:

    ```yaml
    - name: a Pool named in a bound's targets covers that Pool alone
      kind: added
      cases: [access-pool-target]
    - name: the bound decides before the rule that protects root's Conduits
      kind: added
      cases: [access-close-root-outside-bound]
    ```
14. **The standing table.** Once the two cases, their index entries and the expected failure are
    in place, run `python3 tools/promise_standing.py --write` from the repository root, as
    AGENTS.md asks of a change to a case's `promises` or to the expected failures. It rewrites
    `engine/STANDING.md`. The rows of ACC-029, ACC-030 and ACC-056 change, ACC-056's to
    `expected failure`, and so do the totals of `expected failure` and `no case, no reason`. No
    other row changes. Never edit the file by hand.
15. **IN_FORCE.md, line 135** (ruling 67, point 6: when a specification change lands, its *spec
    pending* lines become pointers; ruling 80, "What follows"). The line that begins
    "  - *Coordinator's, spec pending, order 171a* (F2):" is replaced by this one line, at the
    same place and with the same two-space indent, so that no other line of the file moves:

    ```text
      - *Coordinator's* (F2), **in spec/**: a bounded holder's request to open or close that names a Conduit that does not exist is refused, except a repeat of the holder's own request that is answered from its stored outcome; a repeat dispatched as a new attempt is decided as a new request: ACC-029, OPS-029. The exemption of the holder's own repeat is the owner's (80 point 1).
    ```

    Change no other line of IN_FORCE.md.

## Scope

Change only these files, and in each only the lines the decisions name and the rewrapping they
need:

- `spec/README.md`
- `spec/definitions.md`
- `spec/core/operations.yaml`
- `spec/core/requirements.yaml`
- `spec/core/results.yaml`
- `spec/conventions/access.yaml`
- `spec/conventions/operations.yaml`
- `spec/conventions/lifecycle.yaml`
- `spec/interface/implementation.yaml`
- `spec/implementation-defined.yaml`
- `IN_FORCE.md`, line 135 alone (decision 15)
- `experiments/012-model-checking/account.yaml`
- `conformance/conventions/access/index.yaml`
- `conformance/expected-failures.yaml`
- `engine/STANDING.md`, written only by the tool, as decision 14 says

Add these:

- `conformance/conventions/access/pool-target/`;
- `conformance/conventions/access/close-root-outside-bound/`;
- `docs/work-orders/171a-declared.yaml`.

`conformance/manifest.yaml` holds the new entries only until the pieces tool moves them, and ends
unchanged. Change nothing in these:

- `engine/`, apart from `engine/STANDING.md`: the edits that "Done when" asks for are temporary
  and restored;
- `conformance/conventions/access/bounds/`;
- `spec/promises/conduits.yaml`, `spec/promises/pending.yaml`, `spec/promises/contracts.yaml`
  and `spec/core/README.md`;
- any other line of `IN_FORCE.md`;
- `docs/implementation/SAFETY_CATALOGUE.md`, which quotes an earlier commit, as its header says;
- `experiments/012-model-checking/G5.md`.

## Done when

From the repository root, each passes:

```bash
python3 tools/check_schemas.py
python3 tools/set_names.py --check
python3 tools/check_rulings.py
python3 tools/check_docs.py
python3 tools/check_model_account.py
python3 tools/promise_standing.py --check
python3 tools/check_private_paths.py
git diff --check
python3 tools/vector_delta.py HEAD --declared docs/work-orders/171a-declared.yaml
```

`check_model_account.py` must end `Errors: 0`. On `main` at `d4c8cf8b` it counts `holds, not
graded` 66, `restated` 6, `property over-broad: MC3-002` 1 and `text changed, not rerun` 28, and
under "by was" `holds, not graded` 9. Its counts must move as decision 12 says:

- `holds, not graded` goes from 66 to 64;
- `restated` goes from 6 to 4;
- `property over-broad: MC3-002` leaves the standings;
- `text changed, not rerun` goes from 28 to 33;
- under "by was", `holds, not graded` goes from 9 to 11, and two lines appear:
  `property over-broad: MC3-002: 1` and `restated: 2`.

If an order that merged after `d4c8cf8b` changed the account, report the counts you see instead.

`promise_standing.py --check` must end `Errors: 0`, and it fails until decision 14's `--write`
has run.

`vector_delta.py` must end `Vector delta: 2 changes; 0 flips; 0 undeclared; 0 errors.` It reads
the declaration with PyYAML, so where `python3` lacks it, run it with `/usr/bin/python3`.

This prints 0. It reads the five files with their line breaks as spaces, so a copy wrapped over
two lines still counts. It prints 10 on `main` today:

```bash
cat spec/conventions/access.yaml spec/conventions/operations.yaml spec/conventions/lifecycle.yaml spec/interface/implementation.yaml spec/core/operations.yaml | tr -s ' \n' '  ' | grep -o -E 'refused,? under ACC-011|refused under ACC-012|invalid,? whatever the bound' | wc -l
```

This prints nothing:

```bash
git status --short -- engine ':(exclude)engine/STANDING.md' conformance/manifest.yaml conformance/case.schema.json conformance/case-schema conformance/conventions/access/bounds
```

Each of these prints 1. The ninth prints 0 only where CTR-013 was left out, as "IN_FORCE.md's lines
in these files" allows.

```bash
tr -s ' \n' '  ' < spec/README.md | grep -c 'Where `observable` and `then` differ, `then` governs.'
tr -s ' \n' '  ' < spec/conventions/access.yaml | grep -c 'A Pool named in `targets` is an individual target: it covers that Pool alone'
tr -s ' \n' '  ' < spec/conventions/access.yaml | grep -c "every Pool that its \`pools\` names is one that the old one's \`pools\` names"
tr -s ' \n' '  ' < spec/conventions/access.yaml | grep -c 'names a Conduit that does not exist is not covered by its bound'
tr -s ' \n' '  ' < spec/conventions/access.yaml | grep -c "This does not hold for a repeat of the consumer's own request, as OPS-009 describes a match, that neither OPS-012 nor OPS-024 dispatches as a new attempt"
tr -s ' \n' '  ' < spec/conventions/access.yaml | grep -c 'A promise that this order does not name restates a refusal and is never the refusing promise.'
tr -s ' \n' '  ' < spec/conventions/access.yaml | grep -c 'ACC-032 on an operation reserved to root'
tr -s ' \n' '  ' < spec/conventions/operations.yaml | grep -c 'and refused under ACC-029 where it carries one'
grep -c '^    | CTR-013 | ' spec/conventions/access.yaml
grep -c "^  - \*Coordinator's\* (F2), \*\*in spec/\*\*: a bounded holder's request" IN_FORCE.md
grep -c 'Spec pending, ruling 80, order 050' IN_FORCE.md
```

And this prints 0: `grep -c 'spec pending, order 171a' IN_FORCE.md`.

From `engine/`, run each of these, one at a time:

```bash
cargo run --release -p conduit-conformance --example pieces
cargo run --release -p conduit-conformance -- .. access-pool-target
cargo run --release -p conduit-conformance -- .. access-close-root-outside-bound
CONDUIT_VECTOR_THREADS=4 cargo test --release -p conduit-conformance --test vectors
CONDUIT_VECTOR_THREADS=4 cargo test --release
```

What each must show:

- The pieces tool prints `Nothing to move.`
- The `access-pool-target` run prints `PASS access-pool-target (4 steps; ACC-029, ACC-030)` and
  ends `1 cases passed`.
- The `access-close-root-outside-bound` run ends `0 cases passed; 1 expected failures`. Its
  `EXPECTED FAILURE access-close-root-outside-bound (ACC-029, ACC-056): …` line must be followed
  by this error, indented by two spaces:
  `uncited expected promise "ACC-029"; reply: {"reason":"refused"}`. The runner names no step
  for an expectation that fails, and this error can come only from step 2: the build refuses
  the close under ACC-024, with no restriction. Any other error means the case is broken: fix
  it.
- The two test commands pass. `access-bounds` among them passes unchanged. Since order 071 the
  runner uses every thread the machine offers unless `CONDUIT_VECTOR_THREADS` names a number. At
  24 threads it ran out of memory, hence the 4.

Then show that `access-pool-target` can fail. From `engine/`, one at a time:

1. `cp crates/core/src/access.rs target/access.rs.171a`
2. In `covers` (from line 88), let an item of `targets` also cover a Resource whose `owner` it
   names. Then run the `access-pool-target` command.
3. `cp target/access.rs.171a crates/core/src/access.rs`
4. In `subset_in_view` (from line 114), let a new `pools` item pass where the old bound's
   `targets` names it. Run the command again.
5. `cp target/access.rs.171a crates/core/src/access.rs`, and run the command a last time. It must
   pass.

Each of the runs in steps 2 and 4 must fail, printing the line `FAIL 0 cases passed; 1 failed`
and on the next line `access-pool-target [ACC-029, ACC-030]: unexpected field /reply/request`: a
step that expects a refusal got a permitted reply. The runner names no step here either. Each change
reaches one step alone: `covers` decides step 3, the Conduit to the Resource, and
`subset_in_view` decides step 4, the Conduit to `access`; step 2's Pool is covered by its ID
either way.

Restore with `cp`, not `mv`: a moved file keeps its older modification time, so cargo would not
rebuild. `git diff --quiet -- engine ':(exclude)engine/STANDING.md'` must then exit 0.

Do not stage anything. In your report:

- mark each choice you made yourself as your own, the placements of decision 5 as the
  coordinator's, decision 4's exemption of a repeat answered from its stored outcome as the
  owner's (ruling 80, point 1), and its leaving out a repeat dispatched as a new attempt as the
  coordinator's;
- say whether CTR-013 went in, and give anything decision 4's search found;
- give the `FAIL` line and the line after it of each run that had to fail;
- give the `EXPECTED FAILURE` line with its error;
- give the last line of every check.

`git diff --check` and `check_private_paths.py` read tracked files only. The admin then stages the
new files and runs `python3 tools/check_private_paths.py` and `git diff --cached --check`. Before
the merge the admin runs the engine's build, its four shards and the doctests at the local gate
(ruling 80, point 10); GitHub's CI runs on the pull request too, and the merge does not wait for
it.

## Record

Run on 2026-11-16 on the Mac from `main` `55d2b85a`: the worker (1,255 s) and a fresh Codex verifier
(251 s), which found nothing to change in one round. It ran both cases with the runner the worker
had built, but did not rebuild, run `cargo test` or the fix-removal runs. The form of a promise now
says what an observable binds, and that
`then` governs where the two differ (F122), and ACC-024's observable names ACC-024. A Pool named
in a bound's `targets` covers that Pool alone, and a new bound's `pools` must be within the old
one's `pools` (F1), in ACC-029, ACC-030 and their copies. A bounded holder's request to open or
close a Conduit that does not exist is refused under ACC-029, except a repeat of its own request
that is answered from its stored outcome (F2; ruling 80, point 1), in ACC-029, OPS-029, ACC-006,
ACC-011, ACC-012 and the descriptions of `open.v1` and `close.v1`. ACC-056 states one order of
the refusing rules, a cause in the engine's own words and who receives tier 2 (F3). CTR-013 went
into that order and its table, as it still refuses. Six copies give a closed Conduit's refusal to
ACC-001, `refusal.v1`'s `cause` has `minLength: 1`, and DL9 no longer covers a refusal. Two cases
were added: `access-pool-target`, which passes and fails when either `covers` or `subset_in_view`
is loosened, and `access-close-root-outside-bound`, an expected failure until order 046a builds
ACC-056's order. Five rows of the model-checking account became `text changed, not rerun`,
`engine/STANDING.md` was written by the tool, and IN_FORCE.md's line 135 now points into `spec/`.
A fresh read-only Opus reader then verified the order (ruling 79, point 2) on the branch merged
with `main` `ecabe372`. It reran every check, the tools' unit tests, cargo fmt, clippy and
`cargo test --release` with 4 vector threads, all of which passed, and both fix-removal runs on a
scratch copy, each of which failed as "Done when" says. It found no must and three shoulds. For
order 046a it noted that the build's check order in `evaluate_path` differs from ACC-056's in
more places than ACC-024 against the bound: ACC-033's routing comes before ACC-032 and ACC-039,
and ACC-054 before the owner check of a registration under ACC-033. No existing step it sampled
expects a rule that decision 5 changes. Before merge the branch was rebased onto `ecabe372` with
no conflict, and the first should was applied: the copy of ACC-029's rule on a replacement in
`spec/README.md` now reads "a bound that names its Pool in `pools`". The other two are recorded here
for SO-H's sweep of observables. The row `close` of `spec/definitions.md`, where a request that
names a closed Conduit reports invalid (OPS-029) with no word on a bound, is a copy of the kind
decision 4 leaves, beside CON-008, CON-031 and `spec/core/README.md`. NAM-026's observable, where a
call to a tool that is not presented reports refused under IFC-012 while IFC-012 gives ACC-001, is
of decision 7's kind, in `spec/promises/naming.yaml`, outside this order's scope.
