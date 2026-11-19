# Ruling 85: clean reads merge on the gate, a diff-only reread, a one-round Codex trial, readers' tests on the Linux node, and Mac slots signed in after 051b-2

Status: in force; done (point 3: ruling 87); amended by 87 (point 3: one Codex round for every work order, and a first verifier's must no longer starts a second round; point 1, narrowed, as the coordinator reads 87 point 2: a read counts as a read with a must where the Codex verifier reported one) (ruled by the owner on 2026-11-18 or 2026-11-19, as the coordinator's notes date the answers). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topic 13 (point 5, with ruling 82, point 3) and topic 15 (points 1, 2, 4 and 5, point 1 as narrowed (the coordinator's reading of 87 point 2)), the coordinator's row and step 8 in [work orders](../../implementation/WORK_ORDERS.md#roles), and A78's row of [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes). This record says why.

On 2026-11-18 the owner asked: "Is there any way to speed all this up now that we have more
hardware? Should we dispatch a fable agent to analyze our working pattern and efficiency, as well
as what the most efficient way to work on conduit using conduit is?"

The coordinator read that as a review the owner asked for, run on Claude Fable, of the working
pattern and its efficiency and of the most efficient way to work on Conduit through Conduit. [Ruling
79](../ruling-79/README.md), point 1, keeps Fable for the owner's reviews of the whole language,
design, codebase and architecture, started by hand; for this review, of the work, the owner asked
for Fable. That this review sets no standing rule, as the targeted review of [ruling
81](../ruling-81/README.md), point 2, set none, is the coordinator's reading.

The review measured the work-order pipeline from 2026-11-17 to 2026-11-18. Its record is private
and is not in the tree. The findings that the decisions below rest on are given here as the
coordinator's summary of it, not as measures this tree holds:

- Finished work waited a median of about 24 minutes for a read.
- Codex's rounds after the first took about 44 % of Codex's time (its second round alone about
  40 %), and the second round's verdicts decided nothing: 23 of 25 still ended with musts while
  the fresh reader found 0 to 3.
- About 9 of the 23 musts the fresh readers found came from Codex's second round rewriting text
  that the order prescribed word for word.
- The serial chain stepped about every 2 hours 40 minutes.

The coordinator then put four questions to the owner through the session's question tool, each
with options and a recommendation, and the owner chose an option or options in each. The
questions as put, the options as the coordinator's notes hold them and the owner's answers
follow, verbatim. The notes date them 2026-11-18/19 and record no time for any of them.

## The questions

1. As put: "Ruling 79 point 3 has me merge each order after its fresh read. Finished reads waited
   a median 24 min for me. May a clean read merge itself?"

   The owner chose "Clean reads merge (Recommended)", whose text read: "A workflow merges an order
   on the gate's PASS when its fresh reader found no must; I review each merge after and handle
   any must. Reads with a must still wait for me. Amends ruling 79 point 3."

2. As put: "Which checking changes may I make? (Ruling 79's fresh read before every merge stays in
   every case.)"

   The owner chose two options: "Diff-only reread (Recommended), One-round Codex trial
   (Recommended)". Their texts read, for the first: "When the first read finds no must and only
   shoulds are applied verbatim, the reread checks just those edits (6–8 min) instead of a full
   second read. Narrows ruling 79 point 2."; and for the second: "On three engine orders (none
   that A77 pairs), Codex runs one round when its verifier finds no must, and I compare the
   reader's musts with the two-round baseline before adopting it."

3. As put: "Which machine changes are OK?"

   The owner chose three options, which the notes hold by label only: "Readers test on Windows
   (Recommended), WSL 24 GB + PC stays on (Recommended), Gate compile cache (Recommended)".

4. As put: "Ruling 82 point 3 has each dogfood slot signed in after 051b. I logged that as after
   051b-3 (the pasted return). On the Mac, browser logins don't need a pasted return. May Mac
   slots sign in after 051b-2 instead?"

   The owner chose "Yes, after 051b-2 (Recommended)".

For questions 1, 2 and 4 the notes hold only the options the owner chose, and no other option.

## The owner's answers

The answers are this ruling, as ruling 67 has it. What each decides, below, is the coordinator's
reading where it says so. "Windows" and "PC" in the third answer's labels name the second
machine, written here as the Linux node, whose Linux runs under WSL; that is the coordinator's
reading of the labels.

## The decisions

1. **An order whose fresh read finds no must merges through the gate without waiting for the
   coordinator** (answer 1). Ruling 79, point 3, reads: "The coordinator's session runs on Claude
   Opus: it plans, writes and dispatches the orders, decides, and merges." That is amended. Once
   an order's fresh read under ruling 79, point 2, finds no must, the order merges on the local
   gate's pass ([ruling 80](../ruling-80/README.md), point 10) without waiting for the
   coordinator. The coordinator reviews each such merge after it, and handles any must it finds
   there. A read with a must waits for the coordinator, as now. By the coordinator's reading:
   - a gate that does not pass merges nothing, and the order waits for the coordinator;
   - a must found in the review after a merge is fixed in a follow-up, as a run of CI that fails
     after a merge is (ruling 80, point 10);
   - the merge is made under the admin session's merge authority, which does not move (rulings 48
     and 71, point 1, as ruling 80, point 10, amends it);
   - the order's Record, which the admin writes at merge, is written with that review;
   - an order whose first read found a must is merged by the coordinator even after a clean
     reread; and
   - an order whose clean read leaves shoulds merges once their diff-only reread (point 2) finds
     no must.
2. **After a read with no must, a reread of applied shoulds checks only those edits** (answer 2,
   its first option). Ruling 79, point 2, reads: "Each order, once its checks pass, is read by a
   fresh, read-only Claude Opus reader before it merges, beside the fresh Codex verifier that
   reads each round. The coordinator's own fixes to an order are read the same way." That is
   narrowed. Where a fresh read finds no must and only shoulds are applied, each as its finding's
   fix field gives it, the reread checks only those edits and not the whole order again. The
   fresh read before every merge stays, in every case, as the question said. That "applied
   verbatim", in the option's text, means applied as the fix field gives it, and that any other
   edit, or any must, brings a full fresh read as before, is the coordinator's reading.
3. **A trial of one Codex round** (answer 2, its second option). On three engine orders that A77
   does not pair, Codex runs one round when its first verifier finds no must. The coordinator
   compares the musts the fresh reader finds on those orders with the two-round baseline before
   it adopts one round. The trial avoids A77's paired orders because
   [ruling 80](../ruling-80/README.md), point 6, makes A77's result the owner's. By the
   coordinator's reading:
   - a Codex verifier still reads every round (ruling 79, point 2), and a must from the first
     verifier still starts the second round;
   - the two-round baseline is the musts that fresh readers found on orders run with two Codex
     rounds, from the review's window; and
   - the trial's result goes back to the owner with the adoption, which the owner may strike, as
     ruling 80, point 6, has A77's result come to the owner.
4. **The machines** (answer 3).
   - Readers may run their tests on the Linux node.
   - The gate's full run on the Mac still comes before every merge, and so does a targeted run on
     the Mac for an order built on the Linux node (ruling 80, point 10).
   - The owner sets the Linux node's WSL memory to 24 GB, and keeps the computer on, or on a wake
     schedule.
   - The merge gate gets its own compile cache, started when the owner logs in on the Mac, and
     never shared with sandboxed runs.

   The notes hold only the three labels, so these details are the coordinator's: the targeted
   run on the Mac, the wake schedule as an alternative to staying on, that the cache is the
   gate's own, and the cache's start at login and its separation from sandboxed runs. That a
   reader stays read-only in its own tree while its tests run on the Linux node, and that its test
   runs there are ordinary work-order runs, which the coordinator may already run there over SSH
   (rulings 50, 76, point 6, and 77, point 2), so that this amends none of them, is the
   coordinator's reading too.
5. **Dogfood slots on the Mac sign in after 051b-2** (answer 4). [Ruling
   82](../ruling-82/README.md), point 3, has "each slot's sign-in follows 051b". The coordinator
   logged that as after 051b-3, the last of the three parts it cut 051b into, in [order
   117](../../work-orders/117-plan-s2-s3.md) ("A slot's sign-in under ruling 82, point 3, follows
   051b-3") and in [order 051b](../../work-orders/051b-cli-pending-and-answer.md) ("a slot's
   sign-in follows all three parts"). That reading is narrowed: a dogfood slot on the Mac signs
   in after 051b-2, `conduit sign-in`, since on the Mac a browser login needs no pasted return.
   051b-3 still comes before any sign-in that needs a pasted return. By the coordinator's reading:
   - ruling 82, point 3's own words, "follows 051b", are unchanged, and only the coordinator's
     reading of them is narrowed;
   - no Claude Agent becomes a slot before D58's harness change is in place, as before;
   - a sign-in that needs the managed connection of order 050o (ruling 80, point 8) still waits
     for it, and 050o still follows 051b-3; and
   - nothing else in the chain moves for this point.

## What it amends

- **Ruling 79, point 3**: the coordinator no longer merges an order whose fresh read found no
  must; it reviews that merge after it (point 1). It still plans, writes and dispatches the
  orders, decides, and merges every order whose read found a must, which, as the coordinator
  reads point 1, includes an order whose first read found a must and whose reread was clean.
- **Ruling 79, point 2**, narrowed: after a read with no must, a reread of shoulds applied
  verbatim (as the coordinator reads it, as their fix fields give them) checks only those edits
  (point 2). The fresh read before every merge stays.
- **Ruling 82, point 3, as the coordinator read it**, narrowed: a slot on the Mac signs in after
  051b-2, not 051b-3 (point 5).

## What it does not change

- **Ruling 79, points 1, 4 and 5**: Fable stays for the owner's reviews of the whole, started by
  hand; when the Claude allowance runs low, A77's paired reads stop first and dispatch next
  (point 4 as ruling 80, point 6, amends it), and nothing merges unread. A clean read is a read,
  so a merge under point 1 is not unread.
- **Ruling 80, point 6**: A77's paired orders, its question and its result, which is the owner's.
  The trial of point 3 runs on no paired order.
- **Ruling 80, point 10**: the local gate runs on the coordinator's machine, the Mac, and every
  merge waits for it; GitHub's CI is not waited on.
- **The merge authority** of rulings 48 and 71, point 1, and the lanes from the S1 rerun's pin
  (ruling 78, point 1).
- **Ruling 48, point 4, and ruling 71, point 3**: fresh reads by Codex still test what a reader
  without context can do.
- **Ruling 82, point 3's other terms**: order 050 is written now, and 0.2.0 merges after
  048b-join. **Ruling 78, point 1, part 2**: the pin waits for the six sign-in orders, and the
  owner signs each dogfood Agent in once.
- **Ruling 84**: per-Agent homes stand in for isolation until a sandbox method exists.

## Facts recorded with it

- The owner's time: four answers, on 2026-11-18 or 2026-11-19 as the coordinator's notes date
  them. The notes record no times.
- The owner's actions under point 4: setting the Linux node's WSL memory to 24 GB, and keeping
  that computer on or on a wake schedule. This record does not say whether they are done.

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md): topic 15's line on who carries out, reads and merges work
  orders states points 1 to 4; topic 13's line on order 050 and topic 15's line on Conduit building
  Conduit state point 5.
- [Work orders](../../implementation/WORK_ORDERS.md#roles): the coordinator's row says that an
  order whose fresh read found no must merges through the gate, and that the coordinator reviews
  that merge after it; and step 8 says that an order merged on a clean read gets its Record with
  that review.
- [The handover](../../implementation/HANDOVER.md#roles): the admin session's row says the same
  of its merges.
- [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes): A78's row says that a slot's sign-in
  on the Mac follows 051b-2.
- [The ledger](../ruling-log.md) gains row 85, and the status of rulings 79 and 82, there and in
  their records, notes this ruling's amendments.
- Later, by an order, and not in this change: [the method as
  run](../../implementation/METHOD_AS_RUN.md) gains the merge on a clean read, the diff-only
  reread, the trial's one round and the readers' test runs on the Linux node; the merge workflow
  and the gate's own compile cache are built; and order 117's line on a slot's sign-in, order
  051b-3's line on it, and order 051b's as a dated note, each gain a dated amendment citing
  point 5, saying that a Mac slot signs in after 051b-2 (ruling 85, point 5). Which three orders
  carry the trial, and how their A77 records mark it where they are among A77's 20 orders, are
  the coordinator's, logged when the trial starts.
