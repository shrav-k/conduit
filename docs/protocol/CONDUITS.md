# Conduit contracts

Part of the accepted [design contracts](README.md). Updated: 2026-11-02.
The [conceptual review](../archive/reviews/CONCEPTUAL.md) records completed rulings and deferred work;
syntax and implementation examples do not amend these rules.

## Conduits

Conduits expose permitted target operations and parameter constraints from the
[configuration-change contract](LIFECYCLE.md#template-defined-configuration-changes). This applies to
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
cleanup, ordering and reconfiguration require explicit type/runtime contracts.

For a declaration inside an Agent or Resource, the enclosing instance supplies the consumer
endpoint automatically. A reusable search template declaring `storage-read` can therefore be
used by `search-a` and `search-b`, each resolving itself as the consumer without an authored
local-endpoint marker or generated instance name in the template. This supplies endpoint
identity only; authority checks and explicit open/join intent still apply. Preparation supplies
the default establishment trigger. Remote endpoint selection and exact declaration syntax are illustrated in the schema packet.
Conduits are the extensible interaction layer across the five families. Pool access and
Agent communication are examples, not a closed list of endpoint roles or protocols.

Resource-to-Resource access uses Conduits as well. For example, a search Resource reads a
storage Resource through a logical Conduit connection under the applicable access/filter
rules. Pools retain ownership and group the Resources; grouping alone does not establish
that connection or grant access. This models logical relationships, without creating a new
Conduit for every query or underlying socket. Exact endpoint declarations, connection creation
and runtime state placement remain open.

Conduits establish and maintain connections by coordinating endpoint-managed plugin behavior,
including local preparation, transport and filtering. Endpoint templates select implementations;
Conduit templates match their compatible capabilities and declare the connection's requirements,
restrictions and establishment procedure. Each endpoint instance manages its local bindings and
reports their outcomes; the Conduit retains relationship and participation state. It may use
orchestration plugins without taking ownership of endpoint-local code or configuration.
An endpoint can adapt an existing service; paired roles do not imply identical plugin packages
or separate processes. Exact endpoint contracts, binding negotiation and cleanup/recovery remain
in the [implementation authoring review](../proposals/authoring-syntax/AUTHORING.md#5-where-the-example-implementations-execute).

The Conduit engine and plugins perform transfers, maintain clients and run synchronization.
An individual Conduit has instance-managed behavior implemented by the common runtime/plugins;
its endpoint behavior need not share a process or require an independent execution engine.
One-time transfer, persistent
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
**Ended-flow identity:** later participation opens a new Conduit instance with a
new stable ID and fresh attachments. It may explicitly reuse the same Pool-owned Resources or
retained conversation history within current access, without reviving the old attachments or
automatically inheriting their grants. Temporary transport failure while logical participation
remains intact recovers the existing instance. Unknown execution/cleanup state does not prove
that the flow ended or authorize duplicate work. Exact crash detection and detach mechanics
remain implementation work within these boundaries.

Participant attachment state describes the individual relationship; Resource-owned shared
authentication and connection support can have a different authority and lifetime. Template reuse
or joining alone does not merge private Agent context, credentials or access. Connection identity
is separate from process, transport and backing-Resource identity. Further authoritative-state,
recovery and composition details remain in [REVIEW.md](../archive/reviews/CONCEPTUAL.md).

### Composable relationship requirements

A Conduit template defines its relationship contract by composing reusable typed requirements.
Endpoint templates select the plugins that supply compatible behavior; endpoint instances manage
those implementations and local bindings. A Conduit instance realizes the configured relationship
between concrete participants. Plugins satisfy reusable requirement meanings rather than needing
to recognize every user-created Conduit template by name.

Provide a small base vocabulary and allow authors to define additional requirement types. These
are contract/authoring definitions, not a sixth object family, a separate deployment or a runtime
instance per condition. Requirement definitions can themselves compose existing requirements:
`review-access` could combine permitted read operations, a project restriction and a result-size
limit, then be reused by several Conduit templates. Every constituent retains its meaning and limits;
support is checked against the constituents rather than inferred from the bundle's name. No new
plugin code is needed when the selected endpoints support every part. New behavioral meanings still
need suitable implementation support. Publication does not install endpoint code, change fixed
definitions or grant access. Composition syntax and parameter binding remain open.

Each requirement has a reusable, versioned definition of its meaning, typed parameters and
applicability, independent of any single implementation. Multiple endpoint plugins can implement
that same definition. For example, two deployment integrations can support one allowed-regions
contract, while a Conduit supplies the permitted regions. Definitions and implementations may ship
together; independence does not require separate packages or duplicated schemas.

Endpoint plugins publish their supported requirement versions and endpoint roles, including known
limitations, in their inspectable contract bundles. Conduit checks that declared support against
the proposed relationship before establishment, without duplicating requirement schemas. For
example, a deployment plugin might enforce allowed regions for direct deployment requests but not
for an operation that selects a region indirectly. A declaration does not prove runtime fulfillment:
current authentication, readiness and enforcement still need the applicable runtime verification.
This does not change the rule that valid configuration may be registered while its runtime is unready.

Each requirement contract states when its guarantee must hold and any applicable endpoint-role
obligations. An established Conduit records the selected implementations responsible for fulfillment;
the Interface exposes these duties through existing connection/evidence state, without another
authored manifest. For example, a workspace restriction applies before each managed provider request,
while a result constraint may apply before delivery. The selected Resource-side MCP plugin can enforce
the former while the Agent-side plugin presents the permitted tools. These obligations do not impose
a fixed universal phase list. Exact representation, evidence and evaluation mechanisms remain open.

Composed requirements remain individually enforced in v0. Checks that detect known conflicts or
explain a combined effect are optional; a custom type need not supply a general solver or merge
algorithm. For example, result limits of 20 and 10 can both apply without creating a merged
requirement. Every applicable guarantee must still hold at its declared point; an unverifiable
required check keeps the affected behavior unavailable. Report known conflicts and do not present
unproved global compatibility as established. Exact diagnostic and optimization rules remain open.

Requirements express obligations and restrictions. Existing graphs coordinate declared procedures
to establish prerequisites, while instances own ongoing behavior. Resource configuration and plugin
input/output contracts remain; a relationship requirement does not redefine a target operation.
Typed filters are related narrowing requirements, with their existing whole-path access semantics.
Requirements cannot create permission grants or override baseline identity, ownership and lifecycle
rules. Registration still performs no provider effects; observed readiness remains distinct from
valid configuration. Detailed satisfaction/evidence and failure rules require implementation review.

The concrete core catalog, publication syntax, support-matching details, parameter bindings,
version identifiers, satisfaction evidence and enforcement-binding representation remain open. The
[implementation review](../archive/reviews/IMPLEMENTATION.md#i3--plugin-execution-and-distribution)
tracks those choices; accepting the compositional model does not select an expression language,
a universal solver or a worker topology. Experiment 002 separately records the accepted authoring proof.

### Composite Conduits

A reusable Conduit template can compose other Conduits. An Agent can declare one
`euphony-access` connection that establishes repository and Linear components. The composite is
itself a Conduit template/instance, while components retain distinct targets, authority and
lifecycle. Opening versus joining remains explicit; template reuse does not silently share a
connection or credential. This adds no family or required daemon. Supporting Resources, including
shared authentication and durable content, remain Pool-owned.

Composition must preserve caller attribution and restrictions at each component. It does not
combine provider identities or imply permission to use every component. Individual components
remain inspectable.

**Participation boundary:** each consumer's composite attachment tracks its own
participation in the components, making access, readiness and detach cleanup attributable to that
consumer. A leaving releases A's participation through the composite and preserves B's participation
and any independent attachments. Component Conduit instances, upstream clients and authentication
may still be shared explicitly; logical participation does not require duplicate provider login,
processes or Resources. Existing last-consumer and Resource-ownership rules continue to apply.

Exact attachment representation, component reuse selection, required/optional and readiness
aggregation, cleanup recovery and YAML remain open. Ready Pool Resources do not by themselves prove
that an Agent's required composite connection has completed preparation.

### Explicit joining and instantiation

Connection establishment supports both intents explicitly:

- **Join an existing instance:** establish participation in a specified existing relationship
  whose live flow has not ended.
- **Open a new connection:** create a relationship from its establishment configuration,
  subject to the contract's endpoints, authority and provisioning requirements. This can use a
  template or a complete direct definition; exact export/creation fields remain for the schema packet.

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
Resources; the engine and plugins execute and supervise Conduit flows. Their explicit dependencies
and required/optional failure behavior follow the [setup contract](LIFECYCLE.md#lifecycle-protocol); detailed bindings remain authoring work.
A connection cannot provision an unowned Resource or
assume the Agent's loading instructions merely because it requested storage.

### Typed filters

Conduit filters normally use typed declarations with defined semantics, validated
and enforced by supporting plugins. Capability or constraint contracts define the meaning of
restrictions such as permitted operations, paths, provider targets or control values. Configuration
selects the restriction and its parameters; implementation code performs the checks.

Users can implement new filter contracts through plugins, including scripts or graphs. Contract
identity remains separate from the implementation package. This preserves inspectable policy
without requiring each template to embed the checking procedure. Exact schemas, vocabulary,
expression operators and conformance tests remain open. Filters still only narrow
authority; unsupported restrictions must be reported rather than dropped. No additional v0
agent-wide interception or sandbox guarantee follows from this declaration format.

**Composition:** every applicable restriction along the path used for a request must
pass. Composite and component filters apply together with current authority and target limits;
a component cannot widen an enclosing restriction. Independent authorized paths remain complete
alternatives. Preserve target, operation and parameter constraints together: read access to
Euphony on one path plus update access to Demo on another does not authorize updating Euphony.
Actor-wide and target-wide restrictions still apply wherever relevant. This does not intersect
unrelated connections or grant access through a Resource's internal dependencies.
Boolean alternatives within a filter, conflict diagnostics and evaluation implementation remain
open; a policy engine may represent the rule without separate physical hops for each restriction.

**Routing:** if a request has no route already selected, automatically resolve it
only when one eligible route remains. Several eligible routes require an explicit choice at
attachment or invocation; that choice can be configured once and reused. Record the chosen path
and service identity, retaining complete path constraints. Do not select by discovery order or
silently fail over to another route/account after failure. Ordinary reconnection on the same path
is supported. An authorized caller may deliberately choose another route, subject to current
checks and safe-outcome handling; this does not authorize replaying an uncertain write.

### Authority and current access

**Policy uses the requirement model:** Policy is the authoritative application of reusable
requirement definitions/compositions. It records where guarantees and restrictions govern actors,
targets or relationships; it is not a separate executable rule category. The same supported
requirement can constrain a Resource, a Conduit or a proposed grant where its contract supports
that context. Plugins implement the checks and behavior, without a separate policy-plugin class.
Governing bindings are independent managed state with authorized checked changes. Template
declarations, installed plugins and satisfied requirements do not themselves issue authority.

**Explicit target scope:** a governing binding distinguishes an object's own capabilities/operations
from operations on other objects selected through ownership. A requirement on Pool provisioning
does not automatically govern deployment calls through its Resource instances. An explicit
owned-Resource target scope can select relevant capabilities where supported; it need not require
one binding per Resource. A Pool template may declare both kinds of rules together, but their
placement in that template does not determine their reach. V0 has no automatic Pool inheritance.
The access-management Resource stores/administers the bindings; that storage location is not
their target. Requirement applicability/support still matters, and grants still supply permission.
Explicit ownership-based governing scopes match live: newly matching Resource instances become
subject to the requirement at subsequent applicable checks, and instances that stop matching leave
that binding's reach. This neither provisions Resources nor issues grants; other bindings and
current access checks still apply. Explicit references remain bound to their resolved stable IDs.
Exact selector syntax and implementation mechanisms remain open.

Grant administration is an ordinary protected capability. For example, a coordinator's management
grant permits issuance, while requirements limit the proposed recipient grant's targets, operations
and parameters. Checking concrete use and checking proposed grant bounds need their own supported
contracts; no universal subset solver is assumed. This does not introduce automatic Pool inheritance
or another permit/deny language. Existing all-applicable-requirements, complete-path and mandatory
check rules govern evaluation and enforcement.

**Bounded issuance:** a grant-issuance operation may supply its mandatory use restrictions, combining
them with caller-added restrictions and returning the complete issued grant. The caller still
chooses intended recipient, Resource/participation and access under the typed operation; omissions
do not grant the administrator's maximum available authority. Explicit broader requests are refused,
and unsupported required checks cannot be assumed satisfied. A strict complete-proposal operation
may coexist, without requiring every caller to restate all bounds. Retained use restrictions belong
to the recipient's grant; issuance-only checks, such as the issuer's current administrative rights,
do not become live dependencies. Current governing requirements still apply. Exact schemas and
type-specific validation are authoring/conformance work; no universal subset solver is required.

Permissions name operations per capability/control contract, with convenience bundles expanding
into visible grants. Agent limits and explicit exclusions constrain effective access;
missing limits add no authority. Permitted operations require no extra confirmation merely
because permission rules exist. Resource and communication permissions do not imply each other.

**Recorded participation grants:** ordinary managed access requires a recorded grant on the
selected Conduit participation. This applies to Agents, Interfaces and other managed callers,
including the root Interface's broad management access. Policy governs grant issuance and current
use; it cannot directly authorize ordinary use without that grant. Authorized connection setup
can establish the grant as part of connecting, with no separate user-authored manifest or grant
per invocation. A grant can cover many requests. Current permission, complete-path restrictions
and required checks still apply, and a retained record is not an offline permission guarantee.
Initial root authority and grant establishment follow the accepted
[foundational setup contract](MODEL.md#interfaces-and-the-user-workflow);
calling an instance root or copying its configuration cannot issue authority.

**Delegation boundary:** an authorized user or Agent may delegate an explicit subset of
service access when establishing another consumer's Conduit attachment. The attachment records
the delegation; no separate prior Agent-permission setup is required. The creator must have
authority to delegate that access, subject to target policy and recipient limits. For example,
a user can connect a reviewer to shared Linear access with read-only authority in one action.

**Independent issued grants:** check the administrator's current authority when issuing, changing
or revoking a grant, including the permitted target and access extent. Once issued, the grant is
not dependent on its issuer's continuing permissions, execution, connection or grant ancestry.
Losing or ending the issuer's access does not cascade into previously issued grants. Any Interface
or Agent currently authorized to administer the grant may revoke it; original-issuer permission
is not required. Ending the recipient's logical participation or the related Conduit ends its
participation access. Current governing policy and requirements still constrain use.
Normal operation evidence can attribute grant administration, but no separate provenance chain is
required for authorization and provenance is not a condition of grant validity. Knowledge-source
dependencies and Resource readiness retain their distinct contracts.

Keep requested access, the authorized delegation and any additional filters distinguishable.
Authored configuration requests access; it does not issue its own grant. Filters only narrow
granted authority. This adds no permission-registry object family and does not implicitly grant
onward-delegation rights. Temporary socket loss or frontend closure does not itself end logical
participation. Withdrawal and current governing restrictions continue to apply at dispatch and
delivery. This keeps recipient work independent of the UI/process that established its access.
Exact issuance, attribution and revocation representations remain to be drafted.

Selection supports IDs, owning contexts, defined typed fields and path patterns where the
integration supports them. Record classification can narrow knowledge queries under its own
contract; there is no generic registry-object label selector. Matches within established access
rules are live. Authorized
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

### Authority across dependent calls

A Resource serving an Agent or other managed caller may use its own authorized dependency
connections. Preserve the authenticated originating caller and enforce that caller's applicable
restrictions on requested effects and returned content alongside the Resource's dependency grants.
The caller does not need direct access to every internal backing service merely to use an authorized
capability. Shared credentials and broader internal access cannot widen what that caller may cause
or receive through the operation.

For example, a search Resource may index both Euphony and Demo under its own authority; an
Euphony-only Agent cannot use its search connection to retrieve Demo content. A provider-facing
Resource can use shared authentication without handing its credential or raw backing access to
the Agent. Current access, source-dependency and delivery checks retain their existing meaning.

Autonomous maintenance, such as scheduled indexing, uses the Resource's own authority. Capability
contracts distinguish that activity from work serving a caller and define the applicable enforcement
and evidence. Running caller-requested work asynchronously does not by itself remove its restrictions.
Plugins must support the necessary checks; an unsupported required check keeps the affected behavior
unavailable. Authenticated caller context is not a self-asserted argument or automatic token forwarding.

This rule does not create live issuer-ancestry dependencies for independently issued grants, require
one universal identity-chain format, or authorize a dependency call merely because setup succeeded.
Exact context propagation and implementation duties are defined by the supported contracts.

### Protocol and provider compatibility

MCP is the preferred external capability contract. Use standard tools, resources, prompts
and supported extensions where applicable; native adapters address concrete runtime,
account, lifecycle and non-MCP needs. Internal Rust modules need not be MCP servers.

MCP is also the accepted base for public plugin invocation. A small versioned Conduit profile
supplies the instance/operation context and lifecycle/recovery semantics needed by managed
implementations. Exact bindings, transports and execution placement remain separate choices.
Existing provider MCP servers remain usable through adapters without implementing that profile.
This does not turn every provider tool into a setup operation or require one server per object.

**Catalog boundary:** an integration capability contract can define discovery and
invocation while concrete provider operations and their schemas are discovered dynamically.
Conduit does not require its own contract definition for every Linear/MCP tool. The Agent sees
permitted provider-native tools, not a mandatory generic invocation wrapper. Fixed operation
contracts remain appropriate for other capabilities; dynamic catalogs are supported, not required
everywhere. Provider schemas remain part of validation rather than becoming unchecked arguments.

Discovery does not grant authority or silently alter a running instance's control envelope.
Current grants, Conduit filters and target constraints still govern exposure and dispatch.

**V0 refresh boundary:** discover permitted native tools during supported preparation
and explicit refresh. Dynamic discovery does not require automatic replacement of the Agent's
exposed catalog. A changed issue or document is service data, not necessarily a changed tool
contract. A new name, changed schema or relevant service-configuration change can invalidate an
exposed definition; the integration identifies the affected surface using evidence it supports.

Before dispatch, apply current access checks and available compatibility/change evidence. If an
affected definition is known stale/incompatible, or a detected change leaves compatibility unresolved,
withhold that call and report the need for explicit refresh/reconciliation. Refresh supplies the
currently permitted definitions; the caller can then reconsider its request against them. Unaffected
usable operations need not stop. Do not automatically rewrite arguments or replay an uncertain write.

This does not require a network catalog fetch on every invocation or promise advance detection from
every provider. A provider rejection may be the first evidence of drift; report known gaps and avoid
claiming freshness that the integration cannot establish. Detailed invalidation/version evidence,
cache behavior and future first-use/background refresh procedures remain integration work. Future
automation must preserve current authorization, supported activation boundaries and safe retry.

Resource templates for MCP services describe reusable endpoint/process/transport/authentication
setup. Pool-owned instances and connection attachments realize that setup under current access.
Use the [shared Pool access contract](POOLS.md#shared-access-and-authentication), with an explicit
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

### Communication contract

Communication uses common mechanics and explicit, composable capability guarantees carried by
Conduits. Invocation, retention and observation overlap: a retained post is itself an invocation,
and an exposed work operation can retain its request/result and support progress inspection.
They are not an exclusive taxonomy, separate wire protocols or new object families. MCP remains
the preferred external expression where supported; its transport/extension support does not
automatically provide application retention, safe replay or Agent scheduling.

The capability defines what its responses establish. Distinguish transport receipt, storage,
delivery and execution/completion evidence; do not infer that a model read or understood content
from an integration receiving it. Report gaps where native runtimes cannot supply evidence.
This requires no exhaustive domain-state enum, mandatory receipt at every stage or separate
user-maintained ledger. Reuse operation/runtime evidence. Pools own supporting durable Resources;
Agents own private execution/context; Interfaces inspect and invoke through authorized Conduits.

The first communication experiment includes retained Agent messages and state inspection.
Structured asynchronous work submission is deferred, including a generalized contract for job
acceptance, progress, results and cancellation. An Agent can still ask another to review in a
message, and ordinary tools, existing named controls and their operation outcomes remain available.
A request in message prose is not itself a grant or a structured job-acceptance receipt.

The first retained-message capability uses explicitly participating Agent instances and one
shared append-only exchange history, with an independent read position per Agent. Two participants
support direct conversation; additional participants support group discussion. A Pool-owned
Resource retains the history. Current-access checks and bounded catch-up apply on reading;
participation does not itself widen access. One Agent's reading does not advance another's position.
Read progress is not evidence of understanding or execution. Ordinary message posting appends
content; explicit purge and retained-data lifecycle rules remain separate. This selects one
communication capability, not a universal room model for all Conduits or a Resource per message.

New messages normally produce a brief unread notice at the Agent's next normal turn; the Agent
retrieves content explicitly. Posting, replay and notices do not automatically wake an idle Agent
or start model work. Integrations report whether they can supply the notice and any coverage gaps;
unsupported integrations must not claim notification occurred. Existing explicit reads remain
available. An optional per-Agent automatic-wake hook is a future extension. It does not inherit
authorization from ordinary posting or read access. Existing pause/resume rules still apply.

Issue-management integrations and specialized tracking remain ordinary Resource capabilities, with
no required tracker coupling. The first acceptance work is local, progressing from a two-Agent and
Interface walkthrough to the required two-Claude/two-Codex account compatibility checks. Remote
communication remains later work. The retained-history and attention rules above are the selected
first capability, not a universal topology for every Conduit.
The [protocol investigation](../research/CONDUITS.md#messaging-and-networking-investigation) supplies
source evidence and failure examples; broader work-submission examples are future capabilities,
not requirements for the first slice. Concrete fields and transport selection follow those choices.
