# V0 implementation proposal

Status: **proposed for review**, 2026-11-01. No runtime code has been implemented by this packet.
The [specification](../../SPEC.md) and [design contracts](../../DESIGN.md) remain authoritative.
This packet explains how to implement them, identifies consequences that need a ruling, and
proposes a development sequence. It does not silently promote research or fixture syntax into
accepted protocol.

**Review priority:** plugin and template authoring come first. The user clarified that open
choices must be ruled on together, not converted into implementation defaults autonomously.
The initial authoring direction is accepted: keep Conduit support lightweight and prove the
protocol/structure by authoring plugins; grow helpers from demonstrated needs. Plugins publish
inspectable contracts with their implementations; MCP is the public plugin protocol base, with
a small Conduit profile. Code-optional bindings to existing MCP services and independent component
exposure are accepted. The [design-fit audit](../../REVIEW.md#design-fit-audit-code-optional-plugins-and-independent-mcp-access)
preserves the existing families, ownership and access rules. I1 accepts management/execution
separation, shared policy with instance-owned state, no offline grants, and a one-machine
first proof with management and two independent executor processes. I2 accepts native adapters
before the owned Rust model/tool loop and macOS-first native integration/account validation;
I3 accepts endpoint-selected/managed plugins and Conduit contracts composed from reusable typed
requirements, with a small base vocabulary and author extensions. Its conceptual review is complete,
including graph-backed operations, requirement composition, supervised instance-associated helpers
and bundled first-party plugins plus explicit local installation. Detailed mechanisms remain open;
persistence, credentials and evidence precede the final contract/template syntax review. Existing
infrastructure is welcome behind enforced contracts;
the first implementation should establish the authoring and execution patterns before optimizing
their efficiency. The protocol is the durable product; a particular host or storage arrangement
is replaceable.

## Proposed architecture

**I1 is accepted for the first implementation.** Separate execution services supervise Conduit-managed Agents and
Resources, management coordinates configuration/control, and the Interface presents them. Each
executor can host multiple instances; external MCP services retain their own supervision. This
supports eventual distributed placement. Management owns templates, registration and shared
access policy; one designated service owns each instance's resolved definition, confirmed
configuration/revision and operation results. The Interface aggregates these sources with
freshness and gaps. The [cloud follow-up](../../research/CLOUD_AUTHORITY.md) supplies the accepted
authority model's rationale and alternative examples. V0 retains supervision and dispatched
outcomes, while holding new managed requests whose current permission cannot be established.
Offline grants are deferred; existing stop/revocation and result-delivery rules remain applicable.

Rust services implement registry authority, instance supervision, authorized capability traffic
and durable evidence through explicit boundaries. The first proof uses separate management and
two executor processes on one machine, faulting their connections. Two-machine validation follows
before remote support is claimed. Persistence and production deployment details remain under review. Replaceable frontends attach to an Interface; authored workers and existing
MCP services fulfill published plugin contracts through retained implementation bindings. Graphs
can also implement exported typed operations, composing selected operations without new glue code.
Authored code normally uses supervised instance-associated helpers; trusted built-in Rust code may
remain in-process. Instance state authority and endpoint ownership are unchanged; exact helper
grouping remains open. Existing MCP services retain supervision. Code-optional contributions use
shared adapters or graph composition without a worker per package.
Local transactional storage remains an I4 candidate, not a requirement that all distributed state live in one file.
Xerj remains an optional Resource backend; Pi supplies evidence, not a runtime dependency.

Templates remain the complete authoring entry point: typed YAML, finite setup graphs, declared
runtime behavior and supported controls. Users do not author separate operation-request manifests.
CLI/API calls produce internal operation records so the system can recover and explain effects.
Conduit templates compose typed relationship requirements; selected endpoint plugins supply support.
The proposed runtime resolves definitions, checks bindings and coordinates relevant runtime
validation/enforcement with attributable Interface evidence. The model is accepted; specific
extension contracts, algorithms, libraries and the minimal requirement catalog remain under review.

## Reading order

| Document | Purpose |
| --- | --- |
| [AUTHORING](AUTHORING.md) | Concrete author journeys, the accepted lightweight support direction and illustrative APIs. |
| [ARCHITECTURE](ARCHITECTURE.md) | Processes, ownership, persistence, scheduling, access, recovery, observability and concrete execution paths. |
| [CONTRACTS](CONTRACTS.md) | Proposed schemas, plugin/worker contracts, graph interpretation, controls and MCP mappings. |
| [ORGANIZATION](ORGANIZATION.md) | Repository/crate boundaries, dependencies, implementation milestones, validation and maintenance. |
| [REVIEW](REVIEW.md) | Seven grouped decisions with recommendations, alternatives and specific consequences. |

Start with the review's authoring walkthrough, then consult the contracts and architecture. The
contract document is deliberately more concrete than the accepted design, but every spelling
introduced there is a proposal. The existing [YAML experiment](../../../experiments/001-shared-access/README.md)
remains unchanged so reviewers can compare it with the proposed implementation language.

## Authority and change rules

- **Accepted** means behavior already in DESIGN; this packet links to it rather than creating
  a second decision ledger.
- **Proposed** means an implementation choice recommended here, including crate names, storage
  layouts, wire profiles and numeric limits.
- **Review I1–I7** marks choices with product, protocol or compatibility consequences. They are
  pending until ruled on; I1–I3 are conceptually settled, while I4–I7 retain the choices in REVIEW.
  Routine engineering choices can be adopted with the implementation
  and evidence, without asking a separate question for every field.
- A new counterexample can revise this proposal. It cannot silently override the current design.
  After approval, fold enduring contract changes into DESIGN and keep implementation details here;
  replace pending review text with the ruling rather than maintaining parallel histories.

## Scope

Experiment 002's [combined composition-authoring proof](../../../experiments/002-runtime-plugins/README.md#combined-composition-authoring-proof)
is accepted within its existing synthetic fixture: graph-backed operations, derived/explicit results
and recovery, composed/custom requirements and Interface evidence. It does not approve implementation
start, final syntax or permanent core vocabulary. The helper execution default is accepted separately;
exact process grouping remains open.

The proposed sequence begins with synthetic, fault-injectable execution and an Interface, then real
Resources and Agent adapters, then the required two-Claude/two-Codex account exercise. Native
agent supervision and a Conduit-owned model/tool loop have distinct implementation paths. The
accepted I2 order tests real Agents through native Claude/Codex adapters after the synthetic proof,
then builds Conduit's Rust model/tool loop. Both remain required goals, alongside the replaceable
Pi-inspired terminal. Subscription compatibility still requires actual native-adapter evidence;
native integration/account validation starts on macOS. Keep the Rust core portable and OS
behavior behind adapters; other platforms need their own integration evidence before support claims.

No distributed scheduler, general workflow language, public plugin marketplace, new skill
framework, generic tracker subsystem, label catalog or mandatory Wasm engine is proposed.
Independent placement does not require automatic scheduling or federation; their absence must
not force a single execution lifetime or a central hop for every capability interaction.
OS isolation remains an explicit execution mode whose support must be demonstrated, not inferred
from using Rust, workers or MCP.

## Research performed for this proposal

The [Rust runtime report](../../research/RUST_RUNTIME.md) pins Xerj and Pi source and compares
executors, actors, workflow systems and plugin mechanisms. This follow-up also inspected primary
documentation for subprocess cancellation/reaping, SQLite durability and backup, JSON Schema
defaults, YAML parser events, MCP transports, credential-store selection and Cargo workspaces.
The relevant sources are cited beside the resulting recommendations in this packet.

The [distributed-runtime follow-up](../../research/DISTRIBUTED_RUNTIME.md) adds Tilt,
Docker/containerd, Kubernetes, Rust control/execution systems and primary distributed-systems
papers. Together with the cloud follow-up, it informed the accepted execution and authority
splits and the accepted no-offline-grants v0 rule. Proposed tests and failure guarantees remain
unvalidated until the implementation demonstrates them.

Documentation and source inspection are not executable validation. The
[runtime experiment](../../../experiments/002-runtime-plugins/README.md) defines the first
failure tests. This proposal adds detailed implementation obligations but claims no benchmark,
provider compatibility, credential isolation or recovery test has already passed.

Its success must also include an independently authored plugin and a reusable template, loaded
without changing core dispatch code. A demonstration consisting only of hard-coded built-in
integrations would not establish Conduit's central extension model.

Authoring checks completed for this packet: repository Markdown links/anchors/fences, referenced
local pinned source paths, fenced YAML syntax and whitespace checks. The YAML check is a syntax
check using the existing documentation tooling, not proof of a Conduit schema or strict YAML 1.2
implementation. All proposed runtime and provider acceptance tests remain unexecuted.
