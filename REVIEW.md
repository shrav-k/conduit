# Template and instance contract review

Updated: 2026-10-31. [SPEC.md](SPEC.md) defines the four-family model; [DESIGN.md](DESIGN.md)
records accepted behavior. This agenda contains unresolved choices, grouped around connected
outcomes rather than a long sequence of individual field rulings.

## Starting point

- Session, Resource, Pool and Conduit each have template and instance forms. Profile means a
  Session template; an opened connection is a Conduit instance. Neither adds a family.
- Objects use one family `kind` plus a top-level `form` identifying a template or instance.
  Names are unique per kind across both forms in the local registry. Templates name reusable
  concepts; instances default to the template name plus a generated unique suffix. Callers
  may supply explicit unique names; exact suffix format and name constraints remain open.
- Readable object references explicitly include kind, form and name. Kind/name locates the
  target and form validates its intended use; wrong-form targets are rejected.
- Templates and creation choices produce a complete instance spec across all four families.
  Later edits target that instance; its creation template can change or disappear independently.
  No template history or input/override ledger is required. Continuing Resource and authority
  dependencies are explicit; current-access checks and actual execution evidence remain.
- Direct creation from complete instance YAML is also supported, with an explicit unique name.
  Both paths use the same validation and authority rules; Resource creation still goes through
  a Pool. Creation, provisioning and execution remain distinct effects.
- Template use expands an instance draft customized through ordinary instance fields, with no
  separate parameter declarations/mappings. YAML/CLI/API editing may be fully automated;
  registry submissions remain complete configurations.
- Draft assignments replace whole field values, including objects/lists. Nested-child edits
  preserve siblings; preserving or adding entries requires deliberate edits, not implicit merges.
- Instance creation and checked replacement updates store declared schema defaults explicitly.
  Instance equality compares materialized values; templates preserve authored omissions.
  Unchanged updates still require current authorization and matching ID/version checks.
- Typed configuration rejects unknown fields except in schema-declared open maps. Plugin
  schemas may define fields and open maps; map keys/values still follow their contracts.
- Create/replacement writes require their applicable schemas. An unavailable required schema
  rejects the write, preserving source YAML and any existing registry object.
- Otherwise valid configuration may be registered with a missing plugin runtime or offline
  configured service. Report readiness gaps/unknowns and block affected operations; registration
  does not imply activation or relax authority, validation or Pool ownership.
- Pool creation registers identity, configuration and its Resource plan; provisioning is
  requested explicitly. A frontend may combine both intents in one explicit action with
  separate registration and provisioning outcomes.
- Each named plan entry retains its created Resource's stable ID across provisioning runs.
  Bindings are per entry, not per template; additional creation is explicit. Reuse alone
  neither overwrites the Resource spec nor repeats native creation effects.
- Every Resource instance has one owning Pool instance and is created/attached only through
  that Pool. Reusable templates are not deployed Resources and need no owning Pool merely to exist.
- Different Pools may create independent backing data or attach distinct instances to shared
  backing data where the contract supports it. One Resource instance never has multiple owners.
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
The candidate [scenario YAML](research/templates-instances.scenario.yaml) is a review fixture,
not an accepted authoring/API format. It contains no real credentials or executable operations.

## Next discussion

Continue with **plan-entry changes and Resource lifecycle**. Repeated provisioning reuses
entry bindings; decide what removing or changing an entry does to its existing Resource,
and how missing targets or explicit replacements are handled. Then resolve partial outcomes
and ownership transitions. Remaining field-value rules stay on the agenda below.

| Group | Outcome to settle |
| --- | --- |
| 1. Forms, identity and instance creation | One clear authoring/inspection/reference model for all four families. |
| 2. Pool provisioning and Resource ownership | How instances are created, shared backing is represented, and ownership changes safely. |
| 3. Instance lifecycle and change | Which state survives edits, detach, restart and deletion; which operations cause effects. |
| 4. Connections, authority and communication | Initial extensible interaction contracts and their observable guarantees. |
| 5. Resolution, activation and integrations | How selected capabilities become usable in each runtime without hiding native behavior. |
| 6. Validation, evolution and experiment | Language conformance and the smallest convincing end-to-end evidence. |

## 1. Forms, identity and instance creation

| Question | Example and considerations |
| --- | --- |
| What are the remaining materialization rules? | Declared defaults are stored in instances; template omissions are preserved. Define each form's fields, scalar/list normalization, null/empty behavior, required/optional validation and version transitions. References may stay live; external contents and system-owned status are not copied into spec. |
| How are instances addressed and rebound? | Open-new uses a template or complete configuration; join-existing targets a connection instance. Define optional direct-ID references and Pool-local display names without changing registry uniqueness. Settle repeat-request identity, suffix format/name constraints, generated-collision handling, explicit rebind operations and unavailable-target/export-refresh diagnostics. Reusing one template must not deduplicate two intentional instances. |

