# Ruling 81: the sign-in orders held until the targeted review, Fable for that review, and A5's stage 0b

Status: in force (ruled by the owner on 2026-11-16). Holds now: [IN_FORCE.md](../../../IN_FORCE.md), topics 9 and 15, and A5's row of [ROADMAP.md](../../../ROADMAP.md#experiments-and-probes). This record says why.

After [ruling 80](../ruling-80/README.md) merged, later on 2026-11-16, the coordinator put four
questions to the owner through the session's question tool, each with options and a
recommendation. The questions as put, their options and the owner's answers:

1. "Run the next targeted review on Fable, with this target: can the Agent packages 0.2.0's
   sign-in and home surface be frozen (logins, account/API key, hooks, homes, the managed sign-in
   connection)? It would start once SO-A and a short Codex source read are on main, likely
   Nov 17-18. Ruling 79 keeps Fable for whole reviews, so this needs your call." Its options:
   "Yes, Fable, this target (Recommended)"; "Opus readers, Fable verifies"; "Different target".
   The owner chose "Yes, Fable, this target (Recommended)".
2. "Hold writing the sign-in orders (048a-1, 048a-2, 048b-join, 050, 051b) until the review's
   answers are in? The earlier plan (D18) dispatched 048a-1 now, beside 046a and 047, but A5's
   Codex failures and your managed-connection decision both came after that plan." Its options:
   "Hold them (Recommended)"; "Dispatch 048a-1 now". The owner chose "Hold them (Recommended)".
3. "Approve A5 stage 0b? Part A has an order read Codex's source and docs to find how its login
   opens a browser by itself, with no client run and none of your time. Part B is ~10 min of you
   watching Codex's login start in a scratch home with stage 0's guards (a browser with no
   OpenAI/Anthropic session), cancelled before any sign-in." Its options: "Parts A and B
   (Recommended)"; "Part A only"; "Neither". The owner chose "Parts A and B (Recommended)".
4. On ruling 80's point 8: "Ruling 80 (now merged) records your sign-in decision as: Conduit
   'opens, limits and closes the connection' over which the person completes a Claude sign-in.
   You said 'conduit does manage this connection'; 'opens, limits and closes' was my wording.
   Which should the record say?" Its options: "'Manages'; how is 050's (Recommended)"; "Keep my
   wording". The owner chose no option and typed: "What does this wording distinction even mean
   concretely? Your wording seems reasonable".

