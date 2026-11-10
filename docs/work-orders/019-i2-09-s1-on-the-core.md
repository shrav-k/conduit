# Work order 019 (I2-09): S1 whole on the core, and what increment 3 receives

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates
and for tests that bind a loopback port; use it for nothing else.

## What and why

The last order of increment 2, I2-09 of the
[study](../proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders).
Orders 004 to 017 built the core in slices; this one runs S1 through it whole, checks that
increment 2's done-when in [the first build](../implementation/FIRST_BUILD.md#increments) holds,
and writes down what the shell of increment 3 receives from it. It claims nothing about real
Agents: none runs here.

Read: the first build's increments 2 and 3 and "What increment 3 needs"; the study's sections 1
and 7; [the decisions](../proposals/first-build/increment-2/decisions.md); the
[log of increment 2](../proposals/first-build/log-increment-2.md); `conformance/` and its
manifest; and the core as order 017 left it.

## The decisions to apply

1. **S1 whole**: one vector, or a short chain of them, that takes S1 from first start through
   registration, setup and the opening of its four declared Conduits, then T1's and T2's
   requests (A reads the directory, B reads and writes it, both post to and read the history),
   then T7's (A's Conduit to the directory closed, B still working, A's restored by explicit
   replacement), each request ending and recorded as orders 015 and 017 say, with the scripted
   completions of the shell standing in for real servers.
2. **Coverage**: the conformance manifest lists, for every promise that the study's section 2
   says S1 touches, the vectors that show it, or says why none does; a promise S1 touches with
   no vector and no reason is a gap to fill or to log.
3. **The first build's conformance document**: `reference-model/CONFORMANCE.md`, stating each
   choice the first build makes on the implementation-defined list (the profile's, and those
   orders 004 to 017 made), each with its DL item and where it is shown.
4. **What increment 3 receives**: `docs/proposals/first-build/increment-3-handoff.md`, not
   binding: the core's interface as the shell will call it, every intent the core emits and
   every completion it expects, with their shapes; the tool contract the shell presents; and
   what increment 3 must still build or show, from "What increment 3 needs" and experiments 008
   and 009, each as an item the shell's orders can take.
5. **The done-when of increment 2**, checked and reported item by item: S1's documents register
   and resolve; every decision cites a promise; the safety vectors pass; each case the core could
   not decide is a logged finding, settled or left for the admin.
6. **Lean vectors**: `conformance/` holds about 8 MB of cases, many of them whole states of up to
   90 KB. Report what makes them large, and propose how to keep them small and readable, such
   as shared fixtures for a common state and projections in place of whole states, as decision
   7 asks of records. Do not rewrite them in this order; say what a later order would change.
7. **A finding**: as order 004's decision 6 says.

## Scope

As order 009's scope, and the two new files of decisions 3 and 4.

## Done when

S1 runs whole on the core by vectors; the coverage manifest has no unexplained gap; the
conformance document and the handoff are written; the done-when of increment 2 is reported item
by item; every decision cites a promise; and the checks of order 004's done-when pass.

## Record

Written by the admin when it merged the order, on 2026-11-10.

- **Worker**: a first run of 2026-11-09 was lost with the session's scratch space, before it was
  committed; the order ran again, 16 minutes, then a second round. S1 runs whole on the core as
  a 58-step vector; every promise S1 touches has vectors (217) or a stated reason (91); the
  first build's conformance document and the handoff to increment 3 are written; increment 2's
  done-when is reported item by item.
- **Verifier**: 5 minutes. Three `must` in the handoff and the conformance document, and one
  `should`: steps of the whole-S1 vector that did not check their records.
- **Admin**: sent the worker back once; each of the 75 record expectations added now fails the
  vector when removed. Marked increment 2 done.
