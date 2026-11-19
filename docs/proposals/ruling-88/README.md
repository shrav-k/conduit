# Ruling 88: the carried return leaves the dogfood path for the keyed release, and 048a-5's last four planned parts are batched in pairs

Status: in force (ruled by the owner on 2026-11-19, as the coordinator's notes date the answers). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topics 9 and 11 (point 2, in the lines naming the orders that build the sign-in hook part, the relay and polling) and topic 13 (points 1 to 3), and A78's row of [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes) (point 3), and a row of [docs/AUDIT.md](../../AUDIT.md#waiting-for-the-owner) (what is left for the owner). This record says why.

On 2026-11-19 the owner asked: "Check in, what's our progress like? Why is our parallelism so low?
How close are we to dogfood?"

The coordinator answered in the session; that answer is not in the tree. It then put two questions
to the owner through the session's question tool, each with options and a recommendation, on the
sign-in chain that stands between the engine on `main` and the first dogfood slot on the Mac. The
owner chose the recommended option in each. The questions as put, the options as the
coordinator's notes (private) hold them and the owner's answers follow, verbatim. The notes date
them 2026-11-19 and record no time for either.

## The questions

1. As put: "Ruling 86 left 048a-7 and 048a-8 (the carried sign-in return: pasting a code or
   redirect back through Conduit) with no 0.2.0 user, since nothing is keyed in 0.2.0, and a Mac
   browser login needs no return. Take them off the dogfood path, so 048b-join follows 048a-5h
   directly?"

   The owner chose "Defer 7 and 8 (Recommended)", whose text read: "048b-join comes right after
   048a-5h and joins the URL relay and polling only; 048a-7, 048a-8 and 051b-3 (the return through
   Conduit, used by Codex from other machines once keyed) move after dogfood, beside the keyed
   release. Saves about 6 hours on the critical path. Records a dated ruling."

   The other option: "Keep them on the path".

2. As put: "The remaining sign-in chain is cut one behavior per order (5e message/output, 5f
   polling, 5g undo of a wrong account, 5h expiry). Each order costs a ~1h fresh read plus a gate.
   Batch them in pairs?"

   The owner chose "Pairs: 5e+5f, 5g+5h (Recommended)", whose text read: "Two orders instead of
   four, each a little larger; the fresh read and fixer stay, so review depth is unchanged per
   line. Saves about 5 hours. Waives the one-behavior-per-order rule for these two, as 048a-5b
   did."

   The other option: "Keep one per order".

The notes hold the other options by label only. The hours in the options' texts are the
coordinator's estimates, not measures this tree holds.

## The decisions

