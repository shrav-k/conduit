# Proposed implementation contracts

**Deferred authoring packet:** preserve these examples for the later syntax/conformance PR.
The [conceptual review](../../archive/reviews/CONCEPTUAL.md) is complete; [this packet](README.md) changes no accepted rules.

**Placement review:** separate management/execution services and shared policy with instance-owned
state are accepted. V0 has no offline grants. The first proof uses separate management and two
executor processes on one machine; remote support requires later two-machine validation.
References to a host below mean the responsible management
or execution service, not one combined process. Independent execution and network Conduits
require authenticated endpoint identity, retained target outcomes and authority-freshness rules.
Their detailed mechanisms remain implementation/conformance work within
[accepted I1 boundaries](../../archive/reviews/IMPLEMENTATION.md#i1--independent-execution-and-management); serialization alone
would not make this a complete distributed protocol.

Status: draft contract design, 2026-11-02. All new field names, namespaces, limits and wire
profiles here are proposed. [DESIGN](../../protocol/README.md) supplies accepted semantics;
[review I3 and I7](../../archive/reviews/IMPLEMENTATION.md) cover the main extension and authoring choices. Examples are
explanatory, not files accepted by an existing Conduit executable.

The companion [Policy and Grant contracts](AUTHORIZATION.md) specify the accepted authorization
model and proposed detailed structure for Agents, Interfaces and other managed callers. Policy and Grant belong in the
protocol vocabulary beneath the five families; I4 settles their conceptual semantics and I7 chooses
exact schemas. Requirements describe guarantees, plugins implement operations and
checks, and Conduit paths enforce current authority. Neither package installation, graph setup nor
MCP connectivity grants permission by itself. Use that document as the single detailed proposal
for authorization inputs, issued records, decisions, delegation and evidence.

**Accepted persistence extension:** v0 publishes a minimal typed contract for storing/recovering
management and executor state, with one bundled implementation. The owning service still decides
state transitions and permissions. The contract must support its required checked updates,
durable progress and reconciliation of uncertain writes; exact methods, schemas and conformance
fixtures belong to the final authoring pass. The [storage boundary](../../implementation/ARCHITECTURE.md#storage-boundary)
uses a service runtime dependency in v0 with SQLite for the first bundled implementation. Separate public semantics
from engine/hosting details to support a future Pool-owned Resource representation; its migration
and foundational access/lifecycle rules are deferred. This is not a general database/query API or
permission for ordinary plugins to edit another service's state.

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
| Package artifact | Accepted package identity and content digest, plus release identity when released; exact fields and digest format open | Edit-mode captures preserve running work; versioned releases deliberately preserve fixed contents. Covers contracts, graphs, templates and any bundled code, including code-free packages; not publisher trust or remote service code. |
| Storage schema version | Implementation-managed format revision | Interpretation/migration of the selected backend's persisted state; distinct from the public persistence contract and object protocol versions. |
| Observation cursor | Opaque scoped cursor | Where a client resumes reading; not permission or proof it understood earlier content. |

Readable references retain `{kind, form, name}`. Established bindings store resolved IDs separately.
Instance names default to template name plus a generated suffix, with uniqueness checked and
collision retried. Registry-name aliases remain deferred. Names are unique within family across
both forms; a Pool's local entry name is not another global object name.

## 2. Object interpretation and YAML

The common envelope remains `apiVersion`, `kind`, `form`, `metadata`, `spec`. Normal input rejects
`status` and system-owned metadata. Inspection adds system identity, revisions and observations;
configuration export omits system fields and grants. Checked writes carry preconditions separately.

**Accepted composition:** templates require complete compatible bindings, not a primary plugin.
Templates select implementations in one named dependency map; setup steps, runtime drivers and
control handlers use a dependency alias plus public export name, with kind/contract checks at each
use. The table's `plugins` field and the concrete export-reference notation remain proposals.
Declared type defaults can still supply handlers and other supported behavior, resolving their
selected implementations explicitly on the instance.

| Field | Responsibility |
| --- | --- |
| `type` | Exact group/name/version contract; uniform across the five families in this proposal. |
| `plugins` | Named selected implementation dependencies for graph, handler and driver exports. No entry is implicitly primary. |
| `config` | The one stored configuration map: initial materialized values, then confirmed control changes. |
| `setup` | Inline finite graph or explicit graph-source reference in a template; resolved graph in the instance. |
| `runtime` | Named instance-owned driver declarations and typed prepared-input bindings; exact collection layout remains open. |
| `controls` | Type-default selection/narrowing in a template; fixed resolved controls in an instance. |
| `conduits` | Consumer requirements and explicit open/join selections. |
| `owner` | Resource-instance owning Pool, established by Pool-authorized creation rather than self-asserted by YAML. |

**Accepted runtime cardinality:** an instance may declare several named managed runtimes when its
type supports them. For example, one database Resource can use a database driver and an optional
diagnostics helper from separate contributions, each with an explicit startup target. A wrapper
driver solely to combine them is not required. The table's singular `runtime` spelling is provisional;
the collection layout and supporting vocabulary remain open.

This concerns declared managed behavior, not one process per declaration. The instance owns its
components; Resource responsibilities, Pool ownership and endpoint ownership of connection-specific
activity remain unchanged. It does not permit a second primary Agent execution. Existing setup,
control and retirement contracts govern readiness, failure handling and stopping; cardinality does
not create a blanket failure policy.

The named map can hold one implementation or several. A filesystem operation and
an authentication operation may come from different packages. Aliases name implementation bindings,
not Resources, processes or permission. The `plugins` and `{plugin, name}` spellings remain proposals;
the named-map and alias-plus-export structure is accepted, without a primary alias.
Accepted [package scope](../../protocol/MODEL.md#package-scope-and-public-bindings) keeps internal
references in their defining package; caller bindings replace only explicitly exposed dependencies.
Those dependencies may declare default implementation bindings. A valid explicit replacement takes
precedence; an invalid one reports an error rather than selecting the default. Selection follows
the existing implementation-retention rules and does not install packages or grant Resource access.
V0 imports require the selected export to explicitly support the requested contract identity and
exact version, in addition to the existing compatibility checks. Cross-version mappings are deferred;
package release versions do not determine contract-version compatibility.
Do not create a package/module hierarchy in the object registry
merely to resolve exports. The same rule applies to default bindings when they are resolved.

### Parsing and validation pipeline

1. Apply input byte/document/depth limits and parse YAML or JSON while retaining source locations.
   YAML may use a marked event stream; the JSON path must also detect duplicate keys and
   numeric violations before losing their evidence, using the accepted integer/fractional profile.
2. Reject aliases, anchors, custom tags, duplicate keys, non-string keys and nonfinite numbers
   before a generic map conversion could erase their source evidence.
3. Interpret YAML 1.2 Core scalars. Preserve quoted strings; never convert a string to a number
   merely because a schema expects one. Empty collections, omitted fields and allowed nulls differ.
4. Validate common envelope and field ownership. Under the accepted schema-resolution rule,
   resolve from the selected package and its explicitly supplied or declared schema dependencies;
   report missing required definitions without automatic network fetching. Dependencies may be
   acquired explicitly beforehand. No implicit environment/shell expansion occurs during validation.
   Conduit-authored schemas reject unrecognized keywords. Reuse standard descriptive annotations;
   additional custom metadata uses one dedicated namespaced map. These optional tooling hints do
   not implement validation, authority or runtime behavior. The map keyword, namespace syntax,
   value schemas and detailed adapter handling of external MCP schemas remain open.
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

The accepted foundation uses JSON Schema 2020-12 for data validation with equivalent YAML/JSON
representations, plus explicit Conduit semantics for defaults, bindings and transitions.
The remaining pipeline and profile details here are proposed. JSON Schema's `default` is an annotation and does not
materialize values. Implement that pass ourselves. The accepted default-conflict rule reports
conflicting applicable defaults for an omitted configuration field, requiring an explicit valid
value or a correction to the defaults rather than selecting by traversal order. All validation
constraints still apply; the resolution algorithm remains open. Close records explicitly; schemas
with composition need correct `unevaluatedProperties` behavior. [Defaults][schema-defaults] ·
[Object properties][schema-objects]

**Accepted format behavior:** Conduit-authored schemas enforce declared `format` constraints,
using the shared profile's supported formats and reporting unsupported ones instead of silently
accepting them. V0 selects all 19 JSON Schema 2020-12 standard formats, using their standard meanings
and offline syntactic checks, with conformance tests before support claims. Plugin-defined format
validators remain deferred. Custom constraints can still use supported schema keywords/references,
with operations and requirement checks retaining their extension contracts.
External provider MCP schemas retain their declared dialect's semantics under the boundary below.

**Accepted provider boundary:** do not automatically impose the stricter Conduit authoring profile
on existing provider MCP schemas. An unfamiliar descriptive annotation permitted by that dialect
does not alone reject a tool. Report unsupported required validation features without silently
removing constraints or claiming they were enforced. Exact adapter handling and conformance
cases remain open; this rule does not establish runtime authority or enforcement support.

**Accepted integer boundary:** Conduit-declared integer configuration, inputs and outputs use
−9,007,199,254,740,991 through 9,007,199,254,740,991, subject to narrower schema bounds. Detect
out-of-range values before parsing loses precision. A contract needing a larger exact integer
can explicitly use a decimal string; no automatic conversion is introduced.

**Accepted fractional boundary:** ordinary fractional configuration, inputs and outputs use finite
IEEE 754 binary64 semantics, including normal representational rounding. Exact decimal quantities
use explicitly declared decimal-string or scaled-integer contracts. This refines the earlier blanket
precision rule while preserving exact integer checks and provider-native schema semantics.

Remaining proposed interoperability profile: core counters/revisions use strings where large exact
values matter; plugin numeric fields declare bounds. Apply the accepted numeric interpretation and
report unsupported ranges rather than silently substituting zero or infinity. The parser spike should
settle canonicalization and constraint/round-trip conformance; unrestricted decimal precision is not promised. Keep
field-specific list ordering; comments and mapping order are not configuration identity.

**Accepted batch boundary:** v0 supports multiple object definitions in one submission through
individual checked registry writes, with per-object outcomes and retained partial successes.
Each entry has explicit create/update intent and applicable preconditions; there is no atomic batch
guarantee, inferred rollback or automatic setup. An update needs its own target's baseline.

**Accepted prevalidation:** parse the complete submission and validate every entry against its
required schemas before any registry writes. Parsing/schema failures or missing required schemas
prevent all writes. Passing this check does not reserve names/revisions or freeze authority;
state-dependent checks still apply at each write and can produce partial outcomes.

**Accepted graph structure:** registration batches use the existing finite, acyclic operation-graph
contract, constrained to complete supplied definitions and ordinary checked registry writes.
Explicit prerequisites determine order; file position and ordinary object references do not supply
implicit edges. Independent nodes may execute concurrently under target coordination and current
authority. Shared graph failure, cancellation, outcomes and valid-progress recovery semantics apply.
Validate the graph and all definitions before any writes. Earlier node outputs cannot generate or
modify later definitions in this v0 batch form; general operation graphs keep their existing dataflow.
This replaces the unaccepted sequential-order proposal; no new object family or separate workflow
language is introduced.

**Accepted target uniqueness:** allow only one write per object within a registration batch. Reject
duplicate update targets by stable ID and duplicate create targets by kind/name before any writes,
even when explicitly ordered. This does not limit general operation graphs, separate submissions,
or how many distinct instances can be created/launched from the same template. Template reuse is
not a duplicate target. Concrete batch notation and detailed conflict diagnostics remain open.

## 3. Templates, instances and bindings

Template use produces an ordinary instance draft. Callers edit fields, not a new parameter
language. Assigning a field replaces that value; editing a child leaves siblings intact. A
graph-source reference resolves to concrete graph content before execution and is retained in
the instance definition. Package-artifact retention does not introduce a separate per-instance
source-template snapshot or input/override provenance ledger.

Capture implementation bindings from explicit selection or one unambiguous configured default.
Pin published declarations and Conduit-controlled adapter/code artifacts. Code-optional packages
can bind existing MCP implementations with no new executable; external targets follow existing
reference/configuration rules, with observed compatibility distinct from code pinning.
Schema-only registration may know the required binding without having its code or service available.
If selection itself remains unresolved, preserve the request as unresolved and block execution;
the first successful resolution is recorded once and is not recomputed from later defaults.
Instances normally export explicit resolved selections. An export with unresolved dependencies
must say so; it must not claim to be a runnable, fully resolved recipe.
Even a resolved selection records identity, not a guarantee that historical executable contents
remain available. Export does not make a running instance's private code snapshot installable.

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

**Accepted input boundary:** v0 inputs use literal values and typed references, including configuration
and earlier outputs. Calculations and transformations belong in plugin operations; inline expressions
in graph bindings are deferred. Output references retain their existing dependency meaning.

**Accepted reusable-operation scope:** a reusable graph-backed operation receives caller configuration
only through declared typed inputs. It does not implicitly inherit the caller's configuration namespace.
For example, a template binds its captured search limit to the operation's `limit` input; the operation's
internal graph reads that input. Scoped invocation context and explicitly authorized Resource access
remain available under their contracts. The template-local setup example below can reference its own
configuration, but extracting it into a reusable operation requires explicit configuration inputs.
Exact input/context notation remains open.

**Accepted graph-visible invocation metadata:** graphs may reference an explicitly declared, typed,
read-only metadata surface supplied by the runtime. For example, a setup graph can pass its owning
instance ID to a plugin that tags a created service for later reconciliation, without an operation
solely to retrieve that ID.

Its scope is declared invocation facts, not an unrestricted snapshot of instance configuration/status,
ambient environment or credentials. Missing required metadata is reported, not fabricated.
Runtime targets still use their declared typed
inputs, reusable operations still receive caller configuration explicitly, and live Resource reads
still use their authorized capabilities. Referenced IDs become ordinary data for their consumer;
they cannot replace the runtime's authenticated actor or authority checks. Exact available fields,
declaration/versioning, scope and reference encoding remain open. There is no unrestricted `self`
source; the example below selects an explicitly assumed metadata field instead.

**Accepted v0 source inventory:** use the following five categories for graph references.
Arbitrary plugin-defined source resolvers are deferred; category names and exact selectors remain
provisional.

| Value source | Example | Scope |
| --- | --- | --- |
| Declared graph inputs | A reusable operation's complete input payload, with a pointer selecting its supplied query limit | Its published input contract. |
| Captured instance configuration | A template setup graph's configured directory | Captured for that execution; caller configuration enters reusable operations through declared inputs. |
| Earlier step outputs | The directory produced by attachment | Declared publication, validation and continued validity rules. |
| Declared runtime targets | A typed target reference selecting the owning instance's named database runtime | The responsible service resolves its definition; reusable operations receive and forward targets as inputs. |
| Invocation metadata | The owning instance ID supplied by the runtime | Only the declared typed, read-only surface; no authority from passing an ID. |

Reading a file, a Resource record or current service state remains an explicit authorized operation
whose output can feed later steps. References do not acquire
plugin-defined external-fetch behavior. This keeps extension work in operations and capabilities
initially; further source forms could be reviewed later. Exact selectors and source schemas remain open.

**Accepted graph-input root:** this source selects the complete declared input payload of the graph
invocation, using the optional JSON Pointer within it and no separate input-name selector. Given
`{limit: 15, options: {timeout: 30}}`, `/options/timeout` selects 30; omission or an empty pointer
selects the whole payload. Existing missing-selection and validation rules still apply. No ambient
caller settings, metadata or authority enter the source; reusable operations still receive declared
typed inputs. Exact selector fields and input-contract representation remain open.

The source sees the prepared operation-input payload: Conduit applies unambiguous declared defaults
to omitted input fields and validates the result before entering the graph, under the accepted
preparation rule below. An omitted `limit` defaulted to 10 is therefore selectable as `/limit`.

**Accepted runtime-target value:** the source selects one named declaration and returns a
typed target reference, resolved by the responsible service against the owning instance's runtime
definition. It does not supply the structured declaration itself as target data. The operation
receives its prepared settings through declared inputs. Owner/identity, contract and current
authority checks still apply; selection neither starts the runtime nor permits changing its fixed
definition. Reference fields and supported inspection remain open; a typed reference need not imply
one opaque wire representation or prohibit authorized definition inspection.

**Accepted recognition boundary:** references are recognized only in explicitly designated binding
positions. Literal data and ordinary configuration/provider payloads are not scanned recursively for
reference markers. A plugin result retrieved through a reference remains data rather than being
reinterpreted as graph instructions.

**Accepted binding forms:** every supplied graph input binding explicitly selects literal,
reference, object assembly or list assembly, including field/item bindings inside assemblies.
Bare-value shorthand is not supported. Literal payloads may be whole objects/lists and are not
recursively wrapped or interpreted. The forms are authoring instructions; plugins receive resolved
values. Ordinary configuration and provider payloads remain data.

**Accepted form encoding:** every binding object contains exactly one of `literal`, `reference`,
`object` or `list`, with that form's payload as its value. `object` takes a map of child bindings;
`list` takes a list of child bindings. Reject empty binding objects, multiple/unknown form keys and
malformed payloads. Empty assemblies are `{object: {}}` and `{list: []}`; a literal object containing
these keys remains data. The [lifecycle contract](../../protocol/LIFECYCLE.md#lifecycle-protocol)
defines the encoding. Fields inside the `reference` payload are still proposals.

**Accepted reference structure:** source selection and nested JSON Pointer use separate structured
fields directly inside the `reference` payload, without a nested source object. For an output
reference, the source identifies the step and declared output; the pointer selects within that
output value. The following uses the accepted direct-field layout with provisional field names:

```yaml
reference:
  from: output
  node: connect
  port: connection
  path: /endpoint
```

This selects `endpoint` inside the `connection` output from `connect`, retaining the normal output
dependency and missing-value checks. It does not introduce a compact-string reference grammar.

**Accepted nested selection:** use JSON Pointer within an explicitly selected input, configuration
or output value, following the [lifecycle contract](../../protocol/LIFECYCLE.md#lifecycle-protocol).
The pointer does not identify the source or choose the reference payload's fields. For example:

| Selected source value | Pointer | Selected value |
| --- | --- | --- |
| `{"connection":{"endpoint":"https://example.test/mcp"}}` | `/connection/endpoint` | `"https://example.test/mcp"` |
| `{"participants":[{"name":"reviewer"}]}` | `/participants/0/name` | `"reviewer"` |
| `{"a/b":{"~key":15}}` | `/a~1b/~0key` | `15` |

These examples approve nested paths only. A literal string containing one of these paths is still
literal data.

**Accepted omission rule:** omitting the nested pointer selects the whole explicitly chosen source,
equivalent to supplying the empty JSON Pointer. For example, selecting startup's `database` output
without a nested path passes that whole output; selecting `/endpoint` passes its endpoint member
where present and exposed by the output contract. Source selection remains explicit, and missing
sources or selected members still error. Pointer omission does not omit the operation's input or
invoke that input's default. The direct-field reference layout is accepted; field names remain
provisional.

**Accepted missing-selection handling:** when a pointer selects no value, report a reference-resolution
error and prevent the consuming step from running. Do not turn the failed binding into null or an
omitted input that receives a default. For example, `{"connection":{}}` may be valid under a producer
schema that makes `endpoint` optional, but `/connection/endpoint` cannot resolve against it.
With `{"connection":{"endpoint":null}}`, the same pointer selects null, which is accepted only if
the receiving schema permits it. Earlier effects and independent eligible branches retain their
existing rules. Diagnostic field names remain open.

**Accepted assembly boundary:** graphs may explicitly assemble object fields and list items from
literals, references and nested assemblies. A plugin is not required merely to build that structure.
Output references retain their normal dependency semantics, and literal subtrees/resolved values remain
data. This adds no automatic merging, calculations, string manipulation, loops or inline expressions;
those transformations still use plugin operations. The `object` and `list` form encoding is accepted;
reference payload fields remain open.

**Accepted step collection:** a graph's steps form a map of locally unique names to operation
bindings. Duplicate keys are rejected; different graphs can reuse local names without sharing steps
or exposing internal nodes to callers. Dependencies supply ordering constraints rather than document
order. Current dispatch checks and scheduling limits still apply. The example below uses that map
structure with provisional container/field names. Each operation identifies a plugin alias and export.
Every supplied input uses an explicit form recognized only in the schema's designated binding
positions. Form keys and the step-map structure are accepted; other graph layout, implementation
selectors and reference payload spellings below remain illustrative.

Illustrative syntax, extending the existing fixture rather than approving it:

```yaml
setup:
  nodes:
    prepare:
      operation: {plugin: consumer, name: consumer.prepare}
      inputs:
        instanceId: {reference: {from: metadata, path: /instanceId}}
    establish:
      operation: {plugin: database, name: database.establish}
      inputs:
        target: {reference: {from: runtime, name: database}}
        configuration: {reference: {from: config}}
        access: {reference: {from: output, node: prepare, port: database-access}}
    migrate:
      operation: {plugin: database, name: database.migrate}
      inputs:
        runtime: {reference: {from: output, node: establish, port: managed-database}}
      after: [prepare]
```

This fragment assumes the enclosing definition supplies `consumer`/`database` implementations,
typed configuration and a managed runtime declaration provisionally named `database`. It assumes
the runtime's metadata contract exposes an `instanceId` field and `consumer.prepare` accepts that
ID as data; this selects neither a standard metadata schema nor an authorization mechanism.
`database.establish` receives its declared runtime target through an ordinary typed input and
requests startup from the responsible execution service. The `metadata`/`runtime` source spellings,
the runtime-name selector and input names are illustrative, not final syntax.
Reusable graph-backed operations receive and forward the input explicitly. The declaration reference
and the usable database output are distinct. The runtime remains supervised by its responsible
execution service, which may be independent of management. The output
binding already orders `migrate` after `establish`; the explicit prerequisite illustrates ordering
without a value edge. These export names are fictional.

**Accepted prerequisite placement:** declare explicit success prerequisites within the dependent
step beside its operation and inputs, selecting predecessor steps by their local names. A separate
graph-wide authored edge collection is not used. The example's placement is accepted; `after`
remains provisional vocabulary. Output references contribute their dependencies automatically;
the combined graph must still pass cycle and missing-step checks.

**Accepted v0 prerequisite rule:** an explicit ordering prerequisite that passes no data
requires successful completion under the predecessor's operation contract. A failed or uncertain
migration keeps verification blocked; mere completion or a schema-valid failure result does not
satisfy the gate. Dependencies on declared output publication, independent work and required
lifecycle cleanup retain their existing rules. Final field names remain open; additional authored
failure/completion triggers are deferred in v0 as specified below.

If an input needs a calculated value, add a typed plugin operation and bind its output instead
of evaluating an expression inside `inputs`. For example, a path-building operation can consume
configuration values and publish the directory for `database.establish`. Its normal operation
contract supplies input/output validation and any applicable effects/recovery semantics. This
example does not standardize a core path plugin or select additional YAML fields.

For a data-collision example, a document-storage operation may receive a JSON object containing
an example reference, such as `{"from":"output","node":"prepare","port":"path"}`. When authored
as literal document content, it is passed unchanged instead of creating a dependency on `prepare`.
If an earlier operation returns that document and the graph explicitly references its output, the
output binding creates a dependency on that operation, but the document's contents do not create
further graph dependencies. These field names are illustrative; the rule applies to the eventual
reference payload notation. Supply the document under the accepted `literal` form key.

To assemble a document, the graph can declare its `title` field as the literal `Review` and its
`body` field as a reference to `read_document`'s text output. A list can similarly declare a literal
item followed by an item from another output. This structural assembly does not concatenate strings
or merge objects, and it does not scan either field's contents for additional references.

For comparison, the following uses accepted form keys to show a whole literal list and an object
assembled from a literal and a reference. Each supplied binding has a form, while the literal list's
items remain ordinary data:

```yaml
inputs:
  requiredEntries: {literal: [Cargo.toml, src]}
  document:
    object:
      title: {literal: Review}
      body: {reference: {from: output, node: read_document, port: text}}
```

The receiving operation gets the list and assembled document, without these form keys. The example
assumes a declared `read_document` step and compatible receiving contract. The `inputs` container
and the fields inside `reference` remain proposed; the binding form encoding is accepted.

### Early and runtime validation

**Accepted validation boundary:** catch provable wiring/type errors before graph execution, then
prepare defaults and validate each step's actual resolved inputs before dispatch. V0 does not demand
a proof that every possible producer value meets the receiving schema. For example, an integer output permitting
1–100 can feed an integer input accepting 1–10: 5 passes and 50 blocks the receiving step. Do not
coerce or clamp an invalid value. Required schemas and validation features must be supported;
the absence of a static inclusion proof is not permission to omit actual validation. Earlier
effects/evidence are preserved, and rejection does not imply graph rollback. Concrete checking
algorithms and diagnostic coverage remain open.

**Accepted operation-input preparation:** Conduit materializes unambiguous applicable declared
defaults for genuinely omitted fields, then validates the prepared input payload before invoking
Conduit-authored code or graph operations. Both implementations receive those values; graph input
references select within that prepared payload. Child invocations follow the same rule under their
own input contracts. Invalid explicit values, explicit nulls and failed reference selections are
not replaced by defaults; null must be permitted by the schema. Conflicting applicable defaults
require explicit resolution, and invalid prepared inputs prevent invocation. Earlier effects remain
recorded. Detailed default applicability remains open; provider-native MCP schema semantics are
preserved without automatically applying Conduit input-default rules.

**Accepted output boundary:** validate each declared plugin output against its producer's contract
before making it available to downstream steps. Withhold invalid output and report a contract error,
preserving effects and uncertainty under the operation's recovery contract. A malformed directory
descriptor does not prove that directory creation failed or had no effect. No rollback or automatic
safe replay follows. A consumer's broader input schema cannot excuse a producer-contract violation;
producer validation and receiving-input validation are independent checks. A declared failure result
can be schema-valid without establishing domain success or readiness. Provider-native schemas retain
their own interpretation; this does not impose a new output schema on an external MCP tool.

The following representation/checking details remain proposals except where accepted above or
under the existing graph/authority rules:

- Unique node names; finite acyclic dependency graph; every output/input path exists in a contract.
- Dependency edges come from output bindings plus `after`, not mapping order.
- Early checking rejects known structural/type errors without requiring a general JSON Schema
  inclusion prover. Report the limits of what was checked; runtime input validation remains required.
- The five source categories above are accepted; source names and exact selectors remain proposals.
  Source selection and nested JSON Pointer use direct fields inside `reference` as accepted above.
  Single-key forms for every supplied graph
  input binding are accepted; bare literals with collision escapes are not the selected v0 approach.
- Runtime-input and control-handler bindings use the same typed machinery. Setup-only operations
  are not automatically exposed as public capability tools.
- Input acquisition never grants access. A Resource dependency requiring a Conduit must resolve
  through one, even if both instances happen to share a worker.

### Scheduling, outputs and lifetime

**Accepted configuration capture:** configuration references use values captured at the beginning
of that graph execution, rather than rereading settings separately for later steps. A later execution
captures then-current settings. These are execution inputs, not another mutable instance configuration
map or proof of current runtime state. Authority, dependency validity and revision checks when applying
controls remain current. Retry/recovery must revalidate retained results; capture does not make stale
progress reusable. Exact capture/storage representation and recovery mechanics remain proposed.

The scheduler dispatches dependency-ready nodes within global, per-instance and plugin limits.
**Accepted failure policy:** a failed setup/change step does not stop scheduling independent branches
whose own prerequisites remain satisfied. Work with unmet prerequisites stays blocked, and required
unfinished setup is still incomplete. Current authority, readiness, target coordination, cancellation
and mandatory stop rules remain effective. Preserve dispatched outcomes/effects; do not automatically
retry failed work or infer rollback. Authors express required ordering/gates through dependencies.

For example, a graph can prepare a workspace independently of logging into a service. A login failure
blocks service work that needs that login, while the eligible workspace branch continues. This concerns
the graph's already-requested work, not automatic provisioning of unselected Pool entries or Agent turns.

**Accepted cancellation policy:** an authorized explicit graph cancellation stops new ordinary
steps and requests cancellation of in-flight child operations where supported by their contracts.
Keep tracking actual outcomes/effects and performing required cleanup, including preparation's
attachment-release duties. Report unsupported or unconfirmed cancellation without inferring that
effects stopped or were undone. Instance-owned runtimes retain their lifecycle contracts; cancelling
a graph does not itself retire them. Exact propagation and transport mechanics remain proposed.

Plugins handle internal branching/polling; the graph language has no authored loops, conditionals
or dynamic fan-out. A Pool composition operation can process a finite declared selection and
return separately tracked child outcomes without exposing an arbitrary loop language.

**Accepted setup-success default:** setup steps are required unless explicitly optional. Optional
failure remains visible without itself preventing successful setup. Missing inputs, required
downstream work and mandatory type/access requirements still govern the result; optionality
does not weaken validation or manufacture outputs. This is distinct from the accepted required/optional
connection rule and does not alter independent scheduling, output validity, retained effects or
lifecycle duties. Exact step notation and setup-result representation remain open.

**Accepted v0 trigger scope:** defer authored `on failure` and
`after any completion` triggers in v0, retaining successful-completion prerequisites and declared
output-publication dependencies. Operation/instance contracts still perform their required cleanup
and support recovery even when ordinary graph work is blocked or cancelled. Cleanup does not wait
for ordinary-step success, and this does not imply automatic rollback. Provider-specific error
handling remains inside plugins initially. Trigger spellings are illustrative, not accepted fields.

Each output declares type, publication condition and validity rules. Durable descriptors and
live handles differ. Apply producer-output validation before the output becomes available; schema
validity does not replace its publication or lifetime requirements. A ready output can refer to an
instance-owned runtime that has started
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

### Composite outputs and outcomes — proposal

**Accepted default:** derive a composed operation's initial public result contract from its
declared public output bindings and constituent contracts. Authors may explicitly construct a
domain result instead. Account for composed success/failure modes, preserve relevant effects,
uncertainty and scoped evidence, and do not automatically export internal outputs. Detailed
algorithms, mapping syntax and recovery coordination below remain proposed. Keep domain results
open-ended rather than introducing a universal enumeration of business failures.

**Accepted public-value binding forms:** reuse the accepted `literal`, `reference`,
`object` and `list` forms for each declared public output's value, with the same source/scope rules as
input bindings. A graph can publish a database reference with a literal label using object assembly
without an extra assembly operation. Declared publication conditions, validity, output validation and
permitted exposure still apply, including sensitivity/custody rules. A public value does not by itself
confirm whole-operation success, broaden scope or publish other internal outputs. Calculations and
transformations remain plugin operations. Domain-error mapping, output-contract layout and vocabulary
remain open; consumers receive resolved values rather than authoring forms.

Distinguish three concerns within the existing operation contract and evidence system:

| Concern | Meaning in a composed operation |
| --- | --- |
| Public values | The selected typed outputs a caller may use, with their publication and validity conditions. A database identity can exist before its ready-access output is valid. |
| Reported outcome | What the operation can confirm about completion, failures, retained effects and uncertainty. A useful partial output does not mean the whole requested setup completed. |
| Execution evidence | The attributable child attempts, results and causal links needed for inspection/recovery. Access-scoped references can expose detail without returning every internal payload or credential. |

**Proposed derivation mechanics:** use declared public output bindings to derive their data types; use
child operation contracts and graph dependencies to describe declared failure origins and blocked
work. Preserve the node/operation source so identical failure names from different children are
unambiguous. The runtime reports the actual completed, failed, blocked or unresolved portions of
an attempt. These are coordination facts, not a proposed closed wire enum. Independent parallel
outcomes can be reported together without statically enumerating every possible combination.
Derivation uses declared semantics, not guesses about provider behavior or automatic export of
all intermediate values. Missing behavioral information remains an explicit limitation.

**Explicit construction:** an author can select, combine and rename public outputs, or map child
failures into domain-specific results. For example, several connection failures might become
`access-unavailable`, retaining links to permitted causes. Public schemas stay type-checked against
the declared bindings. Such mapping cannot falsely claim no effects, successful readiness or safe
replay when evidence does not establish it. Preserving these facts does not require leaking hidden
source details or secrets. Deliberate partial-success contracts can return usable outputs while
clearly distinguishing unmet requested work. Exact mapping syntax remains I7 work; no general
embedded scripting language or new graph branching feature is selected here.

A derived result is still an inspectable, versioned contract tied to the selected definitions.
Changing a child contract can affect its public shape; deriving a schema does not make that change
automatically compatible. Explicit domain results can provide a more stable public boundary, but
still need validation against the selected implementation and actual outcomes.

**Limits of completeness:** child contracts can enumerate expected domain failures, but a lost
response, unrecognized provider error or violated output contract must still be representable.
Do not interpret an unlisted failure or unavailable observation as proof that nothing happened.
If migration effects cannot be confirmed, report that uncertainty; an error message alone cannot
establish the database schema. Keep known retained outputs subject to their validity rules.

Mapping a failure is not handling its effects: translating `migration-failed` into a public result
does not retry, compensate or roll back. Composed results cannot invent atomicity or strengthen a
child's retry guarantee.

**Accepted recovery default:** graph-backed operations reuse the graph runtime's valid-progress
recovery. An explicit retry of the same unchanged invocation resumes retained child progress and
reuses only still-valid results. The affected child's recovery contract handles unconfirmed external
effects; the parent resumes that composite rather than blindly invoking its whole graph again.
For database setup, retain the verified running database, reconcile the migration outcome and
proceed only as its contract permits. Authors need no separate recovery implementation merely for
composition. Current authority and validity rules still apply; a result mapping grants no retry
permission. This enables neither automatic retries nor promised rollback, guaranteed recoverability
or exactly-once effects. Detailed coordination and custom recovery hooks remain open. The
[database example](AUTHORING.md#derived-and-authored-composite-results) supplies the concrete case.

## 5. Plugin package and runtime contracts

**Accepted entry-document boundary:** one explicit package entry document declares public exports
and dependencies. Definitions may be inline or referenced in package-local files; the document may
be handwritten or generated. Tools inspect this surface without executing plugin code or discovering
exports from directory layout. Including a file in the artifact does not make its definitions public.
Provider-native tool catalogs remain dynamic under their integration contracts. The filename, field
layout, reference notation and archive format below remain proposals.

**Accepted export naming:** public export names are unique across contribution kinds within each
package, whether definitions are inline or referenced from files. Reject collisions without selecting
by kind or declaration order. A resolved export must still satisfy the caller's expected kind and
contract. Internal definition names, contract identities and registry object names remain separate;
exact name grammar and reference notation are open.

**Accepted re-exports:** a package may explicitly expose a declared dependency's public contribution
under its own export name without copying or wrapping it. The original definition, contract, internal
reference scope and resolved dependency identities remain intact. The alias adds no behavior or access;
ordinary export-name uniqueness and kind/contract checks apply. Exact declaration and resolution syntax
remain open.

**Authoring before hosting:** the manifest/port inventory below is a proposed runtime-facing
contract, not a requirement that every author manually implement every field or interface.
The [accepted direction](../../archive/reviews/IMPLEMENTATION.md#review-priority-plugin-and-template-authors) is lightweight
Conduit support, with real plugins proving the public protocol before investing in a helper
library. The division between authored and generated metadata and the concrete package/wire
formats remain open. Persistent drivers need lifecycle behavior; a finite operation need not
implement unused hooks. Small adapters or existing libraries must preserve the same observable
contracts; a comprehensive SDK is not required before the first plugins can be authored.

**Accepted publication boundary:** plugins publish inspectable contracts and implementation bindings;
Conduit can validate against them without executing plugin code. Bindings can use authored code,
an existing local/remote MCP service, a graph of selected operations, or a combination. A reusable
graph may implement an exported typed operation, including for invocation from another graph.
Resolved package artifacts retain package identity and a content digest covering declarations and
any bundled code, including packages with no executable. Deliberate preservation requires a versioned
release with fixed contents, retained independently of running instances; changed contents need a
new release version. Release means intentional publication through a defined public or private
distribution source; it does not require Conduit-owned hosting or publishing. Distribution mechanics
remain open, and a separate local-only release facility is not selected. Edit-mode captures record
the selected contents without becoming preserved releases, even if a draft contains a version field. Release metadata,
commands, digest and bundle formats remain open; separately bound dependencies keep their own identities.
Managed execution of locally authored packages uses an immutable snapshot retained before
execution. Direct handler tests can use editable source; an explicit managed test/start can
capture selected contents automatically. Source edits cannot change a running instance's code or
contracts. Preserving that execution does not require permanent historical artifacts or promise
later recreation of the old implementation. Development can retire the affected instance and
replace it from revised source, with explicit backing-data reuse where supported. Old executable
contents need not be kept solely because evidence records their identity; live activity/recovery
dependencies and separate evidence-retention rules still apply. Missing selected code is reported
without substitution. No selectable multi-build catalog is required for this workflow.
Packages explicitly select included files/directories, with directory and
pattern rules available rather than default capture of the whole working directory. Unselected
files are outside the artifact and its digest. One explicit source directory bounds included
paths and symlink targets; report outside targets rather than silently including them. Authors
can stage files, select a wider explicit directory or use a separate package dependency. This
does not restrict Resource paths or authorized runtime access. Exact inclusion syntax, descriptor
handling, internal symlink representation, storage layout and cleanup remain open; this is not
a snapshot of credentials, Resource data or the host environment.
Registering schemas is not permission to run the package. The proposed manifest/schema bundle
below illustrates that boundary; detailed syntax and package layout remain I7 work.

**Accepted contract authority:** the published, language-neutral contract defines the public API
and behavioral guarantees; implementations conform to it. Authors may handwrite contracts or
generate them from code or graph composition, then explicitly review and version the released
definitions. Internal Rust types do not independently override the contract. Data schemas alone
do not establish effects, authority, recovery or lifecycle guarantees. The subsequent foundation
ruling selects JSON Schema 2020-12 for data contracts without requiring an SDK/generation tool;
provider-native catalogs retain their agreed
dynamic boundary behind the integration contract.

**Accepted distribution scope:** bundled first-party plugins and explicit installation of locally
supplied packages, including code-optional contributions. A marketplace and automatic dependency
resolution across languages are deferred. Installation does not instantiate objects, authenticate
providers or grant access. Package syntax, installation mechanics and generation tooling remain
proposals; exact implementation retention and replacement rules still apply.

| Manifest concern | Required information when applicable |
| --- | --- |
| Package/bindings | Package identity and content digest, release identity when released, contract identity, implementation or service dependencies and compatible profile; executable entry and platform where code is supplied. The package digest covers declarations as well as bundled code; code-free packages also retain it. |
| Contracts | Provided type/capability/filter versions and exact schema bundle identity. Conflicting definitions of the same contract cannot overwrite one another. |
| Operations | Export name, input/output schemas, effect/recovery information, cancellation/interaction support and output validity. Graph-backed exports use the same operation contract; graph-call/value-binding and recovery details remain open. |
| Drivers | Managed runtime exports, prepared inputs, inspect/start/stop/reconnect support and enforcement coverage. |
| Controls | Input schema, supported handler/graph, affected config paths, constraints, coordination/revision requirements and confirmation semantics. |
| Host needs | Explicit storage, credential, process, network and other broker facilities used by the implementation. Declaring a need does not grant it. |
| Compatibility | Provider/native-runtime requirements and known version boundaries, separately from package identity. |

Some recovery behavior is declarative, some is implemented by a handler. An operation may be
non-repeatable, provider-idempotent or inspectable without being generally replay-safe. Do not
infer semantics from a tool name, HTTP method or an advisory MCP annotation alone.

### Composable relationship requirements — proposed contract

The [accepted design](../../protocol/CONDUITS.md#composable-relationship-requirements) uses Conduit templates
composed from reusable typed requirements, with a small base vocabulary and author-defined additions.
**Accepted extension boundary:** each requirement defines reusable, versioned meaning, typed
parameters and applicability independently of a particular implementation. Multiple endpoint plugins
can implement it; definitions and implementations may ship together without separate packages or
repeated schemas. Requirements are contract material within existing extension machinery, not a
sixth object family, separate deployment or runtime instance per condition.

**Accepted support boundary:** endpoint plugins publish supported requirement versions, endpoint
roles and known limitations in their contract bundles. Conduit checks declared compatibility before
establishment; current authentication, readiness and enforcement still require applicable runtime
verification. Shared definitions supply schemas without copying them into each implementation.
This does not certify the implementation or prove a running connection fulfills the requirement.

**Accepted fulfillment boundary:** requirement contracts declare when guarantees must hold and
applicable endpoint-role duties. Established Conduits record which selected implementations fulfill
them, inspectable through the Interface within existing connection/evidence state. This does not
impose a fixed universal phase list or another authored manifest. Exact obligation syntax, evidence
and evaluation mechanisms remain open.

**Accepted composition:** requirement definitions may compose existing requirements without new
plugin code when endpoints support every constituent. Preserve the original meanings and limits;
check constituent support rather than treating a bundle's name as proof or a new grant.

**Accepted Policy consolidation:** [Policy names authoritative requirement application](AUTHORIZATION.md#3-policy-as-authoritative-requirement-application).
Grant administration and governing-binding changes are ordinary protected capability operations,
with recorded grants permitting them and requirements constraining their effects. One plugin can
implement these uses without separate deployment or duplicated schemas. Concrete-request enforcement
and checking proposed grant bounds need their own declared support; shared definitions do not
imply a universal subset checker. Governing bindings remain independent managed state with checked
changes, without a separate policy-rule language or executable definition category.

**Accepted bounded issuance:** a typed issuance operation may supply mandatory use restrictions
and compose additional requested restrictions, returning the complete actual grant. Caller intent
remains explicit; omissions do not expand to maximum administrable access, and explicit broadening
is refused. Strict complete-proposal operations may coexist. Issuance-only checks do not become
ongoing grant dependencies; current governing requirements still apply. Exact schemas and custom
validation support remain authoring/conformance work, without a universal subset-proof requirement.

**Accepted governing scope:** bindings explicitly distinguish an object's own operations from
operations on owned Resources selected by the binding. Pool ownership and declaration placement
do not automatically propagate requirements. The selected capability/context must be supported;
one binding may explicitly cover multiple owned Resources without becoming a permission grant.
Explicit ownership-based governing selections match live at subsequent applicable checks as Resource
instances enter/leave the selected set. They neither provision Resources nor issue grants, and
explicit references retain their resolved stable IDs. Exact selectors and evaluation mechanisms
remain under review; live selection does not provide authority when current facts are unverifiable.

Detailed declaration syntax and matching, the core catalog, composition mechanics, evaluation
protocol and enforcement policies below remain proposed. A Conduit template composes requirements
for roles and their relationship. Concrete instantiation binds existing endpoint capabilities and records resolved
requirements in its retained definition. Plugins do not enumerate user-created Conduit template
names. Matching a name, JSON shape or similarly named field is not sufficient evidence of support.

| Requirement definition concern | Accepted boundary or proposed detail |
| --- | --- |
| Identity and meaning | Accepted: a reusable, versioned behavioral contract independent of any single implementation. Identifier syntax stays open. |
| Typed parameters and subjects | Accepted: the definition owns typed parameters and applicability. Proposed detail: distinguish endpoint-role, relationship, invocation and result subjects. |
| Implementation support | Accepted: publish supported requirement versions, endpoint roles and known limitations; check before establishment, then verify applicable runtime facts. Exact declaration/matching syntax remains open. |
| Satisfaction | Proposed detail: describe evidence/checks, preconditions and guarantees sufficient for a particular requirement. |
| Runtime responsibility | Accepted: definitions declare when guarantees must hold and applicable endpoint-role duties; established Conduits record responsible selected implementations. Detailed evidence/evaluation interfaces remain open. |
| Composition | Accepted: definitions compose existing requirements with individual enforcement; known-conflict or combined-effect checks are optional. Detailed diagnostics, optimizations and parameter bindings remain open. |
| Lifecycle | Describe when evidence must be refreshed and what unavailable support means for the affected behavior. No universal closed domain-state enum is selected. |

These are conceptual responsibilities, not mandatory fields for every tiny extension. A derived
requirement can compose existing ones without new code. A genuinely new semantic requirement needs
a definition plus an implementation that can satisfy/check/enforce it where needed. Pure validation
must remain distinct from effects: registration does not run a login, create a database or install
an endpoint plugin. Existing authorized setup graphs may establish prerequisites explicitly.

**Starter candidates, not an approved catalog:** compatible endpoint capabilities/protocol features;
permitted operations; provider-aware target constraints; placement compatibility for local views;
result-size/count bounds; and applicable identity/binding lifecycle requirements. Baseline authority,
identity, outcome retention and participation rules are not optional knobs that authors can omit.
Domain-specific examples belong in extension packages, not an ever-growing hard-coded core enum.

**Accepted composition checking:** retain and individually enforce every constituent at its
declared point. Contract-specific checks for known conflicts or explanations of combined effects
are optional; custom requirement types need no general solver or merge algorithm. Limits of 20
and 10 can remain separate obligations, with a supported comparison explaining an effective limit
of 10. Report known contradictions and empty permitted sets without claiming unproved global
compatibility. An unverifiable required check still blocks the affected behavior. Preserve complete
operation/target constraints; separate authorized routes remain alternatives. No last-write-wins or
automatic dropping of requirements follows. Exact diagnostic and optimization-equivalence contracts
remain open; no alternative/advisory expression syntax is selected.

**Validation and operation:**

- Parse typed parameters and resolve known requirement definitions without executing endpoint code.
  A missing required schema still prevents the registry write under the existing rule.
- Match selected endpoint declarations and configured parameters. This yields support or useful
  diagnostics, not evidence of a live authenticated connection. Missing runtime support can leave
  otherwise valid registered configuration unready; do not install/substitute code automatically.
- Establish authorized bindings through the existing graphs/endpoint implementations. Record which
  configured requirements have support and which need runtime evidence; no new user-authored plan
  manifest or object is needed.
- Recheck changing facts at their defined point of use and enforce relevant request/result
  restrictions. A startup check cannot replace current authorization. A check result must not be
  treated as permanently valid merely because it was cached.
- Preserve plugin-specific evidence and partial outcomes under existing instance authorities.
  Unknown, unsupported, contradicted and currently unavailable are distinguishable facts, not
  interchangeable successful validation. Exact wire representation remains open.

**Change boundary:** supported controls may change declared requirement parameters within their
fixed contracts. Adding/replacing requirement semantics or selected implementation definitions is
not a generic live mutation; follow v0 replacement rules unless an existing supported control
already covers the action. Current governing access rules and facts still change live. Pin the
resolved requirement meanings as part of the existing retained instance definition, without a new
mandatory template-history archive. Requirement satisfaction never creates a permission grant.

The [authoring example](AUTHORING.md#composable-conduit-contracts) and
[configuration research](../../research/CONFIGURATION.md#composable-requirements-research-follow-up)
explain the proposed extension path and the limits of schema/expression/policy engines. The accepted
model does not select CUE, CEL, Cedar, a universal endpoint API or a worker arrangement.

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

Locally authored plugin code normally executes in supervised instance-associated helpers; trusted
built-in Rust code may remain in-process. The accepted
[instance and endpoint behavior boundary](AUTHORING.md#5-where-the-example-implementations-execute)
uses endpoint-selected plugins managed by their instances. Conduits match compatible capabilities
and coordinate participation; they do not supply new endpoint implementation code. Exact endpoint
connection contracts and establishment/cleanup procedures remain under review. The caller, owning activity,
target and execution location must remain attributable. Detailed worker mechanisms below remain
proposals; no process per plugin, Conduit or invocation is required. Existing MCP services retain
their supervision; a graph-backed composition or declarative binding alone needs no new helper.
Process separation contains ordinary helper crashes but does not supply an OS sandbox.

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
or explicit isolation boundary. This grouping remains a proposal, not a worker requirement for
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
and control constraints. Leave values whose application cannot be confirmed unchanged in storage
and record uncertainty. The Interface must not label those last-confirmed values as known current.

**Accepted partial confirmation:** a control may explicitly support confirming and recording only
a subset of the requested changes, provided the complete resulting stored configuration is valid.
Other effects retain their reported failure, pending work or uncertainty; neither the control as a
whole nor remaining old settings become confirmed current merely because one part succeeded.
Coupled settings can require whole-change confirmation. This does not waive allowed paths,
revision checks, authority or coordination, infer independence between arbitrary fields, or promise
rollback. Preserve effect evidence if it cannot produce a valid configuration under these rules.

**Accepted unchanged-control revision rule:** retain the configuration revision
when a control confirms unchanged stored settings, while recording its invocation and any external
effects. Declared checks/effects still apply; comparison is not permission to skip the handler or
return an earlier result as a new execution. This extends the unchanged registry-write behavior
to controls. Configuration revisions do not establish that runtime state, permissions or observations
are unchanged; operation retry/result retrieval retains its own contracts.

A waiting change retains its coordination semantics. If safe parallel control domains are added,
they need declared disjoint read/write dependencies and transaction checks; merely using different
field names does not prove provider-level independence. Start with serial controls and expected
whole-configuration revisions.

Reject a predictable invalid change before effects. If a buggy provider/handler returns an invalid
confirmation after an effect may have occurred, preserve the evidence and report unresolved or
out-of-contract state; do not turn result-validation failure into a claim that nothing changed.
Reconciliation must establish reality before another conflicting change can proceed.

## 8. Content, handles and secret custody

**Accepted custody extension:** publish a minimal typed credential-custody contract through the
existing plugin-bundle mechanism, with a bundled OS-store adapter as the v0 default and macOS
Keychain as the first platform implementation. Keep later password-manager/credential-service
implementations possible without choosing their APIs or adding them to the first experiment.
Published contracts, implementation bindings and retained-version/replacement rules apply as usual.
Bundle syntax and exact custody methods/schemas remain I7 work.

Contracts must describe the behavior an implementation actually supports, including its protected
reference handling and interaction/recovery limitations. Do not presume all stores support the same
write/delete, unattended access or renewal operations. Native authentication and an external secret
Resource can retain their own custody. The owning authentication Resource's identity/refresh contract
remains distinct from its storage adapter; installing a bundle or invoking a shared service grants
no general authority to reveal its upstream secrets.

**Accepted interaction boundary:** the Interface guides the selected adapter's supported
enrollment/unlock flow. Reuse existing authorized access; required human action becomes a reported
interaction on the affected operation. The user completes the native/provider flow, and the
instance retains progress for recovery as supported. Unattended execution cannot complete that
step until access is available. Reuse existing interaction/recovery contracts rather than adding
a separate authentication workflow protocol; keep passwords/tokens out of ordinary conversations
and logs. Exact interaction schemas remain I7 work.

**Accepted managed-reference boundary:** ordinary typed values can carry inspectable data such as
a prepared storage descriptor. A live database or protected authentication binding instead needs
ownership, resolution and validity semantics. These Conduit-declared references share a typed
contract, with the owning implementation retaining actual live objects and credentials. Capability
contracts define their particular behavior and lifetime guarantees; common reference handling does
not make different capabilities interchangeable. Possession is not a grant, and past publication
does not establish current availability. Resolution and use still check applicable current authority
and validity.

Target identity, responsible endpoint and generation are candidate reference details, not accepted
fields. Exact schemas, transport and resolution remain open; the shared contract does not require
a central broker, another object family or a new graph binding form. These are operation values,
distinct from the graph's `reference` expression that selects such a value. Native provider MCP
schemas retain their own semantics. Inspection/recording must follow the value's sensitivity rules;
a reference is not automatically safe to log, and its stored form cannot establish that an old
process still exists.

Secret inputs are explicit references or interaction channels whose custody remains with their
owning authentication capability. The type/operation schema identifies sensitive fields and
recording needs. The host must not blindly serialize an input structure using debug formatting.
If a required recovery input cannot be retained safely, record the recovery limitation and ask
for supported re-establishment rather than claiming transparent replay.

Content records retain their Resource ownership, revisions, derivation/access dependencies and
permitted provenance. Event views can refer to them without copying full private content into
every observation stream. Full tool/model payloads require declared recording policy under the
accepted I6 capture rules. Exported
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

**Accepted observation extension:** plugin bundles may publish typed observation capabilities,
with compatible implementations for producing, collecting or exporting instance-relevant data.
Use authorized Conduits and the existing MCP inspection/observation mapping; no sixth family or
separate mandatory telemetry protocol is introduced. Independent collector/storage/export services
are Pool-owned Resources. Begin with a lightweight bundled foundation rather than a required
catalog of token, CPU or other metrics.

Contracts describe observation meaning, source/correlation, freshness/coverage and supported
delivery. Metric-specific payloads remain extensible, including measurement scope and attribution
limits. Exact common fields and package syntax are I7 work. Optional observations or exporter
receipts cannot substitute for authoritative operation outcomes and recovery records. Supported
observation controls, connection changes and fixed-implementation upgrades use existing rules.
I6 accepts correlated operation traces, structured diagnostics and a small health metric set, with
optional OpenTelemetry export. The synthetic experiment enables all instrumented traces at source
and explicit bounded fixture-input/output capture. Real full-content capture follows declared
recording policy, with bounds, sensitive-field handling and scoped references as applicable.
Report diagnostic loss and incomplete coverage; required authoritative evidence is not sampled.
This selects initial behavior, not exact wire fields, metric names or a mandatory external backend.

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
