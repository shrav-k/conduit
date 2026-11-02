# Current review

Updated: 2026-11-02, after the protocol-documentation migration. This is the only active review queue.
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

The next ruling concerns unrecognized keywords in Conduit-authored data schemas. The proposed
rule rejects them when accepting a contract, while providing an explicitly declared mechanism
for custom annotations. For example, a misspelled `minimum` must not silently lose its constraint.
This is an authoring-profile choice, not an automatic rejection rule for external MCP catalogs;
the annotation mechanism and any future validation extensions require their own defined semantics.

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
