# Work order 017 (I2-08): records, inspection, and the bounds of repair

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates
and for tests that bind a loopback port; use it for nothing else.

## What and why

I2-08 of the [study](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders):
every order so far has emitted records as it went; this one completes them and checks them
whole, adds inspection, and the repairs the promises allow. Who did what, through which
Conduit, is the accountability Conduit promises.

Read: the "Records" table of the study's section 2; the rows on records, inspection and repair
in section 3, among them E08, E11 and E12; [decision 7](../proposals/first-build/increment-2/decisions.md),
which fixes the vocabulary of events and how records are compared; REC-001 to REC-003, LIF-013,
DL8 and D19 of round 6; and the core as order 016 left it.

## The decisions to apply

1. **Every record the promises require**, with every field `record.v1` asks for, validated
   against it, from every transition of orders 004 to 016, the access decision's record before
   any dispatch it allows; the event types of decision 7, documented in the profile.
2. **Inspection** as LIF-013 and the promises the study lists say, compared by decision 7's
   projections, which never hide an omission.
3. **Retention and purge**: purging records leaves the invocation index whole (order 015), and
   a gap that a purge leaves is shown as one.
4. **Repair's bounds**: a follow-up by expected ID, a template replaced with its revision
   checked, and E08's narrow broker case, with no implicit effect, as section 3 says.
5. **Counting** for D19: the bytes of records and of the invocation index, apart, and their
   growth per request, reported by the runner.
6. **Vectors**: every row of section 3 on records, inspection and repair; every decision cites
   its promise.
7. **A finding**: as order 004's decision 6 says.

## Scope

As order 009's scope.

## Done when

The vectors pass; every record validates against `record.v1`; the runner reports D19's counts;
every decision cites a promise; and the checks of order 004's done-when pass.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 41 minutes, then a second round. Every record validated against `record.v1`,
  inspection, retention and purge that leave the invocation index whole, repair's bounds, and
  D19's counts (a keyed unknown write's index entry kept 373,365 bytes after a purge, as REC-004
  and OPS-008 require); 11 vectors, then 361 in all. It stopped at 2-08: which Pool's authority
  inspects an object that no longer exists.
- **Verifier**: 6 minutes. Three `must`: a successful keyed registration repeated could report
  conflict; template inspection could give a status its schema forbids; and a refused keyed
  request could lose its key in the record.
- **Admin**: settled 2-08 by the least disclosure (only management's authority reaches an object
  that does not exist; a Pool's alone is refused), and sent the worker back once with it and the
  three findings, each fixed with a vector that failed before. It then found that OPS-029 had
  decided 2-08's case the other way, on purpose, and kept its settlement as a choice against
  disclosure, amending OPS-029 and saying so in the log: this is the decision of increment 2 the
  owner may most want to look at. `conformance/` now holds about 8 MB of cases, many of them full
  states; order 019 is asked to report how to keep them lean.
