# Conduit design contracts

Updated: 2026-10-30. [SPEC.md](SPEC.md) defines the five-concept model. This document
records accepted behavior within it; [REVIEW.md](REVIEW.md) contains open choices.
The common authoring envelope is settled; detailed fields, transports, storage engines
and implementation layout remain under review.

## Configuration language

**YAML is the authoring format; typed objects and explicit semantics define the language.**
The common model consists of Profiles, Sessions, Resources, Pools and Conduits. Their
identity, reference, composition and behavioral contracts govern every entry point.
CLI/API/frontends must interpret the same objects consistently. Future generators can
emit this model; they cannot bypass its validation or authority rules.

All five kinds share this authoring envelope:

| Field | Meaning |
| --- | --- |
| `apiVersion` | The versioned contract used to interpret the object. |
| `kind` | Profile, Session, Resource, Pool or Conduit. |
| `metadata` | Author-editable naming/labels and system-owned identity/version information, with ownership defined per field. |
| `spec` | Configuration defined by that kind's contract. |

Metadata ownership is defined per field: authors control naming and labels within their
permissions, while Conduit assigns and maintains identity/version information. Placing a
field in `metadata` does not make it editable. Identity and configuration-version checks
govern updates as described below; exact field names and operation schemas remain open.

Inspection returns the object with a system-owned `status` section alongside its authored
`spec`. Status reports resolution and observed state appropriate to the kind, such as a
Session's bindings, preparation and execution. Normal authored YAML omits `status`; users
and agents request changes through configuration or operations rather than declaring an
observed state to be true.

Normal configuration writes reject an included `status` field rather than silently
ignoring it. Configuration-only export contains reusable, authorable object configuration,
omitting `status` and system-owned identity/version values. Inspection can still return
those system values. Update preconditions are separate operation inputs; they do not
authorize clients to assign system-owned values through object YAML.

The shared envelope does not give every kind the same configuration or status fields.
Exact metadata/status fields, ID encoding and remaining import/export semantics are open.

This accepts the envelope and organizing principle, not every proposed field or algorithm
in the research. In particular, the API group/version, registry storage layout, reference
extensions, YAML subset, schema toolchain and command names remain for review. Configuration
objects also do not force every memory record, native tool or runtime event into an
individually authored YAML file.

### Object identity

Every Profile, Session, Resource, Pool and Conduit has a system-assigned stable ID independent
of its name. Renaming an object preserves that ID. Deleting it and creating another object
with the same name produces a new ID; name reuse does not make them the same object.

Authored YAML normally references an object by `kind` and `name`. Conduit records the stable
ID of the object selected when that reference resolves. Authors need not copy generated IDs
into ordinary references; the readable lookup and the resolved identity remain distinguishable.

V0 resolves these references within one local registry, with object names unique per kind.
A Pool and a Resource may share a name; two distinct Resources may not. The registry
catalogs definitions and IDs, while underlying data and services may be local or remote.
Pool membership does not create a separate naming boundary. Registry membership is not an
access grant, and this naming rule does not require every exposed native item to have a
registry-wide name. Namespaces and cross-registry references are deferred.

V0 kind/name lookups use only current object names; registry object-name aliases are deferred.
After a rename, the old name becomes available for reuse. A fresh lookup of that old name
finds no object unless the name has been reused, in which case it can resolve to a different ID.
This does not redirect established bindings, which retain their resolved identity as below.

A rename uses the normal checked configuration update, changing `metadata.name` while
preserving the object's ID. The original ID/version identifies the target; the new name
must be available for that kind or the update conflicts.

Other registry objects retain their authored kind/name references after a rename. Report
known mismatches between a written name and its resolved target, and offer explicit checked
reference updates. The rename does not rewrite source YAML files.

Configuration-only export preserves authored reference names by default and reports known
mismatches. An explicit option refreshes names from known, unambiguous resolved IDs in the
exported copy only; it does not update registry objects or rebind anything. An old exported
name can still fail or identify a different object on a fresh import.

An established binding created from an explicit kind/name reference retains its resolved
stable ID until explicitly rebound. Renaming the target preserves the binding. Deleting
the target makes it unavailable; a new object using its old name is not substituted
automatically. Rebinding remains subject to current authority.

