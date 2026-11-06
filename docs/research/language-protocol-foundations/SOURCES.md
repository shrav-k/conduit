# Primary sources and reading guide

> Not binding. This research predates the specification in [`spec/`](../../../spec/README.md); where it says accepted, current, SPEC.md or DESIGN.md, it means the documents as they stood before 2026-11-05.

Inspected **2026-11-02**. This guide identifies the source versions and portions used in the
[synthesis](README.md) and [walkthroughs](WALKTHROUGHS.md). Versioned comparisons are deliberate
baselines, not claims that every listed version is the newest release. Moving documentation may
change; fixed document versions and commit links are preferred where available.

[retrievals.json](retrievals.json) records 32 directly retrieved documents, retrieval date,
resolved URL, byte count and SHA-256. Three moving repository documents were re-fetched at their
recorded commits. Hashes identify the response bytes inspected, not a signed release or a
permanent third-party archive. Full source copies are not vendored. Sources inspected through web
retrieval only, including papers and supplemental pages, are listed below without invented hashes.

## Suggested reading order

1. **WIT worlds + Smithy operations:** understand required/provided capability boundaries and
   contract versus implementation/wire identity.
2. **CWL execution model:** understand typed ports, composition and a portable runtime contract.
3. **YANG actions + NMDA:** distinguish controls, confirmed configuration and observations.
4. **TOSCA overview:** compare the entire topology/relationship model, then identify the semantics
   Conduit deliberately does not inherit.
5. **MCP 2026-07-28 + JSON Schema:** verify the actual wire and data substrate before selecting
   package/reference/default syntax.
6. Use the other references to test particular failure cases, not to enlarge the initial language.

## Component and service contracts

