# Conduit specification

Conduit provides shared access, context and communication for independent agents. Its core is
an extensible protocol with a Rust runtime: reusable templates establish instances; typed
capabilities connect through Conduits; an Interface makes the system controllable and observable.
Providers, frontends, storage and skill ecosystems remain replaceable.

Plugin and template authoring are the primary product surface. The protocol must let authors
extend capabilities and compose reusable instance recipes without changing Conduit's core.
Early implementations may reuse existing infrastructure and prioritize clear, correct behavior
over efficiency, provided they preserve and enforce these contracts. Runtime choices can change;
authoring and behavioral compatibility must be explicit.
V0 supplies only lightweight authoring support: real plugins and templates exercise the public
protocol and structure first. A broad SDK/helper library can grow from demonstrated needs later.
Plugins publish inspectable contracts and implementation bindings. A binding can use authored
code, an existing local/remote MCP service, or both; no new plugin-specific executable is required
when existing behavior fulfills the contract. Templates configure and compose these contributions.
Contracts can be validated without executing the implementation. Detailed syntax follows the
remaining conceptual rulings.
MCP is the accepted base for public plugin invocation, with a small versioned Conduit profile
for instance/operation context and lifecycle/recovery semantics. Exact bindings remain under review.
Components expose supported MCP capabilities independently where their declared dependencies
permit, while the Interface provides the aggregate control/observation view. This does not require
a separate process per object or route every call through the Interface. Managed access still
uses Conduits and current authority; standalone implementation use carries its own guarantees.

This is a design, not an implementation claim. [DESIGN.md](DESIGN.md) holds the detailed contracts.
The [walkthrough](../experiments/001-shared-access/README.md) proposes concrete YAML within them.

## Working model

Five families each have template and instance forms:

| Family | Responsibility |
| --- | --- |
| **Interface** | The user's unified control and observability endpoint. Multiple CLI, terminal and test frontends can attach to one instance. |
| **Agent** | An actor with private execution/context, a fixed account/runtime/provider boundary, activation rules and supported controls. |
| **Resource** | One capability responsibility with its related operations, owned by exactly one Pool. |
| **Pool** | Offers named Resource-template entries, binds dependencies and owns deployed Resources and shared service access. |
| **Conduit** | A relationship contract composed from typed requirements, instantiated as an authorized connection between compatible endpoints. |

Templates contain typed configuration, a finite setup graph, ongoing runtime declarations and
supported controls. One template establishes many independent instances; registration alone runs
nothing. An instance retains its resolved definition and current stored settings and need not have
a dedicated process. Plugins supply or bind typed operations and runtime behavior; graphs wire them
together. Capabilities describe compatibility. These are not additional object families.
Endpoint templates select their connection plugins, and endpoint instances manage their local
implementations and bindings. Conduits match compatible endpoint capabilities and coordinate
establishment, restrictions and participation; they do not install new endpoint plugin code.
Connection lifetime remains distinct from the lifetime of shared endpoint services.

Objects should support independent operation and eventual placement across systems. Resources
owned by one Pool may be deployed on different systems, and Conduits may span networks.
Logical ownership does not require co-location. The first implementation uses separate management
and execution services: executors supervise managed Agents/Resources, management coordinates
configuration/control and the Interface presents them. Each executor can host many instances;
external MCP services retain their own supervision. Management owns templates, registration and
shared access policy. One designated service owns each instance's resolved definition, confirmed
configuration/revision and operation results. The Interface aggregates those sources and identifies
stale observations; it is not another configuration authority. Resource backends own their domain
data. V0 has no offline grants: retain supervision and outcomes of already-dispatched work,
but hold new managed requests whose current permission cannot be established. Work with verifiable
permission can continue; existing stop/revocation rules and delivery checks still apply. Concrete
storage and production placement remain open. The first experiment uses management and two
independent executors as separate processes on one machine, with controlled connection failures.
Two-machine validation follows before remote support is claimed; no failure guarantee is yet tested.

After that synthetic proof, test real Agents through native Claude/Codex adapters first, then build
Conduit's own Rust model/tool loop. Both paths remain goals; native-account validation does not
prove the owned loop's behavior. Validate native Agent integration and subscription accounts on
macOS first, with a portable Rust core and OS-specific adapters. Other platform support requires
separate integration evidence.

## The user workflow

1. A frontend attaches to the default Interface. Bootstrap seeds a system Pool, its management
   Resource, the Interface and a management Conduit. Later startup reuses retained objects;
   missing or inconsistent components require explicit repair.
2. The Interface authors templates and composes a Pool's named available Resource entries.
   Listing an entry does not provision it or grant access.
3. Explicit setup provisions selected entries or uses selected existing instances. Every Resource
   has one Pool owner. Ready Resources remain usable while other requested components wait or fail.
4. Agents start with declared Conduits, reusing shared service authentication while retaining
   separate model subscriptions, private state and caller permissions.
5. Interfaces and delegated Agents inspect and invoke through Conduits. They can add/remove
   connections and request supported configuration changes.
6. The Interface follows those actions through durable local evidence, including work initiated
   by Agents and Resources, and presents relevant controls alongside observations.

V0 Interfaces have unrestricted local administration. Agents receive explicit grants; sharing
an OS account does not grant managed access. Unrestricted administration still follows supported
lifecycle/control contracts and cannot rewrite fixed definitions in place.

## Construction and runtime

Templates use YAML with an inline or referenced finite dependency graph. Nodes select typed plugin
operations and bind inputs. Output bindings imply dependencies; explicit ordering handles other
prerequisites. Plugins handle branching, polling and provider procedures. General authored loops,
branching and dynamic fan-out are deferred.

