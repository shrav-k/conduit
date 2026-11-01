# Proposed implementation contracts

**Placement review:** separate management/execution services and shared policy with instance-owned
state are accepted. V0 has no offline grants. The first proof uses separate management and two
executor processes on one machine; remote support requires later two-machine validation.
References to a host below mean the responsible management
or execution service, not one combined process. Independent execution and network Conduits
require authenticated endpoint identity, retained target outcomes and authority-freshness rules.
These remain [review work](REVIEW.md#i1--independent-execution-and-management); serialization alone
would not make this a complete distributed protocol.

Status: draft contract design, 2026-11-01. All new field names, namespaces, limits and wire
profiles here are proposed. [DESIGN](../../DESIGN.md) supplies accepted semantics;
[review I3 and I7](REVIEW.md) cover the main extension and authoring choices. Examples are
explanatory, not files accepted by an existing Conduit executable.

## 1. Identity and version domains

Keep the following values distinct in Rust types and wire records:

| Value | Proposed representation | Meaning |
| --- | --- | --- |
| Registry/object IDs | UUID strings; generated independently of names | Identity, never authority. UUIDv7 is a candidate; ordering must not rely on it. |
| Object configuration revision | Opaque decimal string backed by a monotonic per-object counter | Checked edits/controls. Status updates do not advance it. |
| Content revision | Resource-defined opaque revision | Exact record version; not an object configuration revision. |
| Execution/start/attempt IDs | Independent opaque IDs linked to owner and operation | Distinguish retries, starts and actual worker execution. |
| API version | Proposed `conduit/v0alpha1` | Interpretation of the common object envelope. This is a protocol group, not a DNS or ownership claim. |
| Type/capability version | Exact group/name/version tuple | Configuration or capability semantics, separate from implementation release. |
| Implementation identity | Published contract/binding identity, retained adapter/artifact identities and external service binding where applicable | Conduit-controlled definitions/code are pinned; endpoint identity and observed provider versions do not prove exact hosted code. |
| Storage schema version | Host-internal migration number | Interpretation of persisted tables; never substituted for protocol versions. |
| Observation cursor | Opaque scoped cursor | Where a client resumes reading; not permission or proof it understood earlier content. |

Readable references retain `{kind, form, name}`. Established bindings store resolved IDs separately.
Instance names default to template name plus a generated suffix, with uniqueness checked and
collision retried. Registry-name aliases remain deferred. Names are unique within family across
both forms; a Pool's local entry name is not another global object name.

## 2. Object interpretation and YAML

The common envelope remains `apiVersion`, `kind`, `form`, `metadata`, `spec`. Normal input rejects
`status` and system-owned metadata. Inspection adds system identity, revisions and observations;
configuration export omits system fields and grants. Checked writes carry preconditions separately.

Proposed common spec responsibilities:

| Field | Responsibility |
| --- | --- |
| `type` | Exact group/name/version contract; uniform across the five families in this proposal. |
| `implementation` | The selected type's supplying implementation. |
| `plugins` | Optional named additional implementation dependencies for graph/handler/driver exports. The reserved `owner` alias identifies `implementation`. |
| `config` | The one stored configuration map: initial materialized values, then confirmed control changes. |
| `setup` | Inline finite graph or explicit graph-source reference in a template; resolved graph in the instance. |
| `runtime` | Instance-owned driver declaration and typed prepared-input bindings. |
| `controls` | Type-default selection/narrowing in a template; fixed resolved controls in an instance. |
| `conduits` | Consumer requirements and explicit open/join selections. |
| `owner` | Resource-instance owning Pool, established by Pool-authorized creation rather than self-asserted by YAML. |

The additional `plugins` map addresses a limitation of experiment 001's single fictional
`fixture.core`: a filesystem operation and an authentication operation may come from different
packages. Package aliases bind code, not Resources or permission. **I7** asks whether to adopt
this layout or a uniform implementation map. Do not create a package/module hierarchy in the
object registry merely to resolve exports.

### Parsing and validation pipeline

1. Apply input byte/document/depth limits and parse a marked YAML event stream.
2. Reject aliases, anchors, custom tags, duplicate keys, non-string keys and nonfinite numbers
   before a generic map conversion could erase their source evidence.
3. Interpret YAML 1.2 Core scalars. Preserve quoted strings; never convert a string to a number
   merely because a schema expects one. Empty collections, omitted fields and allowed nulls differ.
4. Validate common envelope and field ownership. Resolve only explicitly referenced local
   schema artifacts; no network fetching or environment/shell expansion during validation.
5. Validate supplied typed values, graph binding positions, control definitions and dependency
   declarations. Defer required-field checks for declared defaulted fields until materialization.
   A template can leave explicitly declared composition inputs unbound; an instance cannot
   execute with unresolved required bindings.
6. For instance creation, apply unambiguous declared defaults and validate the resulting complete
   spec. Templates preserve omissions. Record diagnostics with document, path, source location
   and contract identity.
7. Perform registry/ownership/reference checks. Store atomically, with no provisioning effects.

`yaml-rust2` exposes marked scalar/collection/alias events suitable for the first stage, but the
strict Conduit profile still needs a conformance corpus. Selecting that parser does not prove
our scalar, duplicate-key or denial-of-service behavior. [Parser events][yaml-events]

Use JSON Schema 2020-12 for portable data validation, plus explicitly defined Conduit semantics
for defaults, bindings and transitions. JSON Schema's `default` is an annotation and does not
materialize values. Implement that pass ourselves; reject contradictory or ambiguous default
definitions rather than selecting a branch by traversal order. Close records explicitly; schemas
with composition need correct `unevaluatedProperties` behavior. [Defaults][schema-defaults] ·
[Object properties][schema-objects]

Proposed interoperability profile: core counters/revisions use strings where large exact values
matter; plugin numeric fields must declare bounds. Reject values outside the chosen JSON wire
representation instead of rounding them silently. The parser spike should settle numeric
canonicalization and schema coverage before promising unrestricted arbitrary precision. Keep
field-specific list ordering; comments and mapping order are not configuration identity.

Multi-document input is a client convenience, not an implicit distributed transaction. Proposed
v0 behavior validates the batch first, then explicitly creates documents in order with individual
outcomes; partial registration is reported and no setup starts automatically. An update batch
needs a separate baseline per target. Rollback of already-registered objects is never inferred.

## 3. Templates, instances and bindings

Template use produces an ordinary instance draft. Callers edit fields, not a new parameter
language. Assigning a field replaces that value; editing a child leaves siblings intact. A
graph-source reference resolves to concrete graph content before execution and is retained in
the instance definition. No mandatory template snapshot or input/override history is introduced.

Capture implementation bindings from explicit selection or one unambiguous configured default.
Pin published declarations and Conduit-controlled adapter/code artifacts. Code-optional packages
can bind existing MCP implementations with no new executable; external targets follow existing
reference/configuration rules, with observed compatibility distinct from code pinning.
Schema-only registration may know the required binding without having its code or service available.
If selection itself remains unresolved, preserve the request as unresolved and block execution;
the first successful resolution is recorded once and is not recomputed from later defaults.
Instances normally export explicit resolved selections. An export with unresolved dependencies
must say so; it must not claim to be a runnable, fully resolved recipe.

Creation from an exported instance does not clone identity, setup receipts, runtime handles,
private history or permission grants. For example, an exported directory Resource retaining
`establish: create` may conflict with an existing directory. The caller deliberately changes the
new draft to supported attachment intent instead of silently adopting the old backing.

Pool entries offer templates. Provisioning creates a concrete Resource under that Pool; using
an existing Resource is a separate intent. Existing Resources do not acquire a second owner.
Cross-Pool capability use follows an authorized Conduit to the original owner. Composition binds
local named dependencies to actual returned IDs and retains partial creation outcomes so retry
does not infer identity from a name and create duplicates.

## 4. Graph language

An inline graph is a mapping of node names to operation bindings. Each operation identifies a
plugin alias and export. Inputs are literals or bindings recognized only in declared binding
positions. Ordinary provider JSON is not recursively interpreted as graph instructions.

Illustrative syntax, extending the existing fixture rather than approving it:

```yaml
setup:
  nodes:
    prepare:
      operation: {plugin: owner, name: consumer.prepare}
      inputs:
        consumer: {from: self}
    establish:
      operation: {plugin: database, name: database.establish}
      inputs:
        configuration: {from: config}
        access: {from: output, node: prepare, port: database-access}
    migrate:
      operation: {plugin: database, name: database.migrate}
      inputs:
        runtime: {from: output, node: establish, port: managed-database}
      after: [prepare]
```

This fragment assumes the enclosing definition supplies `owner`/`database` implementations,
typed configuration and the managed runtime declaration. `database.establish` requests startup
from the responsible execution service for that declared instance-owned runtime. Its process
may be independent of management; it is not an untracked daemon owned by a setup node. The output
binding already orders `migrate` after `establish`; the explicit prerequisite illustrates ordering
without a value edge. These export names are fictional.

### Compile-time rules

- Unique node names; finite acyclic dependency graph; every output/input path exists in a contract.
- Dependency edges come from output bindings plus `after`, not mapping order.
- The compiler can reject structurally incompatible types. It cannot prove arbitrary JSON Schema
  implication; runtime input validation is still required, and uncertain compatibility is reported.
- Binding kinds include self, config/path and prior node output. Config paths are typed paths,
  not shell expressions. An explicit literal wrapper is available where a literal object could
  otherwise be ambiguous with a binding marker.
- Runtime-input and control-handler bindings use the same typed machinery. Setup-only operations
  are not automatically exposed as public capability tools.
- Input acquisition never grants access. A Resource dependency requiring a Conduit must resolve
  through one, even if both instances happen to share a worker.

### Scheduling, outputs and lifetime

The scheduler dispatches dependency-ready nodes within global, per-instance and plugin limits.
Plugins handle internal branching/polling; the graph language has no authored loops, conditionals
or dynamic fan-out. A Pool composition operation can process a finite declared selection and
return separately tracked child outcomes without exposing an arbitrary loop language.

Each output declares type, publication condition and validity rules. Durable descriptors and
live handles differ. A ready output can refer to an instance-owned runtime that has started
before setup completes; the instance driver owns its continuing availability. A finishing setup
worker must not become the permanent source of liveness truth.

If readiness depends on a live input, check that input at dispatch/use under its contract. A
ready value becoming unavailable does not rewrite completed history. Dependent operations may
need reconciliation or retry; they cannot assume the last serialized value is still usable.

Save node inputs/fingerprints, selected implementation, attempt identity, outputs and validity
evidence. A retry can reuse a completed node only when its relevant inputs and prerequisites
still satisfy its contract. If a plugin cannot declare config read sets, conservatively treat
all config as relevant. Provider data can change without a config revision; validation may need
provider evidence too. Expired credentials require supported reacquisition, not replay of a
secret snapshot.

Graph dependency acyclicity does not prohibit cycles in the network of Conduits. Preparation
can nevertheless deadlock on cyclic readiness requirements; report the wait chain and require
a supported deferred dependency or changed composition. Do not “solve” it by dropping a required
connection. Bound nested composition depth and total prepared work.

## 5. Plugin package and runtime contracts

**Authoring before hosting:** the manifest/port inventory below is a proposed runtime-facing
contract, not a requirement that every author manually implement every field or interface.
The [accepted direction](REVIEW.md#review-priority-plugin-and-template-authors) is lightweight
Conduit support, with real plugins proving the public protocol before investing in a helper
library. The division between authored and generated metadata and the concrete package/wire
formats remain open. Persistent drivers need lifecycle behavior; a finite operation need not
implement unused hooks. Small adapters or existing libraries must preserve the same observable
contracts; a comprehensive SDK is not required before the first plugins can be authored.

**Accepted publication boundary:** plugins publish inspectable contracts and implementation bindings;
Conduit can validate against them without executing plugin code. Bindings can use authored code,
an existing local/remote MCP service or both. Registering schemas is not
permission to run the package. The proposed manifest/schema bundle below illustrates that
boundary; detailed syntax and package layout are deferred until conceptual review is complete.
Local packages, distribution machinery and authored/generated metadata remain implementation
proposals rather than consequences of accepting publication.

| Manifest concern | Required information when applicable |
| --- | --- |
| Package/bindings | Package/contract identity, implementation or service dependencies and compatible profile; executable entry, platform and code digests where code is supplied. Code-free declarations/adapters still have retained identities. |
| Contracts | Provided type/capability/filter versions and exact schema bundle identity. Conflicting definitions of the same contract cannot overwrite one another. |
| Operations | Export name, input/output schemas, effect/recovery information, cancellation/interaction support and output validity. |
| Drivers | Managed runtime exports, prepared inputs, inspect/start/stop/reconnect support and enforcement coverage. |
| Controls | Input schema, supported handler/graph, affected config paths, constraints, coordination/revision requirements and confirmation semantics. |
| Host needs | Explicit storage, credential, process, network and other broker facilities used by the implementation. Declaring a need does not grant it. |
| Compatibility | Provider/native-runtime requirements and known version boundaries, separately from package identity. |

Some recovery behavior is declarative, some is implemented by a handler. An operation may be
non-repeatable, provider-idempotent or inspectable without being generally replay-safe. Do not
infer semantics from a tool name, HTTP method or an advisory MCP annotation alone.

### Host-facing Rust interfaces

Define a few capability-oriented ports rather than a giant required plugin trait:

- A contract catalog supplies schema/descriptor data.
- An operation executor accepts a typed dispatch context and returns progress/interactions/results.
- A managed driver supplies lifecycle/observation for declared ongoing behavior.
- A capability handler supplies native discovery/invocation where supported.
- A recovery handler establishes what happened to a prior attempt.

Built-in, process-backed and existing-MCP-backed implementations satisfy applicable tests for
the contracts they claim; externally observable provider limits remain explicit.
Rust trait types remain private implementation APIs, not a stable cross-binary Rust ABI. Avoid
requiring a tiny script plugin to implement unused lifecycle interfaces.

### Worker boundary

Physical placement remains pending. The accepted
[instance and endpoint behavior boundary](AUTHORING.md#5-where-the-example-implementations-execute)
uses endpoint-selected plugins managed by their instances. Conduits match compatible capabilities
and coordinate participation; they do not supply new endpoint implementation code. Exact endpoint
connection contracts and establishment/cleanup procedures remain under review. The caller, owning activity,
target and execution location must remain attributable. The worker mechanisms below are candidates
for hosting that behavior, not a requirement that each plugin or Conduit have a separate process.

MCP is accepted as the public plugin invocation base; `rmcp` remains the proposed Rust library
for discovery, calls, resources and supported optional exchanges. A small Conduit profile describes
instance/operation context and lifecycle/recovery semantics that vanilla provider MCP servers do
not promise. Its exact binding of the above ports remains under review; it adds no object family.
The SDK provides transport machinery; it does not supply our durable operation identity.
[Rust MCP SDK][rmcp]

**Accepted:** published plugin contracts can bind an existing MCP service
through a standard adapter, without a new plugin-specific executable. The adapter still supplies
Conduit's context, current access checks, observation and supported lifecycle/recovery boundary.
Provider-native operations/schemas remain discoverable. Unsupported semantics require additional
implementation or an explicitly narrower contract, not a declaration claiming guarantees the
provider cannot establish. Separate contract/adapter pinning from observed hosted-service versions.
Supported component surfaces can be exposed independently over MCP while retaining their required
management/authority dependencies and Interface-visible evidence.

Separate two surfaces: internal setup/driver exports reachable by the host, and capability/control
exports visible to authorized consumers. An upstream MCP tool never becomes an instance setup
operation simply because it appears in `tools/list`. Workers should expose contract descriptors
through a documented resource/manifest surface; ordinary operations can use tools. Avoid custom
JSON-RPC methods until a concrete missing standard facility justifies them.

For a Conduit-aware worker, the proposed handshake verifies package/artifact identity, profile
version and descriptors against the retained binding before admitting work. An ordinary upstream
MCP server is negotiated through its adapter; it is not required to attest to a Conduit artifact
or implement the management profile. Record the compatibility evidence actually available.
The responsible service uses a bounded authenticated channel and
assigns owner/actor context; the worker cannot self-assign administrator authority. Worker results
are validated before they alter stored state.

For code that needs an owned worker, one candidate v0 deployment uses one per active driver owner
or explicit isolation boundary. This remains an I3 proposal, not a worker requirement for
declarative packages or attached external services. It allows many Agents to
share one Pool-owned service Resource and its worker. Cross-instance worker multiplexing can
follow measurements; it must partition state, credentials and cancellation correctly. Do not
allocate workers for registered templates, idle declarations or objects whose behavior needs no
external executable. Trusted dispatch/ownership rules remain in responsible Conduit services;
they are not restricted to one global process. Worker loss, executor loss and management-channel
loss have separate meanings. The designated instance service owns confirmed configuration and
operation records; executors supervise managed activity, and management owns shared registration
and policy. Retain supervision/dispatched outcomes and block new managed requests without
verifiable permission; no offline grants apply. Crash-survival guarantees still need tests before
this lifecycle profile is complete. A current inventory view is not guaranteed by serialization.

For first-party releases, retain the exact worker executable/bundle used by each instance.
A script bundle additionally needs a defined interpreter/dependency closure; hashing just its
entry script does not pin imported code. Initial implementation should avoid promising arbitrary
language packaging until its runner contract supplies that closure. Ordinary third-party MCP
services can still be connected as services with honest external-version coverage.

## 6. Invocation and evidence records

Proposed internal invocation information includes:

| Concern | Example information | Authority |
| --- | --- | --- |
| Logical identity | Operation ID, caller idempotency key and canonical request fingerprint | Host validates uniqueness within caller/registry scope. |
| Attribution | Actor, Interface frontend or Agent execution, target instance and selected Conduit path | Established by authenticated host dispatch, not caller JSON claims. |
| Preconditions | Expected target ID/revision, generation and contract identity | Checked at application; cached discovery is insufficient. |
| Effect inputs | Validated arguments or retained references, selected implementation and service identity | Recovery must know what was actually dispatched. |
| Progress | Attempt, wait/interaction references, timestamps and typed detail | Observations distinguish request acceptance from application. |
| Outcome | Confirmed changes/results, failure evidence, uncertainty and supported next actions | Plugin establishes domain evidence; host validates/commits it. |

This is runtime bookkeeping produced by calls, not a second YAML document users must assemble.
Open-ended structured plugin results remain valid. Core fields express correlation and known
facts; they do not force every domain into a universal list of business states.

### Idempotency and authorization order

After authenticating the caller, look up a supplied logical idempotency key. If the same key was
accepted for a different target/path/arguments/preconditions, return a conflict. If a retained
matching outcome exists, deliver it only under current access without reapplying the effect or
requiring its old revision to equal today's revision. A pending operation returns its existing
identity, not a second dispatch.

For a genuinely new operation or a supported retry attempt, acquire the appropriate coordination
domain, check current revision/authority and record dispatch intent. Same key does not make an
unsafe external write idempotent; provider-side keys or inspection may still be needed. A purged
result leaves enough non-content outcome/tombstone evidence to avoid interpreting “missing” as
“never ran,” subject to the accepted purge contract.

Distinguish unsupported protocol, invalid arguments, denied access, stale revision, unavailable
dependency and uncertain effect. Stable error categories help clients; plugin-specific detail
remains namespaced and schema-described. None of these errors authorizes an automatic retry
with relaxed permissions or a substituted account.

### MCP mapping

| Interaction | Preferred representation and fallback |
| --- | --- |
| Discover/invoke a supported control | A named typed MCP tool with structured result. Provider tools retain native names/schemas where compatible. |
| Inspect operation/instance | MCP resources or typed inspection tools with current, authorized content and revision/cursor. |
| Long operation | Negotiated standard task/progress facilities where supported; otherwise an explicit retained operation reference plus inspect operation. Acceptance is not represented as completed success. |
| Additional human input | Supported elicitation/input mechanisms; otherwise a retained interaction surfaced by the Interface. No waiting forever on a disconnected frontend. |
| Observe changes | Negotiated subscriptions/notifications plus durable catch-up; otherwise bounded polling from a cursor. |
| Cancel/reconcile | Authorized supported follow-up operations. Transport cancellation alone is not proof the effect stopped. |

An operation URI is a reference, not a bearer grant. Standard MCP request IDs, task IDs and
subscription IDs can map to Conduit records without becoming their sole durable identity.
Do not use trace baggage to carry authorization. Feature fallback is negotiated and visible;
critical missing semantics block the operation instead of silently degrading its guarantees.

## 7. Controls and configuration results

A resolved control definition selects a handler or finite graph, typed inputs, allowed config
paths/constraints, required revision checks and coordination domain. Definition changes require a
replacement instance. Supported values can change through the control; no generic patch endpoint
can rewrite a running instance's fixed definition.

The handler returns a proposed confirmed result with evidence, not unrestricted permission to
write arbitrary JSON. The host validates the complete resulting config against the fixed schema
and control constraints. If application cannot be confirmed, leave stored values unchanged and
record uncertainty. The Interface must not label those last-confirmed values as known current.

A waiting change retains its coordination semantics. If safe parallel control domains are added,
they need declared disjoint read/write dependencies and transaction checks; merely using different
field names does not prove provider-level independence. Start with serial controls and expected
whole-configuration revisions.

Reject a predictable invalid change before effects. If a buggy provider/handler returns an invalid
confirmation after an effect may have occurred, preserve the evidence and report unresolved or
out-of-contract state; do not turn result-validation failure into a claim that nothing changed.
Reconciliation must establish reality before another conflicting change can proceed.

## 8. Content, handles and secret custody

Ordinary typed values may be serialized. Managed handles identify their target, responsible
execution endpoint, generation and allowed operations; handle access is checked when used.
Endpoint identity is distinct from a changeable network address. These are proposed handle
semantics, not new YAML keys or a sixth object family. Their serialized inspection form contains no live credential or assumption that the old process still exists.

Secret inputs are explicit references or interaction channels whose custody remains with their
owning authentication capability. The type/operation schema identifies sensitive fields and
recording needs. The host must not blindly serialize an input structure using debug formatting.
If a required recovery input cannot be retained safely, record the recovery limitation and ask
for supported re-establishment rather than claiming transparent replay.

Content records retain their Resource ownership, revisions, derivation/access dependencies and
permitted provenance. Event views can refer to them without copying full private content into
every observation stream. **I6** decides what full payloads are retained by default. Exported
configuration never carries secrets, grants, live handles or readiness as if they were settings.

## 9. Agent and provider adapters

Separate the model-provider protocol from the native-agent protocol. Both implement parts of
an Agent's selected runtime, but they expose different degrees of control and observation.

| Port | Inputs and outputs | Required boundary |
| --- | --- | --- |
| Account verification | Explicit account/runtime selection → supported identity evidence or unresolved state | No ambient account substitution or silent API-billing fallback. |
| Model discovery | Fixed access boundary → currently supported model descriptors | Discovery does not change selection or certify account eligibility. |
| Context conversion | Permitted context items and model/tool catalog → provider request plus conversion/coverage evidence | Do not merge private histories or silently omit required content. |
| Model generation | Prepared request, cancellation and correlation → typed stream and terminal outcome | Host owns admission/recovery policy; interrupted generation is not automatically repeated. |
| Native execution | Private-state/config selection and prepared access → verified execution identity, attach/control/inspect support | Native runtime owns its loop and authentication; report missing capabilities rather than synthesizing them. |
| Usage capture | Available provider/native observations → attributed counts/estimates/unknown coverage | Preserve cache/subagent distinctions and avoid summing overlapping figures. |

Provider descriptors separate API/protocol compatibility, model capabilities and authentication
support. Do not key reusable authentication solely by provider name. Use the explicit account,
client/audience and refresh owner where applicable. A supported model switch changes only the
allowed setting inside the Agent's fixed boundary; it cannot redirect to another subscription.

For the owned loop, persist accepted input and the intended generation before sending it. Build
context from explicit selection, then record the projection/conversion evidence permitted by its
capture policy. Stream partial content to the Interface with a generation ID. Only a complete,
validated tool invocation can enter the effect pipeline; partial streamed arguments are not
executable commands. Preserve provider call IDs within their generation rather than assuming
global uniqueness. Record each tool result before using it in the next model exchange.

A provider may expose incremental content, usage, tool requests and provider-specific blocks.
Use a small common event structure with typed adapter extensions; preserve available original
references where conversion loses detail. Unsupported blocks remain visible as gaps, not invented
portable content. A completed transcript is not a guarantee that the provider can resume its native
execution state. Ongoing context compaction and checkpoint generation follow the separate accepted
contracts and must not become unrequested background model calls.

The first provider contract needs deterministic fake-stream tests for partial output, tool-call
assembly, interrupted transport, cancellation and missing usage. Native adapters additionally
need actual multi-account tests: refresh/logout one account, then verify the other; use the same
account in two Agents and verify private histories stay separate. A directory layout or source
inspection alone cannot satisfy those tests.

## 10. Interface operations and frontend behavior

Use one application service API behind MCP tools and CLI/TUI actions. The CLI must not edit SQLite
directly or implement a second permission/effect pipeline. This is one semantic contract, not a
requirement for one process or gateway. Independently exposed component capabilities use the
same applicable access, operation and evidence rules. Proposed operation groups are:

| Group | Actions | Observable result |
| --- | --- | --- |
| Configuration | Validate, create template, prepare edit, checked replacement, export | Source diagnostics or retained identity/revision; registration performs no setup. |
| Instance | Create from template/direct draft, start, retry setup, inspect, pause/stop/close as supported | Creation and execution outcomes separately identified. |
| Pool | List available entries, provision selected entry, use existing target, retire selected Resource | Concrete owned identities and partial outcomes; no deployment from availability alone. |
| Conduit | Open/join, inspect participation/path, detach, change supported restrictions | Current authority and connection/readiness coverage; no new flow per ordinary invocation. |
| Control | Discover, invoke, inspect/reconcile/cancel where supported | Typed result, confirmed revision or explicit pending/uncertain evidence. |
| Observation | List/snapshot, follow from cursor, retrieve permitted content, inspect usage | Bounded coverage with continuation/gap information. |
| Maintenance | Repair, backup, explicit cleanup/purge | Checked outcomes and preserved unrelated backing. |

Command spelling can follow the schema review. Every effecting command returns a stable operation
reference, and long work can be followed or revisited. A convenience `create-and-start` action
is explicit and shows both outcomes; it is not an implicit effect of reading YAML. Interactive
input is a supported operation, not editing an instance's fixed definition.

The minimal terminal has an instance list, selected-instance state, active/waiting operations,
event/history view and command/input area. It distinguishes last-confirmed values from unknown
current state, and presents plugin-described interactions without executing arbitrary plugin UI
code in the host. Multiple attachments of the same controlling Interface can issue commands;
target coordination resolves conflicts. Cancelling an observer only detaches that observation.
Stopping an Agent or an operation requires the corresponding supported command.

## 11. Conformance deliverables

Each released contract needs valid/invalid schema fixtures, default/equality cases, capability
behavior tests, recovery fixtures and compatibility expectations. The first implementation should
validate both the existing walkthrough and amended examples for multi-plugin graphs. Never silently
rewrite experiment 001 to make a new parser pass; document any proposed language difference first.

Before declaring the worker profile usable, run the same invocation/recovery tests against the
in-process fake and real subprocess transport, then across independently supervised executors.
Also exercise a code-optional binding to an existing fake MCP service and a standard MCP client
using a managed component without the aggregate frontend. Test only supported contract guarantees;
the client must not gain management authority from being protocol-compatible.
Before accepting a provider adapter, test actual identity, parallel-account behavior, refresh ownership, permitted native operations, interruption
and telemetry gaps. A manifest plus passing JSON Schema is not evidence that those effects work.

[yaml-events]: https://docs.rs/yaml-rust2/latest/yaml_rust2/parser/enum.Event.html
[schema-defaults]: https://json-schema.org/understanding-json-schema/reference/annotations
[schema-objects]: https://json-schema.org/understanding-json-schema/reference/object
[rmcp]: https://docs.rs/rmcp/latest/rmcp/
