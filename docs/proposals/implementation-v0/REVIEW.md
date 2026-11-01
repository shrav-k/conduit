# Implementation review: decisions needed

Status: lightweight authoring support, contract publication, MCP, code-optional implementation
bindings, independent component access, management/execution separation and shared policy with
instance-owned state, no offline grants and the one-machine first proof are **accepted** below.
I1 and I2 are settled, including native adapters before the Rust loop and macOS-first native
integration/account validation. I3–I7 retain the choices identified in this review. Other recommendations remain proposed.
Read the [architecture](ARCHITECTURE.md) for the connected system and
[contracts](CONTRACTS.md) for details. This queue concerns implementation boundaries; it does not
reopen the five families, Pool ownership, fixed definitions, shared authentication or typed filters.

## Review priority: plugin and template authors

The user has clarified that these rulings remain collaborative. Do not turn the recommendations
into defaults or close the queue autonomously. Easy, expressive plugin and template authoring is
the primary acceptance concern; deployment and efficiency decisions support that goal. Existing
infrastructure may implement the contracts without becoming part of the public authoring model.

Use these three concrete author journeys to review **I3's conceptual extension boundary first**;
defer I7's detailed contract/template syntax until all conceptual questions are settled:

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
is accepted within the existing synthetic fixture. Next settle physical execution of authored
plugin code, then distribution scope; endpoint ownership remains unchanged.
MCP, code-optional bindings and exact implementation retention are already settled; syntax stays last.

**Review order:** settle remaining conceptual plugin, execution, authority, demonstration,
persistence, credential and observation questions before detailed syntax. I7's identifiers,
field names, template bindings, package layout and schema-authoring conventions are deferred to
the final pass. File fragments and code remain illustrations. No language, process layout or
transport is selected by the authoring/publication/protocol rulings.

I1 and I2 are settled. Review I3's requirement contracts and execution/distribution choices, then I4–I6's
persistence/custody/evidence support. Preserve the existing IDs; this is a
change of review order, not another decision ledger. Finish with the deferred I7 syntax review.

## Pending choices

| Item | Decision | Recommendation |
| --- | --- | --- |
| I3 | Requirement extension contracts, execution and distribution | Composable requirements and endpoint-selected/managed plugins accepted; settle publication, satisfaction and a minimal proof before placement. |
| I4 | Authoritative persistence | SQLite for local control/evidence; Resource backends remain replaceable. |
| I5 | Credential custody | OS-backed Conduit service secrets, native-runtime-owned Agent authentication; no implicit plaintext fallback. |
| I6 | Default observability payload capture | Durable metadata/recovery inputs plus explicitly classified content capture; no blanket full payload logging. |
| I7 | Concrete authoring/schema conventions — review last | Uniform type fields, owner implementation plus named plugin aliases, reviewed JSON Schema bundles; proposed spellings remain deferred. |

I3 establishes the conceptual extension boundary. Accepted I1 and I2 determine how to execute
and demonstrate those contracts; I4–I6 determine retained state and evidence. I7 finalizes their authoring syntax
after those conceptual rulings. Numbers for
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

I3 worker details and I4 persistence choices follow these accepted authority boundaries. Frontend
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
core vocabulary, physical plugin placement or implementation start.

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
universal endpoint API, Boolean expression language, constraint solver, worker arrangement or
sandbox. Establishment/cleanup and I4–I7's persistence, credential, observation and syntax choices
remain in the queue.

**Next ruling — physical execution of authored code:** recommend supervised child/helper processes
for locally authored plugin code, operating in the context of the instance whose activity they
serve. For example, a directory Resource's handler runs in a helper associated with that Resource;
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

