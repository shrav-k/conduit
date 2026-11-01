# V0 implementation proposal

Status: **proposed for review**, 2026-11-01. No runtime code has been implemented by this packet.
The [specification](../../SPEC.md) and [design contracts](../../DESIGN.md) remain authoritative.
This packet explains how to implement them, identifies consequences that need a ruling, and
proposes a development sequence. It does not silently promote research or fixture syntax into
accepted protocol.

## Proposed architecture

One Rust host owns the local registry, instance supervision, authorized dispatch and durable
operation evidence. Replaceable frontends attach to an Interface. Versioned workers implement
provider, filesystem, authentication and other plugin behavior; MCP carries compatible external
contracts. A small local transactional store supplies authoritative state. Xerj remains an
optional Resource backend. Pi supplies design evidence, not a runtime dependency.

Templates remain the complete authoring entry point: typed YAML, finite setup graphs, declared
runtime behavior and supported controls. Users do not author separate operation-request manifests.
CLI/API calls produce internal operation records so the system can recover and explain effects.

## Reading order

| Document | Purpose |
| --- | --- |
| [ARCHITECTURE](ARCHITECTURE.md) | Processes, ownership, persistence, scheduling, access, recovery, observability and concrete execution paths. |
| [CONTRACTS](CONTRACTS.md) | Proposed schemas, plugin/worker contracts, graph interpretation, controls and MCP mappings. |
| [ORGANIZATION](ORGANIZATION.md) | Repository/crate boundaries, dependencies, implementation milestones, validation and maintenance. |
| [REVIEW](REVIEW.md) | Seven grouped decisions with recommendations, alternatives and specific consequences. |

Read the architecture and then the review before examining individual field spellings. The
contract document is deliberately more concrete than the accepted design, but every spelling
introduced there is a proposal. The existing [YAML experiment](../../../experiments/001-shared-access/README.md)
remains unchanged so reviewers can compare it with the proposed implementation language.

## Authority and change rules

- **Accepted** means behavior already in DESIGN; this packet links to it rather than creating
  a second decision ledger.
- **Proposed** means an implementation choice recommended here, including crate names, storage
  layouts, wire profiles and numeric limits.
- **Review I1–I7** marks choices with product, protocol or compatibility consequences. They are
  pending until ruled on. Routine engineering choices can be adopted with the implementation
  and evidence, without asking a separate question for every field.
- A new counterexample can revise this proposal. It cannot silently override the current design.
  After approval, fold enduring contract changes into DESIGN and keep implementation details here;
  replace pending review text with the ruling rather than maintaining parallel histories.

## Scope

The proposed sequence begins with a synthetic, fault-injectable host and Interface, then real
Resources and Agent adapters, then the required two-Claude/two-Codex account exercise. Native
agent supervision and a Conduit-owned model/tool loop have distinct implementation paths. Their
delivery order needs **I2** because the current design calls for both Rust-owned execution and a
Pi-inspired terminal, while subscription compatibility requires native-adapter evidence.

No distributed scheduler, general workflow language, public plugin marketplace, new skill
framework, generic tracker subsystem, label catalog or mandatory Wasm engine is proposed.
OS isolation remains an explicit execution mode whose support must be demonstrated, not inferred
from using Rust, workers or MCP.

## Research performed for this proposal

The [Rust runtime report](../../research/RUST_RUNTIME.md) pins Xerj and Pi source and compares
executors, actors, workflow systems and plugin mechanisms. This follow-up also inspected primary
documentation for subprocess cancellation/reaping, SQLite durability and backup, JSON Schema
defaults, YAML parser events, MCP transports, credential-store selection and Cargo workspaces.
The relevant sources are cited beside the resulting recommendations in this packet.

Documentation and source inspection are not executable validation. The
[runtime experiment](../../../experiments/002-runtime-plugins/README.md) defines the first
failure tests. This proposal adds detailed implementation obligations but claims no benchmark,
provider compatibility, credential isolation or recovery test has already passed.

Authoring checks completed for this packet: repository Markdown links/anchors/fences, referenced
local pinned source paths, fenced YAML syntax and whitespace checks. The YAML check is a syntax
check using the existing documentation tooling, not proof of a Conduit schema or strict YAML 1.2
implementation. All proposed runtime and provider acceptance tests remain unexecuted.
