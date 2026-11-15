# Ruling 79: a fresh Fable reader verifies every work order, and Opus or Codex implements

Status: in force (ruled by the owner on 2026-11-14). Holds now: [IN_FORCE.md: 15. How the work is governed](../../../IN_FORCE.md#15-how-the-work-is-governed). This record says why.

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
and I tell you; nothing merges unread."

The answer changes who verifies, a decision ruling 48 gave, so it is a ruling and not a later
answer (ruling 67).

## The decisions

1. **Reading is Fable's; doing is Codex's or Opus's.** Reviews, the verification of each work
   order and the coordinator's decisions run on Claude Fable, the most capable model. Work orders'
   implementation and their test runs go to Codex workers or to Claude Opus.
2. **Every order is verified by a fresh Fable reader** once its checks pass, read-only, before it
   merges. A fresh Codex verifier stays as a second view on risky orders. The coordinator's own
   fixes to an order are read the same way.
3. **When the Claude allowance runs low, dispatch stops first, and nothing merges unread.** The
   coordinator tells the owner. An order whose worker has finished waits for its Fable read; the
   owner may say otherwise for an order.
4. **When Fable's allowance runs out in the middle of a review,** the owner may ask for the review
   to be finished on Opus, as on 2026-11-14; the record says which model ran which part.

## What it amends

- **Ruling 48**, its direction that verifications are given to Codex sessions: verification is now
  Fable's (points 1 and 2). Codex still carries out work orders, and fresh reads by Codex still
  test what a reader without context can do (ruling 48, point 4; ruling 71, point 3).

## What it does not change

- The fresh author of the S1 rerun stays a Codex session, and both runs must pass (rulings 48
  point 4, 71 point 3 and 78 point 2).
- The second accounts stay reserved for Conduit's own sign-in and isolation test, and are not
  capacity for reading (ruling 72's later answers).
- When the Codex allowance runs out, workers pause and the coordinator tells the owner (ruling 71
  point 6 and its later answers).