This fixes object identity, not content revisions or permissions. Current access checks
continue to apply; live selector membership remains a separate contract. The ID format,
serialized field, cross-registry identity policy, optional direct-ID forms, import/export
handling and exact rebind operations remain open.

### Configuration updates

The local registry holds active configuration in v0. YAML is explicitly applied or exported;
editing an input configuration file alone does not change registry objects, and CLI/API edits
do not silently rewrite that file. Files may remain in Git without automatic watching or
bidirectional synchronization. This defines configuration authority, not the registry's
storage technology or the update behavior of data exposed through Resources. Accepted live
access rules still apply when the registry configuration changes.

Configuration submissions explicitly select create or update intent. Creation reports a
conflict if that kind/name already exists. Updating a missing target reports a conflict
instead of creating it. Neither operation silently switches intent based on name matches;
exact commands and batch handling remain open.

A v0 update replaces the complete authored configuration, including `spec` and editable
metadata. Omitting an optional field removes its explicit setting; the field's omission,
default and validation rules apply instead of retaining its previous value. The object's
identity is preserved, and Conduit continues to maintain its system-owned version information
and status. Clients can edit a retrieved configuration copy before submitting it with the
original baseline. Partial patches are deferred.

Normal updates to an existing configuration object require its expected stable ID and
the configuration version the edit was based on. Reject the update if either differs
from the current object. This prevents stale edits from silently overwriting newer
configuration and prevents a reused name from redirecting an update to a replacement.

Registry-object deletion requires the same expected ID and configuration-version checks.
An intervening configuration change makes deletion conflict; the caller may reread and
reconcile before retrying. Incoming references alone do not block an otherwise permitted
checked deletion. Retain referring objects and their authored references, reporting affected
relationships as unavailable; do not prune those references or cascade removal to referring
objects. For example, deleting a Resource leaves the Pool that referenced it intact.
Further restrictions for running executions and external-data cleanup remain separate decisions.

The configuration version covers `spec` and author-editable metadata. Changes to those
values invalidate the prior version for updates; status-only changes do not. The check
does not freeze live permissions or replace authorization at the time of the operation.

After authorization and the required ID/version checks pass, submitting unchanged authored
configuration reports an unchanged result and retains the current configuration version.
Record the operation outcome without invalidating other edits. Stale submissions still
conflict even if their submitted configuration matches the current values.

Equality compares parsed authored values, ignoring presentation-only formatting, comments
and mapping-key order. An omitted field remains distinct from explicitly setting its default,
even if both currently resolve to the same value. Lists follow their field-specific order
rules. Scalar normalization, individual list contracts and version-transition details remain
to be specified; equality does not compare resolved runtime state.

Clients may reread, reconcile and retry within their permissions without mandatory human
approval. Simply substituting a newly read version does not reconcile an older edit.

The expected ID and configuration version accompany the update as separate operation inputs,
not as fields in the reusable object YAML. V0 CLI/editor workflows retain a recoverable local
baseline containing the original configuration and its ID/version, associated with the edit
but separate from its YAML. An interrupted edit can resume with its original conflict checks.
Direct API clients supply the required preconditions themselves. An operation request or local
edit baseline is not an additional core object kind.

When no usable baseline is available, preserve the YAML and require explicit preparation of
a new baseline against the selected target. A user or agent can perform a recovery/import
operation that compares with current configuration and captures its ID/version before a
checked replacement. This cannot reconstruct missing edit history. An existing stale baseline
still requires reconciliation; it must not be silently replaced with the current version.
Exact request fields, baseline storage format/lifecycle and remaining creation/import/export
details remain open.

### Objects, representations and contracts

Each object contract must distinguish what an author requests, what Conduit resolves and
what a runtime observes. These are representations of the existing concepts, not new
top-level concepts:

| Representation | Responsibility |
| --- | --- |
| Authored intent | Explicit settings, references, requested relationships and constraints, with their configuration sources. |
| Resolved configuration | Effective behavioral values and selected references, with defaults/overrides and origins explainable. A Session retains resolved behavior under the rules below. |
| Observed state | Current binding, compatibility, authentication, preparation and execution information, including unknown or unsupported coverage. |
| Operation evidence | Actual attempts, effects, results and unresolved outcomes attributed to the Session/execution and integration. |

The combined inspection view presents authored configuration and system-reported state
together without merging their meanings. Detailed status contents and references to retained
operation evidence remain kind-specific design choices.

