# Template and instance contract review

Updated: 2026-10-31. [SPEC.md](SPEC.md) defines the four-family model; [DESIGN.md](DESIGN.md)
records accepted behavior. This agenda contains unresolved choices, grouped around connected
outcomes rather than a long sequence of individual field rulings.

**Review Resources before Pools.** Prefer simple, unitary Resources and minimal duplicated bookkeeping.
Use concrete Resource types to establish templates, instances and consumer access, then derive
Pool grouping and provisioning. A persistent declaration/binding layer is not a prerequisite.

## Starting point

- Session, Resource, Pool and Conduit each have template and instance forms. Profile means a
  Session template; a Conduit instance is a live filtered flow. An attachment is logical
  consumer/peer participation, not a transport socket or another family.
- Objects use one family `kind` plus a top-level `form` identifying a template or instance.
  Names are unique per kind across both forms in the local registry. Templates name reusable
  concepts; instances default to the template name plus a generated unique suffix. Callers
  may supply explicit unique names; exact suffix format and name constraints remain open.
- Readable object references explicitly include kind, form and name. Kind/name locates the
  target and form validates its intended use; wrong-form targets are rejected.
- Templates contain the establishment configuration across all four families. Lifecycle actions
  select when to realize it; users do not assemble a separate operation-request recipe.
  Local-directory create/attach intent belongs in the template spec. Type-defined transitions
  produce ongoing instance configuration and observed status; their exact fields remain open.
  Later edits target that instance; its creation template can change or disappear independently.
  No template history or input/override ledger is required. Continuing Resource and authority
  dependencies are explicit; current-access checks and actual execution evidence remain.
- Direct-instance creation/import is under review where exports omit establishment information.
  Preserve one-off authoring without guessing create/attach intent or requiring a parallel recipe.
  Resource creation still goes through a Pool. Registration, provisioning and execution remain
  distinct effects. Ordinary typed-field editing remains the baseline; no generic parameter,
  mapping or scripting language is approved. Templates need not have identical instance fields.
- Draft assignments replace whole field values, including objects/lists. Nested-child edits
  preserve siblings; preserving or adding entries requires deliberate edits, not implicit merges.
- Instance creation and checked replacement updates store declared schema defaults explicitly.
  Instance equality compares materialized values; templates preserve authored omissions.
  Unchanged updates still require current authorization and matching ID/version checks.
- Typed configuration rejects unknown fields except in schema-declared open maps. Plugin
  schemas may define fields and open maps; map keys/values still follow their contracts.
- Resource specs have a dedicated, schema-validated block for type-specific settings alongside
  common fields: `spec.type` contains separate `group`, `name` and `version` fields, `spec.config` holds
  the type's settings, and `spec.conduits` declares connections. Type contracts are versioned
  independently of common apiVersion and plugin releases. Allowed identifiers/version labels
  and declaration contents remain open; type references are not registry object references.
- Logical type groups are independent of implementation packages. A package may supply several
  groups/types. The catalog remains open to new groups and types; no slash-delimited type name.
  The working core groups are filesystem, storage, search and integration. Detailed type contracts
  and third-party contract ownership/collision rules remain for review. Keep an agent-communication
  group as a candidate, with direct messaging, conversations and shared context as design probes.
- Filesystem has separate `local-directory` and `git-worktree` types: shared directory access,
  with Git-specific management in the latter. Local-directory supports explicit attachment and
  creation through its Pool in v0: attachment requires an existing directory; fresh creation
  conflicts with an existing target. Registration and cleanup remain separate effects. Exact
  schemas, Git-specific modes and any shared interface remain open.
- Local-directory templates may use relative backing paths resolved against an explicit
  creation base. Complete instances store absolute host paths, independent of later shell
  working directories. Symlink resolution and Session-local exposure remain separate choices.
- Create/replacement writes require their applicable schemas. An unavailable required schema
  rejects the write, preserving source YAML and any existing registry object.
