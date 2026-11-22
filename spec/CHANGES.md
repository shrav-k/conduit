# Changes to the specification

Status: draft, not yet in force; [the decisions' README](../docs/decisions/README.md) says what
holds until it is. This file adds no behavior: [`spec/`](README.md) binds alone.

The form follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/). Each change to `spec/`
adds one line under `Unreleased`, of the form `- Kind: the promise IDs it changes (decision).`
A prose change names a `spec/README.md#anchor` or a definitions term instead of IDs. Kind is
Added, Changed, Deprecated, Removed, Fixed or Security, and the decision is `DEC-NNN` in
[the log](../docs/decisions/log.md) or "ruling N" for an archived ruling. For example:
`- Removed: IFC-011, handle.v1 (ruling 78 pt 16).` A release moves the `Unreleased` lines under
its own heading.

Decisions before this file are rulings 1 to 92 and the log's DEC-001 to DEC-032. The rulings leave
the tree at their freeze and are then cited by links pinned to their last commit (DEC-031); until
then they are under `docs/proposals/`.

## [Unreleased]

- Changed: PND-010, PND-011 and `sign-in.v1`'s description: an account check that answers after DL18 ended the sign-in's wait changes nothing but another account's hold, and a late expected account's diagnostic names a new sign-in.v1 (DEC-043).
- Changed: PKG-028's and NAM-012's observables and `spec/README.md#an-author-of-documents`: the reference CLI may install an archive's missing dependencies first, each by its own request, from a cache or mirror copy it picks, and pins only that archive, exactly (DEC-048).
- Changed: SCT-002 (DEC-007).
- Added: `spec/README.md#what-the-short-forms-meaning-covers`, ruling 76 points 2 to 5: each flag names its object, commands use the current engine context, apply goes on past a sign-in and answers none, and `conduit plan` is never required (DEC-008).
- Fixed: `spec/README.md#what-the-short-forms-meaning-covers`, a cascade's one-time Conduit is closed and removed by its target's retirement, and apply sends no close (DEC-008).
- Changed: `spec/README.md#an-author-of-documents`, its sectioned package example, which now pins no S1 package and defines none of S1's templates (DEC-007, DEC-039, ruling 78 pt 2).
- Added: REC-007, a request whose access decision or dispatch cannot be recorded reports unavailable, as does one found undispatched at a restart; a dispatched one whose end cannot be recorded ends unknown, unless its implementation answered with a reason and no outcome, which stands; each record is kept late; records held for a late keeping are lost if the engine stops first, and recovery ends what they leave as IMP-007 does (DEC-044).
- Changed: REC-001, a request's record that cannot be kept is kept late, under REC-007 (DEC-044).
- Changed: SEC-016, REC-007 for a read of a secret: unavailable at once, and recorded late (DEC-044).
- Changed: OPS-023, unavailable under REC-007 comes after ACC-037's and ahead of refused (DEC-044).
- Changed: IMP-007, a request found undispatched at a restart ends unavailable under REC-007 (DEC-044).
- Changed: DL10 leaves storage exhaustion beyond REC-007 to each engine (DEC-044).
- Changed: SEC-024, an IPv4 address is RFC 3986's IPv4address, `[::1]` is compared as an IPv6 address, and its restatements in `spec/schema-profile.md` and `spec/core/README.md` become citations (DEC-044).