Identity, editable names, source paths, configuration change versions, retained-record
revisions, plugin releases and provider protocol versions have different meanings. Their
encoding is open; none may silently stand in for another. A reference, label or capability
request is not permission. Configuring an account does not prove which identity actually ran.

Schemas describe data shape. Resource and Conduit contracts also describe operations,
preconditions, transitions, effects and failure behavior. Runtime compatibility, current
authority and successful execution require their own evidence. Preserve provider-native
contracts where possible instead of flattening every integration into one generic payload.

### Composition follows the relationship

There is no universal deep-merge rule. Each public field and operation must state the
meaning of omission, explicit values, empty collections, nullable values where supported,
defaults and updates. Collection order and duplicate identity must have defined meanings.

| Relationship | Established constraint; details still to settle |
| --- | --- |
| Profile → Session | Explicit overrides operate within required limits. Behavioral defaults are resolved, while access limits remain live. Per-field replacement/addition syntax and default-Conduit precedence are open. |
| Resource → Pool | References/selectors collect independently identified Resources. Overlap does not copy ownership or imply repeated activation. Selector algebra, deduplication and membership changes need exact rules. |
| Session → Conduit → Pool | Filters narrow authorized resource/operation combinations. Combining access paths must not pair one path's operations with another path's resources to create unintended access. |
| Profile activation | Preparation and context-loading instructions operate on permitted Resources. Their ordering, conflicts, optionality and update behavior must be explicit. |
| Intent → observation | Runtime observations report whether requests are realized; they do not silently become authored defaults or fresh authority. |

Permission, membership and activation semantics remain independent even when they use
similar selector notation. A serialization library's merge behavior or filesystem order
must not choose these rules accidentally.

### Validation, effects and evolution

Distinguish syntax/schema checks, reference resolution, semantic/transition checks,
environmental probes and effects. A valid object can still have an unresolved reference,
an unsupported native extension or an expired credential. Error/status reporting must
identify the relevant object and field, explain the limitation, and preserve unknowns.
Exact diagnostic codes, stages and partial-failure policy remain open.

Configuration changes, package preparation, binding establishment and agent execution have
different effects. Their operations must say what they do. A Session export is not evidence
of a running execution, and restoring configuration must preserve the explicit-resume and
no-duplicate-execution rules. Current permissions still govern managed actions after a
preview. Local configuration consistency does not imply atomic rollback of external effects.

Version the meaning of configuration, including defaults, list composition, reference
interpretation and transition behavior. Keep concrete input/result/failure examples with
the contracts so implementations and tools can be checked for agreement. Migration rules
must expose changed behavior; updating a version string alone is not a migration contract.
The exact version progression and compatibility policy remain for review.

Keep the core small and extensible through typed contracts. Reusable objects are the first
composition mechanism; additional template languages, expressions and patch systems need
concrete use cases. The following sections retain the accepted domain behavior that the
language must express. [REVIEW.md](REVIEW.md) focuses the remaining choices on those semantics.

## Profiles

A Profile is a reusable template for any number of independent Sessions. It specifies
runtime/model/account defaults, execution preferences and requirements, optional permission
limits, default Conduit declarations, and activation instructions. A Session starts from
one Profile; v0 defers inheritance and composition between Profiles.

Resolve behavioral defaults at Session creation and retain them until explicit
reconfiguration. Later Profile edits affect new Sessions. Record configuration origins,
overrides and actual execution settings. Permission limits remain live; behavioral
snapshots never freeze access or allow overrides to weaken required limits. Whether
referenced resource content follows source updates is a separate open choice.

### Activation instructions

Activation instructions live in the Profile template. They tell a Session how to prepare
and use Resources made available through its Conduits. Resource-access Conduits select
and filter; they do not own import or context-loading instructions. The Session performs
activation through compatible integrations, within current permissions.

Activation behavior is distinct from the Resource content it consumes. Skills, documents,
prompts, tools and configuration bundles are supplied through Pools; the Profile describes
how to use the permitted selection. Discovering a skill, importing it, exposing a tool
and placing instructions in model context can require different behavior.

Profile/model/runtime defaults can handle conventional imports. Exact representations,
compatibility rules, sequencing, duplicate handling, failure behavior and source updates
remain open. Pi's resource loaders, package support and MCP setup are research inputs.
Model preference alone does not establish native plugin or hook compatibility.