- Otherwise valid configuration may be registered with a missing plugin runtime or offline
  configured service. Report readiness gaps/unknowns and block affected operations; registration
  does not imply activation or relax authority, validation or Pool ownership.
- Pool registration and Resource provisioning are separate explicit effects. Pool declaration
  and binding mechanics are paused for reconsideration after the Resource contracts below.
- Every Resource instance has one owning Pool instance and is created/attached only through
  that Pool. Reusable templates are not deployed Resources and need no owning Pool merely to exist.
- Storage, search and MCP access are separate Resources by default, grouped in Pools, even
  when backed by one provider. Each Resource has one responsibility with related operations;
  individually callable operations/tools do not each require a Resource.
- Logical Resource-to-Resource access uses Conduits, including search reading storage.
  Pool grouping alone does not establish the connection or grant access. This does not create
  a Conduit per query/socket; declaration syntax and connection creation remain open.
- Resource and Session templates can reference Conduit templates to declare flows for their
  instances. The enclosing instance automatically supplies the consumer endpoint without a
  local-endpoint marker; this grants no authority or implicit open/join choice.
  The engine/plugins perform the work; a Conduit has no independent runtime.
  One-time transfers, persistent access and ongoing sync are examples, not a closed taxonomy.
- Explicit Session/Resource preparation normally attempts declared open/join actions and reports
  their outcomes. Registration alone starts nothing. Required connection failure blocks the
  declaring consumer's completion of preparation; optional failure is reported and allows
  preparation to continue. Required is the default; optional must be explicit. Saved instance
  configuration materializes that default. Unrelated consumers are unaffected; retry timing remains open.
- Partial preparation retains successful attachments for retry while the consumer stays
  unprepared. Explicit cancellation detaches those attachments; other consumers, backing
  Resources and recorded outcomes survive. Delivered data/external effects are not rolled back.
- The last logical consumer/peer detaching ends a live Conduit and closes remaining provider-side
  connections. Sync subscriptions remain attached between polls or temporary transport failures.
  Backing Resources, templates and retained evidence survive; crash/detach detection remains open.
- Different Pools may create independent backing data or attach distinct instances to shared
  backing data where the contract supports it. One Resource instance never has multiple owners.
- A local-folder Resource describes the configured folder. Session symlinks are preparation
  state, not separate Resources; removing one preserves the Resource, folder and other views.
- An MCP Resource permits different service identities per Session attachment where supported,
  within its target/access constraints. Service logins do not require duplicate Resource instances
  and remain distinct from model subscriptions; credential storage/refresh details are open.
- A connection requests supporting Resources through a Pool. Its shared operational state,
  participant attachments and backing Resource lifetimes are distinct.
- Open-new and join-existing connection intents are explicit. Filters narrow; activation in
  the Session spec prepares permitted Resources. Pool provisioning is a different operation.
- Existing account, context, memory, access, retention and checked-edit rules remain. Instance
  form does not imply a process, automatic reconciliation or automatic cascading deletion.

## Review method

For each group, work through **authored intent → resolved identities/configuration → allowed
operations → observed result**, then one change and one failure. Resolve related questions as
one contract. Move accepted conclusions into DESIGN.md and remove settled questions here.

Do not assume that fewer family names imply one universal schema, lifecycle or merge algorithm.
For each proposed field or stored relationship, identify its consumer and why it cannot be
derived from existing authoritative configuration. Do not add uniform state to all types for
one type's needs. A Resource template configures a type; it does not define the type's semantics.
The candidate [scenario YAML](research/templates-instances.scenario.yaml) is a review fixture,
not an accepted authoring/API format. It contains no real credentials or executable operations.

## Next discussion

Settle **the establishment language and execution boundary** using
[Establishment, ordering, outputs and recovery](research/ESTABLISHMENT.md). Directory examples
alone do not expose enough of the contract. Start with these three cases as one connected review:

1. A Session signs in to a selected Claude subscription, waits for browser interaction, verifies
   identity, then prepares required connections without starting model work implicitly.