Answers 1 to 3 are this ruling, as ruling 67 has it and as ruling 80 sorts its own answers.
Answer 1 changes a ruling's point: ruling 79 point 1 keeps Fable for the owner's reviews of the
whole, and the owner chose it for one targeted review. Answer 2 settles differently a decision
the full review logged for the owner to strike: D18
([decisions.md](../full-review/decisions.md#the-pipeline-and-verification)) had 048a-1
dispatched beside 046a and 047. Answer 3 decides something new: no ruling or register held a
stage 0b of A5. Answer 4 changes no decision. Point 8's wording stands as the owner's, and the
answer is a later answer in [ruling 80's record](../ruling-80/README.md#later-answers). That
sorting is the coordinator's.

## The decisions

1. **The sign-in orders wait for the targeted review** (answer 2). Orders 048a-1, 048a-2,
   048b-join, 050 and 051b are neither written nor dispatched until the record of the targeted
   review of the sign-in and home surface that the Agent packages 0.2.0 would freeze (point 2),
   and the owner's answers to it. The names are the admin's split of batch 1's orders 048a, 048b,
   050 and 051b, in [order 117](../../work-orders/117-plan-s2-s3.md). D18 had 048a-1 dispatched
   beside 046a and 047, with 048b-relay. As the question said, A5's Codex failures in stage 0 and
   the managed connection of ruling 80 point 8 both came after that plan.
2. **Fable runs that one targeted review** (answer 1). Ruling 79 point 1 keeps Claude Fable for the
   owner's reviews of the whole language, design, codebase and architecture, started by hand. For
   this one review, the owner chose Fable. Its target, as the question put it: can the Agent
   packages 0.2.0's sign-in and home surface be frozen. The question named its parts: logins,
   account/API key, hooks, homes and the managed sign-in connection. The coordinator reads them,
   under NAM-024, as CTR-013's `signIn` and its logins, `account` or `apiKey`, the hooks, each
   Agent's home, and the connection Conduit manages for a Claude Agent's sign-in ([ruling
   80](../ruling-80/README.md), point 8). The coordinator's brief also has the review read what the
   full review's orders 16 and 17 add to the wrappers ([its orders](../full-review/orders.md)),
   which 0.2.0 would freeze with that surface. That addition is the coordinator's, open to the
   owner's strike. As the question put it, the review starts once SO-A and a short read of Codex's
   source are on `main`. This sets no standing rule: another targeted review runs on Fable only if
   the owner chooses it. The review's questions stay open until its record and the owner's answers,
   and nothing in this ruling settles one; [IN_FORCE.md](../../../IN_FORCE.md), topic 9, lists them
   as *Open*.
3. **A5's stage 0b is approved, parts A and B** (answer 3). In stage 0, Codex failed both of its
   questions: its status named no account, and its login opened a browser past the shim
   ([the results](../../../experiments/027-a5-sign-in/results-2026-11-16.md)). Stage 0b asks how
   Codex's login opens a browser by itself, and what stops it. In part A, an order reads Codex's
   source and documentation, with no client run and none of the owner's time. In part B, the
   owner watches Codex's login start in a scratch home for about ten minutes, under stage 0's
   guards, in a browser that holds no OpenAI or Anthropic session, and the login is cancelled
   before any sign-in. Part B runs only at a time the owner picks, and signs nothing in. Order
   195 is to pre-register stage 0b in
   [experiment 027's README](../../../experiments/027-a5-sign-in/README.md), as that README asks
   of each stage before it runs, and to carry out part A. Stage 0b gathers facts and decides
   nothing.

## What it amends

- **Ruling 79**, point 1, for one review only: the targeted review of 0.2.0's sign-in and home
  surface runs on Fable (point 2). Point 1 otherwise stands.

The full review's logged decision that these answers settle: D18, for its dispatch of 048a-1
beside 046a and 047 (point 1). The rest of D18 stays the coordinator's, open to the owner's
strike.

## What it does not change

- **The rest of D18**: 048b-relay, the shell's relay of `input_required`, is not held, and D18's
  other steps stand as written.
- **Ruling 80, point 8**: its wording stands as the owner's (answer 4, a later answer in
  [ruling 80's record](../ruling-80/README.md#later-answers)). What "opens, limits and closes"
  means concretely is the managed connection's design, which order 050 writes once the targeted
  review's record and the owner's answers are in (point 1).
- **Ruling 80, point 7**: order 050 stays next on the path to a Fleet that runs in parallel. It is
  written once the targeted review's record and the owner's answers are in (point 1). The review's
  order 62 is not held.
- **A5's other stages**: stages 1 to 3 stay as registered, and A5 gates nothing
  ([ROADMAP.md](../../../ROADMAP.md#experiments-and-probes), A5).

## What follows

- [IN_FORCE.md](../../../IN_FORCE.md): topic 9 gains an *Open* line that lists the targeted
  review's questions, which no session settles until its record and the owner's answers. Topic
  15 points to point 2 for this one review, and its *Coordinator's* line names D18's dispatch of
  048a-1 as settled here.
- [The tracker](../../AUDIT.md#waiting-for-the-owner) lists the owner's answers to the review's
  record, before the five orders are written.
- The targeted review runs on Fable once SO-A and the read of Codex's source are on `main`. Its
  record and the owner's answers decide when the five orders are written, and with what.
- Order 195 pre-registers stage 0b and carries out part A. The coordinator runs part B when the
  owner picks a time.