1. **Orders 048a-7, 048a-8 and 051b-3, the carried return, leave 0.2.0's path and the dogfood
   path, and move after dogfood, beside the keyed release** (answer 1). As
   [order 117](../../work-orders/117-plan-s2-s3.md) and the
   [work orders' index](../../work-orders/README.md) plan them: 048a-7, an accepted return passed
   on, with PND-013's first three checks and a page login's code passed to `sign-in-hook.v1` in
   its retry; 048a-8, a redirect return, PND-013's fourth check, the value rebuilt and passed to
   `sign-in-hook.v1` on that retry; and 051b-3, the carried return from the command line (D53,
   ruling 75 C4's case). Under [ruling 86](../ruling-86/README.md), point 1, nothing is keyed in
   0.2.0, and PND-013's first check needs a keyed hook, so it fails for every 0.2.0 package, and
   these orders have no 0.2.0 user; and on the Mac a browser login needs no return
   ([ruling 85](../ruling-85/README.md), point 5, which IN_FORCE.md's line on 051b-2 states).
   - **048b-join follows 048a-5gh directly** (with point 2, 048a-5h being part of 048a-5gh), and
     048b-relay2, merged. It joins the URL relay and polling only. Its return leg, the retry that
     carries `signInReturn` to `sign-in-hook.v1` and `signInLogin`'s `returned` on the result,
     which [order 048b-join](../../work-orders/048b-join-build-sign-in-through-the-interface.md)
     takes from 048a-7 and 048a-8 (its A4), moves with them. Which lines of 048b-join's text move
     is the coordinator's, at its refresh.
   - **Nothing leaves the accepted scope.** PND-013, PND-014 and IFC-049's return stay in `spec/`
     as written, and the three orders are built after dogfood. The scope guard (IN_FORCE.md,
     topic 1, and [the tracker's](../../AUDIT.md#scope-guard)) has deferring anything need an
     explicit owner ruling (A18), and ruling 86 left this move to one; this ruling is it, so the
     guard's words are unchanged. That it applies the guard and does not amend it is the
     coordinator's reading.
   - *Coordinator's*, open to the owner's strike: "beside the keyed release" places these orders
     in the stage of the later release whose packages list their sign-in operation as keyed
     (ruling 86, point 2), after dogfood, and fixes no order between them and that release's
     package orders, which the coordinator plans then.
   - By the coordinator's reading, open to the owner's strike:
     - order 048a-6, a return where none is taken, has merged and is not moved;
     - 051b-3 still comes before any sign-in that needs a pasted return (ruling 85, point 5), so,
       as ruling 86, point 1, already says, a Codex sign-in from another machine in 0.2.0
       completes only through the person's own SSH forward, the account check alone deciding
       completion;
     - order 050o, the managed connection for a Claude sign-in, still follows 051b-3, as ruling
       85, point 5 has it (and order 117), and so comes after dogfood unless the owner rules
       otherwise, and a sign-in that needs that connection still waits for it;
     - the S1 rerun's pin still waits for the six sign-in orders, these parts included
       ([ruling 78](../ruling-78/README.md), point 1, part 2), since the answer moves them after
       dogfood and not after the pin, so the pin, and with it the lane's restart (ruling 78,
       point 1, part 1), now waits for the keyed release's stage;
     - A77's chain, as [work orders](../../implementation/WORK_ORDERS.md) registers it, is
       unchanged: 048a-7 and 048a-8 stay on it and are not paired, and 048a-5ef and 048a-5gh are
       on it, each made of parts of it; and
     - until 048a-7, a `signInReturn` on a retry of `sign-in.v1` is never passed to
       `sign-in-hook.v1` and is kept nowhere (PND-014); PND-013's not-accepted refusal for it is
       not built in 0.2.0, and [engine/CONFORMANCE.md](../../../engine/CONFORMANCE.md)'s section on
       order 048a-6 records that a retry's return is not yet read, PND-013's checks being later
       orders'.
2. **048a-5's last four planned parts are batched in pairs** (answer 2). The
   coordinator's cut of 048a-5, as order 117 and the work orders' index give it, had 048a-5e,
   the engine's message and the input-required output, with `signInLogin`'s members and DL18's
   expiry; 048a-5f, polling on the retry; 048a-5g, another account found while polling undone by
   a sign-out part; and 048a-5h, DL18's end. They become two orders, each after the one before:
   - **048a-5ef**: the engine's message and the input-required output, with `signInLogin`'s
     members and DL18's expiry, and polling on the retry; after 048a-5d2.
   - **048a-5gh**: another account found while polling undone by a sign-out part, and DL18's end;
     after 048a-5ef.

   [FR-264](../full-review/findings/P01-work-orders-lanes.md#fr-264)'s one behavior per
   order is waived for these two, as it was for
   [048a-5b](../../work-orders/048a5b-build-sign-in-account-first.md). The fresh read and the fixer
   stay for each, as the option said. By the coordinator's reading:
   - only the one-behavior rule is waived, and each pair's text keeps the full review's other
     guides on an order's size (its D6) as 048a-5b's did, saying where it exceeds one;
   - 048a-5d, refreshed and ready, unmerged, and its cut 048a-5d2, IFC-049's URL match, stay as
     they are; and
   - the Done-when line that ruling 86, point 4, gives 048a-5e, that a plain hook's URL request is
     relayed, the hook part kept in progress and not ended unknown, is 048a-5ef's, and the line
     it gives the order that builds IFC-049's URL match is 048a-5d2's.

   The coordinator cut 048a-5d2 from 048a-5d at its refresh; its row in the work orders' index
   lands with that refresh.
3. **The path to the Mac dogfood.** From the two answers and the rulings before them, the path is:
   order 209 (ruling 86's engine half, point 4), after order 207; 048a-5d; 048a-5d2; 048a-5ef;
   048a-5gh; 048b-join; then 051b-2, `conduit sign-in`, the client the live check uses (order
   117). Order 050n, the Agent packages 0.2.0 joined to `main`, follows 048b-join and the live
   check ([ruling 82](../ruling-82/README.md), points 1 and 3), and a dogfood slot on the Mac
   signs in after 051b-2 (ruling 85, point 5). The path as a list is the coordinator's, read from
   these answers, from order 117 and from rulings 82, 85 and 86, and adds no decision of its own.

## What it amends

- **Ruling 86, "What it does not change"**, its line that orders 048a-7, 048a-8 and 051b-3 stay on
  0.2.0's path: they leave it and the dogfood path and move after dogfood, beside the keyed release
  (point 1). Ruling 86's decisions are unchanged.
- **The coordinator's cuts in order 117**, each open to the owner's strike until now: of 048a-5,
  whose 048a-5e to 048a-5h become 048a-5ef and 048a-5gh (point 2); and of 048a-2's entry and
  048a-6, under which 048b-join came after 048a-8, the last part of 048a-2's entry, and which put
  048a-7, 048a-8 and 051b-3 on 0.2.0's path (point 1). These two changes are the owner's, so no
  longer open to strike as the coordinator's.
- **The lines that put 048a-7, 048a-8 and 051b-3 on 0.2.0's path**: order 117's line "048b-join:
  S-M. After 048a-8, the last part of 048a-2's entry, and 048b-relay2", and the "Where it sits"
  of orders 048a-5, 048a-6, 048b-join, 051b, 051b-2 and 051b-3, which follow it. They change
  later, by orders (below).
- **FR-264's one-behavior rule**, for 048a-5ef and 048a-5gh alone (point 2).

## What it does not change

- **Ruling 82, points 1 and 3**: the live check, after 048b-join and before 0.2.0 merges; 0.2.0
  merges after 048b-join; each slot's sign-in follows 051b, on the Mac after 051b-2 (ruling 85,
  point 5).
- **Ruling 85, point 5**: 051b-3 still comes before any sign-in that needs a pasted return, and
  050o still follows 051b-3.
- **Ruling 86**: the Agent items stay plain in 0.2.0; option D in a later release, with its engine
  half, order 209, in 0.2.0; the four changes to `spec/`; and the account check's route.
- **`spec/`**: PND-013, PND-014 and IFC-049's return, which these orders build later.
- **The scope guard** in IN_FORCE.md, topic 1, and the tracker's: this ruling is the explicit owner
  ruling it asks for, recorded in topic 13, by the coordinator's reading (point 1).
- **Ruling 78, point 1, part 2**, by the coordinator's reading (point 1).
- **A77's registration** and its chain, by the coordinator's reading (point 1).
- **The fresh read before every merge** (ruling 79, point 2, as rulings 85 and 87 amend it), for
  the pairs as for every order.

## What is left for the owner

- Whether PND-013's first check alone, the not-accepted refusal with `signInLogin` returned:
  refused, joins 048a-5ef so that 0.2.0 refuses a return explicitly (point 1). It is not moved
  unasked, since the owner deferred 048a-7 and 048a-8 whole.
- 050o's place: it follows 051b-3 by the coordinator's plan, so with point 1 it now follows
  dogfood; whether it stays tied to 051b-3 is the owner's.
- Whether the S1 rerun's pin, and with it the lane's restart, still waits for 048a-7, 048a-8 and
  051b-3, and so for the keyed release's stage, or is taken at dogfood without them (point 1;
  ruling 78, point 1).

## Facts recorded with it

- The owner's time: one dialog of two questions, on 2026-11-19 as the coordinator's notes date
  them. The notes record no times.
- The savings the options named, about 6 hours on the critical path for answer 1 and about 5 hours
  for answer 2, are the coordinator's estimates when the questions were put; the second question
  put each order's cost at about an hour's fresh read and a gate.
- When the owner answered, order 048a-5d had been refreshed and was ready and unmerged, and order
  209's text was written and waited to be dispatched after order 207.

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md): topics 9 and 11 (point 2, in the lines naming the orders
  that build the sign-in hook part, the relay and polling); and topic 13 gains a line for the path
  to the Mac dogfood, the pairs and the carried return's orders moved, with the coordinator's
  readings of point 1.
- [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes): A78's row states the path to the Mac
  dogfood and that the carried return's orders follow dogfood.
- [docs/AUDIT.md](../../AUDIT.md#waiting-for-the-owner): Waiting for the owner gains a row for the
  three questions left for the owner, and IN_FORCE.md topic 13 an *Open* line for them.
- [The ledger](../ruling-log.md) gains row 88, and ruling 86's status, there and in its record,
  notes the amendment of its "What it does not change".
- Later, by orders, and not in this change: order 117's entry and the work orders' index replace
  048a-5e to 048a-5h with 048a-5ef and 048a-5gh, put 048b-join after 048a-5gh, and mark 048a-7,
  048a-8, 051b-3 and 050o after dogfood, beside the keyed release, citing this ruling;
  engine/CONFORMANCE.md (DL18's row and the paragraph on IFC-010's relay) and row 51 of
  docs/proposals/first-build/to-review-with-the-owner.md name 048a-5ef in place of 048a-5e and
  048a-5f, with the order that builds it; the engine's diagnostics and comments that name order
  048a-5e (in engine/crates/core/src/sign_in.rs, requests.rs and lib.rs) name 048a-5ef, in the
  order that next changes them; the section of engine/CONFORMANCE.md on order 048a-6 gains a
  dated note that 048a-7 and 048a-8, the orders that build PND-013's checks, follow dogfood
  (point 1); 048a-5ef and 048a-5gh are written; 048b-join is refreshed to join the URL relay and
  polling only, its return leg moved to 048a-7 and 048a-8; 051b-2 is refreshed before dispatch,
  its assumptions B4 and B5 naming 048a-5ef and 048a-5gh, and its line on the orders before it
  and its PollScript no longer resting on 048a-7, 048a-8 or 048b-join's return leg; and the
  "Where it sits" of orders 048a-5, 048a-6, 051b, 051b-2 and 051b-3 gain a dated note citing
  point 1. When the keyed release is planned, 048a-7, 048a-8 and 051b-3 are refreshed against
  what merged before them.
