# Ruling 89: the readiness review on Fable, three speed levers, the first dogfood milestone, the owner's keys within the Agents' reach, a Claude scratch sign-in, A77's pairing paused, and the S1 rerun's pin at milestone 1

Status: in force (ruled by the owner on 2026-11-19, as the coordinator's notes date the answers, the first two at 08:12 PST). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topic 9 (point 1), topic 13 (point 7) and topic 15 (points 1 to 6), the [dated amendment of A77's registration](../../implementation/WORK_ORDERS.md#amendment-of-2026-11-19-pairing-paused-until-050n-merges) (point 6), [experiment 027's amendment for the live check](../../../experiments/027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs) (points 1 and 5), and the rows of A5, A77 and A78 in [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes) (points 1, 3, 5, 6 and 7). This record says why.

On 2026-11-19, after [ruling 88](../ruling-88/README.md), the owner asked for a review of the
readiness for the first dogfood. The coordinator put questions to the owner through the session's
question tool, in three sittings, each with options and a recommendation. The owner chose the
recommended option in each question but question 4, where the owner typed the answer. The owner's
words, the questions as put and the options as the coordinator's notes (private) hold them follow,
verbatim. The notes date them 2026-11-19, and time the owner's message and the first two answers at
08:12 PST.

## The questions and the answers

### The readiness review and the speed levers (08:12 PST)

The owner: "Since we don't have much in parallel right now, I'd like to plan with you a fable
review of everything relevant before the dogfood. Since limits reset Nov 21 at 1am I'd like as
much as possible to complete everything before then and have the fable run assessing our readiness
to be in parallel. Let's discuss how this might be achievable and whether we can be smarter about
our tests, and where the time is going"

1. As put: "How should the Fable readiness review run?"

   The owner chose "Now, plus a delta pass (Recommended)", whose text read: "Start the
   six-dimension Fable review now on main, both lanes and the written-ahead chain (plus synthesis
   and an adversarial check), and a shorter Fable pass over what changed after 048b-join merges,
   if before the Nov 21 1am reset. Findings feed the remaining chain orders as they're refreshed."

2. As put: "Which speed levers should I adopt now for the remaining chain?"

   The owner chose "Pipeline the chain (Recommended), Warm, targeted reads (Recommended), Less
   prescriptive orders (Recommended)". Not chosen: "Auto-merge after clean reread".

The notes hold the second question's options by label only.

### The first dogfood milestone

3. As put: "What should count as the first dogfood milestone (what we aim to have ready before the
   Nov 21 reset)?"

   The owner chose "Primary slots, serial first (Recommended)", whose text read: "Milestone 1:
   your primary Claude and Codex accounts' Agents signed in through Conduit on the Mac (after 050n
   and 051b-2), running Conduit's work orders one turn at a time. Milestone 2: parallel slots once
   203 (0.2.1) ships, about a day later. Second accounts after the S1 rerun's pin."

### After the Fable readiness review

