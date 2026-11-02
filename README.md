# Conduit

Conduit is an extensible protocol and configuration language for defining capabilities,
establishing instances and connecting independent Agents to shared Resources and one another.
Pools own Resources and shared service authentication; Conduits express permitted relationships;
Interfaces provide management and observability.

The language and public contracts are the primary product. Authors can define capabilities and
requirements, implement operations with code or existing MCP services, compose graphs and publish
repeatable templates. A Rust reference implementation, replaceable frontends and a thin set of
plugins/templates will demonstrate those contracts. Richer integrations can live in ancillary packages.

**Status: protocol design and review. There is no runnable Conduit implementation yet.**
Accepted semantics are documented; exact schemas, package syntax and MCP profile mappings remain
under review. Static examples are not evidence of runtime or provider compatibility.

| Start here | Purpose |
| --- | --- |
| [Learn the protocol](docs/OVERVIEW.md) | Five families, extension concepts and one end-to-end journey. |
| [Protocol reference](docs/protocol/README.md) | Accepted behavioral contracts and conformance boundaries. |
| [Author with the protocol](docs/authoring/README.md) | Define an original extension and template; identify which representations are still proposals. |
| [Reference implementation](docs/implementation/README.md) | Accepted implementation choices, candidate mechanisms and delivery evidence. |
| [Current review](docs/REVIEW.md) | Completed migration rulings and the next authoring/conformance review. |
| [Documentation map](docs/README.md) | Research, experiments, archived reviews and attributed dissent. |

The repository now separates the active reference from completed reviews without discarding their
rationale. The [migration record](docs/archive/migrations/2026-11-protocol-framing.md) accounts for the
previous files and preservation checks. [Contributing](CONTRIBUTING.md) explains how to keep accepted
rules, proposals and evidence distinct and run the repository-local checks.

Original Conduit documentation, contracts and code are licensed under [Apache-2.0](LICENSE).
Copyright 2026 Conduit contributors. Reproduced third-party material retains its own terms and
attribution; see the [license scope](CONTRIBUTING.md#license-and-attribution).
