# Conduit design contracts

Updated: 2026-10-31. Current working design on `research/conduits-as-graphs`; implementation
has not started. [SPEC.md](SPEC.md) gives the model and [REVIEW.md](REVIEW.md) groups open choices.
The earlier design remains on `main` at `c013f0a`. Research supplies evidence and illustrative
syntax; it does not silently settle the remaining contracts.

Prefer minimal bookkeeping: retain authoritative configuration, identity, authority and necessary
operation evidence; derive views rather than maintain parallel editable descriptions. Shared
contracts need not impose the same lifecycle, process model or implementation on every type.

**Primary access model:** establish capabilities and service authentication through a Pool once,
then let authorized Agents and Interfaces use them through individually restricted Conduits.
Sharing service access is the normal path, not an optional optimization of per-Agent logins.

## Families, templates and instances

The five families are **Interface, Agent, Resource, Pool and Conduit**, each with template and
instance forms. Templates are reusable definitions; one can establish many independent instances.
An instance need not run a dedicated process. Agent replaces the former Session name; Profile
was the earlier name for its template. Plugin nodes, executions and attachment records are
supporting concepts, not additional families.

| Family | Template defines | Instance represents |
| --- | --- | --- |
| Interface | User-facing authoring, observation, creation and connection capabilities. | An interaction endpoint presented through a CLI, terminal or other frontend. |
| Agent | Runtime/account/model configuration, activation, connections and supported controls. | An autonomous participant with private context, concrete configuration and executions. |
| Resource | Typed creation/attachment, ongoing capability and supported controls. | One unitary capability with exactly one owning Pool. |
| Pool | Resource composition, shared service-access setup and supported management controls. | Owns its Resource instances and organizes their authentication/access lifecycle. |
| Conduit | Endpoints, interaction/filtering behavior, setup and supported controls. | A connection with concrete participants, filtering and connection state. |

Resource creation/attachment always goes through its owning Pool, including supporting Resources
for Conduits. Templates are reusable across Pools; each resulting Resource instance has one owner.
Backing data, credentials and process lifetime are distinct from registry identity/ownership.

### Interfaces and the user workflow

The default Interface has the broadest local management scope and is the primary user entry point.
It can discover templates and instantiated objects, author/edit Resource and Agent templates,
compose Pools from Resource templates, instantiate Pools and Agents, and observe their state.
Running targets are manipulated through Conduits; visibility is not a direct-edit exception.

The user authors templates, adds Resource-template references to a Pool configuration, establishes
the Pool, its Resources and shared service authentication, then starts Agents with Conduits to
those Pools. New attachments use the established capability without repeating provider login.
The Interface can
open further Conduits to inspect supported controls and request changes. Agents use that same
interaction contract on other targets. A frontend presents it rather than independently performing
provider effects. The Interface identifies the endpoint; the acting human identity remains distinct.

CLI, terminal UI and test clients should exercise the same contract. The initial Interface's broad
scope need not be hard-coded into every future Interface. Its bootstrap, management endpoint,
frontend attachment and identity details remain open within the five-family model.

### Establishment contracts and instance state

**Accepted:** templates contain the complete instructions needed to establish an instance and
specify which instance configuration can subsequently change and within what parameters.
This is part of the protocol, not merely documentation or a frontend hint. Complete means
resolvable typed configuration, graph/runtime declarations and explicit dependencies; it does
not require embedding plugin code, credentials or all referenced Resource data in one YAML file.

A template combines creation/attachment intent, initial configuration, an inline or referenced
setup graph, typed ongoing runtime behavior and its configuration-change contract. Plugins supply
typed implementations. Starting uses that complete recipe; a caller does not assemble another
setup-operation document. Ordinary creation choices edit typed fields before instantiation.

Named handlers implement establishment, supported invocations and permitted changes using a typed
plugin operation or a composed graph. A handler is a procedure, not a sixth family or a mandatory
process. Internal setup operations are not automatically callable by consumers. Instances own
ongoing activity produced by handlers; exact declaration and invocation schemas remain open.

The instance retains the resolved graph, runtime configuration, applicable controls and explicit
dependencies independently of later source-template edits/deletion. Source history or stored
input/override archives are not required. Current access and external data remain live. Changing
a source template does not silently change an existing instance's behavior or control boundaries.

Keep template intent, concrete instance configuration, requested changes and observed state
distinguishable. A value accepted for application is not proof that the runtime uses it. The exact
storage split among spec, status and operation evidence remains open; it must not create conflicting
sources of truth. Mutable settings do not make every field editable, and reusable image-like
templates do not imply that all instance configuration is permanently fixed.

Registration has no provisioning/model-execution effects. An explicit action may combine creation
and start, reporting their outcomes separately. Resources require a Pool owner. Direct creation
from a complete definition remains possible; exports/imports must report missing dependencies
or establishment intent rather than inventing it. Current schema/default rules still apply.

### Template-defined configuration changes

The template must make the allowed configuration changes and their parameters inspectable.
Type/plugin contracts provide actual supported behavior; templates configure the exposed surface.
A declaration cannot make an unsupported runtime operation work or grant caller authority.

The following is the **recommended contract shape for review**, not finalized YAML or a mandatory
field list. Shared versioned type definitions can avoid copying schemas into every template, but
the resolved template/instance must make its effective contract visible.