2. Pool-owned corpus, storage and indexing/search capabilities produce a usable vector index
   through authorized Conduits, with partial batches, generated outputs and recovery.
3. A Session or Resource obtains password-manager access through a Conduit, with participant-
   specific identity, possible human unlock and expiring access.

**Proposed boundary, awaiting ruling:** templates declare the result, identity, dependencies and
supported policy; plugins implement typed, resumable stages; the engine coordinates prerequisites,
interaction waits, progress and output delivery. The requirement for ordering/results/recovery
is demonstrated; whether authors need a general workflow language is a separate question.

Resolve these bundles in order, not as isolated field-name choices:

| Bundle | Ruling and evidence needed |
| --- | --- |
| Establishment ownership | Choose the template/plugin/engine split. Require inspectable semantic stages and failure behavior even when procedure code stays in plugins. Decide whether the first three cases need authored steps. |
| Dependencies and outputs | Define strict prerequisites, bounded independent work, readiness conditions, typed output references/lifetimes and blocking-cycle diagnosis. Keep logical Resource access in Conduits. |
| Interaction and recovery | Define waiting for a person, retained partial setup, safe retry/reconciliation, versioned continuation, cancellation and late-worker results. Separate setup continuation from starting a stopped model execution. |
| Template-to-instance transition | Use successful and failed cases to decide creation-only setting retention, status, instance export/import and one-off authoring. Preserve minimal bookkeeping and current authority. |
| Extensibility | Identify a real case needing user-defined ordered calls, loops or transformations before selecting extra language syntax. Plugins remain extensible; no general workflow engine is presumed. |

Then return to detailed directory/worktree contracts and symlink behavior, followed by storage,
search and MCP. Filesystem/storage/search/integration groups and separate directory/worktree
types remain settled. Content, identity and agent communication remain candidates; do not reopen
those group rulings or choose a communication topology as a side effect of lifecycle design.

The existing preparation baseline still applies: required by default, optional failures reported,
successful attachments retained for retry, and explicit cancellation detaching them. Pool
creation declarations/bindings remain parked; the engine may derive a dependency view without
requiring users to maintain another plan. Composition does not add an object family.

| Group | Outcome to settle |
| --- | --- |
| 1. Resource templates and instances | Typed establishment, ongoing configuration, outputs and consumer boundaries. |
| 2. Pool grouping and Resource ownership | Coordinate understood Resource types with minimal duplicated state. |
| 3. Object configuration and lifecycle | Interaction/recovery protocol, state transitions and export/import. |
| 4. Connections, authority and communication | Extensible flows with explicit dependencies, capabilities and lifetimes. |
| 5. Resolution, activation and integrations | Provider setup and selected capabilities become usable in each runtime. |
| 6. Validation, evolution and experiment | Lifecycle conformance, fault cases and bounded real-provider evidence. |

## 1. Resource templates and instances

