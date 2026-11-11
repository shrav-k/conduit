# Ruling 48: what the first build's review put to the owner, and who does the work

Status: **ruled by the owner on 2026-11-09** (plan-audit ruling 48). This is the record of a
ruling. Where it and the specification differ, the specification governs.
[Ruling 47](../ruling-47/README.md), the owner's pull request 14, is recorded with its public
name and namespace open.

After increment 1 of the first build and the baseline for S1 (A70), the owner asked: "Why are
we using a token rather than our subscription for the codex and Claude tests? Are there any
findings you think I should take a look at?" The coordinator answered the first question: the
runs used the subscriptions, and the token is Conduit's own credential, by which the engine
knows which Agent is asking (IFC-022, IFC-042). For the second it had a review read both
results against the specification, each angle checked by a skeptic and the whole by a critic,
and put four points to the owner with a recommendation each. The owner's answer: "Agree with
all recs, except for 4 let's do fresh reads with codex instead for testing lack of context. I
also want codex to do some of these experiments going forward as well so as to not burn all my
tokens here but you are the admin session and should handle all of the meticulous planning and
decisions. Let's take our learnings from this experiment to figure out how to be more
autonomous for future sessions so that we can leverage codex. We could even figure out how to
run these experiments leveraging a session within conduit itself".

## The points and the answers

| # | Point | Answer |
| --- | --- | --- |
| 1 | What an Agent that Conduit starts may reach through its own login: the claude.ai connectors of a Claude login, the apps of a ChatGPT login, and Claude Code's messaging and scheduling tools, none of which a document declares and some of which ask no one | **a**: off by default in the first build's Agent types, and each can be turned on for one Agent by an input of its template. A service of the account that Agents should share becomes a Resource behind a Conduit |
| 2 | Which login to its model an Agent that the engine runs uses | **a, then b**: in increment 3 the Agents share the owner's login, kept apart by the flags that A70 used; once A4 and A5 have run, each Agent has a configuration home of its own with its own subscription login. A8's check of the subscriptions' terms is widened to Agents that the engine runs |
| 3 | The regular expressions that a `pattern` may use | **b**: the tokens that JSON Schema 2020-12 asks schema authors to keep to for interoperability (core, section 6.4), read as ECMA-262 in Unicode mode. The specification's own patterns are rewritten to keep to them. I-Regexp is not adopted |
| 4 | Who tests the success criterion that a competent reader can complete S1's checked path | **Codex, reading fresh** (the owner's answer, in place of the recommendation of a fresh session of either agent): in increment 4 a Codex session that has only the specification and S1's tasks does T1 to T7, and its measures are recorded beside the coordinator's. Fresh reads by Codex test what a reader without context can do |

## How the work is done from now on

The owner's direction, beside the four points:

- **Codex does work.** Experiments, increments, sweeps, verifications and fresh reads are
  given to Codex sessions as work orders, so that the coordinator's own session is spent on
  planning and decisions.
- **The coordinator is the admin session.** It writes each order, decides what the order
  applies, reviews what comes back, and merges. What an order decides is the coordinator's
  decision, logged and strikeable as before (rulings 37, 38 and 46, point 4).
- **More autonomy, learned from this round.** What the coordinator learned from running A70
  and increment 1 with real agents goes into how orders are written and dispatched.
- **Toward running the work through Conduit.** Once increment 3 runs, the orders themselves
  are to go through Conduit: a Codex worker reaches its order's working tree and the queue of
  orders through Conduits that the admin session declares. That is S1 in daily use.

## What this settles

- R2, M3, 1-04 and M4 of the review, the four points above.
- The coordinator's other findings from the review are its own to fix, under ruling 46, point
  4, and are applied by the first work order.
- How work is divided between the admin session and Codex, written in
  [work orders](../../implementation/WORK_ORDERS.md).

## What follows

1. [Work orders](../../implementation/WORK_ORDERS.md): the roles, the form of an order, how
   one is dispatched and checked, what this round taught, and the path to running orders
   through Conduit (A77, A78).
2. [The first build](../../implementation/FIRST_BUILD.md) gains the requirements of increment
   3 that the review found, the counting rules it asked for, and the fresh Codex author of
   increment 4.
3. The [roadmap](../../../ROADMAP.md#experiments-and-probes) runs A4 before increment 3, adds
   cases to A71, widens A8, and registers A77 and A78.
4. Work order 001 applies the coordinator's fixes from the review and point 3.
