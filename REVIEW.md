# Review agenda

Updated: 2026-10-30. The working model is **Profiles, Sessions, Resources, Pools,
and Conduits**. [SPEC.md](SPEC.md) defines their relationships; [DESIGN.md](DESIGN.md)
contains accepted contracts. This document contains only unresolved choices, research
and experiment planning within that model.

## Review method

Work through concrete configurations and interactions before implementation. Define the
minimum Resource/Pool authoring contract, complete Profile/Session configuration, then
research Conduit access and communication together. Activation details can remain open
until those interactions are concrete, but their owner is settled: **Profile instructions
are applied by Sessions to the Resources their Conduits permit**.

Use one YAML object/version/reference convention across the five concepts, informed by
Kubernetes and networking APIs. Settle identity/name resolution, schema versions, file
layout, validation and authored-versus-observed state together. Keep schema versions,
record revisions, plugin releases and protocol versions distinct. Exact fields below are
illustrative; no `apiVersion` value has been chosen.

Resolve consequential choices in groups and incorporate them into DESIGN.md. Remove
answered questions here. Git history preserves superseded material; no separate decision
ledger or alternate conceptual model is needed.

## Next discussion

The research is ready for review: [Pi findings](research/PI_CONNECTORS.md),
[configuration languages and contracts](research/CONFIGURATION.md), and a
[connected YAML proposal](research/five-concepts.proposal.yaml). The Pi re-review inspected
all 42 built-in provider factories and ran 17 offline history/authentication checks. The
configuration follow-up adds 13 local Compose/Kustomize checks and seven annotated papers.
No live subscription, cluster test, Conduit implementation or new accepted contract is
implied by those results.

Use the same four-Session example to settle these five groups in order. Each group
should produce one coherent ruling; the sections below retain the necessary details.

| Group | Recommended direction for discussion | What still needs a ruling |
| --- | --- | --- |
| 1. Object identity and configuration | YAML/JSON over one typed object graph; versioned envelope; strict core and explicit plugin schemas; inspectable desired/resolved state. | API group/version, reference/rename behavior, file roots, Session ownership of bindings, offline validation versus live probes, and semantic migration rules. |
| 2. Profile preparation and composition | Profile-owned steps plus guidance over permitted Resources; replacement for explicit overrides, set membership for Pools, narrowing for filters, constrained capability unions for access. | Missing/null/empty semantics, default-Conduit precedence, required/optional failures, conflict handling, refresh boundaries and the effects of apply/prepare. |
| 3. Provider execution and continuity | Rust protocol adapters beneath provider definitions/catalogs; explicit account bindings; retained history plus target-specific context projections. | Initial provider/library scope, context evidence/compaction behavior and subscription feasibility criteria. Native-runtime adapters remain separate. |
| 4. Conduit interaction contracts | Compare capability requests, retained exchanges and observations; preserve native MCP operations with version-specific compatibility. | Initial families, addressing/fan-out, durable acceptance, outcomes/replay, attention and cancellation. No room or wake policy is reinstated. |
| 5. First experiment and release boundary | Resolve the YAML graph offline, validate four accounts, exercise the minimal runtime, then shared memory/MCP and the chosen communication flow. | Memory backend/budgets, first native packages/services, isolation backend, success criteria and which capabilities can wait. |