| Question | Example and considerations |
| --- | --- |
| Which types belong in the working core? | Filesystem, storage, search and integration are accepted groups; local-directory and git-worktree are distinct filesystem types. Work through their contracts and other candidate types; content, identity and agent communication remain extensions to explore. Decide contract ownership/collision handling separately; shipping types/templates does not create Resource instances. |
| What establishes compatibility across types? | Evaluate explicit versioned consumer interfaces and per-instance supported operations. A shared group, matching JSON shape or provider name does not establish behavioral compatibility. Keep declared support, current availability and permissions distinguishable without requiring a large capability framework. |
| How are Resource connections declared? | The enclosing Session/Resource supplies consumer identity; explicit preparation attempts declared open/join, required by default. Partial failure retains successful attachments for retry; cancellation detaches them. Settle declaration fields, remote references, retry/timeout limits, incomplete cleanup and ordering. Keep resolved state and later edits free of competing editable copies. Pool ownership/access still apply. |
| What belongs in the type contract, template and instance? | The contract defines establishment and ongoing behavior. The template supplies all establishment choices/references; the instance records ongoing spec and observed status. Decide whether create/attach remains as immutable instance configuration or only as an establishment outcome. Define form-specific validation and field transitions without a second recipe or mandatory template history. |
| What is genuinely common across types? | Resource specs use type {group, name, version}, config and conduits. Settle allowed identifiers/version labels and the contents/shape of connection declarations. Type-contract versions stay independent of common apiVersion and plugin releases. Distinguish type-contract declarations from observed per-instance support; add common descriptors only when examples require them. |
| What is shared and what is prepared per consumer? | Folder symlinks are preparation; MCP identities resolve per attachment. Define credential storage/refresh, client lifetime and search query/filter state without registering every helper. A reusable configuration does not imply shared private state. |
| How are backing data and conflicts represented? | Two instances can attach to the same target; two consumers may share one instance. Distinguish deliberate creation from attachment, read/write conflicts, shared quotas and native data ownership. Do not infer an exclusive writer from registry ownership. |
| What lifecycle does each type actually need? | Define input/output schemas, semantic stages, readiness, possible interaction waits, recovery capability and cleanup. A path attachment may finish immediately; native login can wait; ingestion can checkpoint batches. The common protocol must handle those distinctions without exposing every provider API call in YAML. |
| How are dependencies and outputs connected? | Describe required inputs and completion/readiness gates, including generated IDs, revisions and expiring handles. Output-to-input binding must respect Conduit access and Pool ownership. Decide when input/output changes invalidate dependent work and how to diagnose a blocking bootstrap cycle. |

Use these examples as design probes, not a closed list of types or an accepted schema:

| Example | Questions the contract must answer |
| --- | --- |
| Local filesystem exposed by symlink | Template config declares create/attach; relative paths resolve against an explicit base and instances store absolute host paths. Work through establishment fields and reported results, then symlink resolution, Session-local placement and cleanup timing. Removing a prepared symlink preserves the Resource. Symlinks share backing files and do not isolate writes. |
| MCP service connection | Service identity may differ per attachment within target/access constraints. Settle auth storage/refresh, exposed tools/resources/prompts and required processes. A Conduit Resource is distinct from MCP's protocol resource primitive. |
| Hybrid search over a database | Express separate storage, search and MCP-access Resources, including when supplied by one backend. Settle responsibilities, dependencies, query/filter/result contracts, freshness and indexing ownership if offered. |

**Walkthrough:** write one template and complete instance for the filesystem example, attach
two Sessions and remove one. Repeat for MCP and search. Record only the configuration/state
needed to explain the observed differences. Keep an owning Pool reference without designing
its grouping or provisioning machinery yet.

## 2. Pool grouping and Resource ownership

Return here after the Resource examples. Ownership and Pool-mediated creation remain accepted;
the representation of creation/grouping instructions is being reconsidered.

**Parked declaration proposal:** named entries in a Pool's configuration previously retained
their created Resource IDs across provisioning runs; reuse did not overwrite specs or repeat
native creation effects. Removing an entry ended that association while retaining its registered,
Pool-owned Resource until explicit cleanup. Those rulings describe that proposed mechanism,
whose necessity is now under review. Missing targets, entry edits/renames, re-addition and
replacement were not settled. Prefer a simpler representation if Resource instances and
ordinary operation outcomes can supply the needed behavior without a parallel declaration ledger.

