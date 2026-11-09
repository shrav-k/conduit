# Work order 014 (I2-05): the access decision and the tools an Agent is shown

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates
and for tests that bind a loopback port; use it for nothing else.

## What and why

I2-05 of the [study](../proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders),
the part of the core that is Conduit's own: whether a request may go ahead, and which tools an
Agent is shown. A request is attributed to one instance (IFC-022) and then decided by that
instance's open Conduits, nothing else; an Agent is shown exactly the tools of its open
Conduits (IFC-012). It builds on registration (order 012).

Read: the "The access decision" table of the study's section 2, for the ACC and IFC promises it
lists; the rows S1-exact-access, S1-pool-authority and S1-plain-filter of section 3; the
[decisions](../proposals/first-build/increment-2/decisions.md); experiment 008's
[results](../../experiments/008-real-client/results-2026-11-09.md), which show what a real
client does with a refusal; and the core as order 012 left it.

## The decisions to apply

1. **The access decision is pure**: given the state, the instance that a request was
   established for (or none, or one whose existence cannot be read, under ACC-037), the tool it
   names and its arguments, the core decides as the ACC promises say, routing under ACC-033,
   applying each restriction's bounds, and citing the promise of every outcome. Refused and
   invalid follow the precedence that OPS-023 gives, and a refusal discloses no named object
   (IFC-025).
2. **The tools an instance is shown** are exactly those of IFC-012: one per operation of each
   open Conduit, named by the Conduit's local name and the operation's export name, defined as
   IFC-033 says through order 006's publication, and nothing else. A tool the server offers that
   no contract maps is never shown (CON-017), and the tool list changes when a Conduit opens or
   closes (IFC-013, IFC-038).
3. **Vectors**: S1-exact-access, S1-pool-authority and S1-plain-filter as the study writes them;
   A and B's lists with A's Conduit listing reads and B's reads and writes; a guessed tool
   refused with no invocation; a request from no instance, and from one whose existence cannot
   be read; and each restriction's bound at its limit and past it. Every decision cites its
   promise.
4. **A finding**: as order 004's decision 6 says.

## Scope

As order 009's scope.

## Done when

The vectors pass, with A and B each shown exactly their tools and every request outside them
refused without an invocation or a disclosure; every decision cites a promise; and the checks of
order 004's done-when pass.
