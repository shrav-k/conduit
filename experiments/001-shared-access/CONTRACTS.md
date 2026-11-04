# Fixture contract guide

This guide defines the interpretation of this experiment's proposed YAML. It is a paper contract,
not a registered schema, implementation or selection of a public API namespace. It implements no
provider calls. The accepted semantics are in [DESIGN](../../docs/protocol/README.md); the
[walkthrough](README.md) shows them in use.

## Draft syntax

`apiVersion: conduit.local/v0alpha1` is a placeholder. Resource group/name/version separation and
the common kind/form/metadata/spec envelope are accepted; the specific identifiers here are not.
Selecting one versioned type contract for every template/instance across all five families is
accepted. Types specialize configuration and behavior while preserving family invariants. The
grouping/catalog for non-Resource families and the common field spelling remain provisional.
`metadata.name` is the only authored metadata in these files. No stored IDs, revisions, status,
credentials, grants or live handles appear in the authoring documents.

| Field | Fixture interpretation |
| --- | --- |
| `spec.type` | One versioned type contract in every family. Its version is independent of the common object contract and implementation release. Resource grouping is independent of package installation. |
| `spec.plugins` | Proposed named implementation map with no primary entry. Each export reference uses `{plugin, name}`. `fixture-001` names imaginary code, not a verified hash or available package. |
| `spec.owner` | Resource-instance owning Pool. Registration through that Pool validates and records this relationship; merely writing it cannot claim ownership. |
| `spec.config` | Typed establishment and behavior settings; defaults are materialized for instances. Current confirmed settings replace initial values after successful controls. |
| `spec.setup.nodes` | Finite graph of operation bindings. Mapping order has no execution meaning. |
| `spec.runtime` | Map of named instance-owned runtime declarations selecting drivers. Setup passes prepared inputs to an ordinary startup operation. A managed object need not spawn a process. |
| `spec.controls` | Type-default selection/narrowing in templates; resolved named schemas/handler bindings in instances. |
| `spec.conduits` | Named consumer connection requirements, capability contracts, open/join choice and targets. Enclosing instance supplies the consumer. |

This is neither a package manifest nor a second operation-request language. Creating an instance
copies/resolves its template, makes ordinary field edits, then validates the complete result.
The fixture models an authored implementation; published declarations backed by existing MCP
services are also accepted by the design. Their exact binding fields await the final syntax review.
A template may leave a connection target for composition to bind; the instance cannot leave a
required target unspecified. No named template parameters, string interpolation or implicit merges
are introduced. Capability declarations are requirements, never grants.

Every schema-provided default used here is visible in the instance examples. Templates explicitly
write `required`, empty connections, filter lists and driver inputs where applicable. Controls
are the only shorthand deliberately resolved in the shown instances. Templates may preserve
omission where their type contract permits it.

## Graph binding notation

Every input binding has exactly one form key: `literal`, `reference`, `object` or `list`. The last
two explicitly assemble fields/items from further bindings. These files use references, object
assembly for prepared startup inputs, and literal composition data:

| Binding | Meaning |
| --- | --- |
| `{literal: {provision: {entry: auth}}}` | Supply this object as ordinary data; do not interpret its contents as graph instructions. |
| `{reference: {from: metadata, path: /instanceId}}` | Select the owning instance ID from the explicitly assumed typed metadata surface. For a Conduit graph this identifies the Conduit, not its consuming Agent. |
| `{reference: {from: config}}` | Whole instance configuration captured for this graph execution, with the operation's expected shape. |
| `{reference: {from: config, path: /entries}}` | Select `entries` from that captured configuration using JSON Pointer. |
| `{reference: {from: output, node: login, port: identity}}` | A declared output of an earlier node; implies a dependency. |
| `{reference: {from: runtime, name: service}}` | A typed target selecting the instance's named runtime declaration; neither a live connection nor permission to start it. |
| `{object: {store: {reference: {from: output, node: history, port: store}}}}` | Assemble a startup input object from the established history backing reference. |

