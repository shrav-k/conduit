# Work order 012 (I2-04): registration and the resolving of instances

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates;
use it for nothing else.

## What and why

I2-04 of the [study](../proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders):
the core registers a submitted project atomically and resolves its instances, so that S1's
documents become objects. It builds on first start (order 004), the schemas (order 006) and the
contexts of packages and projects (order 009).

Read: the study's section 1 ("Registration and setup trace") and the "Registration and
resolving" table of section 2, for the OBJ, INS, SUB, NAM and TPL promises it lists; the
[decisions](../proposals/first-build/increment-2/decisions.md); S1's fixtures under
`conformance/fixtures/s1/`; and the core and the runner as order 009 left them.

## The decisions to apply

1. **Registration is atomic** (SUB-003): a submission registers every object it holds or none,
   and a rejected one publishes nothing and starts nothing.
2. **Resolving an instance** under the promises the study lists: its template, its arguments
   with defaults filled, its references stored by ID and never by name, its owner, its
   placement, and the nested objects of its declared slots, with generated names reported as
   DL15's choice in the profile says.
3. **No effect**: registration makes no setup, no run and no call; those are I2-07's.
4. **Order has no meaning** (SUB-001): S1's documents register to the same objects and
   diagnostics in any order of files and of documents.
5. **Vectors**: S1's project registers, with the objects the study counts (15 named and four
   nested) or the count the promises give if that differs, which you report; each rule of
   decision 2 with an accepted and a rejected case; a rejected variant of S1 that publishes
   nothing; the same result in several orders. Every decision cites its promise.
6. **A finding**: as order 004's decision 6 says.

## Scope

As order 009's scope, and `conformance/fixtures/s1/` where a fixture needs correcting, which
you report.

## Done when

S1's documents register and resolve on the core by vectors, in any order, with nothing
published on a rejection and no effect; every decision cites a promise; and the checks of order
004's done-when pass, with every vector run by `conduit-conformance`.