**Alternative:** load locally authored plugin code into the executor process through an extension
API initially. This reduces process/protocol plumbing but shares that executor's memory/failure
boundary and requires compatible loading/retention. Both arrangements preserve the same public MCP
contract and logical instance ownership. Languages, helper packaging, transport and any future Wasm
lane remain separate. The [execution examples](AUTHORING.md#5-where-the-example-implementations-execute)
remain illustrations, not completed implementation evidence.

**Subsequent ruling — distribution scope:** recommend first-party bundled artifacts and explicit
local package installation, deferring a marketplace and arbitrary-language dependency resolver.
A Pool-owned Resource retains worker A when a new release makes B available; a replacement can
select B and explicitly reuse compatible backing. Exact Conduit-controlled implementation retention
and replacement on upgrade are already accepted, not new choices in this review.

The experiment must test whether the Conduit MCP profile stays small before treating its method
set as stable. Protocol support alone does not establish provider lifecycle or effect recovery.
Execution and distribution recommendations above remain unaccepted; lightweight author support,
code-optional integrations and existing-service reuse remain required under either arrangement.

## I4 — Local transactional authority

**Recommendation within the accepted I1 boundaries:** SQLite for each chosen local durable authority, accessed
through a bounded store thread with WAL/FULL commits. Registry revisions, target execution
outcomes and aggregated observations need explicit ownership; an independent execution service
cannot rely exclusively on a vanished management process's in-memory state. Resource backends
remain separate. This is not a proposal to share one SQLite file across machines or to permit
multiple writable copies of an instance spec.

**Example:** Xerj is offline but the Interface can still show that a search Resource is unavailable,
what setup completed and which control is uncertain. Rebuilding its search index does not change
an Agent's ID or erase a delegation. A stopped exporter does not lose local operation outcomes.

**Alternative:** make the control store itself replaceable through an external plugin from day one.
That requires a stronger transactional/concurrency/bootstrap contract and could make repair depend
on the failed plugin. JSONL-only storage is simpler to inspect but moves transaction/index/compaction
responsibilities into our code.

**Ruling needed:** choose local persistence for the accepted authorities and outage/recovery requirements.
Which facts commit together locally, and which use recoverable inter-service exchanges, must be
explicit. This does not make SQLite a required backend for every Resource or settle federation.

## I5 — Credential custody and account compatibility

**Recommendation:** explicit OS credential-store support for secrets Conduit itself owns, with
in-memory fake custody for tests. An unavailable/locked store reports a wait/error instead of
falling back silently. Native Agent adapters keep authentication under their supported native
runtime ownership and prove separate-account behavior. Pool service identity remains separate.

**Example:** one authentication Resource supplies appropriate service bindings to shared integrations.
Agents receive permitted capability use, not a token copied into each private profile. Meanwhile,
each native model account's refresh/logout is tested against the other account and against separate
Agent histories. This cannot be proved by choosing different directory names alone.

**Alternative:** begin with explicitly protected local credential files, postponing OS-store
integration. That simplifies automation/headless setups but changes secret-at-rest expectations and
requires an explicit storage and unlock policy. It must not be an undocumented fallback.

**Ruling needed:** secret-store default and acceptable first-platform enrollment experience. This
does not require another Conduit login or change native runtime authentication ownership.

## I6 — What observability captures by default

**Recommendation:** always retain operation identity, actor/target/path, effect outcomes, coverage
and the safely retained inputs needed for promised recovery. Retain message/history content under
its Resource contract. Capture other full tool/model payloads only under an explicit operation/type
recording policy, with content references, bounds and sensitive-field handling. Show missing,
truncated or deliberately uncaptured content clearly. Never log credentials by default.

**Example:** the Interface can prove a Linear call was admitted, which Agent/account/path performed
it and whether application was confirmed. It may have a permitted stored response, an excerpt or
only a reference depending on the declared policy. A missing full response is not presented as
complete transcript coverage. If omitted input prevents replay, that limitation is recorded.

**Alternative:** full managed tool/model payload capture by default, with explicit sensitive-field
exceptions. This improves debugging/transcript reconstruction but retains more private content and
requires reliable payload classification, size limits and scoped access from the outset.

**Ruling needed:** capture default. Retention-until-explicit-purge for captured evidence remains
accepted; this decides what is captured, not a new automatic-expiry rule. Interface observability
remains a first-class delivery requirement under either option.

## I7 — Authoring conventions and multi-plugin graphs

**Deferred to the end of review:** discuss syntax across all contracts and templates together
after the conceptual questions are settled. Publication with implementations is already accepted;
the field layouts, identifiers and schema-authoring choices below remain proposals.

**Recommendation:** retain the accepted envelope and propose `conduit/v0alpha1` as its initial
identifier. Apply group/name/version type references uniformly to the five families. Keep
`spec.implementation` for the type's owner and add a `spec.plugins` alias map for other exact
implementation dependencies; graph exports use `{plugin, name}`. Keep schema bundles reviewed and
language-neutral, with Rust schema generation used as assistance rather than silent authority.

**Example:** a Resource can use its owner's generic connection preparation, an authentication
export from another package and a database migration export, all explicitly bound. Those code
aliases do not create Resources, own credentials or bypass Conduits. Instance expansion resolves
the graph/controls and records code selection independently of later template changes.

**Alternative:** use one uniform `implementations` map, with an explicit alias for the type's owner.
That is more uniform but adds indirection to every simple template. A Rust-derived public schema
source of truth is also possible, provided release checks prevent accidental language changes.

**Ruling needed:** choose the authoring arrangement and schema source of truth as a bundle, then
review amended YAML examples. Parser scalar edge cases, numeric precision limits, schema-default
coverage and negotiated worker-feature details are implementation/conformance work; a failing
example should surface a concrete decision rather than another speculative questionnaire.

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
