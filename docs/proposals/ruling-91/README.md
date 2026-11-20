# Ruling 91: an account check that answers another account after a sign-in's wait has ended holds the Agent, and the sign-in message for carried: false names no method

Status: in force; done (A88: both decisions written into spec/ and the engine by order 216) (ruled by the owner on 2026-11-19, as the coordinator's notes date the answers, at about 16:45 PST). Holds now: [IN_FORCE.md: 9. Fleets, Interfaces, pending actions, sign-in, and what Agents reach](../../../IN_FORCE.md#9-fleets-interfaces-pending-actions-sign-in-and-what-agents-reach) (points 1 and 2, in spec/ by order 216), and the row of A88 in [the tracker's action items](../../AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept). This record says why.

On 2026-11-19, while writing [order 215](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/215-build-sign-in-follow-up.md), the
sign-in chain's follow-up and the first order that A78's stage 2 sends through Conduit
([ruling 89](../ruling-89/README.md), point 3), the coordinator found two of the chain's findings
that `spec/` does not settle or that are `spec/`'s own words, and so the owner's. Order 215 leaves
both out and says that the coordinator puts them to the owner:

- **S2**, the second read's finding in
  [order 048a-5gh's Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/048a5gh-build-sign-in-undo-and-expiry.md#record): a
  late poll's other account left signed in, with no `SignInUndone` condition and, where no action
  was raised, no hold. DL18 says that no hook runs after the wait ends, and nothing says whether
  the instance is then held.
- **m1**, of the readiness review's delta pass, as the coordinator's notes (private) record it: the
  engine message's sentence that the person completes the login on the node's machine "by remote
  desktop or by SSH", which is IFC-049's text in `spec/interface/implementation.yaml` and the core
  `carried` description in `spec/schemas/contracts.yaml`.

The coordinator put both to the owner through the session's question tool, in one dialog of two
questions, each with options and a recommendation. The owner chose the recommended option in
question 1, and in question 2 the option that was not recommended. The questions as put, the
options chosen and the labels of the others, as the coordinator's notes (private) hold them,
follow verbatim. The notes date them 2026-11-19 and time them at about 16:45 PST.

## The questions and the answers

1. As put: "Writing order 215 (the first dogfood order) turned up a gap the spec doesn't settle. A
   sign-in ends when its wait times out, but an account check that was already in flight can then
   answer with a DIFFERENT account than the one expected. No sign-out runs after the wait ends, so
   that other account stays signed in and nothing holds the Agent. It could take a turn under the
   wrong account until its next setup check catches it. What should happen?"

   The owner chose "Hold the Agent (Recommended)", whose text read: "The late answer holds the
   Agent: everything but sign-in and retirement reports unavailable until a check finds the
   expected account or none. Adds no hook after the wait. I'd write the spec change and a follow-up
   engine order (not on milestone 1's path)." Not chosen: "Leave it for now"; "Allow one late
   sign-out".

2. As put: "The engine's sign-in message for a machine you're not sitting at quotes the spec:
   finish the login 'at it or over a connection Conduit manages, by remote desktop or by SSH,
   pasting the code or forwarding the port'. The earlier Fable review flagged naming those methods
   for your review. Keep the wording?"

   The owner chose "Drop the named methods", whose text read: "Say only 'over a connection Conduit
   manages'. A small spec wording change, then an engine order to match." Not chosen: "Keep it
   (Recommended)".

The notes hold the options not chosen by their labels alone.

## The decisions

1. **A late answer of another account holds the Agent** (answer 1). When an account check that
   was already in flight as a sign-in's wait ended (DL18) answers an account other than the
   expected one, the Agent is held: everything but sign-in and retirement reports unavailable
   until a check finds the expected account or none. No hook is added after the wait. The promises
   it touches are PND-011, which holds the condition; PND-010 and `sign-in.v1`'s description,
   which say what DL18's end does; PND-005, whose hold is the other way an instance waits on
   sign-in; PND-012, for what a record names; and DL18 itself. By the coordinator's reading, open
   to the owner's strike, since the notes hold the option's text and no more:
   - **What it reaches.** By "the Agent" the coordinator reads the instance that `sign-in.v1`
     signs in, an Agent or a Resource, since the operation signs in both. By "an account check that
     was already in flight" it reads an `account-hook.v1` part of `sign-in.v1` in progress when
     DL18 ended the invocation's wait: a poll, or any other account check of that invocation. By
     "a DIFFERENT account than the one expected" it reads any account other than the expected one,
     compared ignoring ASCII case, as PND-011 compares it, and other than the account that the
     check before `sign-in-hook.v1` found. That account counts as no account yet during polling
     (PND-011), and finding it already raised the sign-in action and held the instance under
     PND-005 until a `sign-in.v1` answers signed in, so the question's "nothing holds the Agent"
     did not hold for it. A late answer of it is therefore left as the same answer within the wait
     is, and PND-005's hold stands; counting it would hold every request on an answer that a moment
     earlier counted as no account yet.
   - **The hold is PND-011's `SignInUndone` condition.** The option's held set and its way out, a
     check that finds the expected account or none, are those of the condition that PND-011's
     observable sets when a sign-out does not succeed, and not those of PND-005's hold, which holds
     only model work on an Agent and lifts only when `sign-in.v1` answers signed in. So the late
     answer sets `SignInUndone` to False, with a cause saying that the account check answered after
     the wait had ended, so no sign-out ran. Every other request to the instance, an operation that
     starts its model work among them, reports unavailable with a diagnostic naming PND-011 and the
     condition, and only a later `account-hook.v1` part within `sign-in.v1` or setup that reports
     the expected account or none clears it; reconciliation never clears it.
   - **A setup's parts too.** The option's "everything but sign-in and retirement" is read as
     PND-011's set, which also lets a setup's parts through, since a setup's account check is one
     of the checks that clears the condition, and a held setup could never run it.
   - **What the late answer leaves alone.** It changes no outcome (OPS-013): `sign-in.v1` stays
     unknown, as DL18 ended it, and nothing held resumes. It raises no sign-in action and answers
     none, so PND-005's and PND-012's texts do not change. A late answer of the expected account
     or of none changes nothing and clears nothing. DL18 is unchanged, and its document too: the
     wait is finite, the invocation ends unknown at its end, and no hook runs later.
   - **The end of the wait alone.** The question put the end of the wait. The late answer of a part
     after a cancel of `sign-in.v1` or after a restart, whose parts order 215 ends, is not decided
     here, and stays a late completion, ignored, as order 215 builds it. If the order of A88 finds
     the same gap there, the coordinator brings it back to the owner.
   - **What the session that writes `spec/` writes.** PND-011's observable gains, after the
     sentence on the condition's held requests: "The engine does the same when an account-hook.v1
     part of sign-in.v1 that was in progress when DL18 ended the invocation's wait then reports an
     account other than the expected one and than the one the check before sign-in-hook.v1 found:
     no sign-out-hook.v1 part runs, the invocation stays unknown (OPS-013), and the condition's
     cause says that the check answered after the wait ended." PND-010's sentence "DL18 ending
     without completion ends unknown and no hook runs later" gains "; an account check in progress
     then that later reports an account other than the expected one and than the one found before
     sign-in-hook.v1 holds the instance under PND-011". In `sign-in.v1`'s description, "Failed
     undo holds model work." becomes "A failed undo, or an account other than the expected one and
     than the one the first check found, reported by a check that answers after DL18, holds every
     request to the instance but the parts of sign-in.v1, of a setup and of its retirement
     (PND-011).", which also takes up the amendment of that sentence
     in [order 171b's Record](https://github.com/shrav-k/conduit/blob/b49e8629b087a32f903214ffb517810ab84c471c/docs/work-orders/171b-sign-in-return-and-holds.md#record), which
     found that the sentence "contradicts PND-011 and understates F80's hold". One conformance case shows it: a poll in
     flight as the wait ends answers another account, and the instance's next request is held
     under PND-011 until a later check finds the expected account. The session also rechecks the
     limits in `spec/README.md`, "Pending actions, sign-in and the network" ("A wrong account is
     undone, and a failed undo holds every request to that instance ..."), and IN_FORCE.md's line
     in topic 9 that "A sign-in to another account is undone under PND-011", and extends them or
     records why they stand.
   - **When.** The owner's option names a spec change and a follow-up engine order not on
     milestone 1's path. By the coordinator's reading, one order writes both, the spec change
     first, after the first dogfood milestone ([ruling 89](../ruling-89/README.md), point 3), so
     that the engine and `spec/` change together; until then builds and vectors follow `spec/`.
     It is action item A88.
2. **The sign-in message for carried: false names no method** (answer 2). The owner chose to "Say
   only 'over a connection Conduit manages'": IFC-049's message for a login stating
   `carried: false` names no method. A small spec wording change, then an engine order to match.

   By the coordinator's reading, open to the owner's strike:
   - "Say only 'over a connection Conduit manages'" drops the named methods and keeps "at it or",
     since being at the machine names no method, and the option's label drops the methods alone.
     The message says that nothing comes back through the Interface and that the person completes
     the login on that node's machine, "at it or over a connection Conduit manages", and no longer
     names remote desktop, SSH, pasting the code or forwarding the port.
   - The core `carried` description changes with the message, since it states the same words for
     the same login.
   - The words change in two places of `spec/` and two of the engine, and in no package and no
     case. The limits in `spec/README.md` state the routes too, and keep them (below).
     - `spec/interface/implementation.yaml`, IFC-049's `then`, where "For carried: false it says
       that nothing comes back through the Interface, and that the person completes the login on
       that node's machine, at it or over a connection Conduit manages, by remote desktop or by
       SSH, pasting the code or forwarding the port." becomes "For carried: false it says that
       nothing comes back through the Interface, and that the person completes the login on that
       node's machine, at it or over a connection Conduit manages.";
     - `spec/schemas/contracts.yaml`, the core `carried` description, where "False carries none:
       the person completes the login on the program's machine, at it or over a connection Conduit
       manages, by remote desktop or by SSH, pasting the code or forwarding the port, and no
       process running Conduit's code reads, holds or relays the return (CTR-013, PND-013,
       IFC-049)." becomes "False carries none: the person completes the login on the program's
       machine, at it or over a connection Conduit manages, and no process running Conduit's code
       reads, holds or relays the return (CTR-013, PND-013, IFC-049).";
     - `engine/crates/core/src/sign_in.rs`, the message the engine writes and the test that quotes
       it, which follow the new words in the order of A88.
   - The change is wording only: no promise's `when`, level or observable changes, and no case
     asserts the words (IFC-050).
   - [Ruling 80](../ruling-80/README.md), point 8, is not amended. Its routes, remote desktop and
     SSH by pasting the code or forwarding the port, stand as the owner may take them, over a
     connection Conduit manages, and the limits in `spec/README.md`, "Pending actions, sign-in and
     the network", which point 8 asked to say what it says, keep their words. Only the message and
     the description no longer name the routes. The order of A88 rechecks those limits and
     IN_FORCE.md's line for ruling 80 point 8, which cites IFC-049's message and the core
     `carried` description as where point 8's routes are held, and extends them or records why
     they stand.
   - It is written with decision 1, in the order of A88, after the first dogfood milestone; the
     option's "then an engine order to match" is that order's engine half.

## What it amends

Nothing. Decision 1 settles what `spec/` left open, and decision 2 changes words that order 050b
wrote from ruling 80, point 8, without changing that point's routes (by the coordinator's reading,
open to the owner's strike).

## What it does not change

- **DL18**: the wait, its unknown ending, and that no hook runs after it.
- **PND-005's hold**: an instance held under it stays held until `sign-in.v1` answers signed in.
- **Ruling 80, point 8**: the routes, and the limits in `spec/README.md` that state them
  (decision 2).
- **Ruling 86, point 3 (d)**: a poll that failed or ended unknown counts as no account yet.
- **F95**: the connection Conduit manages binds no person, and the expected account remains the
  only binding.
- **Order 215's decisions**: it changes nothing under `spec/` and leaves S2 and m1 to this ruling,
  so its text stands; its Record is not yet written.
- **Ruling 89's milestones and the path to them**: A88 is not on milestone 1's path.

## What is left for the owner

Nothing new. The coordinator's readings above are open to the owner's strike.

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md), topic 9: a line on the late answer's hold (point 1), and
  one under ruling 80 point 8's line on the message's words (point 2), both written into
  `spec/` and the engine by order 216 (A88).
- [The ledger](../ruling-log.md) gains row 91; no other ruling's status changes.
- [The tracker](../../AUDIT.md#p1--prove-the-language-can-be-rebuilt-and-its-promises-kept) gains
  A88, and [the roadmap](../../../ROADMAP.md#the-order-of-the-work) cites it in step 3.
- Later, and not in this change: the order of A88, written after the first dogfood milestone,
  which writes both decisions into `spec/` and the engine.
