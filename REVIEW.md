# Template and instance contract review

Updated: 2026-10-31. [SPEC.md](SPEC.md) defines the four-family model; [DESIGN.md](DESIGN.md)
records accepted behavior. This agenda contains unresolved choices, grouped around connected
outcomes rather than a long sequence of individual field rulings.

**Active branch discussion:** [Construction templates and runtime instances](research/CONSTRUCTION.md).
Conduits remain ongoing connections and filters; reusable templates contain typed configuration
and an inline or referenced plugin graph, executed on every instance start. Agent is the proposed
replacement for Session. [Plugin contracts](research/CONSTRUCTION.md#plugin-contracts-and-graph-nodes)
now own typed inputs/outputs, setup publication timing and per-output validity/lifetime; nodes bind
values, and those bindings establish dependencies alongside explicit prerequisites. Graphs establish
instances; instances manage ongoing processes, connections and availability. Setup nodes do not
remain live to supervise the capabilities they establish. Templates declare typed ongoing
runtime behavior, using type-provided defaults; setup graphs supply prepared inputs. Graphs may
request startup of the declared runtime during setup, under immediate instance management.
Required setup failure normally leaves that runtime managed and running, with setup incomplete;
inspection, retry or shutdown is explicit. Unchanged setup retries continue from valid recorded
progress. Temporary MCP transport failure after successful setup defaults to bounded automatic
reconnection where supported; templates can select manual recovery. Consumers normally continue
independent work while the unavailable dependency is reported; initial preparation still requires
required connections, and existing mandatory stop rules remain applicable.

Next review [instance executable configuration](research/CONSTRUCTION.md#instance-yaml-and-an-optional-composition-view):
retain the resolved setup graph and runtime configuration in the instance's own spec, or retain
an immutable versioned graph reference that must remain resolvable? This is current executable
configuration, not mandatory template history. Exact fields, status representation and retry
limits remain open. Caching/build optimizations remain implementation concerns. The checkpoint
and detailed agenda below preserve the pre-inquiry discussion for comparison; no additional
runtime family is selected.

**Review Resources before Pools.** Prefer simple, unitary Resources and minimal duplicated bookkeeping.
Use concrete Resource types to establish templates, instances and consumer access, then derive
Pool grouping and provisioning. A persistent declaration/binding layer is not a prerequisite.

## Starting point

[SPEC.md](SPEC.md) defines the model; the linked DESIGN sections retain the detailed rulings.
This agenda summarizes constraints needed for the next discussion; the full accepted contracts
remain in DESIGN.md.

| Area | Accepted baseline and detailed contract |
| --- | --- |
| Families | Session, Resource, Pool and Conduit each have template/instance forms. A Conduit instance is a live filtered flow, implemented by the common engine/plugins rather than its own independent runtime. See [families](DESIGN.md#families-templates-and-instances). |
| Establishment | Templates carry complete setup intent, including create/attach choices. Lifecycle actions select when; they do not supply a second setup recipe. Instances retain ongoing spec and observed status, with no mandatory template history. Exact transitions and incomplete instance imports remain open. See [establishment](DESIGN.md#establishment-contracts-and-instance-state). |
| Configuration | Typed YAML, top-level form, names unique per family, explicit kind/form/name references, stable IDs, checked replacements and strict schemas remain settled. Instance defaults are explicit; templates preserve omissions. Registration has no provisioning/execution effects. See [configuration](DESIGN.md#configuration-language). |
| Resources | One responsibility with related operations; separate type group/name/version, config and conduits. Working groups: filesystem, storage, search and integration. Local-directory and git-worktree are distinct types; create/attach and path-resolution rulings remain. Storage, search and MCP access are separate Resources by default. See [Resources](DESIGN.md#resources). |
| Ownership | Each Resource instance is created/attached only through its one owning Pool. Shared external backing does not imply shared registry ownership. Persistent Pool plans/bindings are parked pending Resource contracts. See [Pools](DESIGN.md#pools). |
| Access preparation | Session/Resource declarations supply their enclosing consumer automatically. Explicit preparation attempts open/join; required by default, optional explicitly. Partial success remains for retry; cancellation detaches. Folder symlinks are preparation, not new Resources; MCP identity can differ per attachment where supported. See [Conduits](DESIGN.md#conduits) and [Resources](DESIGN.md#resources). |
| Flow lifetime | Open-new and join-existing are explicit. Logical consumers/peers, not sockets, keep flows alive; the last departure ends the flow while backing Resources and retained evidence survive. Filters narrow authority; Session activation prepares permitted capabilities. See [Conduits](DESIGN.md#conduits). |
| Retained guarantees | Account/private-context separation, continuity, memory revisions/sharing/search, current-access checks, safe retries, explicit cleanup and retention remain in force. See [Sessions](DESIGN.md#sessions), [memory](DESIGN.md#retained-memory-and-evidence), [authority](DESIGN.md#authority-and-current-access) and [recovery](DESIGN.md#outcomes-and-recovery). |

Content, identity and agent-communication type groups remain candidates. No communication
topology, plugin ABI, workflow language or detailed lifecycle schema has been selected.

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

**Resumption checkpoint:** the accepted model is intact. The last completed review clarified
plugin-owned stages versus optional author-written typed composition using four filesystem
examples. Neither option has been selected. Resume with the establishment-ownership bundle
below, then dependencies/outputs, interaction/recovery and template-to-instance state. The
research recommendations and example field names are not accepted contracts.

Settle **the establishment language and execution boundary** using
[Establishment, ordering, outputs and recovery](research/ESTABLISHMENT.md). First use the
[filesystem comparison](research/ESTABLISHMENT.md#filesystem-plugins-and-template-examples) to
distinguish plugin-owned stages from optional template-authored graphs: existing directory plus
Session symlink, new directory, remote filesystem and Git worktree. No ruling has been made
between these options. An authored graph may compose safe typed plugin actions; it need not
introduce raw scripts or a general programming language. Then stress-test the boundary with:

1. A Session signs in to a selected Claude subscription, waits for browser interaction, verifies
   identity, then prepares required connections without starting model work implicitly.
2. Pool-owned corpus, storage and indexing/search capabilities produce a usable vector index
   through authorized Conduits, with partial batches, generated outputs and recovery.
3. A Session or Resource obtains password-manager access through a Conduit, with participant-
   specific identity, possible human unlock and expiring access.

**Proposed boundary, awaiting ruling:** standard Resource configuration selects complete,
recoverable plugin lifecycles; the engine coordinates dependencies, interaction waits and results.
An optional authored graph could compose those same typed operations for custom setup. Decide
whether that composition ships in v0 separately from loops, scripts or a general workflow language.
Both options need ordering/results/recovery; they differ in who authors the procedure.

Resolve these bundles in order, not as isolated field-name choices:

| Bundle | Ruling and evidence needed |
| --- | --- |
| Establishment ownership | Use the four filesystem cases to choose standard plugin lifecycles and whether optional author-written typed composition is needed in v0. Define what a plugin exports, what the author can vary, and what stages/outputs remain inspectable. Preserve Resource establishment versus Session-access preparation. |
| Dependencies and outputs | Define strict prerequisites, bounded independent work, readiness conditions, typed output references/lifetimes and blocking-cycle diagnosis. Keep logical Resource access in Conduits. |
| Interaction and recovery | Define waiting for a person, retained partial setup, safe retry/reconciliation, versioned continuation, cancellation and late-worker results. Separate setup continuation from starting a stopped model execution. |
| Template-to-instance transition | Use successful and failed cases to decide creation-only setting retention, status, instance export/import and one-off authoring. Preserve minimal bookkeeping and current authority. |
| Extensibility | Compare a custom worktree → seed → validate sequence with ordinary Resource configuration. If typed composition is selected, define its home in the existing families and stage/version/recovery semantics. Loops, arbitrary expressions and shell execution are separate extensions, not prerequisites for a small graph. |

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
