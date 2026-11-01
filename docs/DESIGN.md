# Conduit design contracts

Updated: 2026-11-01. Current working design; implementation
has not started. [SPEC.md](SPEC.md) summarizes the model; [REVIEW.md](REVIEW.md) records the
completed direction review and schema boundary. The [YAML walkthrough](../experiments/001-shared-access/README.md)
proposes concrete syntax. Research supplies supporting evidence, not extra requirements.

Prefer minimal bookkeeping: retain authoritative configuration, identity, authority and necessary
operation evidence; derive views rather than maintain parallel editable descriptions. Shared
contracts need not impose the same lifecycle, process model or implementation on every type.

**Implementation priority:** plugin and template authoring are central to the protocol and the
first implementation's success. Evaluate runtime choices by whether an author can define a small
extension, compose it into templates, establish instances and inspect their behavior without
modifying the core. Existing infrastructure can implement these boundaries; early efficiency is
secondary to correct, enforced contracts and a usable authoring path. This direction does not
approve particular SDKs, plugin packaging, deployment topology or schema spellings. Those rulings
remain collaborative in the implementation review.

**Accepted authoring direction:** start with extremely lightweight Conduit support and author
real plugins against the public contracts to validate the approach and speed development.
Prioritize the protocol and extension structure over a library of helpers. Existing protocol
libraries and small adapters may supply plumbing; richer SDKs can grow from repeated needs.
Plugins still supply their applicable behavior, effects and recovery semantics. This settles
implementation priority, not package syntax, supported languages, transports or process topology.

**Plugin publication:** publish inspectable contracts with their implementation bindings. Conduit can
inspect exports and validate typed configuration/graph bindings without starting plugin code;
actual availability, authority and behavioral conformance still require their respective checks.
Provider-native catalogs can remain dynamic behind an integration's published contract. Detailed
contract/template syntax, package layout and authored-versus-generated descriptors are deferred
until the remaining conceptual questions are settled.

**Primary access model:** establish capabilities and service authentication through a Pool once,
then let authorized Agents use them through individually restricted Conduits. V0 Interfaces
provide unrestricted local administration; differentiated Interface permissions are deferred.
Sharing service access is the normal path, not an optional optimization of per-Agent logins.

## Families, templates and instances

The five families are **Interface, Agent, Resource, Pool and Conduit**, each with template and
instance forms. Templates are reusable definitions; one can establish many independent instances.
An instance need not run a dedicated process. Plugin nodes, executions and attachment records are
supporting concepts, not additional families.

| Family | Template defines | Instance represents |
| --- | --- | --- |
| Interface | Unified control and observability, including authoring, creation and connection capabilities. | The user's interaction and inspection endpoint, presented through a CLI, terminal or other frontend. |
| Agent | Runtime/account/model configuration, activation, connections and supported controls. | An autonomous participant with private context, concrete configuration and executions. |
| Resource | Typed creation/attachment, ongoing capability and supported controls. | One unitary capability with exactly one owning Pool. |
| Pool | Resource composition, shared service-access setup and supported management controls. | Owns its Resource instances and organizes their authentication/access lifecycle. |
| Conduit | A relationship contract composed from typed requirements, endpoints, setup and supported controls. | A connection with concrete participants, filtering and connection state. |

Resource creation/attachment always goes through its owning Pool, including supporting Resources
for Conduits. Templates are reusable across Pools; each resulting Resource instance has one owner.
Backing data, credentials and process lifetime are distinct from registry identity/ownership.

### Authoring concepts beneath the object families

The five families describe the objects Conduit manages. The authoring and extension concepts below
describe how those objects are defined, connected and operated. They are not competing object
families or additional independently provisioned things. These divisions and temporary names are
accepted as our working vocabulary. Names may change after conceptual review; the final naming
and schema pass should preserve these responsibilities rather than silently change their meaning.

| Working term | Responsibility | How it composes |
| --- | --- | --- |
| Contract | An inspectable agreement about meaning, typed values and behavior. Operation, capability, type and requirement contracts describe different aspects of an extension. | Definitions reference other definitions rather than duplicating their meaning. A schema describes valid data; the behavioral contract also explains what it means. |
| Capability | What an endpoint offers, including supported operations and behavior. | An endpoint can offer several capabilities; a relationship selects compatible ones. |
| Requirement | What must hold for a particular relationship or its use, with typed parameters and applicability. This can include compatibility or lifecycle obligations as well as restrictions. | A reusable definition can compose existing requirements. Every constituent retains its meaning and limits; support is checked against the constituents. |
| Operation | A callable action with typed inputs/outputs and applicable effect, completion and recovery semantics. | Graph nodes select operations and bind values. One plugin can supply several operations. |
| Graph | A finite dependency structure describing a procedure. It can implement a reusable typed operation as well as template setup or a supported change. | Nodes call plugin operations, including graph-backed operations; outputs and explicit dependencies order the work. |
| Plugin | A reusable contribution publishing contracts and implementation bindings, backed by authored code, existing MCP services, graphs of operations, or a combination. | A plugin can publish a composite operation implemented by a graph of other plugins' operations. This needs no new object family, process or plugin-specific executable merely for composition. |

A template assembles these definitions for one of the five families: configuration, selected
implementations, setup/change procedures, ongoing runtime behavior and supported controls. Its
instances realize that definition with identity, state and managed activity. The template bridges
authoring and the managed object model; it is not merely a graph. Ongoing services remain under
instance management rather than belonging to a finite setup node.

There are two distinct forms of composition. Requirements combine obligations; graphs combine
actions and their dependencies. A graph may establish prerequisites for a requirement, but
successfully finishing setup does not prove that every later request satisfies it. Conversely, a
requirement declaration does not itself run an authentication or connection procedure. Plugins can
supply the behavior for both, without collapsing declaration, execution and ownership together.
An operation can also be invoked directly through its supported surface; ordinary tool calls do
not need a new authored graph merely to be callable.

