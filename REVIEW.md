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
processes even when started during setup. **Pools normally establish shared service authentication;
Conduits authorize and narrow each caller's use. Templates specify establishment, ongoing behavior
and permitted changes, implemented by typed plugin operations or graphs.** These are working
foundations; exact declaration/implementation details below remain open.

## Next discussion

The [specification](SPEC.md#shared-pool-access-and-authentication) and
[design contracts](DESIGN.md#pools) retain the settled rules. For this walkthrough:

| Settled boundary | Meaning |
| --- | --- |
| Shared authentication | Pool-owned authentication Resources serve other Resources through Conduits, using provider-appropriate bindings. No Resource per token or additional family is implied. |
| Dependency composition | Resource templates declare requirements; Pools supply targets; consumers retain resolved bindings. No automatic account selection or silent rebinding. |
| Capability contracts | Versioned behavior contracts allow compatible Resource types to satisfy a requirement. Compatibility, authority and readiness remain separate checks. |
| Dynamic catalogs | Integrations can discover provider-native operations and schemas without a separate Conduit contract per tool. Discovery grants nothing; fixed catalogs remain valid elsewhere. |
| Delegation | An authorized creator can delegate a subset while establishing another consumer's attachment. Requested YAML grants nothing; filters narrow access; onward delegation is not implied. |
| Local entries | Pool-local component names resolve to that Pool's Resource instances and stable IDs. They are not registry aliases or a second provisioning ledger. |
| Default setup | A reusable graph step consumes the entries and bindings and invokes each Resource's setup contract. Custom graphs can reference the same entries for extra ordering/checks. Setup is explicitly requested. |
| Partial availability | Ready Resources remain usable through authorized Conduits during incomplete Pool setup. Their prerequisites and declared gates still apply; component and Pool status remain distinct. |
| Composite Conduits | Reusable bundles preserve component targets, authority and lifecycle. Each consumer's attachment tracks its own component participation and detach cleanup; underlying flows/clients/authentication may still be shared. Supporting Resources remain Pool-owned. |
| Filters and paths | Typed declarations are implemented by supporting plugins. Every applicable restriction on the chosen path must pass; independent paths preserve complete target/operation/parameter combinations. Custom filter contracts can use scripts/graphs. |
| Configuration controls | Templates normally reuse their type's declared default controls, with optional narrowing or explicit selection of other supported controls. Instances retain that resolved surface. Frontends can still present field editing; plugin updates do not silently add controls. Dynamic provider-native catalogs remain separate. |
| Control exchanges | MCP tools carry typed inputs and extensible plugin results. Reuse supported standard interaction/observation mechanisms where applicable; no exhaustive Conduit domain-state enum. Current authority and evidence still govern each operation. |
| Configuration state | Store confirmed applied settings; keep outstanding requests separately. Plugins establish application evidence. An uncertain outcome is reported alongside the last confirmed value without claiming it is still active. |
| Change coordination | Apply configuration changes one at a time per target instance across its Conduits, unless its contract declares safe parallelism. Ordinary Resource use and queueing/rejection follow their own supported contracts. |

**Current walkthrough:** the [connected YAML draft](research/CONSTRUCTION.md#connected-pool-and-agent-yaml-draft)
shows a reusable service's authentication requirement, a Pool composing authentication/service
Resources, and an Agent attachment requesting read access. It is a review sketch, not executable
configuration or an approved schema. Referenced type/plugin/graph and permission-bundle names
are illustrative; the Interface authorizes actual instantiation and delegation.

The accepted [MCP exchange](research/CONFIGURATION.md#application-requirements-and-outcomes)
distinguishes standard messages from plugin-defined behavior. The accepted
[control selection](research/CONFIGURATION.md#selecting-the-exposed-controls) reuses type defaults
without requiring every template to repeat them. Exact syntax and native-client support remain open.

The [configuration-state rule](research/CONFIGURATION.md#requested-and-applied-configuration)
is also accepted: confirmed settings and outstanding requests remain separate. Exact storage fields
and plugin application evidence remain to be specified.

The [coordination default](research/CONFIGURATION.md#concurrent-configuration-controls) is accepted:
one configuration change applies at a time per target, with declared safe parallelism allowed.

**Next ruling — stale configuration requests:** should configuration controls normally require
the caller's expected configuration revision, checked when the change is applied, with explicit
contract exceptions for safe operations that do not need that precondition? Recommend this so
serial execution cannot silently turn an edit based on old settings into an overwrite of a newer
decision. For example, both Agents inspect revision R; A's change advances it; B's request based
on R reports a conflict. B can reread, reconcile and submit again within its authority.

The alternative makes preconditions optional unless a control explicitly requires them. Revision
values can come from inspection and be carried by clients; no human confirmation or separate
authored request document is required. Exact revision granularity, MCP representation and how
checks compose with declared safe parallelism remain open. See the
[stale-request example](research/CONFIGURATION.md#stale-configuration-requests).

Then review these connected contracts, one bundle at a time:

| Order | Contract and concrete outcome |
| --- | --- |
| 1 | **Resource and authentication composition:** name the owned capabilities, shared bindings, consumers and lifecycle boundaries. Include provider-required per-consumer exceptions and cross-Pool use. |
| 2 | **Access through Conduits:** select capabilities and current caller authority; narrow operations/parameters; expose native tool contracts over a supported mediated path. Separate use of a credential from permission to retrieve it. |
| 3 | **Complete templates and handlers:** establish the capability, maintain runtime state and apply declared changes using plugin operations/graphs. Define readiness, interaction waits, outcomes, safe retry and supported replacement together. |
| 4 | **YAML and inspection:** render those relationships with typed fields, references and defaults; distinguish authored intent, effective controls, pending changes and observed state. No new generic operation manifest is required. |
| 5 | **Interface and experiment:** exercise the same authoring, Pool setup, attachment, observation and management contracts through terminal and headless clients, then verify native integrations. |

The Interface remains the broad local management entry. Pool ownership, shared service
authentication, separate model subscriptions/private Agent state, provider-native tools and
Conduit-mediated changes are not being reopened. Exact handler/control schemas, supported
transitions, state preconditions, contract evolution and bootstrap remain unresolved.

Use [the bounded search control](research/CONFIGURATION.md#a-bounded-control-example) now, followed
by database startup/migrations. They test whether the same contracts work beyond MCP
without promoting one provider's timeout, queue or process behavior into universal rules.

## 1. Resource templates and instances

| Area | Remaining decisions and examples |
| --- | --- |
| Types and compatibility | Keep filesystem, storage, search and integration groups; distinguish local-directory and git-worktree. Explore content, identity and agent-communication types. Define type/capability identifiers, version compatibility and conformance independently of plugin packages. Shared shapes/names alone do not prove behavioral compatibility. |
| Establishment and runtime | Use an existing directory with Agent-local symlinks, explicit new-directory creation, remote filesystem and Git worktree. Retain create/attach intent in the complete template recipe, explicit path base and absolute instance path. Define type-specific setup outputs, waits, readiness, effects and cleanup. |
| Authentication contract | Reusable authentication is a Pool-owned Resource. Define its type/operations and supported bindings while preserving one lifecycle/refresh owner, protected references and separate caller authority. Non-reusable internal state and explicit per-consumer exceptions remain to be specified. |
| Shared versus per-consumer state | Symlinks are preparation state, not additional Resources. Define client reuse, idle cleanup and attachment-local state separately from shared credentials; detaching a consumer must preserve Resource-owned state still needed elsewhere. |
| Storage, search and MCP | Keep separate unitary Resources connected by Conduits. Define storage/search operations, index/input dependencies, freshness, indexing ownership and how MCP exposes existing contracts without silently combining responsibilities. |
| Typed dependencies | Define generated outputs, revisions, expiring handles and prerequisites; diagnose blocking cycles. Preserve one authored connection declaration and derive reverse views rather than maintaining conflicting endpoint copies. |

**Walkthrough:** one Resource template, two instances, two consumers and one removed attachment.
Show what is fixed, shared, prepared per consumer, observed and safely cleaned up. Use an owning
Pool without deciding every grouping feature first.

## 2. Pool grouping and Resource ownership

| Area | Remaining decisions and examples |
| --- | --- |
| Type-aware grouping | Decide which inventory/compatibility views a Pool derives from its owned Resources. Labels/selectors may select its Resources or templates where appropriate; matching foreign Resources does not confer ownership. |
| Establishment declarations | Local Resource entries and a reusable default composition step are accepted. Define typed results and custom graph integration; prefer instance identity and ordinary operation outcomes over a second provisioning-plan ledger. Earlier persistent plan-entry/binding proposals are parked; missing-target/replacement rules were not settled. |
| Partial establishment | Ready Resources remain usable during incomplete Pool setup, subject to their prerequisites and gates. Define aggregate status/completion and later Resource additions without conflating registry atomicity with rollback of external effects. |
| Backing and retirement | Two Pools may intentionally attach separate Resource instances to the same external backing. Define conflicts, cleanup authority, ownership transfer if supported, and retirement with active consumers or retained data. An unowned registration is not permission to delete backing data. |
| Durable content | Map task/personal records and conversation stores to Pool-owned Resources. Contexts are not alternative Resource owners; each message, cursor or file need not become a top-level Resource. Conduit-supporting durable Resources still need an owner. |
| Shared access lifecycle | Define how Pool setup presents login/renewal once for a shared capability, how partial readiness is shown, and how retirement affects its users. Cross-Pool access must preserve the credential owner's authority and provider restrictions. |

**Walkthrough:** create shared and independent backing through two Pools, fail during setup,
retry, and retire an owner with active users. Show survivors and unknown outcomes explicitly.

## 3. Object configuration and lifecycle

| Area | Remaining decisions and examples |
| --- | --- |
| Form-specific schemas | Define required/optional fields, null/empty/omitted values, defaults and normalized export. Keep materialized instance defaults, strict unknown-field handling, stable IDs and kind/form/name references. Status is system-owned, not imported success. |
| Template and instance lifecycle | Define create/start/stop/retry/retire/delete where applicable, not every verb for every family. Direct creation from complete definition and repeated template use remain supported. Registration alone performs no provisioning; Resource creation goes through its Pool. |
| Configuration changes versus state | Named controls, type-default reuse, extensible MCP exchanges, confirmed settings separate from requests and per-target coordination are accepted. Settle storage fields, handler bindings, transition/state preconditions and safe-parallelism details. Control-envelope/graph changes and metadata rules remain unresolved. |
| Setup continuation | Specify minimal retained progress, typed interaction waits, version-compatible recovery, cancellation and stale-worker handling. Setup retries reuse valid progress; runtime ownership starts immediately when setup starts its service. Unknown native state cannot justify a duplicate execution. |
| Addressing and replacement | Define direct-ID support, rebind operations, instance suffix rules and export diagnostics. New instances get new identities; replacement does not silently retarget bindings or inherit private context. Shared memory access is explicit through Conduits. |
| Flow end and deletion | Last logical consumer/peer ends the flow; sockets alone do not define membership. Define crash/lease detection, one-time completion, explicit reopening, live deletion constraints and cleanup. Backing data and evidence survive unless separately purged. |

**Walkthrough:** instantiate twice, stop/restart one, replace its definition with a new instance,
and reconcile a lost response. Keep identities, state and operation outcomes distinct.

## 4. Connections, authority and communication

| Area | Remaining decisions and examples |
| --- | --- |
| Endpoint contract | Generalize Interface/Agent control, Resource access, Resource-to-Resource dependencies and platform management. Define endpoint roles, filtered capabilities and request/result/observation paths without assuming one network topology. |
| Authority and filtering | Typed filters and intersection along each complete authorized path are accepted. Define expression syntax, diagnostics, routing and current checks. Settle issuance/lifetime details for accepted attachment delegation. Keep user authority distinct from frontend identity, and broad management scope distinct from every Interface having unrestricted access. |
| Communication Resources | Explore direct exchange, conversations and shared context as agent-oriented capabilities, potentially backed by storage Resources. Separate retained content from routing and runtime bookkeeping. No rooms-only topology or communication schema is selected. |
| Composition and supporting Resources | Composite Conduits and per-consumer component participation are accepted. Define attachment representation, aggregate readiness and cleanup recovery, plus provisioning rights for Pool-owned supporting Resources. Avoid duplicated setup or implied external rollback. |
| Delivery and attention | Define ordering, durable acceptance, acknowledgement, replay, offline delivery and backpressure for selected communication types. Distinguish available data, notices, queued input and starting model work; current access is checked at delivery. Wake behavior belongs to an explicit supported contract. |
| Coordination and coverage | Identify shared backing conflicts, quotas and advisory versus enforced exclusion. Native operations outside mediation do not acquire guarantees through a Conduit label. Multi-frontend control should fit the same authority/outcome model. |

Use the [five connection scenarios](research/CONDUITS.md), adding the broad management Interface.
**Walkthrough:** control a peer, send retained data to an offline recipient, and let an observer
fall behind. Withdraw access before delivery; show permitted results and unresolved work.

## 5. Resolution, activation and integrations

| Area | Remaining decisions and examples |
| --- | --- |
| YAML surface and references | Define string keys, duplicate keys, scalars/units, aliases, path bases and explicit environment inputs. Preserve separate type versions and checked template writes. Secrets/handles are protected references, not ordinary reusable values. |
| Graph/runtime representation | Settle inline/source graph resolution, input/output bindings, managed start/readiness operations and runtime defaults. Instance export retains the resolved graph without copying code/data. General cross-family composition bundles and caches remain later conveniences; Pool Resource composition is part of the current design. |
| Activation | Define type-compatible instructions/skills, native config projections, duplicates and cleanup. Preparation defaults to required connections; optional failure continues and partial success is retained. Dynamic attachment uses supported operations, not authored-spec edits. |
| Adapter contracts | Distinguish Rust-owned execution/provider connectors from native-agent adapters. Report shared service auth, native model-account support, control, context conversion, hooks, cancel/reconnect, subagents and telemetry. Provider-native MCP tools can be proxied without a generic replacement vocabulary; direct paths need separate coverage evidence. |
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

Implementation has not started. Settle concrete schemas, supported MCP features and the remaining
lifecycle/authority contracts before coding or live account testing.

1. **Management entry point:** one broad Interface discovers all five families, authors/resolves
   templates, composes Pools, validates drafts, creates instances and observes outcomes. Use a headless client and terminal
   against the same management path; define bootstrap explicitly.
2. **Shared service access:** establish a Pool capability and service login once; attach two Agents
   with different restrictions; preserve native tool schemas and record each caller. Detach/revoke
   one caller while the other continues. Separately expire/revoke the shared login and report its
   wider effect. Use synthetic provider behavior before any live sign-in.
3. **Construction and ownership:** filesystem, authentication and database/indexing examples,
   including waiting and partial failure; repeated instantiation through two Pools; Resource-only
   Pool ownership; dynamic Conduit open/join/detach.
4. **Actors and controls:** resolve a template's configurable surface and constraints; an Interface
   and an Agent invoke the same target contract through Conduits. Exercise
   immediate, pending and unknown results, permissions changing before application, and unsupported
   operations without requiring universal queueing or provider-specific settings in the core.
   Include template limits, narrower parameter filters, related-field/transition checks, unsupported
   handlers and unchanged existing instances after source-template edits.
5. **Real integration:** later verify two Claude and two Codex/OpenAI subscriptions with independent
   credentials/private state; a minimal Rust-owned runtime/terminal, native adapters, one memory
   backend, workspace and MCP/skill integration. Record actual observability/control limits.
6. **Communication:** after its contract is selected, demonstrate multiple participants, retained
   data in Pool-owned Resources, current-access delivery, recovery and independent frontend views.

Failure fixtures must cover stale template edits, attempts to bypass Conduit-mediated changes, wrong-account
login, missing references, foreign-Pool ownership, ambiguous creation/write results, partial setup,
expired interaction, stale completion, cancellation after effects, shared-login failure versus caller
revocation, last-consumer loss without shared-credential deletion, and owner retirement. Neither native enforcement nor live provider compatibility is established
by a synthetic test. Supporting evidence remains in [configuration](research/CONFIGURATION.md),
[Pi connectors](research/PI_CONNECTORS.md) and [Pi runtime](research/PI_RUNTIME.md).
