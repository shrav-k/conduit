# Author with the protocol

Conduit's extension boundary is the product: an author should be able to define a new capability,
implement it, compose it into a template and inspect the resulting instance without editing runtime
dispatch. Start from the [overview](../OVERVIEW.md) and [accepted extension model](../protocol/MODEL.md#authoring-concepts-beneath-the-object-families).

This is a semantic authoring path, not a runnable tutorial. Package files, export references and MCP
profile fields are still proposals. The [current review](../REVIEW.md) tracks that boundary; the
[syntax packet](../proposals/authoring-syntax/README.md) preserves concrete examples for the later PR.
The [language/protocol comparison](../research/language-protocol-foundations/README.md) supplies
primary references and worked cases for choosing those representations.

Author configuration and data schemas in constrained YAML 1.2 or equivalent JSON. Both use
one value model; JSON Schema 2020-12 supplies data validation, while Conduit defines execution,
authority and recovery separately. See the accepted [configuration foundation](../protocol/CONFIGURATION.md#configuration-language).
Schema references resolve from the selected package and explicitly supplied or declared schema
dependencies, without automatic network fetching during validation. Missing required definitions
are reported; dependencies may be acquired explicitly beforehand. The detailed schema profile
and reference syntax remain in review.

## From a new capability to an instance

Consider a small document-search capability used by an Agent. The authoring steps are:

1. **Define the public behavior.** Describe typed inputs/results, related operations, effects,
   failures and recovery. A search capability does not automatically confer storage access or certify
   freshness. Requirements can limit its permitted corpus, operations or result size.
2. **Bind an implementation.** A plugin publishes inspectable contracts and bindings to authored code,
   an existing MCP service, graph-backed operations or a combination. Lightweight handlers are supported
   as the initial author experience; a large SDK and a mandatory primary implementation are not required.
3. **Compose setup.** A finite dependency graph can establish access and prepare search. Output bindings
   imply dependencies. The Resource instance manages ongoing activity; a successful setup graph is not
   a permanent readiness guarantee. An uncertain external effect must be reconciled under its contract.
4. **Declare a Resource template.** Supply the configuration, setup graph, ongoing behavior and allowed
   controls needed to establish repeated instances. Instances retain resolved definitions and exact
   selected implementations; supported controls change confirmed settings within those fixed contracts.
5. **Compose it through a Pool.** The Pool lists named Resource-template entries and binds dependencies.
   Merely listing an entry provisions nothing. Explicit setup provisions a new Resource or uses a
   selected existing one. Storage, search and MCP access retain their separate responsibilities.
6. **Connect and inspect.** A Conduit gives an Agent or Interface permitted participation. Endpoint
   implementations satisfy its declared requirements; Resource dependency grants do not erase caller
   restrictions. The Interface exposes progress, applied configuration, failures and coverage limitations.
7. **Publish and prove it.** A common package envelope can carry contracts, implementation bindings and
   templates, or templates alone. Installing it does not instantiate objects, authenticate a provider or
   grant access. Test success, rejected access and uncertainty/recovery through public contracts.
   Internal unit tests alone do not establish interoperability.

The required contracts are detailed in [Resources](../protocol/RESOURCES.md), [Pools](../protocol/POOLS.md),
[Conduits](../protocol/CONDUITS.md), [lifecycle](../protocol/LIFECYCLE.md) and
[configuration/conformance](../protocol/CONFIGURATION.md). These govern the example; it adds no new ruling.

Package composition uses [package-local scope and explicit dependency bindings](../protocol/MODEL.md#package-scope-and-public-bindings).
A package keeps the meaning of its internal names when imported under a caller's alias. Publish
exports for callers and expose any dependency they may replace explicitly; an unrelated same-named
plugin in the caller cannot change the package's implementation. A replaceable dependency may supply
a default binding; a valid explicit replacement takes precedence, and an invalid replacement reports
an error rather than falling back. Runtime Resource access still follows authorized Conduits.
V0 imports require explicit support for their requested contract identity and exact version, alongside
the existing compatibility checks. Different plugin releases may implement that same contract;
cross-version compatibility mappings are deferred.

## Variants the language must support

| Author's contribution | Existing material to carry forward |
| --- | --- |
| A small operation implemented with code | [Handler and filesystem examples](../proposals/authoring-syntax/AUTHORING.md). |
| A declaration-only binding to a provider MCP service | [Authoring examples](../proposals/authoring-syntax/AUTHORING.md) and [MCP boundary](../protocol/CONDUITS.md). |
| An operation composed from other operations | [Graph/result proposals](../proposals/authoring-syntax/CONTRACTS.md), preserving derived/explicit outputs and recovery. |
| A relationship requirement or composition | [Requirement contracts](../protocol/CONDUITS.md#composable-relationship-requirements) and [custom-requirement experiment](../../experiments/002-runtime-plugins/README.md). |
| A template-only collection | [Common package boundary](../protocol/MODEL.md#authoring-concepts-beneath-the-object-families); exact package fields remain open. |
| A complete managed history capability | [Package walkthrough](../proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md), an illustrative draft rather than an installable package. |

## What the next authoring PR must show

Use one complete package-to-instance journey as the backbone, with declaration-only and template-only
variants. Show authored definitions, reference resolution, the resulting instance, what the Interface
observes, and at least one rejection or uncertain outcome. Review identity, defaults, contract versions,
graphs/results, controls and MCP mappings together.

An example may propose a spelling; only a ruling or approved syntax review makes it a contract.
Carry forward failure/recovery cases rather than replacing them with a happy-path sample. Derive
conformance cases from accepted behavior and keep runtime-specific checks distinguishable.