| Concern | Meaning to expose |
| --- | --- |
| Configurable surface | Which settings or named controls are exposed, their types, and which configuration they affect. |
| Value constraints | Allowed values, ranges, units, shapes and related-field conditions. Static limits differ from runtime-discovered eligible choices. |
| Transition conditions | Whether this change is valid from the current state, including readiness, active work and other prerequisites. Valid endpoints alone do not prove a valid transition. |
| Application behavior | The supported operation/handler and whether it applies live, waits, needs a restart, or requires a new instance. Concrete categories/defaults remain to be selected. |
| Results and observations | What proves application, how progress/failure/unknown outcomes appear, and what clients can inspect afterward. |
| Recovery and retention | Supported retry/cancel/reconciliation and what applied settings persist across restarts. No universal queue or rollback policy is implied. |

A template may constrain the values and controls it exposes within its implementation's support.
We recommend constraint intersection rather than last-writer-wins overlays: type support, template
limits, current authority and Conduit filters all apply. Exact authoring shorthand/defaults,
cross-field expressions, handler composition and control-contract evolution remain open.

A setting's value and the rule governing its permitted changes are distinct. Changing a value
must not silently expand the control's allowed range or caller privileges. Whether an instance's
control declaration, graph or runtime implementation can itself change requires an explicit
management/evolution contract; it is not implied by exposing a setting. This remains under review.

### Conduits and configurable endpoints

A running target's effective change contract gives Conduits something concrete to expose/filter:
operations, typed inputs and permitted observations. Interfaces and Agents use the same path:

```text
discover permitted controls → submit through a Conduit → target validates current conditions
  → apply using the supported implementation → report outcome and observed state
```

A Conduit can narrow allowed operations and parameters; it cannot manufacture unsupported controls,
relax target constraints or grant authority by naming a Resource. Publication/discovery, validation
and actual dispatch must agree. Recheck current authority and relevant conditions at application;
previous discovery or enqueueing is not a reservation of access. Native paths outside actual
mediation retain their reported coverage limitations.

Input restriction is not silent rewriting of a requested value. A value outside the allowed
contract is rejected/reported; transformations, if supported, need their own explicit semantics.
No generic recursive dependency is required: a control request is carried over an established
Conduit, not a newly registered Conduit for each request. Controlling a Conduit itself uses an
appropriate management connection; self-modification and bootstrap details remain open.

Configuration changes remain distinct from Resource-content writes, lifecycle actions and
connection membership, even if they share the invocation protocol. Adding a document does not
edit a storage Resource's configuration. Observing state does not start model work. Attaching
another Conduit does not silently rewrite an Agent's template or broaden a control's constraints.

### Lifecycle protocol under review

**Accepted graph boundary:** plugins declare typed inputs/outputs; graph nodes bind them. Output
bindings imply dependencies; explicit prerequisites cover ordering without data. Per-output
publication timing and validity/lifetime are declared separately. A graph runs each instance start;
plugins may verify already-satisfied work. This does not authorize repeating uncertain effects.

Templates declare ongoing runtime behavior, using type defaults; graphs supply prepared inputs.
A graph may request instance-managed runtime startup during setup, such as starting a database
before migrations. Instances own ongoing processes/availability from startup; setup plugins do
not remain permanent supervisors. Graph completion and ongoing readiness are separate facts.

Required setup failure normally retains an already-started managed runtime for inspection and
explicit retry/stop. Retry of unchanged setup continues from still-valid recorded progress under
current authority and plugin recovery rules. Effects of permitted configuration changes on retained
progress need explicit invalidation/recovery semantics; do not reuse incompatible step results.

Required connection failures block initial preparation, optional ones report and continue, and
successful partial attachments are retained for retry. After successful setup, consumers normally
continue independent work when a dependency is unavailable. The selected MCP example uses bounded
reconnect with a manual option; this is not a universal operation retry policy or permission to
replay writes/start new model work. Exact graphs, waits, cancellation, retry bounds, live-change
handlers and recovery compatibility remain in [REVIEW.md](REVIEW.md).

## Configuration language

**YAML is the authoring format; typed objects and explicit semantics define the language.**
The five families and their template/instance forms share identity, reference, composition
and behavioral contracts across every entry point.
CLI/API/frontends must interpret the same objects consistently. Future generators can
emit this model; they cannot bypass its validation or authority rules.

Each object uses its family as `kind` and a top-level `form` beside it to identify a template
or instance. Form is a structural distinction, not an observed lifecycle state: instantiation
creates a separate object rather than changing the template's form. Family and form jointly
determine how to interpret `spec`. The common authoring envelope is:

| Field | Meaning |
| --- | --- |
| `apiVersion` | The common object contract version. A Resource's selected type contract has its own separate schema version. |
| `kind` | The family: `Interface`, `Agent`, `Resource`, `Pool` or `Conduit`. The explicit form further determines the applicable schema and operations. |
| `form` | The template/instance distinction, at the top level alongside `kind`. |
| `metadata` | Author-editable naming/labels and system-owned identity/version information, with ownership defined per field. |
| `spec` | Configuration defined by the family and form's contract. |

Metadata ownership is defined per field: authors control naming and labels within their
permissions, while Conduit assigns and maintains identity/version information. Placing a
field in `metadata` does not make it editable. Identity and configuration-version checks
govern updates as described below; exact field names and operation schemas remain open.

