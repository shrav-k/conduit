# Conduit specification

Updated: 2026-10-31. Accepted working model; implementation has not started.

**Branch context:** this specification preserves the model saved on `main` at `c013f0a`.
The active [Conduit-graph proposal](research/CONDUIT_GRAPHS.md) explores graph-based establishment
and continuing flows. Its detailed contracts remain in review; the distinctions below provide
the baseline against which to evaluate them.

## Working model

Conduit connects models, agents and capabilities through four families: **Session, Resource,
Pool and Conduit**. Each has a reusable **template** and a concrete **instance** form.
This distinction applies throughout the system; it does not require every instance to run a
process or every family to have identical lifecycle operations.

| Family | Template defines | Instance represents |
| --- | --- | --- |
| **Session** | Runtime/model/account defaults, activation instructions, required limits and default connection choices. | A durable participant with resolved settings, private context/history and executions. |
| **Resource** | A typed recipe for creating or attaching data/capabilities, with supported configuration and behavior. | A particular provisioned or attached capability with identity, backing-source information and exactly one owning Pool instance. |
| **Pool** | Resource-template selections, provisioning defaults and policies. | An ownership boundary that creates and contains its own Resource instances. |
| **Conduit** | A typed interaction contract, endpoint requirements, filters and supporting-resource requirements. | A live filtered flow with resolved endpoints, logical consumer/peer attachments and connection state. |

Profile is the former name for a Session template. An attachment is a consumer or peer's
logical participation in a Conduit; neither term adds a family. A plugin's type schema describes valid
contracts; a template supplies reusable configuration for such a contract. Neither template
form nor instance form implies an additional general-purpose programming language.

Each Resource has one responsibility with its related operations; Pools group Resources into
useful sets. Storage, search and MCP access are separate Resources by default, even if supplied
by the same backend.
Logical access between Resource instances is expressed through Conduits, such as search
reading storage. Pool grouping preserves ownership; it does not itself establish access.
Minimize duplicated bookkeeping within these explicit boundaries, keeping one authoritative
configuration per instance. Define Resources before Pool grouping: local paths, MCP services
and hybrid database search require different typed configuration and behavior. A shared envelope
does not imply a uniform runtime or provisioning lifecycle. The next review derives common
Resource requirements from those examples, then considers how Pools group and coordinate
their declared capabilities. The [establishment review](research/ESTABLISHMENT.md) adds native
subscription login, corpus indexing and password-manager authentication to test ordering,
typed outputs and recovery. It recommends declarative templates, recoverable plugin stages
and engine coordination; the lifecycle protocol and composition syntax remain proposals.

```mermaid
flowchart LR
    ST[Session template] -->|instantiate| S[Session instance]
    PT[Pool template] -->|instantiate| P[Pool instance]
    RT[Resource template] -->|provision or attach through Pool| R[Resource instance]
    P -->|creates and exclusively owns| R
    CT[Conduit template] -->|open| C[Live flow: Conduit instance]
    S <-->|join and interact| C
    C -->|authorized access / provisioning request| P
```

This diagram shows common interactions, not a closed topology or process layout. Conduits
are the extensible interaction layer across the families; each contract specifies meaningful
endpoint roles and operations. Templates describe potential behavior; instances carry identity
and realized state. Executions, transports and participant attachments are supporting state.

## Resource ownership and creation

Every Resource instance is created through a Pool instance and belongs to exactly that Pool.
This includes attaching an existing directory/service and provisioning backing Resources for
a connection. A bare Resource-creation path must not bypass Pool ownership. Authoring or
registering a Resource template does not create a deployed Resource.

Different Pools can instantiate the same template. The resulting Resource instances are
distinct even when they intentionally attach to the same repository, vault or remote service.
Contracts must distinguish creating new backing data from attaching to existing data. Separate
registrations do not prove separate files, credentials, quotas or mutation targets.

For a local folder, one Resource instance describes the configured folder; per-Session symlinks
are preparation state. Removing one symlink preserves the Resource and backing folder, without
affecting other Sessions' views. This does not require the same consumer lifecycle for every type.

An MCP Resource can serve Sessions using different permitted service identities, resolved per
attachment where the integration supports it. The configured target and access constraints still
apply; differing service logins alone do not require duplicate Resources or change model accounts.