### Account selection

Profile account references select identities without embedding credentials. Model-provider
and resource-service credentials remain distinct. Conduit verifies the account actually
selected; no silent account or API-billing fallback is permitted. Concurrent subscription
accounts must not overwrite one another's login, refresh, logout or private state.

External-agent integrations delegate authentication to their native runtimes. Conduit's
own runtime needs supported provider/account adapters, with validated subscription
eligibility, identity evidence, credential namespaces and refresh ownership. Successful
login alone does not prove concurrent-account correctness.

Coordinate credential reuse separately from Session history. Do not switch accounts by
swapping one global credential directory or blindly cloning rotating credentials. A
Conduit Profile is distinct from a native agent's login profile. The credential/state
layout is an adapter validation task, not a prescribed filesystem schema.

### Execution policy

Native execution is the v0 default: the runtime runs on the host without Conduit's full
OS sandbox. Isolated execution is selectable, and a Profile may require it. Required
isolation cannot be weakened by launch overrides; unavailable required isolation causes
a visible launch failure. Show the configured mode and record actual enforcement coverage.

Native mode preserves host integrations and suits users granting broad local access.
Account/Session state separation addresses configuration collisions without the setup
cost of mandatory isolation. It is not an OS security boundary. Isolated mode covers
the runtime and its child processes; host tooling, keychains, IDE sockets and local
services may require explicit bridges or remain unavailable. This is why both modes exist.

Isolated execution defaults to declared network destinations: required model/login
services, selected connections, and extra destinations authorized through Conduits.
Profile limits and exclusions still apply; local/private services require explicit
access. Network reachability does not authorize every operation at a provider. Exact
sandbox technology and matching rules remain open. Native host access, network policy,
provider authorization and Conduit-managed access have different coverage.

## Sessions

A Session has a durable identity, resolved configuration, private context/history and
independent Conduit bindings. Sharing a Profile or account does not share private state
or establish communication. One Session may span multiple executions with different
runtime/model/account settings; record the settings actually used in each execution.

There is at most one active primary agent per Session. Independently coordinated parallel
workers use separate Sessions. Runtime-managed subagents may remain within their parent's
execution, with ownership and usage attributed where available.

### Runtime and presentation

Conduit implements its own runtime in Rust: Session supervision, model/tool execution,
context construction, provider dispatch, recovery and observations. Provider adapters
handle request/response conversion and supported authentication. External-agent adapters
connect separate runtimes and report their actual lifecycle/access/telemetry capabilities.

The first frontend is a custom terminal inspired by Pi. Presentation is replaceable;
it does not own credentials, model calls or authoritative execution state. Pi terminal
components may be reused, but Pi's agent loop and Pi Durable are research references.
A memory backend or frontend change must not replace Conduit's execution loop.

One frontend controls direct input, cancellation and settings; others observe. Moving
control to another frontend requires no extra confirmation. Disconnecting detaches and
preserves supported ongoing execution. Keep persisted history, current model context,
frontend state and shared Resource content distinct.

### Lifecycle and continuity

| Operation/state | Contract |
| --- | --- |
| Open, paused, closed | Session lifecycle is separate from running/idle execution and frontend attachment. Paused Sessions require explicit resume; closed Sessions can be explicitly reopened. Closing does not complete work or delete resources. |
| Resume | Keep the logical Session and reuse compatible native state and the applicable permitted checkpoint. Record any transformations or gaps. |
| Fresh | Create a new Session without automatically loading prior conversation/checkpoint content. Existing records and files remain retrievable through permitted access. |
| Fork | Create an independent Session from a selected portable checkpoint with an origin reference. The source Session is unchanged. |
| Unexpected interruption | Reconnect only when the integration verifies the execution is still running and supports reconnection. A stopped execution requires explicit resume to start a replacement. Unknown status never launches a duplicate. |

Fresh/fork do not clone or delete workspace files. Changing accounts or frontends does
not by itself create a checkout. Reusing an account must not expose another Session's
private history, memory or provider-specific state.

