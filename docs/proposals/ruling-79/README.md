# Ruling 79: Opus verifies each work order and coordinates; Fable is kept for the owner's reviews of the whole language and design

Status: in force; amended by 80 (points 2 and 4: an Opus read of round 1 on A77's paired orders, and paired reads stopped before dispatch) (ruled by the owner on 2026-11-14 and 2026-11-15). Holds now: [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed). This record says why.

Ruling 48 gave the work to Codex sessions so that the coordinator's own session would be spent
on planning and decisions: "Experiments, increments, sweeps, verifications and fresh reads are
given to Codex sessions as work orders" ([ruling 48](../ruling-48/README.md#how-the-work-is-done-from-now-on)),
for the owner's reason, "so as to not burn all my tokens here". The full review of 2026-11-14
found that a fresh Codex verifier is a productive reader but not a test of being done
([FR-491](../full-review/findings/G4-who-verifies.md#fr-491),
[FR-500](../full-review/findings/G4-who-verifies.md#fr-500)).

The owner's words, on 2026-11-14, while the review ran on Claude Fable: "Leave implementation and
test running to codex or opus", then "Wait no, reviews should be fable, but implementation should
be opus, I wanted the review to be done with fable for maximum accuracy". When Fable's allowance
ran out: "Try to finish fable's work, we ran into our limits". Asked whether the preference also
covers the verifier that reads every work order, the owner chose "Every order (Recommended)",
whose text read: "A fresh Fable reader verifies every order once the checks pass, with a Codex
reader as a second view on risky orders. When the Claude allowance runs low, dispatch stops first
and I tell you; nothing merges unread." With Fable's allowance out, the owner let orders 151 and 152
merge on their Codex verifiers and the coordinator's read ("Merge these two now"), and then, on
2026-11-15: "Honestly I'm ok with using fable exclusively for review and handling merge and
dispatch with opus", and then: "I meant reviews of the overall language and design not each
individual pr, do this with opus for now as well. We will have manual fable reviews of the entire
codebase and architecture/structure".

The answers change who verifies, a decision ruling 48 gave, so they are a ruling and not a later
answer (ruling 67). The owner's last words settle what "reviews" meant: the whole language and
design, not each pull request.

## The decisions

1. **Fable is kept for reviews of the whole.** Claude Fable reviews the language and the design
   as a whole: the entire codebase, its architecture and its structure. The owner starts these
   reviews by hand. The full review of 2026-11-14 was the first.
2. **Opus verifies each work order, for now.** Each order, once its checks pass, is read by a
   fresh, read-only Claude Opus reader before it merges, beside the fresh Codex verifier that reads
   each round. The coordinator's own fixes to an order are read the same way. The owner said "for
   now": this is a shortcut, and the owner may move the reading of each order to another model
   later.
3. **Opus coordinates; Codex or Opus implements.** The coordinator's session runs on Claude Opus:
   it plans, writes and dispatches the orders, decides, and merges. Work orders' implementation
   and their test runs go to Codex workers or to Claude Opus.
4. **When the Claude allowance runs low, dispatch stops first, and nothing merges unread.** The
   coordinator tells the owner. An order whose worker has finished waits for its read; the owner
   may say otherwise for an order.
5. **When Fable's allowance runs out in the middle of a review,** the owner may ask for the review
   to be finished on Opus, as on 2026-11-14; the record says which model ran which part.

## What it amends

- **Ruling 48**, its direction that verifications are given to Codex sessions: each order is now
  also read by a fresh Opus reader before it merges (point 2). The coordinator stays the admin
  session of ruling 48, run on Opus (point 3). Codex still carries out work orders, and fresh reads by Codex still
  test what a reader without context can do (ruling 48, point 4; ruling 71, point 3).

## What it does not change

- The fresh author of the S1 rerun stays a Codex session, and both runs must pass (rulings 48
  point 4, 71 point 3 and 78 point 2).
- The second accounts stay reserved for Conduit's own sign-in and isolation test, and are not
  capacity for reading (ruling 72's later answers).
- When the Codex allowance runs out, workers pause and the coordinator tells the owner (ruling 71
  point 6 and its later answers).
