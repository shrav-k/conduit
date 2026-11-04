# Model and system responsibilities

Scope: accepted working contracts. Explicitly marked v0/reference-runtime selections retain
their accepted status without becoming requirements for every capability implementation.
See the [reference boundary](README.md#protocol-and-reference-implementation) and the
[current review queue](../REVIEW.md) for unresolved questions; historical review links below record prior rulings.

Part of the accepted [design contracts](README.md). Updated: 2026-11-03.
The [conceptual review](../archive/reviews/CONCEPTUAL.md) records completed rulings and deferred work;
syntax and implementation examples do not amend these rules.

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

### Vocabulary review boundary

Only the five main family names—**Interface, Agent, Resource, Pool and Conduit**—are settled.
All other Conduit-owned terminology and field spellings are working vocabulary, including names
previously selected while reviewing a structure. Continue using those names consistently so the
documents and examples remain readable; their behavioral meanings and accepted structural rules
remain in force. For example, the four explicit binding forms and their one-key encoding remain
accepted even though the current `literal`, `reference`, `object` and `list` spellings are provisional.
Vocabulary uncertainty does not block logical or structural review.

A separate future PR will review terms collaboratively, one by one, and then update the glossary,
protocol text, schemas and examples coherently. Record confusing collisions as they arise, without
opportunistic renaming or treating understandable example syntax as final vocabulary approval.
Names defined by external standards retain those standards' meanings. The
[current review](../REVIEW.md#future-vocabulary-pr) tracks the naming work separately from the
ongoing walkthroughs; a naming change alone must not silently alter a contract.

### Authoring concepts beneath the object families

The five families describe the objects Conduit manages. The authoring and extension concepts below
describe how those objects are defined, connected and operated. They are not competing object
families or additional independently provisioned things. The divisions are accepted; the names are
temporary under the [vocabulary boundary](#vocabulary-review-boundary). The future vocabulary PR
should preserve these responsibilities rather than silently change their meaning.

| Working term | Responsibility | How it composes |
| --- | --- | --- |
| Contract | An inspectable agreement about meaning, typed values and behavior. Operation, capability, type and requirement contracts describe different aspects of an extension. | Definitions reference other definitions rather than duplicating their meaning. A schema describes valid data; the behavioral contract also explains what it means. |
| Capability | What an endpoint offers, including supported operations and behavior. | An endpoint can offer several capabilities; a relationship selects compatible ones. |
| Requirement | What must hold for a particular relationship or its use, with typed parameters and applicability. This can include compatibility or lifecycle obligations as well as restrictions. | A reusable definition can compose existing requirements. Every constituent retains its meaning and limits; support is checked against the constituents. |
| Policy | Authoritative application of reusable requirements to actors, targets or relationships. | Uses the same requirement definitions, composition and implementation support. Governing bindings are checked managed state; no separate policy language or executable rule category is required. |
| Grant | A particular issued authorization for an identified subject, target/capability and permitted extent, with its own participation lifetime and authorized administration. | Issuance/changes require current administrative authority. Issued grants have no live dependency on their issuer or the issuer's grants; provenance is not an access condition. |
| Operation | A callable action with typed inputs/outputs and applicable effect, completion and recovery semantics. | Graph nodes select operations and bind values. One plugin can supply several operations. |
| Graph | A finite dependency structure describing a procedure. It can implement a reusable typed operation as well as template setup or a supported change. | Nodes call plugin operations, including graph-backed operations; outputs and explicit dependencies order the work. |
| Plugin | A reusable contribution publishing contracts and implementation bindings, backed by authored code, existing MCP services, graphs of operations, or a combination. | A plugin can publish a composite operation implemented by a graph of other plugins' operations. This needs no new object family, process or plugin-specific executable merely for composition. |
| Package | A distribution envelope with declared identity, version, typed contents and dependencies. | Can contain plugin contributions, templates, or both, including template-only collections referencing other available packages. It is not another runtime object family. |

Packages use a common outer contract while each contribution retains its own typed validation and
behavior. A template-only collection need not supply executable code or a plugin implementation.
Installing a package makes its definitions available; it does not instantiate objects, authenticate
services or grant access. The common envelope does not require an identical source directory layout,
bundled dependency copies or a new general-purpose package manager. Exact fields, dependency references
and archive layout remain authoring work; automatic dependency resolution stays deferred.

**Package entry document:** each package has one explicit entry document declaring its public
exports and dependencies. Definitions can be inline or referenced in package-local files, so a
small package can keep its declarations together while a larger one separates contracts, graphs
and templates. The entry document may be handwritten or generated. Tools inspect the declared
surface without executing plugin code or inferring exports from directory layout; merely including
a file in an artifact does not export its definitions. Provider-native tool catalogs remain dynamic
under their integration contracts. Filenames, fields, reference notation and archive layout remain
open; existing package capture and supplied-dependency rules still apply.

Policy and Grant are essential protocol concepts for Agents and Interfaces, alongside the other
concepts here. Their authorization contracts must apply consistently to managed capability use,
controls and delegation, rather than only to Interface administration. Conduit paths and their
selected endpoint implementations enforce the applicable authority and requirements. The root's
broad default authority and Agents' explicit delegation remain different permissions within that
structure. Grant issuance/change/revocation and governing-requirement changes are ordinary protected
capability operations: the caller needs a grant permitting the operation, with requirements
constraining its proposed effect. This avoids a separate grant-administration rule language.
Other managed callers, including Resource-to-Resource access, retain their existing attribution
obligations. The [authorization contracts](../proposals/authoring-syntax/AUTHORIZATION.md) illustrate
the accepted model and proposed detailed representations. Startup and initial storage choices are settled;
the dependent-call authority rule was settled in [C2](../archive/reviews/CONCEPTUAL.md#c2--whose-authority-governs-a-dependent-call)
and is specified in [Conduits](CONDUITS.md#authority-across-dependent-calls).

A template assembles these definitions for one of the five families: configuration, selected
implementations, setup/change procedures, ongoing runtime behavior and supported controls. Its
instances realize that definition with identity, state and managed activity. The template bridges
authoring and the managed object model; it is not merely a graph. Ongoing services remain under
instance management rather than belonging to a finite setup node.

**V0 composition:** no primary plugin is required. A complete template binds the selected type's
required behavior to compatible contributions from one or several plugins, including setup,
ongoing runtime/capability behavior and supported controls. Convenience comes from complete standard
templates, reusable operations and declared type defaults, including default handler implementations.
These resolve into retained instance bindings; no implicit merging of installed packages, template
inheritance or new preset family is introduced. Explicit invalid selections do not silently fall
back to defaults. Declared contract/type compatibility is checked before execution; actual placement,
identity, access and readiness are verified at their required points. Conformance tests provide
behavioral evidence; matching schemas or package names alone do not establish compatibility.
This preserves instance-owned state/runtime responsibility and fixed-definition replacement rules.

**Template implementation selections:** templates select implementations in one named dependency
map. Setup operations, runtime drivers and control handlers refer to a dependency alias plus a
public export name, with the expected kind and contract checked at each use. No alias is primary.
For example, `messages` selects the history package; setup selects `messages` plus
`history.establish`, while the runtime selects `messages` plus `history.managed`. Aliases identify
implementation selections, not Resource instances, processes or access grants. Package-internal
references retain their defining scope, and resolved implementations remain subject to the existing
retention/replacement rules. Exact map fields and reference notation remain open.

There are two distinct forms of composition. Requirements combine obligations; graphs combine
actions and their dependencies. A graph may establish prerequisites for a requirement, but
successfully finishing setup does not prove that every later request satisfies it. Conversely, a
requirement declaration does not itself run an authentication or connection procedure. Plugins can
supply the behavior for both, without collapsing declaration, execution and ownership together.
An operation can also be invoked directly through its supported surface; ordinary tool calls do
not need a new authored graph merely to be callable.

Reusable requirement compositions share graphs' literal/reference/object/list value-building forms,
with references scoped to the composition's declared parameter payload. This forwards or assembles
typed constituent parameters without plugin glue, ambient configuration or graph execution sources.
The shared notation does not change the distinction between obligations and actions or replace
validation, current authority and constituent enforcement.
Constituents form a map keyed by locally unique names, each selecting a requirement contract and
parameter binding. Reusing a contract under different entries is allowed; map order does not
sequence enforcement. Local names support inspection without adding runtime objects.
V0 requirement composition is finite and acyclic: reject recursive definition cycles, while allowing
independent reuse with distinct parameters. This does not constrain the topology of live Conduits
between objects or prohibit recursive data schemas.

A Conduit uses both forms: its contract composes relationship requirements and its procedures
coordinate establishment through selected endpoint capabilities. Its instance represents the live
relationship. It is neither the universal name for any graph nor the package that supplies endpoint
code. The [Linear walkthrough](../proposals/authoring-syntax/AUTHORING.md#one-linear-connection-through-the-authoring-concepts)
illustrates these boundaries.

**Reusable executable composition:** a graph may implement a typed operation contract and be
published through a plugin. Another graph or permitted caller invokes that exported operation
through the normal operation contract. From the author's perspective this is a composite plugin;
the graph describes its implementation, while the plugin publishes its contract and binding. A
plugin can export several operations, and a template-local graph need not become a published plugin.
No special composite-plugin family or mandatory packaging unit is introduced.

The exported contract describes inputs, outputs and applicable behavior; internal nodes bind those
values and call selected operations. V0 graph inputs use literals, typed references and explicit
object/list assembly; calculations and other transformations use plugin operations, with inline
binding expressions deferred under the [graph-input boundary](LIFECYCLE.md#lifecycle-protocol).
Reusable graph-backed operations receive caller configuration through their declared typed inputs;
they do not implicitly inherit the caller's configuration namespace. A template can pass its captured
search limit into an indexing operation's `limit` input, while another template supplies that input
from a differently named setting. Internal nodes use the declared operation inputs. Scoped invocation
context, such as identity and operation tracking, and explicitly authorized Resource access remain
available under their contracts. Graphs may reference the runtime's explicitly declared, typed,
read-only invocation metadata, without exposing the whole instance or treating supplied IDs as
authenticated identity or permission. Exact metadata fields and input/context notation remain
authoring work under the [lifecycle contract](LIFECYCLE.md#lifecycle-protocol).
V0 references select declared inputs, captured instance configuration, earlier step outputs,
declared runtime targets or typed invocation metadata. Other data access uses explicit authorized
operations; arbitrary plugin-defined reference resolvers are deferred. This preserves the explicit
input boundaries of reusable operations.
Composition preserves existing authority, endpoint selection,
implementation retention and instance-owned runtime rules. It does not automatically make several
external effects atomic or safe to retry. V0's finite dependency-graph boundary still applies;
nesting does not introduce authored recursion or loops. Exact graph-call/binding syntax, publication
layout and detailed outcome/recovery coordination remain open.

Executable graphs share required-by-default steps with explicit optionality across setup, reusable
operations and change handlers. Required steps must succeed for successful completion; optional
failures remain visible without alone preventing it. Actual dependencies, mandatory contracts and
lifecycle duties still apply. Public outputs and permitted partial control confirmations do not
establish success for unfinished required work. See the [graph execution rules](LIFECYCLE.md#lifecycle-protocol).

Graph-backed operations reuse Conduit's graph recovery by default. An explicit retry of the same
unchanged invocation resumes retained child progress, reuses only still-valid results, and delegates
uncertain external effects to the affected child's recovery contract. A composite author need not
write separate recovery code merely for composition. This neither starts automatic retries nor
promises rollback or recovery unsupported by a child. Custom recovery hooks and exact coordination
remain open; instance supervision and current authority still apply.

Composite output contracts account for their composed success and failure modes. Derive the
initial public result contract by default from declared public output bindings and constituent
contracts; authors may explicitly construct a domain result instead. Preserve relevant effects,
uncertainty and links to scoped evidence without automatically exporting internal outputs. Public
output values use the same literal/reference/object/list binding forms as graph inputs under their
declared publication, validity and access contracts. Public results and retained execution evidence
serve different purposes. Detailed derivation/mapping
mechanics and recovery coordination remain under review in the
[composite output proposal](../proposals/authoring-syntax/CONTRACTS.md#composite-outputs-and-outcomes--proposal).

### Package scope and public bindings

A published package resolves its internal references within its own defining scope. Importing it
under a caller-local alias does not rename or rebind those internal references. A same-named
contribution in the importing template cannot silently override a package's internal dependency;
this boundary is preserved when the package is used in a composed graph.

Exports identify contributions the package makes available to callers. Imports declare dependencies
that need bindings. Callers can supply replacements only through dependency bindings the package
explicitly exposes, subject to the required contract compatibility. Internal names do not become
caller-configurable merely because the package is imported.

Public export names are unique across all contribution kinds within a package. Selecting a package
and export name identifies one contribution; resolution still checks its expected kind and contract.
For example, a package can export the template `shared-history` and operation `establish`, but cannot
also export an operation named `shared-history`. Reject that collision rather than choosing by kind
or declaration order. This applies across inline and referenced definitions and regardless of their
file layout. Internal definition names, contract identities and registry object names retain their
separate rules. Exact name grammar and reference notation remain open.

A package may explicitly re-export a declared dependency's public contribution under its own public
export name, without copying the definition or introducing a wrapper. For example, an Euphony starter
package can expose a history package's `shared-history` template as `conversation-history`. The alias
preserves the original definition, contract and internal reference scope; resolved dependencies retain
their identities. Re-exporting does not adapt behavior, rebind private dependencies or grant access.
Package-wide export-name uniqueness and the usual kind/contract checks still apply. Exact declaration
and resolution notation remain open.

For example, a history package's internal `checks` helper still selects its history validator when
the importing template also calls an unrelated repository validator `checks`. The caller may
replace the history validator only if the package declares that dependency replaceable. Its
explicit binding must satisfy the required contract; name coincidence supplies no binding.

An explicitly replaceable dependency may declare a default implementation binding. When the caller
supplies no replacement, that declared default is selected. A compatible explicit replacement
takes precedence; an invalid explicit selection reports an error rather than falling back.
This does not authorize automatic installation or substitution with another available package.
Existing schema-validation and readiness rules apply when the selected contribution is unavailable.
The resolved implementation is retained on the instance under the existing fixed-definition rules.

For v0, an imported dependency requires an export that explicitly supports the requested contract
identity and exact version. Existing export-kind, type and behavioral compatibility checks still
apply; a support declaration alone does not prove conformance. Cross-version compatibility mappings
are deferred, and matching input/output shapes do not substitute for the required contract.
Contract versions are independent of implementation releases: different plugin releases can fulfill
the same contract version. This creates no obligation to retain older contracts or compatibility code.

These are definition and implementation-selection rules. Selecting an export or supplying a
dependency does not establish a live Resource connection, grant permission or change Pool ownership.
Resolved instance bindings retain the existing implementation-retention and replacement rules.
Exact import/export fields, default-binding representation and reference spellings remain authoring work.

### Distribution and execution

**Distribution direction:** objects should operate independently and eventually be deployable
across systems. One Pool can own Resources deployed on different systems; Conduits can span a
network. Pool ownership is not a placement or process-parenting rule. A unified Interface does
not require one execution host. The authority split and v0 outage rule below are accepted;
the first proof uses separate processes on one machine, as recorded in
[implementation review I1](../archive/reviews/IMPLEMENTATION.md#i1--independent-execution-and-management).
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
The initial persistence contract/engine is accepted below; exact packaging, transport and durability
mechanisms remain implementation work.

The same fixture will prove executable and requirement composition together: an authored plugin,
a graph-backed operation with derived/explicit results and recovery, composed requirements and a
custom requirement outside core dispatch, all inspectable through the Interface. The
[accepted proof scope](../../experiments/002-runtime-plugins/README.md#combined-composition-authoring-proof)
does not select final syntax, a permanent vocabulary or implementation start. The helper execution
default below is accepted separately; exact process grouping remains open.

### Shared policy and instance-owned state

Management is authoritative for templates, registration/naming and shared access policy. One
designated service is authoritative for each instance's resolved definition, confirmed
configuration/revision and operation results. The responsible service can host many instances;
management can host passive instance authorities and executors active ones. This state authority
is distinct from a Resource's logical Pool ownership and from a plugin implementing a handler.

**Interface-independent policy:** shared policy is independent of the Interface that administers
it. Multiple authorized Interfaces can administer the same policy; the owner-operated root has
broad default administration. Replacing an Interface requires valid access to the retained policy,
not a transfer of policy ownership or recreation of grants. A new Interface cannot acquire the old
one's authority merely by reusing its name. Existing recipient-participation, withdrawal and target
lifecycle rules remain intact. Narrower per-Interface authority stays deferred for v0.
The [authorization contracts](../proposals/authoring-syntax/AUTHORIZATION.md) formalize Policy as
authoritative requirement application and Grant as issued authority for all managed actors.
Management retains governing requirement bindings independently of the Interface that changed
them. A dedicated access-management Resource owned by the system Pool exposes grant and governing
binding administration/inspection. The existing system management Resource retains registry,
template and instance-creation responsibilities. Both may share a management process and storage
implementation; endpoint enforcement remains with its responsible implementations. Exact binding
representation and type-specific validation remain authoring work; startup and initial storage
choices are accepted below.

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

**Authority-state persistence:** v0 publishes a minimal typed persistence contract that plugins can
implement, starting with one bundled implementation. Management and execution services retain
ownership of their configuration, grants, progress and outcomes; the selected implementation
provides the storage and recovery guarantees those services require. The implementation needed for
initial loading must be available before registry access. Publishing this contract does not expose
raw authority-store writes to Agents or replace a Resource's domain-data capability. V0 loads the
plugin as a service runtime dependency, with status and supported recovery visible through the
Interface. Preserve the public semantics, implementation bindings and service-owned state so a future
Pool-owned persistence Resource can expose the same capability. Its foundational access/lifecycle
and deliberate migration need later design; that representation is not required in v0. The bundled
implementation initially uses SQLite, with separate local databases for management and each executor
in the first experiment. The public contract stays engine-independent. Exact schemas, Rust bindings
and durability settings remain implementation/authoring work; no cross-service transaction,
replication or atomic external provider effects are implied.

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

**Initial execution default:** locally authored plugin code runs in supervised helpers associated
with the instance it serves; trusted built-in Rust code may remain inside the executor. The
instance's designated service retains authority, progress and runtime supervision. This separates
ordinary helper crashes from the executor memory space without creating another object family
or providing an OS sandbox. It does not require a process per plugin, invocation or instance.
Graph composition alone needs no extra helper, and existing MCP services retain their own
supervision. Exact helper grouping, packaging and transport remain implementation choices.

**Initial distribution scope:** Conduit bundles first-party plugins and supports explicit
installation of locally supplied plugin packages. Packages may contain code, contracts binding
existing MCP services, graph-backed operations or templates. A marketplace and automatic dependency
resolution across programming languages are deferred. Installation makes contributions available;
it does not instantiate objects, authenticate providers or grant access. Exact package syntax
remains open; selected implementations still follow the existing retention/replacement rules.

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
policy and instance-state authorities defined above. The accepted outage rules still apply;
production placement and client enrollment remain review questions. Protocol accessibility alone
does not settle them.

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

**Foundational system:** initial local startup establishes a system Pool owning two management
Resources: the existing system management Resource for templates, registration and instance creation,
and a dedicated access-management Resource for grants and governing requirement bindings. The default
root Interface starts from a locally available definition before registry services exist. Its declared
graph directs this foundational setup and establishes management Conduits; the runtime supplies initial
owner recognition and minimum loading/persistence support. During explicit first initialization,
the runtime authorizes foundational setup from the verified local-owner context. Once access
management is ready, setup establishes ordinary recorded management grants for subsequent use.
Initialization authority does not remain an application bypass or silently reopen after an outage.
Both Resources can share a service and storage implementation. Explicit setup resume and foundational
repair use the locally loadable root Interface's declared recovery operations, with runtime owner
verification available even when ordinary management access is broken. Services load the accepted
minimal persistence plugin contract as a runtime dependency, with SQLite as the initial bundled
implementation; detailed storage/recovery mechanics remain implementation work.
Resource creation still goes
through its owning Pool. No application Pool, Agent or external service login is required first.
Built-in definitions, exact operations and frontend transport/lifecycle remain open.

**Later startup:** reuse the registered system Pool, both management Resources and default
Interface with their existing identities, saved settings and grants, subject to current access
checks. Missing or inconsistent expected components are reported for explicitly requested repair;
ordinary startup does not recreate them from defaults or silently retarget established bindings.
Preserve available records and backing data. An unavailable registry or service is not evidence
of deletion or a fresh installation. Supported reconnection to the same targets is distinct from
replacing registered objects. Repair is a deliberately requested operation, not an additional
confirmation on every launch. Repair is part of unrestricted Interface administration and uses the
root's declared recovery operations when the normal management path is unavailable. Explicit resume
checks retained progress and continues valid unfinished setup; uncertain effects require reconciliation.
Exact recovery methods and persisted schemas remain implementation/authoring work. Differentiated recovery authority and Interface
restrictions are deferred; this rule does not preserve ended flows or automatically restart Agent work.
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

**Extensible observation:** use published plugin contracts and authorized Conduits for observation
as well as control. Start with a lightweight bundled implementation for the common inspection and
retained-evidence foundation. Plugins can add instance-relevant observations such as model usage,
CPU measurements or service health without adding an object family or a fixed universal metric
catalog. These additions do not all have to ship in v0. A producer publishes supported typed
observations; a Conduit governs access/delivery; the Interface inspects them. Independently managed
collectors, history stores and exporters can be Pool-owned Resources, connected by Conduits.

Common correlation, freshness and coverage let the Interface combine sources; metric payloads and
their meanings belong to their versioned contracts. Report measurement scope and attribution: a
shared process's CPU usage must not be presented as an exact measurement of one hosted instance.
Observation alone grants no controls or secret access. Supported settings and new observation
attachments can be changed under existing control/connection rules; changing fixed definitions
or implementations still follows replacement rules. A separately attached observer need not
replace the observed instance when its existing capabilities suffice.

Managed components produce evidence even while no frontend is attached. Retention is independent
of a frontend's lifetime, and reconnecting frontends inspect the same underlying history/state.
The Interface is the unified access and interaction surface, not the sole producer of observations
or a requirement for an always-running UI. A dropped observation stream does not cancel Agent work.
Report gaps and stale/unknown state; a latest-state view need not replay every intermediate update.

**Observation ordering:** retained observation feeds define ordering and resume cursors within
each declared source. The Interface combines sources without promising one global event order.
Preserve each feed's order and known causal links; timestamps alone do not establish cross-source
causality or a global commit order. For example, two executors can independently report a database
becoming ready and an Agent starting without those reports establishing which preceded the other.

A source feed is a declared observation boundary, not automatically one process or one object.
This does not require sampled metrics or latest-state observations to provide lossless histories,
move authoritative state into the Interface, or grant access through a cursor. Exact cursor encoding,
snapshot/catch-up mechanics and delivery guarantees remain authoring work.

Export failure must not erase local committed outcomes. Diagnostic traces and metrics can refer
to retained evidence without becoming its sole copy. Preserve applicable content access and secret
handling; pluggable export does not imply putting credentials or every full payload into traces.
Optional capture, metrics and exporters cannot replace the required authoritative configuration,
operation outcomes and recovery records. Their failure or detachment does not erase that evidence
or change state ownership. Coverage includes what Conduit manages and what native integrations
actually report, with missing coverage visible.

The initial observation bundle supplies correlated operation/graph/plugin traces, structured
diagnostics and a small service-health metric set, with optional OpenTelemetry export. The low-volume
synthetic experiment records all instrumented operation traces at source and explicitly enables
bounded fixture-input/output capture; buffering/export gaps remain visible. Real prompts, documents,
tool payloads and helper output follow declared recording policies, including bounds and handling
of sensitive fields. Keep credentials out of ordinary logs. Message/history content retains its Resource
contract, without requiring duplicate full payloads in observations. Authoritative operation/recovery
evidence remains unsampled and independent of optional diagnostic pipelines. Retention until explicit
purge remains unchanged.

Specialized domain tracking can be supplied by custom Resources without changing
this common foundation or introducing a new family. Linear/Jira integration is independent of it.

