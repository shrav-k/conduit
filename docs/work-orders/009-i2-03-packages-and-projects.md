# Work order 009 (I2-03, first part): packages and projects, from snapshots

You are a Codex worker carrying out one work order in this repository. Read
[AGENTS.md](../../AGENTS.md), its section "If you were given a work order" above all, then this
order. Your report must match the schema in `004-i2-01-vectors-and-first-start.schema.json`, the
schema every build order of increment 2 shares. This order has network access, to fetch crates;
use it for nothing else.

## What and why

The first part of I2-03 of the
[study](https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/plan.md#7-a-proposed-sequence-of-work-orders): the
contexts in which a document's names resolve. A submitted project and the packages it pins give
each document the contracts and templates its names reach, and the core resolves them without
fetching anything. The admin split I2-03 in two, as the study allowed: this part works from
preinstalled snapshots of packages, as [decision 2](../proposals/first-build/increment-2/decisions.md)
says; installing from an archive, with real bytes and hashes, is the second part, its own
order.

Read: the study's section 1 and the "Registration and resolving" table of section 2, for the
PKG, MAN and NAM promises it lists; the drafts under
[https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/s1](https://github.com/shrav-k/conduit/tree/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/increment-2/s1); the decisions; SUB-008 for the
digest of a submitted project; and the core, the runner and the vectors as orders 004 and 006
left them.

## The decisions to apply

1. **A catalog of installed packages**, given to the core as an input: each a snapshot of an
   address, a version, a digest and its loaded documents, marked as a snapshot, never fetched.
   The core keeps it immutable once given.
2. **Resolving a project**: its manifest's pins against the catalog, its exports and names
   under the PKG, MAN and NAM promises the study lists, the qualified references of SCH-015,
   and the core package's contracts, which no package may replace (NAM-020, FST-005). A missing
   or a mismatched pin is rejected, under the promise that requires it, and nothing is fetched.
3. **The digest of a submitted project** as SUB-008 says, computed from the manifest as
   submitted, never from paths the runner expanded or its fixtures' metadata.
4. **S1's packages and project**: move the drafts of the filesystem server's package, the
   history's and the two Agent types', with S1's project, into `conformance/fixtures/s1/`, with
   their digests computed rather than illustrative, and show by vectors that they resolve.
5. **Vectors** for each rule above, an accepted and a rejected case for each, every decision
   citing its promise, and none that claims an installation: a snapshot is not one.
6. **A finding**: as order 004's decision 6 says.

## Scope

Create and change: `conformance/`, `reference-model/crates/core/`,
`reference-model/crates/conformance/`, `reference-model/Cargo.toml` and `Cargo.lock`,
`reference-model/README.md`, and `https://github.com/shrav-k/conduit/blob/f1cb473493fbb7b8667686d6fdf639ae96579ec3/docs/proposals/first-build/log-increment-2.md`. Report
anything else; change nothing under `spec/` unless a finding is settled in this order, which
none is yet.

## Done when

S1's packages and project resolve from snapshots by vectors; a missing or mismatched pin is
rejected without a fetch; the core package's contracts cannot be replaced; every decision
cites a promise; and the checks of order 004's done-when pass, with every vector run by
`conduit-conformance`.

## Record

Written by the admin when it merged the order, on 2026-11-09.

- **Worker**: 23 minutes, then a second round. An immutable catalog of snapshots, the resolving
  of a project's pins, exports and names, SUB-008's digest, and S1's fixtures with computed
  digests; 58 vectors, 104 in all, then 107.
- **Verifier**: 5 minutes. Three `must`: a binding through a parent that may be null was
  accepted; a template's override of an implementation was matched by its spelling rather
  than by the operation it resolves to; and normalizing `..` in a relative program could
  change which program it names.
- **Admin**: sent the worker back once with the three findings; each fix has a vector that
  failed before it. Kept the published tarball of the filesystem server (36 KB) as a fixture,
  so that its pin's hash is real. **Corrected the same day**: the admin had written that the
  tarball carries its licence; it does not (its `package.json` says "SEE LICENSE IN LICENSE",
  and no such file is inside). Order 013 removes it, and keeps only its hash.
