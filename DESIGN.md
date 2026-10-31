# Conduit design contracts

Updated: 2026-10-31. [SPEC.md](SPEC.md) defines the four-family template/instance model. This document
records accepted behavior within it; [REVIEW.md](REVIEW.md) contains open choices.
The common authoring envelope is settled; detailed fields, transports, storage engines
and implementation layout remain under review.

## Families, templates and instances

The four families are Session, Resource, Pool and Conduit. Each has a template describing
reusable configuration and an instance representing something established. An instance need
not have a running process. Session templates replace the separate Profile concept; a
Conduit instance is called a connection. Neither term introduces another family.

| Family | Template responsibility | Instance responsibility |
| --- | --- | --- |
| Session | Model/agent/account defaults, limits, activation and connection choices. | Durable participant identity, private context, resolved behavior and executions. |
| Resource | Typed create/attach configuration for data or capabilities. | Concrete capability with native backing identity and exactly one owning Pool instance. |
| Pool | Resource-template selections, provisioning defaults and policy. | Creates and exclusively owns its Resource instances. |
| Conduit | Interaction contract, endpoints, filters and supporting-resource requirements. | Opened connection, resolved endpoints, participants and operational state. |

Templates can be reused. Creating instances does not mutate a template into an instance.
Resource instances are created only through their owning Pool, including attachment of
external data and Resources required by connections. Different Pools can instantiate the same
Resource template with separate backing or explicit attachment to the same backing target.
Resource identity/ownership, backing identity, authentication and process lifetime are distinct.

The shared distinction does not impose one lifecycle or mandatory process on every family.
A template registration, instance creation, external provisioning, model execution and later
reconciliation are separate effects. No automatic reconciler or cascading purge is implied.
Instance creation/materialization details, reference extensions and lifecycle operations
remain in [REVIEW.md](REVIEW.md).

### Template changes and resolved configuration

Across Session, Resource, Pool and Conduit, create an instance either from complete instance
YAML or from a template and creation choices. Templates are optional creation helpers. Both
paths produce a complete instance `spec`, authoritative for that instance's configuration,
and follow the same validation and authority rules. Direct creation requires an explicit
unique name. Resource creation always goes through its owning Pool; complete YAML cannot
bypass that operation. This choice does not combine creation, provisioning and execution.
Later changes edit the instance directly through checked updates. Editing or
deleting the creation template does not change an existing instance or create a requirement
to recover the template before operating it. Adding a Resource to a Pool template therefore
does not provision it in existing Pools.

Template use expands its configuration into an instance draft. Callers customize ordinary
instance fields through YAML, CLI or API; v0 has no separately declared template parameters
or parameter-to-field mappings. Edits may be automated and require no interactive step.
Normal validation and default materialization apply to the resulting configuration. Draft
edits prepare a complete configuration submission; they do not introduce a partial-update
registry protocol or a retained override layer.

Template-origin information may be retained optionally. No source-template snapshots,
input/override history or persistent template-version binding is required to operate the
instance. This does not remove stable IDs, configuration-version checks, actual execution
settings or operation-outcome evidence.

Complete configuration can contain explicit references: continuing Resource and authority
dependencies must be represented on the instance, with no implicit live inheritance from
its creation template. Explicit references to templates for future creation, such as a Pool's
Resource plan, remain subject to normal reference checks. Creation choices must satisfy
required constraints; copying configuration never grants authority. Current access checks,
Resource data and matching remain independent of the retained spec. Runtime observations
belong in `status`. Content-version rules, materialization details and family-specific
reconfiguration effects remain in review.

## Configuration language

**YAML is the authoring format; typed objects and explicit semantics define the language.**
The four families and their template/instance forms share identity, reference, composition
and behavioral contracts across every entry point.
CLI/API/frontends must interpret the same objects consistently. Future generators can
emit this model; they cannot bypass its validation or authority rules.

Each object uses its family as `kind` and a top-level `form` beside it to identify a template
or instance. Form is a structural distinction, not an observed lifecycle state: instantiation
creates a separate object rather than changing the template's form. Family and form jointly
determine how to interpret `spec`. The common authoring envelope is:

