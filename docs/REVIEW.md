# Current review

Updated: 2026-11-02, after the protocol-documentation migration. This is the only active review queue.
The five-family model and completed conceptual/implementation rulings remain accepted in the
[protocol reference](protocol/README.md). The migration did not settle new semantics or approve syntax.

There is no identified contradiction requiring us to reopen the overall model. Remaining work falls
into three broader discussions, publication decisions owned by the project, and the later authoring/
engineering pass. An open detail in an older report is not automatically another ruling.

## Discuss before the syntax PR

| Topic | What is open | What is already fixed / useful next example |
| --- | --- | --- |
| **M1 — Early delivery and compatibility** | Discuss minimal release contents alongside M2. Formal release coordination, exact packaging and delivery tooling are not prerequisites for the early working protocol; a coordinated distribution remains an option rather than a selected requirement. | **Accepted: no backward-compatibility promises**, initially or automatically at maturity; users can pin a working set. No legacy compatibility code is required solely to support old releases. Particular-contract conformance and retained implementation identity still apply. [Compatibility rule](protocol/CONFIGURATION.md#release-compatibility), [early delivery priority](implementation/README.md#delivery-and-evidence). |
| **M2 — Thin public proof and supported package boundary** | Which minimal capability contracts and example contributions belong in the first public conformance kit/reference distribution, and which are optional ancillary packages? Define support without turning the candidate Resource catalog into a mandatory standard library. | Declared-capability conformance, independent client/plugin proof and Experiment 002's composition/recovery scope are accepted. Compare a tiny authored operation, a declaration-only MCP binding and a template-only package. Decide the promised surface; leave export names/manifests for syntax. [Conformance](protocol/CONFIGURATION.md#declared-capability-conformance), [planned proof](../experiments/002-runtime-plugins/README.md). |
| **M3 — Memory content contexts within Pool-owned Resources** | The memory contract retains “task, Pool or personal collection” as examples of a record's owning context. Clarify how those contexts are represented inside a memory Resource and how sharing/derived-source checks refer to them. Is a context domain data managed by that capability, and which ownership/access boundaries must it expose? This is not yet a ruling. | A Resource has exactly one owning Pool; content ownership does not add another Resource owner or object family. Direct live read-only sharing, revision checks, whole-record delivery and source dependencies remain accepted. Walk through a personal note shared into project knowledge, then a derived summary. [Memory contract](protocol/RESOURCES.md). |

M1 and M2 complete the repository/publication framing; M3 is a semantic mapping left visible in the
existing memory text. None blocks this documentation migration. Resolve them collaboratively before
using their answers to select public schemas or build a package distribution.

## Owner decisions before public distribution

- **License and contribution terms.** No repository license was available to carry forward. Select
  these deliberately; the migration adds no license, CLA or DCO requirement.
- **Public identities.** Choose the GitHub organization/transfer, package naming authority and any
  domain before publishing permanent identifiers. Directory names do not decide protocol identities.
- **Change and release authority.** Identify who approves protocol changes, maintains supported
  packages and publishes releases. Define the initial support/security contact when distribution begins.

These are project-owner decisions, not reasons to reopen the five families or invent another runtime
authority model. The [repository proposal](archive/reviews/REPOSITORY-2026-11-02.md) retains options and rationale.

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

The [authoring packet](proposals/authoring-syntax/README.md) is parked until the discussions above.
Review four connected batches rather than restarting a long list of isolated questions:

1. **Identity and publication:** API/type/capability/requirement versions, schema dialect, package identity,
   typed contents/dependencies, artifact identity and export/reference scope. A mandatory primary plugin
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
