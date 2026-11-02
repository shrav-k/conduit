# Implementation review: decisions needed

Status: lightweight authoring support, contract publication, MCP, code-optional implementation
bindings, independent component access, management/execution separation and shared policy with
instance-owned state, no offline grants and the one-machine first proof are **accepted** below.
I1 and I2 are settled, including native adapters before the Rust loop and macOS-first native
integration/account validation. I3's conceptual extension, execution and distribution choices are
settled. I4's authority, startup/recovery and persistence choices are also settled; detailed
mechanisms and syntax remain open. I5 is conceptually settled: OS-backed custody through replaceable
implementation bundles and Interface-guided adapter enrollment/unlock. I6 accepts extensible
observation plugins and Conduits, a lightweight debugging bundle and explicit content-capture policy.
I1–I6 are conceptually settled; I7 is the remaining authoring/schema review.
Other recommendations remain proposed.
Read the [architecture](ARCHITECTURE.md) for the connected system and
[contracts](CONTRACTS.md) for details. This queue concerns implementation boundaries; it does not
reopen the five families, Pool ownership, fixed definitions, shared authentication or typed filters.

## Review priority: plugin and template authors

The user has clarified that these rulings remain collaborative. Do not turn the recommendations
into defaults or close the queue autonomously. Easy, expressive plugin and template authoring is
the primary acceptance concern; deployment and efficiency decisions support that goal. Existing
infrastructure may implement the contracts without becoming part of the public authoring model.

These three author journeys ground **I3's accepted conceptual extension boundary** and its
implementation tests. Defer I7's detailed contract/template syntax until the remaining conceptual
questions are settled:

1. **Compose without new code:** use existing filesystem, authentication and MCP capabilities in
   a reusable Pool/Agent template, with explicit bindings, setup and permitted controls.
2. **Add a small operation:** supply a typed handler, such as a checksum operation, and reference
   it from a graph without writing unrelated service lifecycle machinery.
3. **Add a managed capability:** supply a new Resource type with setup, ongoing behavior,
   observation and supported changes, then instantiate and connect it using the same framework.

For each journey, identify what the author writes, what reusable tooling supplies, what the
protocol guarantees, and how it is tested. Keep the accepted typed contracts, finite graphs,
instance-owned lifetimes and fixed implementation bindings; their precise authoring surface is
still to be settled. Template-only composition should not require inventing a new plugin.

**Accepted authoring direction:** begin with extremely lightweight support from Conduit, with
most integration behavior authored as plugins to test the approach and speed development. The
priority is the public protocol and extension structure, not building a library of helpers.
Use existing libraries and minimal adapters where useful; richer support can grow from actual
plugin-authoring needs. Plugins remain responsible for applicable effects/recovery behavior.
This does not mandate a comprehensive helper runner or require every author to hand-write a
protocol server. The [walkthrough](AUTHORING.md) illustrates compatible authoring paths and the
extra responsibilities of effectful operations and managed capabilities.