Portable checkpoints contain optional objective/constraints, decisions, progress/evidence,
remaining work/questions, and artifact/source references. The active agent writes them
at planned handoff, explicit pause, task completion or on demand; Conduit requests and
saves them with known provenance. Failed capture continues with available permitted
context and explicit gaps, including absence of any checkpoint. That is distinct from
choosing fresh. Transcripts are retained when available; complete normalized history is
not guaranteed. Runtime recovery state and compaction summaries are not automatically
portable task checkpoints.

### Observation

Attribute available token usage, tool activity, transcripts and subagent observations
to Session/execution identity. Usage records identify account and reporting source, and
distinguish reported counts, estimates, unknown data and observed zero. Sum only figures
known not to overlap; unresolved parent/subagent totals remain separate. Missing telemetry
is a visible integration gap. Exact cost accounting and comprehensive external coverage
remain staged work.

Frontend progress and optional trace exports are not the only evidence of completed work.
Conduit observes the operations it dispatches; external integrations declare coverage.
Retention follows the Resource contracts below. User hooks and broader interception
remain extension goals rather than assumptions about every runtime.

## Resources

Resources are independently addressable data or capabilities with extensible contracts.
They can be referenced by several Pools without copying content or transferring ownership.
A repository, knowledge collection, skill, document or provider capability is an example,
not a required closed taxonomy. A Resource need not be a copied payload or a separately
authored YAML file; declaration, discovery and granularity remain for review.

Resources carry enough identity and contract information to resolve their source and
supported operations. Labels are optional selection metadata, not identity or authority.
Exact metadata and kind schemas are open. Provider-owned objects retain native contracts;
they need not adopt Conduit's retained-record schema.

### Native resources and integrations

Users bring compatible skills/plugins through Pools and Conduits. User-level native
settings are selected explicitly; an existing setup can be selected once, but the user's
default agent profile is not automatically inherited. Mandatory host/organization policies
still apply. Selecting a package does not grant extra MCP or resource access.

Use ecosystem tooling for compatible skill/plugin installation and updates. Adapters may
invoke it on request and record source/version information and outcomes. Conduit does
not require a new skill format or package manager. Executable native extensions need a
compatible runtime adapter or port; report unsupported hooks. Profile activation instructions
control preparation of the selected resources.

Resource providers retain source content and operations. Linear is the first tracker
integration; the provider owns issue fields, assignments, status and workflow. Conduit
supports work associations without requiring an internal ticket engine or full replica.
Storage, search and graph backends are replaceable; Xerj is a candidate plugin backend.
Authoritative records, observations of external state and search projections are distinct.

### Retained memory and evidence

Knowledge, checkpoints, usage observations and artifact references share small record
metadata while retaining type-specific content and operations. Plugins may introduce
namespaced record kinds. Memory is exposed as Resources through Pools and Conduits.
Work and personal knowledge remain separate unless explicitly shared.

| Concern | Contract |
| --- | --- |
| Identity and ownership | Stable internal record IDs survive names/paths/external-key changes. Each record has one owning context, such as a task, Pool or personal collection. Ownership metadata is distinct from Pool membership and from a Session's working context. |
| References | Support live and revision-pinned references. Missing pinned content is reported unavailable, never silently substituted with a newer revision. References alone grant no access. |
| Revision and concurrency | Each accepted mutable-record update creates an immutable revision. Updates name their base revision; stale writes conflict. Agents may reconcile and retry without mandatory human approval. |
| Provenance and time | Separate original source/author, revision writer and derivation links. Preserve local recording time separately from labelled source times. Unknown provenance remains unknown. |
| Lifecycle | V0 knowledge supports create, read/search, revision-checked update, archive/restore and explicit purge under operation permissions. Archive removes ordinary retrieval, not explicit authorized access. Purge removes retained content/history with a minimal deletion marker. |
| Retention | Records, revisions and captured history under Conduit's control remain until explicit purge; automatic expiry is deferred. Report native/backend limits outside that control. A missing payload is not a promise of recoverability. |
| Artifacts | References are searchable/linkable independently of their files. Removing an artifact record does not delete the referenced file or imply that a snapshot existed. |

Users and agents write knowledge explicitly. Classification is optional, entirely
user-defined and may use multiple dimensions/labels without advance declaration. Optional
definitions are ordinary accessible knowledge records referenced explicitly; matching
names do not bind automatically. Background extraction and classifier interfaces are deferred.

