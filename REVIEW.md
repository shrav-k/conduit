# Protocol and instance contract review

Updated: 2026-10-31. The current [specification](SPEC.md) and [design contracts](DESIGN.md) define
**Interface, Agent, Resource, Pool and Conduit**, each with template and instance forms. The
pre-inquiry design remains on `main` at `c013f0a`; research is supporting evidence and examples.

## Review method

Resolve connected contracts through complete examples:
**discover → choose intent → validate/authorize → act → observe → recover**.
Separate a protocol guarantee from a type's supported operation and its provider implementation.
Avoid making one provider's queue, settings or process model universal. Group rulings instead of
reviewing every option independently; require each stored relationship to justify its bookkeeping.

The working foundations are reusable typed YAML templates with inline/referenced setup graphs;
instance configuration independent of later template edits; runtime-owned behavior; dynamic Conduit
attachments; Pool-only Resource creation/ownership; current authority; and safe outcome/retry
handling. Plugins declare inputs/outputs, graph bindings imply dependencies, and instances manage
processes even when started during setup. **Templates specify complete establishment and which
instance configuration may change within what parameters.** This is settled protocol direction;
its declaration/implementation details below remain open.

## Next discussion

**Start with the management Interface and the common interaction contract.** The basic Interface
has the broadest local management scope. It is the primary user path for authoring Resource/Agent
templates, adding Resource templates to Pool configurations, instantiating Pools and Agents with
Conduits to those Pools, and observing all instances. CLI, terminal UI and tests use the same capabilities.
Observing/instantiating is distinct from modifying a running target: the latter requires a Conduit.
Agents can invoke target operations through Conduits as well. Resource creation still belongs
to a Pool, and broad management authority cannot manufacture an unsupported provider operation.

| Contract bundle | Questions to resolve together |
| --- | --- |
| Declaration and exposure | How do templates select named controls and constraints from type/plugin contracts? Are all controls explicitly listed, or can versioned defaults be expanded for inspection? Distinguish configuration settings from data operations, lifecycle and connection membership. |
| Constraints and Conduit filtering | How are value types/ranges, related-field rules, dynamic eligible choices and old-to-new transition conditions represented? Validate the proposed intersection of type support, template limits, current authority and connection restrictions; avoid merge precedence or silent value rewriting. |
| Invocation and application | What common request/result/observation contract distinguishes received, pending, applied, failed and unknown outcomes? Do settings use typed field updates, named operations or both? How are application behavior and restart/replacement requirements made inspectable without a universal scheduler? |
| Handlers and recovery | Does a control reference a runtime operation, a plugin operation, or optionally a composed graph? Which setup primitives can be reused? Settle concurrency/version checks, safe retries, cancellation, persistence across starts and invalidation of setup progress as one contract. |
| Evolution and management | Can a running instance change its control envelope, setup graph or implementation, and by which separate management contract? Source-template edits must not silently alter it. Settle bootstrap/identity and management of Conduits without a recursive connection-per-request requirement. |