The configuration recommendation is to define semantics before adding a template or
expression language. Review [composition operators](research/CONFIGURATION.md#composition-needs-several-operators)
and [validation/effect boundaries](research/CONFIGURATION.md#validation-and-effects-are-separate-contracts)
with the same YAML example. The tested Compose include behavior differed from its current
documentation; this supports keeping semantic fixtures alongside schema versions.

The Pi findings still matter: its inspected MCP lifecycle predates the chosen July 2026
standard, and its history conversion does not fence opaque state by account. These affect
adapter contracts and acceptance tests, without changing the five concepts.

## Profiles

**Configuration and reuse.** Specify the runtime/model/account references, default Conduit
references, activation instructions and limits a Profile can author. How are reusable
loading conventions supplied without Profile inheritance? Define Session overrides,
Profile deletion and explicit reconfiguration without freezing live permission limits.

**Activation instructions.** Use the [loader findings and preparation proposal](research/PI_CONNECTORS.md#profile-activation-and-pool-resources)
to determine what Profile/model/runtime defaults handle, how an
author overrides them in the template, and how a Session applies them to permitted
Resources. Separate discovery, compatible import/registration and context loading. Resolve
conflicts, duplicates, required versus optional preparation, budgets, errors and update
behavior. A Conduit filters the available set; it does not carry activation instructions.

**Account and execution policy.** Validate supported subscription enrollment, verifiable
identity, refresh ownership, credential reuse and private-state separation for two Claude
and two Codex/OpenAI accounts. Distinguish own-runtime provider support from external-agent
adapters. Select an isolation backend and define capability/network matching; native
execution remains the default, with optional Profile-required isolation. Decide how
configuration authors see resolved policy and whether generated files always state the mode.

The [connected example](research/five-concepts.proposal.yaml) now supplies Profile defaults
and activation alongside their referenced objects. Its fields remain proposals. Credentials
stay outside the files; actual skill/document content comes through Pools.

## Sessions

**Configuration record and execution.** Define the persisted Session shape: Profile origin,
resolved settings, explicit overrides, private history and Conduit binding references.
Choose one authoritative representation of bindings rather than two independently editable
lists. Specify when reconfiguration takes effect during an active execution, how actual
settings are attributed, and which changes require a new execution.

**Own runtime and frontends.** Select the smallest Rust runtime that supports the first
workflow. The Pi research proposes persisted input/tool intent/outcome, child ownership,
bounded state updates and context-source records. Decide v0 recovery granularity,
context construction/compaction and the control/state API. Frontend commands, observations
and controller transfer must preserve the accepted Session lifecycle. Component reuse
from `pi-tui` is optional; stock UI parity and Pi runtime adoption are not open goals.

**Continuity and evidence.** Specify checkpoint capture and native-history conversion
per integration, supported reconnect behavior, subagent attribution and usage coverage.
Determine how the Session records Profile activation results and supplied Resource versions.
Retained behavior, current access and changing Resource content need separate treatment.

The [configuration proposal](research/CONFIGURATION.md#profile-and-session) distinguishes
authored requests from resolution/execution evidence. Conduit assigns stable identity. Creating
a Session through CLI/API should not require manually authoring a file for every conversation;
exact authoring/export semantics remain to be settled.

## Resources

**Definition and discovery.** Specify the common identity/metadata contract and extensible
kind schemas. Decide how local declarations, inline definitions and provider discovery
coexist; how references survive renames; and the useful granularity of packages, directories,
services and individual objects. Preserve provider-native operations without requiring a
handwritten object for every external item. Resource kinds need not share one payload schema.

**Compatibility and versions.** Describe supported operations and runtime/plugin requirements
in a form that Conduits can select and Profile activation can use. Decide live versus pinned
source selection, source updates and unavailable versions. Keep names, labels, compatibility
and authority distinct. Determine which metadata is authored and which adapters discover.

**First Resource contracts.** Pick a memory backend and search-limit values/units; preserve
the accepted lifecycle, revision, provenance, sharing and coverage behavior. Specify workspace
base/branch/directory settings and adapter cleanup evidence. Define Linear integration and
work references while retaining the provider's workflow ownership. Choose the first native
skills/MCP sources needed to exercise Profile activation, using their existing tooling.

Advanced sharing/publication, background classification/extraction, automatic expiry and
exact cost accounting remain later capabilities. Their relevant constraints are in DESIGN.md;
they should not dominate the first experiment.

## Pools

**Membership and lifecycle.** Define explicit reference and label-selector syntax, names,
resolution boundaries and the consequences of membership edits. Decide whether nesting
is needed, how missing members are reported, and how overlap is represented without
copying Resource identity or ownership. Pool lifecycle must distinguish removing a collection
from deleting any referenced content.

**Defaults and composition.** Complete precedence between Profile default Conduits, Pool
access defaults and explicit Session choices. Preserve the accepted rule that explicit
selection replaces Pool defaults unless an add action is requested. Define how a Session
can see the resolved collection and the filtered resources it can actually use.

**Concrete examples.** Author a reusable frontend-engineering Pool and an application Pool
that reference some of the same Resources. Test explicit membership without labels, selector
membership, different backend kinds and unsupported capabilities. These are uses of one
extensible Pool contract, not fixed Pool categories.

## Conduits

**Declarations and bindings.** Define how a reusable declaration becomes an independently
authenticated Session binding. Specify endpoints, grants, filters, opening/closing and
observed state. Resource selections resolve through Pools; communication links Sessions.
Keep declaration lifecycle, active bindings and underlying client/process lifecycles distinct.

**Filtering and authority.** Define resource/operation filtering and composition across
multiple Conduits within live Profile limits and exclusions. Decide whether filter rules
are inline, reusable or both without promoting them to a sixth core concept. Cover selectors,
current-rule checks, missing permissions and revocation. Resource selection feeds Profile
activation; it does not determine context loading by itself.

### Interaction research

Review access and communication as related Conduit contracts. Activation research concerns
the receiving Profile/Session's use of the filtered Resource set; communication research
concerns the exchange between Sessions. Their boundaries must agree without forcing identical
operations, persistence or attention behavior.

| Contract area | Questions to resolve |
| --- | --- |
| Interaction patterns | Which contracts are needed for resource invocation/read/query, requests/results, asynchronous exchange, publication/subscription, state observation and handoff? Derive a small v0 set rather than adopting a broker's entire vocabulary. |
| Addressing and topology | How are Sessions and Pool selections addressed? Compare direct/routed and one-to-one/one-to-many relationships, discovery and independent recipient state. |
| Delivery and outcomes | Define ordering boundaries, correlation, durable acceptance, acknowledgement, replay, duplication and unknown outcomes. Separate receipt, execution and completion. |
| Attention and lifecycle | How do exchange, notification, queued input and starting an agent turn differ? Handle offline/paused/closed Sessions, cancellation, timeouts, backpressure and reconnect. |
| Context and references | When is content retained, referenced, retrieved or delivered? Preserve provenance and private Session context, with current recipient access checks. |
| Protocol and extension boundaries | Map contracts to MCP and other suitable protocols. Preserve native capabilities; compare local-first operation with remote extension and Rust integration. |
| Observation and tracking | Attribute operations and coverage, retain useful diagnostics and associate provider-native tracker items without making a tracker define every interaction. |

Use primary documentation, pinned source and focused tests. These are research references,
not selected dependencies:

| Reference | Relationship it informs |
| --- | --- |
| [Pi re-review](research/PI_CONNECTORS.md) and [earlier investigation](research/PI_RUNTIME.md), including loaders, pi-ai, pi-mcp, pi-telemetry, Durable and Chord | Profile activation, Session execution, Resource compatibility and Conduit contracts. |
| [Kubernetes objects](https://kubernetes.io/docs/concepts/overview/working-with-objects/), [labels](https://kubernetes.io/docs/concepts/overview/working-with-objects/labels/), [API versions](https://kubernetes.io/docs/reference/using-api/), [CRD versioning](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definition-versioning/) | Common authoring conventions, references, extensibility and version evolution across the model. |
| [Gateway API](https://gateway-api.sigs.k8s.io/docs/concepts/api-overview/) and [reference grants](https://gateway-api.sigs.k8s.io/reference/api-types/referencegrant/) | Declared relationships, selection, authority and observed binding status. |
| [MCP architecture](https://modelcontextprotocol.io/docs/2026-07-28/learn/architecture) and [A2A](https://a2a-protocol.org/v1.0.1/specification/) | Resource capability exchange and Session interaction; distinguish protocol guarantees from application contracts. |
| [NATS/JetStream](https://docs.nats.io/concepts/jetstream), [Zenoh](https://spec.zenoh.io/spec/1.0.0/concepts/entities.html), [gRPC](https://grpc.io/docs/what-is-grpc/core-concepts/) and HTTP/JSON-RPC | Routing, streams, queries, requests, persistence, cancellation and transport tradeoffs. |
| Local agentwatch, pstack-pi and jxp-skills; inspected pi-messenger/pi-chat | Practical coordination, runtime integration, replay, attention, work associations and usage evidence. |
| [Xerj](https://github.com/xerj-org/xerj), native skill sources such as [Superpowers](https://github.com/obra/superpowers), [Matt Pocock's skills](https://github.com/mattpocock/skills), and [Ponytail](https://github.com/DietrichGebert/ponytail) | Replaceable Resource backends and native packages; no mandated storage or skill framework. |
| [Spec Kit](https://github.com/github/spec-kit) | Concrete scenarios and reviewable requirements separated from implementation planning. |

The [completed comparison](research/PI_CONNECTORS.md#communication-lessons-for-typed-conduits)
and [YAML failure matrix](research/CONFIGURATION.md#failure-matrix-for-the-example) use these
flows: four independent Sessions accessing shared Pools; a review request and result; a finding sent to multiple
permitted recipients including one reconnecting later; a slow observer; a permission change
during queued work; and continuation across model/account/frontend changes. They include
failure examples, candidate YAML, proposed interaction families and Rust adaptation boundaries.
Use the grouped rulings above to resolve the remaining choices. No communication topology,
transport or automatic wake policy is selected yet.

## Experiment across the five concepts

Design the experiment before starting implementation or enrolling accounts. Keep own-runtime
subscription feasibility early so UI work does not conceal a blocking integration assumption.
The [Pi trial proposal](research/PI_RUNTIME.md#proposed-experiment-sequence) supplies fault cases,
not a fixed implementation order.

| Interaction to exercise | Evidence required |
| --- | --- |
| Profile → Session | Four independently authenticated Sessions from reusable templates, across two Claude and two Codex/OpenAI subscriptions; actual identities and refresh/logout isolation. |
| Session execution and presentation | A minimal Rust-owned model/tool loop with intent/outcome evidence, controller transfer, detachment and reconnect. A fake provider can test mechanics but not subscription eligibility. |
| Resource → Pool | Reuse one Resource across collections; exercise native skills/configuration and one memory backend without copied ownership or a required new package format. |
| Session → Conduit → Pool | Apply current grants/filters, then Profile activation to the permitted set. Show what was prepared, skipped or unavailable and preserve actual runtime compatibility. |
| Session ↔ Conduit ↔ Session | Demonstrate the selected communication contract, including attention, interruption and reconnect behavior once that contract is agreed. |
| Changes and failures | Profile edits, live access removal, stale record writes, uncertain tool effects, source/backend failures, lost frontend updates and non-overlapping usage totals. |

Specify expected observations, failure criteria, restart and cleanup. Evaluate native and
isolated execution explicitly; external Claude Code/Codex adapters require their own checks.
General durable workflows, recoverable scripts, comprehensive MCP interception and broad
plugin compatibility can follow a demonstrated core. ABI, crates, storage engine and host
process layout are implementation choices downstream of these contracts.
