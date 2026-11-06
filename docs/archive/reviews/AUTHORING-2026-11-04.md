# Completed authoring review — 2026-11-04

**Historical decision record, not an active question queue.** Preserved from
[`82e87cd`](https://github.com/shrav-k/conduit/blob/82e87cd18eeb000f5277158e8dd3e0cae100eb1c/docs/REVIEW.md),
after [PR #3](https://github.com/shrav-k/conduit/pull/3) merged the behavior/structure review.
The original record follows with its title changed and relative links relocated. Its dated status
and recommendations retain their historical context; this archive does not ratify illustrative syntax.
Current behavior lives in the [protocol reference](../../protocol/README.md), ongoing work in
[current review](../../REVIEW.md), and naming references in the [vocabulary map](../../VOCABULARY.md).

---

Updated: 2026-11-04, after the protocol-documentation migration. This is the only active review queue.
The five-family model and completed conceptual/implementation rulings remain accepted in the
[protocol reference](../../protocol/README.md). The migration did not settle new semantics or approve syntax.

The post-migration review is complete. M1–M3 and the initial owner decisions are settled or explicitly
deferred below. Early delivery stays lightweight; contexts fit the existing Resource/Pool/Conduit
model. The next review is the concrete authoring/conformance walkthrough, preceded by the requested
[language/protocol research](../../research/language-protocol-foundations/README.md). No syntax was approved by
closing this review, and an open detail in an older report is not automatically another ruling.

## Future vocabulary PR

**Accepted review boundary:** Interface, Agent, Resource, Pool and Conduit are the settled names.
All other Conduit-owned terms and field spellings are temporary, including names provisionally
selected in earlier rulings. Accepted meanings and structures remain intact. Continue logical and
structural review with consistent working names; vocabulary work must not block it. See the
[protocol boundary](../../protocol/MODEL.md#vocabulary-review-boundary).

After the logical/structural questions are resolved, use a separate PR to review the vocabulary
together, term by term. For each term, show its responsibility, how it differs from nearby concepts,
and its use in a complete example. Agree on the terms before updating the glossary, documentation,
schemas and fixtures as a coherent set. External-standard terminology retains its defined meaning;
existing source examples and archives need not be rewritten as though their historical terms changed.

Carry these observations from the directory walkthrough into that PR:

- `path` currently names both a filesystem location and a pointer selecting nested data.
- `node` and `port` refer to a graph step and one of its declared outputs, which can suggest host/network meanings.
- A consumer input and producer output can both be named `directory`, but their names belong to
  separate contracts; the explicit binding connects them, not name matching.
- Contract, capability, requirement, policy, grant, operation, graph, plugin and package are working
  labels for the accepted responsibilities beneath the families, not final vocabulary.

This section records the future PR's scope, not approval of any replacement names or creation of
that PR now. The reference payload's direct-field structure is accepted separately; its supporting
field names still belong in this vocabulary review.

## Behavior and structure merge checkpoint

**Review complete; ready for the authorized documentation PR.** The accepted type-contract ruling
closes the last explicitly pending behavior choice. The directory, authenticated MCP, database
and shared-history examples now preserve the accepted boundaries, and Experiment 001's stale
binding/runtime notation has been migrated explicitly. No runtime implementation or conformance
claim follows from this checkpoint.

| Review area | Settled boundary | Work carried forward |
| --- | --- | --- |
| Packages and types | Published contracts; scoped dependencies/re-exports; one type per template/instance across all five families; no primary plugin. | Concrete manifests, identifiers, schema files and package capture/release tooling. |
| Graphs and lifecycle | Explicit typed value flow, finite dependencies, named instance-owned runtimes, ordinary startup operations, defined publication/default/recovery/cancellation behavior. | Executable schemas, driver/wire details and implementation algorithms, checked against these guarantees. |
| Access, controls and observation | Recorded participation grants, composed requirements, checked controls, current authority, retained operation/interaction state and source-scoped observation continuity. | Domain contracts and wire encodings; optional capabilities must declare their own guarantees. |
| Evidence | Updated paper fixtures and planned independent client/plugin checks; research probes retain their limited claims. | Implement and execute conformance fixtures later; static checks are not runtime evidence. |

Permanent supporting vocabulary remains a separate owner review. A concrete implementation or
schema counterexample that changes an accepted guarantee must return for a ruling; this closure
does not preapprove every illustrative field or promote older research recommendations to decisions.

The owner requests a PR merged to `main` once this behavior/structure review is complete.
The directory, authenticated MCP and database walkthroughs have supplied the structural rulings
recorded below; remaining implementation details and deliberately deferred work stay explicit.
Do not infer a new ruling from an illustrative example. The separate vocabulary
PR is not a prerequisite for this merge; use consistent working names in this review's result.
Prepare the PR from the resulting coherent documentation, verify its checks and merge under this
authorization once that checkpoint is met. This does not authorize starting runtime implementation.

## Migration-review rulings

- **M1 accepted:** no backward-compatibility promises or obligation to maintain legacy compatibility
  code. Users can pin a working set. Early delivery prioritizes a working protocol; strict release
  coordination, exact packaging and installers are not prerequisites. Claims about particular
  contracts and retained implementation identity still apply. See [compatibility](../../protocol/CONFIGURATION.md#release-compatibility).
- **M2 accepted direction/scope:** language/contracts, a minimal reference implementation, and small
  authoring examples with conformance checks using the accepted Experiment 002 scope. Bundled
  capabilities are replaceable and do not become a universal standard library. A documented working
  source checkout can suffice initially. Individual package contents and artifacts will be refined
  with the authoring slice; no detailed catalog or release format was selected. See
  [initial delivery](../../implementation/README.md#delivery-and-evidence).
- **M3 resolved through the existing model:** contexts can be ordinary Resources exposing specific
  plugin capabilities. A Pool owns the Resource, and Conduits supply its permitted access and
  dependencies. No universal Context layer or owning-context hierarchy is added. A filtered Conduit
  suffices when no separate Resource behavior/lifecycle is needed. Record identity, revisions,
  provenance and sharing remain capability contracts; exposed source records retain their ownership.
  See [memory and evidence](../../protocol/RESOURCES.md#retained-memory-and-evidence).

## Owner decisions before public distribution

- **License — accepted:** Apache-2.0 for original Conduit documentation, contracts and reference
  code. The official [LICENSE](../../../LICENSE) is included; reproduced third-party material retains its
  own terms and attribution. No separate CLA or DCO requirement was selected. Any additional
  contribution procedure remains open if needed; see [license scope](../../../CONTRIBUTING.md#license-and-attribution).
- **Public identities — deferred:** keep the current `shrav-k/conduit` repository location while
  protocol work continues. Organization selection/transfer, domains and permanent package naming
  remain for the owner to choose later. The accepted one-main-repository direction is unchanged;
  provisional examples and directory names do not establish permanent protocol identities.
- **Change and release authority — accepted:** the repository owner retains final authority over
  protocol direction. Collaborators propose/review changes; merges and releases use existing repository
  permissions and explicitly appointed maintainers/delegated responsibilities. Existing task
  authorizations remain valid. No fixed quorum or formal governance process is required initially.
  See [project ownership](../../../CONTRIBUTING.md#project-ownership-and-publication). Public support/security
  contact details and additional contribution procedures are deferred until needed for distribution.

These are project-owner decisions, not reasons to reopen the five families or invent another runtime
authority model. The [repository proposal](REPOSITORY-2026-11-02.md) retains options and rationale.

Apache-2.0 was selected for permissive reuse with an explicit, scoped contributor patent grant.
The [official license text](https://www.apache.org/licenses/LICENSE-2.0.html) was checked on
2026-11-02. Licensing does not change the no-backward-compatibility rule or select a release process.

## Refinements to carry into concrete examples

These seams need complete examples and conformance cases. Existing guarantees constrain the answer;
return for a conceptual ruling only if an example exposes a real behavior choice.

| Seam | Preserve while refining | Source |
| --- | --- | --- |
| Controls spanning multiple settings/effects | Declared partial confirmation is accepted, subject to validation of the complete resulting stored configuration. Retain uncertainty, revision checks and target coordination; coupled settings may require whole-change confirmation. No implied atomicity/rollback. | [Configuration updates](../../protocol/CONFIGURATION.md#configuration-updates), [controls](../../protocol/LIFECYCLE.md). |
| Composite results, cancellation and recovery | Deliberate public outputs, retained effects/evidence, valid child progress, retirement duties and other participants. Custom recovery hooks/evidence formats remain unselected. | [Composition](../../protocol/MODEL.md#authoring-concepts-beneath-the-object-families), [retirement](../../protocol/LIFECYCLE.md#retirement-and-unregistration). |
| Connection/composite readiness | Required preparation gates, independent later work, per-consumer participation and ready Pool Resources are distinct. Describe observable partial state without one universal lifecycle enum. | [Pools](../../protocol/POOLS.md), [Conduits](../../protocol/CONDUITS.md). |
| Independent MCP access and client enrollment | Familiar provider tools plus current authority, caller identity and evidence. A directly accessible component still needs verifiable authority; no offline grants or bypass is implied. | [Independent components](../../protocol/MODEL.md#plugins-and-independently-accessible-mcp-components). |
| Foundational repair and credential interaction | Locally verifiable owner recovery, retained identities/progress and adapter-guided enrollment. No automatic reset, secret logging or duplicate execution when state is unknown. | [Interface workflow](../../protocol/MODEL.md#interfaces-and-the-user-workflow), [authentication](../../protocol/POOLS.md#shared-access-and-authentication). |

## Later authoring and conformance pass

The [research comparison](../../research/language-protocol-foundations/README.md) and
[worked cases](../../research/language-protocol-foundations/WALKTHROUGHS.md) now inform this pass.
They recommend separating contract imports, graph value bindings and authorized live connections;
they do not decide reference spellings, a new expression language or a runtime dependency.
Twenty [library probes](../../../experiments/003-language-probes/README.md) expose validation pitfalls;
they are not Conduit conformance results. Research recommendations remain in these four batches,
without reopening settled concepts or creating a second question queue.

The [authoring packet](../../proposals/authoring-syntax/README.md) preserves the examples for review.
The authoring review accepts [package-local scope and explicit dependency bindings](../../protocol/MODEL.md#package-scope-and-public-bindings):
caller aliases do not override package internals, and replacements use explicitly exposed bindings.
Replaceable dependencies may declare default implementation bindings. Compatible explicit replacements
take precedence; invalid selections report errors without fallback. These decisions do not accept
`spec.plugins`, `{plugin, name}` or an import/export file layout.
V0 imports also require an export's explicit support for the requested contract identity and exact
version, alongside existing compatibility checks. Cross-version mappings are deferred; implementation
release versions remain independent, with no obligation to maintain old contracts or compatibility code.
The [configuration foundation](../../protocol/CONFIGURATION.md#configuration-language) is accepted:
constrained YAML 1.2 for human authoring, equivalent JSON input/interchange, and JSON Schema
2020-12 for data contracts, with one shared value model. Conduit defines lifecycle, authorization
and other behavioral semantics separately. The [comparison](../../research/language-protocol-foundations/FORMATS.md)
retains the rationale; detailed schema features and reference syntax remain open.

The [schema-resolution boundary](../../protocol/CONFIGURATION.md#schema-resolution) is accepted:
resolve from the selected package and explicitly supplied or declared schema dependencies;
report missing required definitions without automatic network fetching during validation.
Explicit acquisition beforehand remains possible, and exact reference syntax is open.

The [schema-keyword rule](../../protocol/CONFIGURATION.md#schema-keyword-acceptance) is accepted:
reject unrecognized keywords in Conduit-authored schemas. Reuse standard descriptive annotations
where applicable, with additional custom metadata in one dedicated map of namespaced entries.
Custom annotations supply optional tooling hints, not validation, authority or runtime behavior.
The map keyword, namespace syntax, value schemas and external MCP adapter details remain open.

The [default-conflict rule](../../protocol/CONFIGURATION.md#schema-default-resolution) is accepted:
conflicting applicable defaults for an omitted configuration field require an explicit valid
value or a correction to the defaults. Schema order does not choose a winner, and all applicable
validation constraints still apply. Exact default-resolution mechanics remain open.

The [format rule](../../protocol/CONFIGURATION.md#schema-keyword-acceptance) is accepted: `format` in
Conduit-authored schemas is an enforced validation requirement. V0 selects all 19 standard formats
defined in JSON Schema 2020-12, with offline syntactic checks and conformance tests before support
claims. Unsupported formats are reported; plugin-defined validators are deferred. Custom operations,
Resource contracts and requirement checks remain extensible. Provider MCP dialect semantics remain unchanged.

The [provider MCP schema boundary](../../protocol/CONFIGURATION.md#provider-mcp-schemas) is accepted:
preserve the declared dialect's semantics without automatically imposing Conduit's stricter
authoring profile. Allowed descriptive annotations do not alone reject a tool; unsupported
required validation features are reported without stripping constraints or claiming enforcement.
Exact adapter handling and conformance cases remain open.

The [package-artifact rule](../../protocol/CONFIGURATION.md#implementation-selection) is accepted:
resolved artifacts retain a content digest alongside package identity and, when released, release
identity, covering contracts, graphs, templates and any bundled code, including code-free packages.
The digest identifies content, not publisher trust or remote service code. Hash algorithms,
bundle boundaries and concrete fields remain open.

The [edit-mode/release boundary](../../protocol/CONFIGURATION.md#implementation-selection) is accepted:
deliberately preserving an implementation for later selection requires intentional versioned
publication of fixed package contents, identified by a digest and retrievable from a public or
private distribution source. Conduit resolves/verifies/executes the package without necessarily
owning publication or hosting. A separate local-only release facility is not selected. Changed
published contents require a new release version. Otherwise authors work in edit mode, preserving
the running instance's code/contracts against source edits without archiving every revision. A source commit,
draft version field or test capture alone does not release a package. Direct handler tests can use
editable source; managed test/start workflows can capture contents automatically. Ordinary iteration
replaces affected instances, with explicit backing-data reuse where supported; keeping both revisions running is
optional. No archive of every edit, manual release per edit or selectable development-build
catalog is required. An edit-mode digest identifies content without guaranteeing its future
availability; missing restart/recovery code is reported without substitution. Retained evidence
does not itself require retaining executables. Instance retirement does not delete a preserved
release. This does not freeze credentials, Resource data or the host environment. Exact
publication/acquisition mechanisms, metadata, storage and removal mechanics remain open.

The [package-content rule](../../protocol/CONFIGURATION.md#implementation-selection) is accepted:
explicitly declare included files/directories, with directory and pattern rules available.
Do not capture the whole working directory by default. Unselected files do not contribute to
the artifact or its digest. Exact syntax, descriptor handling and internal symlink representation remain open.

The [capture-boundary rule](../../protocol/CONFIGURATION.md#implementation-selection) is accepted:
one explicit source directory bounds included paths and symlink targets. Outside targets are
reported rather than silently copied. Authors may stage shared files, choose a wider explicit
directory or use separate package dependencies. Resource paths and authorized runtime access
are unaffected; internal symlink representation remains open.

The proposed same-name/release multi-artifact catalog is superseded by edit mode plus deliberate
versioned releases. Different edit-mode captures do not become multiple released contents under one
version. Protecting active execution does not require an archive of every development build.
Contract checks and no silent substitution remain applicable; public registry mechanics remain open.

The [graph-input boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: v0 inputs use
literals and typed references to configuration or earlier outputs. Plugin operations perform
calculations and transformations; inline expressions in bindings are deferred. Output bindings
imply dependencies as already agreed. Exact reference syntax remains open, and branching/loops
still remain inside plugins initially.

The [reference-recognition boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: recognize
references only in explicitly designated binding positions. Literal payloads, ordinary configuration
and plugin results are not recursively reinterpreted as graph instructions, even when they resemble
reference notation. Resolving a reference retrieves data, not another instruction to evaluate.
The accepted single-key forms now distinguish bindings from literal data; reference payload fields remain open.

The [structured-input boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: graphs can
explicitly assemble object fields and list items from literals, references and nested assemblies
without a plugin merely to build that structure. Output references still imply dependencies; literal
payloads remain data. Calculations and other transformations remain plugin operations. No automatic
merging, string manipulation, loops or inline expressions are introduced. Assembly encoding now uses
the accepted `object` and `list` form keys below.

The [input-validation boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: catch provable
wiring/type errors before execution, then validate each step's resolved inputs before dispatch.
No static proof that every possible upstream output satisfies the receiving schema is required.
A producer permitting integers 1–100 can feed a consumer requiring 1–10, with 5 accepted and 50
rejected before the receiving step runs. Required schema support remains mandatory; rejection does
not undo earlier effects. Exact checking algorithms and diagnostics remain open.

The [output-validation boundary](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: validate each
declared plugin output against its producer's contract before downstream use. Withhold invalid
output and report a contract error while preserving effects and uncertainty. A malformed descriptor
does not establish that creation had no effect or is safe to replay. Producer-output and receiving-input
checks remain separate; provider-native schema interpretation is unchanged. Exact result/error
representation remains open.

The [graph-failure scheduling rule](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: independent
branches continue when their own prerequisites remain satisfied. Unmet prerequisites block affected
work, required unfinished setup remains incomplete, and current authority, coordination, cancellation
and stop rules still apply. Preserve effects and dispatched outcomes without automatic retry or rollback.

The [explicit graph-cancellation rule](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: stop
starting ordinary steps and request cancellation of in-flight child operations where supported.
Track actual outcomes, effects and uncertainty, and perform required cleanup. Preparation's existing
attachment-release duties still apply; an already-started runtime follows its instance lifecycle.
A request is not proof of stopped work, rollback or completed cleanup. Exact propagation and
transport mechanics remain open.

The [configuration-capture rule](../../protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: configuration
references within one graph execution use values captured when it begins. Later executions capture
the then-current settings. The instance still has one stored configuration map; captured inputs do
not establish current runtime state. Authority, dependency validity and control-application revision
checks remain current, and retry/recovery must revalidate retained results. Exact representation and
recovery mechanics remain open.

The [reusable-operation input boundary](../../protocol/MODEL.md#authoring-concepts-beneath-the-object-families)
is accepted: caller configuration is supplied through declared typed inputs, with no implicit
inheritance of the caller's configuration namespace. A template graph passes its captured limit
into the reusable operation's `limit` input. Scoped invocation context and explicitly authorized
Resource access remain available under their contracts. Exact input/context syntax remains open.

The [package entry-document rule](../../protocol/MODEL.md#authoring-concepts-beneath-the-object-families)
is accepted: one explicit entry document declares public exports and dependencies, with definitions
inline or referenced in package-local files. It may be handwritten or generated; inspecting it
requires no plugin execution or directory-based export discovery. Provider-native catalogs remain
dynamic. Filenames, fields and reference syntax remain open. The
[complete package walkthrough](../../proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md) illustrates both
single-file and multi-file declarations.

The [public export-name rule](../../protocol/MODEL.md#package-scope-and-public-bindings) is accepted:
names are unique across contribution kinds within a package, including inline and referenced
definitions. Selecting the package and export name identifies one contribution; resolution still
checks its expected kind and contract. Internal names, contract identities and registry object
naming remain separate. Exact name grammar and reference notation remain open.

The [explicit re-export rule](../../protocol/MODEL.md#package-scope-and-public-bindings) is accepted:
a package may expose a declared dependency's public contribution under its own export name without
copying or wrapping it. Preserve the original definition, contract, internal reference scope and
resolved dependency identities. Export-name uniqueness and kind/contract checks still apply; the
alias adds no behavior or access. Exact declaration and resolution notation remain open.

The [template implementation-selection structure](../../protocol/MODEL.md#authoring-concepts-beneath-the-object-families)
is accepted: one named dependency map, with setup operations, runtime drivers and control handlers
selecting an alias plus a public export name. There is no primary alias; each use checks the expected
kind and contract. Package-internal scope and retained implementation identities remain unchanged.
`spec.plugins`, `{plugin, name}` and other concrete field spellings are still proposals.

The dedicated custom-annotation map is accepted under the schema-keyword rule above. Presentation
and documentation hints do not change validation, grants or control behavior. Exact spelling and
value schemas remain open; provider-native schema semantics are unchanged.

The v0 format-extension boundary is accepted: use a shared published set of standard formats,
deferring plugin-defined validators. Schema authors can still express custom declarative constraints
through supported keywords and reusable schemas; provider MCP dialect semantics are unchanged.

The shared format set is accepted above. The [format-set follow-up](../../research/language-protocol-foundations/FORMATS.md#standard-format-set-follow-up)
retains the rationale; the decision does not select UUID object IDs, graph-reference notation or
network access, and no runtime support is claimed by recording it.

The [portable integer boundary](../../protocol/CONFIGURATION.md#numeric-interoperability) is accepted:
Conduit-declared integer configuration, inputs and outputs in v0 use −9,007,199,254,740,991 through
9,007,199,254,740,991. A contract needing a larger exact integer can explicitly use a decimal string;
there is no automatic conversion or rounding into range. Core revision/ID encodings and
provider-native schemas remain separate. The [number-portability follow-up](../../research/language-protocol-foundations/FORMATS.md#number-portability-follow-up)
retains the rationale.

The [fractional-number boundary](../../protocol/CONFIGURATION.md#numeric-interoperability) is accepted:
ordinary fractional configuration, inputs and outputs use finite binary64 (double-precision)
semantics, including normal representational rounding. Exact decimal quantities use explicit
decimal-string or scaled-integer contracts; integer limits remain strict. This refines the earlier
blanket precision wording. The [follow-up](../../research/language-protocol-foundations/FORMATS.md#fractional-number-follow-up)
retains the rationale; detailed validation/serialization fixtures remain engineering and conformance
work, with any newly exposed public behavior choice returned for review.

The [batch submission boundary](../../protocol/CONFIGURATION.md#batch-configuration-submissions) is accepted:
v0 supports multi-object configuration submissions as a convenience over ordinary checked writes,
with per-object outcomes and retained partial successes. No atomic batch guarantee or rollback is
implied. Explicit create/update intent, update preconditions, Pool ownership and instance-control
rules remain mandatory; registration starts no runtime. Exact YAML/JSON representation remains open.

Whole-batch prevalidation is accepted: parse the complete submission and validate every entry against
its required schemas before any registry writes. A malformed/schema-invalid entry or unavailable
required schema prevents all writes, with diagnostics identifying the problem. Passing this check
does not reserve names or revisions, freeze permissions, or guarantee later writes: current
authorization, identity/version and other state-dependent checks still apply at each write, with
partial outcomes reported under the accepted batch rule.

The batch-graph boundary is accepted: registration batches use the existing finite dependency-graph
semantics with complete supplied definitions, explicit prerequisites and ordinary checked writes.
File order does not schedule work, and registry references do not implicitly generate graph edges.
Independent nodes are eligible for parallelism under current checks and coordination. Shared graph
failure, cancellation, outcome and valid-progress recovery rules apply. All definitions and graph
structure are validated before writes; earlier outputs cannot generate later definitions in this
v0 batch form. General operation graphs retain their existing dataflow capabilities. This replaces
the unaccepted sequential-order recommendation and settles batch scheduling through the shared
graph rules; it introduces neither a new object family nor an atomic batch.

The one-write-per-object rule is accepted for registration batches: reject two updates targeting
the same stable ID or two creates for the same kind/name before any writes, even if explicitly
ordered. Combine edits into one replacement or use separate submissions. This does not restrict
general operation graphs or separate clients' checked writes. It also does not limit instantiation:
the same `reviewer` template can launch multiple independent Agent instances with distinct identities,
private state and lifecycles. Sharing a source template does not make those instances duplicate targets.
Batch notation and detailed conflict diagnostics remain open.

Nested value selection is accepted: use standard
[JSON Pointer](https://www.rfc-editor.org/rfc/rfc6901.html) paths within an explicitly selected graph
input, configuration or output value. `/connection/endpoint` selects a field and
`/participants/0/name` selects a field in the first array item. Exact reference payload fields remain
open. This does not select object identities, authorize Resource access
or evaluate literal strings; registration batches still require complete supplied definitions.

Missing-selection handling is accepted: report a reference-resolution error and prevent the consuming
step from running when its pointer selects no value. Do not substitute null or treat the bound input
as omitted to obtain a default, including for optional receiving fields. An existing null value is
selected data subject to the receiving schema. Earlier effects and independent eligible branches
retain their agreed behavior; declared defaults for genuinely omitted fields are unchanged. Exact
diagnostic fields remain authoring work.

Pointer omission is accepted: an omitted nested pointer selects the whole explicitly chosen source,
equivalent to the empty JSON Pointer. The source itself remains explicit and must resolve. This
neither omits the receiving input nor applies its default; missing sources or selected members
retain their existing error behavior. Reference field names remain open; the direct-field layout
is accepted below.

Reference structure is accepted: use separate structured fields for source selection and the nested
JSON Pointer, rather than encoding the whole reference in a string. For example, identify step
`connect`, its declared output `connection`, and pointer `/endpoint` separately. Exact field names
remain provisional. Source-selection fields sit directly inside `reference` beside its optional
pointer, without a nested source object. Typed source checks,
output dependencies and missing-value/literal-data rules remain unchanged.

Explicit binding forms are accepted: each supplied v0 graph input binding selects a literal,
reference, object assembly or list assembly form, including field/item bindings inside assemblies.
There is no bare-value shorthand. A literal can contain an entire ordinary object/list without
wrapping its contents; resolved references likewise remain data. Plugins receive resolved values.
Ordinary configuration/provider payloads do not acquire graph wrappers.

Form encoding is accepted: a binding object has exactly one form key—`literal`,
`reference`, `object` or `list`—whose value contains that form's payload. For example,
`{literal: 15}` or `{object: {title: {literal: Review}}}`. Object assembly contains a map of child
bindings; list assembly contains a list of child bindings. Bindings with zero, multiple or unknown
form keys and malformed payloads are errors; empty assemblies and literal payloads containing form-like keys
retain their ordinary meanings. This adds no shorthand, merging or expression evaluation.

Reference payload layout is accepted after the comparative research and walkthroughs: use direct
source-selection fields beside the optional pointer. Names such as `from`, `path`, `node` and
`port` remain working vocabulary. Exact source selectors and metadata fields/encoding remain open;
the five-category v0 source inventory, typed metadata access and pointer omission are accepted below.

Use the requested
[graph authoring comparison and decision audit](../../research/language-protocol-foundations/GRAPH_AUTHORING_REVIEW.md).
It recommends checking complete directory, authenticated MCP and database setup examples against
the accepted rules. Review authoring costs, transformation granularity and failure behavior alongside
field layout. This is research guidance, not a new ruling or an automatic reopening of accepted
contracts. Identify any proposed revision explicitly; the protocol reference remains authoritative.

The first [guided directory walkthrough](../../proposals/authoring-syntax/AUTHORING.md#guided-directory-walkthrough)
now traces configuration, plugin/graph bindings, Pool-owned instantiation, subsequent Agent access and
failure/retry. It is an illustrative recipe with a required setup-time layout check, not an executed
deployment or a new ruling. Its authoring costs and distinctions can be compared with the authenticated
MCP and database cases. The direct-field reference layout is now accepted; the example does not
settle the remaining selector encoding or vocabulary.

The second [guided authenticated MCP walkthrough](../../proposals/authoring-syntax/AUTHORING.md#guided-authenticated-mcp-walkthrough)
now traces Pool composition, a shared authentication Resource, a service Resource's setup outputs,
separately authorized Agent attachments and ongoing refresh/reconnect. It introduces the producer
contracts before the reference fragment and distinguishes graph results from usable credentials
and current permission. This is an assumed compatible integration, not verified Linear support or
a runtime test; no new behavior or field layout is approved by this walkthrough.

The third [guided database/migration walkthrough](../../proposals/authoring-syntax/AUTHORING.md#guided-database-and-migration-walkthrough)
now covers instance-managed startup, an explicit migration-success prerequisite for verification,
retained runtime/backing after failure, unknown effects, valid-progress retry and independent Pool
work. The candidate ordering field is illustrative. No database was started, and no automatic
rollback, replay or transaction across domain and authority stores is assumed. These examples
support the current graph boundary but do not approve unresolved syntax or close the merge checkpoint.

**Runtime binding is accepted:** setup uses ordinary typed operations explicitly bound to the
instance's declared runtime, with prepared inputs and results carrying defined guarantees.
The instance owns ongoing activity from startup; later setup failure can leave it running.
The [database/cloud comparison](../../research/RUNTIME_ESTABLISHMENT.md) records the alternatives and
rationale. Request acceptance, usability for setup, successful application preparation and current
availability remain distinct; no universal status enum or new object family is introduced.

**Runtime target placement is accepted:** the target is a declared typed operation input using
ordinary graph bindings. Reusable graph-backed operations explicitly receive and forward it;
no separate runtime-context binding is required. Database startup receives the runtime target and
prepared storage; migration later uses its usable database result. A declaration reference proves
neither startup nor authority. Authenticated caller identity and authority remain trusted invocation
context, with current checks at resolution/use. Reference encoding and protected transport stay open.

The [reference comparison](../../research/language-protocol-foundations/GRAPH_AUTHORING_REVIEW.md#reference-layout-decision)
preserves the direct-versus-grouped tradeoffs behind the accepted layout. No change to access,
dependencies, validation or pointer behavior follows from that placement.

**Ordering prerequisite default is accepted:** explicit ordering without data normally requires
the predecessor's successful completion under its operation contract. Failure or uncertainty does
not release the dependent step. The database example requires migration success before verification;
mere completion or a schema-valid failure payload is insufficient. Declared output-publication
dependencies, independent branches and required lifecycle cleanup retain their existing rules.

**Graph-visible invocation metadata is accepted:** graphs may reference explicitly declared, typed,
read-only facts supplied by the runtime, such as the owning instance ID used to tag a created service
for recovery. No operation solely to retrieve that ID is required. This does not expose the whole
instance, ambient environment or credentials. Supplied IDs cannot replace authenticated caller
context or confer access. Exact metadata fields, scope and source encoding remain open.

**V0 reference-source inventory is accepted:** declared graph inputs, captured instance configuration,
earlier step outputs, declared runtime targets and typed invocation metadata. Other data access uses
explicit authorized operations; arbitrary plugin-defined reference resolvers are deferred. Reusable
operations still receive caller configuration and runtime targets through declared inputs. No
implicit inheritance or live registry reads follow. Source names, selectors and metadata contents
remain open.

**Multiple named runtimes are accepted:** an instance may declare several when its type supports
them, such as a database service plus an optional diagnostics helper. Each startup operation targets
its intended declaration explicitly. Components remain instance-owned; this does not require a
process per declaration, create new object families, allow extra primary Agent executions or change
endpoint ownership of connection-specific activity. Existing setup, control and retirement contracts
govern their behavior. The number of declarations does not introduce a new global failure policy;
exact declaration syntax and contract-specific coordination remain authoring work.

**Shared managed-reference contract is accepted:** Conduit-declared live service and protected
authentication bindings share ownership, resolution and validity semantics, while their owning
implementations retain actual live objects/credentials. Capability contracts supply particular
behavior and lifetime guarantees; common reference handling does not make different capabilities
interchangeable. Possession grants no access, current authority still applies, and retained values
do not establish current availability. This concerns operation values, not the graph's `reference`
authoring form. It selects no wire fields, central broker or new source category and does not
reinterpret native provider MCP schemas. Exact representation and resolution mechanisms remain open.

**Declared partial confirmation is accepted:** a control can confirm a subset of its requested
changes when its contract supports that result and the complete resulting stored configuration is
valid. Record the confirmed settings and retain the remaining effects' failure, pending work or
uncertainty separately. A confirmed query-limit change can therefore be recorded while a timeout
change is unresolved; its last confirmed timeout is not presented as known current. Coupled settings
may require whole-change confirmation. This neither infers independence between arbitrary fields
nor promises rollback; current authority, revision checks and coordination still apply.

**Unchanged-control revision rule is accepted:** when a control confirms unchanged stored settings,
retain the configuration revision and record the invocation, outcome and effects separately. A
request for a query limit already set to 15 can still verify or refresh the provider. All applicable
checks and declared handler behavior remain required; equal values do not justify a skipped handler
or cached success presented as a new execution. This matches the existing unchanged registry-write
rule without treating configuration revisions as operation or observation identities.
Ordinary MCP-client access itself is already accepted in the model; exact enrollment/transport
mechanics remain refinements rather than another approval of that interoperability direction.

**Setup success requirements are accepted:** steps are required by default, with explicit optionality
for nonessential steps. Required migration failure blocks successful setup; optional diagnostics
failure remains visible without itself blocking success. Actual dependencies and mandatory type/access
requirements still apply. Optionality cannot supply missing results or waive enforcement/validation.
Independent scheduling, output validity, retained effects and lifecycle cleanup remain unchanged.
Exact notation remains open.

**V0 ordering-trigger scope is accepted:** authored prerequisites without data require successful
completion; declared output-publication dependencies remain supported. Additional authored
`on failure` and `after any completion` triggers are deferred. Required cleanup and supported
recovery remain responsibilities of operation/instance contracts even when ordinary graph work
cannot continue. They do not depend on a failed operation reporting success. This preserves the
initial plugin-owned branching/error-handling boundary without automatic rollback; names are illustrative.

**Graph-step collection is accepted:** use a map keyed by names unique within the graph, with each
entry containing an operation binding. Dependencies determine ordering constraints; document order
has no execution meaning. Current dispatch checks and scheduling limits still apply. Step names are
distinct from plugin export names and runtime declarations. Different graphs may reuse local names;
this grants no new visibility into a called graph's internals. Container names and supporting
vocabulary remain open.

**Public graph value bindings are accepted:** reuse the four input-binding forms—literal, reference,
object assembly and list assembly—under existing source/scope rules. A graph can publish an object
combining a literal label with a database reference from startup without an extra assembly operation.
Publication conditions, validity, output validation, access and sensitivity still apply. Constructing
a public value does not confirm whole-graph success, broaden scope or export other internal outputs.
Domain-error/outcome mapping, remaining contract fields and final field names remain open.

**Graph-input root is accepted:** the source selects the complete declared input payload and uses
the optional JSON Pointer within it, without a separate input-name selector. Given
`{limit: 15, options: {timeout: 30}}`, `/options/timeout` selects 30; omitting the pointer selects the
whole payload. This supplies no ambient caller configuration, runtime metadata or authority.
Validation, scope and missing-selection rules still apply; source field names and input-contract
representation remain open.

**Runtime-target value is accepted:** the source selects one named declared runtime and yields a
typed target reference that the responsible service resolves against the instance-owned definition.
Prepared settings remain explicit operation inputs. The structured declaration itself is not
supplied as target data. Owner/identity, contract and current authority checks still apply; selection
grants no access, starts nothing and permits no definition edits. Reference fields and inspection
capabilities remain separate; this does not prescribe one opaque wire format or prohibit authorized
definition inspection. The target remains distinct from the usable service result produced by startup.

**Operation-input defaults are accepted:** Conduit materializes unambiguous applicable declared
defaults for genuinely omitted inputs, then validates the prepared payload before invoking code
or a graph. The graph-input source sees that payload: an omitted `limit` defaulted to 10 is visible
as 10. Child invocations use the same preparation rule under their own contracts. Explicit invalid
values, explicit nulls and failed reference selections do not acquire fallback defaults; null must
satisfy its schema. Conflicting applicable defaults require explicit resolution. Provider-native
MCP schemas retain their separate semantics; detailed default applicability remains open.

**Prerequisite placement is accepted:** declare explicit success prerequisites on the dependent
step beside its operation and inputs, using predecessor names local to that graph. Verification
names migration; a separate authored graph-wide edge collection is not used. Output references
still imply their own dependencies without duplicate declarations. Validate the combined graph
for missing steps and cycles, retaining successful-completion checks and independent scheduling.
This introduces no additional trigger behavior. Final field names remain for the vocabulary PR.

**Runtime collection is accepted:** use a map keyed by names unique within the instance definition,
such as `database` and `diagnostics`, with each entry holding its declaration. Runtime-target references
select the local name, not a list position; duplicate keys are errors. Other instances may reuse the
same local names without sharing identities or authority. Map order has no execution meaning and
declaration alone starts nothing. Instance ownership, explicit startup and existing lifecycle rules
remain unchanged; driver/input fields and vocabulary remain separate.

**Public-output prerequisites are accepted:** an output may declare local steps whose successful
completion it requires, independently of its value binding, using the same success semantics as
step prerequisites. A public database output can bind startup's reference but wait for verification
success, without an extra forwarding operation. Failed or uncertain prerequisites withhold the
affected output. Publication conditions, current validity, output validation and access still apply;
publication neither confirms whole-graph success nor promises permanent database availability.
Output field names and declaration layout remain open.

**Public-output publication timing is accepted:** an output is eligible once its value bindings,
explicit prerequisites and other declared publication conditions are satisfied, without an implicit
wait for unrelated work or whole-graph success. A database output can be available after verification
while independent diagnostics continue. Current validity, validation, access and stronger declared
conditions still apply. Publication neither completes outstanding work nor marks the graph successful
or its owning instance prepared.

**Shared graph-step requirements are accepted:** steps are required by default, with explicit
optionality, across setup, reusable graph-backed operations and change-handler graphs. Required steps
must succeed for successful graph completion; optional failures remain visible without alone preventing
it. Public-output availability, permitted partial control confirmation, actual dependencies and mandatory
type/access obligations retain their rules. Optionality does not abandon outstanding activity, erase
effects or report still-running work as finished. Result schemas and field names remain separate.

**Public-output requirements are accepted:** declared public graph outputs are required for successful
completion unless explicitly optional. A required database output must be produced under its contract;
an optional diagnostic report may be absent with its known reason or uncertainty reported. Optional
steps do not automatically make their public outputs optional. This concerns whole named outputs,
not optional members inside a value. Validation, access, reference/dependency checks, publication
conditions, effects and outstanding work retain their rules. Absence produces no fabricated value
or null. Exact output-contract notation remains open.

**Published value stability is accepted:** each named step output and public graph output retains
one fixed published value per producing attempt. Redelivery of that recorded value is permitted;
conflicting publication reports a contract error without replacing the value supplied downstream,
while preserving effects and uncertainty. A later attempt can publish a separately identified result
without rewriting earlier evidence. This does not promise exactly-once transport or authorize retry
or effect replay. Live Resource state, availability, access and progress/streaming observations retain
their own contracts; a fixed reference is not proof of current usability. Existing validity and
recovery checks still apply. Attempt identifiers and comparison encoding remain open.

**Public-output declaration grouping is accepted:** use a map keyed by names unique within that
graph's public outputs, with each entry grouping its value binding and applicable output-specific
declarations, such as optionality and success prerequisites. The database entry binds startup's
reference and requires verification success; a separate diagnostic-report entry is explicitly
optional. Duplicate output keys are rejected. Map order schedules neither execution nor publication.
Existing result-contract derivation, publication/validity and fixed-value semantics remain intact,
without requiring inherited contract information to be repeated. Final field spellings and remaining
contract-field encoding stay open. The database walkthrough now shows the grouped structure.

**Omitted optional-object defaulting is accepted:** leave the object absent when it has no applicable
default of its own, even if its child properties declare defaults. An optional `retry` object does
not become `retry: {delaySeconds: 5}` merely because the child defaults to 5. An author can explicitly
default the parent when that presence is intended, subject to the existing conflict and validation
rules. Apply this during instance configuration and Conduit-authored operation-input preparation,
including child invocations. Provider-native schemas, template omission preservation, required-field
validation, explicit nulls and failed references retain their rules. Other applicability details
remain separate.

**Supplied-object defaulting is accepted:** apply unambiguous applicable child defaults to omitted
properties within a supplied object, including `{}`. Supplying `retry: {}` prepares
`retry: {delaySeconds: 5}` when that child default is declared. Supplied values remain subject to
validation, without fallback for invalid values or nulls; default conflicts require resolution.
This clarifies the earlier empty-collection wording: the parent is present and is not replaced by
a whole-object default, but omitted children can be defaulted. Do not merge old configuration or
fill empty lists. The absent-parent rule, template omission preservation and provider-native
semantics remain intact. Preparing receiving inputs also preserves fixed producer output values.

**V0 conditional-default scope is accepted:** defer defaults whose applicability depends on a
conditional or alternative schema branch. Reject unsupported branch-local defaults when accepting
Conduit-authored configuration/input schemas, while retaining conditional/alternative validation.
Local/remote variants can validate different timeout requirements, with templates supplying the
timeout explicitly or a common default declared outside the alternatives. Explicit payloads do
not bypass unsupported schema declarations. Ordinary defaults and composed-default conflict rules
remain, with no trial insertion to select a branch. The
[source comparison](../../research/language-protocol-foundations/FORMATS.md#default-materialization-follow-up)
retains the rationale. This concerns schema defaulting, not authored graph branching or provider-native
schema semantics; exact restricted keyword paths and diagnostics remain profile work.

**Early default validation is accepted:** reject a declared default provably invalid against its
own schema after applicable supported preparation when accepting a Conduit configuration/input
schema. An integer constrained to 1–100 cannot declare 0 as its default even if one caller supplies
15. Report the declaration error. This does not require proof across every future composition or
replace full prepared-payload validation. Conflicting otherwise-valid defaults retain their separate
explicit-resolution rule, and provider-native schema semantics remain unchanged.

**Requirement-composition parameter wiring is accepted:** reuse literal/reference/object/list
binding forms, with references scoped to the composition's declared parameter payload. `review-access`
can forward `workspace` and `maxResults` to their respective restrictions and supply a literal
operation restriction. No plugin is needed merely to forward or assemble values. Literal payloads
remain data and constituent parameters still validate. Shared value construction does not make
requirements executable graphs, import graph-only sources or ambient caller configuration, supply
authority, or replace constituent support and current enforcement. Supporting names and remaining
declaration fields remain authoring/vocabulary work.

**Requirement constituent collection is accepted:** use a map keyed by locally unique names, each
selecting a requirement contract and parameter binding. `workspace`, `operations` and `resultLimit`
can identify the parts of `review-access`. Reject duplicate keys; repeated contract selections under
different local names remain distinct occurrences and may carry different parameters. Local names
support diagnostics/inspection without becoming contract identities or new runtime objects. Map
order does not sequence enforcement or change the obligation to satisfy every applicable constituent.
Container/field spellings and exact contract-reference syntax remain separate.

**Requirement recursion boundary is accepted:** v0 composition is finite and acyclic. Reject direct
and indirect cycles between resolved composed requirement definitions during validation, including
cycles through declared dependencies. Independent reuse remains allowed with distinct parameters
and obligations; repeated aliases/local names alone do not establish recursion. This concerns
requirement-definition composition, not the topology of live Conduits, recursive data schemas or
package dependencies generally. Passing validation does not prove endpoint support or fulfillment.
Exact diagnostics and expansion/resource limits remain implementation work.

**MCP invocation placement is accepted:** use separate MCP request metadata for Conduit context,
keeping each operation's domain input schema and ordinary tool arguments intact. A directory
operation receives `path`; operation and owning-instance identifiers accompany the call outside
those arguments. The responsible service establishes or verifies authenticated context and current
authority; metadata does not confer permission. Plain upstream MCP services remain adapter-backed
without implementing this profile or automatically receiving managed context. Graph-visible
metadata keeps its declared read-only boundary. Exact profile keys, context fields and discovery encoding
remain separate. See the [MCP research follow-up](../../research/language-protocol-foundations/README.md#6-mcp-supplies-interaction-not-all-of-conduits-semantics).

**Deferred-work baseline is accepted:** ordinary MCP tools return a retained operation reference
and provide authorized inspection of progress and outcomes. An Interface can start an index build,
receive acceptance and inspect it later without Tasks support. Clients still need the published
inspection contract. Negotiated task/progress extensions may expose that same work without becoming
mandatory or dispatching it again. Finite calls may complete directly; acceptance is not completed
success, and a reference grants no authority. Domain recovery and retained evidence remain separate
from transport handles. This adds no Agent job-submission capability. Exact tool/result names and
extension mapping remain authoring work.

**Per-request profile requirements are accepted:** each managed request identifies its Conduit
profile revision and required features, without a required prior negotiation agreement. Clients/adapters
supply the metadata; the receiver checks support before acting. Reconnecting to inspect an index
build addresses the same operation with profile information supplied again. Discovery may be cached,
but neither cached declarations nor metadata grants authority or permits dropping required semantics.
Instance/operation records, authentication, current permissions, selected artifact checks and
applicable MCP rules remain effective. Exact profile identifiers, feature fields and discovery
encoding remain open; plain upstream services still use adapters.

**Human-interaction wait ownership is accepted:** retain a typed interaction reference and safe
pending-state information under the managed operation. An authorized Interface attachment can
rediscover the wait after the original frontend closes. For example, a password-manager unlock wait
appears in a reopened terminal; the adapter offers its still-valid flow or reports that a new
interaction is needed under its recovery contract. Retention does not extend challenge validity,
transfer identity, expose credentials, prove completion or guarantee arbitrary suspended-code
recovery. Current authority and sensitivity rules still apply. Exact interaction schemas, MCP
presentation and expiry/replacement mechanics remain authoring work.

**Observation ordering is accepted:** retained feeds scope ordering and resume cursors to each
declared source. The Interface combines feeds without a global event-order guarantee, preserving
local order and known causal links rather than inferring causality from timestamps. A source feed
is a declared observation boundary, not automatically a process or object. Sampled metrics and
latest-state views need not become lossless histories; authoritative state ownership and current
access remain unchanged. Exact cursor encoding, pagination, transport and remaining delivery mechanics
remain open. See the
[observation research follow-up](../../research/OBSERVABILITY.md#identity-and-causality).

**Snapshot-to-feed handoff is accepted:** the managed state-tracking capability pairs a source
snapshot with a resume position for subsequent retained changes. A Resource becoming ready between
the snapshot and opening the feed must be covered by catch-up, or continuity must be reported as
unestablished. If resumption is unavailable, report the gap and need for fresh state rather than
silently jumping to the latest position. This is a source-level guarantee, not a global snapshot,
lossless metric history or exactly-once delivery promise. Weaker/native sources disclose coverage;
current access and explicit-purge rules remain unchanged. Exact cursor encoding, pagination and
transport remain authoring work.

**Type-contract scope is accepted:** extend the Resource model to templates/instances in all five families: each
selects one versioned type contract for its specialized configuration and supported behavior. For
example, a Resource selects a local-directory type, while an Agent selects a native-agent type.
The family still supplies its fundamental role and invariants; the type cannot override ownership,
authority or fixed-definition rules. Multiple plugin contributions can fulfill the selected type,
without a primary implementation. Type-contract versions remain independent of the common object
contract and implementation releases. This chooses a shared structural model, not final type
names, grouping for the other families, field spellings, a type catalog or another object family.
See the [Experiment 001 contract guide](https://github.com/shrav-k/conduit/blob/7927d68ce4eea16ba54f45ef92387fc43f7b37fe/experiments/001-shared-access/CONTRACTS.md#draft-syntax).

The accepted source inventory, input root, named runtime-target value and output-selection behavior
need no further ruling merely to rename their fields. The composition walkthrough preserves these
boundaries, with protected-result validity explicit under its contract. Exact target-reference
encoding and the contents of each runtime declaration remain open. Start/wait operation granularity follows
each operation's declared guarantees rather than a universal split. These are concrete authoring
seams for concrete schemas and implementation, not unresolved conceptual review or blockers to
the behavior/structure documentation merge.

The package-to-instance review covered four connected batches, which also organize the later
schema and conformance implementation work:

1. **Identity and publication:** API/type/capability/requirement versions, schema profile, package identity,
   typed contents/dependencies, artifact identity and reference syntax within the accepted package scope. A mandatory primary plugin
   is not part of the model; `spec.plugins` and `{plugin, name}` are still proposals.
2. **Templates and composition:** contribution/default binding, setup/runtime/control declarations,
   graph inputs/public results, requirement parameters, Pool-local dependencies and provision/use intent.
3. **Managed invocation and observation:** MCP profile negotiation, operation correlation, authenticated
   caller/participation, protected handles, grant/requirement evidence, control outcomes, lifecycle progress
   and observation coverage. Preserve extensibility rather than force a closed enum.
4. **Conformance and fixtures:** capability claims, independently consumable tests, rejection, partial
   failure and uncertain-effect cases; the paper examples and planned fixtures now reflect accepted behavior.

**Walkthrough migration:** Experiment 001 now uses explicit binding forms, JSON Pointer selection,
captured configuration, declared identity metadata in place of unrestricted `self`, and a named
runtime collection. Its uniform type selection follows the accepted ruling above. The Pool's
composition payload is explicitly literal data interpreted by its operation contract; it is not
another graph reference resolver. The guide also distinguishes normal Interface management from
the root's foundational setup/recovery and includes the accepted source snapshot/feed handoff.
Experiment 001 and the history package walkthrough now pass prepared inputs through explicit
startup operations selecting named runtime targets, removing the older implicit runtime-input
wiring. Their fictional shared startup operation illustrates existing semantics, not a new core export.
These paper-fixture changes do not constitute runtime execution or passed conformance.

## Implementation details, not new conceptual rulings

Crate/module boundaries, helper grouping, the initial transport, SQLite tables/durability settings,
retry budgets, parser/tool choices and website generator remain engineering proposals. Choose them
against accepted contracts and evidence; elevate a choice when it changes a public guarantee, support
promise or user-visible scope. No runtime implementation starts during this migration. The
[implementation packet](../../implementation/README.md) preserves the candidates.

## Explicitly later scope

Do not convert these into blockers for the first slice: a second complete runtime; multi-machine
support claims before validation; multi-user/differentiated Interface permissions; offline grants;
graph loops/dynamic fan-out; in-place instance-definition upgrades; marketplace or automatic
cross-language dependency resolution; automatic wake; structured asynchronous work jobs; advanced
memory sharing/publication; general automatic catalog refresh; ownership transfer; or a Pool-owned
authority store. Existing native/isolated execution choices and testing obligations remain accepted;
stronger isolation claims require their own evidence.

## Closed and preserved

- [Conceptual review](CONCEPTUAL.md): C1 capability conformance, C2 Resource grants plus
  caller restrictions and C3 retirement are complete.
- [Implementation review](IMPLEMENTATION.md): I1–I6 and I7's contract-authority/composition
  boundaries are complete; I7's syntax was deliberately deferred.
- [Collaborator comparison](../collaborator-proposals/REVIEW.md): both PRs merged; accepted refinements
  are in the reference and original dissent remains attributed in the archive.
- [Migration record](../migrations/2026-11-protocol-framing.md): source accounting and checks, not another decision ledger.

After a ruling, update the active contract and examples together, then remove its pending status here.
Preserve intentional reversals and rationale; file moves and code convenience cannot decide them.

---

## Preserved authoring index

The previous cumulative index is preserved below from the same `82e87cd` baseline,
[original file](https://github.com/shrav-k/conduit/blob/82e87cd18eeb000f5277158e8dd3e0cae100eb1c/docs/proposals/authoring-syntax/README.md).
Its title is omitted and relative links are relocated; its full body remains historical evidence.

Start at [Author with the protocol](../../authoring/README.md) for the accepted semantic path.
Remaining review work is consolidated in [REVIEW](../../REVIEW.md); this packet preserves examples
for later syntax discussion and adds no second active question queue.

The [vocabulary boundary](../../protocol/MODEL.md#vocabulary-review-boundary) now makes all
Conduit-owned names below the five object families provisional, including earlier selected field
spellings. Keep using them consistently while reviewing logic and structure; naming has a separate
future PR and is not a prerequisite for these walkthroughs.

Status: **behavior/structure review complete; concrete schemas and supporting vocabulary remain proposals**,
2026-11-04. The [current review](../../REVIEW.md#behavior-and-structure-merge-checkpoint) records the
merge boundary and follow-up work; closure does not approve every illustrative field. The
[conceptual closure review](CONCEPTUAL.md) is complete. The published-contract and no-primary-plugin
rulings remain accepted. Templates now use one named dependency map and alias-plus-export references
for setup, runtime and controls, with kind/contract checks and no primary alias. The specific
`spec.plugins` / `{plugin, name}` spellings remain proposals.
Moving these examples does not ratify their names, package layout or scoping rules.

All five families now select one versioned type contract for specialized configuration and supported
behavior, while preserving their family invariants. Multiple compatible plugin contributions can
fulfill that type without a primary implementation. This extends the accepted Resource model;
the example catalogs, grouping for other families and field spellings remain provisional.

Reusable requirement compositions use the literal/reference/object/list binding forms to pass
typed parameters to constituents. References are scoped to the composition's declared parameter
payload, without graph execution sources or ambient caller configuration. No forwarding/assembly
plugin is needed; constituent validation, support, current authority and enforcement remain separate.
This shares value construction without making requirements executable graphs. Constituents form a
map keyed by locally unique names, each selecting a requirement contract and parameter binding.
Reject duplicate keys, while allowing repeated contracts under distinct names with different values.
Map order does not sequence enforcement. Names support inspection without creating runtime objects;
exact field spellings and contract-reference encoding remain open.
V0 requirement composition is finite and acyclic. Reject direct/indirect definition cycles during
validation while preserving independent reuse and its distinct parameters. This does not prohibit
cycles in live Conduit connections, recursive data schemas or package dependencies generally.

Conduit invocation context uses separate MCP request metadata; ordinary operation arguments and
their domain input schemas remain intact. The responsible service establishes or verifies trusted
context; supplied metadata cannot grant authority. Plain upstream MCP services remain adapter-backed.
Each managed request identifies its profile revision and required features, without a required prior
negotiation agreement. Discovery may be cached; current support and authority checks remain effective.
Exact profile fields and discovery encoding remain open.

Deferred managed work has an ordinary-MCP-tool baseline: acceptance returns a retained operation
reference, and authorized inspection retrieves progress/outcomes. Negotiated task/progress extensions
may expose the same operation without duplicate dispatch. Clients still need the published inspection
contract; acceptance is not completion. Exact tool names and result encoding remain open.

Human-interaction waits retain typed references and safe pending state under their managed operation.
An authorized Interface attachment can rediscover a wait after the original frontend closes; adapter
contracts determine whether its challenge is usable or a new interaction is needed. Retention does
not expose credentials, extend validity or prove completion. Exact presentation and fields remain open.

Retained observation feeds define ordering and resume cursors within each declared source. The
Interface combines feeds and known causal links without promising a global order or inferring it
from timestamps. Managed state snapshots pair with a resume position for subsequent retained
changes, covering the handoff to the feed or explicitly reporting a continuity gap and need for
fresh state. This is not a global snapshot, exactly-once delivery promise or change to explicit-purge
rules. Weaker sources disclose coverage; exact encoding, pagination and transport remain open.

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
Within a supplied object, including `{}`, omitted child properties receive unambiguous applicable
defaults. Supplied values still validate without fallback; whole-object defaults do not replace a
supplied object, old settings are not merged, and empty lists stay empty. Input preparation preserves
retained producer outputs and provider-native schema semantics.
V0 defers branch-dependent defaults in Conduit configuration/input schemas. Report unsupported
branch-local defaults when accepting a schema, while retaining conditional/alternative validation.
Templates can supply variant-specific values explicitly or use ordinary defaults outside the
alternatives; composed-default conflict rules and provider-native semantics remain unchanged.
Schema acceptance also rejects defaults provably invalid against their own schema after supported
preparation, without requiring proof across every future composition. Actual prepared payloads still
validate, and conflicting otherwise-valid defaults retain their explicit-resolution rule.
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
| [AUTHORIZATION](../../proposals/authoring-syntax/AUTHORIZATION.md) | Accepted requirements/grants illustrated with unapproved representation and enforcement details, moved here from the runtime packet. |
| [AUTHORING](../../proposals/authoring-syntax/AUTHORING.md) | Small-handler, managed-capability, code-optional MCP and composite-operation author journeys; illustrative APIs. |
| [CONTRACTS](../../proposals/authoring-syntax/CONTRACTS.md) | Proposed schemas, graph/output bindings, operation context and managed MCP mappings. |
| [PACKAGE_WALKTHROUGH](../../proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md) | One history package from publication through setup/runtime/controls, applying accepted package scope while leaving binding/default syntax open. |

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