Inspection returns the object with a system-owned `status` section alongside its authored
`spec`. Status reports resolution and observed state appropriate to the kind, such as an
Agent's bindings, preparation and execution. Normal authored YAML omits `status`; users
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
as `{kind: Agent, form: template, name: reviewer}`. Kind/name locates the object; form states
the expected template/instance distinction and must match the target. A wrong-form target is
rejected. Typed fields such as `templateRef` still require explicit form, and their contracts
must agree with it. This makes the reference understandable independently of its containing field.
Conduit records the stable ID selected when the reference resolves. Authors need not copy
generated IDs into ordinary references; readable lookup and resolved identity remain distinct.

V0 resolves references within one local registry. Names are unique per family `kind` across
both forms: an Agent template and Agent instance cannot both be named `reviewer`; a Resource
may use that name because it belongs to another family. Pool ownership does not partition
this registry naming boundary. Additional Pool-local display names and optional direct-ID
references remain open. The registry catalogs definitions and IDs while backing data may be
local or remote. Registry presence grants no access and need not name every native item globally.

Templates name reusable concepts; instance names distinguish specific creations. Template-based
creation defaults to the template's name plus a generated unique suffix, such as `reviewer-7f3a`.
Callers may supply an explicit unique name instead. The one-off/import path under review retains
the explicit-unique-name requirement. All paths obey per-family uniqueness across forms. Exact suffix
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

The checked full-replacement workflow below governs **template/authoring registry edits**.
It does not authorize replacing a running instance's spec through a generic registry write.
Instance configuration changes use the template-declared control contract through a Conduit.
Their precise concurrency/version and partial-change semantics remain to be settled; checked
identity and safe outcomes remain necessary. Registry deletion rules below retain their own scope.

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

A v0 template update replaces the complete authored configuration, including `spec` and editable
metadata. Omitting an optional field removes its explicit setting; the field's omission,
default and validation rules apply instead of retaining its previous value. Instance creation
materializes declared schema defaults into its saved configuration;
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
governed by their own permissions. Agent deletion also requires confirmed execution end
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

Equality ignores presentation-only formatting, comments and mapping-key order. For instance creation,
omitting a defaulted field and explicitly supplying the same value produce equivalent resolved
configuration. Equality/version behavior for subsequent control operations remains under review.
For template updates, compare parsed authored values: omission remains distinct from explicitly setting a default. Lists follow
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
| Operation evidence | Actual attempts, effects, results and unresolved outcomes attributed to the Agent/execution and integration. |

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
These are draft-editing semantics; template writes use complete checked replacements. They do
not select the shape of runtime configuration-change requests.

Field contracts separately define omission, empty collections, nullable values, defaults,
collection order and duplicate identity. Relationship operations such as combining authorized
access paths retain their own semantics; field assignment does not redefine those operations.

| Relationship | Established constraint; details still to settle |
| --- | --- |
| Agent template → Agent | Interpret typed establishment configuration and retain ongoing instance settings. Customize ordinary typed fields with replacement semantics. Continuing authority is explicit; detailed transition/dependency semantics remain under review. |
| Resource template → Pool → Resource instance | Only a Pool creates/attaches its instances. Template reuse across Pools creates distinct instances. Selectors cannot assign a foreign instance to another owner; target domains and live behavior remain open. |
| Agent → Conduit → Pool | Filters narrow authorized resource/operation combinations. Combining access paths must not pair one path's operations with another path's resources to create unintended access. |
| Agent activation | Instructions materialized in the Agent spec operate on permitted Resources. Their ordering, conflicts, optionality and update behavior must be explicit. |
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
different effects. Their operations must say what they do. An Agent export is not evidence
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

## Agents

### Templates

An Agent template is reusable configuration for any number of independent Agents. It specifies
runtime/model/account defaults, execution preferences and requirements, optional permission
limits, default connection choices, activation instructions and the exposed configuration controls
with their constraints. Establishment uses one Agent
template's configuration; one-off/import syntax remains under review. V0 defers inheritance
and composition between Agent templates.

Establishment produces the Agent's complete spec. The Agent operates from that spec
and explicit live dependencies; retain actual execution settings.
Permission limits remain current. Whether referenced resource content follows source updates
is a separate open choice.

### Activation instructions

Activation instructions are supplied directly in the Agent spec or materialized from its
creation template. They tell it how to prepare and use Resources made available through its Conduits.
Resource-access Conduits select
and filter; they do not own import or context-loading instructions. The Agent performs
activation through compatible integrations, within current permissions.

Activation behavior is distinct from the Resource content it consumes. Skills, documents,
prompts, tools and configuration bundles are supplied through Pools; the Agent spec describes
how to use the permitted selection. Discovering a skill, importing it, exposing a tool
and placing instructions in model context can require different behavior.

Defaults for the Agent template, model and runtime can handle conventional imports. Exact representations,
compatibility rules, sequencing, duplicate handling, failure behavior and source updates
remain open. Pi's resource loaders, package support and MCP setup are research inputs.
Model preference alone does not establish native plugin or hook compatibility.

### Account selection