A Conduit uses both forms: its contract composes relationship requirements and its procedures
coordinate establishment through selected endpoint capabilities. Its instance represents the live
relationship. It is neither the universal name for any graph nor the package that supplies endpoint
code. The [Linear walkthrough](proposals/implementation-v0/AUTHORING.md#one-linear-connection-through-the-authoring-concepts)
illustrates these boundaries.

**Reusable executable composition:** a graph may implement a typed operation contract and be
published through a plugin. Another graph or permitted caller invokes that exported operation
through the normal operation contract. From the author's perspective this is a composite plugin;
the graph describes its implementation, while the plugin publishes its contract and binding. A
plugin can export several operations, and a template-local graph need not become a published plugin.
No special composite-plugin family or mandatory packaging unit is introduced.

The exported contract describes inputs, outputs and applicable behavior; internal nodes bind those
values and call selected operations. Composition preserves existing authority, endpoint selection,
implementation retention and instance-owned runtime rules. It does not automatically make several
external effects atomic or safe to retry. V0's finite dependency-graph boundary still applies;
nesting does not introduce authored recursion or loops. Exact graph-call/binding syntax, publication
layout and composite outcome/recovery details remain open.

Composite output contracts account for their composed success and failure modes. Derive the
initial public result contract by default from declared public output bindings and constituent
contracts; authors may explicitly construct a domain result instead. Preserve relevant effects,
uncertainty and links to scoped evidence without automatically exporting internal outputs. Public
results and retained execution evidence serve different purposes. Detailed derivation/mapping
mechanics and recovery coordination remain under review in the
[composite output proposal](proposals/implementation-v0/CONTRACTS.md#composite-outputs-and-outcomes--proposal).

### Distribution and execution

**Distribution direction:** objects should operate independently and eventually be deployable
across systems. One Pool can own Resources deployed on different systems; Conduits can span a
network. Pool ownership is not a placement or process-parenting rule. A unified Interface does
not require one execution host. The authority split and v0 outage rule below are accepted;
the first proof uses separate processes on one machine, as recorded in
[implementation review I1](proposals/implementation-v0/REVIEW.md#i1--independent-execution-and-management).
Remote deployment guarantees still require separate validation.
This direction does not claim remote execution is implemented or approve automatic migration,
federation or offline authorization.

**First implementation arrangement:** use separate management and execution services. Execution
services supervise Conduit-managed Agents and Resources; management coordinates configuration
and control; the Interface presents the system. Each executor can host multiple instances,
including on the same machine initially. External MCP services retain their own supervision.
This settles execution custody without introducing an object family or requiring one process
per object.

**First independence proof:** run management and two independent executors as separate processes
on one machine, with controlled failures of their connections. Exercise plugin/template authoring,
instance-owned state, recovery and the no-offline-grants rule through serialized service boundaries.
Validate across two machines before claiming remote support. This selects the first experiment's
layout, not a production placement limit or proof that failure guarantees have already been met.
Storage, worker packaging and exact transports remain implementation review choices.

### Shared policy and instance-owned state

Management is authoritative for templates, registration/naming and shared access policy. One
designated service is authoritative for each instance's resolved definition, confirmed
configuration/revision and operation results. The responsible service can host many instances;
management can host passive instance authorities and executors active ones. This state authority
is distinct from a Resource's logical Pool ownership and from a plugin implementing a handler.

Keep one authoritative instance configuration. When a supported control confirms application,
its responsible service records the confirmed settings, revision and outcome. Pending or uncertain
changes remain separate. Other services can retain observations, but cannot independently edit
their copies or substitute a stale view for the authoritative revision. Resource backends remain
authoritative for their domain data; a Conduit client Resource does not become the authority for
an external service's internal state.

The Interface combines these sources with identifiable freshness and gaps. A current instance
read/export must distinguish its authoritative source from an older aggregate observation. The
single logical registry remains the naming and configuration surface; it does not require all
facts to share one database. Registration, renames, retirement and other cross-authority operations
retain their existing ID/version and lifecycle checks; coordination mechanics remain implementation
work. No distributed transaction or automatic transfer of an unavailable authority is implied.

For example, if a control changes a search limit from 10 to 15, the instance service can retain
15 and its result even if the response to management is lost. The Interface recovers that result
instead of reapplying the change. If the external effect occurred before confirmation was
recorded, its outcome remains uncertain under the existing recovery contract.

### V0 outage admission

V0 provides no offline grants. Keep surviving instances supervised and retain outcomes of
already-dispatched operations, which may finish within their admitted contracts. Hold new
managed requests when current permission cannot be established. Work with verifiable permission
can continue; existing explicit stop, withdrawal and isolation rules still apply. Starting an
Agent or graph does not authorize every later tool call or node.

Retaining a result does not authorize its delivery: managed retrieval and delivery recheck current
access. A blocked request may remain pending or report temporary unavailability according to its
contract; this ruling does not require indefinite queues or automatic replay. Reconnection does
not revive withdrawn access or make uncertain effects safe to repeat.

An unavailable frontend, registration service, policy service, provider and executor are distinct
failures. If management and policy share a process, its failure can block new managed capability
calls even while execution and outcome recording continue. A live connection, shared service
credential or cached grant cannot replace current authorization. Permission checks can be local
where the relevant authority is available; no external-network check is inherently required.
Delegated participation still survives its issuing frontend or coordinator stopping, as agreed;
that is different from admitting work without verifiable governing authority.

Bounded offline grants are deferred. Exact process placement, persistence and demonstrated
crash-survival guarantees remain implementation work. Native actions outside Conduit's mediation
retain their actual enforcement limits; preserving a process is not a sandbox guarantee.

### Plugins and independently accessible MCP components

A plugin publishes reusable contracts and explains how they are fulfilled. Its implementation
binding can select authored code, an existing local/remote MCP service, a graph composing selected
operations, or a combination. No new plugin-specific executable is required when existing bindings,
adapters or graph composition can supply the behavior. Plugins, packages and adapters are not
additional object families.
Templates configure and compose contributions; instances realize them. Ordinary endpoint/account
configuration for an existing MCP-access type does not itself require a new plugin or Resource type.

**Endpoint implementation selection:** endpoint templates select the plugins that supply their
supported connection capabilities. Instances retain and manage those implementations and local
bindings. Conduit templates declare the compatible capabilities/roles they need and coordinate
connection policy and establishment; opening a Conduit does not select or install new plugin code
at an endpoint. New parameters and supported dynamic provider catalogs can use existing contracts;
changing fixed implementation definitions still requires a replacement instance in v0.

Published contracts remain inspectable without executing code. A standard adapter can supply
discovery, invocation and Conduit-managed context/evidence while provider-native catalogs stay
dynamic. A declaration cannot manufacture unsupported effects, lifecycle, recovery or filtering
guarantees. Where declared semantics require additional behavior, supply an implementation or
expose the supported contract instead. Exact binding syntax and any supported transformations
are deferred; this does not introduce a second graph or programming language.

Configured capabilities remain Resources with one owning Pool. A search Resource can use a
separate MCP-access Resource through a Conduit; code-optional binding does not bypass that
dependency. Reusable service authentication stays Pool-owned. Code/package discovery neither
creates instances nor grants service access. Attaching an external service gives the Resource
its configured association and client lifecycle, not ownership of the provider's infrastructure;
shutdown or deletion of backing services still requires supported explicit lifecycle operations.

Components can expose their supported capabilities, inspection and controls independently over
MCP; the Interface supplies the aggregate view rather than the exclusive invocation gateway.
An independently addressable surface need not own a process or expose internal setup/driver
operations. Public capabilities remain distinguishable from management exports. Ordinary MCP
clients can use supported public surfaces through the applicable authentication/access binding;
they need not implement Conduit's graph engine merely to invoke a tool.

Independent use has two boundaries. An implementation can serve ordinary MCP outside Conduit
with the dependencies and guarantees it supplies. A Conduit-managed component retains its Pool
ownership where applicable, current Conduit access, supported controls and operation evidence,
even without an attached Interface frontend. Its minimal authority/runtime dependencies remain
necessary. This permits using parts of the system without requiring every frontend, Agent driver
or integration, but does not promise that every component can run without its declared dependencies.
Separate execution services own managed activity in the first implementation, with the shared
policy and instance-state authorities defined above. Outage behavior, concrete placement and
client enrollment remain review questions; protocol accessibility alone does not settle them.

### Interfaces and the user workflow

Control and observability are inseparable parts of the Interface contract. The same surface that
authors, starts, connects or changes instances exposes their state, activity and outcomes. This
includes Agent-initiated and Resource-to-Resource activity, not only commands issued by a frontend.
CLI, terminal and test frontends present the same underlying observation/control capabilities.

**V0 authority:** Interfaces, especially the default Interface, are the user's unrestricted
local administration surface. They can inspect and administer every template and instance across
all five families: authoring, creation, connections, configuration, lifecycle and repair. Narrower
Interface scopes and differentiated authority across multiple Interfaces are deferred while the
rest of the protocol is developed. This changes Interface permissions, not delegated Agent access.
Normal interactions use Conduits; supported runtime behavior determines how changes take effect.
Changing fixed setup graphs or control definitions requires a replacement instance in v0,
including through this unrestricted Interface. Supported settings, connections and lifecycle
operations remain available.

The user authors templates, adds Resource-template references to a Pool configuration, establishes
the Pool, its Resources and shared service authentication, then starts Agents with Conduits to
those Pools. New attachments use the established capability without repeating provider login.
The Interface can
open further Conduits to inspect supported controls and request changes. Agents use that same
interaction contract on other targets. A frontend presents it rather than independently performing
provider effects. The Interface identifies the endpoint; the acting human identity remains distinct.

CLI, terminal UI and test clients exercise the same administration contract. Future Interface
scoping can narrow this authority; designing it is not a prerequisite for v0.

**Frontend relationship:** the default Interface is a reusable logical instance. CLI,
terminal and test clients attach to it, with multiple clients supported at once. Each attachment
remains identifiable. Owner-operated frontends share the unrestricted v0 administration authority.
Another Interface can be instantiated for a different configuration or endpoint; its future access
policy is deferred. Opening a frontend need not create an Interface object. Direct Agent control
is shared across its controlling Interface's frontends as described under runtime/presentation.
This does not require an always-running process or give Agents the user's administrative identity.

**Bootstrap:** initial local startup seeds a minimal system Pool owning a management
Resource, a default Interface and a management Conduit connecting them. A small runtime bootstrap
establishes this initial set before its operations can be invoked. The management Resource exposes
core template/registry management and instance-creation capabilities through the same five-family
model. Resource creation still goes through its owning Pool. No application Pool, Agent or external
service login is required first; this does not introduce a second registry or require a new daemon.
Built-in definitions, exact management operations and frontend transport/lifecycle remain open.

**Later startup:** reuse the registered system Pool, management Resource and default
Interface with their existing identities, saved settings and grants, subject to current access
checks. Missing or inconsistent expected components are reported for explicitly requested repair;
ordinary startup does not recreate them from defaults or silently retarget established bindings.
Preserve available records and backing data. An unavailable registry or service is not evidence
of deletion or a fresh installation. Supported reconnection to the same targets is distinct from
replacing registered objects. Repair is a deliberately requested operation, not an additional
confirmation on every launch. Repair is part of unrestricted Interface administration. Its route
when the normal management path is unavailable, exact steps and partial initial-setup recovery
remain open. Differentiated recovery authority and Interface restrictions are deferred; this
rule does not preserve ended flows or automatically restart Agent work.
Later access after a management Conduit fully ends opens a new Conduit instance to
the retained system objects. Transport recovery while logical participation remains intact
continues the existing instance; reopening a terminal alone is not evidence that a flow ended.

**Initial authority:** v0 establishes one local registry owner using the initializing
OS-user context, without separate Conduit login. The default Interface acts for that owner;
Agents receive explicit delegated access through their own Conduits. Owners can delegate broad
or narrow access under the existing grant model. Running an Agent under the owner's OS account
does not itself constitute a managed grant. Exact owner/client recognition, credential handling
and native-versus-isolated enforcement remain separate; this authority rule is not proof of
isolation between processes with the same host privileges.

### Interface observability

V0 requires a durable, inspectable local observability foundation with pluggable export, exposed
through the Interface as a core capability. Follow Conduit-managed setup steps, controls, access
decisions, messages, tool calls and failures through their actor, instance, execution and operation
identities. Include available model/subagent activity and usage. Users can move from an observed
failure to the affected instance, its configuration and supported controls through the same Interface.
Exact event fields and exporter formats are drafting work; an external dashboard/backend is not
required for the first experiment.

Managed components produce evidence even while no frontend is attached. Retention is independent
of a frontend's lifetime, and reconnecting frontends inspect the same underlying history/state.
The Interface is the unified access and interaction surface, not the sole producer of observations
or a requirement for an always-running UI. A dropped observation stream does not cancel Agent work.
Report gaps and stale/unknown state; a latest-state view need not replay every intermediate update.

Export failure must not erase local committed outcomes. Diagnostic traces and metrics can refer
to retained evidence without becoming its sole copy. Preserve applicable content access and secret
handling; pluggable export does not imply putting credentials or every full payload into traces.
Coverage includes what Conduit manages and what native integrations actually report, with missing
coverage visible. Specialized domain tracking can be supplied by custom Resources without changing
this common foundation or introducing a new family. Linear/Jira integration is independent of it.

### Establishment contracts and instance state

Templates contain the complete instructions needed to establish an instance and
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
distinguishable. Configuration changes advance stored instance settings only when
the handler confirms application. Pending or uncertain requests remain separately recorded in
operation evidence. Plugins define what establishes application and describe any partial effects.
A last confirmed setting must not be presented as still active when the outcome is unknown.
Initial creation settings remain establishment inputs, not evidence that setup succeeded.

**Configuration representation:** `spec.config` is the instance's single stored
configuration map. It contains materialized initial values before setup and advances affected
values only after a supported control confirms application. Fixed setup/control definitions stay
elsewhere in the spec; observations and outstanding requests remain separate. Field placement does
not grant mutability. No second original/desired/applied configuration map or required template
history is introduced. Exact observation/operation-evidence fields and physical storage remain open.

A fixed graph can read current settings on later explicit starts under the type's restart contract.
Revalidate recorded setup progress against those inputs before reusing it. For example, a confirmed
query-limit change from 10 to 15 makes 15 the instance's stored setting; its unchanged template still
starts new instances at 10. A pending or uncertain change does not overwrite the last confirmed
setting or establish what the runtime is currently doing.

Registration has no provisioning/model-execution effects. An explicit action may combine creation
and start, reporting their outcomes separately. Resources require a Pool owner. Direct creation
from a complete definition remains possible; exports/imports must report missing dependencies
or establishment intent rather than inventing it. Current schema/default rules still apply.

### Template-defined configuration changes

**Control surface:** templates expose permitted configuration changes as named typed
controls backed by handlers. A control such as `set-query-limit` accepts typed inputs, applies
the supported change and reports its outcome. One control can cover several related settings;
this does not itself guarantee atomic application. Templates normally reuse their selected type's
declared default controls, including reusable schemas and implementations. They may narrow that
surface or explicitly select other supported controls. The instance retains the resolved surface;
a plugin update does not silently add controls. Type defaults are distinct from every internal
plugin operation and from the dynamic provider-native catalogs exposed by integration capabilities.

Frontends may present ordinary field editing and invoke the corresponding control through an
authorized Conduit. The interaction does not require a separate user-authored operation manifest
or make all instance fields writable. A declaration cannot make an unsupported runtime operation
work or grant caller authority. Exact default-selection notation and field/handler bindings remain open.

The following lists **shared contract concerns**, not a mandatory field list or exhaustive
application-state enum. MCP is the accepted standard external expression of this open-ended
communication pattern. Shared versioned type definitions can avoid copying
schemas into every template; the resolved template/instance must make its effective contract visible.

| Concern | Meaning to expose |
| --- | --- |
| Configurable surface | Which named controls are exposed, their types, and which configuration they affect. |
| Value constraints | Allowed values, ranges, units, shapes and related-field conditions. Static limits differ from runtime-discovered eligible choices. |
| Transition conditions | Whether this change is valid from the current state, including readiness, active work and other prerequisites. Valid endpoints alone do not prove a valid transition. |
| Application behavior | Plugin-defined requirements, effects and continuations implemented by the supported handler. Examples such as waiting or restarting do not form a closed Conduit taxonomy. |
| Results and observations | What proves application, how progress/failure/unknown outcomes appear, and what clients can inspect afterward. |
| Recovery and retention | Supported retry/cancel/reconciliation and what applied settings persist across restarts. No universal queue or rollback policy is implied. |

**Exchange:** expose named controls as discoverable MCP tools with typed inputs,
explanatory content and plugin-defined structured results. Reuse supported standard mechanisms for
progress, additional input, observation and long-running work where applicable. MCP's wire/result
types do not require every plugin's domain lifecycle to fit a shared enum. Plugins define domain
behavior; Conduit preserves attribution, current authority and evidence of effects. Follow-up calls
remain authorized operations, not authority supplied by the preceding result.

Generic Interfaces can render the exchange; richer automation can interpret a plugin's schema.
No mandatory combined MCP server or MCP boundary between internal Rust components follows.
Exact mappings, optional-feature fallbacks and client compatibility remain in
[review](REVIEW.md#next-discussion). Confirmed settings and outstanding requests remain separate
under the accepted instance-state rule; accepting a request does not itself select a queue or
automatic reconciliation policy.

**Coordination:** configuration changes apply one at a time per target instance by
default, across all authorized Conduits to it. The type/plugin contract can declare safe parallel
application. Ordinary Resource operations retain their own concurrency rules. Whether a busy
target queues or rejects another change remains plugin-defined. Serial application does not
by itself prevent a later request from relying on stale settings or make external effects atomic.
Type-specific conflict domains and reconciliation mechanics must be specified by the implementing contracts.

**State check:** configuration controls normally require the caller's expected
configuration revision, checked when applying the change. A stale request reports a conflict;
the caller can reread and reconcile within its authority. Control contracts may explicitly permit
safe unconditional operations. Clients can carry the revision from inspection without mandatory
human confirmation or a separate authored request document. Merely substituting the latest
revision is not reconciliation. Exact revision granularity, MCP fields and coordination with safe
parallelism remain open; checks must be coordinated with application rather than only discovery
or enqueueing, and a local revision cannot establish unseen provider state.

A template may constrain the values and controls it exposes within its implementation's support.
The accepted access-path rule intersects applicable type support, template limits, current authority
and Conduit filters; none overrides another to widen access. Exact authoring shorthand/defaults,
cross-field expressions, handler composition and version compatibility remain open.

A setting's value and the rule governing its permitted changes are distinct. Changing a value
must not silently expand the control's allowed range or caller privileges. **For v0:**
an instance's resolved setup graph and control definitions remain fixed. Changing setup steps,
control schemas/constraints or handler bindings requires a new instance from a revised definition.
Existing controls still change their permitted settings; dynamic attachments and current access
rules remain live. Provider catalogs retain dynamic discovery, with explicit exposure refresh under
R5. In-place definition upgrades are deferred.
R6 also retains exact Conduit-selected implementations and requires replacement instances for their
upgrades. A new instance does not automatically acquire the old one's bindings, private context or permissions.
R1 confirms that this applies to Interface administration as well as Agent requests. Broad
administrative authority does not introduce an in-place definition-upgrade mechanism. The
Interface can revise templates, create replacement instances and explicitly reconnect consumers.
A replacement may explicitly reuse existing backing data where the type supports attachment;
replacement does not itself authorize backing-data deletion. This keeps the interpretation of
running work, retained setup progress and control handlers fixed without requiring upgrade
migration rules in v0.

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
appropriate management connection. The initial connection is established by the accepted system
bootstrap. The v0 Interface has unrestricted administration; Agents retain explicit delegated
access. Runtime recovery and supported changes to live management access remain open, while
Interface permission scoping is deferred.

Configuration changes remain distinct from Resource-content writes, lifecycle actions and
connection membership, even if they share the invocation protocol. Adding a document does not
edit a storage Resource's configuration. Observing state does not start model work. Attaching
another Conduit does not silently rewrite an Agent's template or broaden a control's constraints.

### Lifecycle protocol

**Graph boundary:** v0 setup and change-handler graphs are finite, acyclic dependency
graphs of typed plugin operations. Plugins declare typed inputs/outputs; graph nodes bind them.
Output bindings imply dependencies; explicit prerequisites cover ordering without data. Branching,
loops and provider-specific procedure remain inside plugins initially. User-authored conditions,
iteration and dynamic graph fan-out are deferred. Independent steps may run concurrently where
supported; interaction waits and valid-progress recovery do not require authored loops.

For example, authenticate → connect → initialize remains visible composition, while the
authentication plugin handles an existing-login check, challenges and provider polling. The
reusable Pool composition step can establish declared entries through their own setup contracts;
authors need not write a loop. Plugins still obey current authority and safe recovery contracts.
This choice limits execution-language complexity without restricting the network of ongoing
Conduit connections to an acyclic topology or giving setup nodes ownership of live services.

Per-output publication timing and validity/lifetime are declared separately. A graph runs each
instance start; plugins may verify already-satisfied work. This does not authorize repeating
uncertain effects.

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
| `metadata` | Author-editable naming and system-owned identity/version information, with ownership defined per field. No generic object-label map in v0. |
| `spec` | Configuration defined by the family and form's contract. |

Metadata ownership is defined per field: authors control naming within their
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

**Instance export:** export the current stored `spec.config` with the retained fixed
definitions and explicit dependencies. Report outstanding requests/uncertain observations alongside
the export, without treating them as authored values. The configuration is an input for fresh creation,
not proof of live state. Do not copy credentials, grants, readiness, live handles or execution progress.
Account/Resource references remain explicit and resolve under current authority on fresh use.

For example, an instance whose query limit was confirmed as 15 exports 15; the unchanged source
template can still export 10. Fresh creation from the instance export uses a new identity and unique
name, and Resource creation still goes through an owning Pool. Backing create/attach intent remains
explicit; report creation-only settings that need editing rather than silently changing intent or
copying an established binding. The export does not resume the source instance's execution.

The shared envelope does not give every kind the same configuration or status fields.
Exact metadata/status fields, ID encoding and remaining import/export semantics are open.

This accepts the envelope and organizing principle, not every proposed field or algorithm
in the research. In particular, the API group/version, registry storage layout, reference
extensions, schema toolchain and command names remain to be drafted. The v0 YAML interpretation
profile is accepted below. Configuration objects also do not force every memory record, native tool or runtime event into an
individually authored YAML file.

### YAML interpretation

**V0 profile:** use YAML 1.2 Core scalar interpretation, restricted to string
mapping keys and finite supported numeric values. Typed schemas enforce field types, ranges and
units without silently converting a string to a number or boolean. Reject duplicate mapping keys
and unknown fields outside schema-declared open maps. Unsupported numeric range/precision is an
error, not permission to truncate. Equivalent JSON/YAML inputs have the same typed meaning.

Omission follows the declared default/optional rule; instances materialize declared defaults.
Null is an explicit value accepted only where the field permits it, including a blank YAML scalar;
it does not universally mean reset or use-default. Empty lists/maps remain explicitly empty and
follow their field's contract rather than inheriting values. For example, integer `queryLimit: 15`
can be valid, `queryLimit: "15"` fails that integer contract, and `enabled: yes` fails a boolean one.

Comments, ordinary mappings/sequences and multiline strings remain supported. V0 rejects
anchors/aliases, merge-key behavior and custom tags; templates, typed references and declared
graphs provide reuse. Loading configuration does not implicitly expand shell/environment strings.
An explicit typed input binding or plugin operation can supply a value or perform its declared
work; script-based plugins remain supported. This profile governs Conduit's configuration loader,
not the contents/formats of opaque native files supplied through Resources.

Parser choice, diagnostics and schema-specific numeric/unit limits remain implementation work.
Multiple-object submission and batch outcomes follow their operation contracts; parsing YAML does
not imply atomic deployment, authority or successful setup.

### Implementation selection

**V0 rule:** select Conduit plugin implementations explicitly or through an
unambiguous configured default. Resolve the exact release/artifact identity before execution and
retain it for the instance's subsequent starts, operations and recovery. Installing a newer package,
changing a default or package discovery order must not silently change that binding. Record an
unresolved requested dependency as unresolved, not as located or verified code.

Contract identity/version defines promised configuration and behavior; implementation bindings
identify the declarations, adapters and any authored code supplying it. Multiple implementations
can deliberately support the same contract. Code-optional contributions retain their resolved
binding definitions and exact Conduit-controlled adapters even when no new executable is included.
Conflicting definitions of one contract identity/version cannot silently overwrite each other;
unresolved selection ambiguity blocks affected execution rather than choosing the newest package.
Required schemas must be available for registry writes. With known schemas, otherwise valid
configuration can still register while code is missing; report its readiness gap without substitution.

Adopting another implementation release/artifact requires a replacement instance in v0, even where
the new release claims the same contract. Explicit backing-data reuse remains possible under the
type's create/attach rules; it does not transfer runtime progress, connections or grants implicitly.
This avoids in-place code migration rules for active work and retained setup progress. It means
keeping an older implementation available or replacing dependent instances when adopting fixes.
The retained binding is current executable configuration, not mandatory template-history storage.

This rule applies to Conduit-selected declarations, mappings and plugin/adapter code. It does not
freeze externally operated services/native tool catalogs or promise control of external native-agent
binaries. A recorded service reference is not proof of exact provider code. Adapters declare and report those
separate compatibility/version boundaries. Dynamic Conduit attachments and supported catalog refresh
retain their accepted behavior. Artifact verification, package storage/delivery, plugin ABI and exact
binding fields remain engineering design within this contract; public naming governance is deferred.
Service-target and authentication selections continue to follow the existing instance configuration,
control and stable-reference rules; an implementation binding supplies no implicit access or fallback.

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
Callers may supply an explicit unique name instead. Direct creation from complete instance
configuration also requires an explicit unique name. All paths obey per-family uniqueness across
forms. Exact suffix format, name constraints and generated-collision handling remain open. Generated names remain
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
continue to apply; matches under supported typed access filters remain live. The ID format,
serialized field, cross-registry identity policy, optional direct-ID forms, import/export
handling and exact rebind operations remain open.

### Configuration updates

The checked full-replacement workflow below governs **template/authoring registry edits**.
It does not authorize replacing a running instance's spec through a generic registry write.
Instance configuration changes use the template-declared control contract through a Conduit.
They apply one at a time per target by default unless the contract allows safe parallelism.
They normally check the caller's expected configuration revision at application, with explicit
contract exceptions. Exact revision mechanics and partial-change semantics remain to be settled;
checked identity and safe outcomes remain necessary. Registry deletion retains its own scope.

The local logical registry exposes active configuration through the authorities defined above;
its aggregate observations are not independent writable state. YAML is explicitly applied or exported;
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
is stronger than an ordinary incoming reference: a Pool instance cannot be deleted while it
owns Resource registrations. Deletion requires explicit Resource retirement first, including supported
stop/disconnect steps and checked unregistration. An explicit bulk retirement action may handle
the Resources and then their Pool; partial failure retains the Pool and reports unfinished work.
Ordinary Pool deletion does not implicitly retire owned Resources. Ownership transfer is deferred.

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
configuration. Control contracts still need explicit no-op and revision-result semantics; registry equality alone does not define their effects.
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
encoding is open; none may silently stand in for another. A reference or capability
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

| Relationship | Established constraint |
| --- | --- |
| Agent template → Agent | Interpret typed establishment configuration and retain ongoing instance settings. Customize ordinary typed fields with replacement semantics. Continuing authority and dependencies remain explicit; the type defines supported transitions. |
| Resource template → Pool → Resource instance | Only a Pool creates/attaches its instances. Explicit named entries identify available templates; reuse across Pools creates distinct owned instances. Referencing a foreign instance does not transfer ownership. |
| Agent → Conduit → Pool | Filters narrow authorized resource/operation combinations. Combining access paths must not pair one path's operations with another path's resources to create unintended access. |
| Agent activation | Instructions materialized in the Agent spec operate on permitted Resources. Their ordering, conflicts, optionality and update behavior must be explicit. |
| Intent → observation | Runtime observations report whether requests are realized; they do not silently become authored defaults or fresh authority. |

Permission, membership and activation semantics remain independent even when they use
similar reference/filter notation. A serialization library's merge behavior or filesystem order
must not choose these rules accidentally.

### Validation, effects and evolution

Distinguish syntax/schema checks, reference resolution, semantic/transition checks,
environmental probes and effects. A valid object can still have an unresolved reference,
an unsupported native extension or an expired credential. Error/status reporting must
identify the relevant object and field, explain the limitation, and preserve unknowns.
Exact diagnostic codes, stages and partial-failure policy remain open.

Typed configuration rejects unknown fields for both templates and instances, except within
maps whose schemas explicitly allow arbitrary keys. For example, `metadata.naem` is an unknown
field, as is `metadata.labels` in v0. A plugin contract may explicitly declare an open options
map; its keys and values still obey that contract. Openness is limited to the declared location. Plugin
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
Exact version identifiers and compatibility representation are schema work. Any proposed change
to accepted interpretation or instance-upgrade behavior returns for review.

Keep the core small and extensible through typed contracts. Reusable objects are the first
composition mechanism; additional template languages, expressions and patch systems need
concrete use cases. The following sections retain the accepted domain behavior that the
language must express. [REVIEW.md](REVIEW.md) defines the next schema and acceptance walkthrough.

## Agents

### Templates

An Agent template is reusable configuration for any number of independent Agents. It specifies
runtime/model/account defaults, execution preferences and requirements, optional permission
limits, default connection choices, activation instructions and the exposed configuration controls
with their constraints. Establishment uses one Agent template's configuration or a directly
authored complete instance definition. Export/creation representation belongs to the configuration
profile; direct creation itself is accepted. V0 defers inheritance and composition between Agent templates.

Establishment produces the Agent's complete spec. The Agent operates from that spec
and explicit live dependencies; retain actual execution settings.
Permission limits remain current. Resource data stays available through live authorized reads;
refresh of loaded content follows the activation contract below.

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

**Activation policy:** apply the Agent's declared compatible preparation actions.
Required activation failure prevents readiness; optional failure is visible without blocking
unrelated usable capabilities. Resolve conflicting exposed names through explicit selection or
supported distinct names, not discovery order. Avoid duplicate equivalent loading without merging
different configurations, access paths or authority. Compatibility depends on the actual runtime;
model preference alone does not establish native plugin or hook compatibility.

For example, preparation can advertise selected skills for on-demand loading, inject a required
engineering guide and expose optional Linear tools. These are actions in the existing setup/control
model. Template/type defaults can supply conventional behavior without a human prompt for each step.

Source content changes become available for ordinary authorized reads, but do not silently replace
already-loaded instructions or executable extensions. A supported explicit refresh applies at a safe
runtime boundary and reports what was loaded; it does not rewrite earlier conversation history.
An authorized Interface or Agent can request refresh without mandatory human confirmation. Where
refresh is unsupported, report the limitation and use supported restart/replacement behavior; changing
fixed activation/control definitions still requires a new instance. Late dependency failures retain
the agreed independent-work behavior, not a new whole-Agent pause requirement.

Automatic reload/refresh policies are deferred beyond v0, including automatic catalog replacement
on first use. Initial discovery and explicit refresh remain supported. Current access restrictions,
credential refresh and bounded transport reconnection keep their own contracts. The MCP section
below distinguishes stale tool definitions from changed service data. Exact action fields, evidence,
compatibility diagnostics and future automatic-refresh triggers remain schema/adapter work.

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

**Accepted delivery order:** after the synthetic runtime experiment, validate real Agents through
native Claude/Codex adapters first, then build Conduit's own Rust model/tool loop. The native path
tests separate subscription accounts, private state and shared Resource access through Conduits;
it does not establish the owned loop's behavior or unobserved native activity. Both paths remain
goals.

**Accepted initial platform:** validate native Agent integration and subscription accounts on
macOS first. Keep the Rust core portable and isolate OS-specific process,
credential and filesystem behavior behind adapters. Other platforms require their own integration
evidence before support is claimed; this is initial test coverage, not a permanent platform limit.

The first frontend is a custom terminal inspired by Pi. Presentation is replaceable;
it does not own credentials, model calls or authoritative execution state. Pi terminal
components may be reused, but Pi's agent loop and Pi Durable are research references.
A memory backend or frontend change must not replace Conduit's execution loop.

**Shared Interface control:** an Agent's direct user-control belongs to its controlling
Interface. All authorized frontend attachments of that Interface can issue input, cancellation and
settings commands; a terminal and CLI need no controller handoff between them. Each request keeps
its caller/attachment identity and passes through its permitted Conduit path and target contract.
Owner-operated frontends use the Interface's unrestricted v0 authority; configuration revision
checks and per-target coordination still provide meaningful outcomes. Simultaneous submission
does not promise simultaneous execution.

**Delegated Agent control:** an authorized Agent may invoke another Agent's permitted
controls through its own Conduit and identity while user-control through the Interface remains
available. No exclusive controller handoff is required. Each request remains subject to current
authority, relevant state checks and target coordination. Neither actor has automatic priority;
coexistence does not imply simultaneous execution. Ordinary communication delivery and wake
behavior remain separate from permission to invoke a control.

Differentiated control/authority across distinct Interfaces is deferred. Precise input scheduling
and loss/recovery mechanics remain open.
Disconnecting a frontend detaches and preserves supported ongoing Agent execution. Keep persisted
history, current model context, frontend state and shared Resource content distinct.

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

Agent observation participates in the shared [Interface observability](#interface-observability)
contract; it is not a separate per-frontend logging system.

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
| `spec.config` | Type-specific settings validated for the selected type and form. Templates declare establishment intent; instances retain initial and then confirmed settings in one map under the shared configuration contract. Exact fields are schema work. Unknown fields are rejected except in schema-declared open maps. |
| `spec.conduits` | The consumer's Conduit declarations. Their concrete entry syntax is proposed in the walkthrough. |

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
requirement to implement both immediately. Additional types and their detailed schemas are extensible.
Keep an additional agent-communication group under consideration for direct messaging,
multi-participant conversations and shared context. Agent-specific contracts can justify a
distinct group even when they use familiar storage/integration mechanisms. Its name, types,
Resource-versus-Conduit responsibilities and shared-context semantics remain open. The first retained-history capability is defined under [communication](#communication-contract);
that does not require every communication Resource to use that shape.

Conflicting definitions for a contract identity/version must be reported rather than
silently overwritten. Public contract ownership/governance, allowed identifiers and version labels
remain drafting/future work. Groups and type contracts add no registered object family, assign no Pool ownership and
grant no access. [Resource typing](research/RESOURCES.md) retains the supporting investigation,
candidate types and proposed interface boundaries; those proposals are not built-in requirements.

Resource type contracts are versioned independently of the common object `apiVersion`.
The common format can remain unchanged while a type's configuration/behavior contract evolves.
Plugin implementation releases are separate from both versions; a plugin release number does
not substitute for either contract version. The retained implementation binding and replacement
rule apply; identifier spelling and concrete compatibility checks remain to be drafted.

**Compatibility boundary:** required connections name a versioned capability contract,
describing the operations and behavioral guarantees a consumer needs. Different Resource types
may fulfill that same contract; consumers need not require a particular concrete Resource type.
A Resource type still defines configuration, establishment and lifecycle. A capability contract
defines the promised consumer-facing behavior; it is not an Interface instance or a sixth family.

For example, local-directory and git-worktree could both fulfill a directory-access contract while
retaining distinct creation and cleanup rules. This is a compatibility example, not a finalized
contract name or guarantee of identical filesystem behavior. Matching requires the promised
semantics, not merely a shared group, operation name or data shape. Pool target selection remains
explicit; provider/account constraints, current permissions and readiness remain separate checks.
Capability identifiers, compatible-version rules, conformance evidence and any supported adaptation
remain open. A matching declaration is not proof of actual implementation or current availability.

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
Discovery uses defined fields and relationships, never granting authority or transferring ownership.

Resource instances record enough identity/contract information to resolve their source and
supported operations. Provider-owned data retains native contracts; attaching it does not
make Conduit the owner of the provider account or authorize deleting that backing data.

Resource templates and instances provide concrete protocol review cases. A local path exposed through a
symlink, an MCP connection and hybrid search over a database are contrasting examples, not
interchangeable implementations of a fixed lifecycle. Settle their typed configuration,
backing identity, operations and shared/per-consumer state before deciding Pool grouping.
A type contract defines supported semantics; a template supplies reusable establishment
configuration and declares the permitted configuration changes within that support.
Other Resource contracts can be developed against the [schema review criteria](REVIEW.md#drafting-and-deferred-work).

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

Resource providers retain source content and operations. Linear/Jira integrations are ordinary
Pool-owned Resources accessed through Conduits; the provider owns its issue fields, assignments,
status and workflow. Message history is also a Resource. There is no special tracker abstraction,
required issue link or mandatory Linear/Jira integration in the core protocol. If an application
needs specialized message tracking or work associations, it can supply a custom Resource capability.
This does not introduce an internal ticket engine or make common observability optional.
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
establishment settings. These entries describe what the Pool can provision; listing or adding a
template does not create/attach a Resource instance, authenticate a service or connect an Agent.
Available templates and the Pool's owned Resource instances are distinct views, not new object
families. "Catalog" is only shorthand for that available-template view; it does not require a
separate object, package registry or service. A Pool template describes availability, provisioning
settings, policies and permitted management/configuration changes. A Pool instance creates/attaches selected
Resources through explicit provisioning or declared setup-graph steps and exclusively owns the
resulting Resource instances. Pools remain the resource-supply and shared service-access boundary
for Agents and connections, including skills,
tools, memory and connection support such as conversation storage. A Pool need not run a process.

**Dependency composition:** reusable Resource templates declare their required connections;
Pool composition supplies concrete targets during selected Resource instantiation. The consumer
instance supplies its own local endpoint and retains the resolved Conduit declaration/target binding. For example,
one reusable MCP template can connect to Euphony's authentication Resource in a work Pool and to a
personal authentication Resource in another Pool without embedding either concrete identity in the
reusable template. This also applies to other Resource dependencies, such as search using storage.

This separates reusable requirements from creation-time wiring. It is not automatic account
selection, a new template-parameter DSL or permission granted by Pool membership. Keep one resolved
connection assignment on the consumer instance, with derived reverse views. Later template/Pool
edits do not silently rebind established connections. Compatibility uses the capability contracts
above; exact declaration syntax, version matching and explicit rebinding controls remain open.

**Local composition:** Pool templates name their Resource entries locally, allowing a
dependency binding to refer to another entry in that composition. For example, `linear` binds its
authentication requirement to `auth`. Provisioning the selected composition resolves that reference
to the concrete authentication Resource for this Pool, and the consumer retains its resolved stable
target ID. Catalog entries alone have no deployed Resource identity. Other Pool instances own their
own deployed Resources; template reuse alone does not share instance identity. Local entry names
are not global registry aliases or an alternative instance identity; referencing one grants no access.

**Provisioning intent:** templates/graphs explicitly distinguish provisioning from a
named Pool entry from using a selected existing Resource instance. The former creates a Resource
through its owning Pool; the latter binds an already-registered instance without changing its owner.
Both resolve to a concrete Resource binding. An entry is a reusable provisioning choice, not a
singleton instance or an implicit reuse-or-create instruction. A matching template/name alone must
not decide between those actions. Pool composition or explicit creation inputs can supply an existing
target without embedding a particular deployment's identity in every reusable template.

For example, two tasks can provision separate worktrees from one entry, while several Agents share
the Pool's existing authenticated Linear Resource through their own Conduits. A new service can bind
the same authentication Resource rather than provision another login. Provisioning with attach
behavior is different: it creates a new registration around existing backing, such as a local folder.
Composition carries the choice without a separate operation manifest or a manual question on every
start. Authority, compatibility and explicit binding rules still apply; retry reuses valid recorded
progress rather than treating a provision instruction as permission to create duplicates.

**Composition mechanism:** a reusable graph step consumes the composition's explicit
provisioning/existing-instance choices and dependency bindings. It establishes newly requested
Resources through their own setup contracts and resolves existing targets without treating their
selection as a new provisioning request. It exposes concrete results for subsequent bindings.
A declared setup graph may establish a chosen composition; catalog membership alone
must not cause all entries to deploy. Custom graphs can reference the same entries for extra
ordering or checks; authors need not restate Resource configuration or hand-wire every component's
setup. Required dependencies govern preparation, not YAML mapping order. This preserves each
Resource's runtime ownership, authorization and recovery contract. Exact operation names, typed
results and graph syntax remain open.

**Partial availability:** ready Resources are normally usable through authorized Conduits
while other components of their Pool are waiting or have failed. Each Resource's own prerequisites
and any declared gates still apply. A ready checkout can be used while shared authentication waits
for consent and Linear remains unavailable. Report the Pool's incomplete setup and component
states; do not infer readiness merely from a running process. Successful partial work remains
retained under the existing recovery rules. Aggregate status/completion fields and explicit
whole-Pool gate contracts remain open; this adds no automatic retry or cleanup behavior. Templates
merely available in the catalog are not pending/failed setup components; only requested setup
contributes such progress and readiness obligations.

Creating a Pool registers its identity and configuration. Provisioning is explicitly requested
to create or attach Resources through their owning Pool. A frontend may offer both in one
explicit action with separate outcomes; successful registration does not imply provisioning.
No background reconciler is implied.

Use the accepted local composition declarations, resolved instance bindings and ordinary operation
evidence without requiring a second provisioning-plan ledger. Pools group unitary Resources and
derive useful views from their contracts. Exact composition fields and result wiring belong to the
configuration profile in [REVIEW.md](REVIEW.md#review-outcomes). Stable identity, explicit
effects, safe retry and separate backing-data cleanup remain requirements.

**Label removal:** Pool inclusion uses explicit named Resource-template entries.
V0 has no generic registry-object labels or label-based selectors, including for discovery or
automatic Pool inclusion. Inspect/query defined fields such as kind, form, Resource type, owning
Pool and explicit relationships. No label inheritance or selector-refresh contract is needed.
Adding/removing an available-template entry is distinct from provisioning/retiring its deployed
Resources; availability changes do not silently delete instances or retarget existing bindings.
Knowledge-record classification and typed Conduit filters remain separate. Reusing templates
creates distinct owned instances, not overlapping ownership; Pool schemas stay open to new types.

Connection participants can access Resources in several Pools through authorized paths to
the owners. Sharing backing data between separate instances does not transfer ownership or
establish access. Pool ownership, current access, Agent activation and native backing-data
management have distinct meanings.

Creating a Resource for a connection must identify its owning Pool; the connection requests
provisioning through that Pool. Provisioning retries reuse valid recorded progress; any further dynamic support requirements
need an explicit type contract. A template registration alone does not create Resources.

**Connection selection:** omit Pool-provided default Agent connections in v0.
Agent templates declare starting Conduits, optionally referencing reusable composite Conduit
templates. Authorized Interface/Agent operations explicitly add or remove connections later.
Pools make provisioning templates and owned Resources available; they do not automatically suggest
or add a connection set when an Agent uses the Pool. This avoids a second source of launch choices
and its merge/precedence rules. Defaults inside Resource configuration remain supported and are
unrelated to Agent connection selection. Current authority and Agent limits still constrain each
connection; declaring one does not issue its own grant.

**Retirement:** keep a Pool instance registered until its owned Resource instances
have been explicitly retired through their supported lifecycle and checked unregistration.
One explicit bulk action can retire the Resources and then remove the Pool. If retirement only
partly succeeds, preserve the Pool and report remaining work; do not claim rollback of completed
steps. This also covers Resource registrations created during incomplete Pool setup. Ordinary
Pool deletion does not implicitly cascade retirement, and ownership transfer is deferred in v0.
Stopping a managed client does not itself delete a remote account or stop a shared external
service. Backing data and retained evidence follow their separate cleanup/retention contracts.
Missing ownership remains an inconsistency to resolve, never implicit orphan-purge authority.

### Shared access and authentication

Pools are the normal place to establish and manage shared service access. Their
owned capabilities retain the selected provider bindings so multiple authorized Agents can use
them without each completing provider login. This applies beyond MCP: a suitable integration can
manage database, repository, cloud or credential-service access. Actual provider support and any
interactive renewal requirements remain explicit.

A binding means a managed association to an authorized service identity, not a copied secret in
configuration. Plugins implement authentication; instance runtimes own continuing state/refresh.
The Pool owns and organizes the capability rather than introducing a second authentication engine.

**Representation:** reusable authentication is a Pool-owned Resource, accessed by service
Resources through Conduits. It owns the reusable authentication/refresh lifecycle; consumers own
their service behavior and clients without becoming separate owners of the login. Pool templates
can compose these Resources and connections so one integration does not require manual authoring
of every helper. This adds no family and does not make each token or login attempt a Resource.

One authentication Resource can provide supported bindings for several service consumers; tokens,
audiences and provider grants may differ even when the underlying account is the same. Sharing
the account never establishes credential compatibility. Its exact type group/name, operations,
connection binding and handling of non-reusable internal authentication state remain open.

| Concern | Contract |
| --- | --- |
| Service authority | The provider identity and grant bound to the capability; reuse is intentional and must be supported. |
| Caller authority | The Agent/Interface identity and its current permission to use that capability. Shared login does not mean identical rights. |
| Connection restrictions | Each Conduit narrows permitted Resources, operations and parameters. Pool membership alone does not grant access. |
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

For example, an Euphony Pool owns an authentication Resource and a Linear Resource connected to
it through a Conduit. A review Agent reads issues while
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
in the [implementation authoring review](proposals/implementation-v0/AUTHORING.md#5-where-the-example-implementations-execute).

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
recovery and composition details remain in [REVIEW.md](REVIEW.md).

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

Requirements express obligations and restrictions. Existing graphs coordinate declared procedures
to establish prerequisites, while instances own ongoing behavior. Resource configuration and plugin
input/output contracts remain; a relationship requirement does not redefine a target operation.
Typed filters are related narrowing requirements, with their existing whole-path access semantics.
Requirements cannot create permission grants or override baseline identity, ownership and lifecycle
rules. Registration still performs no provider effects; observed readiness remains distinct from
valid configuration. Detailed satisfaction/evidence and failure rules require implementation review.

The concrete core catalog, publication syntax, support-matching details, composition operators,
version identifiers, satisfaction checks and runtime enforcement bindings remain open. The
[implementation review](proposals/implementation-v0/REVIEW.md#i3--plugin-execution-and-distribution)
tracks those choices; accepting the compositional model does not select an expression language,
a universal solver, a worker topology or an expanded experiment scope.

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
Resources; the engine and plugins execute and supervise Conduit flows. Their ordering and
failure behavior are unresolved, but a connection cannot provision an unowned Resource or
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

Permissions name operations per Resource contract, with convenience bundles expanding
into visible grants. Agent limits and explicit exclusions constrain effective access;
missing limits add no authority. Permitted operations require no extra confirmation merely
because permission rules exist. Resource and communication permissions do not imply each other.

**Delegation boundary:** an authorized user or Agent may delegate an explicit subset of
service access when establishing another consumer's Conduit attachment. The attachment records
the delegation; no separate prior Agent-permission setup is required. The creator must have
authority to delegate that access, subject to target policy and recipient limits. For example,
a user can connect a reviewer to shared Linear access with read-only authority in one action.

Keep requested access, the authorized delegation and any additional filters distinguishable.
Authored configuration requests access; it does not issue its own grant. Filters only narrow
granted authority. This adds no permission-registry object family and does not implicitly grant
onward-delegation rights. **Lifetime:** the grant belongs to recipient logical
participation and survives the issuing frontend closing or coordinator execution stopping.
Recipient detachment ends that attachment's access; temporary socket loss alone does not.
Withdrawal and current governing restrictions continue to apply at dispatch and delivery.
Stopping the issuer is distinct from withdrawing its grant or changing an applicable governing
restriction. This keeps recipient work independent of the UI/process that established its access.
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
The [protocol investigation](research/CONDUITS.md#messaging-and-networking-investigation) supplies
source evidence and failure examples; broader work-submission examples are future capabilities,
not requirements for the first slice. Concrete fields and transport selection follow those choices.
