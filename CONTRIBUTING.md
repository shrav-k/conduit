# Contributing to Conduit

Conduit is currently a protocol/language design with a planned Rust reference implementation.
There is no runnable runtime, published package format or implemented SDK yet. Start with the
[documentation map](docs/README.md) and [current review](docs/REVIEW.md).

## Where work belongs

- Change accepted behavior in [docs/protocol/](docs/protocol/README.md), explicitly identifying changes
  to guarantees. Update related examples/conformance cases in the same review.
- Develop original extensions through [docs/authoring/](docs/authoring/README.md). Preserve unapproved
  representations in [docs/proposals/authoring-syntax/](docs/proposals/authoring-syntax/README.md).
- Keep runtime mechanisms in [docs/implementation/](docs/implementation/README.md). A library, crate or
  database layout is not automatically part of the public protocol.
- Put sourced comparisons and actual probes in [docs/research/](docs/research/README.md). Attribute
  source versions and distinguish inference from observed behavior.
- Keep [experiments/](experiments/README.md) outside docs with scope, instructions and evidence.
  Expected outcomes must not be presented as passed tests.
- Preserve completed reviews, attribution and dissent in [docs/archive/](docs/archive/README.md).
  Accepted rules must remain accessible in the reference, not only in history.

## Review discipline

The published language-neutral contract is authoritative; implementation code and static examples
do not silently redefine it. Plugin/template authoring and independently consumable conformance are
central. Test public boundaries without private runtime imports or undeclared local checkouts.
Distinguish protocol behavior, reference-implementation choices and optional capabilities.

There is [no backward-compatibility promise between releases](docs/protocol/CONFIGURATION.md#release-compatibility).
Keep the contract a release claims to implement explicit; pinning a working set is the continuity
mechanism. This does not imply maintenance of old releases or mandatory migration tooling.

Use one current review queue. Record an unresolved question with its affected guarantee, a concrete
example and the choice needed. Do not reopen settled concepts because older research used different
names. Separate mechanical moves from semantic edits where practical; repair links and preserve source
pins. The repository holds canonical documentation sources for a future website, without a second
editable reference. Exact syntax remains for the designated authoring pass.

## Documentation checks

From a checkout with Python 3 available:

```sh
python3 tools/check_docs.py
git diff --check
```

The checker validates repository Markdown paths, heading anchors, reference links and fenced-code
balance without network access. It does not validate external links, strict YAML conformance,
provider behavior, runtime recovery or semantic correctness.

The dated migration has a separate historical audit:

```sh
python3 tools/check_doc_migration.py
```

It verifies preservation against the recorded baseline and mechanical-move commit, including explicitly
listed editorial changes. By default it checks the commit that introduced the migration record, so
future edits need not remain frozen. Full Git history containing those commits is needed. See the
[migration record](docs/archive/migrations/2026-11-protocol-framing.md) for working-tree use and limits.

## License and attribution

Original Conduit documentation, contracts and code use the [Apache License, Version 2.0](LICENSE).
The repository includes the unmodified official license text. Preserve applicable copyright,
license and attribution notices when adding or adapting material.

This adoption does not relicense reproduced third-party material. In particular, the historical
collaborator submissions in `docs/archive/collaborator-proposals/PR-001-roadmap.md` and
`docs/archive/collaborator-proposals/PR-002-protocol.md` retain their authorship and source attribution;
this change does not assert an additional license grant from their authors. Quoted source material
in research likewise retains its applicable terms. The archive index records the original sources.

No separate contributor agreement or DCO/sign-off requirement is introduced by this ruling.
Any future contribution procedure remains an explicit project decision.

## Remaining publication decisions

One main repository, a common package envelope and Apache-2.0 are accepted. Organization identity,
change/release authority and any additional contribution procedures remain in
[review](docs/REVIEW.md#owner-decisions-before-public-distribution). This guide introduces no review
quorum or package-release promise.
