# 028 — A78: stage 2, results of 2026-11-21
Not binding.

Every result here is obtained with every Agent under one operating-system user, the owner's
([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 2). No dedicated macOS user is
planned: ruling 84, point 1, amends [ruling 75 B3](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-75/README.md), and
isolation is to come later from a sandbox method, whose design states whether a result obtained
inside one shows Agents kept apart. Under one user, an Agent that runs code can read, use or
replace another Agent's login and act as root's Interface, so no result here shows Agents kept
apart ([the full review's D60](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/full-review/decisions.md)). A guard that held is
not proof either: the guards read a slot that the Agents they check can write
([finding 4-22](../../docs/implementation/first-build/log-increment-4.md)).

Each Agent keeps its client's login in a home of its own, which keeps the accounts apart for the
clients but is no wall against an Agent that runs code
([ruling 84](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-84/README.md), point 2): under one user, any Agent of
stage 2, `reader-1` among them, can read or use another's login, and each one's own tools reach
every slot. Each can also read what the owner's environment and shell profile hold, an API-key
variable among them, a reach the owner accepts for the dogfood
([ruling 89](https://github.com/shrav-k/conduit/blob/077fa47ecd4d2172d4f9204c6fcbf0f43a4e2b9d/docs/proposals/ruling-89/README.md), point 4), and stage 2 shows nothing about
whether an Agent used it. A sign-in that carried the stage shows that it did so on this store,
for this stage's length, not across a package's upgrade or the install of 0.2.1 (A78's P4, not
registered). An account check that answered the named account shows what the client said, not
which account the provider charged. One turn at a time shows nothing about turns at once: two
pairs show five Agents kept in one store and two slot clones used in turn, not two orders run
together. And no read of `reader-1` is paired with another reader's, so stage 2 shows nothing
about how well it reads.

## Verdict

No order of stage 2 passes, and stage 2's claims are not shown.

- Stage 2's claims hold when each of its first five orders passes
  ([its pass conditions](README.md#stage-2s-pass-conditions)). Those are 217, 218, 219, 222 and
  220, by dispatch. None passes.
- For every one of the 22 orders, condition 4 is not shown and condition 6 fails. Conditions 5,
  7 and 8 hold for every order, by the records, and condition 2 by the coordinator's session
  record. Conditions 1 and 3 also fail for 226, 232 and 236, and condition 9 for every order but
  217 and 219.
- Claim 1 is not shown, since its orders do not pass. Its own condition, 8, holds: the records
  show no sign-in action raised after the owner's five sign-ins.
- Claim 2 is not shown. `reader-1` gave each order its read through Conduit, but only 3 of its
  25 turns were the first of a new conversation, and no guard read its clone.
- 65 of the 104 turns were sent while another was in flight, which the registration does not
  allow without a dated amendment, and none was written ([below](#turns-at-once)). Every order
  but 236 has such a turn. Over the 39 turns sent with none in flight, the verdict is the same.

The registration asks each order for a directory with its scrubbed records export and its own
result ([Stage 2's orders](README.md#stage-2s-orders)). None was made. The export holds no message
or path. Its only private values are the five accounts that the sign-in answers' evidence names
(`pending-action-answered`, `config/account`), which the registration's placeholders replace. For
now this one file stands for the stage ([For the owner](#for-the-owner), item 3).

## The evidence, and its limit

The coordinator exported Conduit's records with serve stopped, at 14:44 PST on 2026-11-21, by
`conduit records --store <store>`: exit 0, stderr empty. The export, `<records>`, holds 1,704
CloudEvents, from the store's init at 23:45:05 PST on 2026-11-19 to 14:33:17 on 2026-11-21, the
end of a turn of orders 240 to 242: 310 access decisions, 385 dispatches, 385 invocation endings,
385 request endings, 234 object changes and 5 pending actions answered. Before it, serve had held
the store since 08:35, so `conduit records` exited 1 (`store already has an active shell`). The
export holds no message or path. Its only private values are the five accounts that the sign-in
answers' evidence names (`pending-action-answered`, `config/account`), which the registration's
placeholders replace. It is not committed, and nothing here quotes it.

So this file is built from the records and from what the coordinator kept:

- the records: each turn's dispatch, end, outcome, attempt, caller, target and access decision;
  each restart's setups and account checks; each sign-in action; every other request;
- each `conduit call`'s printed result, which gives the outcome, the invocation ID and the
  diagnostics, and its stderr;
- the `start` and `end` lines its shell printed just before and after each call, and which turn
  tool each call named, from its command history;
- its notes (private).

Each row of [the turn table](#the-turns) was checked against the records by invocation ID. All
104 were found, and in all 104 the outcome and the Agent's turn tool matched. The times differ
as a clock around a call would: the dispatch is 0 to 3 s after the clock's `start` (the same
second in 8 rows) and the end 0 to 2 s before its `end` (the same second in 6), so only rows 6 and
34 matched to the second. Every row now gives the records' dispatch and end, and its seconds
from them. The overlaps and fresh reads came out the same in every row. The records' milliseconds
also tell apart the six pairs the clock put in the same second ([Turns at once](#turns-at-once)).

The records do not give guards (Conduit has none), a turn's cause beyond its outcome, the
reports, the clones, the clients' versions, the account a check answered, or serve's own start
and stop. Each measure that needs these rests on the coordinator's files, as said, or is not
shown.

Placeholders, as the registration asks:

| Placeholder | Stands for | Class |
| --- | --- | --- |
| `<store>` | The dogfood store's directory | path |
| `<records>` | The coordinator's file of the records export | path |
| `<direct>` | The coordinator's directory of each call's message, arguments, printed result and stderr | path |
| `<tasks>` | The coordinator's directory of background command outputs | path |
| `<slot-1>`, `<slot-2>`, `<slot-r>` | The two slot clones and the reader's clone | path |

No name, account value or credential is in this file. Paths are the only redactions.

## The stage's bounds

- **The start.** [The amendment of 2026-11-20](README.md#amendment-of-2026-11-20-stage-2-directly-through-conduit)
  merged at 08:42:35 PST (pull request 444). By its own words, and because an order is "fixed as
  stage 2's at its dispatch", orders 215 and 216 are outside. Both were dispatched before the
  merge. Their five turns all succeeded:

  | Order | Agent | Role | Start | End | Invocation |
  | ---: | --- | --- | --- | --- | --- |
  | 215 | `worker-1` | worker | 11-20 07:28:44 | 07:47:35 | `7ebc71f5-bbb9-41e4-ab11-df33ba9cceb0` |
  | 215 | `verifier-1` | verifier | 11-20 07:48:19 | 07:58:40 | `e1d97cbb-e9ca-4a5b-b17e-a7d621d8644f` |
  | 215 | `worker-1` | fix 1 | 11-20 07:59:14 | 08:01:03 | `f89f6aca-7221-4d21-ac69-48c42d5bc236` |
  | 216 | `worker-2` | worker | 11-20 08:29:12 | 08:48:01 | `22f87d65-3614-4112-8433-cba6b5006df5` |
  | 216 | `verifier-2` | verifier | 11-20 08:48:55 | 08:55:53 | `f1cece81-8388-45d0-a2c1-dd46cf47258f` |

- **The end.** The owner ended stage 2 once orders 236 to 239 were in
  ([DEC-037](../../docs/decisions/log.md#dec-037-2026-11-21-the-post-dogfood-review)). Order 239
  merged at 10:59:18 PST on 2026-11-21 (pull request 494). Order 237's text was written, but its
  dispatch is held, so it sent no turn. The end is read as 239's merge, with 237 held. A reading
  that waits for 237 would make orders 240 to 242 stage 2's. Who held 237, and whether it counts
  as in, is the owner's to confirm ([For the owner](#for-the-owner), item 4).
- **The orders.** There are 22, in dispatch order: 217, 218, 219, 222, 220, 223, 224, 225, 221,
  230, 227, 229, 231, 228, 226, 232, 234, 233, 235, 236, 238 and 239. Between them they sent
  104 turns, from 09:43:41 on 2026-11-20 to 10:37:23 on 2026-11-21 by the records: 22 first
  worker turns, 35 fix turns, 22 verifier turns and 25 turns of `reader-1`. By Agent:
  `worker-1` 28, `worker-2` 29, `verifier-1` 11, `verifier-2` 11 and `reader-1` 25.
- **After the end.** From 12:55 on 2026-11-21, orders 240 to 242 had sent 15 turns on the same
  store by 14:31. They are not counted here. The export holds no other turn: its 124 are these
  15, the 104 and 215's and 216's 5.

## Turns at once

All counts here are from the records' dispatch and end times, to the millisecond.

- 77 of the 104 turns overlapped another turn: each of the two was dispatched before the other
  ended. 27 overlapped none.
- 65 were sent while another was in flight. 60 of them started inside another's span by the
  coordinator's clock as well. The other 5 are the later of the five pairs, of six that the clock
  put in the same second, in which no third turn was in flight: 3 was dispatched 0.50 s after 2,
  12 0.64 s after 11, 25 0.65 s after 24, 60 0.69 s after 59 and 97 0.75 s after 96. 36 and 37,
  also in one second by the clock, were both sent while 35 was in flight.
- 39 were sent with none in flight: 1, 2, 8 to 11, 13, 14, 15, 19, 20, 22, 23, 24, 26, 27, 33,
  40, 44, 59, 80 to 84, 87 to 96 and 101 to 104. 12 of them had another turn sent beside them
  while they ran. The clock alone gave 34, without 2, 11, 24, 59 and 96.
- At most three ran at once.
- The first overlap was turns 2 and 3: `verifier-1` on 217, dispatched at 10:25:11 on 2026-11-20,
  and `worker-2` on 218, half a second later.
- The registration sends one turn at a time until 0.2.1's `new-conversation.v1` is installed. It
  says "a turn sent while another is in flight is not stage 2's". Only a dated amendment merged
  before the first such turn can let turns run at once
  ([What stage 2 leaves unregistered](README.md#what-stage-2-leaves-unregistered)). No such
  amendment was written.
- The coordinator says it sent turns at once to follow the owner's "Both in parallel" (07:49 PST
  on 2026-11-20 by the coordinator's session record; ruling 92's record says about 02:40), whose
  option read "S2 on one lane and the dogfood gaps on the other, with the Mac and the Linux node
  each running one". That answer was about the next build priority. The coordinator also cites
  its own pace levers. It found the conflict on 2026-11-21, logged it in its notes (private) and
  told the owner.
- No serve restart was made while a turn was in flight.
- So the 65 turns sent in flight are outside the registration. Nothing about turns at once is a
  result here.
- Only order 236 ran with all its turns inside the registration; 239's turn 97 was sent while
  238's turn 96 was in flight. Order 236 still fails conditions 1, 3, 6 and 9, and condition 4 is
  not shown.

## Measures and pass conditions

Each condition is shown twice: over every turn of stage 2's orders, and over the 39 turns sent
with no other in flight, the registration's count: 1, 2, 8 to 11, 13, 14, 15, 19, 20, 22, 23,
24, 26, 27, 33, 40, 44, 59, 80 to 84, 87 to 96 and 101 to 104 ([the turn table](#the-turns)).

| Condition | Over every turn (104) | Over the turns sent alone (39) |
| --- | --- | --- |
| 1. No turn lost | Fails: 4 lost. Turns 89 and 90 (`reader-1`, order 236) ended `unknown`, in the records as in the call results. Turns 72 and 82 (`reader-1`, orders 226 and 232) left reports that are invalid, with no repair turn sent; a report left so is read as still invalid after its repair turn. | Fails: 3 lost, turns 82, 89 and 90. |
| 2. Set-up at most 240 s | Holds, by the coordinator's session record, not by the records, which hold neither serve's stop nor its start. There were 4 restarts, before turns 1, 2 and 3, 11 and 12, and 91. By the session record, the stop was a command of 8 s, then the start one of 21 s at 09:43 and 24 s at 10:24; at 12:19 and 08:35 one command did both, in 35 and 36 s. From each restart command's start to root's tools answering took at most 36 s. Root's tools answered 12 to 14 s after serve's start, polled every 2 s, before the setups ended, which the records put 15 to 20 s after the first setup's dispatch. Every other turn had no restart before it. | Holds, by the coordinator's session record, not by the records: 4 restarts (before turns 1, 2, 11 and 91), at most 36 s from each restart command's start to root's tools answering, and 12 to 14 s from serve's start. |
| 3. Reports valid, causes named | Fails. 100 of the 102 reports are valid against their schemas at the first try. `reader-1`'s reports on 226 and 232 lack `findings`, and no repair turn was sent. The cause of turns 89 and 90 is `client failure: other`, which names none. | Fails: 36 of 37 valid (232's reader); turns 89 and 90 the same. |
| 4. No guard broken or unknown | Not shown: Conduit has no guard ([the amendment of 2026-11-20](README.md#amendment-of-2026-11-20-stage-2-directly-through-conduit)), and the records hold no guard's answer. | Not shown. |
| 5. No turn recorded as not having occurred | Holds. None of the export's 385 `invocation-ended` records has `reconciled` true or the outcome `failed`, and no `reconcile.v1` was dispatched. Each turn had one attempt, which ended `succeeded` or `unknown`. | Holds, the same. |
| 6. Each friction in the open log before the next order | Fails. [The open log](../../docs/implementation/first-build/log-increment-4.md)'s last row is 4-37, from before stage 2. The stage's frictions (gaps 1 to 16, the lost turns, the reports left invalid, the moves of 224 and 226 between pairs) are only in the coordinator's notes (private). | Fails, the same. |
| 7. Every turn found in the export | Holds, with the coordinator's call results read as the log, since the amendment retired the harness. Each of the 104 calls' invocations is in the export with its caller, its Agent (the target's ID, one to one with the five turn tools), its dispatch, its end and its outcome, and the outcome and tool match the call's. | Holds, the same. |
| 8. No sign-in asked again | Holds. The records' 5 sign-in actions, one per Agent, were raised by the init's setups at 23:45 on 2026-11-19 and answered by the owner's sign-ins from 23:50 to 23:52. No record after them raises one, up to the export's last at 14:33 on 2026-11-21: no turn's end, and none of the 20 setups at the stage's 4 restarts, whose account checks all succeeded. No turn ended on a sign-in. | Holds, the same. |
| 9. Every read fresh | Fails: 9 of 47 reads were their Agent's first turn since a restart (verifiers 6 of 22, `reader-1` 3 of 25). | Fails: 8 of 23 (verifiers 6 of 13, `reader-1` 2 of 10). |

Over the clock's 34 turns sent alone, each value is the same but 2 (2 restarts, before turns 1
and 91, 12 and 14 s from serve's start), 3 (31 of 32 valid) and 9 (6 of 20: verifiers 4 of 11, `reader-1` 2 of 9).

Without `new-conversation.v1`, a turn sent with no restart before it goes on in its Agent's last
conversation, which keeps the earlier turns' context (gap 10 in the coordinator's notes). The
amendment reads condition 9 from Conduit's records. They show each restart as a setup of every
Agent, first dispatched at 09:43:10, 10:24:32 and 12:19:28 on 2026-11-20 and 08:35:43 on
2026-11-21, 2 to 9 s after the coordinator's restart commands, and no `new-conversation.v1`.
Read from them, the fresh reads are as the restart commands gave them. For `reader-1` the
package agrees. Its result reports the client's tools as `current` on turns 5, 15 and 91 only,
and as `session_init_unverified` on its other 20 turns that succeeded. Its lost turn 89 failed as
"client turn 18", which matches the 18 turns `reader-1` was sent from the restart at 12:19 on
2026-11-20 to that read.

Stage-level measures:

- **Restarts.** 4 in the stage: 09:43, 10:24 and 12:19 on 2026-11-20, and 08:35 on 2026-11-21,
  each a setup of all five Agents in the records. One more at 07:27 on 2026-11-20 came before
  order 215. Each took 12 to 14 s, as above.
- **Waits for a turn in flight.** None is logged. Instead, 65 turns were sent beside one already
  in flight.
- **Restarts and new conversations.** The registration counts restarts to show what a new
  conversation without a restart would remove. Here restarts were few because conversations were
  carried on instead. The cost fell on condition 9, not on set-up time.
- **Work seconds**, from each turn's dispatch to its end in the records. The total is 67331 s;
  over the 39 turns sent alone it is 18675 s (15311 s over the clock's 34). The medians of the
  turns that succeeded: first worker turns 1015 s, fix turns 572 s, verifier turns 286 s and
  `reader-1`'s turns 275 s.
- **`reader-1`'s turns.** Their seconds are in the turn table, with a median of 275 s over the
  23 that succeeded. No result reports tokens.
- **Access.** From turn 1's dispatch to turn 104's end the records hold 190 access decisions, and
  every one permitted. 104 are the turns' own, each `mediated` and naming its Agent's turn tool.
  The other 86 are the restarts' setups and the coordinator's lists and the 0.2.1 install's
  calls. None was refused. The store's 6 opens (`open.v1`) were at its init, at 23:45 on
  2026-11-19; none was in the stage, and nothing was ever closed. So no order tested access
  (D57).
- **Attempts and reconciliation.** Every turn had one attempt, and no `reconcile.v1` was
  dispatched. After turn 89 ended `unknown`, its Agent was sent a new turn request (90); after
  90, a restart and another (91). No field of the records links a new request to the turn it
  replaced.
- **Sign-in actions and account checks.** The records hold 5 sign-in actions, as condition 8
  says: raised at the init and answered at the owner's sign-ins, whose `sign-in.v1` took 10 to
  50 s each and succeeded; each one's `sign-in-hook.v1` part ended `unknown`. In the stage none
  was raised and none was pending after a restart. Each of the 20 setups at the stage's restarts
  ran its account check, which succeeded, and no `sign-out-hook.v1` undid an account. By the
  engine's rule a setup raises an action on any answer but the account its Agent names; the
  export does not carry the answer itself.

Measures not shown, for want of records or logs:

- the seconds spent readying each clone;
- each client's version at an order's first turn;
- the account each check answered (the export does not carry it);
- the store's size per invocation;
- the lines of the Fleet's documents changed for an order (the coordinator changed none for a
  turn).

Not counted for this file, though their sources exist: each round's seconds (the clock lines),
the musts the coordinator accepted (its notes) and each `reader-1` verdict (the reports).

## Per order

Each verifier or reader pass gives its musts, shoulds and nits (m/s/n), from its report. "Reads
fresh" counts the verifier and reader turns that were their Agent's first since a restart. Every
`reader-1` read ran through Conduit, so none ran outside Conduit in its place. "Sent alone" counts
the order's turns sent with no other in flight, by the records. "In export" counts its turns found
in the records with caller, Agent, dispatch, end and outcome. "Refused/opened/closed" counts the
access decisions not permitted and the `open.v1` and `close.v1` that succeeded, from the order's
first dispatch to its last end. No order raised a sign-in action.

All 22 orders merged (pull requests 453 to 494). Twenty merged on reads that were not fresh,
which condition 9 says nothing merges on. 226 and 232 merged on invalid reader reports with no
repair turn and no read outside Conduit. 236's lost read was sent again through Conduit, not
outside it as the registration asks.

| Order | Agents | Turns | In export | Sent alone | Verifier m/s/n | Reader m/s/n | Reports invalid | Lost | Reads fresh | Refused/opened/closed |
| ---: | --- | ---: | --- | ---: | --- | --- | --- | ---: | --- | --- |
| 217 | `reader-1`, `verifier-1`, `worker-1` | 5 | 5 of 5 | 2 | 2/1/0 | 2/2/1 | none | 0 | 2 of 2 | 0/0/0 |
| 218 | `reader-1`, `verifier-2`, `worker-2` | 6 | 6 of 6 | 3 | 3/0/0 | 0/4/0 | none | 0 | 1 of 2 | 0/0/0 |
| 219 | `reader-1`, `verifier-1`, `worker-1` | 5 | 5 of 5 | 4 | 2/0/0 | 0/1/2 | none | 0 | 2 of 2 | 0/0/0 |
| 222 | `reader-1`, `verifier-2`, `worker-2` | 4 | 4 of 4 | 2 | 10/0/0 | 0/3/0 | none | 0 | 1 of 2 | 0/0/0 |
| 220 | `reader-1`, `verifier-1`, `worker-1` | 4 | 4 of 4 | 3 | 0/1/0 | 0/0/0 | none | 0 | 0 of 2 | 0/0/0 |
| 223 | `reader-1`, `verifier-2`, `worker-2` | 4 | 4 of 4 | 2 | 4/0/0 | 0/2/0 | none | 0 | 0 of 2 | 0/0/0 |
| 224 | `reader-1`, `verifier-1`, `worker-1`, `worker-2` | 6 | 6 of 6 | 0 | 2/0/0 | 0/3/1 | none | 0 | 0 of 2 | 0/0/0 |
| 225 | `reader-1`, `verifier-2`, `worker-2` | 4 | 4 of 4 | 1 | 2/0/0 | 0/0/1 | none | 0 | 0 of 2 | 0/0/0 |
| 221 | `reader-1`, `verifier-1`, `worker-1` | 4 | 4 of 4 | 1 | 3/0/0 | 0/0/1 | none | 0 | 0 of 2 | 0/0/0 |
| 230 | `reader-1`, `verifier-2`, `worker-2` | 4 | 4 of 4 | 1 | 1/0/0 | 0/1/0 | none | 0 | 0 of 2 | 0/0/0 |
| 227 | `reader-1`, `verifier-1`, `worker-1` | 5 | 5 of 5 | 1 | 7/0/0 | 0/1/1 | none | 0 | 0 of 2 | 0/0/0 |
| 229 | `reader-1`, `verifier-2`, `worker-2` | 5 | 5 of 5 | 0 | 4/0/0 | 0/1/0 | none | 0 | 0 of 2 | 0/0/0 |
| 231 | `reader-1`, `verifier-2`, `worker-2` | 5 | 5 of 5 | 0 | 3/0/0 | 0/1/0 | none | 0 | 0 of 2 | 0/0/0 |
| 228 | `reader-1`, `verifier-1`, `worker-1` | 4 | 4 of 4 | 0 | 6/0/0 | 0/2/0 | none | 0 | 0 of 2 | 0/0/0 |
| 226 | `reader-1`, `verifier-2`, `worker-1`, `worker-2` | 5 | 5 of 5 | 0 | 4/0/0 | 0/0/0 | reader's | 1 | 0 of 2 | 0/0/0 |
| 232 | `reader-1`, `verifier-1`, `worker-1` | 5 | 5 of 5 | 3 | 3/0/0 | 0/0/0 | reader's | 1 | 0 of 2 | 0/0/0 |
| 234 | `reader-1`, `verifier-1`, `worker-1` | 4 | 4 of 4 | 0 | 0/1/0 | 0/1/0 | none | 0 | 0 of 2 | 0/0/0 |
| 233 | `reader-1`, `verifier-2`, `worker-2` | 3 | 3 of 3 | 0 | 2/0/0 | 0/0/1 | none | 0 | 0 of 2 | 0/0/0 |
| 235 | `reader-1`, `verifier-1`, `worker-1` | 3 | 3 of 3 | 1 | 1/1/0 | 0/2/1 | none | 0 | 0 of 2 | 0/0/0 |
| 236 | `reader-1`, `verifier-2`, `worker-2` | 9 | 9 of 9 | 9 | 1/0/0 | lost, lost, 1/3/1, 0/1/0 | none | 2 | 1 of 5 | 0/0/0 |
| 238 | `reader-1`, `verifier-1`, `worker-1` | 5 | 5 of 5 | 2 | 2/1/0 | 1/0/3 | none | 0 | 1 of 2 | 0/0/0 |
| 239 | `reader-1`, `verifier-2`, `worker-2` | 5 | 5 of 5 | 4 | 2/0/0 | 0/0/2 | none | 0 | 1 of 2 | 0/0/0 |

- On 217, `reader-1`'s 2 musts came from the coordinator's copy into `<slot-r>`. The copy was
  made without `--delete`, so three deleted files came back. The coordinator withdrew those
  musts.
- Orders 230 and 231 had their first turns before their texts were on `main`. 230's first turn
  was at 15:54 and its text merged with its build at 16:55. 231's first turn was at 16:51 and its
  text merged at 18:25. The registration puts an order's text on `main` before its first turn.
- Orders 224 and 226 moved from pair 1 to pair 2 mid-order. 224's fix 3 (turn 38) ran on
  `worker-2` in `<slot-2>`, with `worker-1`'s changes copied in. 226's first turn (57) was
  `worker-1`'s in `<slot-1>`, and its fixes (66, 70) and its verifier ran on pair 2. Every other
  order ran on one pair, its turns in that pair's slot and the reader's in `<slot-r>`. Both moves
  were the coordinator's interventions.
- In eight orders a worker's turn succeeded but stopped under the order's own stop rule, and the
  coordinator decided or widened the scope by hand: 218 (turn 3), 220 (21), 222 (16), 224 (28),
  226 (57), 228 (51), 229 (48) and 232 (68).
- The coordinator also changed an order's scope or text in the middle of the order, in 220, 223,
  224, 226, 227 and 231. It fixed files itself in 218, 222 and 228, and mirrored them into the
  slot where the worker went on.

## Turns lost, and other failures

- **Turn 89**: order 236, `reader-1`, 00:34:37 to 00:37:00 on 2026-11-21, by the records.
  - It ended `unknown` under IFC-010. The package's error reads: "client turn 18 failed: client
    failure: other; session cannot restart implicitly". Its `failedTurn` gives cause
    `client-failure`, class `other`, client started, stop confirmed, no survivors.
  - At 00:37:01 the coordinator's own Claude Code session on the same Mac printed "You've hit your
    weekly limit · resets 1am (America/Los_Angeles)". Three of its agents writing orders failed
    with the same words. The coordinator's notes put the limit at 00:35.
  - This file does not show that the coordinator's session uses the account `reader-1` names. So
    the excuse in condition 1 is not shown, and the turn counts as lost.
- **Turn 90**: the same read, dispatched again at 08:33:17, after the owner's next message.
  - It ended `unknown` within 1 s: "session failed at turn 18: client failure: other; implicit
    restart refused".
  - The package refuses every later turn of a failed conversation (open log, 4-21). Only a restart
    of serve gave `reader-1` a new conversation, and that restart ends every Agent's conversation.
  - Serve was restarted at 08:35:34, with all Agents idle and `conduit pending` empty. The read
    then ran fresh as turn 91, from 08:36:14 to 08:40:59, and succeeded.
- **Turns 72 and 82**: `reader-1` on 226 and 232. Both succeeded, but their reports lack
  `findings`, and no repair turn was sent. Each counts as lost under condition 1.
- **A call not made**, at 00:30:11 on 2026-11-21: the coordinator's first call of 236's fix turn.
  - The command defined its directory variable and then sourced the coordinator's variables file,
    which defines the same name. The arguments path then pointed at a file that does not exist.
    `cat` failed, and `conduit call` got an empty argument and exited 1.
  - By the CLI's code, arguments that are not JSON are refused before serve is reached, so no
    request was made. The records agree: they hold nothing between turn 87's end at 00:29:54 and
    turn 88's dispatch.
  - The call was sent again at 00:30:27 as turn 88, and succeeded.
- No turn ended `failed`. Every other turn succeeded.

## Manual steps

Each is a gap in the coordinator's notes (private), to be ordered as engine and package work
(ruling 92, points 2 and 9).

1. Serve was started by hand under `env -i`, with a PATH of tool directories and a call timeout
   of 7200 s. This happened 5 times, once before the stage (gap 1).
2. Before each order, each slot and the reader's clone were brought to `main` by hand, as a fresh
   clone with the build cache kept (gaps 2 and 11). The coordinator's command history holds 48
   commands that made such a clone in the stage: 13 of `<slot-1>`, 13 of `<slot-2>` and 22 of `<slot-r>`. One copy
   into `<slot-r>` left deleted files behind (order 217). One set of clones came from a stale
   local `main`, and 222's start check stopped it.
3. A worker and its verifier share one directory. The verifier is asked to stay read-only by its
   message alone (gaps 3 and 8).
4. Each turn's order, rules and report schema were pasted by hand into `turn.v1`'s message
   (gap 4).
5. Each worker's changes were copied by hand into a fresh trusted worktree, leaving out `.git` and
   build output. The gate ran there (gap 5).
6. A fresh conversation needs a restart of serve, which ends every Agent's conversation. There
   were 4 such restarts (gap 11).
7. At 12:14 on 2026-11-20 the coordinator installed the 0.2.1 archives and replaced the Codex
   verifier template with 0.2.1. It put the template back by 12:17. Instances keep the template
   they resolved, so upgrading means replacing each instance and signing each one in again
   (gap 13).
8. Merges conflicted on the order index and on the gate's engine list, and the conflicts were
   resolved by hand, more than a dozen times, until order 238 (gaps 14 and 15).
9. The weekly limit's dead session needed a restart of serve (above).
10. Turns ran at once with no amendment (gap 16, above).

## Owner sittings

- 2026-11-19, about 23:00 to 23:35 PST: experiment 027's live check and the Claude scratch
  sign-in, before stage 2.
- 2026-11-20, just after midnight: the five sign-ins, in one sitting, as the amendment of
  2026-11-20 records. The records put them earlier: `sign-in.v1` dispatched from 23:49:40 to
  23:52:22 PST on 2026-11-19, the last ending at 23:52:32.
- None during the stage's turns.
  - On 2026-11-20 the owner moved the 0.2.1 sign-ins to the second-account sitting.
  - On 2026-11-21 the owner held the S1 rerun's sitting
    ([DEC-038](../../docs/decisions/log.md#dec-038-2026-11-21-the-s1-rerun-waits-for-the-simplification-chain)).
  - The owner answered questions in chat throughout the stage. On 2026-11-20 the owner struck the
    coordinator's reading that `reader-1` waits for a guard (the coordinator's notes, private).
    The amendment of 2026-11-20 leaves exactly that to the owner.

## Stops and pauses

- **A pause.** The Claude weekly limit stopped the coordinator's session at 00:37 on 2026-11-21;
  its message gave the reset as 1am. No turn was sent until 08:33, after the owner's next message
  at 08:31.
- **Stop rule 4: two turns in a row lost.** By this file's reading of condition 1, the rule's
  condition was met: turns 89 and 90 were both lost on `reader-1`, and 90 to the package's
  refusal to restart a failed conversation. No stop was declared or lifted under the rule, and
  turns 91 to 104 were sent after it. The coordinator restarted serve and went on, and told the
  owner of the limit that morning. If 89 was the allowance's end, it was a pause; that is not
  shown.
- **Stop rule 1** had no guard to answer it.
- **Stop rules 2, 3 and 5.**
  - No sign-in was asked again: the records show no sign-in action raised in the stage. Every
    setup's account check succeeded and raised no action; the export does not carry the account
    each answered.
  - Serve came back within 14 s each time, by the coordinator's session record.

## For the owner

1. **The 0.2.1 install.** It happened at 12:14 on 2026-11-20, between turns 10 and 11. No instance
   left 0.2.0, and no turn was in flight. The registration ends stage 2 at "an upgrade of the
   store's Agent packages" unless an amendment says otherwise. Is this such an upgrade? If it is,
   stage 2 ended there, and turns 11 to 104 are outside as well.
2. **Order 235.** Its pull request (488) changed this directory: the coordinator pinned the links,
   outside every turn
   ([the amendment of 2026-11-21](README.md#amendment-of-2026-11-21-links-after-work-order-235)).
   The registration says no order that changes this directory goes through Conduit in stage 2.
3. **The records.** They were exported with serve stopped at 14:44 on 2026-11-21, and this file
   is checked against them. The registration asks for each order's directory with its scrubbed
   export. The export's only private values are the five accounts the sign-in answers name. With
   those five values replaced, should the export, split by order, be committed as the
   registration asks?
4. **The end.** [DEC-037](../../docs/decisions/log.md#dec-037-2026-11-21-the-post-dogfood-review)
   ends stage 2 "once orders 236 to 239 are in", and 237 sent no turn. This file reads the end as
   239's merge, with 237 held. If 237 must be in first, orders 240 to 242 are stage 2's. Who held
   237, and does it count as in?

## The turns

Times are PST: a turn's start is its dispatch in the records and its end its end there, to the
second; within one second the order is the records' milliseconds. "Seconds" is end minus
dispatch, from the milliseconds, rounded. "Overlaps" lists the other turns whose span crossed
this one's. "Read fresh" says, for a verifier or reader turn, whether it was its Agent's first
turn since a restart; if not, which turn of that conversation it was.

| # | Order | Agent | Role | Start | End | Seconds | Outcome | Invocation | Overlaps | Read fresh |
| ---: | ---: | --- | --- | --- | --- | ---: | --- | --- | --- | --- |
| 1 | 217 | `worker-1` | worker | 11-20 09:43:41 | 10:15:52 | 1932 | succeeded | `327e8503-36cc-486a-a36e-3da0a7918892` | none |  |
| 2 | 217 | `verifier-1` | verifier | 11-20 10:25:11 | 10:31:37 | 386 | succeeded | `8de5846b-0b9d-4298-8811-d8134e88e6bb` | 3 | yes |
| 3 | 218 | `worker-2` | worker | 11-20 10:25:11 | 10:51:45 | 1593 | succeeded | `a36cda9b-8920-4fee-bedf-5ac776a385db` | 2, 4, 5 |  |
| 4 | 217 | `worker-1` | fix 1 | 11-20 10:32:03 | 10:41:40 | 577 | succeeded | `99828519-6f12-4c89-aa2d-23c905bb6074` | 3 |  |
| 5 | 217 | `reader-1` | reader | 11-20 10:42:47 | 10:57:20 | 873 | succeeded | `9946f6df-467d-4f12-b657-45bec20acc10` | 3, 6 | yes |
| 6 | 218 | `worker-2` | fix 1 | 11-20 10:56:47 | 11:25:21 | 1713 | succeeded | `376a07c2-91b1-4073-a0cc-4d24cfe8206b` | 5, 7 |  |
| 7 | 217 | `worker-1` | fix 2 | 11-20 10:58:28 | 11:05:15 | 408 | succeeded | `ae7e2b99-7d6c-4aa6-9453-454c1a274a73` | 6 |  |
| 8 | 218 | `verifier-2` | verifier | 11-20 11:27:53 | 11:37:55 | 602 | succeeded | `a81e5c46-7e53-47a3-8e8a-d001930f002b` | none | yes |
| 9 | 218 | `worker-2` | fix 2 | 11-20 11:38:29 | 11:55:45 | 1037 | succeeded | `da3e7568-5d6f-424a-8f64-6bc02798736a` | none |  |
| 10 | 218 | `reader-1` | reader | 11-20 11:56:39 | 12:13:13 | 994 | succeeded | `451d3d25-ac50-4564-ae71-63ddd5e09b7a` | none | no, turn 2 |
| 11 | 219 | `worker-1` | worker | 11-20 12:20:19 | 12:37:47 | 1048 | succeeded | `313c0761-4bd7-4abf-bef8-bb9f7b8ba59b` | 12 |  |
| 12 | 218 | `worker-2` | fix 3 | 11-20 12:20:20 | 12:31:18 | 659 | succeeded | `0f93d8ca-2773-44f8-913c-34f3237ff2cb` | 11 |  |
| 13 | 219 | `verifier-1` | verifier | 11-20 12:48:38 | 12:51:49 | 191 | succeeded | `54f3d420-826f-46d3-913e-15a6eb2061cc` | none | yes |
| 14 | 219 | `worker-1` | fix 1 | 11-20 13:07:06 | 13:11:18 | 252 | succeeded | `ba94a2c2-baf4-49c3-b2aa-e17a223f2cc6` | none |  |
| 15 | 219 | `reader-1` | reader | 11-20 13:12:01 | 13:16:57 | 297 | succeeded | `59804b2e-a737-48e0-a9ea-24989046a634` | 16, 17 | yes |
| 16 | 222 | `worker-2` | worker | 11-20 13:12:24 | 13:13:54 | 90 | succeeded | `5ee71074-607b-4737-b016-d4ae31c58746` | 15 |  |
| 17 | 222 | `worker-2` | fix 1 | 11-20 13:14:43 | 13:24:15 | 572 | succeeded | `2acba9bd-c39e-4aea-a31c-e4bba40db385` | 15, 18 |  |
| 18 | 219 | `worker-1` | fix 2 | 11-20 13:17:29 | 13:19:40 | 130 | succeeded | `ac484860-a0c7-4252-b668-18c1d730a687` | 17 |  |
| 19 | 222 | `verifier-2` | verifier | 11-20 13:24:51 | 13:29:41 | 289 | succeeded | `597ef1ae-c2e8-4855-93b9-86658c96d41e` | none | yes |
| 20 | 222 | `reader-1` | reader | 11-20 13:33:07 | 13:44:31 | 683 | succeeded | `b33f86f0-b74a-4736-b03c-5144372e3384` | 21 | no, turn 2 |
| 21 | 220 | `worker-1` | worker | 11-20 13:37:49 | 13:44:46 | 417 | succeeded | `4f2debf8-978c-4638-8133-86c5cc5f083c` | 20 |  |
| 22 | 220 | `worker-1` | fix 1 | 11-20 13:46:16 | 13:57:42 | 686 | succeeded | `2a821e2c-2afe-4318-a7c5-3e31a9a3c062` | none |  |
| 23 | 220 | `verifier-1` | verifier | 11-20 13:58:34 | 14:01:13 | 159 | succeeded | `fa0f3d54-ce21-44b7-b1a2-7e7d6fc6f8db` | none | no, turn 2 |
| 24 | 220 | `reader-1` | reader | 11-20 14:01:49 | 14:03:50 | 121 | succeeded | `6bb7912c-9932-4609-8d70-7539d0066584` | 25 | no, turn 3 |
| 25 | 223 | `worker-2` | worker | 11-20 14:01:49 | 14:20:24 | 1115 | succeeded | `f12aaa11-492a-4ff2-9e91-76f190ffd8c6` | 24 |  |
| 26 | 223 | `verifier-2` | verifier | 11-20 14:20:52 | 14:26:53 | 361 | succeeded | `adbd0013-2df5-4547-82cf-86f31a1a56dd` | none | no, turn 2 |
| 27 | 223 | `worker-2` | fix 1 | 11-20 14:27:17 | 14:41:26 | 849 | succeeded | `e2ff8c37-b8e0-40ef-9e56-496a447c0a96` | 28, 29 |  |
| 28 | 224 | `worker-1` | worker | 11-20 14:32:19 | 14:33:43 | 84 | succeeded | `a41647c6-26f5-4e60-a782-199e8667ee58` | 27 |  |
| 29 | 224 | `worker-1` | fix 1 | 11-20 14:34:07 | 14:58:08 | 1441 | succeeded | `fa7316fc-4ed3-4509-bd4f-c624938a9b18` | 27, 30, 31 |  |
| 30 | 223 | `reader-1` | reader | 11-20 14:42:22 | 14:47:18 | 296 | succeeded | `b0c73249-78f1-4c5d-b4a4-a46abeb7d1d2` | 29 | no, turn 4 |
| 31 | 225 | `worker-2` | worker | 11-20 14:48:47 | 15:12:31 | 1425 | succeeded | `a2777e75-4b9e-483d-9715-a563c8668eea` | 29, 32 |  |
| 32 | 224 | `verifier-1` | verifier | 11-20 14:58:44 | 15:02:30 | 227 | succeeded | `74ce63ca-2921-4438-827f-7a55c565bccf` | 31 | no, turn 3 |
| 33 | 225 | `verifier-2` | verifier | 11-20 15:15:35 | 15:20:29 | 294 | succeeded | `04ce2f50-66ad-48eb-97eb-7144b042420d` | 34 | no, turn 3 |
| 34 | 224 | `worker-1` | fix 2 | 11-20 15:16:27 | 15:24:27 | 479 | succeeded | `f3cb445e-c0b0-4921-b71a-ca4f941903f8` | 33, 35 |  |
| 35 | 225 | `worker-2` | fix 1 | 11-20 15:21:05 | 15:25:52 | 287 | succeeded | `6515e6a7-892d-4da6-8a9a-fe4b998c8387` | 34, 36, 37 |  |
| 36 | 224 | `reader-1` | reader | 11-20 15:25:24 | 15:29:59 | 275 | succeeded | `60ad1a19-5605-4ef9-bb84-06f14ac5937a` | 35, 37 | no, turn 5 |
| 37 | 221 | `worker-1` | worker | 11-20 15:25:24 | 15:51:01 | 1536 | succeeded | `b367ce0e-18f6-4493-bcfe-1ab38c668ed1` | 35, 36, 38, 39 |  |
| 38 | 224 | `worker-2` | fix 3 | 11-20 15:30:45 | 15:38:15 | 450 | succeeded | `6945e8a1-6838-4363-810c-ef5eea58047a` | 37, 39 |  |
| 39 | 225 | `reader-1` | reader | 11-20 15:30:55 | 15:34:56 | 241 | succeeded | `52d061c7-c586-4393-8ddc-e05ba9b2f7a4` | 37, 38 | no, turn 6 |
| 40 | 221 | `verifier-1` | verifier | 11-20 15:52:44 | 15:57:25 | 281 | succeeded | `0644ee2a-9391-4bda-8785-97cf661c9023` | 41 | no, turn 4 |
| 41 | 230 | `worker-2` | worker | 11-20 15:54:06 | 16:23:11 | 1745 | succeeded | `5709d4ad-a13f-4f84-90af-33ac32c6085b` | 40, 42, 43 |  |
| 42 | 221 | `worker-1` | fix 1 | 11-20 15:57:50 | 16:04:08 | 379 | succeeded | `a3603138-f0b6-40b9-ba3e-03b684523e96` | 41 |  |
| 43 | 221 | `reader-1` | reader | 11-20 16:04:44 | 16:09:17 | 273 | succeeded | `97458d4d-151c-4c9c-ad52-0542469f9842` | 41 | no, turn 7 |
| 44 | 230 | `verifier-2` | verifier | 11-20 16:23:57 | 16:28:11 | 254 | succeeded | `34d8435b-523a-4a47-b467-a03ce2f741dd` | 45 | no, turn 4 |
| 45 | 227 | `worker-1` | worker | 11-20 16:24:22 | 17:26:25 | 3724 | succeeded | `2f639851-563a-4564-a2a7-812c17ce803b` | 44, 46, 47, 48, 49, 50 |  |
| 46 | 230 | `worker-2` | fix 1 | 11-20 16:28:37 | 16:32:32 | 235 | succeeded | `f334b232-d5fb-4e91-9485-149a54f71494` | 45 |  |
| 47 | 230 | `reader-1` | reader | 11-20 16:34:36 | 16:38:41 | 245 | succeeded | `033a0153-b832-4ce9-ada7-6ca75a473e88` | 45 | no, turn 8 |
| 48 | 229 | `worker-2` | worker | 11-20 16:46:24 | 16:49:45 | 202 | succeeded | `60e7a5c7-37cd-4474-afbf-050f236a1373` | 45 |  |
| 49 | 231 | `worker-2` | worker | 11-20 16:51:29 | 17:03:25 | 716 | succeeded | `683c918a-2ad2-4377-98b8-58df69075dc1` | 45 |  |
| 50 | 231 | `worker-2` | fix 1 | 11-20 17:03:54 | 17:28:22 | 1468 | succeeded | `2e64c09a-ae8f-4a2b-9bbd-163cf0770327` | 45, 51 |  |
| 51 | 228 | `worker-1` | worker | 11-20 17:27:14 | 17:39:52 | 758 | succeeded | `55255682-cde2-40b1-b408-5141cfca4bae` | 50, 52, 53 |  |
| 52 | 231 | `verifier-2` | verifier | 11-20 17:28:53 | 17:34:17 | 324 | succeeded | `459d20ff-ed9c-4f19-b2e5-888e39c01097` | 51 | no, turn 5 |
| 53 | 231 | `worker-2` | fix 2 | 11-20 17:35:34 | 17:59:27 | 1433 | succeeded | `3ef771c6-d8a9-4ce9-a321-cdf5768366c2` | 51, 54, 55, 56, 57 |  |
| 54 | 228 | `verifier-1` | verifier | 11-20 17:41:54 | 17:46:55 | 301 | succeeded | `cab3bead-f3c5-408e-8c05-949d9f5d0c89` | 53 | no, turn 5 |
| 55 | 228 | `worker-1` | fix 1 | 11-20 17:47:26 | 17:54:41 | 434 | succeeded | `fcadb252-867c-41db-a560-16c07f85d72b` | 53 |  |
| 56 | 228 | `reader-1` | reader | 11-20 17:55:20 | 17:59:27 | 246 | succeeded | `b8ad8b84-7664-45a1-99e4-cb51907f0acc` | 53, 57 | no, turn 9 |
| 57 | 226 | `worker-1` | worker | 11-20 17:55:47 | 18:12:09 | 982 | succeeded | `b2beea52-986c-4b4d-9e4d-c891349b7c2f` | 53, 56, 58 |  |
| 58 | 231 | `reader-1` | reader | 11-20 18:00:11 | 18:04:42 | 271 | succeeded | `5e749de5-d594-4078-9821-d9a71040c8bd` | 57 | no, turn 10 |
| 59 | 227 | `worker-1` | fix 1 | 11-20 18:26:32 | 18:54:19 | 1667 | succeeded | `344e3b87-c39a-44f8-a38f-c5e2767aea49` | 60 |  |
| 60 | 229 | `worker-2` | fix 1 | 11-20 18:26:33 | 18:59:26 | 1973 | succeeded | `deede271-6107-4b0b-80ed-75e67b5b78b4` | 59, 61 |  |
| 61 | 227 | `verifier-1` | verifier | 11-20 18:55:05 | 19:01:19 | 374 | succeeded | `a97dffe5-9deb-413b-8437-796480f46b4d` | 60, 62 | no, turn 6 |
| 62 | 229 | `verifier-2` | verifier | 11-20 19:00:02 | 19:07:13 | 431 | succeeded | `42cb8ad1-1db2-49e6-9704-6183e9196ec1` | 61, 63 | no, turn 6 |
| 63 | 227 | `worker-1` | fix 2 | 11-20 19:01:57 | 19:49:12 | 2835 | succeeded | `a55fb0c8-77cc-4d7a-b0e2-543a9b0c57c9` | 62, 64, 65, 66 |  |
| 64 | 229 | `worker-2` | fix 2 | 11-20 19:07:46 | 19:18:50 | 665 | succeeded | `b0f6837c-6c39-4ca6-b94c-3578deb135b4` | 63 |  |
| 65 | 229 | `reader-1` | reader | 11-20 19:19:31 | 19:25:00 | 328 | succeeded | `e29c47f4-b871-43e9-a275-f814d4bd1cd6` | 63, 66 | no, turn 11 |
| 66 | 226 | `worker-2` | fix 1 | 11-20 19:19:41 | 19:57:08 | 2247 | succeeded | `cb28f63d-7040-4311-87f5-7738341000f7` | 63, 65, 67, 68 |  |
| 67 | 227 | `reader-1` | reader | 11-20 19:49:55 | 19:54:27 | 271 | succeeded | `8dc2c5a0-29fd-461e-8275-273c2468b599` | 66, 68 | no, turn 12 |
| 68 | 232 | `worker-1` | worker | 11-20 19:50:06 | 20:09:02 | 1136 | succeeded | `26f49bd3-882e-4454-b098-2ca30bd5f0e8` | 66, 67, 69, 70 |  |
| 69 | 226 | `verifier-2` | verifier | 11-20 19:57:41 | 20:06:02 | 501 | succeeded | `d0ea1d00-c025-44e8-a092-7b88343ab672` | 68 | no, turn 7 |
| 70 | 226 | `worker-2` | fix 2 | 11-20 20:06:27 | 20:19:07 | 759 | succeeded | `93313368-569f-4617-95aa-eb1cb67517d2` | 68, 71 |  |
| 71 | 234 | `worker-1` | worker | 11-20 20:09:37 | 20:20:27 | 651 | succeeded | `7a555814-f958-4dfd-b57f-459e9d12e48d` | 70, 72, 73 |  |
| 72 | 226 | `reader-1` | reader | 11-20 20:19:45 | 20:24:48 | 303 | succeeded | `47648b5c-2ad5-4092-8fbf-bb3455830bf0` | 71, 73, 74, 75 | no, turn 13 |
| 73 | 233 | `worker-2` | worker | 11-20 20:19:53 | 20:41:19 | 1285 | succeeded | `15d1271a-8e58-4e2a-8206-dc35fa1248fc` | 71, 72, 74, 75, 76, 77 |  |
| 74 | 234 | `verifier-1` | verifier | 11-20 20:21:01 | 20:23:34 | 153 | succeeded | `f5c31ead-39a1-45eb-8939-651aefd40185` | 72, 73 | no, turn 7 |
| 75 | 234 | `worker-1` | fix 1 | 11-20 20:24:02 | 20:34:43 | 641 | succeeded | `2bf74526-cc6f-41a4-8f15-b27b64c00f54` | 72, 73 |  |
| 76 | 234 | `reader-1` | reader | 11-20 20:35:29 | 20:40:18 | 289 | succeeded | `39e02985-81ef-42d1-a76a-d201cbb8b1fe` | 73, 77 | no, turn 14 |
| 77 | 232 | `worker-1` | fix 1 | 11-20 20:35:30 | 21:00:20 | 1490 | succeeded | `132226f0-fbe0-45de-b6fd-3bcc4b3fe7eb` | 73, 76, 78, 79 |  |
| 78 | 233 | `verifier-2` | verifier | 11-20 20:43:58 | 20:48:37 | 279 | succeeded | `24b607ba-3ad9-4bd7-b675-421c5fddd481` | 77 | no, turn 8 |
| 79 | 233 | `reader-1` | reader | 11-20 20:49:20 | 20:51:55 | 155 | succeeded | `02f7aba1-bd91-4dcb-b916-471173bd1cb2` | 77 | no, turn 15 |
| 80 | 232 | `verifier-1` | verifier | 11-20 21:00:52 | 21:05:37 | 284 | succeeded | `a0a37bc7-0b26-43e1-b716-8e8652fa28e5` | none | no, turn 8 |
| 81 | 232 | `worker-1` | fix 2 | 11-20 21:06:01 | 21:13:31 | 450 | succeeded | `29562a41-e479-4055-876b-787cb98891b4` | none |  |
| 82 | 232 | `reader-1` | reader | 11-20 21:14:02 | 21:15:59 | 116 | succeeded | `1dca403d-353e-43ff-b198-edb9767af9c8` | none | no, turn 16 |
| 83 | 235 | `worker-1` | worker | 11-20 23:57:55 | 11-21 00:07:53 | 598 | succeeded | `8294cef1-95cb-4a84-9c19-0dc0fc45a5fa` | none |  |
| 84 | 236 | `worker-2` | worker | 11-21 00:08:53 | 00:24:21 | 929 | succeeded | `c1ddc2a6-1d21-48a9-9374-15e647f58888` | 85, 86 |  |
| 85 | 235 | `verifier-1` | verifier | 11-21 00:09:29 | 00:13:23 | 234 | succeeded | `75a8a28b-e6e3-4463-b227-b8b27311f94c` | 84 | no, turn 9 |
| 86 | 235 | `reader-1` | reader | 11-21 00:14:16 | 00:19:06 | 290 | succeeded | `845d5885-305c-4409-909d-299cea69cf6d` | 84 | no, turn 17 |
| 87 | 236 | `verifier-2` | verifier | 11-21 00:25:29 | 00:29:54 | 265 | succeeded | `2324aae8-e3c7-4134-a4bf-36b9a7dc872c` | none | no, turn 9 |
| 88 | 236 | `worker-2` | fix 1 | 11-21 00:30:28 | 00:34:07 | 219 | succeeded | `de75b1a7-d3ac-4510-a6fc-6d9e4f13f0a5` | none |  |
| 89 | 236 | `reader-1` | reader | 11-21 00:34:37 | 00:37:00 | 143 | unknown | `3d3ec1ba-2511-4aa8-9c55-45433f379e0f` | none | no, turn 18 |
| 90 | 236 | `reader-1` | reader | 11-21 08:33:17 | 08:33:18 | 1 | unknown | `2e41327c-4a66-4de6-a0a7-2460384a0b08` | none | no, turn 19 |
| 91 | 236 | `reader-1` | reader | 11-21 08:36:14 | 08:40:59 | 285 | succeeded | `6a0ecf5a-43a8-4a81-9a89-198ab404f1c5` | none | yes |
| 92 | 236 | `worker-2` | fix 2 | 11-21 08:41:32 | 08:49:47 | 495 | succeeded | `c4e563c6-cbf4-472b-a392-bf87522ed72e` | none |  |
| 93 | 236 | `reader-1` | reader | 11-21 08:50:06 | 08:52:19 | 132 | succeeded | `6249ece8-f969-4866-b002-65df34a6d72d` | none | no, turn 2 |
| 94 | 236 | `worker-2` | fix 3 | 11-21 08:52:37 | 08:55:22 | 165 | succeeded | `24d8dc28-5a85-476f-92f8-52e8e8df1730` | none |  |
| 95 | 238 | `worker-1` | worker | 11-21 09:29:42 | 09:40:19 | 638 | succeeded | `0fa77158-0296-4a17-ba7a-68f92aa1562c` | none |  |
| 96 | 238 | `verifier-1` | verifier | 11-21 09:40:55 | 09:43:17 | 142 | succeeded | `681a1a4a-d0f6-4dc7-9072-a0974d090b01` | 97 | yes |
| 97 | 239 | `worker-2` | worker | 11-21 09:40:56 | 10:12:08 | 1872 | succeeded | `e0999bbd-9368-4dfb-9734-8944c67fd69c` | 96, 98, 99, 100 |  |
| 98 | 238 | `worker-1` | fix 1 | 11-21 09:43:39 | 09:48:46 | 307 | succeeded | `170bdb9c-2d4e-4f3a-b86c-b757ec3857a1` | 97 |  |
| 99 | 238 | `reader-1` | reader | 11-21 09:49:09 | 09:53:29 | 260 | succeeded | `51d8a63a-ca05-461f-9d0e-3cb2b39396dc` | 97 | no, turn 3 |
| 100 | 238 | `worker-1` | fix 2 | 11-21 09:53:51 | 09:58:54 | 303 | succeeded | `019d8457-03ae-43c0-9c58-c06048c97787` | 97 |  |
| 101 | 239 | `verifier-2` | verifier | 11-21 10:12:32 | 10:17:20 | 288 | succeeded | `97115c68-1e16-459a-ab90-0cf46105cda7` | none | yes |
| 102 | 239 | `worker-2` | fix 1 | 11-21 10:17:38 | 10:25:25 | 467 | succeeded | `459da13b-ce71-41a2-bdb6-bd53f1370abe` | none |  |
| 103 | 239 | `reader-1` | reader | 11-21 10:25:48 | 10:31:35 | 347 | succeeded | `1ea1c80c-c376-46f8-b2be-4eb69a25460d` | none | no, turn 4 |
| 104 | 239 | `worker-2` | fix 2 | 11-21 10:31:55 | 10:37:23 | 328 | succeeded | `368fe9f3-cbf9-4d18-9c45-0fe2b2f407a9` | none |  |

## Commands

They run on the coordinator's private files, so a reader of this repository cannot run them.

```sh
conduit records --store <store> > <records>   # serve stopped: exit 0; serve running: exit 1, store already has an active shell
jq -r .type <records> | sort | uniq -c   # 310 access decisions, 385 dispatches, 385 invocation endings, 234 object changes, 5 pending actions answered, 385 request endings
jq -r 'select(.type=="invocation-dispatched") | .data.operation' <records> | sort | uniq -c   # 124 turn.v1, 6 open.v1; no close.v1, reconcile.v1, sign-out-hook.v1 or new-conversation.v1
jq -r 'select(.type=="access-decision") | .data.permitted' <records> | sort | uniq -c   # 310 true
jq -r 'select(.type=="invocation-dispatched") | .data.attempt' <records> | sort | uniq -c   # 385 at 1
jq -r 'select(.type=="invocation-ended") | .data.outcome' <records> | sort | uniq -c   # 378 succeeded, 7 unknown: turns 89 and 90 and the 5 sign-in hooks
jq -c 'select(.type=="invocation-ended" and .data.reconciled == true)' <records> | wc -l   # 0
jq -r 'select(.type=="invocation-ended" and (.data.pendingAction|length) > 0) | .time' <records>   # 10, all from 07:45Z to 07:52Z on 2026-11-20
jq -r 'select(.data.invocation=="ID") | [.time, .type, .data.outcome // ""] | @tsv' <records>   # one turn's records, by its invocation ID
jq -r '._meta["io.github.shrav-k.conduit/outcome"]' <direct>/*-result.json | sort | uniq -c   # every result, 215, 216 and 240 to 242 included
jq -r '._meta["io.github.shrav-k.conduit/invocation"]' <direct>/NAME-result.json
grep -E '^(start|call exit|end) ' <tasks>/ID.output   # the clock lines around one call
```

These give the records' totals, not the per-turn counts. The coordinator's scratch scripts
(private, in Python: `map.py`, `analyze.py` and `validate.py`, one more for the turns sent alone,
and `check_records.py` and `from_records.py` for the records) compute them from `<records>`,
`<direct>`, `<tasks>` and its session record, by these rules:

- **The turn list.** Each `conduit call` of a `turn.v1` tool in the session record, mapped to its
  result's name and its background task, and by its invocation ID to the records. A turn's start
  and end are its `invocation-dispatched` and `invocation-ended` records' times, and its Agent is
  its access decision's `tool`. The call the CLI refused at 00:30:11 on 2026-11-21 is not a turn.
- **Stage 2's turns.** Those of orders 217 to 239, sent after the amendment's merge and before
  239's merge, numbered by dispatch.
- **Overlaps.** Two turns overlap when each is dispatched before the other ends.
- **Sent in flight.** A turn is sent in flight when another was dispatched before it and had not
  ended. A turn is sent alone when it was not. No two turns share a dispatch time to the
  millisecond.
- **Fresh reads.** A read is fresh when no earlier turn of its Agent was dispatched between the
  last restart before it and its own dispatch. A restart is a setup of every Agent in the
  records.
- **Seconds.** End minus dispatch, rounded. Medians are over the turns that succeeded.
- **Reports.** Each report's text was parsed as one JSON object and checked with Python's
  `jsonschema`. Workers' reports were checked against
  [`004-i2-01-vectors-and-first-start.schema.json`](../../docs/work-orders/004-i2-01-vectors-and-first-start.schema.json),
  and verifiers' and `reader-1`'s against
  [`verify.schema.json`](../../docs/work-orders/verify.schema.json), the schemas each turn's
  message named.
