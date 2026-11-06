# V0 reference implementation proposal

**Not binding.** The [specification](../../spec/README.md) alone binds. This document uses the
words from before the vocabulary ruling; the terms in force are in
[spec/definitions.md](../../spec/definitions.md). The tracker is the [plan audit](../AUDIT.md),
not REVIEW. Where the text below says that the review queue is the only active decision queue,
that the protocol topics supply accepted semantics, or that there is no backward-compatibility
promise, it no longer holds: those topics are superseded, and what the language promises about
versions is in [spec/promises/naming.yaml](../../spec/promises/naming.yaml).

Two documents in this directory are current: [independent builds](INDEPENDENT_BUILDS.md), the
process and readiness gate for the two builds, and the [engine build brief](ENGINE_BUILD_BRIEF.md),
a draft that is not issued. The rest of this page, [ARCHITECTURE](ARCHITECTURE.md) and
[ORGANIZATION](ORGANIZATION.md) are earlier proposals for a reference implementation.

**Direction note, 2026-11-05.** This document predates the owner's rulings recorded in the [plan
audit](../AUDIT.md) and is not an input for engine builders. The proof order, the role of Experiment
002 and the deferral of a second runtime described below are superseded; see [independent
builds](INDEPENDENT_BUILDS.md).

This is the reference-implementation design, subordinate to the public contracts. Moving it out
of the proposals directory does not approve candidate mechanisms or claim they are implemented.
The [current review queue](../REVIEW.md) is the only active decision queue; the I1–I7 summary below is historical.

Navigation audited: 2026-11-04. This packet preserves implementation planning; **no runtime implementation or
provider experiment has run**. The [design](../protocol/README.md) supplies accepted semantics. The
[conceptual review](../archive/reviews/CONCEPTUAL.md) is complete; the
[authoring/syntax packet](../proposals/authoring-syntax/README.md) now reflects completed behavior/structure
review, with concrete representations still proposed. Use the [vocabulary map](../VOCABULARY.md) when
reviewing names; physical process/crate terms must remain distinct from the five object families.

The public protocol, plugin authoring and repeatable templates take priority over early efficiency.
Use existing infrastructure and lightweight support to prove the contracts. A Rust process layout,
package example or database choice cannot silently define the public model.

## Reading map

| Document | Purpose / authority |
| --- | --- |
| [Closed implementation review](../archive/reviews/IMPLEMENTATION.md) | Compact I1–I7 status with links to accepted contracts; no separate active decision queue. |
| [ARCHITECTURE](ARCHITECTURE.md) | Detailed runtime mechanisms and examples; accepted boundaries are marked, other mechanisms remain proposals. |
| [AUTHORIZATION](../proposals/authoring-syntax/AUTHORIZATION.md) | Accepted policy/grant model illustrated with proposed representation and enforcement details. |
| [ORGANIZATION](ORGANIZATION.md) | Proposed crate/dependency organization, milestones and evidence requirements. Its milestone table is superseded by the plan audit's experiment program. |
| [INDEPENDENT_BUILDS](INDEPENDENT_BUILDS.md) | The process, isolation rules and readiness gate for the two independent engine builds. |
| [ENGINE_BUILD_BRIEF](ENGINE_BUILD_BRIEF.md) | The exact brief each builder receives. Draft; not issued. |
| [Deferred authoring](../proposals/authoring-syntax/README.md) | Contract, plugin/package and template examples for a later PR. |
| [Collaborator comparison](../archive/collaborator-proposals/REVIEW.md) | Completed assessment of merged PR #1/#2, with adopted refinements and attributed dissent preserved. |

## Accepted implementation boundaries

Management owns registration/templates and shared access policy. One designated service owns each
instance's resolved definition, confirmed settings and outcomes. Executors supervise managed activity;
Resource backends own domain data; Interfaces combine evidence with freshness/coverage. No offline
grants are available. Shared logical ownership does not require shared processes or host placement.

The first proof runs management and two independent executors on one machine. Authored code normally
uses supervised instance-associated helpers; existing MCP services keep their supervision. Graphs and
code-optional packages do not require a worker per contribution. Endpoint instances retain selected
implementations; Conduits coordinate relationship requirements and participation.

A minimal published persistence contract is loaded as a service dependency with bundled SQLite.
Conduit-retained secrets use replaceable OS credential custody, initially Keychain. The root Interface
directs foundational setup/recovery. Observation is extensible and inspectable through the Interface,
with local authoritative evidence independent of optional exports. These choices are accepted initial
implementations, not mandatory storage/OS dependencies for every future conforming implementation.

## Delivery and evidence

The [release compatibility rule](../protocol/CONFIGURATION.md#release-compatibility) makes no
backward-compatibility promise. Users can pin a working release/package set; a newer runtime need
not support its older configuration, plugins or stores. Migration support, where supplied, must be
explicit. Strict release coordination is not required for early delivery.

**Early delivery priority:** get the protocol and authoring path working. Exact packaging, release
tooling and coordination should remain lightweight and adaptable; a formal distribution pipeline
or mixed-version support matrix is not a prerequisite for the first proof. Test and describe the
working configuration actually used, preserving public boundaries and evidence. No legacy code
is required solely to support old releases.

**Initial delivery scope:** language/contracts, a minimal reference implementation and small
authoring examples with conformance checks, anchored in Experiment 002 below. A reproducible
working source checkout with a documented build/start path can satisfy early delivery; separate
installers and elaborate release machinery are not prerequisites. Refine individual package
contents through the authoring slice rather than declaring a broad standard library now.
Bundled example/integration packages remain replaceable capabilities; bundling does not make
them mandatory for every conforming implementation. Declared-capability conformance still governs
what a client, plugin or runtime claims to implement. This preserves the independent client/plugin
proof and the later native-account validation sequence.

Keep the [Experiment 002](../../experiments/002-runtime-plugins/README.md) scope: author a plugin,
compose exported operations with derived/explicit results and recovery, add composed/custom requirements,
exercise independent service failures and inspect effects through the Interface. It must use public
extension contracts rather than special-case core dispatch. Implementation has not started.

Next validate real Agents through native Claude/Codex adapters on macOS, then build the owned Rust
model/tool loop. The two-Claude/two-Codex subscription goal needs actual account/private-state evidence.
Two-machine validation is required before claiming remote support. Isolation support requires its own
tests; native mode remains the accepted default. Pi is a research/frontend reference, Xerj an optional
Resource backend. Neither is an adopted runtime dependency.

The finer milestones and library selections remain proposals. Conceptual closure is complete;
review authoring/MCP profiles and conformance fixtures in a later PR before implementation. No root
model, VM default, Sealmesh dependency, second full runtime or new protocol wire is added by this packet.

## Research and maintenance

[Research](../research/README.md) preserves primary-source comparisons, source pins and limits.
Earlier static YAML/link checks establish only their stated properties, not runtime recovery,
authorization, provider compatibility or strict-parser conformance. Keep actual results separate from
expected outcomes. Record a semantic change once in the design and link to it here; avoid restoring
long duplicated ruling histories. Git retains earlier proposals and discussion examples.