**Accepted publication boundary:** publish inspectable contracts with the implementation, so
Conduit can validate templates without starting plugin code. The
[filesystem example](AUTHORING.md#a-concrete-package-to-review) illustrates this boundary;
its filenames, fields and layout are not approved syntax. Published contracts do not certify
runtime readiness or implementation conformance. Provider-native dynamic catalogs remain separate.

**Accepted invocation boundary:** MCP is the public plugin protocol base, with a small versioned
Conduit profile for instance/operation context and lifecycle/recovery semantics. This does not
settle execution custody, exact profile bindings or transport, and does not require existing
provider MCP servers to implement the profile.

**Accepted implementation bindings and component access:** plugins publish contracts fulfilled
by authored code, existing local/remote MCP services, graphs of selected operations, or a combination.
No new plugin-specific executable is required when bindings, adapters or graph composition supply
the behavior. Supported MCP
capabilities can be used independently while the Interface aggregates control/observations.
The [design-fit audit](../../REVIEW.md#design-fit-audit-code-optional-plugins-and-independent-mcp-access)
retains Pool ownership, current access, explicit lifecycle and required evidence. No process per
component, implicit discovery grant or invented provider guarantee follows. Ordinary endpoint
configuration can remain a Resource template; the [search example](AUTHORING.md#worked-comparison-configuring-search-versus-publishing-a-search-capability)
shows when a reusable contribution is useful. Syntax remains deferred.

**Accepted execution custody:** separate execution services supervise Conduit-managed Agents
and Resources; management coordinates configuration/control and the Interface presents the system.
Each executor may host multiple instances, including initially on the same machine. External MCP
services retain their own supervision. This adds no object family or process-per-object rule.

**Accepted authority:** shared registration/template and access-policy authority; one designated
service owns each instance's resolved definition, confirmed configuration/revision and operation
results. The Interface aggregates sources with freshness and gaps. A service can host many instances.

**Accepted outage rule:** no offline grants in v0. Retain supervision and already-dispatched
outcomes; hold new managed requests when current permission cannot be established. Work with
verifiable permission can continue, subject to existing stop/revocation and delivery rules.

**Accepted first proof:** management and two independent executors run as separate processes
on one machine, with controlled connection failures. Validate across two machines before claiming
remote support. The experiment remains unimplemented.

**Accepted I2 delivery order:** after the synthetic runtime proof, validate real Agents through
native Claude/Codex adapters first, then build Conduit's own Rust model/tool loop. Both remain goals.

**Accepted initial platform:** macOS-first native integration and subscription-account validation,
with a portable Rust core and OS-specific adapters. Other platforms need their own evidence.

**Accepted framing:** retain the temporary names and divisions in the
[concept map](../../DESIGN.md#authoring-concepts-beneath-the-object-families), with final naming open.
Requirements compose obligations; graphs compose operations and can themselves implement typed
operations published through plugins. See the
[composite operation example](AUTHORING.md#publishing-a-graph-as-a-reusable-operation). Independent
requirement definitions, published support with runtime verification and endpoint-selected/managed
implementations remain accepted. Composite public results are derived by default from declared
public bindings and constituent contracts, with explicit construction available. Graph recovery
is the default for composed operations, resuming valid child progress on explicit retry and
preserving child recovery boundaries. Requirement contracts declare fulfillment timing/role duties;
established Conduits record the responsible implementations. Individual requirement enforcement
with optional composition checks is accepted. Experiment 002's combined composition-authoring proof
is accepted within the existing synthetic fixture. Locally authored code normally runs in supervised
instance-associated helpers; trusted built-in Rust code may remain in-process. Endpoint ownership
remains unchanged. Bundled first-party plugins and explicit local package installation are accepted;
a marketplace and automatic dependency resolution across languages are deferred. I3's conceptual
review is complete; exact process grouping and package syntax remain implementation/authoring work.
MCP, code-optional bindings and exact implementation retention are already settled; syntax stays last.

**Review order:** I1–I6 settle conceptual plugin, execution, authority, demonstration, persistence,
credential and observation boundaries. I7 now reviews identifiers, field names, template bindings,
package layout and schema-authoring conventions together. File fragments and code remain
illustrations until ruled on; a syntax choice does not implicitly select a new process or transport.

I1–I6 are settled at the conceptual level. Continue with I7 authoring syntax, using connected
package, contract and template examples. Preserve the existing IDs; this remains one review queue
rather than another decision ledger.

**Progress and remaining review:** the original R1–R7 direction review is complete. I4 already
settles the authorization model, independent grants, dedicated access-management Resource and
explicit governing targets with live ownership-based matching, and bounded issuance operations.
Root-directed first launch, owner-authorized initialization and explicit recovery through the root
Interface are also accepted, as is a minimal published persistence contract with one bundled
implementation loaded as a service runtime dependency. SQLite is selected, with separate local
management/executor databases for the first experiment. A future Pool-owned representation remains
a design direction. I5 settles custody adapters and Interface-guided enrollment/unlock. I6 settles
the lightweight observation package and capture policy. The consolidated I7 syntax/example pass remains.
Concrete counterexamples may reveal another choice; this is not a fixed count of future questions.
Do not turn routine schema fields, internal methods or tuning into additional ruling queues.
Neither implementation nor the runtime experiment has started.

## Pending choices

| Item | Decision | Recommendation |
| --- | --- | --- |
| I7 | Concrete authoring/schema conventions | Contract authority and composition without a primary plugin are accepted. Next choose a named plugin map with uniform export references, then complete the connected authoring example. |

I3 establishes the conceptual extension boundary. Accepted I1 and I2 determine how to execute
and demonstrate those contracts; I4 settles authority/persistence and I5 settles custody/interaction.
I6 settles observation and capture; I7 now finalizes authoring syntax. Numbers for
queue sizes, internal helper names and performance tuning need tests, not separate ruling queues.

## I1 — Independent execution and management

**Accepted execution arrangement:** use separate management and execution services for the first
implementation. Execution services supervise Conduit-managed Agents and Resources; management
coordinates configuration/control; the Interface presents the system. Each execution service can
host many instances, and several can initially run on one machine. External MCP services retain
their own supervision. Pool ownership is logical and does not determine placement. These services
are implementation roles, not new object families or a daemon per object/Pool.

The [distributed-runtime comparison](../../research/DISTRIBUTED_RUNTIME.md) and
[cloud follow-up](../../research/CLOUD_AUTHORITY.md) supply the rationale and alternative examples.
Their shared-policy/instance-state arrangement and the no-offline-grants v0 outage rule are
accepted. Storage choices and demonstrated remote availability guarantees remain separate.

**Accepted authority split:**

| Facts | Authoritative owner |
| --- | --- |
| Templates, registration/naming and shared access policy | Management authority |
| An instance's resolved definition, confirmed configuration/revision and operation results | One designated instance service; a service can host many instances |
| Backing domain data | The Resource backend/provider |
| Aggregate inspection | Interface views identify sources, freshness and gaps; no second writable authority |

Management can host passive instance authorities and executors active ones. State authority is
separate from a Resource's Pool owner and from the plugin code fulfilling a contract. Keep one
logical registry and one authoritative `spec.config`; cached views cannot independently change it.
Registration, names, retirement and other cross-authority changes retain checked coordination.
Storage engines, automatic migration/failover and exact service placement are not selected here.

**Change/recovery example:** a search Resource starts at limit 10/R7. Its control applies 15 and
the responsible service retains 15/R8 with the result, but the response to management is lost.
The Interface recovers the result from that service; an aggregate view still showing 10 identifies
its age. If the provider effect happened before any confirmation was retained, the outcome is
uncertain and still requires effect inspection or supported deduplication, never blind replay.

**Accepted outage admission:** no offline grants in v0. Retain supervision and recoverable
results; allow already-dispatched operations to finish within their admitted contracts. Hold new
managed requests when current permission cannot be established. Work with verifiable permission
can continue; existing explicit stop/revocation and isolation rules still apply. A live Agent or
graph is not blanket permission for its future tool calls or nodes.

The [concrete examples](../../research/CLOUD_AUTHORITY.md#concrete-outage-comparison-before-ruling)
explain the accepted tradeoff. If registration and authorization share a process, its outage can
block new capability calls even while executors remain alive and retain results. Other failures
may leave the relevant authority reachable. A separate logical responsibility is not a deployment
guarantee, and local authority does not inherently require external-network access.

Outcome retention does not itself authorize delivery; managed retrieval/delivery rechecks current
access. Holding dispatch need not mean indefinite queuing: pending versus temporary unavailability
follows the capability contract. No automatic replay policy is chosen. Bounded offline grants are
deferred. Native actions outside Conduit's mediation retain their actual enforcement limits.

**Example:** a dispatched migration may finish and retain its outcome while management is down.
The next protected operation waits if it cannot verify permission. On reconnection, current access
and effect-recovery rules apply; neither revoked access nor uncertain writes are replayed blindly.

**Accepted first proof:** experiment 002 runs management and two independently supervised
executors as separate processes on one machine, with controlled failures of their connections.
Use serialized service boundaries and independently retained state so shared memory or a common
process lifetime cannot hide protocol requirements. Test plugin/template authoring, retained
supervision, held unverifiable requests, target-owned results, stale views and verified reattachment.

Two-machine validation follows before claiming remote support. The initial proof does not establish
machine-loss tolerance, production remote enrollment or credential placement. Concrete transports,
worker packaging and storage remain review work; no fixture or failure guarantee is demonstrated
by documenting it. **I1's conceptual rulings and first-proof scope are complete.**

Accepted I3 helper execution and I4 persistence follow these authority boundaries. Frontend
detachment, explicit resume of stopped Agent work, current authority checks and no duplicate
unknown work remain accepted. This review does not reopen those behaviors.

## I2 — First real Agent path and supported platform

**Accepted delivery order:** after the synthetic runtime experiment, validate real Agents through
native Claude/Codex adapters first, then build Conduit's own Rust model/tool loop. Both remain goals;
no Pi runtime is adopted. This prioritizes subscription-account separation and shared Resource
access without pretending native adapters implement Conduit's provider/tool loop.

**Example:** native Agent instances exercise shared MCP and communication while Conduit owns
instance identity, connections, history Resources and attributable evidence. Actual tests must
verify account/private-state separation and report integration coverage gaps. The later owned-loop
driver supplies Rust-controlled context, provider dispatch and tool iteration. Native success
alone cannot establish compatibility of subscription authentication with direct provider access.

**Accepted initial platform:** validate native Agent integration and subscription accounts on
macOS first. Keep the Rust core portable and OS-specific process, credential and filesystem
behavior behind adapters. Other platforms require their own integration evidence before support
is claimed. This selects initial test coverage, not a permanent platform restriction.

Core compilation alone does not establish Linux/Windows native-driver or isolation support.
Exact CI coverage and additional platform schedules remain implementation work. The replaceable
Pi-inspired terminal remains a goal. **I2's delivery order and initial platform rulings are complete.**

## I3 — Plugin execution and distribution

**Accepted:** public plugin invocation uses MCP with a small versioned Conduit profile for
instance/operation context and lifecycle/recovery semantics. Published contracts may bind authored
code, existing local/remote MCP services through compatible adapters, graphs of operations, or a
combination. No new executable
is required for a declarative contribution. Independently usable MCP surfaces preserve managed
ownership/access/evidence without forcing every internal export or object to become a standalone
server. Hosted implementations cannot inherit a binary-pinning guarantee; retain exact
declarations/adapters and report actual compatibility evidence. Profile details remain open.

**Accepted — endpoint-selected and endpoint-managed plugins:** endpoint templates select the
implementations that supply their connection capabilities. Their instances retain the selected
implementations and manage local bindings, execution and cleanup. Conduits declare and match
compatible endpoint capabilities, coordinate restrictions and establishment, and retain logical
participation/relationship state. They may use orchestration plugins but do not select or install
new endpoint code. The [instance boundary](ARCHITECTURE.md#instance-execution-boundary) and
[authoring examples](AUTHORING.md#endpoint-selected-plugins-and-conduit-matching) explain the split.

This preserves fixed definitions, implementation retention and existing capability matching.
New connection parameters and dynamic provider catalogs can use supported contracts; genuine new
endpoint implementation behavior requires a revised definition/replacement. Releasing one binding
preserves other participation and independently shared Resources. Detachment does not roll back
confirmed target configuration or provider effects. Logical ownership does not choose a process
count, programming language or OS sandbox.

**Accepted design direction — composed typed requirements:** Conduit templates compose reusable
typed requirements, with a small base vocabulary and author-defined extensions. Endpoint-selected
plugins satisfy the applicable meanings; instances bind concrete endpoints and preserve relationship
state/evidence. [DESIGN](../../DESIGN.md#composable-relationship-requirements) records this model.
Publication syntax, detailed support matching, composition and runtime contracts remain under review.
Compatibility concerns reusable typed meanings and supported versions, rather than an implementation
enumerating every user-named Conduit template it can serve.

The [proposed requirement contract](CONTRACTS.md#composable-relationship-requirements--proposed-contract)
covers extension publication, support/evidence, composition, runtime responsibility and changes.
The [authoring walkthrough](AUTHORING.md#composable-conduit-contracts) separates
composition without code from adding a genuinely new requirement meaning. The
[research follow-up](../../research/CONFIGURATION.md#composable-requirements-research-follow-up)
compares CUE, JSON Schema, CEL and Cedar without selecting one as the engine.

**Accepted — reusable definitions, multiple implementations:** a requirement's versioned meaning,
typed parameters and applicability are independent of any single implementation. Multiple endpoint
plugins can implement the same definition; definition and implementation may ship together without
separate packages or duplicate schemas. An allowed-regions contract can therefore be reused by two
deployment integrations. Exact version/schema fields and publication mechanics remain open.

**Accepted — published support, runtime verification:** endpoint plugins publish supported
requirement versions and endpoint roles, including known limitations, in the contract bundle.
Conduit checks declared compatibility before establishment. A deployment plugin might support
allowed regions for direct deployments but not an operation that selects a region indirectly;
the requirement name alone does not imply support for every use. The shared definition supplies
its schema. Current authentication, readiness and actual enforcement still require applicable
runtime verification. Otherwise valid configuration may remain registered but unready. This does
not select a general schema-subtyping solver, negotiation format or enforcement API.

**Accepted — reusable composition:** a requirement definition may compose existing requirements
without new plugin code when endpoint implementations support every constituent. For example,
`review-access` combines permitted read operations, a project restriction and a result-size limit
for reuse across Conduit templates. Constituents retain their meanings and limits; a bundle name
supplies no permission or implementation behavior. Nesting/parameter-binding syntax remains open.

**Accepted — provisional vocabulary and executable composition:** keep the current concept names
and divisions as temporary working terms; final names may change after conceptual review. A graph
can implement a typed operation contract and be published through a plugin for reuse by callers or
other graphs. This creates a composite plugin from an author's perspective: the graph implements
behavior and the plugin publishes its contract/binding. It adds no object family or process per
composition; exact declaration syntax remains open. A plugin may export several operations; template-local
graphs need not become published plugins. Existing finite-graph, authority, implementation-retention
and instance-supervision rules continue to apply.

**Accepted — derive public results by default:** derive the initial result contract from declared
public output bindings and constituent contracts; authors may explicitly construct a domain result
instead. Account for composed success/failure modes and preserve relevant effects, uncertainty and
links to scoped evidence without automatically publishing internal outputs. Detailed derivation,
mapping and recovery semantics remain open. See the
[proposal](CONTRACTS.md#composite-outputs-and-outcomes--proposal) and
[database example](AUTHORING.md#derived-and-authored-composite-results).

**Accepted — reuse graph recovery by default:** graph-backed operations reuse retained child
progress on an explicit retry of the same unchanged invocation, reusing only still-valid results
and delegating uncertain effects to child recovery contracts. Authors need no bespoke recovery
implementation merely for composition. A parent resumes the existing composite rather than
blindly reissuing its graph. This does not enable automatic retries or promise rollback or recovery
unsupported by a child. Detailed coordination and custom recovery hooks remain open.

**Accepted — declared fulfillment duties:** each requirement contract states when its guarantee
must hold and applicable endpoint-role obligations. An established Conduit records the responsible
selected implementations, inspectable through the Interface using existing connection/evidence
state. A workspace restriction can require enforcement before each managed provider request by the
Resource-side MCP plugin, while the Agent-side plugin presents permitted tools. Other requirements
can apply before result delivery or at establishment. This imposes no universal fixed phase list,
extra authored manifest or always-running checker. Exact evidence and evaluation mechanisms remain open.

**Accepted — individual enforcement, optional composition checks:** retain and enforce each
constituent requirement at its declared point. Optional checks can detect known conflicts or explain
combined effects, such as two limits of 20 and 10 imposing a maximum of 10. Custom authors need no
general solver or merge algorithm. Known conflicts are reported and unproved global compatibility
remains explicit; an unverifiable required check still blocks affected behavior. Exact diagnostics
and optimization rules remain open.

**Accepted — combined composition-authoring proof:** Experiment 002 will exercise executable and
requirement composition together using its existing fake service and two synthetic consumers.
Author a small plugin, export a graph-backed operation reusing it, and test derived/explicit results
plus valid-progress recovery. Compose starting requirements and add a custom requirement outside
core dispatch. Operation allowlisting, result limits and allowed regions on a fake deployment remain
candidate examples. Inspect implementation selection, fulfillment duties, child progress and
allowed/rejected outcomes through the Interface. The
[accepted proof scope](../../../experiments/002-runtime-plugins/README.md#combined-composition-authoring-proof)
adds no live cloud dependency or broad SDK prerequisite. It does not approve exact syntax, permanent
core vocabulary or implementation start. The physical execution default is accepted separately below.

Final package/schema spelling remains I7 work. Graph representation, publication and runtime
supervision retain their distinct responsibilities.

The conceptual composition review is recorded in these groups; detailed mechanisms remain open:

1. **Composition and authoring:** distinguish the accepted composition of obligations from the
   procedural composition of plugin operations. Graph-backed callable operations are accepted;
   default result derivation and graph recovery are accepted. Exact coordination/custom hooks
   remain open; consolidate final names after the concepts stabilize. Detailed representations
   remain I7 work, using existing publication without another object family or broad SDK prerequisite.
2. **Composition and execution:** individual enforcement with optional composition checks,
   fulfillment duties and responsible-implementation records are accepted. Detailed evidence,
   changing-fact handling and evaluation interfaces remain open. Preserve whole-path narrowing,
   current access and effect/recovery rules rather than inventing new authority through requirements.
3. **First vocabulary and proof:** the combined synthetic proof is accepted, including an external
   custom requirement without core changes. The candidate fixture examples do not constitute a
   permanent core vocabulary; detailed artifact/schema choices remain for I7.

Typed plugin input/output schemas and Resource configuration contracts remain. The Conduit owns
relationship requirements and composition; it cannot redefine target operation semantics or claim
unsupported enforcement. MCP remains the public invocation base. The proposal does not select a
universal endpoint API, Boolean expression language, constraint solver or OS sandbox. Detailed
establishment/cleanup remains implementation work; I7's syntax choices remain in the queue.
I4's persistence, I5's custody/interaction and I6's observation/capture boundaries are settled.

**Accepted — physical execution of authored code:** locally authored plugin code normally runs
in supervised child/helper processes associated with the instance whose activity it serves.
For example, a directory Resource's handler runs in a helper associated with that Resource;
its execution service retains supervision, authoritative progress and confirmed configuration.
An ordinary helper crash can be handled without that helper sharing the executor's memory space.
This is physical placement, not a new owner or independently managed object family: endpoint
instances still select/manage their connection implementations, and Conduits coordinate the relationship.

Trusted built-in Rust code may remain in the executor process. Graph-backed operations reuse graph
execution without a separate helper merely for composition; authored child operations use the selected
execution path. Existing MCP services retain their own supervision, and declarative service bindings
need no new worker. This selects neither one process per invocation/plugin/instance nor an OS sandbox.
An instance-associated runner could host several compatible handlers; exact sharing/granularity must
preserve identity, credentials, cancellation and retained implementations and remains separate.

**Rationale:** helpers separate ordinary authored-code crashes from the executor memory space and
support language-neutral implementations through the public MCP contract. The cost is process and
protocol plumbing; instance authority, recovery and supervision remain with the designated service.
Languages, helper packaging, transport and any future Wasm lane remain separate. The
[execution examples](AUTHORING.md#5-where-the-example-implementations-execute) remain illustrations,
not completed implementation evidence.

**Accepted — distribution scope:** v0 supports bundled first-party plugins and explicit
installation of locally supplied plugin packages. A marketplace and automatic dependency resolution
across programming languages are deferred. For example, an author supplies a checksum package
with contracts and a prepared executable; a Linear package can instead contain declarations binding an existing MCP
service through an available adapter. Neither needs a hosted Conduit package registry. Installing
a package does not instantiate Resources, authenticate providers or grant access. Authors can
use existing language tooling to prepare their implementations; package syntax and prerequisites
remain for the final authoring review.
A Pool-owned Resource retains worker A when a new release makes B available; a replacement can
select B and explicitly reuse compatible backing. Exact Conduit-controlled implementation retention
and replacement on upgrade are already accepted, not new choices in this review.

The experiment must test whether the Conduit MCP profile stays small before treating its method
set as stable. Protocol support alone does not establish provider lifecycle or effect recovery.
This closes I3's conceptual review. Lightweight author support, code-optional integrations and
existing-service reuse remain required; the helper default does not require every package to ship
code. Detailed package syntax, transport, fulfillment evidence and recovery mechanisms still need
implementation and conformance work; no implementation has started.

## I4 — Local transactional authority

**Status: conceptually settled.** Authority, startup/recovery, the minimal public persistence
contract, service runtime dependency and initial SQLite engine are accepted. Exact schemas,
implementation mechanisms and validation remain work to do.
The runtime can start the root Interface from a locally available definition before registry
services exist. Its declared graph establishes the system Pool, system management and dedicated
access-management Resources, and their management connections. Runtime support supplies initial
owner recognition and minimum loading/persistence. Continuing administration/observability use
the existing Interface role; no new object family or autonomous reconciliation loop is introduced.
See the [Interface walkthrough](ARCHITECTURE.md#interface-as-system-director).

**First authorization is accepted:** during explicit initial setup, the runtime authorizes the
selected root's foundational graph from verified local-owner context. Setup establishes ordinary
recorded management grants once access management is ready, then uses normal managed paths. This
[initialization boundary](ARCHITECTURE.md#first-authority-and-grant-establishment) breaks the first-grant
dependency without leaving an application bypass or silently reactivating after an outage. The
definition cannot authorize itself, and ordinary Agents receive no offline grants.

**Explicit recovery route is accepted:** partial-setup resume and repair of broken management access
use the locally loadable root Interface's declared recovery operations, with runtime owner
verification available independently of that broken path. The [recovery examples](ARCHITECTURE.md#interrupted-setup-and-repair)
retain valid progress, identities, data and outcomes and reconcile uncertain effects. This does not
authorize automatic reinitialization. Exact methods and retained-record layout remain implementation/
authoring work.

**Persistence extension is accepted:** publish a minimal typed contract through the plugin machinery
and start with one bundled implementation. Management/executors retain state-transition and authority
ownership; Resource domain-data backends remain independent. The implementation needed for initial
loading must be available before registry access. The [examples](ARCHITECTURE.md#concrete-persistence-examples)
distinguish grant persistence, database effect recovery and shared message content. This does not
select an engine, require several implementations or finalize public method/schema names.

**Store representation is accepted:** load the persistence plugin as a service runtime dependency
in v0, with selection, failures and supported recovery visible through the Interface. The
[lifecycle](ARCHITECTURE.md#store-representation-and-lifecycle) follows service setup, explicit repair
and maintenance, without a separate storage Resource to retire. Preserve the public contract and
service-owned state to support a [future Pool-owned representation](ARCHITECTURE.md#future-pool-owned-persistence-resources).
Its self-starting access/lifecycle and deliberate migration need later design; these do not reopen
the v0 choice or add another blocking review bundle.

**Initial engine is accepted:** use SQLite for the bundled implementation, with separate local
databases for management and each executor in the first experiment. Services communicate through
their contracts; the public persistence contract remains engine-independent. This does not provide
cross-service transactions, atomic external provider effects, replication or failover. Exact Rust
bindings, tables and durability settings remain implementation/authoring work.

**Accepted authority ruling:** policy is independent of any particular Interface. Multiple
authorized Interfaces can administer it; the owner-operated root has broad default administration.
Replacement establishes valid administrative access without transferring policy ownership or
recreating grants. Existing recipient/target/withdrawal rules remain; reusing a name cannot preserve
an old instance's authority. This does not narrow today's broad v0 Interface administration.
See the [cloud research](../../research/CLOUD_AUTHORITY.md#9-interface-administration-and-independent-policy)
and [lifecycle example](ARCHITECTURE.md#root-authority-lifecycle-comparison).

**Current direction:** the user explicitly requires formal Policy and Grant structures alongside
requirements, plugins, graphs and operations. They cover Agents as well as Interfaces and remain
beneath the five families. Independence and that formalization direction are accepted; the details
below require joint rulings. The [research](../../research/AUTHORIZATION.md) compares Cedar,
Kubernetes RBAC, OPA, Zanzibar, Macaroons/Biscuit and MCP/OAuth. The
[authorization proposal](AUTHORIZATION.md) is the canonical semantic structure; I7 still owns syntax.

**Recorded participation grants are accepted:** ordinary managed use, including broad root
management, requires a grant on the selected participation. Policy governs issuance and current use;
there is no grantless direct-policy alternative. Authorized connection setup can establish a grant
covering many requests without another user manifest or per-call token. The accepted owner-authorized
first initialization establishes the initial root grants once access management is ready.

**Grant administration/lifetime is accepted:** check the administrator's current authority when
issuing, changing or revoking a grant. Issued grants are independent of issuer provenance, later
permissions and grant ancestry. Another authorized Interface or Agent may revoke them; ending
recipient participation/the related Conduit ends that access. Current governing policy and
requirements remain effective. Normal operation attribution is evidence, not a required ancestry
chain or live access dependency. The proposed administrative/derived lifetime distinction is removed.

**Policy/Requirement consolidation is accepted:** [Policy is authoritative requirement application](AUTHORIZATION.md#3-policy-as-authoritative-requirement-application).
Grant administration and governing-binding changes are ordinary protected capabilities using
recorded grants and requirements, without a separate policy language or special administration
rule category. Governing bindings remain independent managed state. Existing all-applicable checks,
complete-path alternatives and mandatory-check failure behavior are settled and carry forward.

**Management placement is accepted:** the system Pool owns a dedicated access-management Resource
for grants and governing requirement bindings, alongside the existing system management Resource
for templates, registration and instance creation. Both may share a management service and storage
implementation; this requires no separate process/database or centralized endpoint enforcement.
Interfaces and authorized Agents administer access through Conduits. Exact type/capability names
and first-establishment/repair wiring remain open.

**Explicit governing targets are accepted:** bindings distinguish an object's own operations from
operations on explicitly selected owned Resources. Pool ownership and the location of a declaration
do not propagate requirements automatically. A Pool template can bundle both kinds of rules, and
an explicit supported ownership scope can cover multiple Resources. Explicit ownership-based
governing selections now match live at applicable checks as Resource instances enter/leave the
selected set, without provisioning or grants. Explicit references retain their resolved stable IDs.
Exact selectors remain open; this does not restore generic object labels/selectors.

**Bounded issuance is accepted:** [issuance operations may supply mandatory use bounds](AUTHORIZATION.md#carrying-use-bounds-into-issued-grants),
compose additional restrictions and return the complete grant. Requested intent stays explicit;
omissions do not mean maximum administrable access and explicit broader requests are refused.
Strict complete-proposal operations may coexist. Retained restrictions belong to the new grant;
issuance-only checks do not become live issuer dependencies. Detailed schemas/type-specific checks
are I7/conformance work.

**Next review:** the I7 contract, package and template syntax pass.
Exact loader/record fields, storage settings and recovery methods remain in I7 or implementation
work; they need not become individual conceptual rulings.

The proposal's field inventories, identity mapping, decision/evidence contract and candidate
Experiment 002 cases are not adopted schemas or new experiment scope. It chooses no evaluator,
token format or public policy DSL. Local root-definition loading and graph-directed
establishment, owner-authorized initialization, explicit recovery route, minimal published
persistence contract, service runtime dependency and SQLite engine are accepted; the detailed format
remains open. The future Resource representation is deferred, with its migration path kept in view.

The [recovery examples](ARCHITECTURE.md#worked-storage-and-recovery-examples) establish the relevant
failure cases; the [storage boundary](ARCHITECTURE.md#storage-boundary) separates
contract, representation and engine. The minimal public contract and one bundled implementation are
accepted, as are the service runtime dependency and SQLite engine. Detailed recovery algorithms
remain implementation work. The accepted management/instance-state authority split is unchanged; storage supplies persistence without
becoming a second owner of policy or instance transitions. A future Pool-owned Resource could expose
the same capability with additional acyclic loading, self-administration and migration rules. That
does not require another object family or moving Resource domain data into the authority store.
Exact startup configuration and public schemas remain open.

**Selected engine:** SQLite supplies local authority storage in the first bundled implementation.
In Experiment 002, management retains templates/registration/policy,
while the two executors each retain their assigned instances' definitions, confirmed revisions and
operation progress/results. The Interface aggregates source-attributed views. A search Resource
can still use Xerj, a database Resource PostgreSQL, and a directory Resource the filesystem.
[SQLite usage guidance](https://sqlite.org/whentouse.html)

The architecture's bounded store thread and WAL/FULL commits remain proposals. Any persistence
implementation must support the promised checked revisions, atomic groups, durable outcome records
and recovery from uncertain commits. An external plugin adds reachability/bootstrap obligations;
an internal adapter still needs migrations and conformance tests. No cross-service transaction,
automatic failover or offline permission grant follows. Services exchange information through their
contracts; a WAL database is not shared across machines. [WAL constraints](https://sqlite.org/wal.html)

I4 is closed at the conceptual level. Detailed SQL, method names and helper organization need
implementation and conformance evidence rather than additional conceptual rulings.

## I5 — Credential custody and account compatibility

**Accepted:** secrets Conduit itself retains default to an OS credential store, supplied as a
replaceable adapter through the existing plugin contract/implementation-bundle machinery.
Publish a minimal typed custody contract with the bundled OS implementation; later password-manager
or credential-service implementations may satisfy it without changing the object model. macOS
Keychain is the first platform implementation. Additional providers are not required in v0, and
in-memory fake custody remains available for tests rather than as a production fallback.

Apple provides Keychain services for encrypted storage of small secrets, including keys and
passwords; the backend supplies custody, not Conduit's permission system.
[Apple Keychain services](https://developer.apple.com/documentation/security/keychain-services)

| Credential | Accepted ownership and handling |
| --- | --- |
| A service token/refresh secret Conduit retains for a shared authentication Resource | Store through the selected OS-backed custody implementation; ordinary configuration, SQLite state and events carry a reference rather than the secret value. |
| A native Agent's subscription sign-in/refresh state | Retain the accepted native-runtime ownership; this ruling does not move it into Conduit's custody or copy it among Agents. |
| A secret already managed by an external credential service | Its declared Resource/integration contract retains that ownership; the local default does not require importing every external secret. |

An ordinary Agent's grant to invoke shared Linear tools does not by itself grant permission to
retrieve the upstream credential. Locked, denied or unavailable custody is reported for supported
unlock/recovery; no silent plaintext fallback follows. A custody implementation declares its
supported behavior and limitations rather than implying that every backend offers identical
write, renewal or unattended-access capabilities. Implementation retention/replacement rules apply;
installing a different bundle does not migrate secrets or reinterpret existing references.

**Example:** one authentication Resource supplies appropriate service bindings to shared integrations.
Agents receive permitted capability use, not a token copied into each private profile. Meanwhile,
each native model account's refresh/logout is tested against the other account and against separate
Agent histories. This cannot be proved by choosing different directory names alone.

The [custody bundle example](AUTHORING.md#credential-custody-as-an-implementation-bundle) separates
published support, implementation and authentication ownership without prescribing final syntax.

**Accepted enrollment/unlock:** the Interface guides the selected adapter's supported flow. Existing
authorized access is reused. When human action is required, the affected operation reports that
need through the Interface and the user completes the native/provider flow. Unattended execution
cannot complete that step until access is available; the instance retains progress and recovery
follows the operation's contract. Unrelated work can continue. Passwords and tokens remain outside
ordinary conversations and logs. This reuses existing human-interaction waits and preserves native
authentication ownership, without introducing another Conduit login.

I5 is closed at the conceptual level. Exact item/reference, interaction and bundle schemas belong
in I7; adapter behavior, challenge recovery and separate-account compatibility still require tests.

## I6 — What observability captures by default

**Accepted:** use adaptable observation plugins and authorized Conduits, beginning with a lightweight
bundled implementation. Instance plugins publish supported observations; Pool-owned Resources can
collect, retain or export them; the Interface presents the combined investigation view. Observation
is part of the Interface's core function, independent of whether a frontend is currently attached.

The initial package includes correlated operation/graph/plugin traces, structured diagnostics and
errors, a small service-health metric set, and optional OpenTelemetry export. The Interface connects
failures to their actors, instances, access paths, configuration, retained outcomes and supported
recovery actions. Start with measurements useful for debugging durations, pending work, reconnects,
persistence failures and observation loss; exact names and fields remain authoring work, not a
closed metric vocabulary. Token/CPU collectors and external dashboards are extensions, not prerequisites.

Always retain authoritative operation identity, actor/target/path, effect outcomes, coverage and
the safely retained inputs needed for promised recovery. These records are independent of diagnostic
sampling, buffering and export. Exporter loss cannot erase committed outcomes, and an exporter
receipt cannot establish that an operation applied. Report missing, stale, truncated or dropped
observations explicitly; unavailable measurements are not zero.

For the low-volume synthetic Experiment 002, record all instrumented operation traces at source
and explicitly enable bounded fixture-input/output capture. This does not promise lossless delivery:
report buffering/export gaps. Use fake credential references and known fixture data. For real prompts,
documents, tool arguments/results and helper output, full-content capture requires a declared
operation/type recording policy with bounds, scoped content references where appropriate and
sensitive-field handling. Keep credentials out of ordinary logs. Message/history content retains
its Resource contract; observation does not create a second mandatory transcript store.

**Example:** the Interface can show that a Linear call was admitted, which Agent/account/path
performed it and whether application was confirmed. It may expose a permitted stored response,
an excerpt or only a reference depending on recording policy. A missing full response is not
complete transcript coverage. If omitted input prevents replay, report that recovery limitation.

Supported observation attachments/settings use the existing connection/control rules; fixed
definition or implementation changes still require replacement. Retain captured evidence until
explicit purge under the agreed lifecycle; no automatic-expiry rule or external backend is selected.
The [research](../../research/OBSERVABILITY.md) supports this package. Its additional backend, library
and wire recommendations remain proposals. I6 is conceptually closed; I7 settles concrete syntax.

## I7 — Authoring conventions and multi-plugin graphs

**Accepted — contract authority:** the published, language-neutral contract defines a plugin's
public API and behavior. Implementations conform to it. Authors may handwrite or generate the
contract, then explicitly review and version the released definitions. Internal Rust types do
not independently redefine public inputs, outputs or guarantees. For example, an implementation
may rename its internal `batch_size` variable while `set-query-limit` still accepts public input
`limit`, an integer from 1–100. Data schemas alone do not define effects, authorization, recovery
or lifecycle guarantees; the behavioral contract remains necessary. No schema dialect, package
layout or SDK is selected by this ruling. Dynamic provider catalogs retain their agreed boundary.

**Accepted — v0 composition and defaults:** templates may fulfill their type contract through
explicit compatible contributions without a mandatory primary implementation. Complete standard
templates, reusable operations and declared type defaults keep ordinary authoring simple. Defaults
may include handler implementations; they resolve into the instance's retained bindings without
making one package primary. No implicit merging of installed packages, template inheritance or
new preset family is required. Explicit invalid selections do not silently fall back to defaults.

Compatibility includes operation meaning, state/runtime expectations and recovery semantics, not
only matching input shapes. Validate declared contracts/bindings, verify current identity, access,
placement and readiness at their required points, and use conformance/integration tests as evidence.
No general compatibility solver or central certification service is required. Existing schema-write,
readiness, exact-implementation retention and replacement rules continue to apply. The
[walkthrough](AUTHORING.md#worked-comparison-two-plugins-in-one-directory-template) explains the
accepted model, including [compatibility](AUTHORING.md#compatibility-what-must-agree) and
[defaults](AUTHORING.md#defaults-values-behavior-selections-and-complete-recipes).

### Next ruling: one plugin map and one export-reference form

**Recommendation:** use `spec.plugins` as a map of locally named implementation selections, with
`{plugin, name}` references wherever a graph operation, runtime driver or control handler selects
an export. An illustrative template fragment:

```yaml
plugins:
  filesystem:
    package: example.filesystem
    release: 0.1.0
  checks:
    package: example.project-checks
    release: 0.1.0
setup:
  nodes:
    attach:
      operation: {plugin: filesystem, name: directory.establish}
      inputs:
        configuration: {from: config}
    check:
      operation: {plugin: checks, name: verify-layout}
      inputs:
        directory: {from: output, node: attach, port: directory}
        requiredEntries: [Cargo.toml, src]
runtime:
  driver: {plugin: filesystem, name: directory.managed}
  inputs:
    directory: {from: output, node: attach, port: directory}
```

The names are local aliases, not object IDs, authority, process placement or primary roles. Export
validation still distinguishes operations, drivers and handlers. Package/release fields are a draft
selection format; exact retained identity and package details remain to be reviewed. Defaults must
resolve to known bindings too. This example does not settle nested graph/package alias scoping.

**Alternative:** put the complete implementation selection beside each export reference, for example:

```yaml
operation:
  implementation: {package: example.filesystem, release: 0.1.0}
  name: directory.establish
```

This makes an individual binding self-contained but repeats selections across setup, runtime and
controls. The recommended map gives the author one place to select each reused contribution.
Neither option restores a primary implementation. The map name and reference fields remain
proposed; the updated fixtures illustrate them for review rather than ratifying syntax by example.

**Remaining authoring pass:** use the chosen structure to review one package and consuming template
with graph/control bindings and published schemas. `conduit/v0alpha1` as the common API identifier,
uniform group/name/version type references across all five families, and the concrete schema dialect
are still proposed. Parser scalar edge cases, numeric precision limits, schema-default coverage
and negotiated worker-feature details are implementation/conformance work; a failing example
should surface a concrete decision rather than another speculative questionnaire.

## Decisions not requested again

We retain the five families; Pool-only Resource ownership; templates as complete recipes; finite
typed graphs; instance-owned runtimes; current authority checks; native tool preservation; fixed
instance definitions and implementations; supported controls; shared service auth; private Agent
context; retained communication with independent readers; no default automatic wake; and explicit
cleanup/purge. None is being quietly traded away for convenience in a crate or framework.

## After review

Record rulings here in place, fold any changed accepted behavior into DESIGN, and update the
concrete experiment packet before writing code against its new syntax. Implement M0–M2 first.
Use the runtime experiment to resolve engineering uncertainties, then execute the selected real
Agent path. This queue should shrink as choices settle, not become another permanent micro-decision ledger.