Omitting `path`, or supplying the empty JSON Pointer, selects the whole chosen value. A missing
selection blocks the consumer with an error; it does not become an omitted/defaulted input.
These markers are recognized only in schema-declared binding positions. Literal payloads, ordinary
configuration and provider content are not recursively interpreted as graph code. Producer outputs
must validate before publication; resolved operation inputs receive applicable defaults and must
validate before invocation. Existing effects and independent branches retain their agreed behavior.

For this fixture only, the metadata contract is assumed to expose a required, read-only `instanceId`
string. The `instanceId`, `consumerId` and `ownerId` operation inputs accept that identifier as data;
the responsible service still resolves the target and verifies current authority. No unrestricted
`self` source, ambient configuration or credential access is implied. Actual metadata declaration
syntax and ID encoding remain open. Reusable operations receive configuration through declared
inputs rather than inheriting the caller's configuration source.

Each example names its sole runtime `service`; this is an arbitrary local name, not a required or
primary runtime. Additional names are allowed where the selected type supports them. Runtime
declarations alone start nothing. The ordinary `runtime.start` operation receives that typed target,
captured configuration and explicitly assembled prepared inputs. Its selected driver validates
the prepared inputs against its contract, and the responsible service manages startup and ongoing
activity. The runtime declaration itself contains no hidden references to setup-step outputs.
This follows the same explicit startup model as the
[database walkthrough](../../docs/proposals/authoring-syntax/CONTRACTS.md#4-graph-language).
`runtime.start` is a fictional reusable operation for this example, not a required core export or
special graph instruction. Its production wire/driver schema still needs a concrete definition.
The Pool runtime has no prepared child-output dependency and may start while selected Resources
are still preparing; required provisioning failures still leave overall setup incomplete.
Explicit success prerequisites belong on the dependent step;
this packet does not need or finalize that field's spelling. Steps here explicitly declare the
accepted required-by-default behavior.

`pool.compose` is one reusable plugin operation. Its finite `items` input distinguishes
`provision: {entry, name}` from `existing: {kind, form, name}`. A local `bind` entry connects a
consumer requirement to another selected item's output. The operation handles this finite
composition under its contract; it does not add authored graph loops or dynamic fan-out.
Dependencies, not list/map order, decide which Resources can prepare. The Pool supplies owner
identity, performs authorized creation and records ordinary progress, not a second editable plan.

Pool provisioning omits child names so repeated creation can generate distinct template-name
suffixes. The walkthrough shows example resolved names; dependent drafts must use actual returned
references. Callers can supply a unique name explicitly, as in the alternate existing-auth example.
Names remain distinct from stable internal IDs.

## Type and capability boundaries

| Template/type | Capabilities and responsibility |
| --- | --- |
| `project-directory` / filesystem.local-directory | Directory access. Attach verifies an existing path; create requires absence. Agent symlinks are preparation state, not new Resources. |
| `service-authentication` / integration.service-authentication | Managed service identity and supported bindings, including coordinated refresh. No token in ordinary capability results. Its final type group remains a naming proposal. |
| `issue-service` / integration.mcp | Authenticated MCP discovery/invocation for the configured workspace. Native tools/schemas remain provider-owned. |
| `conversation-history` / storage.message-history | Append-only messages, explicit participation, bounded reads, individual reader progress and inspection. A domain-specific history Resource may own its backing store; it is not a generic search/MCP/database bundle. |
| `project-pool` / composition.resource-pool | Named template availability, explicit selected composition and owned Resource lifecycle. |
| `collaborator` / agent.fixture | A synthetic actor with private state and explicit account identity. Does not validate Claude/Codex compatibility. |
| `owner-console` / interface.local-owner | Unrestricted local-owner administration with shared frontend control and observation. |
| `capability-route` / connection.capability-route | Explicit consumer→target connection, capability matching and typed filtering. No backing-data ownership. |

The `communication.retained-messages` capability is not a new registry family or approval of a
closed Resource-group catalog. Each capability reference here uses exact version matching for the
fixture. General version negotiation/adaptation is not chosen by this example.

## Plugin operations and runtimes

All operations below are supplied by the fictional `fixture.core` implementation through the
explicit alias `fixture`. Runtime drivers and resolved control handlers use the same proposed
`{plugin, name}` reference form; contract validation distinguishes the export kinds. Their outputs
are runtime records with declared validity, not YAML values copied into an instance spec.

| Operation | Inputs → outputs | Effects and recovery |
| --- | --- | --- |
| `directory.establish` | config → directory | Attach/create under explicit intent; output identifies verified backing. Unknown creation outcome requires reconciliation. Missing backing invalidates reuse. |
| `auth.establish` | config → binding | May wait for interactive consent; output identifies managed service access without disclosing credentials. Reuse only while identity and authority remain valid. |
| `consumer.prepare` | consumerId → connections plus declared named connection ports | Resolve the consumer and open/join declared Conduits under current authority. Required connections block preparation; optional failure is reported. Retain successful attachments for retry. For `issue-service`, the named `authentication` port supplies the managed binding. |
| `mcp.prepare` | config, authentication → client | Prepare the client against the selected endpoint/identity. The Resource runtime owns ongoing transport and catalog state; no Agent login is repeated. |
| `history.establish` | config → store | Create the selected backing or verify recorded progress. Attachment is an explicit alternate intent; name matching is not permission to adopt backing. |
| `pool.compose` | ownerId, entries, selected items → resources | Resolve the owning Pool and establish only selected items under current authority. Bind local dependencies, retain valid partial work, expose ready Resources independently. |
| `agent.authenticate` | account → identity | Establish/verify a private model account context. Sharing a service authentication Resource does not merge Agent logins. |
| `agent.activate` | identity, access, rules → environment | Prepare supported native configuration/tool exposure; required failures block, optional omissions remain visible. |
| `route.establish` | instanceId, config → binding | Resolve compatible endpoints and current delegated authority. The input ID identifies the flow owner; the actual consumer is in config. A route is usable only after matching/enforcement checks. |
| `runtime.start` | target, configuration, prepared → runtime | Request startup through the responsible service using the selected driver's contract. Validate target, prepared inputs and current authority before dispatch. The fixture promises confirmed driver readiness on success; an uncertain start is reconciled without duplicate startup. The instance retains ongoing supervision and output availability. |

Instance drivers maintain ongoing state and output availability. For example, `auth.managed`
coordinates refresh and `mcp.managed` handles bounded reconnect. Setup plugins do not become
independent daemons. Dropped transports do not imply logical detachment. Driver-specific failure
and status fields remain open, without a universal domain-state enumeration.

## Controls and filters

The fixture history type has one default configuration control, `set-read-limit`. It accepts an
integer 1–100, applies the bounded-read setting and confirms the resulting configuration. It uses
an expected revision and serial application. The resolved definition is shown in `instances.yaml`.
The remaining fixture types have no type-default configuration controls in this minimal packet;
ordinary creation, inspect, join/detach and lifecycle operations remain their supported operations.
Future real types can expose additional controls without changing the five families.

`access.operations` filters narrow operation names. `integration.fixture-issue-access` constrains
workspace and action semantics for the synthetic MCP catalog. A real provider adapter must map its
native operations/arguments/results correctly and reject unsupported enforcement; these declarations
are not proof of provider enforcement. Local symlink exposure also does not enforce OS read-only
access in native mode. The Interface must show that coverage boundary.

The owner delegates permitted capability use when creating Agent attachments. Neither a template
nor a filter creates permission. Resource policies, current caller authority and every filter on
the selected route still apply. Provider identity and caller identity remain separate evidence.

## Bootstrap boundary

Before this packet's application setup, the locally loadable root Interface directs establishment
of the system Pool, management and access-management Resources, and its ordinary management
connections. Its declared recovery operations use runtime verification of the local owner when
the ordinary path is broken. Exact built-in recipes and persistence loading are outside this
example. The included Interface template illustrates the normal management path to
`system-management-0001`; it is not the complete root establishment/recovery definition and does
not eliminate the dedicated access-management Resource. Existing identities and uncertain outcomes
must survive explicit recovery; normal startup does not silently recreate missing system objects.

The three categories of instance references outside `templates.yaml` come from this documented setup:
application Resource instances from Pool setup, Agent instances from creation, and the seeded
management Resource. Static validation can check that declared scenario mapping; it cannot prove
that any of those instances actually exists or is ready.