4. As put, but for one clause: "Dogfood Agents will run as your macOS user and can read your shell
   profile/environment (ruling 70). [The first of two questions, on keys of an earlier date, which
   the coordinator's notes (private) hold], and is it OK that an Agent could read whatever API-key
   variable your shell still sets?"

   The owner typed: "It’s ok if the agents can read the keys for the dogfood, isolation is a later
   concern"

5. As put: "Milestone 1 includes your primary Claude account. Claude's sign-in has never run live.
   Add a Claude scratch-home sign-in (about 5 min) to the live-check sitting, and include the Claude
   reader slot only once the D58 guard is in the harness?"

   The owner chose "Yes, both (Recommended)".

6. As put: "A77's paired reads add a blind Opus read, my judging, and a second Codex round to each
   paired order. Pause pairing until 050n merges, to save time and Claude allowance on the dogfood
   path?"

   The owner chose "Pause until 050n (Recommended)".

7. As put: "Ruling 88 deferred the carried return (048a-7, 048a-8, 051b-3). The S1 rerun's pin
   waits for 'the six sign-in orders'; do those now mean the 0.2.0 ones, so the pin (which gates
   second accounts and the Linux node joining) can come at milestone 1?"

   The owner chose "Pin at milestone 1 (Recommended)".

The notes hold no other option of questions 1 and 3 to 7, and no option's text for questions 4 to 7.
The clause left out of question 4 is held in the coordinator's notes and not in this tree; the
owner's answer does not speak to it. The hours, minutes and days in the options' texts are the
coordinator's estimates, not measures this tree holds.

## The readiness review, in brief

The review that answer 1 started ran on Claude Fable over six dimensions (the specification and
rulings, the engine's sign-in, the Agent packages, the dogfood's operations, security and privacy,
and the process and its throughput), with a synthesis and an adversarial check of it. Its reports
are the coordinator's notes (private). Its verdict, as the coordinator summarizes it: not ready on
the day; what is ruled and written holds together, and what stands between the tree and a first
turn is operational. Four blockers:

- **B1.** The dogfood harness has no D58 guard, which the rulings make a precondition for any
  Claude slot (IN_FORCE.md, topic 15).
- **B2.** The live check's engine would drop the sign-in's session while the owner signs in: the
  serving engine's idle limit is the call timeout plus 60 seconds, 180 by default, and a running
  call does not reset it (engine/CONFORMANCE.md, DL10).
- **B3.** Experiment 027's pre-registered Type name for the live check, `codex-account-check.v1`,
  would break the Codex sign-in hook, which finds its stated logins by the Type's exact name,
  `codex.v2`, before any login.
- **B4.** Every unconfined Agent can reach what the owner's operating-system user's environment
  and shell profile hold; the review asked for the owner's word under ruling 70.

Among its majors: the Claude sign-in path has never run against the real client (M1), and the S1
rerun's pin, by ruling 88's coordinator's reading, had moved behind the keyed release (M6). By the
coordinator's reading, answer 4 answers B4, answer 5 M1 and B1's place in milestone 1, and answer 7
M6; 027's amendment below takes B2 and B3. B1's guard is the coordinator's to build, in the harness
outside the tree.

## The decisions

1. **The Fable readiness review runs now, and a delta pass after 048b-join** (answer 1). The
   six-dimension review, its synthesis and its adversarial check ran on `main`, both lanes and the
   written-ahead chain; a shorter Fable pass reads what changed once 048b-join merges, if that is
   before the reset the option names. Its findings feed the chain's orders at their refresh.
   - By the coordinator's reading, as for the targeted review of
     [ruling 81](../ruling-81/README.md), point 2, the owner chose Fable for this review and its
     delta pass, and this sets no standing rule; ruling 79 keeps Fable for the owner's reviews.
   - By the coordinator's reading, its findings are the coordinator's to apply, each open to the
     owner's strike, except where a finding needs the owner's word, which the coordinator asks.
2. **Three speed levers for the remaining chain** (answer 2): pipelining the chain, warm targeted
   reads, and less prescriptive orders. "Auto-merge after clean reread" was not chosen. As the
   coordinator applies them, a reading open to the owner's strike, since the notes hold the labels
   alone:
   - **Pipelining the chain.** Each order of the sign-in chain is dispatched on its predecessor's
     verified round-1 branch, with a dispatch note relaxing its start check's "merged" to that
     branch, and is rebased onto `main` once the predecessor merges. A fix after that rebase is
     read as ruling 87 has it.
   - **Warm, targeted reads.** The fresh reader runs the tests that bear on the order's change, on
     a warm build, and cites the worker's last whole-suite line; the whole suite runs once, in the
     local gate before the merge.
   - **Less prescriptive orders.** At the refresh before dispatch, a Done-when that pins a hash or
     a line count is dropped and the behavior tests are kept, and exact text is prescribed only for
     the specification's wording, diagnostics and prescribed cases. An order's decisions stay as
     written.
   - **Not chosen.** An order whose first read found a must still waits for the coordinator after a
     clean reread, as IN_FORCE.md, topic 15, reads ruling 85, point 1.
3. **The first dogfood milestone** (answer 3).
   - **Milestone 1**: the Agents of the owner's primary Claude and Codex accounts signed in through
     Conduit on the Mac, after 050n and 051b-2, running Conduit's work orders one turn at a time.
     The Claude reader slot joins it as point 5 says.
   - **Milestone 2**: parallel slots, once order 203 (a new conversation without restarting the
     engine, 0.2.1) ships.
   - **Second accounts** after the S1 rerun's pin, which point 7 places at milestone 1.

   By the coordinator's reading: the milestones are targets for the work's order, and set no gate
   and no deadline that binds; milestone 2 is where IN_FORCE.md, topic 15, already lets A78's
   orders run in parallel ("until order 050 and the review's order 62", order 203 being order 62);
   and [ruling 84](../ruling-84/README.md), point 3, still holds, so no Agent doing Conduit's own
   orders names a second account until the owner says so.
4. **During the dogfood, Agents may read the owner's keys and environment; isolation is a later
   concern** (answer 4). What the owner's operating-system user's environment and shell profile
   hold, an API-key variable included, is within reach of the dogfood's unconfined Agents, as
   [ruling 70](../ruling-70/README.md) lets them run, and the owner accepts that reach for the
   dogfood. By the coordinator's reading:
   - this answers the readiness review's B4 under ruling 70, by accepting the reach, and the review
     no longer holds the dogfood for it;
   - reach is not use: no Agent is given or configured with a model service's API key
     ([ruling 83](../ruling-83/README.md)), and nothing switches an Agent to one
     ([ruling 82](../ruling-82/README.md), point 2);
   - the sandbox method of ruling 84, point 1, is the later concern the answer names; and
   - starting the engine and the harness with a cleared environment stays the coordinator's
     hygiene, which narrows what logs and records can carry, not an Agent's reach.
5. **A Claude scratch-home sign-in in the live check's sitting, and the Claude reader slot only
   once D58's guard is in the harness** (answer 5).
   - In the sitting of A5's live check the owner signs a scratch Claude home in once, as
     [027's amendment](../../../experiments/027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs)
     pre-registers it in this change ([ruling 45](../ruling-45/README.md)).
   - The Claude reader slot that question 5 names, by the coordinator's plan (the coordinator's
     notes, private) the one Claude slot of milestone 1, joins milestone 1 only once D58's guard
     is in the harness, as IN_FORCE.md, topic 15, already has it for any Claude slot.

   By the coordinator's reading: the Claude sign-in runs after the Codex check, through
   `conduit sign-in`, under 027's guards; it passes when the account that
   `claude-code.v2`'s account hook reports is the expected one (PND-011); it is not part of A5's
   gate, which concerns `codex.v2` alone (ruling 82, point 1), and changes nothing in
   `claude-code.v2` or 050n's join; if it fails, or ends a fault, the coordinator brings that to
   the owner before any Claude slot signs in; and milestone 1 may begin with its Codex slots while
   the Claude slot waits for D58's guard.
6. **A77's pairing is paused until order 050n merges** (answer 6). A77's registration gains a
   [dated amendment](../../implementation/WORK_ORDERS.md#amendment-of-2026-11-19-pairing-paused-until-050n-merges),
   in this change, as it changes only by one. By the coordinator's reading, open to the owner's
   strike:
   - the 20-order window keeps counting, and an order dispatched in the pause is one of A77's
     orders, not paired, its record saying why, as the registration has it for an order dispatched
     while the Claude allowance runs low;
   - an order whose paired read ran before the answer keeps its pairing and the round 2 that the
     registration prescribes;
   - pairing starts again with the first of A77's orders dispatched after 050n merges, as
     registered, and a group with fewer than five paired orders when the window ends is not shown,
     as registered;
   - order 209 was dispatched unpaired on the owner's answer, before the amendment merged, as the
     coordinator's notes (private) record; and
   - the pause amends A77's registration, the coordinator's choices that
     [ruling 80](../ruling-80/README.md), point 6, settled, and not ruling 80 itself, as the
     amendment of 2026-11-19 that restarted the window did not.
7. **The S1 rerun's pin at milestone 1** (answer 7). "The six sign-in orders" that the pin waits
   for ([ruling 78](../ruling-78/README.md), point 1, part 2) are 0.2.0's sign-in orders, so the
   pin, as far as sign-in holds it, comes at milestone 1, and orders 048a-7, 048a-8 and 051b-3,
   which ruling 88 moved after dogfood, do not hold it. This replaces the coordinator's reading in
   ruling 88, point 1, that the pin waits for them and so for the keyed release's stage, and
   answers the third of the questions ruling 88 left to the owner. What the pin gates, as the
   question named it, the second accounts and the Linux node joining, can then come after
   milestone 1, and so can the lane's restart (ruling 78, point 1, part 1). By the coordinator's
   reading, open to the owner's strike:
   - order 050o, while it follows 051b-3, does not hold the pin either;
   - the answer changes only what sign-in holds: the pin still fixes the build that the S1 rerun
     measures, and if anything else it waits for has not merged at milestone 1, the coordinator
     tells the owner; and
   - the owner still signs each dogfood Agent in once (ruling 78, point 1, part 2).

## What it amends

- **Ruling 78, point 1, part 2**, in its lines that IN_FORCE.md, topic 13, holds: "the six sign-in
  orders" are 0.2.0's, so the pin comes at milestone 1 (point 7).
- **Ruling 79, point 1, for one review only**: the readiness review for the first dogfood and its
  delta pass run on Fable (point 1). Point 1 otherwise stands.
- **Ruling 80, point 7**, its "the plain runner keeps the other orders meanwhile": by the
  coordinator's reading of point 3, until milestone 1 (see "After the ruling" below).
  *Coordinator's*, open to the owner's strike.
- **Ruling 82, point 1**: the coordinator's reading that answer 5 keeps Q3 option a's two terms,
  now: the Type keeps the name `codex.v2`, at a scratch address, installed only in the scratch
  store (the coordinator's application of the readiness review's B3, under point 1).
  *Coordinator's*, open to the owner's strike.
- **Ruling 88, point 1**, its coordinator's reading that the S1 rerun's pin still waits for
  048a-7, 048a-8 and 051b-3, and so for the keyed release's stage (point 7). Ruling 88's
  decisions are unchanged.
- **A77's registration** in [work orders](../../implementation/WORK_ORDERS.md#a77s-measures-registered-before-the-first-paired-read),
  by a dated amendment: no order is paired until 050n merges (point 6).
- **Experiment 027's pre-registration of stage 1's first step**, by a dated amendment before the
  run: the Type keeps the name `codex.v2`, the client is `conduit sign-in`, a second tab is
  expected, the engine runs with a call timeout of 1200 seconds (the coordinator's application of
  the readiness review's B2, B3 and D1-08, under point 1, each open to the owner's strike), and a
  Claude scratch-home sign-in follows (point 5, and the readiness review's M1).

## What it does not change

- **Ruling 70**, under which answer 4 is given, and the sandbox method of ruling 84, point 1. -
  **Rulings 82 and 83**, but for the coordinator's reading of 82 point 1 above: A5's gate concerns
  `codex.v2` alone; no Agent holds a model service's API key, and nothing switches an Agent to one
  (82 point 2, as 83 amends it). - **Ruling 84, point 3**: the second accounts' first sign-ins come
  at the pin, and no Agent doing Conduit's own orders names one until the owner says so. - **Ruling
  88's decisions**: 048a-7, 048a-8 and 051b-3 stay after dogfood, beside the keyed release; the
  pairs; and the path to the Mac dogfood. - **The fresh read before every merge** (ruling 79, point
  2, as rulings 85 and 87 amend it), and the gate's full run on the Mac before every merge (ruling
  85, point 4). - **D58** and IN_FORCE.md's rule that no Claude Agent becomes a dogfood slot before
  its harness change is in place. - **Ruling 80, point 6**, by the coordinator's reading (point 6).

## What is left for the owner

- Of ruling 88's three questions, two: whether PND-013's first check alone joins 048a-5ef, and
  whether order 050o stays tied to 051b-3, and so after dogfood.

## Facts recorded with it

- The owner's time: one message and one dialog of two questions at 08:12 PST, one question on the
  milestone, and one dialog of four questions after the readiness review, on 2026-11-19 as the
  coordinator's notes date them.
- When the owner answered the last dialog, A77 had paired three orders, two code and one text;
  order 209's text then merged (#407), and order 209 was dispatched unpaired, as the coordinator's
  notes (private) record.

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md): topic 9's line on the live check (point 1, the coordinator's
  reading of 82 point 1); topic 13's line on the pin (point 7), ruling 88's coordinator's line, and
  its *Open* line, which notes that ruling 89 answered the pin's question; topic 15's lines on who
  reads and who does (points 1 and 2), on A77 (point 6) and on the dogfood (points 3 to 5). -
  [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes): A5's row (027's amendment and the pin),
  A77's row (the pause) and A78's row (the milestones, the Claude slot and the pin). -
  [docs/AUDIT.md](../../AUDIT.md#waiting-for-the-owner): ruling 88's row under Waiting for the owner
  notes that ruling 89 answered the pin's question. - [The ledger](../ruling-log.md) gains row 89,
  and the status of rulings 78, 79, 82 and 88, there and in their records, notes the amendments. -
  [Work
  orders](../../implementation/WORK_ORDERS.md#amendment-of-2026-11-19-pairing-paused-until-050n-merges):
  A77's dated amendment (point 6). - [Experiment
  027](../../../experiments/027-a5-sign-in/README.md#amendment-of-2026-11-19-the-live-check-as-it-runs):
  its dated amendment for the live check (points 1 and 5). - Later, and not in this change: the
  delta pass after 048b-join; the chain's texts refreshed with the review's findings; D58's guard in
  the harness, outside the tree; the live check's run sheet completed, outside the tree.

## After the ruling

2026-11-19, later, the coordinator's note; it changes no decision, so it gets no new number
([ruling 67](../ruling-67/README.md)). IN_FORCE.md, topic 15, kept a last sentence from
[ruling 80](../ruling-80/README.md), point 7: until order 050 and the review's order 62, A78's
orders 2 and 3 run through Conduit one at a time "and the plain runner carries the rest". That
differs from the owner's words in point 3, milestone 1 "running Conduit's work orders one turn at
a time", and under [AGENTS.md](../../../AGENTS.md#what-holds), point 2, such a line is a defect
that the owner's words govern until it is fixed. The coordinator rewrote it to say that, until
order 050 and the review's order 62 let orders run in parallel, A78's orders 2 and 3 run through
Conduit one at a time and, from milestone 1, Conduit's work orders one turn at a time, the plain
runner carrying the other orders until then. What goes beyond the owner's words is marked there
as the coordinator's, open to the owner's strike: milestone 1 starts with A78's stage 2, its run
(order 211); and point 3 is the owner's later word for orders after A78's order 3 to go through
Conduit from milestone 1, before order 203, so that it, not ruling 80, point 7's "the plain
runner keeps the other orders meanwhile", governs the time after milestone 1; after it, the plain
runner still carries an order that the dogfood itself needs before its turns can run, such as a
fix of the engine or the harness that they wait on, an order that no Agent of milestone 1 can run
or that A78's registration keeps out of Conduit, and any order while that registration pauses the
turns through Conduit. The milestones' terms above, which set no gate and no deadline that binds,
are unchanged. [Order 211](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/211-a78-stage-2-registration.md), which
registers stage 2 as milestone 1's run, waits for this change before it starts.
