# Current review

Updated: 2026-11-03, after the protocol-documentation migration. This is the only active review queue.
The five-family model and completed conceptual/implementation rulings remain accepted in the
[protocol reference](protocol/README.md). The migration did not settle new semantics or approve syntax.

The post-migration review is complete. M1–M3 and the initial owner decisions are settled or explicitly
deferred below. Early delivery stays lightweight; contexts fit the existing Resource/Pool/Conduit
model. The next review is the concrete authoring/conformance walkthrough, preceded by the requested
[language/protocol research](research/language-protocol-foundations/README.md). No syntax was approved by
closing this review, and an open detail in an older report is not automatically another ruling.

## Migration-review rulings

- **M1 accepted:** no backward-compatibility promises or obligation to maintain legacy compatibility
  code. Users can pin a working set. Early delivery prioritizes a working protocol; strict release
  coordination, exact packaging and installers are not prerequisites. Claims about particular
  contracts and retained implementation identity still apply. See [compatibility](protocol/CONFIGURATION.md#release-compatibility).
- **M2 accepted direction/scope:** language/contracts, a minimal reference implementation, and small
  authoring examples with conformance checks using the accepted Experiment 002 scope. Bundled
  capabilities are replaceable and do not become a universal standard library. A documented working
  source checkout can suffice initially. Individual package contents and artifacts will be refined
  with the authoring slice; no detailed catalog or release format was selected. See
  [initial delivery](implementation/README.md#delivery-and-evidence).
- **M3 resolved through the existing model:** contexts can be ordinary Resources exposing specific
  plugin capabilities. A Pool owns the Resource, and Conduits supply its permitted access and
  dependencies. No universal Context layer or owning-context hierarchy is added. A filtered Conduit
  suffices when no separate Resource behavior/lifecycle is needed. Record identity, revisions,
  provenance and sharing remain capability contracts; exposed source records retain their ownership.
  See [memory and evidence](protocol/RESOURCES.md#retained-memory-and-evidence).

## Owner decisions before public distribution

- **License — accepted:** Apache-2.0 for original Conduit documentation, contracts and reference
  code. The official [LICENSE](../LICENSE) is included; reproduced third-party material retains its
  own terms and attribution. No separate CLA or DCO requirement was selected. Any additional
  contribution procedure remains open if needed; see [license scope](../CONTRIBUTING.md#license-and-attribution).
- **Public identities — deferred:** keep the current `shrav-k/conduit` repository location while
  protocol work continues. Organization selection/transfer, domains and permanent package naming
  remain for the owner to choose later. The accepted one-main-repository direction is unchanged;
  provisional examples and directory names do not establish permanent protocol identities.
- **Change and release authority — accepted:** the repository owner retains final authority over
  protocol direction. Collaborators propose/review changes; merges and releases use existing repository
  permissions and explicitly appointed maintainers/delegated responsibilities. Existing task
  authorizations remain valid. No fixed quorum or formal governance process is required initially.
  See [project ownership](../CONTRIBUTING.md#project-ownership-and-publication). Public support/security
  contact details and additional contribution procedures are deferred until needed for distribution.

These are project-owner decisions, not reasons to reopen the five families or invent another runtime
authority model. The [repository proposal](archive/reviews/REPOSITORY-2026-11-02.md) retains options and rationale.

Apache-2.0 was selected for permissive reuse with an explicit, scoped contributor patent grant.
The [official license text](https://www.apache.org/licenses/LICENSE-2.0.html) was checked on
2026-11-02. Licensing does not change the no-backward-compatibility rule or select a release process.

## Refinements to carry into concrete examples

These seams need complete examples and conformance cases. Existing guarantees constrain the answer;
return for a conceptual ruling only if an example exposes a real behavior choice.

| Seam | Preserve while refining | Source |
| --- | --- | --- |
| Controls spanning multiple settings/effects | Last confirmed configuration, explicit uncertainty, revision checks and target coordination. No implied atomicity/rollback; specify what each handler can confirm after partial external effects. | [Configuration updates](protocol/CONFIGURATION.md#configuration-updates), [controls](protocol/LIFECYCLE.md). |
| Composite results, cancellation and recovery | Deliberate public outputs, retained effects/evidence, valid child progress, retirement duties and other participants. Custom recovery hooks/evidence formats remain unselected. | [Composition](protocol/MODEL.md#authoring-concepts-beneath-the-object-families), [retirement](protocol/LIFECYCLE.md#retirement-and-unregistration). |
| Connection/composite readiness | Required preparation gates, independent later work, per-consumer participation and ready Pool Resources are distinct. Describe observable partial state without one universal lifecycle enum. | [Pools](protocol/POOLS.md), [Conduits](protocol/CONDUITS.md). |
| Independent MCP access and client enrollment | Familiar provider tools plus current authority, caller identity and evidence. A directly accessible component still needs verifiable authority; no offline grants or bypass is implied. | [Independent components](protocol/MODEL.md#plugins-and-independently-accessible-mcp-components). |
| Foundational repair and credential interaction | Locally verifiable owner recovery, retained identities/progress and adapter-guided enrollment. No automatic reset, secret logging or duplicate execution when state is unknown. | [Interface workflow](protocol/MODEL.md#interfaces-and-the-user-workflow), [authentication](protocol/POOLS.md#shared-access-and-authentication). |

## Later authoring and conformance pass

The [research comparison](research/language-protocol-foundations/README.md) and
[worked cases](research/language-protocol-foundations/WALKTHROUGHS.md) now inform this pass.
They recommend separating contract imports, graph value bindings and authorized live connections;
they do not decide reference spellings, a new expression language or a runtime dependency.
Twenty [library probes](../experiments/003-language-probes/README.md) expose validation pitfalls;
they are not Conduit conformance results. Research recommendations remain in these four batches,
without reopening settled concepts or creating a second question queue.

The [authoring packet](proposals/authoring-syntax/README.md) preserves the examples for review.
The authoring review accepts [package-local scope and explicit dependency bindings](protocol/MODEL.md#package-scope-and-public-bindings):
caller aliases do not override package internals, and replacements use explicitly exposed bindings.
Replaceable dependencies may declare default implementation bindings. Compatible explicit replacements
take precedence; invalid selections report errors without fallback. These decisions do not accept
`spec.plugins`, `{plugin, name}` or an import/export file layout.
V0 imports also require an export's explicit support for the requested contract identity and exact
version, alongside existing compatibility checks. Cross-version mappings are deferred; implementation
release versions remain independent, with no obligation to maintain old contracts or compatibility code.
The [configuration foundation](protocol/CONFIGURATION.md#configuration-language) is accepted:
constrained YAML 1.2 for human authoring, equivalent JSON input/interchange, and JSON Schema
2020-12 for data contracts, with one shared value model. Conduit defines lifecycle, authorization
and other behavioral semantics separately. The [comparison](research/language-protocol-foundations/FORMATS.md)
retains the rationale; detailed schema features and reference syntax remain open.

The [schema-resolution boundary](protocol/CONFIGURATION.md#schema-resolution) is accepted:
resolve from the selected package and explicitly supplied or declared schema dependencies;
report missing required definitions without automatic network fetching during validation.
Explicit acquisition beforehand remains possible, and exact reference syntax is open.

The [schema-keyword rule](protocol/CONFIGURATION.md#schema-keyword-acceptance) is accepted:
reject unrecognized keywords in Conduit-authored schemas. Reuse standard descriptive annotations
where applicable, with additional custom metadata in one dedicated map of namespaced entries.
Custom annotations supply optional tooling hints, not validation, authority or runtime behavior.
The map keyword, namespace syntax, value schemas and external MCP adapter details remain open.

The [default-conflict rule](protocol/CONFIGURATION.md#schema-default-resolution) is accepted:
conflicting applicable defaults for an omitted configuration field require an explicit valid
value or a correction to the defaults. Schema order does not choose a winner, and all applicable
validation constraints still apply. Exact default-resolution mechanics remain open.

The [format rule](protocol/CONFIGURATION.md#schema-keyword-acceptance) is accepted: `format` in
Conduit-authored schemas is an enforced validation requirement. V0 selects all 19 standard formats
defined in JSON Schema 2020-12, with offline syntactic checks and conformance tests before support
claims. Unsupported formats are reported; plugin-defined validators are deferred. Custom operations,
Resource contracts and requirement checks remain extensible. Provider MCP dialect semantics remain unchanged.

The [provider MCP schema boundary](protocol/CONFIGURATION.md#provider-mcp-schemas) is accepted:
preserve the declared dialect's semantics without automatically imposing Conduit's stricter
authoring profile. Allowed descriptive annotations do not alone reject a tool; unsupported
required validation features are reported without stripping constraints or claiming enforcement.
Exact adapter handling and conformance cases remain open.

The [package-artifact rule](protocol/CONFIGURATION.md#implementation-selection) is accepted:
resolved artifacts retain a content digest alongside package identity and, when released, release
identity, covering contracts, graphs, templates and any bundled code, including code-free packages.
The digest identifies content, not publisher trust or remote service code. Hash algorithms,
bundle boundaries and concrete fields remain open.

The [edit-mode/release boundary](protocol/CONFIGURATION.md#implementation-selection) is accepted:
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

The [package-content rule](protocol/CONFIGURATION.md#implementation-selection) is accepted:
explicitly declare included files/directories, with directory and pattern rules available.
Do not capture the whole working directory by default. Unselected files do not contribute to
the artifact or its digest. Exact syntax, descriptor handling and internal symlink representation remain open.

The [capture-boundary rule](protocol/CONFIGURATION.md#implementation-selection) is accepted:
one explicit source directory bounds included paths and symlink targets. Outside targets are
reported rather than silently copied. Authors may stage shared files, choose a wider explicit
directory or use separate package dependencies. Resource paths and authorized runtime access
are unaffected; internal symlink representation remains open.

The proposed same-name/release multi-artifact catalog is superseded by edit mode plus deliberate
versioned releases. Different edit-mode captures do not become multiple released contents under one
version. Protecting active execution does not require an archive of every development build.
Contract checks and no silent substitution remain applicable; public registry mechanics remain open.

The [graph-input boundary](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: v0 inputs use
literals and typed references to configuration or earlier outputs. Plugin operations perform
calculations and transformations; inline expressions in bindings are deferred. Output bindings
imply dependencies as already agreed. Exact reference syntax remains open, and branching/loops
still remain inside plugins initially.

The [reference-recognition boundary](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: recognize
references only in explicitly designated binding positions. Literal payloads, ordinary configuration
and plugin results are not recursively reinterpreted as graph instructions, even when they resemble
reference notation. Resolving a reference retrieves data, not another instruction to evaluate.
Exact notation and literal disambiguation remain open.

The [structured-input boundary](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: graphs can
explicitly assemble object fields and list items from literals, references and nested assemblies
without a plugin merely to build that structure. Output references still imply dependencies; literal
payloads remain data. Calculations and other transformations remain plugin operations. No automatic
merging, string manipulation, loops or inline expressions are introduced; constructor syntax is open.

The [input-validation boundary](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: catch provable
wiring/type errors before execution, then validate each step's resolved inputs before dispatch.
No static proof that every possible upstream output satisfies the receiving schema is required.
A producer permitting integers 1–100 can feed a consumer requiring 1–10, with 5 accepted and 50
rejected before the receiving step runs. Required schema support remains mandatory; rejection does
not undo earlier effects. Exact checking algorithms and diagnostics remain open.

The [output-validation boundary](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: validate each
declared plugin output against its producer's contract before downstream use. Withhold invalid
output and report a contract error while preserving effects and uncertainty. A malformed descriptor
does not establish that creation had no effect or is safe to replay. Producer-output and receiving-input
checks remain separate; provider-native schema interpretation is unchanged. Exact result/error
representation remains open.

The [graph-failure scheduling rule](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: independent
branches continue when their own prerequisites remain satisfied. Unmet prerequisites block affected
work, required unfinished setup remains incomplete, and current authority, coordination, cancellation
and stop rules still apply. Preserve effects and dispatched outcomes without automatic retry or rollback.

The [explicit graph-cancellation rule](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: stop
starting ordinary steps and request cancellation of in-flight child operations where supported.
Track actual outcomes, effects and uncertainty, and perform required cleanup. Preparation's existing
attachment-release duties still apply; an already-started runtime follows its instance lifecycle.
A request is not proof of stopped work, rollback or completed cleanup. Exact propagation and
transport mechanics remain open.

The [configuration-capture rule](protocol/LIFECYCLE.md#lifecycle-protocol) is accepted: configuration
references within one graph execution use values captured when it begins. Later executions capture
the then-current settings. The instance still has one stored configuration map; captured inputs do
not establish current runtime state. Authority, dependency validity and control-application revision
checks remain current, and retry/recovery must revalidate retained results. Exact representation and
recovery mechanics remain open.

The [reusable-operation input boundary](protocol/MODEL.md#authoring-concepts-beneath-the-object-families)
is accepted: caller configuration is supplied through declared typed inputs, with no implicit
inheritance of the caller's configuration namespace. A template graph passes its captured limit
into the reusable operation's `limit` input. Scoped invocation context and explicitly authorized
Resource access remain available under their contracts. Exact input/context syntax remains open.

The [package entry-document rule](protocol/MODEL.md#authoring-concepts-beneath-the-object-families)
is accepted: one explicit entry document declares public exports and dependencies, with definitions
inline or referenced in package-local files. It may be handwritten or generated; inspecting it
requires no plugin execution or directory-based export discovery. Provider-native catalogs remain
dynamic. Filenames, fields and reference syntax remain open. The
[complete package walkthrough](proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md) illustrates both
single-file and multi-file declarations.

The [public export-name rule](protocol/MODEL.md#package-scope-and-public-bindings) is accepted:
names are unique across contribution kinds within a package, including inline and referenced
definitions. Selecting the package and export name identifies one contribution; resolution still
checks its expected kind and contract. Internal names, contract identities and registry object
naming remain separate. Exact name grammar and reference notation remain open.

The [explicit re-export rule](protocol/MODEL.md#package-scope-and-public-bindings) is accepted:
a package may expose a declared dependency's public contribution under its own export name without
copying or wrapping it. Preserve the original definition, contract, internal reference scope and
resolved dependency identities. Export-name uniqueness and kind/contract checks still apply; the
alias adds no behavior or access. Exact declaration and resolution notation remain open.

The [template implementation-selection structure](protocol/MODEL.md#authoring-concepts-beneath-the-object-families)
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

The shared format set is accepted above. The [format-set follow-up](research/language-protocol-foundations/FORMATS.md#standard-format-set-follow-up)
retains the rationale; the decision does not select UUID object IDs, graph-reference notation or
network access, and no runtime support is claimed by recording it.

The [portable integer boundary](protocol/CONFIGURATION.md#numeric-interoperability) is accepted:
Conduit-declared integer configuration, inputs and outputs in v0 use −9,007,199,254,740,991 through
9,007,199,254,740,991. A contract needing a larger exact integer can explicitly use a decimal string;
there is no automatic conversion or rounding into range. Core revision/ID encodings and
provider-native schemas remain separate. The [number-portability follow-up](research/language-protocol-foundations/FORMATS.md#number-portability-follow-up)
retains the rationale.

The [fractional-number boundary](protocol/CONFIGURATION.md#numeric-interoperability) is accepted:
ordinary fractional configuration, inputs and outputs use finite binary64 (double-precision)
semantics, including normal representational rounding. Exact decimal quantities use explicit
decimal-string or scaled-integer contracts; integer limits remain strict. This refines the earlier
blanket precision wording. The [follow-up](research/language-protocol-foundations/FORMATS.md#fractional-number-follow-up)
retains the rationale; detailed validation/serialization fixtures remain engineering and conformance
work, with any newly exposed public behavior choice returned for review.

The [batch submission boundary](protocol/CONFIGURATION.md#batch-configuration-submissions) is accepted:
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
`/participants/0/name` selects a field in the first array item. The source selector, reference wrapper
and exact field names remain open. This does not select object identities, authorize Resource access
or evaluate literal strings; registration batches still require complete supplied definitions.

Next, settle unresolved selections. Recommend reporting a reference-resolution error and withholding
the consuming step when a pointer has no target, rather than silently substituting null or treating
the input as omitted to obtain a default. For example, `/connection/endpoint` cannot supply an
endpoint if the selected output has no such member. An existing null value remains a selected value
subject to the receiving schema. This handling is pending; the alternative treats a missing selection
as omitted input under declared default/optional-field rules. Existing independent-branch and
effect-preservation rules remain unchanged. Exact diagnostic fields remain authoring work.

Continue the complete package-to-instance walkthrough through four connected batches:

1. **Identity and publication:** API/type/capability/requirement versions, schema profile, package identity,
   typed contents/dependencies, artifact identity and reference syntax within the accepted package scope. A mandatory primary plugin
   is not part of the model; `spec.plugins` and `{plugin, name}` are still proposals.
2. **Templates and composition:** contribution/default binding, setup/runtime/control declarations,
   graph inputs/public results, requirement parameters, Pool-local dependencies and provision/use intent.
3. **Managed invocation and observation:** MCP profile negotiation, operation correlation, authenticated
   caller/participation, protected handles, grant/requirement evidence, control outcomes, lifecycle progress
   and observation coverage. Preserve extensibility rather than force a closed enum.
4. **Conformance and fixtures:** capability claims, independently consumable tests, rejection, partial
   failure and uncertain-effect cases; revise Experiment 001 YAML and Experiment 002 fixtures explicitly.

## Implementation details, not new conceptual rulings

Crate/module boundaries, helper grouping, the initial transport, SQLite tables/durability settings,
retry budgets, parser/tool choices and website generator remain engineering proposals. Choose them
against accepted contracts and evidence; elevate a choice when it changes a public guarantee, support
promise or user-visible scope. No runtime implementation starts during this migration. The
[implementation packet](implementation/README.md) preserves the candidates.

## Explicitly later scope

Do not convert these into blockers for the first slice: a second complete runtime; multi-machine
support claims before validation; multi-user/differentiated Interface permissions; offline grants;
graph loops/dynamic fan-out; in-place instance-definition upgrades; marketplace or automatic
cross-language dependency resolution; automatic wake; structured asynchronous work jobs; advanced
memory sharing/publication; general automatic catalog refresh; ownership transfer; or a Pool-owned
authority store. Existing native/isolated execution choices and testing obligations remain accepted;
stronger isolation claims require their own evidence.

## Closed and preserved

- [Conceptual review](archive/reviews/CONCEPTUAL.md): C1 capability conformance, C2 Resource grants plus
  caller restrictions and C3 retirement are complete.
- [Implementation review](archive/reviews/IMPLEMENTATION.md): I1–I6 and I7's contract-authority/composition
  boundaries are complete; I7's syntax was deliberately deferred.
- [Collaborator comparison](archive/collaborator-proposals/REVIEW.md): both PRs merged; accepted refinements
  are in the reference and original dissent remains attributed in the archive.
- [Migration record](archive/migrations/2026-11-protocol-framing.md): source accounting and checks, not another decision ledger.

After a ruling, update the active contract and examples together, then remove its pending status here.
Preserve intentional reversals and rationale; file moves and code convenience cannot decide them.
