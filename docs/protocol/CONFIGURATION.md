# Configuration and identity

Scope: accepted working contracts. Explicitly marked v0/reference-runtime selections retain
their accepted status without becoming requirements for every capability implementation.
See the [reference boundary](README.md#protocol-and-reference-implementation) and the
[current review queue](../REVIEW.md) for unresolved questions; historical review links below record prior rulings.

Part of the accepted [design contracts](README.md). Updated: 2026-11-03.
The [conceptual review](../archive/reviews/CONCEPTUAL.md) records completed rulings and deferred work;
syntax and implementation examples do not amend these rules.

## Configuration language

**Constrained YAML 1.2 is the human authoring format; JSON is an equivalent input and
interchange representation.** Both use one JSON-compatible value model and the same typed
interpretation. JSON Schema 2020-12 is the accepted basis for configuration, operation
input/result and requirement-parameter data contracts. Schemas themselves may be authored
as YAML mappings or JSON; authors need not maintain duplicate representations.

Conduit's explicit behavioral contracts define graph execution, default materialization,
controls, authorization, effects and recovery. Data-schema validation does not establish
those guarantees. The detailed schema profile, reference syntax, validators and
canonical byte encoding remain to be specified; this ruling does not accept every proposed
schema feature or introduce another mandatory configuration language.

The five families and their template/instance forms share identity, reference, composition
and behavioral contracts across every entry point.
CLI/API/frontends must interpret the same objects consistently. Future generators can
emit this model; they cannot bypass its validation or authority rules.

Each object uses its family as `kind` and a top-level `form` beside it to identify a template
or instance. Form is a structural distinction, not an observed lifecycle state: instantiation
creates a separate object rather than changing the template's form. Family and form jointly
determine how to interpret `spec`. The common authoring envelope is:

| Field | Meaning |
| --- | --- |
| `apiVersion` | The common object contract version. A Resource's selected type contract has its own separate schema version. |
| `kind` | The family: `Interface`, `Agent`, `Resource`, `Pool` or `Conduit`. The explicit form further determines the applicable schema and operations. |
| `form` | The template/instance distinction, at the top level alongside `kind`. |
| `metadata` | Author-editable naming and system-owned identity/version information, with ownership defined per field. No generic object-label map in v0. |
| `spec` | Configuration defined by the family and form's contract. |

Metadata ownership is defined per field: authors control naming within their
permissions, while Conduit assigns and maintains identity/version information. Placing a
field in `metadata` does not make it editable. Identity and configuration-version checks
govern updates as described below; exact field names and operation schemas remain open.

Inspection returns the object with a system-owned `status` section alongside its authored
`spec`. Status reports resolution and observed state appropriate to the kind, such as an
Agent's bindings, preparation and execution. Normal authored YAML omits `status`; users
and agents request changes through configuration or operations rather than declaring an
observed state to be true.

Normal configuration writes reject an included `status` field rather than silently
ignoring it. Configuration-only export contains reusable, authorable object configuration,
omitting `status` and system-owned identity/version values. Inspection can still return
those system values. Update preconditions are separate operation inputs; they do not
authorize clients to assign system-owned values through object YAML.

**Instance export:** export the current stored `spec.config` with the retained fixed
definitions and explicit dependencies. Report outstanding requests/uncertain observations alongside
the export, without treating them as authored values. The configuration is an input for fresh creation,
not proof of live state. Do not copy credentials, grants, readiness, live handles or execution progress.
Account/Resource references remain explicit and resolve under current authority on fresh use.

For example, an instance whose query limit was confirmed as 15 exports 15; the unchanged source
template can still export 10. Fresh creation from the instance export uses a new identity and unique
name, and Resource creation still goes through an owning Pool. Backing create/attach intent remains
explicit; report creation-only settings that need editing rather than silently changing intent or
copying an established binding. The export does not resume the source instance's execution.

The shared envelope does not give every kind the same configuration or status fields.
Exact metadata/status fields, ID encoding and remaining import/export semantics are open.
Under the [vocabulary boundary](MODEL.md#vocabulary-review-boundary), the envelope's current field
spellings are also working names. Its accepted roles, ownership and structure remain in force.

This accepts the envelope and organizing principle, not every proposed field or algorithm
in the research. In particular, the API group/version, registry storage layout, reference
extensions, schema toolchain and command names remain to be drafted. The v0 YAML interpretation
profile is accepted below. Configuration objects also do not force every memory record, native tool or runtime event into an
individually authored YAML file.

### YAML interpretation

**V0 profile:** use YAML 1.2 Core scalar interpretation, restricted to string
mapping keys and finite supported numeric values. Typed schemas enforce field types, ranges and
units without silently converting a string to a number or boolean. Reject duplicate mapping keys
and unknown fields outside schema-declared open maps. Numeric values follow the shared integer and
fractional rules below: integer precision loss and unsupported ranges are errors; ordinary fractional
values permit the declared binary64 rounding. Equivalent JSON/YAML inputs have the same typed meaning.
The JSON input path must likewise reject duplicate keys and numeric violations before parsing
loses the evidence needed to check them. Presentation differences do not create different
semantics; source comments and formatting need not survive configuration export.

Omission follows the declared default/optional rule; instances materialize declared defaults.
Null is an explicit value accepted only where the field permits it, including a blank YAML scalar;
it does not universally mean reset or use-default. Empty lists/maps remain explicitly empty and
follow their field's contract rather than inheriting values. For example, integer `queryLimit: 15`
can be valid, `queryLimit: "15"` fails that integer contract, and `enabled: yes` fails a boolean one.

Comments, ordinary mappings/sequences and multiline strings remain supported. V0 rejects
anchors/aliases, merge-key behavior and custom tags; templates, typed references and declared
graphs provide reuse. Loading configuration does not implicitly expand shell/environment strings.
An explicit typed input binding or plugin operation can supply a value or perform its declared
work; script-based plugins remain supported. This profile governs Conduit's configuration loader,
not the contents/formats of opaque native files supplied through Resources.

Parser choice, diagnostics and schema-specific numeric/unit limits remain implementation work.
Multiple-object submissions use the batch contract below; parsing YAML does not imply atomic
deployment, authority or successful setup.

### Numeric interoperability

V0 Conduit-declared integer configuration, inputs and outputs use the inclusive range
−9,007,199,254,740,991 through 9,007,199,254,740,991. Individual schemas may impose narrower bounds.
This shared range supports exact exchange with ordinary JavaScript clients as well as Rust;
it is not permission to round an out-of-range value into an accepted one. Detect violations before
parsing or serialization loses the original value.

For example, an integer history counter of `9007199254740991` is within the shared range;
`9007199254740992` is outside it. A capability needing a larger exact counter can explicitly
declare a string with decimal-digit constraints and define its meaning. Conduit does not
automatically convert numeric inputs into that string contract.

Ordinary fractional configuration, inputs and outputs use finite IEEE 754 binary64
(double-precision) semantics in v0. For example, temperature `0.7` uses its normal floating-point
approximation. A contract requiring exact decimal quantities uses an explicit decimal string or
scaled integer, such as a duration in integer milliseconds, with its units and meaning declared.
Conduit does not silently convert between these contracts.

This explicitly permits normal fractional representational rounding, refining the earlier blanket
precision rule. Integer exactness and limits remain strict; the choice does not authorize rounding
to bypass an integer contract or silently replacing an out-of-range value with zero or infinity.
Core revision/ID encodings and storage types remain separate. Provider-native schemas retain their
own interpretation. Detailed comparison, `multipleOf` and round-trip conformance cases still need
specification; choosing binary64 does not itself validate an implementation's constraint checks.

### Schema resolution

Validation resolves schema references only from the selected package and its explicitly supplied
or declared schema dependencies. Required definitions must be available; report a missing
definition rather than fetching it automatically from the network or treating it as unconstrained.
Packages and schema dependencies may be acquired explicitly before validation. A network-shaped
schema identifier is not permission to retrieve that address during validation.

This rule concerns definitions used for validation. It does not establish Resource access,
authenticate an external service or grant a runtime capability. Package-local scope and declared
dependency boundaries still apply. Exact reference spelling, artifact layout and resolver mechanics
remain authoring and implementation work.

### Schema keyword acceptance

When accepting Conduit-authored data schemas, reject unrecognized schema keywords rather than
silently accepting them as annotations. For example, `minimun: 1` must report an error instead
of losing the intended `minimum` constraint. This checks schema-keyword positions; names inside
declared maps such as `properties` and `$defs` remain governed by those maps' contracts.

Use standard descriptive schema annotations such as `title`, `description` and `examples` where
they fit. Additional custom descriptive metadata uses one dedicated annotations map with namespaced
entries. For example, a frontend can supply a slider/grouping hint for a query-limit field, while a
documentation tool supplies a section hint under its own namespace. A frontend may use an ordinary
numeric input when it does not support the slider hint; the field's declared validation still applies.

Custom annotations do not implement validation, authorization or runtime behavior. A limit of 1–100
belongs in the input schema; permission to change it belongs in grants/governing requirements; how
the change is applied belongs in the control/handler contract. Unknown schema keywords outside the
dedicated extension mechanism remain errors. The map keyword, namespace syntax and detailed value
schemas remain open; illustrative `annotations` and `example.ui` spellings are not selected syntax.
External provider MCP schemas follow the separate boundary below.

In Conduit-authored data schemas, `format` is an enforced validation requirement. V0 selects all 19
standard formats defined in [JSON Schema 2020-12 section 7.3](https://json-schema.org/draft/2020-12/json-schema-validation#section-7.3),
using their standard meanings and offline syntactic checks:

| Area | Formats |
| --- | --- |
| Time | `date-time`, `date`, `time`, `duration` |
| Email and hostnames | `email`, `idn-email`, `hostname`, `idn-hostname` |
| IP addresses | `ipv4`, `ipv6` |
| Identifiers/references | `uri`, `uri-reference`, `iri`, `iri-reference`, `uuid` |
| Templates, pointers and expressions | `uri-template`, `json-pointer`, `relative-json-pointer`, `regex` |

Conformance tests must establish support before an implementation claims it. Report unsupported
formats rather than treating their constraints as satisfied. V0 defers plugin-defined format
validators so authored-schema validation can run before plugin code starts.

For example, `date-time` validates timestamp syntax and `uri` validates a URI without contacting
its endpoint. These checks do not establish reachability or existence. Supporting `uuid` or
`json-pointer` does not select Conduit's object-ID encoding or graph-reference notation. A project-specific
identifier can use ordinary string constraints and a reusable schema instead of registering `format: workspace-id`.
Custom operations, Resource contracts and requirement checks remain extensible. Format validation
does not replace explicit field types or establish runtime readiness, access authority or successful
effects. Provider MCP schemas retain the separate interpretation below; any future custom-format
extension needs an explicit contract.

### Provider MCP schemas

Existing provider MCP schemas retain their declared dialect's semantics. Do not automatically
impose Conduit's stricter authoring profile on them. An unfamiliar descriptive annotation allowed
by the provider's dialect does not alone reject a tool. Report unsupported required validation
features without silently removing constraints or claiming they were enforced.

This preserves provider-native schema interpretation while retaining the applicable compatibility,
authorization and enforcement checks. Exact adapter handling and conformance cases remain open;
preserving a schema does not by itself demonstrate support for its required semantics.

### Schema default resolution

When an omitted configuration field has conflicting applicable defaults from composed schemas,
report the conflict instead of choosing a default by schema order. Resolution requires an
explicit valid value or a correction to the conflicting defaults. For example, defaults of 10
and 20 for `limit` do not select a winner; an explicit value of 15 can proceed if it satisfies
all applicable constraints. Supplying a value does not override any validation constraint.

Conduit also prepares declared defaults for omitted inputs to Conduit-authored operations before
validating and invoking either code or a graph implementation. A reusable graph's input source sees
the prepared payload. The same no-order-based-conflict rule applies; explicit values and failed
references do not receive fallback defaults. See [operation input preparation](LIFECYCLE.md#lifecycle-protocol).
Provider-native MCP inputs retain their separate schema semantics.

The accepted omission, null and empty-value rules remain unchanged. Exact default-resolution
mechanics, including how applicability is determined, remain authoring work; this ruling does
not add ordered precedence between schema defaults.

### Implementation selection

**V0 rule:** select Conduit plugin implementations explicitly or through an
unambiguous configured default. Resolve the exact selected contents before execution, including
release identity when using a released package. Retain that binding for the instance's starts,
operations and recovery. Installing a newer package,
changing a default or package discovery order must not silently change that binding. Record an
unresolved requested dependency as unresolved, not as located or verified code.

A resolved package artifact records its package identity and content digest, plus its versioned
release identity when released. Coverage includes its contracts, graphs, templates and any bundled
code; packages containing only templates or implementation bindings follow the same rule.
Separately bound dependencies retain their own selected identities.

The digest identifies selected content, not publisher trust or externally operated MCP service
code. The hash algorithm, exact bundle boundaries, encoding and storage mechanics remain open.
This makes package-content identity explicit without adding a backward-compatibility promise.

Local authoring distinguishes **edit mode** from a **versioned release**:

- In edit mode, authors change source and explicitly create replacement instances from the selected
  current contents. No release is needed for each edit, and earlier edits are not preserved for
  future creation merely because an instance used them.
- To preserve an implementation for later selection, authors explicitly version and publish it.
  A release is an intentional, versioned publication of fixed package contents, identified by a
  content digest and retrievable from a defined distribution source. That source may be public or
  private. Editing source does not change the release; publishing changed contents requires a new
  version. A release is independent of the instances that use it.

A working-directory capture or a version string in a draft is not itself a release. Source control
can support authoring and publication, but a source commit alone is not publication of a usable
package. Declaration-only packages may be fully represented by their released source; authored code
also needs the executable artifact or an explicitly supported build arrangement its binding requires.
No particular build arrangement is selected by this boundary.

Conduit resolves selected packages, verifies their identity and executes them under their contracts.
It need not operate the publisher, hosting service or release store; existing distribution mechanisms
may supply those roles. Exact publication/acquisition commands, metadata, storage and removal
mechanics remain open. A separate local-only release facility is not selected. These are
authoring/distribution distinctions, not permission to mutate running instance definitions.

For managed execution of a locally authored package, preserve its selected code and contracts
before execution so source edits cannot change a running instance. This preservation serves the
instance's execution; it does not require a permanent package archive or make the captured contents
available for creating other instances. Recreating an old implementation from edited source or an
exported instance configuration is not guaranteed. Preserved versioned releases provide the explicit
route to selecting old implementations again; Git or any particular storage mechanism is not
required by the protocol.

Ordinary development can retire the affected instance and create a replacement from revised source,
explicitly reusing backing data where supported. Running both revisions is optional. Capture may be
handled automatically by an explicit development-test or start workflow; authors need not assemble
archives, publish releases or manage a catalog of simultaneous builds for each iteration.

Retirement can release an edit-mode instance's private executable contents once no live activity or
unfinished recovery requires them. It does not remove an independently preserved versioned release.
Retained instance records or outcome evidence alone do not require an executable archive;
their own retention rules still apply. Restart or recovery must use the recorded implementation
when available and report unavailability otherwise, never silently adopt revised code. Exact
cleanup mechanics remain open. A runtime may share or cache captured contents without exposing
edit-mode build history as selectable releases.

Direct handler tests may run from editable source. Managed integration tests use the captured
artifact and the ordinary instance contracts. This does not permit live replacement of an
existing instance's fixed definitions. Captured contents exclude credentials and Resource data
and do not freeze the host environment; separately supplied runtimes/dependencies retain their
declared boundaries. Storage layout and cleanup remain open.

Packages explicitly declare which files and directories belong in their captured artifact.
Directory and pattern rules can select groups of files without listing each one individually;
capture does not include the entire working directory by default. Unselected working files
are outside the package artifact and its content digest. Exact inclusion syntax, descriptor
handling and internal symlink representation remain open.

Each capture uses one explicit package source directory. Included paths and symlink targets
must resolve within that directory; report an outside target rather than silently including
it. Authors can stage shared files inside, choose a wider explicit source directory with
inclusion rules, or use a separate package dependency. This boundary applies to captured
package files, not Resource paths or authorized runtime filesystem access.

Contract identity/version defines promised configuration and behavior; implementation bindings
identify the declarations, adapters and any authored code supplying it. Multiple implementations
can deliberately support the same contract. Code-optional contributions retain their resolved
binding definitions and exact Conduit-controlled adapters even when no new executable is included.
Conflicting definitions of one contract identity/version cannot silently overwrite each other;
unresolved selection ambiguity blocks affected execution rather than choosing the newest package.
Required schemas must be available for registry writes. With known schemas, otherwise valid
configuration can still register while code is missing; report its readiness gap without substitution.

Adopting another implementation release/artifact requires a replacement instance in v0, even where
the new release claims the same contract. Explicit backing-data reuse remains possible under the
type's create/attach rules; it does not transfer runtime progress, connections or grants implicitly.
This avoids in-place code migration rules for active work and retained setup progress. Development
can replace affected instances without keeping the old version available for fresh creation.
The retained binding identifies executable configuration, not mandatory template-history storage
or a guarantee that historical executable contents remain available.

This rule applies to Conduit-selected declarations, mappings and plugin/adapter code. It does not
freeze externally operated services/native tool catalogs or promise control of external native-agent
binaries. A recorded service reference is not proof of exact provider code. Adapters declare and report those
separate compatibility/version boundaries. Dynamic Conduit attachments and supported catalog refresh
retain their accepted behavior. Artifact verification, package storage/delivery, plugin ABI and exact
binding fields remain engineering design within this contract; public naming governance is deferred.
Service-target and authentication selections continue to follow the existing instance configuration,
control and stable-reference rules; an implementation binding supplies no implicit access or fallback.

### Release compatibility

Conduit makes **no backward-compatibility promise between releases**. This applies to the
language/contracts, reference runtime and its component interfaces, packages and persisted formats;
it is especially important during early iteration and does not automatically change as the project
matures. Users can retain a known working combination by pinning its versions/artifacts. A future
compatibility or support commitment would require a separate explicit decision.

A release need not accept earlier configuration, run earlier plugins, interoperate with older
services or open older state. Upgrading may require revised definitions, replacement instances or
manual migration; migration tooling and continued maintenance of pinned releases are not promised.
Where migration is provided, its contract must describe changed behavior and supported recovery.

The current implementation need not retain legacy parsers, compatibility adapters, old protocol
execution paths or storage readers solely to support earlier releases. Retaining an exact artifact
selected by an existing instance is separate from supporting that artifact in a newer runtime.
An older working installation may be pinned; its existence does not impose legacy support on the
current codebase. The existing explicit artifact-cleanup and no-silent-substitution rules still apply.

This does not weaken conformance to the particular published contracts an implementation claims
to support, permit conflicting definitions to silently replace one contract identity/version, or
permit a new package installation to substitute an existing instance's retained implementation.
Unsupported configurations/dependencies remain explicit. Exact identity and version syntax are
still authoring work; release coordination and tested-distribution contents remain separate choices.

Pinning only a plugin does not establish compatibility with a different host or persisted format;
users may need to pin the whole working set. Pinning Conduit-controlled artifacts does not freeze
external services, native-agent binaries or the host environment. Their actual compatibility
continues to be reported under the existing integration contracts.

### Object identity

Every registered template and instance has a system-assigned stable ID independent
of its name. Renaming an object preserves that ID. Deleting it and creating another object
with the same name produces a new ID; name reuse does not make them the same object.

Readable configuration-object references explicitly include `kind`, `form` and `name`, such
as `{kind: Agent, form: template, name: reviewer}`. Kind/name locates the object; form states
the expected template/instance distinction and must match the target. A wrong-form target is
rejected. Typed fields such as `templateRef` still require explicit form, and their contracts
must agree with it. This makes the reference understandable independently of its containing field.
Conduit records the stable ID selected when the reference resolves. Authors need not copy
generated IDs into ordinary references; readable lookup and resolved identity remain distinct.

V0 resolves references within one local registry. Names are unique per family `kind` across
both forms: an Agent template and Agent instance cannot both be named `reviewer`; a Resource
may use that name because it belongs to another family. Pool ownership does not partition
this registry naming boundary. Additional Pool-local display names and optional direct-ID
references remain open. The registry catalogs definitions and IDs while backing data may be
local or remote. Registry presence grants no access and need not name every native item globally.

Templates name reusable concepts; instance names distinguish specific creations. Template-based
creation defaults to the template's name plus a generated unique suffix, such as `reviewer-7f3a`.
Callers may supply an explicit unique name instead. Direct creation from complete instance
configuration also requires an explicit unique name. All paths obey per-family uniqueness across
forms. Exact suffix format, name constraints and generated-collision handling remain open. Generated names remain
separate from stable system IDs, and a specific instance name does not imply automatic expiry.

V0 kind/name lookups use only current object names; registry object-name aliases are deferred.
After a rename, the old name becomes available for reuse. A fresh lookup of that old name
finds no object unless the name has been reused, in which case it can resolve to a different ID.
This does not redirect established bindings, which retain their resolved identity as below.

A rename uses the normal checked configuration update, changing `metadata.name` while
preserving the object's ID. The original ID/version identifies the target; the new name
must be available within the applicable naming boundary or the update conflicts.

Other registry objects retain their authored kind/form/name references after a rename. Report
known mismatches between a written name and its resolved target, and offer explicit checked
reference updates. The rename does not rewrite source YAML files.

Configuration-only export preserves authored reference names by default and reports known
mismatches. An explicit option refreshes names from known, unambiguous resolved IDs in the
exported copy only; it does not update registry objects or rebind anything. An old exported
name can still fail or identify a different object on a fresh import.

An established binding created from an explicit kind/form/name reference retains its resolved
stable ID until explicitly rebound. Renaming the target preserves the binding. Deleting
the target makes it unavailable; a new object using its old name is not substituted
automatically. Rebinding remains subject to current authority.

This fixes object identity, not content revisions or permissions. Current access checks
continue to apply; matches under supported typed access filters remain live. The ID format,
serialized field, cross-registry identity policy, optional direct-ID forms, import/export
handling and exact rebind operations remain open.

### Configuration updates

The checked full-replacement workflow below governs **template/authoring registry edits**.
It does not authorize replacing a running instance's spec through a generic registry write.
Instance configuration changes use the template-declared control contract through a Conduit.
They apply one at a time per target by default unless the contract allows safe parallelism.
They normally check the caller's expected configuration revision at application, with explicit
contract exceptions. Exact revision mechanics and partial-change semantics remain to be settled;
checked identity and safe outcomes remain necessary. Registry deletion retains its own scope.

The local logical registry exposes active configuration through the authorities defined above;
its aggregate observations are not independent writable state. YAML is explicitly applied or exported;
editing an input configuration file alone does not change registry objects, and CLI/API edits
do not silently rewrite that file. Files may remain in Git without automatic watching or
bidirectional synchronization. This defines configuration authority, not the registry's
storage technology or the update behavior of data exposed through Resources. Accepted live
access rules still apply when the registry configuration changes.

Configuration submissions explicitly select create or update intent. Creation reports a
conflict if that qualified name already exists. Updating a missing target reports a conflict
instead of creating it. Neither operation silently switches intent based on name matches;
exact commands remain open and batches follow the contract below. Creating Resource instances must
go through a Pool operation; a generic registry write cannot bypass the owner requirement. Registering
a template is distinct from provisioning instances or launching execution.

A v0 template update replaces the complete authored configuration, including `spec` and editable
metadata. Omitting an optional field removes its explicit setting; the field's omission,
default and validation rules apply instead of retaining its previous value. Instance creation
materializes declared schema defaults into its saved configuration;
configuration-only exports include those explicit values. Template YAML preserves authored
omissions. Fields without declared defaults retain their required/optional semantics; this
does not invent values or copy runtime observations into configuration. The object's
identity is preserved, and Conduit continues to maintain its system-owned version information
and status. Clients can edit a retrieved configuration copy before submitting it with the
original baseline. Partial patches are deferred.

Normal updates to an existing configuration object require its expected stable ID and
the configuration version the edit was based on. Reject the update if either differs
from the current object. This prevents stale edits from silently overwriting newer
configuration and prevents a reused name from redirecting an update to a replacement.

Registry-object deletion requires the same expected ID and configuration-version checks.
An intervening configuration change makes deletion conflict; the caller may reread and
reconcile before retrying. Incoming references alone do not block an otherwise permitted
checked deletion. Retain referring objects and their authored references, reporting affected
relationships as unavailable; do not prune those references or cascade removal to referring
objects. Deleting a Resource registration does not delete its owning Pool. Pool ownership
is stronger than an ordinary incoming reference: a Pool instance cannot be deleted while it
owns Resource registrations. Deletion requires explicit Resource retirement first, including supported
stop/disconnect steps and checked unregistration. An explicit bulk retirement action may handle
the Resources and then their Pool; partial failure retains the Pool and reports unfinished work.
Ordinary Pool deletion does not implicitly retire owned Resources. Ownership transfer is deferred.

Deleting a Resource registration preserves its backing files, repositories and service data,
including data created through Conduit. Cleanup or purge requires separate explicit operations
governed by their own permissions. Agent deletion also requires confirmed execution end
under the [Agent lifecycle contract](AGENT.md#lifecycle-and-continuity). All instance unregistration
also follows [confirmed retirement duties](LIFECYCLE.md#retirement-and-unregistration). Type contracts
define specific disconnection and helper cleanup duties; missing required confirmation blocks
unregistration.
An unowned Resource is an ownership inconsistency to resolve, not automatic purge authority.
The existing retain-until-explicit-purge rule remains; automatic orphan collection is not selected.

The configuration version covers stored `spec` and author-editable metadata. Changes to those
values invalidate the prior version for updates; status-only changes do not. The check
does not freeze live permissions or replace authorization at the time of the operation.

After authorization and the required ID/version checks pass, submitting unchanged
configuration reports an unchanged result and retains the current configuration version.
Record the operation outcome without invalidating other edits. Stale submissions still
conflict even if their submitted configuration matches the current values.

Equality ignores presentation-only formatting, comments and mapping-key order. For instance creation,
omitting a defaulted field and explicitly supplying the same value produce equivalent resolved
configuration. Control contracts still need explicit no-op and revision-result semantics; registry equality alone does not define their effects.
For template updates, compare parsed authored values: omission remains distinct from explicitly setting a default. Lists follow
their field-specific order rules. Scalar normalization, individual list contracts and
version-transition details remain open; equality does not compare runtime state, current
Resource contents or live authority. Source YAML files are not silently rewritten.

Clients may reread, reconcile and retry within their permissions without mandatory human
approval. Simply substituting a newly read version does not reconcile an older edit.

The expected ID and configuration version accompany the update as separate operation inputs,
not as fields in the reusable object YAML. V0 CLI/editor workflows retain a recoverable local
baseline containing the original configuration and its ID/version, associated with the edit
but separate from its YAML. An interrupted edit can resume with its original conflict checks.
Direct API clients supply the required preconditions themselves. An operation request or local
edit baseline is not an additional core object kind.

When no usable baseline is available, preserve the YAML and require explicit preparation of
a new baseline against the selected target. A user or agent can perform a recovery/import
operation that compares with current configuration and captures its ID/version before a
checked replacement. This cannot reconstruct missing edit history. An existing stale baseline
still requires reconciliation; it must not be silently replaced with the current version.
Exact request fields, baseline storage format/lifecycle and remaining creation/import/export
details remain open.

### Batch configuration submissions

V0 supports submitting several object definitions together as a convenience over individual checked
registry writes. Return per-object outcomes and preserve successful writes when another entry fails;
there is no atomic batch guarantee or inferred rollback of earlier registrations/updates.

A registration batch uses a constrained form of the existing finite, acyclic operation graph. Each
node invokes an ordinary checked registry create/update with a complete supplied authored definition
and its applicable preconditions. Complete definitions follow the existing omission/default rules;
this does not require filling every optional template field. Execution outputs do not construct or
modify later definitions in a v0 registration batch. General operation graphs retain their existing
output-binding capabilities.

Authors declare prerequisite edges explicitly. File position does not establish order, and ordinary
references inside object definitions do not automatically become graph edges. Independent eligible
nodes may run concurrently, subject to authority, target coordination and execution limits. Parallel
eligibility does not require an implementation to execute every independent write simultaneously.

For example, a history Resource-template registration can succeed while an Agent-template registration
encounters a name conflict. The history template remains registered and both outcomes are reported.
An incomplete batch must not be presented as complete success; existing operation tracking and
uncertain-outcome rules still apply.

Each entry retains explicit create/update intent and its applicable identity/version preconditions.
Allow at most one write to a target object within a registration batch. Before any writes, reject
two updates targeting the same stable object ID or two creates targeting the same kind/name, even
when those nodes are explicitly ordered. Combine edits into one complete checked replacement or
submit them separately. Separate clients remain subject to the existing per-write concurrency checks;
this restriction does not limit general operation graphs.

This is a limit on writes to the same registry object, not on instantiating a template. A `reviewer`
Agent template can produce many distinct instances, such as `reviewer-7f3a` and `reviewer-9c2b`.
Each has its own identity and follows its creation/start contract; sharing a source template does
not make the instances duplicate targets or consume the template. Registration itself starts no runtime.

Batching does not authorize generic edits to running instance definitions, bypass Resource creation
through a Pool, provision Resources or start runtimes. Registration and subsequent setup remain
separate actions.

Parse the complete submission, validate its graph structure, and validate every supplied definition
against its required schemas before any registry writes. If parsing fails, a required schema is
unavailable, or an entry fails schema validation, write none of the entries and report the problem
with its available location/entry context. For example, an unknown field in the Agent template
prevents registration of both it and the otherwise-valid history template. A cycle or invalid
prerequisite also blocks execution before writes.

Passing this prevalidation does not reserve names or revisions, freeze permissions, or guarantee
later writes. Current authorization, identity/version preconditions and other state-dependent checks
still apply at each write; later conflicts can leave the accepted partial outcomes described above.
Registration remains separate from runtime readiness and setup.

Use the shared [graph scheduling, cancellation and recovery contracts](LIFECYCLE.md#lifecycle-protocol).
An unsatisfied prerequisite blocks its dependent writes; independent eligible branches may continue.
Preserve completed results and uncertainty, and reuse only valid progress on an explicit retry of
the same invocation. Cancellation, partial failure and recovery do not imply rollback, automatic
retry or runtime teardown. For example, failed history-template registration blocks a dependent
Pool-template registration while an unrelated reviewer-template registration can proceed.

This is a graph invocation through existing management capabilities, not another object family or
a replacement for an instance's setup graph. It does not edit fixed instance definitions. Concrete
batch notation and detailed conflict diagnostics remain open.

### Objects, representations and contracts

Each object contract must distinguish what an author requests, what Conduit resolves and
what a runtime observes. These are representations of the existing concepts, not new
top-level concepts:

| Representation | Responsibility |
| --- | --- |
| Authored intent | Explicit settings, references, requested relationships and constraints. Creation choices can customize a template without becoming a retained override layer. |
| Instance configuration and resolution | A complete instance spec contains its settings and explicit dependencies. Resolving those dependencies establishes current bindings and availability; template history is unnecessary. |
| Observed state | Current binding, compatibility, authentication, preparation and execution information, including unknown or unsupported coverage. |
| Operation evidence | Actual attempts, effects, results and unresolved outcomes attributed to the Agent/execution and integration. |

The combined inspection view presents authored configuration and system-reported state
together without merging their meanings. Detailed status contents and references to retained
operation evidence remain kind-specific design choices.

Identity, editable names, source paths, configuration change versions, retained-record
revisions, plugin releases and provider protocol versions have different meanings. Their
encoding is open; none may silently stand in for another. A reference or capability
request is not permission. Configuring an account does not prove which identity actually ran.

Schemas describe data shape. Resource and Conduit contracts also describe operations,
preconditions, transitions, effects and failure behavior. Runtime compatibility, current
authority and successful execution require their own evidence. Preserve provider-native
contracts where possible instead of flattening every integration into one generic payload.

### Composition follows the relationship

In draft editing, explicitly assigning a field replaces its entire value, including an object
or list. Assigning `[c]` to a field containing `[a, b]` produces `[c]`. Editing a nested child
leaves its siblings intact; assigning its parent replaces that entire object. Retaining,
adding or removing entries requires deliberate edits, with no implicit object merge or list
append. Normal validation and default materialization still apply to the resulting draft.
These are draft-editing semantics; template writes use complete checked replacements. They do
not select the shape of runtime configuration-change requests.

Field contracts separately define omission, empty collections, nullable values, defaults,
collection order and duplicate identity. Relationship operations such as combining authorized
access paths retain their own semantics; field assignment does not redefine those operations.

| Relationship | Established constraint |
| --- | --- |
| Agent template → Agent | Interpret typed establishment configuration and retain ongoing instance settings. Customize ordinary typed fields with replacement semantics. Continuing authority and dependencies remain explicit; the type defines supported transitions. |
| Resource template → Pool → Resource instance | Only a Pool creates/attaches its instances. Explicit named entries identify available templates; reuse across Pools creates distinct owned instances. Referencing a foreign instance does not transfer ownership. |
| Agent → Conduit → Pool | Filters narrow authorized resource/operation combinations. Combining access paths must not pair one path's operations with another path's resources to create unintended access. |
| Agent activation | Instructions materialized in the Agent spec operate on permitted Resources. Their ordering, conflicts, optionality and update behavior must be explicit. |
| Intent → observation | Runtime observations report whether requests are realized; they do not silently become authored defaults or fresh authority. |

Permission, membership and activation semantics remain independent even when they use
similar reference/filter notation. A serialization library's merge behavior or filesystem order
must not choose these rules accidentally.

### Validation, effects and evolution

Distinguish syntax/schema checks, reference resolution, semantic/transition checks,
environmental probes and effects. A valid object can still have an unresolved reference,
an unsupported native extension or an expired credential. Error/status reporting must
identify the relevant object and field, explain the limitation, and preserve unknowns.
Exact diagnostic codes, stages and remaining operation-specific partial-failure details remain open.
Graph scheduling after failure follows the accepted [lifecycle rule](LIFECYCLE.md#lifecycle-protocol).

Typed configuration rejects unknown fields for both templates and instances, except within
maps whose schemas explicitly allow arbitrary keys. For example, `metadata.naem` is an unknown
field, as is `metadata.labels` in v0. A plugin contract may explicitly declare an open options
map; its keys and values still obey that contract. Openness is limited to the declared location. Plugin
contracts can define their own fields and open maps under the same rule. Unknown fields are
not silently discarded or retained outside those locations.

Create and replacement writes require the applicable configuration schemas, including required
plugin schemas. If a required schema is unavailable, reject the write: preserve source YAML
and any existing registered object rather than register an unvalidated replacement or new
object. This supports field validation and declared-default materialization.

When schemas and required registry checks pass, accept configuration even when its plugin
runtime is missing or a configured external service is offline. Report known readiness gaps
or unknown readiness, without claiming successful activation. Operations requiring unavailable
dependencies remain blocked. Registration remains separate from activation; validation,
authority and Pool-ownership requirements still apply. Schema availability establishes the
ability to validate configuration, not the availability of its operational dependencies.

Configuration changes, package preparation, binding establishment and agent execution have
different effects. Their operations must say what they do. An Agent export is not evidence
of a running execution, and restoring configuration must preserve the explicit-resume and
no-duplicate-execution rules. Current permissions still govern managed actions after a
preview. Local configuration consistency does not imply atomic rollback of external effects.

Version the meaning of configuration, including defaults, list composition, reference
interpretation and transition behavior. Keep concrete input/result/failure examples with
the contracts so implementations and tools can be checked for agreement. Where migration is
provided, its rules must expose changed behavior; updating a version string alone is not a migration contract.
Exact version identifiers and compatibility representation are schema work. Any proposed change
to accepted interpretation or instance-upgrade behavior returns for review.

Keep the core small and extensible through typed contracts. Reusable objects are the first
composition mechanism; additional template languages, expressions and patch systems need
concrete use cases. The following sections retain the accepted domain behavior that the
language must express. [Current review](../REVIEW.md) separates remaining decisions from the later schema and acceptance walkthrough.


## Declared-capability conformance

Conformance applies to explicitly declared supported capabilities. A client, plugin or component
may implement a limited surface without implementing the entire runtime or all Resource types.
Its claim must identify the public contracts it supports and their applicable common obligations.
A capability declaration is not proof that an implementation conforms.

A runtime claiming portable template execution must honor the relevant public graph, control,
lifecycle and recovery semantics, together with required contracts for the selected contributions.
It cannot claim that execution surface while treating required behavior as opaque configuration.
A limited client may consume a supported capability without implementing the construction engine;
unsupported executable semantics must be reported rather than silently ignored.

This separates public behavior from reference-implementation choices such as Rust crate layout,
physical plugin ABI, process grouping or storage engine. MCP/profile compatibility and the agreed
managed-access/evidence obligations remain applicable to supported surfaces. Exact profile names,
version negotiation and schema spelling remain authoring work.

**First conformance milestone:** require an independently authored client and plugin to work with
its reference runtime through published contracts and conformance tests. Exercise the public
extension boundary rather than depending on private implementation knowledge or special core dispatch.
A second complete runtime is deferred. Passing this milestone supplies evidence for those client/plugin
surfaces; it does not establish portability between two runtimes, unsupported capabilities, or
unexecuted provider/platform scenarios. No conformance test has run merely by adopting this scope.