| Question | Example and considerations |
| --- | --- |
| What should type-aware grouping do? | Group unitary Resources using declared types/capabilities, compatible Session preparation and supported operations. Derive inventory views where possible. Decide what Pools select, validate or coordinate without duplicating Resource configuration or silently activating capabilities. |
| Do Pool instances need retained creation declarations? | Compare template-time expansion/explicit Resource operations with persistent declarations and per-entry bindings. Keep the least state needed for repeated operations, later additions and partial work. Only define declaration-edit edge cases if this mechanism remains necessary. |
| How are partial outcomes handled? | Use Resource-specific effects to identify required/optional work, dependencies, independent progress and success/blocked/unknown reporting. Reuse operation IDs/outcomes for safe retry before adding a second reconciliation system. Registration remains distinct from provisioning. |
| How does a Pool establish configured Resources? | Read establishment choices from typed configuration. Resolve owner/name/context without a separate operation recipe. Settle template references or inline configuration, partial failure and readiness; avoid duplicate setup state. A connection requiring a conversation store must resolve its provisioning Pool. |
| How are selectors used now? | Explicit references/labels can select templates or a Pool's own Resource instances. Decide which contexts permit each, live behavior and duplicate rules. Matching a foreign Resource must not adopt it or make it a second member. |
| How do separate instances describe shared backing? | Two Pools attach to the same Linear workspace or local directory; another pair provisions separate worktrees. Record the distinction, management/deletion authority, credential boundaries and conflicts on the actual backing target across registrations. |
| What happens when ownership ends or changes? | Define whether transfer is supported, which authority is required, and how owner deletion handles active uses, retained data and incomplete provisioning. An unowned Resource is a lifecycle problem, not automatic proof that its backing data can be purged. |
| How do content contexts fit Pool ownership? | A memory Resource contains task/personal records and a workspace has a task association. Preserve record provenance and sharing without making those contexts alternative Resource owners or registering every message/file as a top-level Resource. |

**Walkthrough:** instantiate the same resource template in two Pools; test shared and independent
backing modes. Open a connection needing storage in a third Pool. Fail halfway through provisioning,
retry, then retire a Pool with active users and retained conversation history. Show exact survivors.

## 3. Object configuration and lifecycle

| Question | Example and considerations |
| --- | --- |
| What are the remaining materialization rules? | Declared defaults are stored in instances; template omissions are preserved. Define each form's fields, scalar/list normalization, null/empty behavior, required/optional validation and version transitions. References may stay live; external contents and system-owned status are not copied into spec. |
| How does one-off authoring and instance reuse work? | Can an unregistered template document supply establishment configuration? What can instance export reconstruct, and which choices must authors explicitly add? No guessed create/attach policy, imported ready status, or automatic recreation of lost backing. Decide this alongside the transition, not as a new operation language. |
| Is more language expressiveness needed? | The three establishment cases require multi-stage execution. Decide which stage ordering belongs in plugin code and which composition users author. Compare declarative typed bindings against ordered typed calls, a workflow plugin and richer code generation; preserve one validated semantic model. |
| What is the recoverable establishment protocol? | Define meaningful phase/result persistence, effective-input identity, version compatibility, structured interaction waits, native job reconciliation and stale-worker rejection. Plugin code can resume only at supported boundaries. Retry limits cannot override unsafe effects; cancellation is not a rollback. |
| How are instances addressed and rebound? | Open-new uses establishment configuration; join-existing targets a connection instance. Define optional direct-ID references and Pool-local display names without changing registry uniqueness. Settle retry identity, suffix format/name constraints, generated-collision handling, explicit rebind operations and unavailable-target/export-refresh diagnostics. Reusing one template must not deduplicate two intentional instances. |
| What identity/state belongs to each instance? | Identify authoritative connection state, participant attachments, native clients, execution state and backing Resources; reverse views should not become competing editable authorities. Not every instance needs a running process. |
| Which operations and transitions exist per family? | Distinguish instantiate, provision, attach, open, join, detach, suspend, close, stop, retire, unregister and purge as applicable. Select initial supported operations instead of applying every verb to every family. |
| How do instance configuration edits take effect? | Checked updates edit the complete instance spec; templates do not propagate changes. Define validation, active-work boundaries, partial failure and restart requirements. Distinguish changes to instance settings from live authority, Resource content and matching. |
| How do attachments end and flows recover? | Last logical consumer/peer departure ends the flow and closes providers; temporary transport failure does not itself detach. Define explicit detach, crash/lease detection, incomplete cleanup, one-time completion and later reopening/new-instance identity. Backing Resources and retained evidence survive; Session reconnect/resume rules remain unchanged. |
| What does deletion mean for templates and instances? | Preserve checked deletion and external-data protection. Distinguish ordinary incoming references from ownership dependencies and live runtime constraints. Session deletion already requires confirmed execution end. Define other families without silent cascades or implicit shutdown. |

