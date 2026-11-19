# Ruling 87: every work order runs one Codex round, the Opus reader judges the verifier's musts, and the one-round trial is closed

Status: in force (ruled by the owner on 2026-11-19, as the coordinator's notes date the answer). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topic 15 (points 1 to 4), and the coordinator's row and the section on dispatch and checking in [work orders](../../implementation/WORK_ORDERS.md#roles). This record says why.

[Ruling 85](../ruling-85/README.md), point 3, set a trial: "On three engine orders that A77 does
not pair, Codex runs one round when its first verifier finds no must. The coordinator compares the
musts the fresh reader finds on those orders with the two-round baseline before it adopts one
round." By the coordinator's reading recorded there, a must from the first verifier still started
the second round, and the trial's result was to go back to the owner. The review behind ruling 85
had found, by the coordinator's summary of its private record, that about 9 of the 23 musts the
fresh readers found came from Codex's second round rewriting text that the order prescribed word
for word.

## The trial's result

The trial ran on three orders of the sign-in chain, which A77 does not pair. What each Record on
`main` says:

- **Order 048a-6** (#386, [its Record](../../work-orders/048a6-build-sign-in-return-not-taken.md#record))
  ran two Codex rounds, since its first verifier found four musts. Round 2 changed the shell's two
  files alone, taking `server.rs`'s diff from 6 added lines to 123 added and 23 removed. The fresh
  Opus reader's only must came from that round: it "grew the shell well past decision 8, which
  decision 9 forbids". The fixer restored the two files as round 1 left them, and the reread found
  no must.
- **Order 048a-5b** (#387, [its Record](../../work-orders/048a5b-build-sign-in-account-first.md#record))
  ran two Codex rounds, since its first verifier found seven musts. Round 2 departed from the
  order's prescribed text. The fresh readers found four musts, all from those departures, then one,
  then none. The coordinator accepted the departures as amendments to the order, each grounded in
  the specification and open to the owner's strike.
- **Order 048a-5c** (#397, [its Record](../../work-orders/048a5c-build-sign-in-fixed-port.md#record))
  ran one Codex round, since its verifier found no must. The fresh reader found none either.

Order 048a-5c's merge commit (#397), not its Record, says that a diff-only reread of the reader's
two shoulds, applied, passed under ruling 85, and that the coordinator merged the order on the local
gate, taking its departures at the merge.

The coordinator's comparison, from these Records and not a measure this tree holds: on the two
orders whose first verifier found a must, all five musts of the first fresh reads came from Codex's
second round, against about 9 of 23 in the review's window; on the order run in one round, the
fresh read found no must.

## The question

The coordinator put one question to the owner through the session's question tool, with options
and a recommendation. The question as put, the option the owner chose and the labels of the
others follow, verbatim, as the coordinator's notes (private) hold them. The notes date it
2026-11-19 and record no time.

As put: "The one-round trial of Codex orders (048a-6, 048a-5b, 048a-5c) is done. When round 1 was
clean, one round sufficed; when Codex's verifier found musts, its second round made things worse
both times (scope growth, departures from prescribed text). How should orders run from now on?"

The owner chose "One round; Opus fixes musts (Recommended)", whose text read: "Every order runs one
Codex round. If the verifier finds musts, skip Codex's round 2: the fresh Opus read gets the
verifier's report too, and the Opus fixer applies the musts it confirms, as it already does for
reader musts. Saves ~25 min per order and avoids round-2 drift. A77-paired orders keep their
registered round 2."

The other options were "One round unless a must" and "Back to two rounds". The notes hold them by
label only.

## The owner's answer

The answer is this ruling, as ruling 67 has it. What it decides, below, is the coordinator's
reading where it says so. The "~25 min per order" is the option's estimate, not a measure.

## The decisions

1. **Every work order runs one Codex round: a worker and a fresh Codex verifier.** Codex runs no
   second round. By the coordinator's reading:
   - a round 1 resumed after an interruption, such as a pause of the machine it runs on, is still
     round 1;
   - a fix after a rebase goes to the Opus fixer, as every other fix does, and not to a Codex round;
     and
   - a new order that replaces or finishes an order, as the method's outcomes after a read allow,
     runs its own one round.
2. **Where the verifier reports a must, the fresh Opus reader judges the verifier's findings.** The
   fresh Opus reader of [ruling 79](../ruling-79/README.md), point 2, reads the verifier's report
   and confirms or refutes each of its findings. The Opus fixer applies the confirmed musts with the
   reader's own. By the coordinator's reading, the read then counts as a read with a must under
   ruling 85, point 1, so the order waits for the coordinator, which merges it. Also by the
   coordinator's reading:
   - this holds wherever the verifier reported a must, even where the reader refutes every one of
     them and finds none of its own, so that the coordinator sees each refutation before the merge;
   - a should or nit of the verifier that the reader confirms is handled as the reader's own shoulds
     and nits are;
   - the fixer's change is read by a fresh Opus reader in full, as the coordinator's fixes are under
     ruling 79, point 2, and not by ruling 85, point 2's diff-only reread, which follows only a read
     with no must; and
   - an order whose verifier reported a must is merged by the coordinator even after a clean
     reread, as ruling 85, point 1, has it, in the coordinator's reading, for a first read with a
     must.
3. **A77's paired orders keep their round 2.** An order that A77 pairs keeps the round 2 that A77's
   registration prescribes ([work orders](../../implementation/WORK_ORDERS.md#the-paired-read)): it
   is dispatched for round 1 alone, and round 2 is started by the continue runner with the
   coordinator's judged findings. By the coordinator's reading, such an order runs no Codex round
   after that round 2, and a must that round 2's verifier reports goes to the fresh Opus reader and
   the Opus fixer as point 2 has it. Also by the coordinator's reading, on such an order only the
   verifier of the last round, round 2, counts for point 2: round 1's verifier's findings are judged
   in the paired read and start round 2, as A77's registration has it, so a must of round 1's
   verifier alone does not make the order the coordinator's to merge.
4. **Ruling 85, point 3's trial is closed with this result.** One Codex round is adopted for every
   work order, not for engine orders alone, and the reading that a first verifier's must starts a
   second round no longer holds.

## What it amends

- **Ruling 85, point 3**: its trial is done, and every order now runs one Codex round (points 1 and
  4). Its reading that a must from the first verifier still starts the second round is replaced by
  point 2.
- **Ruling 85, point 1**, narrowed, as the coordinator reads point 2: a read counts as a read with
  a must where the Codex verifier reported a must, so such an order waits for the coordinator even
  when the fresh reader finds none. The narrowing is the coordinator's reading, not the owner's
  words, which say nothing of who merges.
- **Ruling 79, point 2**: each order has one Codex round for its verifier to read, and where that
  verifier reports a must, the fresh Opus reader also confirms or refutes each of its findings
  (points 1 and 2). The fresh Opus read before every merge stays.

## What it does not change

- **Ruling 48** gives verifications to Codex sessions, and **ruling 76**, point 6, lets ordinary
  work-order workers run on the Windows computer over SSH. That neither sets a number of Codex
  rounds, and that neither changes, is the coordinator's reading. A Codex verifier still reads each
  order's round, and fresh reads by Codex still test what a reader without context can do (ruling
  48, point 4).
- **Ruling 79, points 1, 3, 4 and 5**: Fable stays for the owner's reviews of the whole; the
  coordinator runs on Opus, and Codex or Opus implements; when the Claude allowance runs low, A77's
  paired reads stop first and dispatch next, and nothing merges unread.
- **Ruling 80, point 6**: A77's registration, its question, its paired read and its result, which
  is the owner's, as amended on 2026-11-19. By the coordinator's reading, A77's orders that are not
  paired run one round under point 1, and its result, which gives the rounds of each of A77's
  orders, and its baseline, whose orders ran up to two, stand as registered. A dated note in A77's
  section of
  [work orders](../../implementation/WORK_ORDERS.md#note-of-2026-11-19-one-codex-round) records
  this change of method; it is the coordinator's, open to the owner's strike, and changes none of
  A77's claims, orders, reads, records or verdicts.
- **Ruling 85, points 2, 4 and 5**, and point 1 for an order whose verifier reported no must.
- **The merge authority** of rulings 48 and 71, point 1, as ruling 80, point 10, amends it, and the
  local gate before every merge.

## Facts recorded with it

- The owner's time: one answer, on 2026-11-19 as the coordinator's notes date it. The notes record
  no time.
- The trial's three orders: their Records date their runs 2026-11-19, in UTC, and each is merged
  on `main` (#386, #387 and #397).

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md): topic 15's line on who carries out, reads and merges work
  orders states points 1 to 4, in place of ruling 85, point 3's trial.
- [Work orders](../../implementation/WORK_ORDERS.md#roles): the coordinator's row says that, as
  the coordinator reads point 2, it merges an order whose last Codex verifier reported a must; the
  section on dispatch and checking says that an order runs one Codex round and that the reader
  judges a verifier's musts; and A77's section gains a dated note of the change of method.
- [The ledger](../ruling-log.md) gains row 87, and the status of rulings 79 and 85, there and in
  their records, notes this ruling's amendments and the end of ruling 85, point 3's trial.
- [The handover](../../implementation/HANDOVER.md#roles): its admin row names no rounds, but it
  names what the coordinator merges, so, as the coordinator reads point 2, it adds an order whose
  last Codex verifier reported a must, and 87 point 2 (the coordinator's reading) joins its
  rulings. Its row on workers and verifiers, a Codex verifier that checks each round, stays true.
- No action item is added.
- Later, by an order, and not in this change: [the method as
  run](../../implementation/METHOD_AS_RUN.md) gains one Codex round (its `ROUNDS` variable, section
  3's rounds and section 6's reader, which confirms or refutes a verifier's musts), beside what
  ruling 85 left to it.