| Field | Meaning |
| --- | --- |
| `apiVersion` | The versioned contract used to interpret the object. |
| `kind` | The family: `Session`, `Resource`, `Pool` or `Conduit`. The explicit form further determines the applicable schema and operations. |
| `form` | The template/instance distinction, at the top level alongside `kind`. |
| `metadata` | Author-editable naming/labels and system-owned identity/version information, with ownership defined per field. |
| `spec` | Configuration defined by the family and form's contract. |

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

Every registered template and instance has a system-assigned stable ID independent
of its name. Renaming an object preserves that ID. Deleting it and creating another object
with the same name produces a new ID; name reuse does not make them the same object.

Readable configuration-object references explicitly include `kind`, `form` and `name`, such
as `{kind: Session, form: template, name: reviewer}`. Kind/name locates the object; form states
the expected template/instance distinction and must match the target. A wrong-form target is
rejected. Typed fields such as `templateRef` still require explicit form, and their contracts
must agree with it. This makes the reference understandable independently of its containing field.
Conduit records the stable ID selected when the reference resolves. Authors need not copy
generated IDs into ordinary references; readable lookup and resolved identity remain distinct.

V0 resolves references within one local registry. Names are unique per family `kind` across
both forms: a Session template and Session instance cannot both be named `reviewer`; a Resource
may use that name because it belongs to another family. Pool ownership does not partition
this registry naming boundary. Additional Pool-local display names and optional direct-ID
references remain open. The registry catalogs definitions and IDs while backing data may be
local or remote. Registry presence grants no access and need not name every native item globally.

Templates name reusable concepts; instance names distinguish specific creations. Template-based
creation defaults to the template's name plus a generated unique suffix, such as `reviewer-7f3a`.
Callers may supply an explicit unique name instead. Direct creation without a template requires
an explicit unique name. Both paths obey per-family uniqueness across forms. Exact suffix
format, name constraints and generated-collision handling remain open. Generated names remain
separate from stable system IDs, and a specific instance name does not imply automatic expiry.

V0 kind/name lookups use only current object names; registry object-name aliases are deferred.
After a rename, the old name becomes available for reuse. A fresh lookup of that old name
finds no object unless the name has been reused, in which case it can resolve to a different ID.
This does not redirect established bindings, which retain their resolved identity as below.

A rename uses the normal checked configuration update, changing `metadata.name` while
preserving the object's ID. The original ID/version identifies the target; the new name
must be available within the applicable naming boundary or the update conflicts.

Other registry objects retain their authored kind/form/name references after a rename. Report
known mismatches between a written name and its resolved target, and offer explicit checked
reference updates. The rename does not rewrite source YAML files.

Configuration-only export preserves authored reference names by default and reports known
mismatches. An explicit option refreshes names from known, unambiguous resolved IDs in the
exported copy only; it does not update registry objects or rebind anything. An old exported
name can still fail or identify a different object on a fresh import.

An established binding created from an explicit kind/form/name reference retains its resolved
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
conflict if that qualified name already exists. Updating a missing target reports a conflict
instead of creating it. Neither operation silently switches intent based on name matches;
exact commands and batch handling remain open. Creating Resource instances must go through
a Pool operation; a generic registry write cannot bypass the owner requirement. Registering
a template is distinct from provisioning instances or launching execution.

A v0 update replaces the complete authored configuration, including `spec` and editable
metadata. Omitting an optional field removes its explicit setting; the field's omission,
default and validation rules apply instead of retaining its previous value. Instance creation
and checked replacement updates materialize declared schema defaults into the saved `spec`;
configuration-only exports include those explicit values. Template YAML preserves authored
omissions. Fields without declared defaults retain their required/optional semantics; this
does not invent values or copy runtime observations into configuration. The object's
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
objects. Deleting a Resource registration does not delete its owning Pool. Pool ownership
is stronger than an ordinary incoming reference: owner retirement, transfer and incomplete
provisioning need a lifecycle contract before a Pool deletion path can be finalized.