Memory search defaults to all memory permitted by the Session's current Conduits and
limits. Queries may narrow by owning context, kind or classification; narrowing cannot
expand access or implicitly search every repository/service. Results contain bounded
excerpts, record/revision references and permitted provenance. Full content requires an
explicit read. Configurable defaults/caps bound result count and total returned text;
agents can adjust within caps, and responses identify limits reached.

Partial search returns available results with an incomplete status, known coverage gaps
and backend-supplied freshness. Unknown coverage/freshness stays unknown. An unavailable
backend is not “no matches.” Reads, search, caches, transcripts and derived content use
current access checks and reveal only permitted diagnostic/provenance details.

### Sharing and derived content

V0 supports authorized, revocable, direct live read-only shares between owning contexts.
The source retains its ID and owner; recipients explicitly select incoming shares through
their access configuration. Other independent grants remain valid after one share is
withdrawn. Common knowledge can be updated through separately granted update rights.
Pool membership alone does not create a share.

Share management is separate from read/update permission. A manager may grant only rights
it currently holds. Selecting a context covers directly owned records by default;
descendants and incoming shares require explicit inclusion, which presets may provide.

Derived records and ordinary copies require read access to the record and all declared
source inputs. Copies retain those dependencies despite independent identity/content.
Unverifiable required permissions withhold content without deleting it. Missing source
payloads do not block stored derived content when current access remains verifiable;
historical provenance gaps alone do not block otherwise permitted use. Hidden source IDs,
titles, paths, URLs and previews are withheld with a generic notice.

The following preserve agreed extension semantics; they are not v0 delivery requirements:

| Extension | Contract |
| --- | --- |
| Richer shares | Pinned shares expose an exact revision; additional operations are explicit and bounded by current authority. Revision-pinned references remain usable in v0 without granting access. |
| Onward delegation | Explicit permission allows grants within received rights, linked to the parent. Recipients manage their own onward grants, not the incoming grant. Parent restrictions, including loss of delegation permission, constrain dependent grants; other independent grants remain valid. |
| Grant lifetime | Issued grants survive changes to their creator's Session/Profile. Parent-constrained access recovers when that same chain permits it. Explicitly withdrawn grants require authorized restoration/reissue; a new parent does not replace an old parent link implicitly. |
| Independent publication | A separate record may release source-access dependencies only with read/publication authority for each released restricted input and destination creation rights. Retain source/version provenance. The published record has its own audience/lifecycle and does not follow source changes or withdrawal. |
| Published edits | Edits using already-published content need ordinary update rights. New restricted material requires current read/publication authority for those inputs and destination update rights. Explicitly released attribution remains readable. |

### Workspace resources

A task or experiment owns workspaces/worktrees; Sessions attach through permitted resource
access. V0 supports existing folders/checkouts and Conduit-managed Git worktrees, recording
management status. Attachment preserves existing files and Git state and grants no deletion
authority. Concurrent independent writers use separate task-owned worktrees by default;
sequential handoffs can reuse a workspace. Shared editing is opt-in, without automatic
merge/file-lock guarantees. A worktree is not an isolation boundary.

Retain workspaces after Session closure or task completion until explicit cleanup.
Cleanup of attached folders only unregisters them. Remove a managed worktree only when
no execution uses it and no unique local work would be lost; uncertainty preserves it
with an explanation. Exact creation/base/branch/path settings remain open.

## Pools

Pools are named, extensible collections of Resource references. They are the sole route
for supplying Resources to Sessions through Conduit, including baseline configuration,
skills and tool connections. The same Resource can appear in multiple Pools; membership
does not copy content, grant access, change ownership or automatically activate it.

Membership supports explicit references and label selectors. Pool schemas must accommodate
new Resource kinds without a new field per category. A skill collection, application domain
or document collection is a use of the same Pool mechanism. Pool labels and Resource
selection are organizational metadata, not implicit permission grants.

Pool membership selection, Conduit filtering and Profile activation instructions have
different jobs. Pool selectors collect candidates. Conduits expose authorized selections.
Sessions apply Profile instructions to those permitted resources. Multiple memberships do
not imply multiple copies or automatic duplicate activation; resolution remains for review.

When launching against a Pool, its access defaults apply only if no explicit selection
is supplied. Explicit selection replaces those defaults; extending them requires an
explicit add action. Profile limits and exclusions always constrain the result. How this
combines with Profile default Conduits is open; neither form of default can bypass limits.

