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

- Changed: SCT-002 (DEC-007).
- Added: `spec/README.md#what-the-short-forms-meaning-covers`, ruling 76 points 2 to 5: each flag names its object, commands use the current engine context, apply goes on past a sign-in and answers none, and `conduit plan` is never required (DEC-008).
- Fixed: `spec/README.md#what-the-short-forms-meaning-covers`, a cascade's one-time Conduit is closed and removed by its target's retirement, and apply sends no close (DEC-008).
- Changed: `spec/README.md#an-author-of-documents`, its sectioned package example, which now pins no S1 package and defines none of S1's templates (DEC-007, DEC-039, ruling 78 pt 2).