**Walkthrough:** create two instances from one template, edit one instance, then rename, edit
and delete the template. Show independent instance specs and explicit dependencies, including
a Pool's Resource-template references for future provisioning. Repeat with a name collision
and direct creation from exported configuration with a distinct name.

## 2. Pool provisioning and Resource ownership

| Question | Example and considerations |
| --- | --- |
| What do plan-entry changes do? | Named entries retain Resource-ID bindings across runs. Define removal, edits, renames, missing targets and explicit replacement. Distinguish the plan's creation instructions from the Pool's ownership and each Resource's authoritative spec. Reuse does not imply automatic overwrite, replacement or background reconciliation. |
| How does a plan handle partial outcomes? | Define required/optional items, dependencies, independent work, success/blocked/unknown reporting and safe retry. Registration remains distinct from provisioning, including in a combined frontend action. |
| How is Resource creation requested through a Pool? | Define create-new-backing versus attach-existing, instance field values, native target identity, owner assignment and readiness. A connection requiring a conversation store must name or resolve its provisioning Pool before a Resource is usable. |
| How are selectors used now? | Explicit references/labels can select templates or a Pool's own Resource instances. Decide which contexts permit each, live behavior and duplicate rules. Matching a foreign Resource must not adopt it or make it a second member. |
| How do separate instances describe shared backing? | Two Pools attach to the same Linear workspace or local directory; another pair provisions separate worktrees. Record the distinction, management/deletion authority, credential boundaries and conflicts on the actual backing target across registrations. |
| What happens when ownership ends or changes? | Define whether transfer is supported, which authority is required, and how owner deletion handles active uses, retained data and incomplete provisioning. An unowned Resource is a lifecycle problem, not automatic proof that its backing data can be purged. |
| How do content contexts fit Pool ownership? | A memory Resource contains task/personal records and a workspace has a task association. Preserve record provenance and sharing without making those contexts alternative Resource owners or registering every message/file as a top-level Resource. |

**Walkthrough:** instantiate the same resource template in two Pools; test shared and independent
backing modes. Open a connection needing storage in a third Pool. Fail halfway through provisioning,
retry, then retire a Pool with active users and retained conversation history. Show exact survivors.

## 3. Instance lifecycle and change

| Question | Example and considerations |
| --- | --- |
| What identity/state belongs to each instance? | Identify authoritative connection state, participant attachments, native clients, execution state and backing Resources; reverse views should not become competing editable authorities. Not every instance needs a running process. |
| Which operations and transitions exist per family? | Distinguish instantiate, provision, attach, open, join, detach, suspend, close, stop, retire, unregister and purge as applicable. Select initial supported operations instead of applying every verb to every family. |
| How do instance configuration edits take effect? | Checked updates edit the complete instance spec; templates do not propagate changes. Define validation, active-work boundaries, partial failure and restart requirements. Distinguish changes to instance settings from live authority, Resource content and matching. |
| What persists after the last participant leaves? | Contrast a durable exchange, an ephemeral request, a service client and its retained outcome. Define runtime restart/recovery and whether a connection can exist with no attached Sessions. Session reconnect/resume rules remain unchanged. |
| What does deletion mean for templates and instances? | Preserve checked deletion and external-data protection. Distinguish ordinary incoming references from ownership dependencies and live runtime constraints. Session deletion already requires confirmed execution end. Define other families without silent cascades or implicit shutdown. |

**Walkthrough:** one participant changes model/account, another disconnects, then the connection
runtime fails after a remote effect was accepted. Show which identities survive, what can reconnect,
what requires explicit resume, what authority is rechecked and what remains unknown.

## 4. Connections, authority and communication

| Question | Example and considerations |
| --- | --- |
| What is the minimum typed interaction contract? | Define endpoint roles, inputs/results, capabilities, operation identity, receipt versus completion, cancellation and recovery. Pool access and Session exchange are examples, not an exhaustive endpoint taxonomy. |
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

1. **Offline model:** author reusable templates, instantiate two Pools, express both separate
   and shared backing, and open/join connections with inspectable ownership and no orphan path.
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
Retain references to the research in [CONFIGURATION.md](research/CONFIGURATION.md),
[PI_CONNECTORS.md](research/PI_CONNECTORS.md) and [PI_RUNTIME.md](research/PI_RUNTIME.md).