**Walkthrough:** one participant changes model/account, another disconnects, then the connection
runtime fails after a remote effect was accepted. Show which identities survive, what can reconnect,
what requires explicit resume, what authority is rechecked and what remains unknown.

## 4. Connections, authority and communication

| Question | Example and considerations |
| --- | --- |
| What is the minimum typed interaction contract? | Define endpoint roles, inputs/results, capabilities, operation identity, receipt versus completion, cancellation and recovery. Pool access and Session exchange are examples, not an exhaustive endpoint taxonomy. |
| What belongs in agent-communication Resources versus Conduits? | Explore a dedicated group for direct messages, conversations and shared context, justified by agent-specific semantics even when backed by ordinary storage. Compare Session addressing, history/context ownership and persistent capabilities with live routing/filtering and attachments. Decide whether direct messages and rooms need separate types or configuration; distinguish shared snapshots/references from mutable shared state. No name, topology or extra runtime is selected yet. |
| How is authority established and narrowed? | Separate caller request from an authorized grant; authenticate participants and shared runtime service identity. Define reusable/inline filters, exclusions and complete resource/operation/constraint tuples across paths. Never combine unrelated fields into broader permission. |
| How does a connection use supporting Resources? | Resolve the owning Pool, provisioning authority and access to retained history or queues. Runtime bookkeeping need not be a separate Resource per cursor, but cannot be an unscoped storage route for durable shared content. |
| How do connections compose? | Compare a named selection with a composite Conduit template. An `euphony-access` setup can include repository, credential, AWS and Linear capabilities with separate clients, principals, failure states and lifetimes. Define dependencies and partial effects without promising external rollback. |
| Which communication semantics are first? | Compare request/result, retained exchange and observation using actual Session workflows. Specify addressing/fan-out, joining, ordering, durable acceptance, acknowledgement, replay/deduplication, offline recipients and bounded backpressure. |
| How do exchanges affect attention/context? | Distinguish passive availability, unread notices, queued input and starting model work. Preserve user control and paused/closed semantics; decide wake hooks, permitted checkpoint/context delivery and native tracker associations. Previous room-only rules remain reopened. |
| What coordination is actually supported? | Identify backing-target conflicts across multiple connections/Resource registrations, shared quotas and advisory versus enforced exclusion. Native tools outside mediation cannot be covered by an unsupported guarantee. |

Use [the five connection scenarios](research/CONDUITS.md) throughout. **Walkthrough:** one
request produces a pending result, a finding reaches several participants including one offline,
and an observer falls behind. Remove access before delivery. Show acceptance, processing,
completion, omission and unknown outcomes without inventing a model turn or widening access.

## 5. Resolution, activation and integrations

| Question | Example and considerations |
| --- | --- |
| What is the constrained YAML data surface? | Within the accepted strict unknown-field rule, define string keys, duplicate-key handling, scalars/units, null/empty/omitted values, aliases and normalization. Choose explicit configuration roots, path bases and environment inputs. Credentials are references, not ordinary interpolated values. |
| How do defaults and selections compose? | Materialize Session-template connection choices, selected Pool defaults and creation choices in the instance spec. Explicit selection already replaces Pool defaults unless add is requested. Define other precedence, list ordering and duplicate identity without a universal deep merge. |
| What is the activation contract? | Session specs contain instructions and structured preparation, initially supplied by templates. Distinguish these from Pool provisioning and connection startup. Define required/optional actions, ordering, compatibility, duplicate skills/commands, refresh and cleanup of projections. |
| What are the first adapter contracts? | Separate own-runtime model providers from native-agent adapters. Define reported support for credentials, context conversion, hooks, reconnect/cancel, subagents and telemetry; do not infer support from model branding. |
| How are accounts and native packages prepared? | Validate four independent subscriptions, actual identity, refresh/logout concurrency, native config selection and private state. Use native skill/plugin tooling; unsupported extensions stay explicit. |
| How are MCP and resource-service clients realized? | Separate definition reuse, per-participant auth, native registration and live client/helper lifetime. Preserve provider-native contracts and negotiated versions; report restrictions that cannot be enforced. |
| Which memory, workspace and isolation details are needed first? | Select backend/search budgets, content-context mapping, worktree base/path settings and isolated-mode technology/network matching. Retain scoped memory, explicit safe cleanup, native default and required-isolation rules. |

