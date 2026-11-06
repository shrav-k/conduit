# Authoring and syntax proposals

**Direction note, 2026-11-05.** This document predates the owner's rulings recorded in the [plan
audit](../../AUDIT.md) and is not an input for engine builders. Its examples predate the
[placement](../placement/README.md) and [instantiation](../instantiation/README.md) rulings, and the
naming review it puts first now comes last.

Status: **behavior/structure review complete; concrete representations and supporting vocabulary
remain provisional**, 2026-11-04. [PR #3](https://github.com/shrav-k/conduit/pull/3) merged the accepted
rulings and migrated paper fixtures. Nothing in this packet is an installable package, runnable
runtime or claim of conformance.

Start with [Author with the protocol](../../authoring/README.md). Use the
[current review](../../REVIEW.md) for remaining work and the [vocabulary map](../../VOCABULARY.md)
for naming references. This folder illustrates the accepted behavior; it is not another specification
or decision queue. All Conduit-owned spellings except the five family names remain working vocabulary.

## Reading map

| Document | Purpose |
| --- | --- |
| [AUTHORING](AUTHORING.md) | Worked code, MCP, graph, requirement and template cases, including failure and recovery boundaries. |
| [CONTRACTS](CONTRACTS.md) | Detailed proposed schema/manifest/wire organization, with accepted semantics explicitly marked. |
| [PACKAGE_WALKTHROUGH](PACKAGE_WALKTHROUGH.md) | A history package from published contracts through dependencies, setup, runtime and controls. |
| [AUTHORIZATION](AUTHORIZATION.md) | The accepted requirement/policy/grant model illustrated through proposed records and enforcement interfaces. |

## Accepted rules to apply

Read the canonical topic rather than treating a proposed field layout as authoritative:

| Area | Canonical home |
| --- | --- |
| Five families, versioned types, plugins/packages, scoped dependencies and re-exports | [Model](../../protocol/MODEL.md#authoring-concepts-beneath-the-object-families). |
| YAML/JSON value model, schema resolution, formats, defaults and numeric limits | [Configuration language](../../protocol/CONFIGURATION.md#configuration-language). |
| Implementation retention, development/publication and artifact identity | [Implementation selection](../../protocol/CONFIGURATION.md#implementation-selection). |
| Object identity, checked writes, export and graph-structured batches | [Identity and updates](../../protocol/CONFIGURATION.md#object-identity), [batches](../../protocol/CONFIGURATION.md#batch-configuration-submissions). |
| Typed graph inputs/outputs, source scope, dependencies, startup, recovery and cancellation | [Lifecycle protocol](../../protocol/LIFECYCLE.md#lifecycle-protocol). |
| Supported changes, confirmation, concurrency and invocation context | [Controls](../../protocol/LIFECYCLE.md#template-defined-configuration-changes). |
| Requirements, authority, participation, filters and routing | [Conduits](../../protocol/CONDUITS.md). |
| Source-scoped observations, retained state and frontend continuity | [Interface observability](../../protocol/MODEL.md#interface-observability). |

The [completed review](../../archive/reviews/AUTHORING-2026-11-04.md) preserves the original ruling
record and this index's former cumulative summary. No accepted decision was removed by shortening
this navigation page. The [research comparison](../../research/language-protocol-foundations/README.md)
retains sources and alternatives; its recommendations do not silently amend accepted contracts.

## Later PR scope

Review vocabulary through stable topic IDs, then turn a complete contract/package/template/instance
journey into concrete schemas and wire examples. Keep definition identity, template-local aliases,
public exports, provider MCP method names and authorized live connections distinct. No mandatory
primary implementation or additional object family is introduced.

Remaining representation work includes exact identifiers, manifests, declaration and profile fields,
archive layout, diagnostics and executable conformance fixtures. Already accepted binding forms,
source categories, reference scope, schema rules and lifecycle guarantees are not pending choices.
Return a new behavioral conflict to the current review with a concrete example rather than resolving
it through an incidental spelling or implementation shortcut.

Update [Experiment 001](../../../experiments/001-shared-access/README.md) and the
[Experiment 002 plan](../../../experiments/002-runtime-plugins/README.md) with explicit changes and
accurately scoped evidence. [Library probes](../../../experiments/003-language-probes/README.md)
establish only their recorded properties. Runtime mechanisms remain in
[implementation planning](../../implementation/README.md), with no competing active ruling queue.