The template declares ongoing behavior; setup supplies prepared inputs. A graph can request that
its instance start a database, then run migrations. The instance owns that service from startup.
Readiness, setup completion and execution status remain distinct. Human waits, partial progress
and uncertain effects must be inspectable.

Failed setup retains successful work by default. Retry reuses progress only while results and
prerequisites remain valid. External effects are not assumed reversible or safe to repeat.
Supported bounded transport reconnect does not replay uncertain writes or start model work.

## Configuration and controls

The envelope is `apiVersion`, `kind`, top-level `form`, `metadata` and `spec`. Inspection adds
system-owned `status`; authoring rejects it. Stable IDs survive renames; names are unique per kind
across forms. Readable references include kind/form/name, while established bindings retain the
resolved identity until explicitly rebound.

The logical registry exposes active configuration through its authoritative owners; aggregate
observations are not independently writable copies. Writes explicitly create or update; editing a file does
not trigger reconciliation. Template updates replace complete authored configuration with separate
ID/version checks. Strict typed YAML excludes aliases, anchors, merge keys, custom tags, implicit
expansion and silent scalar conversion.

Instances materialize defaults and retain resolved setup/runtime/control definitions and selected
implementation bindings, including Conduit-controlled declarations, mappings and adapter/code artifacts.
Changes to those fixed definitions require replacement instances; external services retain their
separate compatibility/version boundaries. Later
template edits do not alter existing instances; mandatory template-history and override ledgers
are unnecessary.

Named typed controls expose allowed changes, reusing type defaults within template and access
limits. `spec.config` holds initial values, then confirmed applied settings. Pending or uncertain
requests remain separate; last confirmed does not prove current when an outcome is uncertain.
Changes serialize per target by default and normally check the inspected configuration revision.

MCP is the preferred external expression of controls/capabilities. Results remain extensible;
there is no universal domain-state enum or required MCP boundary between all internal components.
Compatible model changes within an Agent's fixed access boundary can use controls; changing
subscription accounts or provider domains requires a new Agent.

## Resource ownership and shared access

Resource types use separate group/name/version fields, typed `config` and declared `conduits`.
Working groups are filesystem, storage, search and integration; further groups/types are open.
Related operations share a Resource. Distinct responsibilities such as storage, search and MCP
access remain separate by default, even when backed by one provider.

Pools explicitly reference named Resource-template entries. Composition distinguishes provisioning
from an entry and using an existing instance. Required versioned capabilities bind to concrete
targets; membership is not permission. V0 has no generic object labels/selectors or Pool-provided
default Agent connections.

A reusable authentication Resource can serve integrations through Conduits. Its instance manages
credentials and refresh; ordinary callers receive permitted access, not a copied token. Provider
MCP tool names and schemas stay native where supported. Dynamic discovery grants no new authority.
Caller identity and shared provider identity are recorded separately.

## Connections, filtering and communication

Conduit templates define relationship contracts by composing reusable typed requirements.
A small base vocabulary is extensible with author-defined requirement types. Endpoint-selected
plugins satisfy the applicable meanings; Conduit instances bind concrete participants and retain
relationship state/evidence. Requirements are contract material, not another object family.
Concrete definitions, evaluation rules and the first catalog remain under review.

Conduits support explicit opening/joining, reusable composition and per-consumer participation.
Typed filters narrow operations, targets and parameters. Every restriction along the selected
path must pass. Separate authorized paths remain alternatives, without mixing their permissions.
Ambiguous routes need explicit selection; failures do not silently switch paths or accounts.

Preparation establishes required connections by default; optional failures report and continue.
Later dependency failures allow independent work. Current authority is checked on managed access
and delivery; no check erases already-delivered native context. Delegation follows the recipient's
logical participation, subject to withdrawal/current limits, rather than the issuing frontend.

The last logical consumer/peer leaving ends a Conduit. Later participation uses a new instance;
a socket outage while participants remain does not. Supporting durable data stays in Pool-owned
Resources.

The first communication capability uses shared append-only message history, explicit participants
and independent read positions, plus inspection. Reading does not imply understanding or task
acceptance. Next-turn unread notices do not wake idle Agents; content is retrieved explicitly.
Other communication modes and custom tracking remain Resource capabilities. Linear/Jira and
retained messages need no special tracker abstraction or mandatory issue linkage.

## Observability and evidence

Control and observability are inseparable in an Interface. Durable local evidence with pluggable
export persists even without a frontend attached. Correlate setup, controls, access decisions,
messages, tools, failures and available model/subagent usage through actors, instances, operations
and paths. Frontends inspect the same evidence.

Report uncertainty, stale views and integration gaps. Distinguish usage counts, estimates, unknown
data and observed zero; avoid overlapping parent/subagent totals. Export failure cannot erase
local committed outcomes. Credentials/payloads follow their own access and retention contracts.

## Scope and validation

Conduit owns its runtime and can offer a Pi-inspired terminal; it does not depend on Pi's runtime.
BYO skills preserve compatible native formats and tooling. Native execution is the default, with
optional or required isolation. Logical mediation does not prove complete native-host interception.

The [first experiment](../experiments/001-shared-access/README.md) moves from static YAML and synthetic
checks to two Agents, then separately verifies two Claude and two Codex accounts. Authentication,
account isolation, native compatibility and telemetry need actual evidence.

Detailed contracts retain portable context, bounded scoped memory search, revisioned records,
direct read-only sharing, retention until explicit purge and safe workspace cleanup. Pool deletion
requires explicit Resource retirement first; ordinary deletion/detachment preserves backing data.
Advanced sharing, remote administration, automatic wake, broad interception, authored graph loops
and in-place definition upgrades remain deferred.