Agent account references select identities without embedding credentials. Model-provider
and resource-service credentials remain distinct. Conduit verifies the account actually
selected; no silent account or API-billing fallback is permitted. Concurrent subscription
accounts must not overwrite one another's login, refresh, logout or private state.

External-agent integrations delegate authentication to their native runtimes. Conduit's
own runtime needs supported provider/account adapters, with validated subscription
eligibility, identity evidence, credential namespaces and refresh ownership. Successful
login alone does not prove concurrent-account correctness.

Shared [Pool service authentication](#shared-access-and-authentication) supplies capabilities such
as Linear, GitHub, AWS or a password manager without another service login for each Agent. It does
not replace an Agent's explicitly selected model subscription or merge private state. Reuse of
model-account credentials is governed by that runtime's account contract; do not automatically
redirect model requests to a Pool's service identity.

Coordinate credential reuse separately from Agent history. Do not switch accounts by
swapping one global credential directory or blindly cloning rotating credentials. An
Agent template is distinct from a native agent's login profile. The credential/state
layout is an adapter validation task, not a prescribed filesystem schema.

### Execution policy

Native execution is the v0 default: the runtime runs on the host without Conduit's full
OS sandbox. Isolated execution is selectable; a template requirement is materialized in the
Agent spec, alongside explicit authority dependencies. Required isolation cannot be weakened
by launch overrides; unavailable required isolation causes
a visible launch failure. Show the configured mode and record actual enforcement coverage.

Native mode preserves host integrations and suits users granting broad local access.
Account/Agent state separation addresses configuration collisions without the setup
cost of mandatory isolation. It is not an OS security boundary. Isolated mode covers
the runtime and its child processes; host tooling, keychains, IDE sockets and local
services may require explicit bridges or remain unavailable. This is why both modes exist.

Isolated execution defaults to declared network destinations: required model/login
services, selected connections, and extra destinations authorized through Conduits.
Agent limits and exclusions still apply; local/private services require explicit
access. Network reachability does not authorize every operation at a provider. Exact
sandbox technology and matching rules remain open. Native host access, network policy,
provider authorization and Conduit-managed access have different coverage.

### Instances

An Agent has a durable identity, resolved configuration, private context/history and
independent connection attachments. Sharing an Agent template or account does not share private state
or establish communication. One Agent may span multiple executions; record their actual settings.
The selected Agent example keeps its runtime/account/provider boundary fixed while allowing
supported model selection within it. That policy illustrates a template-declared control contract,
not a universal mutability rule for every family or integration.

There is at most one active primary execution per Agent instance. Independently coordinated parallel
workers use separate Agents. Runtime-managed subagents may remain within their parent's
execution, with ownership and usage attributed where available.

### Runtime and presentation

Conduit implements its own runtime in Rust: Agent supervision, model/tool execution,
context construction, provider dispatch, recovery and observations. Provider adapters
handle request/response conversion and supported authentication. External-agent adapters
connect separate runtimes and report their actual lifecycle/access/telemetry capabilities.

The first frontend is a custom terminal inspired by Pi. Presentation is replaceable;
it does not own credentials, model calls or authoritative execution state. Pi terminal
components may be reused, but Pi's agent loop and Pi Durable are research references.
A memory backend or frontend change must not replace Conduit's execution loop.

The existing default is one controlling frontend for direct input, cancellation and settings,
with other frontends observing. Its mapping to Interface attachments and Agent-originated control
now needs review; it does not create a path around Conduits. Moving
control to another frontend requires no extra confirmation. Disconnecting detaches and
preserves supported ongoing execution. Keep persisted history, current model context,
frontend state and shared Resource content distinct.

### Lifecycle and continuity

| Operation/state | Contract |
| --- | --- |
| Open, paused, closed | Agent lifecycle is separate from running/idle execution and frontend attachment. Paused Agents require explicit resume; closed Agents can be explicitly reopened. Closing does not complete work or delete resources. |
| Resume | Keep the logical Agent and reuse compatible native state and the applicable permitted checkpoint. Record any transformations or gaps. |
| Fresh | Create a new Agent without automatically loading prior conversation/checkpoint content. Existing records and files remain retrievable through permitted access. |
| Fork | Create an independent Agent from a selected portable checkpoint with an origin reference. The source Agent is unchanged. |
| Unexpected interruption | Reconnect only when the integration verifies the execution is still running and supports reconnection. A stopped execution requires explicit resume to start a replacement. Unknown status never launches a duplicate. |
| Delete Agent | Require its agent executions to be confirmed ended, including native agents that are idle but still alive. Reject deletion while an execution is live or its status is unknown, preserving the Agent so the caller can explicitly stop or reconcile and retry. Deletion does not initiate shutdown; closing a conversation and purging data are separate operations. |

Fresh/fork do not clone or delete workspace files. A new Agent for a different account or a
frontend change does not by itself create a checkout. Reusing an account must not expose another Agent's
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
to Agent/execution identity. Usage records identify account and reporting source, and
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

The Resource template also declares allowed configuration changes and their parameters, using
the common control contract within its type's supported behavior. Its exact field layout remains
open; the previously settled type/config/connection fields are:

Resource specs use these fields in both template and instance forms:

| Field | Contract |
| --- | --- |
| `spec.type` | A type-contract reference with separate `group`, `name` and `version` fields. This is distinct from a registry object reference using kind/form/name. |
| `spec.config` | Type-specific settings validated for the selected type and form. Templates declare establishment intent; instances retain their ongoing configuration. The transition and exact fields remain in review. Unknown fields are rejected except in schema-declared open maps. |
| `spec.conduits` | The consumer's Conduit declarations. Their entry shape and contents remain under review. |

Resource types have logical groups independent of installation packages. A plugin package may
supply types from several groups; changing implementation packaging need not rename contracts.
Group and type name are separate fields, not a slash-delimited identifier. Use `spec.type.group`
for the group, `spec.type.name` for the type within it, and `spec.type.version` for its contract
version. The group catalog is extensible; it is not a fixed list of Resource kinds.

The first working catalog uses four core groups:

| Group | Responsibility |
| --- | --- |
| `filesystem` | Expose or manage filesystem trees. |
| `storage` | Persist data with defined operations. |
| `search` | Retrieve relevant results from configured sources. |
| `integration` | Expose configured native service/tool interfaces. |

These group boundaries are accepted. Within `filesystem`, `local-directory` and `git-worktree`
are separate types: both expose directory access, while the latter also has Git-specific
creation, attachment and cleanup obligations. This selects contract boundaries, not a
requirement to implement both immediately. Other types and implementation scope need review.
Keep an additional agent-communication group under consideration for direct messaging,
multi-participant conversations and shared context. Agent-specific contracts can justify a
distinct group even when they use familiar storage/integration mechanisms. Its name, types,
Resource-versus-Conduit responsibilities and shared-context semantics remain open. This does
not select a room topology or a jointly editable memory model.

Contract ownership/third-party collision rules, allowed identifiers and version labels remain
open. Groups and type contracts add no registered object family, assign no Pool ownership and
grant no access. [Resource typing](research/RESOURCES.md) retains the supporting investigation,
candidate types and proposed interface boundaries; those proposals are not built-in requirements.

Resource type contracts are versioned independently of the common object `apiVersion`.
The common format can remain unchanged while a type's configuration/behavior contract evolves.
Plugin implementation releases are separate from both versions; a plugin release number does
not substitute for either contract version. Version identifiers and compatibility/migration
details remain open.

The same template may create distinct instances in multiple Pools. A contract supports new
backing data, attachment to existing backing data, or both explicitly. Different instances
may therefore reach the same native target without sharing registry ownership. Creating a
new instance does not prove that files, credentials, quotas or remote data were copied.

Resources should be as simple and unitary as possible: one responsibility with its related
operations. A storage Resource may read/write/delete records, a search Resource runs queries,
and an MCP-access Resource handles its configured integration. Keep those responsibilities as
separate Resources by default and group them in Pools, including when one provider supplies
all three. Separate registrations do not require separate backend processes or copied data.
Access between Resource instances is expressed through Conduits; exact configuration syntax,
connection creation and type schemas remain open.

A repository, storage collection, search capability, skill, document or provider access is an
example, not a closed taxonomy. Related operations stay within their Resource; each operation
or native tool does not require a separate Resource merely because it is independently callable.
Native files, tickets and records likewise need not each become a registry object.
Labels aid selection, never identity, authority or implicit ownership transfer.

Resource instances record enough identity/contract information to resolve their source and
supported operations. Provider-owned data retains native contracts; attaching it does not
make Conduit the owner of the provider account or authorize deleting that backing data.

Resource templates and instances provide concrete protocol review cases. A local path exposed through a
symlink, an MCP connection and hybrid search over a database are contrasting examples, not
interchangeable implementations of a fixed lifecycle. Settle their typed configuration,
backing identity, operations and shared/per-consumer state before deciding Pool grouping.
A type contract defines supported semantics; a template supplies reusable establishment
configuration and declares the permitted configuration changes within that support.
The minimum common fields and consumer boundaries for other Resource types remain open in
[REVIEW.md](REVIEW.md#1-resource-templates-and-instances).

For a local folder, the Resource instance describes the configured folder attachment. Each
Agent's symlink is preparation state, not another Resource instance. Two Agents may use
the same Resource through different local paths; removing one prepared symlink leaves the
Resource, its owning Pool, backing folder and other Agent's view intact. Creating another
symlink does not copy the folder or isolate concurrent edits. Exact path placement and cleanup
timing remain open; other Resource types may require different consumer lifetimes.

The `local-directory` type defines folder access without Git lifecycle management. Registering
a checkout under that type does not imply managing its branches or worktree creation/removal.
The `git-worktree` type additionally represents its relationship to a Git repository and the
supported worktree-management operations. Both retain Pool ownership, Agent-specific
preparation and explicit backing-data cleanup rules.

In v0, `local-directory` declares establishment intent in its template spec: attach an existing
directory or create a new one through its Pool. Attachment never creates a missing path.
A fresh creation reports a conflict when its target already exists instead of silently adopting it. Registration
alone performs neither operation, and backing-data cleanup remains separate. Retries still use
recorded operation outcomes and the existing safe-retry rules; an uncertain outcome does not
justify blindly repeating creation. Instantiation reads this declared intent rather than asking
the caller to assemble a separate provisioning recipe. Exact configuration/transition syntax,
Git-worktree-specific modes and any shared directory interface remain open.

Local-directory templates may contain relative backing paths. Creation resolves them against
an explicit base and saves the resulting absolute host path in the complete instance spec.
For example, `./repo` with base `/work/euphony` becomes `/work/euphony/repo`. Later use does
not depend on the launching terminal's working directory or reinterpret the original relative
path. This is the backing path, distinct from each Agent's prepared path. Symlink resolution,
Agent-local placement and the exact configuration/operation fields remain open.

### Native resources and integrations

Users bring compatible skills/plugins through Pools and Conduits. User-level native
settings are selected explicitly; an existing setup can be selected once, but the user's
default agent profile is not automatically inherited. Mandatory host/organization policies
still apply. Selecting a package does not grant extra MCP or resource access.

Use ecosystem tooling for compatible skill/plugin installation and updates. Adapters may
invoke it on request and record source/version information and outcomes. Conduit does
not require a new skill format or package manager. Executable native extensions need a
compatible runtime adapter or port; report unsupported hooks. Agent activation instructions
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
| Identity and ownership | Stable internal record IDs survive names/paths/external-key changes. Each record has one owning context, such as a task, Pool or personal collection. These content contexts do not create additional owners of the containing Resource instance. Mapping them within Pool-owned memory needs a concrete schema; an Agent's working context remains separate. |
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

Memory search defaults to all memory permitted by the Agent's current Conduits and
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
| Grant lifetime | Issued grants survive changes to their creator's Agent or its template. Parent-constrained access recovers when that same chain permits it. Explicitly withdrawn grants require authorized restoration/reissue; a new parent does not replace an old parent link implicitly. |
| Independent publication | A separate record may release source-access dependencies only with read/publication authority for each released restricted input and destination creation rights. Retain source/version provenance. The published record has its own audience/lifecycle and does not follow source changes or withdrawal. |
| Published edits | Edits using already-published content need ordinary update rights. New restricted material requires current read/publication authority for those inputs and destination update rights. Explicitly released attribution remains readable. |

### Workspace resources

A workspace Resource instance belongs to its Pool and is associated with a task or
experiment; Agents attach through permitted access. V0 supports existing folders/checkouts
and Conduit-managed Git worktrees, recording management status. Attachment preserves existing files and Git state and grants no deletion
authority. Concurrent independent writers use separate task-associated worktrees by default;
sequential handoffs can reuse a workspace. Shared editing is opt-in, without automatic
merge/file-lock guarantees. A worktree is not an isolation boundary.

Retain workspaces after Agent closure or task completion until explicit cleanup.
Cleanup of attached folders only unregisters them. Remove a managed worktree only when
no execution uses it and no unique local work would be lost; uncertainty preserves it
with an explanation. Exact creation/base/branch/path settings remain open.

## Pools

Users compose a Pool through an Interface, referencing reusable Resource templates and their
establishment settings. A Pool template describes those selections, defaults, policies and
permitted management/configuration changes. A Pool
instance creates/attaches and exclusively owns its Resource instances. Pools remain the
resource-supply and shared service-access boundary for Agents and connections, including skills,
tools, memory and connection support such as conversation storage. A Pool need not run a process.

Creating a Pool registers its identity and configuration. Provisioning is explicitly requested
to create or attach Resources through their owning Pool. A frontend may offer both in one
explicit action with separate outcomes; successful registration does not imply provisioning.
No background reconciler is implied.

Persistent Resource declarations and per-entry bindings are back under review until Resource
contracts are defined. The previous Pool proposal and its lifecycle rulings are consolidated
in [REVIEW.md](REVIEW.md#2-pool-grouping-and-resource-ownership); they should not force extra
configuration/state layers into the Resource model. Pools group unitary Resources and should
account for their types when coordinating capabilities. Exact grouping behavior, provisioning
syntax and partial outcomes remain open. Stable identity, explicit effects, safe retry and separate
backing-data cleanup remain requirements regardless of the chosen representation.

Resource templates can be selected by explicit references and labels where supported.
Selection among an existing Pool's owned instances is another operation. Exact selector
syntax, target domains and refresh rules remain open; neither labels nor references can
silently adopt an instance from another Pool. Reusing templates creates new owned instances,
not overlapping ownership. Pool schemas remain open to new Resource contracts.

Connection participants can access Resources in several Pools through authorized paths to
the owners. Sharing backing data between separate instances does not transfer ownership or
establish access. Pool ownership, current access, Agent activation and native backing-data
management have distinct meanings.

Creating a Resource for a connection must identify its owning Pool; the connection requests
provisioning through that Pool. How provisioning retries partial work or resolves dynamic
support requirements remains open. A template registration alone does not create Resources.

When launching against a Pool, its access defaults apply only if no explicit selection
is supplied. Explicit selection replaces those defaults; extending them requires an
explicit add action. Agent limits and exclusions always constrain the result.
Combination with connection choices materialized in the Agent spec remains open.

Pool retirement, possible ownership transfer, active-use handling and incomplete provisioning
need explicit contracts. Deleting a grouping cannot silently orphan deployed Resources under
this model. The response to an orphan does not imply deleting attached external data or
purging retained history. Ownership cleanup and backing-data cleanup remain separate.

### Shared access and authentication

**Accepted:** Pools are the normal place to establish and manage shared service access. Their
owned capabilities retain the selected provider bindings so multiple authorized Agents can use
them without each completing provider login. This applies beyond MCP: a suitable integration can
manage database, repository, cloud or credential-service access. Actual provider support and any
interactive renewal requirements remain explicit.

A binding means a managed association to an authorized service identity, not a copied secret in
configuration. Plugins implement authentication; instance runtimes own continuing state/refresh.
The Pool owns and organizes the capability rather than introducing a second authentication engine.
One service Resource can manage its binding; reusable authentication across several Resources may
justify a separate Pool-owned capability. That representation is the next review decision, not a
new object family or a requirement for a Resource per token.

| Concern | Contract |
| --- | --- |
| Service authority | The provider identity and grant bound to the capability; reuse is intentional and must be supported. |
| Caller authority | The Agent/Interface identity and its current permission to use that capability. Shared login does not mean identical rights. |
| Connection restrictions | Each Conduit narrows permitted Resources, operations and parameters. Pool membership or a matching label does not grant access alone. |
| Credential custody | Keep upstream authentication with its managed owner. Ordinary capability use does not hand the provider token to the caller. Secret retrieval/injection is a separate explicit operation where supported. |
| Transport | Shared identity does not mandate one socket/client/process. Clients can be pooled or separate while refresh has one coordinated owner. |
| Attribution | Record the actual caller and Conduit locally. The provider may attribute all calls to the shared service identity; do not invent per-Agent upstream attribution. |
| Lifetime | Detachment releases the flow's own state, not shared credentials or another consumer's client. Login failure/revocation can affect all users of that binding. |

The selected binding's provider permissions, Resource policy, caller authority and Conduit filters
all constrain use. An additional authorized Agent attaches to the existing capability; it does
not rerun its service login as a mandatory attachment step. Revoking just that Agent's access leaves
the binding available to others. Templates declare supported shared setup and management controls;
binding replacement, logout, refresh failures and idle runtime policy need explicit type semantics.

Per-consumer service identities remain an explicit exception when selected or required by the
provider. Several identities can target the same service. Matching endpoints or using a common
template does not implicitly combine accounts, grants or OAuth state. Cross-Pool reuse also needs
an explicit authorized path to the owning capability, not duplicated ownership or copied tokens.
Shared authentication does not merge private Agent context or selected model subscriptions.

For example, one Euphony Linear Resource uses a managed login. A review Agent reads issues while
an implementation Agent may update them through a different Conduit. Removing the review Agent
preserves the implementation Agent's access. The Resource's authentication state and each caller's
connection/authorization state remain independently observable. See the
[full walkthrough](research/CONDUITS.md#5-access-linear-through-mcp).

## Conduits

Conduits expose permitted target operations and parameter constraints from the
[configuration-change contract](#template-defined-configuration-changes). This applies to
Interface-to-instance controls as well as Agent/Resource access and communication.

A Conduit template describes establishment, ongoing interaction/filtering and its own permitted
configuration changes. Changes to its filters or endpoints require an exposed management contract;
an ordinary request carried through it does not implicitly grant that capability. Opening from
that configuration establishes an authorized flow with resolved endpoints,
logical attachments and connection state. Agent and Resource templates can reference
Conduit templates as connection declarations; the resulting consumer instances retain those
declarations in their configuration. Explicit preparation of an Agent or Resource normally
attempts each declaration's configured open/join action, rather than waiting for first use.
Registering configuration alone opens nothing. Report connection/authentication outcomes as
part of preparation; an attempt does not imply successful establishment. Declarations default
to required; optional connections must be marked explicitly. Saved instance configuration
materializes this default, while templates may omit it. A failed required connection prevents its
consumer from completing preparation; an optional failure is reported and preparation may
continue. Unrelated consumers are unaffected. Declarations do not require authoring the same
endpoint relationship twice.

If preparation partially succeeds before a required connection fails, retain successful
attachments for retry. The consumer remains unprepared but partly connected; retry reuses
retained attachments that remain usable under current access. Explicit cancellation detaches
the attachments retained by that preparation. Preserve other consumers, backing Resources and
recorded outcomes; cancellation does not undo data already delivered or accepted external
effects. The last-consumer rule governs when a flow ends. Retry/timeout rules, incomplete
cleanup, ordering and reconfiguration remain under review.

For a declaration inside an Agent or Resource, the enclosing instance supplies the consumer
endpoint automatically. A reusable search template declaring `storage-read` can therefore be
used by `search-a` and `search-b`, each resolving itself as the consumer without an authored
local-endpoint marker or generated instance name in the template. This supplies endpoint
identity only; authority checks and explicit open/join intent still apply. Preparation supplies
the default establishment trigger. Remote endpoint selection and exact declaration syntax
still require review.
Conduits are the extensible interaction layer across the five families. Pool access and
Agent communication are examples, not a closed list of endpoint roles or protocols.

Resource-to-Resource access uses Conduits as well. For example, a search Resource reads a
storage Resource through a logical Conduit connection under the applicable access/filter
rules. Pools retain ownership and group the Resources; grouping alone does not establish
that connection or grant access. This models logical relationships, without creating a new
Conduit for every query or underlying socket. Exact endpoint declarations, connection creation
and runtime state placement remain open.

The Conduit engine and plugins perform transfers, maintain clients and run synchronization.
An individual Conduit has instance-managed behavior implemented by the common runtime/plugins;
it need not have a dedicated process or independent execution engine. One-time transfer, persistent
access and ongoing sync are supported directions with different interaction semantics, not
a closed taxonomy. Logical connection state is distinct from transport clients and backing data.
Every supporting Resource is created/attached through, and exclusively owned by, a Pool.
Routing state and cursors need not each be registered Resources; durable shared content still
follows Pool ownership and current access rules.

A live Conduit ends when its last logical consumer or peer detaches. Release remaining
connections/leases owned by that flow; preserve Resource-owned authentication and clients serving
other flows. Providers alone do not keep the Conduit alive. A subscription remains
attached between polls, and temporary transport failure affects readiness without removing
that attachment. Ending the flow does not delete its template, backing Resources or retained
configuration/operation evidence, and does not prove an already accepted remote operation was
cancelled. Report incomplete or unknown cleanup rather than claiming confirmed disconnection.
Crash detection, explicit detach and later reopening/reuse rules still need concrete contracts.

Participant attachment state describes the individual relationship; Resource-owned shared
authentication and connection support can have a different authority and lifetime. Template reuse
or joining alone does not merge private Agent context, credentials or access. Connection identity
is separate from process, transport and backing-Resource identity. Further authoritative-state,
recovery and composition details remain in [REVIEW.md](REVIEW.md).

### Explicit joining and instantiation

Connection establishment supports both intents explicitly:

- **Join an existing instance:** establish participation in a specified existing relationship.
- **Open a new connection:** create a relationship from its establishment configuration,
  subject to the contract's endpoints, authority and provisioning requirements. One-off/import
  syntax remains in review alongside the template-to-instance transition.

Reusing a template name does not implicitly choose between these intents. For example,
two Agents can join the same discussion, or create separate discussions from the same
template. This choice does not by itself determine whether clients, credentials, backing
data or processes are shared. Joining preserves participant-specific caller authorization while
service authentication follows the explicitly configured shared or per-consumer mode;
creating another relationship does not imply copying its underlying Resources.

Exact selection syntax, instance addressing/identity, repeat-request behavior and explicit
connection-reconfiguration operations remain open. Both modes belong within
the existing Conduit concept, rather than requiring another top-level kind.

### Resource selection and activation

For resource access, the Conduit selects and filters available Resources and operations.
Activation instructions belong to the Agent spec and are applied by the Agent. Filters
are narrowing-only: removing a constraint can reveal an existing grant, but a filter
cannot create one. Exact type names, endpoint forms, lifecycle and composition remain open.
Agent activation prepares the participant. Pool operations create/attach supporting
Resources; the engine and plugins execute and supervise Conduit flows. Their ordering and
failure behavior are unresolved, but a connection cannot provision an unowned Resource or
assume the Agent's loading instructions merely because it requested storage.

### Authority and current access

Permissions name operations per Resource contract, with convenience bundles expanding
into visible grants. Agent limits and explicit exclusions constrain effective access;
missing limits add no authority. Permitted operations require no extra confirmation merely
because permission rules exist. Resource and communication permissions do not imply each other.

Selection supports IDs, owning contexts, metadata predicates and path patterns where the
integration supports them. Matches within established access rules are live. Authorized
connection, filter and Agent-limit changes apply to subsequent managed requests and are
shown and recorded; they do not silently change model/account defaults or inject context.
Continuing authority dependencies are explicit on the instance. Creation-template edits do
not update those limits. Applying edits to instance provisioning/selection and activation
settings during active work remains a family-specific lifecycle question.

If an isolated execution cannot revoke removed access live, stop it and require explicit
restart under current rules, preserving Agent identity. Managed requests use new rules
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
Use the [shared Pool access contract](#shared-access-and-authentication), with an explicit
per-consumer identity exception where necessary. A different login alone does not require another
service Resource instance; binding representation and native support determine the valid setup.
Conduit's runtime owns its MCP clients; external-agent adapters prepare native registration
from effective selections. Registration, provider authentication and Conduit Agent
authority are distinct. Reusing a definition does not promise shared live clients, OAuth
state or backend persistence. Avoid rewriting global settings for each launch and validate
inherited/plugin connections and host configuration behavior.

Preserve provider-native tool names, schemas, descriptions, results and workflows where
possible. This is contract compatibility, not a promise that an external native client connects
directly to the provider's hosted MCP URL. A Conduit-managed MCP proxy can expose the permitted
provider tools and invoke the real upstream service; its endpoint and authentication boundary
are distinct. Provider-specific entries can remain separate, with client-required namespacing;
no combined gateway or generic replacement tool is required. A Conduit-owned runtime may mediate
internally without another MCP server. Literal direct provider connections only have the controls
their verified native integration supports. Integration capabilities must cover nested calls and
actual dispatch rather than treating discovery lists as enforcement.

Authenticate the local caller separately from authorizing the upstream service request. Do not
accept a Linear token as a bearer credential for a different Conduit MCP audience. Protected
references and a supported upstream OAuth/token flow are distinct from token passthrough.
[MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)

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

Communication is an interaction between Agents through Conduits, with Resources and
Pools supplying authorized shared material. The type-specific contracts still need to
settle addressing, acceptance, results, retention/replay, attention, cancellation and
tracker associations. Keep receipt, execution and completion distinct. Research should
produce concrete flows and failure cases before protocol or implementation selection.
