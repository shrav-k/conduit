# Deferred authoring and syntax proposal

Start at [Author with the protocol](../../authoring/README.md) for the accepted semantic path.
Remaining review work is consolidated in [REVIEW](../../REVIEW.md); this packet preserves examples
for later syntax discussion and adds no second active question queue.

The [vocabulary boundary](../../protocol/MODEL.md#vocabulary-review-boundary) now makes all
Conduit-owned names below the five object families provisional, including earlier selected field
spellings. Keep using them consistently while reviewing logic and structure; naming has a separate
future PR and is not a prerequisite for these walkthroughs.

Status: **ready for the next syntax/conformance review; examples remain proposals**, 2026-11-02. The
[conceptual closure review](../../archive/reviews/CONCEPTUAL.md) is complete. The published-contract and no-primary-plugin
rulings remain accepted. Templates now use one named dependency map and alias-plus-export references
for setup, runtime and controls, with kind/contract checks and no primary alias. The specific
`spec.plugins` / `{plugin, name}` spellings remain proposals.
Moving these examples does not ratify their names, package layout or scoping rules.

Controls may explicitly support partial confirmation: record confirmed permitted settings only
when the complete resulting stored configuration is valid, and keep remaining effects' failure,
pending work or uncertainty separate. Coupled settings may require whole-change confirmation.
This does not imply rollback, generic field independence or that last-confirmed values are still
active. Exact result representation remains open.
Controls confirming unchanged stored configuration retain its revision while recording their
invocation and effects. Applicable checks and declared handler behavior still run; matching values
do not justify skipping work or substituting a cached result for a new execution.

Steps are required by default across setup, reusable graph-backed operations and change-handler
graphs, with explicit optionality for nonessential work. Required steps must succeed for successful
graph completion; optional failures remain visible without alone preventing it. Actual dependencies,
validation and mandatory type/access obligations still apply. Public outputs and permitted partial
control confirmations keep their own rules. Optionality abandons no outstanding work and does not
change scheduling, retained effects or lifecycle duties. Exact step notation remains open.
Graph steps form a map keyed by locally unique names. Dependencies supply ordering constraints;
document order has no execution meaning. Different graphs may reuse local names without sharing
steps or exposing called graphs' internals. Container names remain provisional.

The accepted package entry boundary uses one explicit document declaring exports and dependencies,
with inline or package-local referenced definitions. It may be handwritten or generated; inspection
does not execute plugin code or infer exports from directory layout. Provider-native catalogs remain
dynamic. Concrete filenames, fields and reference notation in this packet remain proposals.
Public export names are unique across contribution kinds within a package, regardless of file layout.
Resolution still checks the expected kind and contract. Internal names, contract identities and
registry object names retain their own rules; exact name grammar remains open.
Packages may explicitly re-export a declared dependency's public contribution under their own export
name without copying or wrapping it. Preserve the original definition, contract, internal scope and
resolved dependency identities; the alias adds no behavior or access. Exact notation remains open.

The first authoring ruling now accepts [package-local scope and explicit dependency bindings](../../protocol/MODEL.md#package-scope-and-public-bindings).
Caller-local aliases cannot override a package's internal references; replaceable dependencies must
be explicitly exposed. Those dependencies may supply default implementation bindings, with compatible
explicit replacements taking precedence and invalid replacements producing errors rather than fallback.
V0 imports require explicit support for the requested contract identity and exact version in addition
to existing compatibility checks; cross-version mappings are deferred. Implementation release versions
remain independent and no backward-compatibility maintenance obligation is added.
Constrained YAML 1.2, equivalent JSON input/interchange and JSON Schema 2020-12 data contracts
are also accepted in the [configuration foundation](../../protocol/CONFIGURATION.md#configuration-language).
The remaining representations and detailed schema profile below are proposals within those boundaries.
Explicit success prerequisites belong on the dependent step beside its operation and inputs,
referencing predecessor step names in the same graph. Output bindings still imply dependencies;
the combined graph retains cycle/missing-step checks and independent scheduling. No separate authored
edge collection is required. Placement is accepted; supporting field names remain provisional.
The [graph-input boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is also accepted: v0 inputs
use literals and typed references to configuration or earlier outputs, while plugin operations
perform calculations and transformations. Inline binding expressions are deferred. References are
recognized only in designated binding positions; literal payloads and resolved plugin results remain
data rather than being recursively reinterpreted. Explicit object/list assembly from literals and
references is accepted without a plugin merely to assemble the structure. It adds no automatic merging
or inline computations. Nested selection within an explicitly chosen input, configuration or output
value uses standard JSON Pointer paths. Source selection and the pointer use separate structured
fields, not a compact reference string. Every graph input binding explicitly identifies a literal,
reference, object assembly or list assembly form, including children of assemblies. Literal payloads
and resolved references remain ordinary data; there is no bare-value binding shorthand. Each binding
uses exactly one key: `literal`, `reference`, `object` or `list`, with its payload underneath.
Declared public graph output values reuse those same four forms and source/scope rules. Publication,
validity, output validation, sensitivity and access still apply; assembly does not establish overall
success or expose other internal outputs. Domain-error mapping and remaining contract fields remain open.
Public outputs form a map keyed by names unique within that graph's public outputs. Each entry groups
its binding and applicable declarations, including optionality and success prerequisites. Reject
duplicate keys; map order does not schedule work or publication. Contract derivation and inherited
guarantees remain unchanged, without requiring them to be repeated; exact field names stay provisional.
Named public outputs are required for successful graph completion unless explicitly optional.
An optional output may be absent with its reason or uncertainty reported; an optional producer step
does not automatically make its output optional. Present values still require validation and access,
and absent values do not become null or bypass consumer reference/dependency checks.
Public outputs may also declare successful-completion prerequisites using local step names, separate
from their value bindings. A database output may use startup's reference but wait for verification
success, without a forwarding operation. Failed or uncertain prerequisites withhold that output;
the remaining publication, validity, validation and access conditions still apply. Final fields remain open.
An output becomes eligible once its own bindings, prerequisites and declared conditions are satisfied,
without implicitly waiting for unrelated work or whole-graph success. Stronger declared conditions
still apply. Output availability does not mark the graph complete or its owning instance prepared.
Named step and public graph outputs retain one fixed published value per producing attempt.
Redelivery is permitted; a conflicting publication cannot replace it. Later attempts retain distinct
result provenance. Live Resource state, output validity, access and progress/observation contracts
remain separate; this does not promise exactly-once delivery or authorize replay.
Object/list assembly payloads contain child bindings. The accepted reference layout puts source
fields directly beside the optional pointer, without a nested source object. Omitting the pointer
selects the whole explicitly chosen source, equivalent to an empty pointer. The accepted v0 sources
are declared graph inputs, captured instance configuration, earlier step outputs, declared runtime
targets and typed invocation metadata. Other data access uses explicit authorized operations;
arbitrary plugin-defined reference resolvers are deferred. Source names and exact selectors remain open.
The graph-input source selects its complete declared input payload, with the optional JSON Pointer
selecting within it and no separate input-name selector. Omitted/empty pointers select the payload
whole; no ambient caller configuration, invocation metadata or authority is added to it.
A missing selected value causes a reference-resolution error and blocks the consuming step, without
substituting null or applying a default for omitted input. An actual selected null is validated normally.
Earlier effects and independent eligible branches retain their agreed behavior.
Configuration references use values captured when each graph execution begins. Later executions use
then-current settings; capture does not freeze permissions, dependency validity or control revision
checks. Retry/recovery must still revalidate retained results. Exact representation remains open.
Reusable graph-backed operations receive caller configuration through declared typed inputs, without
implicit configuration inheritance. Scoped invocation context and authorized Resource access retain
their own contracts. Graphs may reference explicitly declared, typed, read-only metadata supplied
by the runtime, such as their owning instance ID. This exposes neither an unrestricted instance
snapshot nor credentials, and passing an ID does not establish caller identity or authority.
Exact metadata fields and input/context notation remain open.

Setup requests instance-managed startup through ordinary typed operations explicitly bound to
declared runtimes. The runtime-target source selects one named runtime and yields a typed reference
that its responsible service resolves against the instance-owned definition. The target is a declared
typed input, explicitly forwarded through reusable compositions; prepared settings remain separate
explicit inputs. The instance owns ongoing activity from startup, and published results
establish their contract's guarantees. Caller identity and authority remain trusted context;
target selection grants no access, starts nothing and permits no definition edits. Exact target-reference
encoding and inspection remain open.
An instance may declare multiple named managed runtimes when its type supports them, each explicitly
targeted and instance-owned. Existing lifecycle contracts govern their behavior; the allowance adds
neither a process-per-declaration rule nor another primary Agent execution. Runtime declarations form
a map keyed by names unique within the instance definition; each entry contains its declaration.
Duplicate keys are errors. Map order has no execution meaning, and registration starts nothing.
Container and driver/input field names remain open.
Conduit-declared live service and protected authentication bindings share a typed managed-reference
contract for ownership, resolution and validity. Capability-specific behavior and lifetime remain
explicit; actual live objects/credentials stay with their owner. Possession grants no access, and
a stored reference does not prove current availability. This concerns operation values, separately
from graph reference notation; exact fields and transport remain open.
The [input-validation boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: catch
provable wiring/type errors early and validate actual resolved inputs before each step's dispatch.
For Conduit-authored operations, prepare unambiguous declared defaults for omitted input fields
before that validation and invocation. Code and graph implementations receive the same prepared
payload, and the graph-input source exposes it. Explicit invalid/null values and broken references
do not receive fallback defaults; provider-native MCP inputs retain their own semantics.
V0 requires no proof of complete producer-to-consumer schema inclusion; invalid actual inputs block
the receiving step, with prior effects/evidence preserved. Required schema support remains mandatory;
checking algorithms and diagnostics remain open.
Declared plugin outputs are also validated against their producer's contract before downstream use.
Invalid output is withheld with a contract error, preserving effects/uncertainty rather than claiming
rollback or safe replay. Downstream input checks remain separate, and provider-native schema semantics
are unchanged. This is accepted behavior; exact error/result notation remains open.
V0 explicit ordering prerequisites without data require successful completion under the earlier
operation's contract. Failure or uncertainty does not satisfy them. Output-publication dependencies
and required lifecycle cleanup retain their own rules; prerequisite notation remains provisional.
Additional authored failure/completion triggers are deferred. Plugins and instance lifecycle contracts
still fulfill required cleanup and supported recovery when ordinary graph work is blocked or cancelled.
After a setup/change graph step fails, independent steps continue when their own prerequisites and
current authority remain satisfied. Unmet prerequisites block affected work, required unfinished
setup remains incomplete, and cancellation/stop rules still apply. No automatic retry or rollback
follows. Detailed scheduling mechanics remain open within this accepted policy.
Explicit graph cancellation stops new ordinary steps and requests supported cancellation of
in-flight children. Track actual outcomes and required cleanup; cancellation does not prove effects
stopped or replace instance lifecycle rules. Exact propagation/transport mechanics remain open.
The accepted [schema-resolution rule](../../protocol/CONFIGURATION.md#schema-resolution) uses
the selected package and supplied/declared schema dependencies, reporting missing definitions
without automatic network fetching during validation. Dependency acquisition remains explicit.
Conduit-authored schemas also [reject unrecognized keywords](../../protocol/CONFIGURATION.md#schema-keyword-acceptance),
with additional custom descriptive annotations in one dedicated map of namespaced entries.
Reuse standard descriptive annotations where applicable. Custom hints do not implement validation,
permissions or runtime behavior; the map keyword, namespace syntax and value schemas remain open.
Provider MCP schemas [retain their declared dialect's semantics](../../protocol/CONFIGURATION.md#provider-mcp-schemas);
allowed descriptive annotations do not alone reject a tool, and unsupported required validation
features are reported without stripping constraints or claiming enforcement. Adapter details remain open.
Formats in Conduit-authored schemas are enforced. V0 selects all 19 JSON Schema 2020-12 standard
formats with offline syntactic checks and conformance tests before support claims. Unsupported
formats are reported; plugin-defined validators are deferred. Supported keywords/reusable schemas
can still describe custom constraints, and operations, Resource contracts and requirement checks
remain extensible.
The accepted [default-conflict rule](../../protocol/CONFIGURATION.md#schema-default-resolution)
requires an explicit valid value or corrected defaults when an omitted field has conflicting
applicable schema defaults. Traversal order cannot select a winner.
An omitted optional object without its own applicable default stays absent; child-property defaults
do not create it. Authors may explicitly default the parent. This applies to instance configuration
and Conduit-authored operation inputs, retaining validation and template omission preservation.
The accepted [integer range](../../protocol/CONFIGURATION.md#numeric-interoperability) is
−9,007,199,254,740,991 through 9,007,199,254,740,991 for Conduit-declared configuration, inputs
and outputs. Larger exact values can use an explicitly string-typed contract; implicit conversion
and rounding into range are forbidden. Ordinary fractional values use finite binary64 semantics,
including normal representational rounding. Exact decimals use explicit decimal-string or
scaled-integer contracts. Core revision/ID encodings and detailed conformance fixtures remain open.
The accepted [batch boundary](../../protocol/CONFIGURATION.md#batch-configuration-submissions) supports
multi-object submissions through ordinary checked writes, with per-object outcomes and retained
partial successes. Explicit intent and preconditions remain required; no atomicity or automatic setup
is implied. Complete parsing and schema validation precede all writes; an invalid entry or missing
required schema prevents them. Current authority and state checks still apply at each write.
Registration batches use finite graphs with complete supplied definitions and explicit prerequisites;
independent eligible writes may run concurrently. Shared graph failure, cancellation and recovery
semantics apply. File order does not supply dependencies and outputs do not construct later batch
definitions. Only one write per target object is allowed in a batch; duplicate update IDs or create
kind/name pairs are rejected before writes even if ordered. Multiple distinct instances from one
template remain supported. Batch notation and detailed conflict diagnostics remain open.
Resolved package artifacts retain [package identity and a content digest, plus release identity when released](../../protocol/CONFIGURATION.md#implementation-selection),
covering declarations and any bundled code, including template-only packages. Digest
algorithms, bundle boundaries and concrete fields remain open; this does not establish publisher trust.
Managed execution of local packages uses retained immutable snapshots, while direct handler tests
may use editable source. Explicit managed test/start workflows can capture a revision automatically;
the running instance preserves its code/contracts without archiving every edit-mode revision.
Deliberate preservation requires a versioned publication of fixed package contents through a public
or private distribution source. Conduit need not host or publish releases; the mechanism remains open,
and a local-only release facility is not selected. Changed released contents require a new version; draft
version fields or test captures alone are not releases. Ordinary development replaces affected
instances without requiring two active versions or publication per edit. Missing selected code
blocks affected restart/recovery rather than being replaced
silently. Included files/directories are explicitly declared, with directory/pattern rules available;
capture does not default to the
whole working directory. Captured paths and symlink targets must remain within one explicit
source directory; outside targets produce an error rather than implicit inclusion. Exact
inclusion syntax, internal symlink representation and cleanup remain open, and existing instances
do not adopt edited source automatically. Resource paths/runtime access remain separate.

Before resuming syntax rulings, read the [language/protocol research](../../research/language-protocol-foundations/README.md)
and [worked comparisons](../../research/language-protocol-foundations/WALKTHROUGHS.md).
WIT/Smithy import boundaries, CWL composition and YANG controls provide concrete alternatives;
their recommendations do not silently amend this packet or the accepted protocol.

| Document | Preserve it for |
| --- | --- |
| [AUTHORIZATION](AUTHORIZATION.md) | Accepted requirements/grants illustrated with unapproved representation and enforcement details, moved here from the runtime packet. |
| [AUTHORING](AUTHORING.md) | Small-handler, managed-capability, code-optional MCP and composite-operation author journeys; illustrative APIs. |
| [CONTRACTS](CONTRACTS.md) | Proposed schemas, graph/output bindings, operation context and managed MCP mappings. |
| [PACKAGE_WALKTHROUGH](PACKAGE_WALKTHROUGH.md) | One history package from publication through setup/runtime/controls, applying accepted package scope while leaving binding/default syntax open. |

These documents include useful conceptual examples, but the [design](../../protocol/README.md) is the source
of accepted semantics. Earlier question sequences remain in Git rather than another active queue.
The [implementation proposal](../../implementation/README.md) preserves runtime options separately.
The [post-migration review](../../REVIEW.md) is complete: lightweight delivery, no backward-compatibility
promise, contexts as ordinary Resources, Apache-2.0, deferred public naming and owner-led authority
are recorded. These decisions do not approve the package/export/schema spellings in this packet.

## Later PR scope

Review one connected contract/package/template/instance journey, including defaults, fixed
implementation identities, graph composition, controls and failures. Select syntax only after the
semantics are clear. Keep package identity, template-local aliases, published exports and provider
MCP method names distinguishable. No mandatory primary implementation is reintroduced.

Choose API/type/capability version identifiers, the detailed schema profile, supported representation rules,
reference scope and MCP/profile bindings together. Define executable conformance examples from
accepted behavior, including rejected/unsupported and uncertain cases. Do not let fixture contents
automatically become protocol requirements. Update Experiment 001's proposed YAML and Experiment
002's test plan with explicit changes and evidence when that pass happens.

Nothing in this folder is an installable plugin package or an implemented runtime. Existing static
checks prove only the properties they actually tested. This reorganization adds no runtime evidence.
