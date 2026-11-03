# Construction, controls and lifecycle

Scope: accepted working contracts. Explicitly marked v0/reference-runtime selections retain
their accepted status without becoming requirements for every capability implementation.
See the [reference boundary](README.md#protocol-and-reference-implementation) and the
[current review queue](../REVIEW.md) for unresolved questions; historical review links below record prior rulings.

Part of the accepted [design contracts](README.md). Updated: 2026-11-03.
The [conceptual review](../archive/reviews/CONCEPTUAL.md) records completed rulings and deferred work;
syntax and implementation examples do not amend these rules.

## Establishment contracts and instance state

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

## Template-defined configuration changes

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
[the authoring pass](../REVIEW.md#later-authoring-and-conformance-pass). Confirmed settings and outstanding requests remain separate
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
Preserving code/contracts for running work does not promise later recreation of that implementation.
The [implementation-selection rules](CONFIGURATION.md#implementation-selection) distinguish retained
bindings and evidence from executable availability; missing code is reported without substitution.
R1 confirms that this applies to Interface administration as well as Agent requests. Broad
administrative authority does not introduce an in-place definition-upgrade mechanism. The
Interface can revise templates, create replacement instances and explicitly reconnect consumers.
A replacement may explicitly reuse existing backing data where the type supports attachment;
replacement does not itself authorize backing-data deletion. This keeps the interpretation of
running work, retained setup progress and control handlers fixed without requiring upgrade
migration rules in v0.

## Conduits and configurable endpoints

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

## Lifecycle protocol

**Graph boundary:** v0 setup and change-handler graphs are finite, acyclic dependency
graphs of typed plugin operations. Plugins declare typed inputs/outputs; graph nodes bind them.
Output bindings imply dependencies; explicit prerequisites cover ordering without data. Branching,
loops and provider-specific procedure remain inside plugins initially. User-authored conditions,
iteration and dynamic graph fan-out are deferred. Independent steps may run concurrently where
supported; interaction waits and valid-progress recovery do not require authored loops.

**Graph inputs:** v0 inputs use literal values and typed references to configuration or earlier
step outputs. Calculations and transformations are implemented by plugin operations, not inline
expressions in graph bindings. References select declared values/fields; they do not implicitly
evaluate string interpolation, shell syntax or an expression language. Reusable graph-backed
operations still receive their declared typed inputs under the ordinary operation contract.
Exact reference syntax remains authoring work.

**Reference recognition:** the graph recognizes references only in explicitly designated binding
positions. Literal data, ordinary configuration/provider payloads and values returned by plugins
are not recursively searched for graph instructions. Resolving a reference retrieves its value;
it does not reinterpret that value as another reference. For example, a storage plugin can receive
a document containing an example of graph-reference notation without causing the graph engine to
resolve the example. Actual references still supply their declared dependencies. Exact binding
notation and literal disambiguation remain open; this does not require one particular wrapper
around every scalar.

**Structured inputs:** graphs may explicitly assemble objects and lists from literals, references
and nested assemblies, without a plugin merely to build that structure. Each declared field or
list item supplies a value; output references contribute their normal execution dependencies. For example,
a document can combine the literal title `Review` with a body taken from an earlier step's output.
Literal subtrees and resolved output values remain data under the recognition rule above. Assembly
does not add automatic merging, arithmetic, string manipulation, loops or inline expressions;
calculations and other transformations still use plugin operations. Exact constructor syntax remains open.

**Input validation:** catch provable graph wiring/type errors before execution, including known
invalid references and literal inputs. V0 does not require a static proof that every possible
producer output satisfies the receiving input schema. Required schemas and their validation
features must still be available and supported; lack of such a proof does not waive validation.
Resolve and assemble each step's inputs, then validate the actual values against its input contract
before dispatch. Reject invalid inputs without invoking that step. Early checks do not replace
runtime validation, current authority or readiness checks. Exact checking algorithms and diagnostic
coverage remain authoring/conformance work.

For example, an output declared as an integer from 1–100 may feed an input accepting 1–10.
A resolved value of 5 can proceed; 50 is rejected before the receiving step runs, without coercion
or silently narrowing the value. This rejection does not undo earlier steps or establish that
they had no effects; their outcomes and recovery obligations remain recorded under existing rules.

**Output validation:** validate each declared plugin output against its producer's output contract
before making it available to downstream steps. Withhold invalid output and report a contract error,
preserving available effect evidence and uncertainty. A receiving input's acceptance cannot substitute
for this producer-side check; valid producer output still undergoes the receiving step's input check.

For example, a directory-creation operation can create the directory and then return a malformed
descriptor. Do not expose that descriptor as usable output, claim that nothing happened, infer
rollback or blindly repeat creation. Retain the outcome/effect evidence and use the operation's
recovery contract. A schema-valid declared failure result remains a valid result representation;
validation success does not itself establish successful effects, readiness or continued availability.
This concerns Conduit-declared outputs. Provider-native schemas retain their
[existing interpretation](CONFIGURATION.md#provider-mcp-schemas); no new provider schema is imposed.

For example, database startup reads its directory from configuration, and a migration step receives
the database handle published by startup. If the directory must be constructed from a base and a
relative path, a typed plugin operation performs that transformation and startup references its
output. That binding supplies the dependency. Such operations use the existing plugin contracts,
including validation, effects and recovery where applicable; they add no new object family or
implicit access. Plugins may implement internal calculations and branching as already agreed.

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
handlers and recovery compatibility remain in [current review](../REVIEW.md#refinements-to-carry-into-concrete-examples).


## Retirement and unregistration

Retirement stops admitting ordinary new work to the retiring instance and releases its own activity and
attachments through its supported lifecycle contract. Allow unregistration only after the contract's
required shutdown duties are confirmed. Missing or uncertain confirmation leaves retirement
incomplete and the instance registered for inspection, reconciliation or explicit retry.

The contract defines the duties appropriate to the type, including owned runtime/helper activity,
setup work and local connection state. This is a minimum observable guarantee, not a universal
lifecycle enum or a requirement to stop every backing service. Retiring a managed client does not
imply shutting down an external provider or a shared authentication Resource.

Preserve dependent instance identities, unrelated/shared capabilities and retained operation evidence.
Affected consumers observe unavailable access rather than being deleted or silently retargeted.
Already-dispatched external effects can outlive local shutdown; keep their known or uncertain outcomes
without claiming cancellation or rollback. Local shutdown confirmation does not establish the outcome
of a remote write. Further cancellation/reconciliation follows that operation's contract and current
authority; unregistration does not erase evidence or authorize replay.

For example, retiring Linear access releases its local bridge and attachments while preserving its
Agents and shared authentication Resource. An outstanding provider write remains attributable after
local shutdown. Closing an Interface frontend is only detachment; explicitly retiring the Interface
follows its own shutdown duties. A Conduit's logical flow may end before all cleanup is confirmed;
its unregistration still requires its declared shutdown duties to be fulfilled.

Existing Agent execution-end checks and Pool-owned Resource retirement checks remain in force.
Backing files, remote service data and retained evidence keep their separate explicit cleanup/purge
contracts. Incoming references alone do not cascade deletion. This rule does not introduce automatic
retirement, background garbage collection or permission to bypass an unresolved shutdown duty.