Exact selector syntax, membership update behavior, duplicate resolution and nesting remain
open. The resource-authoring layout and Pool lifecycle must preserve the distinction
between resource existence, ownership, membership, access and activation.

## Conduits

Conduits establish typed, authorized relationships for resource access through Pools and
communication between Sessions. Profiles reference default declarations; each Session
binds them independently. Shared declarations do not merge Session state, authentication,
permissions or live clients. A Conduit is a logical relationship, not a socket, MCP
connection, frontend attachment or automatic model turn.

For resource access, the Conduit selects and filters available Resources and operations.
Activation instructions belong to the Profile and are applied by the Session. Filters
are narrowing-only: removing a constraint can reveal an existing grant, but a filter
cannot create one. Exact type names, endpoint forms, lifecycle and composition remain open.

### Authority and current access

Permissions name operations per Resource contract, with convenience bundles expanding
into visible grants. Profile limits and explicit exclusions constrain effective access;
missing limits add no authority. Permitted operations require no extra confirmation merely
because permission rules exist. Resource and communication permissions do not imply each other.

Selection supports IDs, owning contexts, metadata predicates and path patterns where the
integration supports them. Matches within established access rules are live. Authorized
Conduit/filter/Profile-limit changes apply to subsequent managed requests and are shown
and recorded; they do not silently change model/account defaults or inject context.
Pool membership rebinding and behavioral activation updates remain separate open questions.

If an isolated execution cannot revoke removed access live, stop it and require explicit
restart under current rules, preserving Session identity. Managed requests use new rules
immediately. Unsupported/restricted bindings remain unavailable while usable capabilities
proceed; an integration must not silently discard a restriction to enable broader access.

Check whole-record content against current recipient access at retrieval and delivery,
including queued operations, retries, replays and checkpoint reuse. Mixed-access content
is withheld as a whole; narrower content requires an authorized extract. Already-delivered
native context cannot be erased by subsequent checks. Details follow Resource ownership,
derivation and provenance rules.

### Protocol and provider compatibility

MCP is the preferred external capability contract. Use standard tools, resources, prompts
and supported extensions where applicable; native adapters address concrete runtime,
account, lifecycle and non-MCP needs. Internal Rust modules need not be MCP servers.

Connection definitions describe reusable endpoint/process/transport/authentication setup.
Conduit's runtime owns its MCP clients; external-agent adapters prepare native registration
from effective selections. Registration, provider authentication and Conduit Session
authority are distinct. Reusing a definition does not promise shared live clients, OAuth
state or backend persistence. Avoid rewriting global settings for each launch and validate
inherited/plugin connections and host configuration behavior.

Preserve provider-native tool names, schemas, descriptions, results and workflows where
possible. A common gateway or separate provider endpoints can provide mediation; no one
combined endpoint is required. Integration capabilities must cover nested calls and actual
dispatch rather than treating discovery lists as enforcement.

The long-term direction is attribution and inspection of outgoing MCP calls, with permission
checks that can allow, limit or block them. V0 need not implement comprehensive upstream
operation enforcement. Conduit-managed resource checks remain core. Each mode reports
observed activity, enforced restrictions and uncovered paths; configuring a connection or
filter alone does not establish agent-wide isolation.

### Outcomes and recovery

For operations Conduit controls, retries reuse an operation ID and return an available
recorded result under current access. Distinguish confirmed effects, failure and unknown
outcomes; timeout alone does not prove failure. Automatically retry only when repetition
is known safe. After an uncertain write, inspect local/provider state before reissuing;
unresolved outcomes stay unknown. Agents can reconcile within their permissions.

External integrations promise only guarantees they support. Operations outside Conduit's
control remain outside this contract. A purged/unavailable operation result does not prove
the operation never ran. These rules support future communication contracts without
selecting a delivery guarantee, broker, ordering scheme or automatic-wake policy.

### Communication research boundary

Communication is an interaction between Sessions through Conduits, with Resources and
Pools supplying authorized shared material. The type-specific contracts still need to
settle addressing, acceptance, results, retention/replay, attention, cancellation and
tracker associations. Keep receipt, execution and completion distinct. Research should
produce concrete flows and failure cases before protocol or implementation selection.