Deleting a Resource registration preserves its backing files, repositories and service data,
including data created through Conduit. Cleanup or purge requires separate explicit operations
governed by their own permissions. Session deletion also requires confirmed execution end
under the [lifecycle contract](#lifecycle-and-continuity). Further effects of deleting other
object forms, client disconnection and transient helper-process cleanup remain runtime decisions.
An unowned Resource is an ownership inconsistency to resolve, not automatic purge authority.
The existing retain-until-explicit-purge rule remains; automatic orphan collection is not selected.

The configuration version covers stored `spec` and author-editable metadata. Changes to those
values invalidate the prior version for updates; status-only changes do not. The check
does not freeze live permissions or replace authorization at the time of the operation.

After authorization and the required ID/version checks pass, submitting unchanged
configuration reports an unchanged result and retains the current configuration version.
Record the operation outcome without invalidating other edits. Stale submissions still
conflict even if their submitted configuration matches the current values.

Equality ignores presentation-only formatting, comments and mapping-key order. For instances,
compare the candidate configuration after declared defaults are materialized with the stored
configuration. Omitting a defaulted field and explicitly supplying the same value are equivalent;
if no stored configuration value changes, retain the version. For templates, compare parsed
authored values: omission remains distinct from explicitly setting a default. Lists follow
their field-specific order rules. Scalar normalization, individual list contracts and
version-transition details remain open; equality does not compare runtime state, current
Resource contents or live authority. Source YAML files are not silently rewritten.

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
| Authored intent | Explicit settings, references, requested relationships and constraints. Creation choices can customize a template without becoming a retained override layer. |
| Instance configuration and resolution | A complete instance spec contains its settings and explicit dependencies. Resolving those dependencies establishes current bindings and availability; template history is unnecessary. |
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

In draft editing, explicitly assigning a field replaces its entire value, including an object
or list. Assigning `[c]` to a field containing `[a, b]` produces `[c]`. Editing a nested child
leaves its siblings intact; assigning its parent replaces that entire object. Retaining,
adding or removing entries requires deliberate edits, with no implicit object merge or list
append. Normal validation and default materialization still apply to the resulting draft.
These are client editing semantics; the registry still accepts complete checked replacements.

Field contracts separately define omission, empty collections, nullable values, defaults,
collection order and duplicate identity. Relationship operations such as combining authorized
access paths retain their own semantics; field assignment does not redefine those operations.

| Relationship | Established constraint; details still to settle |
| --- | --- |
| Session template → Session | Expand to an instance draft and edit ordinary fields within required constraints. Assignments replace field values. Continuing authority is explicit; default-Conduit precedence remains open. |
| Resource template → Pool → Resource instance | Only a Pool creates/attaches its instances. Template reuse across Pools creates distinct instances. Selectors cannot assign a foreign instance to another owner; target domains and live behavior remain open. |
| Session → Conduit → Pool | Filters narrow authorized resource/operation combinations. Combining access paths must not pair one path's operations with another path's resources to create unintended access. |
| Session activation | Instructions materialized in the Session spec operate on permitted Resources. Their ordering, conflicts, optionality and update behavior must be explicit. |
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

Typed configuration rejects unknown fields for both templates and instances, except within
maps whose schemas explicitly allow arbitrary keys. For example, `metadata.lables` is an
unknown field; custom keys inside the declared `metadata.labels` map are permitted. Keys and
values still obey the map's contract; openness is limited to the declared location. Plugin
contracts can define their own fields and open maps under the same rule. Unknown fields are
not silently discarded or retained outside those locations.

Create and replacement writes require the applicable configuration schemas, including required
plugin schemas. If a required schema is unavailable, reject the write: preserve source YAML
and any existing registered object rather than register an unvalidated replacement or new
object. This supports field validation and declared-default materialization.

When schemas and required registry checks pass, accept configuration even when its plugin
runtime is missing or a configured external service is offline. Report known readiness gaps
or unknown readiness, without claiming successful activation. Operations requiring unavailable
dependencies remain blocked. Registration remains separate from activation; validation,
authority and Pool-ownership requirements still apply. Schema availability establishes the
ability to validate configuration, not the availability of its operational dependencies.

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

## Sessions

### Templates

A Session template is reusable configuration for any number of independent Sessions. It specifies
runtime/model/account defaults, execution preferences and requirements, optional permission
limits, default connection choices, and activation instructions. A Session can be created
directly from complete instance YAML or from one Session template; v0 defers inheritance
and composition between Session templates.

Both creation paths produce the Session's complete spec. The Session operates from that spec
and explicit live dependencies; retain actual execution settings.
Permission limits remain current. Whether referenced resource content follows source updates
is a separate open choice.

### Activation instructions

Activation instructions are supplied directly in the Session spec or materialized from its
creation template. They tell it how to prepare and use Resources made available through its Conduits.
Resource-access Conduits select
and filter; they do not own import or context-loading instructions. The Session performs
activation through compatible integrations, within current permissions.

Activation behavior is distinct from the Resource content it consumes. Skills, documents,
prompts, tools and configuration bundles are supplied through Pools; the Session spec describes
how to use the permitted selection. Discovering a skill, importing it, exposing a tool
and placing instructions in model context can require different behavior.

Defaults for the Session template, model and runtime can handle conventional imports. Exact representations,
compatibility rules, sequencing, duplicate handling, failure behavior and source updates
remain open. Pi's resource loaders, package support and MCP setup are research inputs.
Model preference alone does not establish native plugin or hook compatibility.

### Account selection

Session account references select identities without embedding credentials. Model-provider
and resource-service credentials remain distinct. Conduit verifies the account actually
selected; no silent account or API-billing fallback is permitted. Concurrent subscription
accounts must not overwrite one another's login, refresh, logout or private state.

External-agent integrations delegate authentication to their native runtimes. Conduit's
own runtime needs supported provider/account adapters, with validated subscription
eligibility, identity evidence, credential namespaces and refresh ownership. Successful
login alone does not prove concurrent-account correctness.

Coordinate credential reuse separately from Session history. Do not switch accounts by
swapping one global credential directory or blindly cloning rotating credentials. A
Session template is distinct from a native agent's login profile. The credential/state
layout is an adapter validation task, not a prescribed filesystem schema.

### Execution policy

Native execution is the v0 default: the runtime runs on the host without Conduit's full
OS sandbox. Isolated execution is selectable; a template requirement is materialized in the
Session spec, alongside explicit authority dependencies. Required isolation cannot be weakened
by launch overrides; unavailable required isolation causes
a visible launch failure. Show the configured mode and record actual enforcement coverage.

Native mode preserves host integrations and suits users granting broad local access.
Account/Session state separation addresses configuration collisions without the setup
cost of mandatory isolation. It is not an OS security boundary. Isolated mode covers
the runtime and its child processes; host tooling, keychains, IDE sockets and local
services may require explicit bridges or remain unavailable. This is why both modes exist.

Isolated execution defaults to declared network destinations: required model/login
services, selected connections, and extra destinations authorized through Conduits.
Session limits and exclusions still apply; local/private services require explicit
access. Network reachability does not authorize every operation at a provider. Exact
sandbox technology and matching rules remain open. Native host access, network policy,
provider authorization and Conduit-managed access have different coverage.

### Instances

A Session has a durable identity, resolved configuration, private context/history and
independent connection attachments. Sharing a Session template or account does not share private state
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
| Delete Session | Require its agent executions to be confirmed ended, including native agents that are idle but still alive. Reject deletion while an execution is live or its status is unknown, preserving the Session so the caller can explicitly stop or reconcile and retry. Deletion does not initiate shutdown; closing a conversation and purging data are separate operations. |

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

A Resource template describes how to create or attach data/capabilities through a typed
contract. It is reusable configuration, not a deployed capability, and need not belong to a
Pool merely to exist. Creating or attaching a Resource instance is exclusively a Pool operation.
Every instance has exactly one owning Pool; one instance cannot be a member of several Pools.

The same template may create distinct instances in multiple Pools. A contract supports new
backing data, attachment to existing backing data, or both explicitly. Different instances
may therefore reach the same native target without sharing registry ownership. Creating a
new instance does not prove that files, credentials, quotas or remote data were copied.

A repository, knowledge collection, skill, document or provider capability is an example,
not a closed taxonomy. One Resource may expose many native files, tools, tickets or records;
each need not become a separately authored registry object. Labels aid selection, never
identity, authority or implicit ownership transfer. Exact field names and granularity remain open.

Resource instances record enough identity/contract information to resolve their source and
supported operations. Provider-owned data retains native contracts; attaching it does not
make Conduit the owner of the provider account or authorize deleting that backing data.

### Native resources and integrations

Users bring compatible skills/plugins through Pools and Conduits. User-level native
settings are selected explicitly; an existing setup can be selected once, but the user's
default agent profile is not automatically inherited. Mandatory host/organization policies
still apply. Selecting a package does not grant extra MCP or resource access.

Use ecosystem tooling for compatible skill/plugin installation and updates. Adapters may
invoke it on request and record source/version information and outcomes. Conduit does
not require a new skill format or package manager. Executable native extensions need a
compatible runtime adapter or port; report unsupported hooks. Session activation instructions
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
| Identity and ownership | Stable internal record IDs survive names/paths/external-key changes. Each record has one owning context, such as a task, Pool or personal collection. These content contexts do not create additional owners of the containing Resource instance. Mapping them within Pool-owned memory needs a concrete schema; a Session's working context remains separate. |
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
Pool ownership alone does not create a share. Sharing a record grants access to source
content; it does not assign the containing Resource instance to a second Pool.

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
| Grant lifetime | Issued grants survive changes to their creator's Session or its template. Parent-constrained access recovers when that same chain permits it. Explicitly withdrawn grants require authorized restoration/reissue; a new parent does not replace an old parent link implicitly. |
| Independent publication | A separate record may release source-access dependencies only with read/publication authority for each released restricted input and destination creation rights. Retain source/version provenance. The published record has its own audience/lifecycle and does not follow source changes or withdrawal. |
| Published edits | Edits using already-published content need ordinary update rights. New restricted material requires current read/publication authority for those inputs and destination update rights. Explicitly released attribution remains readable. |

### Workspace resources

A workspace Resource instance belongs to its Pool and is associated with a task or
experiment; Sessions attach through permitted access. V0 supports existing folders/checkouts
and Conduit-managed Git worktrees, recording management status. Attachment preserves existing files and Git state and grants no deletion
authority. Concurrent independent writers use separate task-associated worktrees by default;
sequential handoffs can reuse a workspace. Shared editing is opt-in, without automatic
merge/file-lock guarantees. A worktree is not an isolation boundary.

Retain workspaces after Session closure or task completion until explicit cleanup.
Cleanup of attached folders only unregisters them. Remove a managed worktree only when
no execution uses it and no unique local work would be lost; uncertainty preserves it
with an explanation. Exact creation/base/branch/path settings remain open.

## Pools

A Pool template describes resource provisioning selections, defaults and policies. A Pool
instance creates/attaches and exclusively owns its Resource instances. Pools remain the
resource-supply boundary for Sessions and connections, including skills, tools, memory and
connection support such as conversation storage. A Pool need not run a process.

Creating a Pool registers its identity and configuration, including any declared Resource
plan. Registration alone does not execute that plan or create its planned Resource instances.
Provisioning must be requested explicitly to create or attach the planned Resources through
their owning Pool. A frontend may offer registration and provisioning as one explicit action,
while preserving separate outcomes and reporting readiness. A successful registration does
not imply successful provisioning. Exact operation syntax, repeated-run behavior and partial
provisioning rules remain open; no background reconciler is implied.

Resource templates can be selected by explicit references and labels where supported.
Selection among an existing Pool's owned instances is another operation. Exact selector
syntax, target domains and refresh rules remain open; neither labels nor references can
silently adopt an instance from another Pool. Reusing templates creates new owned instances,
not overlapping ownership. Pool schemas remain open to new Resource contracts.

Connection participants can access Resources in several Pools through authorized paths to
the owners. Sharing backing data between separate instances does not transfer ownership or
establish access. Pool ownership, current access, Session activation and native backing-data
management have distinct meanings.

Creating a Resource for a connection must identify its owning Pool; the connection requests
provisioning through that Pool. How provisioning retries partial work or resolves dynamic
support requirements remains open. A template registration alone does not execute a plan.

When launching against a Pool, its access defaults apply only if no explicit selection
is supplied. Explicit selection replaces those defaults; extending them requires an
explicit add action. Session limits and exclusions always constrain the result.
Combination with connection choices materialized in the Session spec remains open.

Pool retirement, possible ownership transfer, active-use handling and incomplete provisioning
need explicit contracts. Deleting a grouping cannot silently orphan deployed Resources under
this model. The response to an orphan does not imply deleting attached external data or
purging retained history. Ownership cleanup and backing-data cleanup remain separate.

## Conduits

A Conduit template describes a reusable typed interaction. Opening from that template or
complete instance configuration creates an authorized **connection**, a Conduit instance with
resolved endpoints, participant attachments and operational state.
Conduits are the extensible interaction layer across the four families. Pool access and
Session communication are examples, not a closed list of endpoint roles or protocols.

A connection may use a shared runtime, retained history and per-participant clients. Every
supporting Resource is created/attached through, and exclusively owned by, a Pool. Connection
bookkeeping such as routing state and cursors need not each be registered Resources, but durable
shared content cannot use this distinction to bypass Pool ownership or current access rules.

Session-owned participant attachments remain a useful direction for ordinary Pool access;
shared connection state can have a different authority and lifetime. Template reuse or joining
a connection does not merge private Session context, credentials or access. Connection identity
is separate from process, transport and backing-Resource identity. The exact authoritative
records, instance lifetime, supervision and composition remain in [REVIEW.md](REVIEW.md).

### Explicit joining and instantiation

Connection establishment supports both intents explicitly:

- **Join an existing instance:** establish participation in a specified existing relationship.
- **Open a new connection:** create a relationship from a Conduit template or complete instance
  configuration, subject to the contract's endpoints, authority and provisioning requirements.

Reusing a template name does not implicitly choose between these intents. For example,
two Sessions can join the same discussion, or create separate discussions from the same
template. This choice does not by itself determine whether clients, credentials, backing
data or processes are shared. Joining preserves participant-specific access/authentication;
creating another relationship does not imply copying its underlying Resources.

Exact selection syntax, instance addressing/identity, repeat-request behavior and explicit
connection-reconfiguration operations remain open. Both modes belong within
the existing Conduit concept, rather than requiring another top-level kind.

### Resource selection and activation

For resource access, the Conduit selects and filters available Resources and operations.
Activation instructions belong to the Session spec and are applied by the Session. Filters
are narrowing-only: removing a constraint can reveal an existing grant, but a filter
cannot create one. Exact type names, endpoint forms, lifecycle and composition remain open.
Session activation prepares the participant. Pool operations create/attach supporting
Resources; connection supervision runs any shared interaction service. Their ordering and
failure behavior are unresolved, but a connection cannot provision an unowned Resource or
assume the Session's loading instructions merely because it requested storage.

### Authority and current access

Permissions name operations per Resource contract, with convenience bundles expanding
into visible grants. Session limits and explicit exclusions constrain effective access;
missing limits add no authority. Permitted operations require no extra confirmation merely
because permission rules exist. Resource and communication permissions do not imply each other.

Selection supports IDs, owning contexts, metadata predicates and path patterns where the
integration supports them. Matches within established access rules are live. Authorized
connection, filter and Session-limit changes apply to subsequent managed requests and are
shown and recorded; they do not silently change model/account defaults or inject context.
Continuing authority dependencies are explicit on the instance. Creation-template edits do
not update those limits. Applying edits to instance provisioning/selection and activation
settings during active work remains a family-specific lifecycle question.

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

Resource templates for MCP services describe reusable endpoint/process/transport/authentication
setup. Pool-owned instances and connection attachments realize that setup under current access.
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