| Primary source | Version / status inspected | Sections used and reason |
| --- | --- | --- |
| [WIT specification source](https://github.com/WebAssembly/component-model/blob/fed10b0b823b82553dd8363f16a640816f5eac65/design/mvp/WIT.md) | Commit `fed10b0b823b82553dd8363f16a640816f5eac65`; evolving Component Model design. | Package names, imports/exports, world inclusion, handle ownership and name resolution. Some features are explicitly gated; no blanket implementation-support claim. |
| [WIT overview](https://component-model.bytecodealliance.org/design/wit.html), [worlds](https://component-model.bytecodealliance.org/design/worlds.html) | Moving official guide. | Human-readable explanation of interfaces, imports and composition; underlying isolation belongs to the component execution model. |
| [Smithy model](https://smithy.io/2.0/spec/model.html), [IDL](https://smithy.io/2.0/spec/idl.html) | Smithy 2.0 specification. | Namespaces, shape identities and traits; distinguishes model identity from local spelling. |
| [Smithy service types](https://smithy.io/2.0/spec/service-types.html) | Smithy 2.0. | Operation input/output/errors, service/resource surfaces; do not equate Smithy resources with Pool-owned Conduit Resources. |
| [Behavior traits](https://smithy.io/2.0/spec/behavior-traits.html), [protocol traits](https://smithy.io/2.0/spec/protocol-traits.html), [mixins](https://smithy.io/2.0/spec/mixins.html) | Smithy 2.0. | Idempotency, protocol mapping and reusable declarations. These are separate mechanisms, not one generic merge. |

## Network management and topology

| Primary source | Version / status inspected | Sections used and reason |
| --- | --- | --- |
| [YANG](https://www.rfc-editor.org/rfc/rfc7950.html) | RFC 7950, YANG 1.1, August 2016. | Data model, configuration/state distinction, RPCs/actions/notifications (§§7.14–7.16), feature declarations. |
| [NMDA](https://www.rfc-editor.org/rfc/rfc8342.html) | RFC 8342, March 2018. | §5 datastores, especially intended versus operational information. Does not replace Conduit's accepted last-confirmed configuration model. |
| [NETCONF](https://www.rfc-editor.org/rfc/rfc6241.html) | RFC 6241, June 2011. | §§8.3–8.4 candidate configuration/confirmed commit. Its guarantees have a defined datastore scope. |
| [TOSCA](https://docs.oasis-open.org/tosca/TOSCA/v2.0/cs01/TOSCA-v2.0-cs01.html), [authoritative Markdown](https://docs.oasis-open.org/tosca/TOSCA/v2.0/cs01/TOSCA-v2.0-cs01.md) | Version 2.0, **Committee Specification 01**, 5 December 2024. | §§3.2, 3.5 and 15.1: capabilities/dependencies, topology versus workflow and substitution mappings. This is a named committee specification, not an assertion of a newer ratification status. |

## Execution and composition

| Primary source | Version / status inspected | Sections used and reason |
| --- | --- | --- |
| [CWL Workflow Description](https://www.commonwl.org/v1.2/Workflow.html) | Page identifies v1.2.1; authoring examples use `cwlVersion: v1.2`. | Execution model, requirements/hints, WorkflowStep inputs and public outputs. Important mismatch: nested requirement precedence is not Conduit's all-applicable-restrictions rule. |
| [Tekton Tasks](https://tekton.dev/docs/pipelines/tasks/), [Pipelines](https://tekton.dev/docs/pipelines/pipelines/), [TaskRuns](https://tekton.dev/docs/pipelines/taskruns/) | Moving official docs. | Results, explicit dependencies, missing-result behavior, retries and execution state. No cluster or API version was tested. |
| [Open Workflow DSL](https://github.com/open-workflow-specification/specification/blob/2108908485b2061ccf55a8444a8c48e203aedec4/dsl-reference.md) | Commit `2108908485b2061ccf55a8444a8c48e203aedec4`; examples include DSL 1.0.4. | Reusable functions, task inputs/outputs, errors, protocol calls and MCP binding. Earlier research calls this Serverless Workflow; use the inspected repository identity. The inspected MCP binding defaults to 2025-06-18 and requires initialization. |
| [Amazon States Language](https://states-language.net/spec.html) | Public specification inspected on the research date; page copyright through 2024. | Tasks, retry/catch and input/output processing. No AWS state machine was deployed. |
| [Temporal Activity Execution](https://docs.temporal.io/activity-execution) | Moving official documentation. | Activity task loss/timeouts, retry policies and cooperative cancellation. Documentation review, not a Temporal implementation evaluation. |

## Configuration and validation

| Primary source | Version / status inspected | Sections used and reason |
| --- | --- | --- |
| [JSON Schema Core](https://json-schema.org/draft/2020-12/json-schema-core), [Validation](https://json-schema.org/draft/2020-12/json-schema-validation) | Draft 2020-12 specifications, published revisions dated 16 June 2022. | Dialects/vocabularies, unknown keywords, composition, annotations/defaults and format behavior. JSON Schema draft naming is not an IETF RFC designation. |
| [CUE specification](https://cuelang.org/docs/reference/spec/) | Moving official language reference. | Constraints, unification, disjunction/defaults and definitions. No CUE evaluator was run. |
| [CEL language definition](https://github.com/cel-expr/cel-spec/blob/21ec6f4e52e8d694687ff9ed7408d3eccbb955d7/doc/langdef.md) | Commit `21ec6f4e52e8d694687ff9ed7408d3eccbb955d7`. | Environment/type/evaluation semantics; host extension functions still need specified behavior and resource limits. |
| [Dhall safety guarantees](https://docs.dhall-lang.org/discussions/Safety-guarantees.html) | Moving official documentation. | Total evaluation versus import dependencies/integrity and practical resource limits. Optional-generator precedent only. |
| [OpenTofu provider requirements](https://opentofu.org/docs/v1.11/language/providers/requirements/), [module providers](https://opentofu.org/docs/v1.11/language/modules/develop/providers/) | **v1.11 documentation**, intentionally versioned; newer releases exist. | Global source identities, local names, explicit module requirements and configuration binding. No implicit provider/account inheritance is proposed for Conduit. |
| [OpenTofu references](https://opentofu.org/docs/v1.11/language/expressions/references/), [depends_on](https://opentofu.org/docs/v1.11/language/meta-arguments/depends_on/) | v1.11 documentation. | Unknown values, dependency inference and behavior-only ordering. No OpenTofu engine was run. |
| [Pulumi inputs/outputs](https://www.pulumi.com/docs/iac/concepts/inputs-outputs/), [secrets](https://www.pulumi.com/docs/iac/concepts/secrets/) | Moving official docs. | Dependency-carrying values and secret propagation. Ordinary references and authorized Conduits remain different. |
| [Compose startup](https://docs.docker.com/compose/how-tos/startup-order/), [service reference](https://docs.docker.com/reference/compose-file/services/) | Moving Docker docs. | Started, healthy and successfully completed dependency conditions. No Compose runtime test in this pass. |
| [Kubernetes CRDs](https://kubernetes.io/docs/tasks/extend-kubernetes/custom-resources/custom-resource-definitions/) | Moving official docs. | Structural validation, defaults and CEL rules; this pass reuses the earlier detailed configuration comparison. No cluster test or Kubernetes backend selected. |

Nickel and Jsonnet remain covered by the earlier
[alternative-language comparison](../CONFIGURATION.md#alternative-languages-and-contract-systems).
They were not freshly benchmarked or selected as dependencies in this pass.

## Invocation and events

| Primary source | Version / status inspected | Sections used and reason |
| --- | --- | --- |
| [MCP changes](https://modelcontextprotocol.io/specification/2026-07-28/changelog), [base](https://modelcontextprotocol.io/specification/2026-07-28/basic), [tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools) | **2026-07-28**, matching the current design reference. | Stateless requests, discovery, request capability metadata, catalog variation by authorization and schema/result support. Do not transplant older initialization/session examples. |
| [MCP authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization) | 2026-07-28. | Authentication/authorization boundary and credential use; Conduit participation grants need their own verified binding. |
| [MCP Tasks extension](https://modelcontextprotocol.github.io/ext-tasks/specification/2026-07-28/tasks.html) | `io.modelcontextprotocol/tasks`, 2026-07-28. | Negotiation, task handles, get/update/cancel and outcomes. Independent extension, not an unconditional core feature. |
| [OpenAPI](https://spec.openapis.org/oas/v3.1.1.html) | **3.1.1 comparison baseline**, not a latest-version claim. | Components, operation descriptions and Schema Object/dialect. Direct source retrieval succeeded; one web-rendering request failed. No generated client was tested. |
| [AsyncAPI](https://www.asyncapi.com/docs/reference/specification/v3.0.0) | **3.0.0 comparison baseline**; the site also advertises 3.1.0. | Operations/messages/channels, protocol bindings and correlation. No broker or generated implementation tested. |
| [CloudEvents](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md) | Specification tag **v1.0.2**. | Event context, identity and extension attributes. Does not itself define our retention or reader-state capability. |

## Research papers

- **Crusoe et al., 2022:** [*Methods Included: Standardizing Computational Reuse and Portability
  with the Common Workflow Language*](https://ir.cwi.nl/pub/33643/33643.pdf), Communications of
  the ACM 65(6), 54–63; [DOI 10.1145/3486897](https://doi.org/10.1145/3486897).
  Reviewed the standards/implementation distinction, dataflow discussion and conformance sections.
  Evidence for a portable authoring/conformance strategy, not Conduit-specific effect guarantees.
- **Honda, Yoshida and Carbone, 2008:** [*Multiparty Asynchronous Session Types*](https://www.doc.ic.ac.uk/~yoshida/multiparty/multiparty.pdf),
  POPL, 273–284; [DOI 10.1145/1328438.1328472](https://doi.org/10.1145/1328438.1328472).
  Reviewed the author manuscript's global/local protocol model and assumptions. No proof audit,
  session-type implementation or claim of verified Conduit endpoints.
- **Gupta et al., 2012:** [*P: Safe Asynchronous Event-Driven Programming*](https://www.microsoft.com/en-us/research/publication/p-safe-asynchronous-event-driven-programming/),
  MSR-TR-2012-116. Reviewed the primary report page/abstract, not the complete formal development.
  Use as a follow-up reference for modeling adverse event sequences, not as evaluated tooling.

The previous [paper review](../CONFIGURATION.md#research-papers-and-their-limits) preserves
configuration-error, configuration-complexity and higher-order contract research with its original
limits. This report does not repeat those studies or treat their conclusions as new empirical tests.

## Local experimental evidence

See [probe instructions](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/003-language-probes/README.md), [source](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/003-language-probes/probe.py) and
[recorded results](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/experiments/003-language-probes/results.json). Twenty cases passed with Python 3.13.2,
`jsonschema` 4.25.1 and `ruamel.yaml` 0.18.16. The recorded dependency list is provided for
reproduction. These are selected library behaviors, not comprehensive standard conformance,
Rust implementation evidence or proof that the Conduit language already exists.