**Walkthrough:** the same Resource template is instantiated through Pools for own-runtime and
native-agent Sessions. Show the native schemas retained, different preparation, exact identities,
unsupported hooks and a change of model/account without leaking private context.

## 6. Validation, evolution and experiment

| Question | Example and considerations |
| --- | --- |
| What do offline validation and explicit probes promise? | Schemas and required registry checks govern registration; operational readiness is reported separately. Define how readiness is established/refreshed. Separate syntax, reference, semantic and transition checks from network/login/process effects. Preview does not freeze current access. Preserve multiple scoped errors and unknown live facts. |
| How are diagnostics and status represented? | Choose metadata/status field names, source locations, stable categories, evaluated versions and independently reported validity, auth, readiness and enforcement. Avoid one universal ready flag. |
| What are version/migration boundaries? | Choose API group/version, family/form schemas and plugin dialect. Separate template config versions, instance resolved versions, record revisions and runtime releases. Unknown semantics must not silently degrade to a familiar version. |
| How do edits and batches remain recoverable? | Specify existing baseline storage/association/lifecycle, checked import/export and rebind, source preservation and batch atomicity. Local registry atomicity does not imply rollback of external provisioning. |
| What is durable evidence? | Define operation/provisioning/activation results, provenance and unknown effects; distinguish these from progress views. Preserve retained transcripts, usage coverage and non-overlapping totals. |
| What demonstrates conformance? | Require matching static resolution across CLI/API/frontends and lifecycle/authority failure cases. Identify native/live tests needed beyond schema validation and synthetic adapters. |

## Initial experiment

Implementation has not started. Settle the contracts above and define a bounded experiment
before moving from documentation into implementation or live account testing.

1. **Offline model:** document template-to-instance transitions and dependency/output contracts
   for filesystem, authentication and indexing. Add synthetic stage/interaction/failure cases; the
   current structural fixture alone is insufficient. Then express grouping in two Pools,
   separate/shared backing and open/join with inspectable ownership and no orphan path.
2. **Accounts:** verify two Claude and two Codex/OpenAI subscription identities, independent
   credential refresh/logout and private Session state; record unsupported behavior explicitly.
3. **Runtime:** a minimal Rust-owned model/tool loop and terminal, with native-agent adapters
   treated separately. Test interruption, unknown writes and model-context conversion.
4. **Resources:** one memory backend, workspace and native MCP/skill integration through
   Pool-created instances. Pick actual search caps, provider versions and activation behavior.
5. **Interaction:** one agreed communication contract backed by a Pool-owned Resource, showing
   open versus join, multiple participants, current-access delivery and restart/retention behavior.

Required failure fixtures include stale edits, missing template/instance references, attempted
foreign-Pool ownership, ambiguous create retry, partial provisioning, shared-backing conflicts,
auth refresh failure, last participant departure, unknown remote effect and owner retirement.
Establishment adds wrong-account authentication, expired interaction, crash after login success,
partial ingest, input/model change mid-build, stale completion, cancellation after a write and
missing/incompatible recovery handlers. No live login is implied by these documentation cases.
Retain references to the research in [CONFIGURATION.md](research/CONFIGURATION.md),
[PI_CONNECTORS.md](research/PI_CONNECTORS.md) and [PI_RUNTIME.md](research/PI_RUNTIME.md).
