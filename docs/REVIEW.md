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
reject unrecognized keywords in Conduit-authored schemas and allow custom annotations through
an explicitly declared mechanism. The annotation syntax and detailed adapter handling of external MCP catalogs
remain separate; annotations do not supply enforcement by themselves.

The [default-conflict rule](protocol/CONFIGURATION.md#schema-default-resolution) is accepted:
conflicting applicable defaults for an omitted configuration field require an explicit valid
value or a correction to the defaults. Schema order does not choose a winner, and all applicable
validation constraints still apply. Exact default-resolution mechanics remain open.

The [format rule](protocol/CONFIGURATION.md#schema-keyword-acceptance) is accepted: `format` in
Conduit-authored schemas is an enforced validation requirement. The shared profile defines
supported formats and unsupported formats are reported rather than treated as satisfied.
The supported set and custom-extension mechanism remain open.

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

The next proposed ruling concerns declared plugin outputs: validate an output against its producer's
contract before making it available to downstream steps, withholding invalid output while recording
the contract error and any effects/uncertainty, or rely only on each downstream input check? The first
option is recommended. A directory-creation operation may have created the directory even if its
returned descriptor is malformed; withholding that descriptor must not imply rollback or safe replay.
This is pending and concerns Conduit-declared outputs, not imposing new schemas on provider-native tools.

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
