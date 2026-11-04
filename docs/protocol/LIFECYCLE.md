# Construction, controls and lifecycle

Scope: accepted working contracts. Explicitly marked v0/reference-runtime selections retain
their accepted status without becoming requirements for every capability implementation.
See the [reference boundary](README.md#protocol-and-reference-implementation) and the
[current review queue](../REVIEW.md) for unresolved questions; historical review links below record prior rulings.

Part of the accepted [design contracts](README.md). Updated: 2026-11-04.
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

**Partial confirmation:** a control may explicitly support confirming only part of a requested
configuration change. Store the confirmed permitted changes when the complete resulting stored
configuration satisfies the schema and control constraints. Retain failure, pending work and
uncertainty for the remaining effects separately; confirming a subset does not establish success
for the entire request. A setting whose application is uncertain keeps its last confirmed stored
value without being presented as known current.

For example, a search control can confirm a query-limit change from 10 to 15 while a timeout
change from 30 to 60 seconds remains uncertain. If its contract supports that partial result,
store the limit of 15 and retain 30 as the last confirmed timeout, with the uncertainty visible.
Controls with coupled settings can require confirmation of the whole change. The shared rule
does not infer independence between arbitrary fields, allow invalid configuration or imply
rollback. Current authority, revision checks and coordination still apply. If confirmed external
effects cannot produce a valid stored configuration, preserve their evidence and report the
unresolved or out-of-contract state rather than claiming that nothing changed.

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

**Unchanged configuration:** when a control confirms unchanged stored settings, retain the current
configuration revision and record the invocation, outcome and any effects separately. All applicable
authority, revision, transition and coordination checks still apply, as does the handler's declared
behavior. Equal requested/stored values do not justify skipping the handler or presenting an earlier
result as a new execution. For example, setting an already-selected limit of 15 may verify or refresh
the provider without changing the configuration revision. That revision describes configuration;
it does not establish that no work occurred or that runtime state, permissions or observations are
unchanged. Existing operation-retry/result-retrieval contracts remain separate.

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

**Setup success requirements:** setup steps are required by default; a template may explicitly
mark nonessential steps optional. A required migration must succeed for successful setup, while
failure of optional diagnostics is reported without itself preventing setup success. Optionality
does not erase failures, uncertainty or effects, and does not change independent scheduling or
required lifecycle cleanup.

Actual dependencies still apply. If a required step needs an optional step's output or successful
completion, that prerequisite must be satisfied. Optionality cannot manufacture a missing value,
waive a mandatory type/access requirement or bypass definition/input/output validation. Coupled
requirements remain governed by their contracts. This rule concerns setup success, separately
from the already accepted required/optional connection rule; exact step notation remains open.

**Ordering prerequisites:** in v0, an explicit prerequisite that passes no data requires the
predecessor to complete successfully under its operation contract. A failed or uncertain outcome
does not satisfy it; merely finishing, returning a response or producing a schema-valid failure
payload is insufficient. For example, schema verification can use the database output from startup
while separately requiring migration success. Current authority, input validity and other applicable
prerequisites still govern dispatch.

This rule does not change dependencies on declared output publication or their validity/lifetime
rules. Independent eligible branches may continue, and required lifecycle cleanup retains its own
contract. Reusing a previously successful step on retry still requires valid progress. No automatic
retry, rollback or authored branching follows; exact prerequisite notation remains open.

V0 defers additional authored `on failure` and `after any completion` triggers. Operation and
instance contracts still perform required cleanup and support recovery when ordinary graph work
is blocked or cancelled. For example, migration failure blocks schema verification, while the
migration implementation remains responsible for its declared lock-release or recovery duties.
Cleanup must not depend on a failed operation reporting success. These illustrative trigger names
do not introduce syntax; provider-specific error handling remains inside plugins initially.

[Registration batches](CONFIGURATION.md#batch-configuration-submissions) use a constrained form of
this graph contract: complete supplied definitions, explicit prerequisites and ordinary checked
registry writes. They reuse scheduling, outcome, cancellation and valid-progress recovery semantics;
their definitions are validated before writes and are not generated from earlier node outputs.
General setup/change graphs retain their output-bound inputs and per-step input validation.

**Graph inputs:** v0 inputs use literal values and typed references, including configuration and
earlier step outputs. Calculations and transformations are implemented by plugin operations, not inline
expressions in graph bindings. References select declared values/fields; they do not implicitly
evaluate string interpolation, shell syntax or an expression language. Reusable graph-backed
operations receive caller configuration only through declared typed inputs, without implicitly
inheriting the caller's configuration namespace. A template's graph can bind captured configuration
values into those inputs. Scoped invocation context and explicitly authorized Resource access
remain available under their contracts; see [reusable composition](MODEL.md#authoring-concepts-beneath-the-object-families).
References use separate structured source and JSON Pointer fields as defined below. Every graph
input binding uses exactly one accepted form key; fields inside `reference` remain authoring work.

**V0 reference sources:** the supported categories are declared graph inputs, captured instance
configuration, earlier step outputs, declared runtime targets and typed invocation metadata.

| Source | Value and scope |
| --- | --- |
| Declared graph inputs | Values supplied under that graph-backed operation's input contract. |
| Captured instance configuration | Configuration available to the owning template's graph, captured for the execution. Reusable operations receive caller settings through explicit inputs. |
| Earlier step outputs | Declared published results, subject to producer/consumer validation and continued validity. Earlier means the dependency relation, not YAML mapping order. |
| Declared runtime targets | Explicit references to the owning instance's declared runtime behavior. Reusable operations receive and forward targets through typed inputs. |
| Invocation metadata | Only the runtime's declared, typed, read-only metadata surface. |

Other data access, such as reading a file, a Resource record or current service state, uses explicit
authorized operations whose results can feed later steps. Arbitrary plugin-defined reference
resolvers are deferred in v0; this does not restrict authoring new operations or capabilities.
The source set introduces no implicit parent configuration, unrestricted `self` snapshot, live
registry lookup or authority through a reference. Source names, selector fields, metadata contents
and protected-reference transport remain authoring work.

**Graph-visible invocation metadata:** graphs may reference explicitly declared, typed, read-only
metadata supplied by the runtime under its invocation contract. For example, setup can pass its
owning instance ID to an operation that tags a created service for later reconciliation, without
a plugin operation merely to retrieve that ID. Required metadata that is unavailable is reported,
not fabricated or silently replaced with caller-supplied claims.

This exposes declared invocation facts, not an unrestricted instance snapshot, ambient environment
or credentials. Runtime targets remain declared typed inputs; reusable operations receive caller
configuration explicitly. Resource reads still use authorized capabilities. An ID passed onward
is data and cannot replace the runtime's authenticated actor or authority checks. Exact metadata
fields, declaration/versioning, scope and reference encoding remain authoring work.

**Configuration capture:** configuration references within one graph execution use values captured
when that execution begins. Later steps do not silently reread changed instance settings. For example,
two steps referencing a search limit receive 10 even if a supported control confirms a change to 15
between them; a later execution captures 15. The captured values are execution inputs, while
`spec.config` remains the instance's single stored configuration map.

Capture does not establish current runtime state or freeze authority, dependency validity or
configuration revision checks when applying a control. These checks remain current under their
contracts, and may block work even when its captured inputs are schema-valid. Retry/recovery still
revalidates retained results against relevant inputs and prerequisites; capture does not justify
reusing stale results, bypassing access changes or blindly repeating effects. Exact capture/storage
representation and recovery mechanics remain open.

**Reference recognition:** the graph recognizes references only in explicitly designated binding
positions. Literal data, ordinary configuration/provider payloads and values returned by plugins
are not recursively searched for graph instructions. Resolving a reference retrieves its value;
it does not reinterpret that value as another reference. For example, a storage plugin can receive
a document containing an example of graph-reference notation without causing the graph engine to
resolve the example. Actual references still supply their declared dependencies.

**Explicit binding forms:** every supplied v0 graph input binding identifies a literal, reference,
object assembly or list assembly form, including each field/item binding inside an assembly.
Bare values are not binding shorthand. A literal form can contain an entire ordinary object or list;
its contents are data, not more bindings. A reference retrieves data without reinterpreting it.
Assembly forms explicitly combine their child bindings into the resulting object or list.

This applies at graph authoring positions. Ordinary configuration and provider payloads keep their
data representation, and plugins receive resolved input values without the authoring forms.

**Binding encoding:** a binding is an object containing exactly one of these keys, with its payload
as that key's value. The same encoding applies to child bindings inside assemblies.
The four forms and this structure are accepted; these key spellings are working names under the
[vocabulary boundary](MODEL.md#vocabulary-review-boundary), to be settled in the future naming PR.

| Form key | Payload | Resolved value |
| --- | --- | --- |
| `literal` | An ordinary value allowed by the data profile | The payload unchanged, including whole objects/lists |
| `reference` | A structured source selection and nested JSON Pointer | The selected value; payload field names remain open |
| `object` | A string-keyed map of child bindings | An object with those keys and resolved child values |
| `list` | An ordered list of child bindings | A list of resolved child values in the same order |

For example, `{literal: 15}` supplies 15, `{object: {title: {literal: Review}}}` constructs a document,
and `{list: [{literal: first}, {literal: second}]}` constructs a list. Empty assemblies use
`{object: {}}` and `{list: []}`. Reject an empty binding object, multiple form keys, unknown binding
keys or a malformed form payload. `{literal: {reference: example}}` is valid literal data under this
encoding; its inner key does not select a graph form. Receiving schemas still validate resolved values.

**Reference structure:** represent source selection and the nested JSON Pointer in separate
structured fields, rather than one encoded reference string. For a prior output, identify its step
and declared output separately from the pointer within that value. For example, select step
`connect`, output `connection`, and pointer `/endpoint`. The source fields establish the dependency
and selected output; the pointer selects within it. Source-selection fields sit directly inside
the `reference` payload alongside its optional pointer, without an additional nested source object.
This direct-field layout is accepted. Source/contract checks, missing-value handling and literal-data
rules still apply. Supporting field names and exact source selectors remain open; this does
not approve final payload spellings, add a compact-string shorthand or create a reusable source object.

**Nested value selection:** within an explicitly selected graph input, captured configuration value
or declared output, use [JSON Pointer (RFC 6901)](https://www.rfc-editor.org/rfc/rfc6901.html).
For example, `/connection/endpoint` selects a field and `/participants/0/name` selects a field in
the first array item. The empty pointer selects the whole source value; `/` selects its empty-name
member. Standard token escaping applies: `~1` represents `/`, and `~0` represents `~`.

**Omitted nested pointer:** omitting the pointer selects the whole explicitly chosen source value,
equivalent to supplying the empty JSON Pointer. Source selection remains explicit; omission does
not infer a source, bypass source validation or turn an unavailable value into a default. For example,
selecting step `connect` and its `connection` output without a nested pointer passes that whole
output; `/endpoint` selects its endpoint member if present and exposed by the output contract.
Missing sources or selected members still produce reference-resolution errors. This semantic
default does not select the reference payload's final field names.

This selects data within the chosen source, not the source itself, a registry object or a Resource
access path. Literal strings remain data under the recognition rule above. Exact reference payload
fields remain open; accepting JSON Pointer does not introduce expressions, wildcard
selection or automatic reference evaluation.

**Missing referenced values:** report a reference-resolution error and prevent the consuming step
from running when a graph reference's pointer selects no value. Do not substitute null or treat the
input as omitted to obtain a default, even if the receiving field is optional or defaulted. For
example, a schema-valid output may omit an optional `endpoint` member, but a binding selecting
`/connection/endpoint` still fails if that member is absent. An actual selected null value is data
and must satisfy the receiving schema; null does not mean the selection is missing.

Catch provable errors early and unresolved selections when inputs are resolved, under the existing
input-validation boundary. Preserve earlier effects, outcomes and uncertainty; independent branches
whose prerequisites remain satisfied continue under the normal scheduling rules. This handling does
not change declared defaults for genuinely omitted configuration/input fields. Exact diagnostic
fields remain authoring work.

**Structured inputs:** graphs may explicitly assemble objects and lists from literals, references
and nested assemblies, without a plugin merely to build that structure. Each declared field or
list item supplies a value; output references contribute their normal execution dependencies. For example,
a document can combine the literal title `Review` with a body taken from an earlier step's output.
Literal subtrees and resolved output values remain data under the recognition rule above. Assembly
does not add automatic merging, arithmetic, string manipulation, loops or inline expressions;
calculations and other transformations still use plugin operations. Assemblies use the `object` and
`list` encoding above; fields inside reference payloads remain open.

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

**Scheduling after failure:** after a setup/change graph step fails, v0 continues scheduling
independent steps whose own prerequisites remain satisfied. For example, a failed service login
does not prevent unrelated workspace preparation. Work with unmet prerequisites remains blocked;
successful independent branches do not make required unfinished setup complete. Current authority,
readiness, target coordination, scheduling limits, cancellation and mandatory stop rules still apply.
Authors declare dependencies/gates where later work must wait for earlier work to succeed.

Retain completed and already-dispatched outcomes, effects and uncertainty under their existing
contracts. Continuing eligible branches neither retries failed work nor rolls back effects. This
default concerns scheduling within the requested graph; it does not start unrelated Agent turns,
provision unselected Pool entries or bypass another instance's prerequisites.

**Explicit graph cancellation:** when an authorized caller cancels a graph, stop starting ordinary
steps and request cancellation of its in-flight child operations where their contracts support it.
Continue tracking actual outcomes, effects and uncertainty, and perform required contract-defined
cleanup. Unsupported cancellation or missing confirmation must remain visible; the request alone
does not establish that work stopped, effects were undone or cleanup completed.

For example, cancellation can ask an active download to stop while an already-created database
remains governed by its instance lifecycle. Cancelling preparation still detaches the attachments
retained by that preparation under the [connection rules](CONDUITS.md#conduits),
preserving other consumers and backing Resources. Graph cancellation does not itself retire an
instance or authorize arbitrary cleanup, rollback or replay. Exact propagation and transport
mechanics remain open within this boundary.

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
A graph requests instance-managed startup through an ordinary typed plugin operation explicitly
bound to the instance's declared runtime, such as starting a database before migrations. The
operation supplies prepared inputs through its compatible contract; the instance's responsible
service owns ongoing activity from startup. Setup plugins do not remain permanent supervisors.
This reuses operation composition, validation and recovery rather than introducing special
lifecycle node kinds or automatic startup merely because input values become available.
Registration still starts nothing.

**Multiple managed runtimes:** an instance may declare several named managed runtimes when its
type supports them. For example, a database Resource can declare a database driver and an optional
diagnostics helper supplied by different contributions. Operations explicitly select their intended
runtime targets; all components remain under the owning instance's lifecycle and supervision.
Existing setup, control and retirement contracts determine preparation requirements, failure
handling and shutdown duties. Cardinality alone adds no blanket failure or readiness policy.
This permits neither an additional primary Agent execution nor a change in Resource/Pool or
endpoint ownership. A declaration does not imply a separate operating-system process.

**Runtime target inputs:** an operation receives its declared runtime target as an explicit typed
input through the ordinary graph binding machinery. A reusable graph-backed operation declares
that input and explicitly passes it to the child operations that need it; the target is not supplied
through a separate runtime-context binding. For example, database startup receives the declared
runtime target and prepared storage, then publishes a usable database result under its contract.
The declaration reference identifies what to establish; it does not establish startup or substitute
for that later result. Operations that need no runtime target require no such input.

Authenticated caller identity and authority remain trusted invocation context. Supplying a target
does not grant access or let the caller forge that context; resolution and invocation still enforce
current authority and the target's contract. This choice fixes target placement, while reference
encoding, runtime selection fields and protected-reference transport remain authoring work.

**Managed references:** Conduit-declared live service and protected authentication bindings share
a typed managed-reference contract for ownership, resolution and validity. The owning implementation
retains actual live objects and credentials; operations exchange references under compatible
capability contracts. Those contracts supply the particular behavior and lifetime guarantees.
For example, database startup can publish a managed service reference consumed by migration, while
authentication publishes a protected binding consumed by connection setup. A common reference
contract does not make those capabilities interchangeable.

Possession of a reference supplies no grant. Resolution and use remain subject to current authority
and the binding's validity rules. Successful schema validation or a retained reference alone cannot
establish that a service is currently available. Inspection, recording and export must respect the
value's sensitivity and custody rules; a reference is not automatically safe to log.

This describes values passed between operations, separately from the graph's `reference` form
that selects a value. It adds neither another source category nor a mandatory central broker or
object family. Native provider MCP schemas retain their own semantics. Exact reference fields,
wire encoding, transport and resolution mechanisms remain authoring work.

The startup operation's contract defines the guarantees of its published results. Request
acceptance, usability for subsequent setup, completed application preparation and current runtime
availability remain distinguishable. For example, a database usable for migration is not evidence
that its application schema is ready. Startup cannot wait for a condition whose establishment
requires a later step that is itself waiting for startup's output. These are contract-specific
guarantees, not a universal readiness enum or permanent availability promise.

An explicit runtime binding does not grant authority, expose internal setup operations or make an
arbitrary returned process ID a managed runtime. Graph completion does not end continuing runtime
activity. No Runtime object family, mandatory primary plugin or process per instance follows;
an attachment type can require no persistent process, and a cloud provider retains supervision of
its own service. Exact target-binding representation, prepared-input layout and named-runtime
declaration syntax remain authoring work. Start/wait operation granularity must make each
operation's guarantees explicit under the existing contracts. The
[database/cloud comparison](../research/RUNTIME_ESTABLISHMENT.md) records the alternatives and rationale.

Required setup failure normally retains an already-started managed runtime for inspection and
explicit retry/stop. Retry of unchanged setup continues from still-valid recorded progress under
current authority and plugin recovery rules. Effects of permitted configuration changes on retained
progress need explicit invalidation/recovery semantics; do not reuse incompatible step results.

Required connection failures block initial preparation, optional ones report and continue, and
successful partial attachments are retained for retry. After successful setup, consumers normally
continue independent work when a dependency is unavailable. The selected MCP example uses bounded
reconnect with a manual option; this is not a universal operation retry policy or permission to
replay writes/start new model work. Exact graphs, waits, cancellation mechanics, retry bounds, live-change
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