Conduits access Resources through their owning Pools. A connection that needs conversation
storage requests it from a selected Pool, then uses that Pool-owned Resource. Connections and
Sessions may use several Pools; this does not make them owners of the Resources. Cross-Pool
access follows authorized connections to the owner, not duplicate membership of one instance.

Pool ownership is not automatic access, activation or authority to destroy external data.
Selectors may help choose templates or the Pool's owned instances; their exact targets and
live behavior remain open. They cannot implicitly adopt another Pool's Resource instance.
Pool retirement, ownership transfer and orphan handling require explicit lifecycle design.
No automatic purge follows from a missing Pool reference.

## How instances work together

1. **Create a Session.** Establish it from typed template configuration. One-off configuration
   and instance import are under review where establishment information differs. Later edits
   target the instance; current access limits are explicit. V0 defers Session-template inheritance.
2. **Register Pools, then explicitly create/attach Resources.** Pool registration and Resource
   provisioning are separate effects; a frontend may combine them in one explicit action with
   separate outcomes. The Pool owns the resulting instances. Persistent creation declarations
   and their bindings are back under review pending Resource contracts; no background
   reconciler is implied.
3. **Open or join a connection explicitly.** Opening uses declared establishment configuration;
   joining targets an existing connection. Reusing a template name
   does not choose between these operations or imply shared clients, credentials or private context.
   Session and Resource templates can reference Conduit templates to describe flows their
   instances will establish. The enclosing instance automatically supplies the consumer endpoint;
   it need not repeat its identity in the declaration. This grants no authority and does not
   choose open versus join. Explicit consumer preparation normally attempts those declared
   actions and reports their outcomes; registration remains passive. A failed required connection
   blocks its consumer's completion of preparation; optional failures are reported and allow
   preparation to continue. Declarations default to required; optional is explicit, and saved
   instance configuration materializes the default. Partial failure retains successful attachments
   for retry; explicit cancellation detaches those retained by the preparation without undoing
   delivered data or external effects. Other consumers and backing Resources are preserved.
   Remote endpoint selection, exact fields and retry/cleanup details remain open.
4. **Apply access and activation.** Connection filters narrow authorized resources and
   operations. The Session applies activation instructions in its own spec to the permitted
   inventory. Availability alone does not load instructions or content into the model.
5. **End the live flow after the last consumer/peer detaches.** Close remaining provider-side
   connections. Logical subscriptions stay attached between polls or during temporary transport
   failure. Templates, backing Resources and retained evidence survive; ending a Conduit does
   not establish cancellation of an already accepted remote operation.

Session activation, Pool provisioning and connection supervision are different responsibilities.
The first prepares a participant; the second creates/attaches owned Resources; the third
implements an interaction through the engine/plugins. A Conduit has no independent runtime.
One-time transfers, persistent access and ongoing sync may use different interaction contracts.
Their ordering and dependency rules remain for review.

## Configuration language

**YAML is the authoring format; typed objects and explicit semantics define the language.**
The common envelope is `apiVersion`, `kind`, `form`, `metadata`, `spec`, with system-owned
`status` in inspection. Each object uses one family `kind`—Session, Resource, Pool or Conduit—
plus a top-level `form` identifying a template or instance. Validation must
distinguish both family and form; sharing a kind does not imply identical payloads or operations.

Resource specs use `type: {group, name, version}`, schema-validated `config`, and `conduits` for
connection declarations. The type reference is distinct from registry kind/form/name references.
Logical type groups are independent of plugin packaging; group and type name are separate
fields, never a combined slash-delimited identifier. The first working core groups are
`filesystem`, `storage`, `search` and `integration`; new groups and types can extend the catalog.
Within `filesystem`, `local-directory` and `git-worktree` are distinct types. Both expose a
directory; Git-specific management belongs to the worktree contract. Their detailed schemas
remain open. In v0, local-directory supports explicit attachment and creation through its Pool:
attachment requires an existing directory; fresh creation conflicts with an existing target.
Create/attach intent belongs in the Resource template's spec; triggering establishment does
not require a separate operation-request recipe. Registration performs neither effect, and
cleanup remains separate. The type contract must define how establishment settings become
instance configuration and observed results; exact fields remain open.
Relative local-directory template paths resolve against an explicit creation base; the complete
instance stores an absolute host backing path. Session-local exposure is separate.
An agent-communication group is a candidate for direct messaging, conversations and shared
context. Its contracts and name remain open, alongside other core types and third-party
naming rules.
Type contracts have their own schema versions, independent of common object `apiVersion` and
plugin implementation releases. Allowed type identifiers/version labels, Conduit declaration
contents and migration details remain open.