**Current walkthrough:** [Linear MCP from establishment to shutdown](research/CONDUITS.md#5-access-linear-through-mcp).
An Interface instantiates a Pool-owned service Resource; two Agents attach with separately permitted
service identities. Follow authentication, native tool invocation, a declared client-timeout change
through a management Conduit, and one consumer detaching. Distinguish shared Resource configuration,
per-attachment state, provider content and template edits. The example's timeout/control semantics
are proposals, not new protocol defaults or claims about Linear's settings.

**Next structural ruling:** should templates bind lifecycle, invocation and configuration-change
handlers to either one typed plugin operation or a graph, with instances owning ongoing activity?
Internal handlers need not be externally callable. This is a shared implementation boundary, not
an extra object family or an obligation to wrap each request in a graph. Follow with database startup,
migrations and recovery to test its limits before selecting exact fields.

Retain the [bounded search-control fragment](research/CONFIGURATION.md#a-bounded-control-example)
as a later constraint probe: template range 1–100, narrower Conduit cap 20, requests for 15, 80 and
an unrelated field. Compare Interface, Agent and headless clients; test invalid transitions and
lost responses as well as accepted values. These probes do not settle every control's policy.

No final command names, transport, invocation envelope or universal live-config patch is selected.
Template construction is still comprehensive: invoking start does not require assembling another
setup recipe. Model switching remains an integration example in the construction research; pending
selection supersession is no longer a core-review question.

The sections below retain relevant unresolved work for later bundles. Resources precede detailed
Pool grouping; the Interface walkthrough uses the established Pool-ownership boundary without
assuming a persistent provisioning-plan ledger.

## 1. Resource templates and instances

| Area | Remaining decisions and examples |
| --- | --- |
| Types and compatibility | Keep filesystem, storage, search and integration groups; distinguish local-directory and git-worktree. Explore content, identity and agent-communication types. Define type group/name/version identifiers and compatible consumer contracts independently of plugin packages. Shared shapes/names alone do not prove behavioral compatibility. |
| Establishment and runtime | Use an existing directory with Agent-local symlinks, explicit new-directory creation, remote filesystem and Git worktree. Retain create/attach intent in the complete template recipe, explicit path base and absolute instance path. Define type-specific setup outputs, waits, readiness, effects and cleanup. |
| Shared versus per-consumer state | Symlinks are consumer preparation, not additional Resources. MCP service identities may differ per attachment. Define client/auth lifetime and observable support without registering every helper. Shared backing does not imply shared private state or exclusive writing. |
| Storage, search and MCP | Keep separate unitary Resources connected by Conduits. Define storage/search operations, index/input dependencies, freshness, indexing ownership and how MCP exposes existing contracts without silently combining responsibilities. |
| Typed dependencies | Define generated outputs, revisions, expiring handles and prerequisites; diagnose blocking cycles. Preserve one authored connection declaration and derive reverse views rather than maintaining conflicting endpoint copies. |

**Walkthrough:** one Resource template, two instances, two consumers and one removed attachment.
Show what is fixed, shared, prepared per consumer, observed and safely cleaned up. Use an owning
Pool without deciding every grouping feature first.

## 2. Pool grouping and Resource ownership

| Area | Remaining decisions and examples |
| --- | --- |
| Type-aware grouping | Decide which inventory/compatibility views a Pool derives from its owned Resources. Labels/selectors may select its Resources or templates where appropriate; matching foreign Resources does not confer ownership. |
| Establishment declarations | Decide whether retained creation entries are needed at all. Prefer instance identity and ordinary operation outcomes over a second provisioning-plan ledger. Earlier plan-entry/binding proposals are parked; missing-target/replacement rules were not settled. |
| Partial establishment | Templates carry setup choices; graph/lifecycle contracts coordinate required work and partial results. Define Pool readiness and later Resource additions without conflating registry atomicity with rollback of external effects. |
| Backing and retirement | Two Pools may intentionally attach separate Resource instances to the same external backing. Define conflicts, cleanup authority, ownership transfer if supported, and retirement with active consumers or retained data. An unowned registration is not permission to delete backing data. |
| Durable content | Map task/personal records and conversation stores to Pool-owned Resources. Contexts are not alternative Resource owners; each message, cursor or file need not become a top-level Resource. Conduit-supporting durable Resources still need an owner. |

**Walkthrough:** create shared and independent backing through two Pools, fail during setup,
retry, and retire an owner with active users. Show survivors and unknown outcomes explicitly.

## 3. Object configuration and lifecycle

| Area | Remaining decisions and examples |
| --- | --- |
| Form-specific schemas | Define required/optional fields, null/empty/omitted values, defaults and normalized export. Keep materialized instance defaults, strict unknown-field handling, stable IDs and kind/form/name references. Status is system-owned, not imported success. |
| Template and instance lifecycle | Define create/start/stop/retry/retire/delete where applicable, not every verb for every family. Direct creation from complete definition and repeated template use remain supported. Registration alone performs no provisioning; Resource creation goes through its Pool. |
| Configuration changes versus state | Templates declare the allowed surface and parameters; settle its schema, type implementation binding, Conduit filtering and applied-state representation. Control-envelope/graph changes, generic patches and metadata rules are unresolved; do not assume all fields are editable or fixed. |
| Setup continuation | Specify minimal retained progress, typed interaction waits, version-compatible recovery, cancellation and stale-worker handling. Setup retries reuse valid progress; runtime ownership starts immediately when setup starts its service. Unknown native state cannot justify a duplicate execution. |
| Addressing and replacement | Define direct-ID support, rebind operations, instance suffix rules and export diagnostics. New instances get new identities; replacement does not silently retarget bindings or inherit private context. Shared memory access is explicit through Conduits. |
| Flow end and deletion | Last logical consumer/peer ends the flow; sockets alone do not define membership. Define crash/lease detection, one-time completion, explicit reopening, live deletion constraints and cleanup. Backing data and evidence survive unless separately purged. |

**Walkthrough:** instantiate twice, stop/restart one, replace its definition with a new instance,
and reconcile a lost response. Keep identities, state and operation outcomes distinct.

## 4. Connections, authority and communication

| Area | Remaining decisions and examples |
| --- | --- |
| Endpoint contract | Generalize Interface/Agent control, Resource access, Resource-to-Resource dependencies and platform management. Define endpoint roles, filtered capabilities and request/result/observation paths without assuming one network topology. |
| Authority and filtering | Authenticate the actor and connection; filters narrow granted authority. Define complete resource/operation/constraint combinations and current checks. Keep user authority distinct from frontend identity, and broad management scope distinct from every Interface having unrestricted access. |
| Communication Resources | Explore direct exchange, conversations and shared context as agent-oriented capabilities, potentially backed by storage Resources. Separate retained content from routing and runtime bookkeeping. No rooms-only topology or communication schema is selected. |
| Composition and supporting Resources | An access template can compose repository, credentials, AWS and Linear connections with distinct identities and lifetimes. Resolve owners/provisioning rights for durable supporting Resources; avoid duplicated setup or implied external rollback. |
| Delivery and attention | Define ordering, durable acceptance, acknowledgement, replay, offline delivery and backpressure for selected communication types. Distinguish available data, notices, queued input and starting model work; current access is checked at delivery. Wake behavior belongs to an explicit supported contract. |
| Coordination and coverage | Identify shared backing conflicts, quotas and advisory versus enforced exclusion. Native operations outside mediation do not acquire guarantees through a Conduit label. Multi-frontend control should fit the same authority/outcome model. |

Use the [five connection scenarios](research/CONDUITS.md), adding the broad management Interface.
**Walkthrough:** control a peer, send retained data to an offline recipient, and let an observer
fall behind. Withdraw access before delivery; show permitted results and unresolved work.

## 5. Resolution, activation and integrations

| Area | Remaining decisions and examples |
| --- | --- |
| YAML surface and references | Define string keys, duplicate keys, scalars/units, aliases, path bases and explicit environment inputs. Preserve separate type versions and checked template writes. Secrets/handles are protected references, not ordinary reusable values. |
| Graph/runtime representation | Settle inline/source graph resolution, input/output bindings, managed start/readiness operations and runtime defaults. Instance export retains the resolved graph without copying code/data. Multi-object composition and caches remain later conveniences. |
| Activation | Define type-compatible instructions/skills, native config projections, duplicates and cleanup. Preparation defaults to required connections; optional failure continues and partial success is retained. Dynamic attachment uses supported operations, not authored-spec edits. |
| Adapter contracts | Distinguish Rust-owned execution/provider connectors from native-agent adapters. Report supported auth, control, context conversion, hooks, cancel/reconnect, subagents and telemetry. Reuse native skill/plugin tooling and provider-native MCP contracts. |
| Isolation and access | Verify separate subscriptions and private Agent state, explicit config sources, native default and optional required isolation. Define the first isolated implementation/network matching later. Broad Interface control does not change selected execution mode or provider credentials. |
| Memory and workspaces | Select backend/search limits and content contexts; retain bounded excerpts/references, explicit reads, lifecycle/revision checks, direct read-only sharing, usage coverage and explicit safe workspace cleanup. |

## 6. Validation, evolution and experiment

| Area | Remaining decisions and evidence |
| --- | --- |
| Validation versus readiness | Distinguish schema/reference checks from effectful probes and actual readiness. Missing schemas block writes; unavailable runtime/services can be reported separately. Preview does not reserve access or prove login. |
| Versions and diagnostics | Choose API/type/plugin versions, source-location errors, outcome categories and observed-state fields. Unknown semantics must not silently degrade. Report support, authorization, setup completion and availability separately. |
| Recovery and evidence | Specify request/outcome identity, safe retries, retained progress, protected credential state, import/export baselines and batch behavior. Retain evidence until explicit purge; observed zero, estimates and unavailable telemetry differ. |
| Interface conformance | Exercise discovery, construction and control through CLI, terminal and test clients against the same contracts. Compare typed inputs/results, not rendered text. Separate simulated coverage from live-provider evidence. |

## Initial experiment

Implementation has not started. Settle the common interaction contract and a bounded schema
before coding or live account testing.

1. **Management entry point:** one broad Interface discovers all five families, authors/resolves
   templates, composes Pools, validates drafts, creates instances and observes outcomes. Use a headless client and terminal
   against the same management path; define bootstrap explicitly.
2. **Construction and ownership:** filesystem, authentication and database/indexing examples,
   including waiting and partial failure; repeated instantiation through two Pools; Resource-only
   Pool ownership; dynamic Conduit open/join/detach.
3. **Actors and controls:** resolve a template's configurable surface and constraints; an Interface
   and an Agent invoke the same target contract through Conduits. Exercise
   immediate, pending and unknown results, permissions changing before application, and unsupported
   operations without requiring universal queueing or provider-specific settings in the core.
   Include template limits, narrower parameter filters, related-field/transition checks, unsupported
   handlers and unchanged existing instances after source-template edits.
4. **Real integration:** later verify two Claude and two Codex/OpenAI subscriptions with independent
   credentials/private state; a minimal Rust-owned runtime/terminal, native adapters, one memory
   backend, workspace and MCP/skill integration. Record actual observability/control limits.
5. **Communication:** after its contract is selected, demonstrate multiple participants, retained
   data in Pool-owned Resources, current-access delivery, recovery and independent frontend views.

Failure fixtures must cover stale template edits, attempts to bypass Conduit-mediated changes, wrong-account
login, missing references, foreign-Pool ownership, ambiguous creation/write results, partial setup,
expired interaction, stale completion, cancellation after effects, auth failure, last consumer
loss and owner retirement. Neither native enforcement nor live provider compatibility is established
by a synthetic test. Supporting evidence remains in [configuration](research/CONFIGURATION.md),
[Pi connectors](research/PI_CONNECTORS.md) and [Pi runtime](research/PI_RUNTIME.md).