Retain the accepted registry and editing contracts in [DESIGN.md](DESIGN.md#configuration-language):

- Stable IDs survive rename; replacement objects receive new IDs. Readable references record
  resolved identity, and established explicit bindings require explicit rebind to another ID.
- Readable configuration-object references explicitly include `kind`, `form` and `name`.
  Kind/name locates the object; form checks the expected template/instance distinction.
  A wrong-form target is rejected, including in fields that already imply a form.
- Use one local registry with names unique per family across templates and instances. A Session
  template and Session instance cannot both be named `reviewer`. Templates name reusable concepts;
  instances default to the template name plus a generated unique suffix, with explicit unique
  names allowed. Exact suffix format and name constraints remain open.
- YAML registration/export is explicit. Editing a file does not update active configuration;
  changing a template is distinct from instantiation, provisioning, execution or reconciliation.
- Across all four families, templates contain the establishment configuration; explicit
  lifecycle actions select when to realize it. Typed transitions produce instance configuration
  and observations; instantiation need not copy every template field into the instance.
  Establishment-only settings, ongoing settings and generated results need explicit semantics.
  Users edit ordinary typed fields; no separate operation-recipe or generic parameter/mapping
  language is required. Assigning a field replaces its whole value, including objects/lists;
  editing a nested child preserves its siblings. Resources require Pool-mediated creation.
  Registration, provisioning and execution remain distinct effects. Later changes edit the
  instance directly; its creation template can change or disappear without changing it.
- Direct creation/import from complete instance YAML is being reviewed where instance exports
  omit establishment information. Preserve the one-off use case without guessing missing
  create/attach intent or adding a parallel request recipe. Exact authoring syntax remains open.
- Template-origin information is optional; no source snapshots or input/override history are
  required. Continuing Resource and authority dependencies must be explicit on the instance.
  Current-access checks, Resource data, runtime status and actual execution evidence remain
  separate from configuration.
- Instance creation and checked replacement updates materialize declared schema defaults into
  saved/exported instance specs. Template YAML preserves omissions. Fields without declared
  defaults retain their required/optional semantics.
- Typed configuration rejects unknown fields except within maps explicitly declared open by
  their schemas. Plugin contracts may define their own fields and open maps; keys and values
  still obey the applicable map contract.
- Creation and replacement require all applicable configuration schemas. If a required schema
  is unavailable, reject the write and preserve source YAML and any existing registered object.
  If schemas and required registry checks pass, register configuration despite a missing plugin
  runtime or offline configured service. Report known readiness gaps or unknown readiness;
  operations requiring unavailable dependencies remain blocked. Registration is separate from
  activation and does not relax validation, authority or Pool ownership.
- Create/update intent is explicit. Updates replace complete authored configuration and use
  separate expected-ID/configuration-version checks. Status-only changes do not invalidate edits.
- Unchanged configuration preserves its version after checks. Compare instances after default
  materialization, so omission and an equivalent explicit default produce the same stored
  value. Templates retain that authored distinction; presentation-only differences do not count.
- CLI/editor edits retain recoverable baselines; absent baselines need explicit preparation.
  Status is rejected on normal writes and omitted with system IDs/versions from reusable exports.
- Deletion is checked; registration removal, native shutdown and backing-data purge are distinct.
  Pool ownership introduces lifecycle obligations that ordinary reference behavior does not settle.

References, defaults, field values, selectors, filters, activation and provisioning require
field-specific semantics. Draft assignments do not implicitly merge objects or append lists;
there is no implicit textual substitution.
Authored intent, resolved configuration, observations and operation outcomes stay distinguishable.

## Implementation direction and retained behavior

Conduit owns its runtime in Rust, including execution, context construction, provider/tool
dispatch and recovery. The first frontend is a custom terminal inspired by Pi. Pi's runtime
and Durable are research references; external-agent adapters separately support native runtimes.
Provider adapters and native-agent adapters have different authentication and lifecycle contracts.

MCP is the preferred external capability protocol. Preserve provider-native names, schemas and
workflows. Resource-service authentication is separate from model subscriptions. Native skills
and plugins are brought through compatible ecosystems, without a new Conduit package manager.
Linear owns its tracking workflow; memory/search/graph providers remain replaceable plugins.

Native execution is the v0 default, with selectable or Session-required isolation. Four
independent accounts remain an early acceptance target: two Claude and two Codex/OpenAI
subscription identities. Actual identity, subscription compatibility and refresh/logout isolation
must be verified. No silent account or billing fallback is permitted.

Session continuity, portable checkpoints, current-access checks, memory revisions/provenance,
bounded search with coverage reporting, read-only sharing, workspace retention, safe retry and
usage attribution remain as specified in [DESIGN.md](DESIGN.md). A four-family vocabulary does
not relax these rules or make native host access fully mediated.

## Acceptance scenarios

These are intended outcomes, not completed tests. [REVIEW.md](REVIEW.md#initial-experiment)
defines the remaining selections and staged evidence.

| Interaction | Observable outcome |
| --- | --- |
| Template versus instance | References identify the intended form; registering a template does not start work. The complete instance spec remains usable after its creation template changes or disappears, subject to explicit dependencies and current authority. |
| Establishment from configuration | A Resource template carries create/attach intent. A lifecycle trigger uses that intent through the owning Pool; actual outcomes become instance observations. Instance import must report missing establishment information, with its exact authoring path still under review. |
| Typed Resources | Filesystem, MCP and search examples have explicit type-specific configuration, operations and consumer state. A template configures a contract; it does not define a new type. Pool grouping follows these semantics. |
| Unitary composition | Each Resource has one responsibility with related operations. Storage, search and MCP access are separate Resources by default, grouped in Pools; this does not require separate backend processes/data. |
| Resource connections | Explicit consumer preparation attempts declared open/join actions under applicable authority and filters. Required failure blocks that consumer's preparation; optional failure is reported and permits continuation. Registration alone starts nothing; unrelated consumers are unaffected. |
| Conduit lifetime | A sync subscription stays attached between polls or during transport failure. The last logical consumer/peer detaching ends the flow and closes provider-side connections; Resources and retained evidence survive. |
| Folder preparation | Two Sessions use one folder Resource through their own symlinks. Preparing/removing a symlink creates/deletes no Resource or backing folder and does not affect the other Session's view. |
| MCP authentication | Two Sessions can use one MCP Resource with different permitted service identities where supported. Resolve identity per attachment within target/access constraints; shared configuration does not imply shared authentication state. |
| Pool registration and provisioning | Registering a Pool does not itself provision Resources. Explicit provisioning has its own outcome, including when a frontend combines registration and provisioning. The declaration/binding representation remains under review. |
| One template, several Pools | Pools create distinct Resource instances; each has exactly one owner. A foreign Resource cannot become a local member through a label or reference. |
| Shared versus independent backing | Both explicit attachment to shared backing and provisioning independent backing are expressible; target identity and actual enforcement are reported. |
| Connection support | Opening a connection requiring storage provisions it through an identified Pool. Joining reuses the intended connection without creating a duplicate conversation. |
| Concurrent accounts | Four Sessions retain independent private state and verified account selection while accessing authorized common Resources. |
| Session activation | Instructions materialized in the Session spec prepare only permitted Resources; connection filters do not become loading instructions. |
| Context and communication | Sessions exchange permitted context through the selected contract while retaining private histories; memory retrieval reports omissions and coverage. |
| Change and failure | Stale edits conflict, unknown effects remain unknown, and retries do not mistake a repeated create request for a deliberate second instance. |
| Lifecycle | Detach, close, stop, unregister and purge have distinct outcomes; removing configuration does not silently destroy files or retained evidence. |

## Design boundary

The four families, family kinds with explicit forms, exclusive Pool ownership and Pool-only
Resource creation are settled. Review Resource templates/instances and type-specific semantics
first, then Pool grouping. Detailed addressing, lifecycle, provisioning and communication
contracts remain open. Prefer derived views over additional bookkeeping; settle the connected
choices before implementation.
Research proposals and the synthetic scenario are discussion aids, not additional requirements.
