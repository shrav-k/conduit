# Language and protocol foundations

Research date: **2026-11-02**. This is evidence and recommendations for the next authoring review,
not an amendment to the [accepted protocol](../../protocol/README.md). No package spelling,
schema dialect, new language dependency or implementation has been selected by this research.

**Recommendation:** retain typed YAML and MCP, and design Conduit's authoring contract around
explicit imports/exports, typed operation bindings and observable effects. The most useful new
references are **WIT and Smithy for contracts**, **CWL for portable composition**, and
**YANG for managed configuration and operations**. TOSCA is the closest broad conceptual
comparison, but its orchestration and dependency-resolution model should not be adopted wholesale.
Docker and Kubernetes remain useful references for particular lifecycle boundaries.

This reinforces the five-family model. It also exposes decisions that cannot be settled merely
by choosing YAML keys: which contract identities are compatible, where defaults resolve, what
crosses a graph boundary, and how a caller distinguishes confirmed effects from uncertain ones.

Read this synthesis first, then the [worked comparisons](WALKTHROUGHS.md). The
[source guide](SOURCES.md) identifies versions, sections, primary links and limitations.
[Twenty offline probes](../../../experiments/003-language-probes/README.md) make several parsing/schema pitfalls reproducible.
The [current review](../../REVIEW.md) remains the only ruling queue.

## Scope and method

The earlier [configuration](../CONFIGURATION.md), [establishment](../ESTABLISHMENT.md),
[communication](../CONDUITS.md) and [runtime](../RUST_RUNTIME.md) research remains useful.
This pass concentrates on language construction and interoperability rather than repeating
provider inventories, runtime selection or repository organization.

We inspected primary specifications and official documentation across contract languages,
configuration systems, workflow languages, management protocols and event APIs. Thirty-two
retrieved source documents have URL/content fingerprints in [retrievals.json](retrievals.json);
moving WIT, CEL and Open Workflow sources are pinned to commits. Additional primary sources and
three research papers are linked in the source guide. A retrieval fingerprint records the bytes
inspected; it does not certify a source's claims, freeze a website, or imply its entire specification
was reviewed. The guide identifies the relevant portions.

The only executed experiments here are local Python library probes. No workflow platform,
Conduit runtime, model account, cloud service or MCP deployment was exercised. Native-language
snippets in the walkthrough are explanatory examples, not certified runnable packages.

## The comparison that matters

These systems solve different problems. The table identifies useful mechanisms, not a ranking
of interchangeable products. References lead to the detailed source guide.

| Reference | Strongest contribution | Boundary Conduit should preserve |
| --- | --- | --- |
| [WIT / Component Model](SOURCES.md#component-and-service-contracts) | Required/provided interfaces, package-local names, typed handles. | Contract composition need not select Wasm, process layout or access authority. |
| [Smithy 2.0](SOURCES.md#component-and-service-contracts) | Data shapes, operations, modeled errors and behavioral traits independent of wire binding. | A trait declaration is not proof that its implementation honors the behavior. |
| [YANG / NETCONF / NMDA](SOURCES.md#network-management-and-topology) | Configuration, operational observations, targeted actions and notifications. | Do not import editable datastores or a desired-state controller in place of named controls. |
| [TOSCA 2.0 CS01](SOURCES.md#network-management-and-topology) | Typed topology, relationship contracts, reusable service composition and lifecycle operations. | Its dependency requirements and automatic fulfillment are not Conduit's governing requirements or grants. |
| [CWL 1.2.1](SOURCES.md#execution-and-composition) | Typed step ports, nested workflows, explicit public outputs, executable portability criteria. | Batch process/dataflow semantics do not provide interactive authentication or ongoing instance supervision. |
| [Tekton](SOURCES.md#execution-and-composition) | Reusable tasks, result dependencies and separately recorded executions. | Kubernetes execution and retry defaults are not protocol requirements. |
| [Open Workflow](SOURCES.md#execution-and-composition) | Reusable operations, protocol bindings and structured task inputs/results. | Its wider control flow and binding versions exceed our selected v0 subset. |
| [Amazon States Language / Temporal](SOURCES.md#execution-and-composition) | Explicit failures, retry conditions, waits and durable operation boundaries. | Retrying scheduling is not proof that an external mutation is safe to repeat. |
| [OpenTofu / Pulumi](SOURCES.md#configuration-and-validation) | Local provider names, dependency values and separate implementation configuration. | No implicit account inheritance, automatic planning/apply lifecycle or general-purpose expression language follows. |
| [JSON Schema / CUE / CEL / Dhall](SOURCES.md#configuration-and-validation) | Data validation, constraint combination, bounded expressions and explicit imports. | These mechanisms have different jobs; none supplies complete lifecycle or authorization semantics. |
| [Compose / Kubernetes](SOURCES.md#configuration-and-validation) | Explicit startup gates, schemas and separation of declarations from observations. | Reconciliation, cascading cleanup and environment substitution remain independent choices. |
| [MCP](SOURCES.md#invocation-and-events) | Native tool discovery/invocation, interaction patterns and negotiated extensions. | Transport state, operation evidence and Conduit participation are different identities. |
| [OpenAPI / AsyncAPI / CloudEvents](SOURCES.md#invocation-and-events) | API/message descriptions, protocol bindings and event envelopes. | Description/correlation does not promise delivery, execution, authorization or retained history. |

## 1. Use contract languages to define boundaries

### WIT: imports and exports without a mandatory primary plugin

WIT interfaces group callable behavior and types; worlds state required imports and offered
exports. Packages and local names give composition a defined resolution scope. WIT's resource
handles distinguish passing a value from borrowing or transferring an opaque entity.
These are especially useful precedents for Conduit's multi-plugin templates.
[WIT overview](https://component-model.bytecodealliance.org/design/wit.html) ·
[worlds](https://component-model.bytecodealliance.org/design/worlds.html)

The proposed lesson is to make a template's required behavior inspectable before selecting
implementations. A filesystem contribution and a checks contribution can jointly satisfy it;
neither needs to be a privileged primary plugin. A caller's alias should not silently change
names inside a published composite. Imports need explicit binding rules.

Do not equate a WIT resource with a Conduit Resource. WIT handle ownership concerns an ABI-level
entity and its lifetime, not Pool ownership or authorization. A copied Conduit identifier cannot
become permission to use a capability. Native MCP and script implementations also do not gain
Wasm's isolation merely by adopting similar contract vocabulary.

### Smithy: behavior alongside data, separate from transport

Smithy describes operations with input/output shapes and modeled errors. Its traits describe
properties such as read-only behavior, idempotency and token-based duplicate detection; protocol
traits separately describe wire mappings. This is a better precedent for our public contracts
than treating a Rust function signature or a JSON object schema as the entire API.
[Service types](https://smithy.io/2.0/spec/service-types.html) ·
[behavior traits](https://smithy.io/2.0/spec/behavior-traits.html) ·
[protocol traits](https://smithy.io/2.0/spec/protocol-traits.html)

For Conduit, an operation's input might be structurally valid while its invocation is forbidden,
its prerequisite unavailable, or its previous effect uncertain. Those distinctions belong in the
contract and evidence model. Conduit need not invent a fixed universal error taxonomy: an
extensible domain result can coexist with shared rules for correlation, effects and uncertainty.
The selected implementation must fulfill the public meaning; publishing an annotation alone
does not supply recovery or enforcement.

Smithy's mixins illustrate reusable definitions, but schema composition, handler selection and
runtime ownership remain different operations. Reusing a shape should never silently replace a
control handler or merge grants. [Mixins](https://smithy.io/2.0/spec/mixins.html)

## 2. Learn from network management without adopting its controller

YANG models configuration/state data, RPCs, object-targeted actions and notifications. It is a
particularly close reference for a system where an Interface can inspect an instance and invoke
typed controls. Its action boundary is more relevant than generic field mutation.
[RFC 7950, especially §§7.14–7.16](https://www.rfc-editor.org/rfc/rfc7950.html)

NMDA distinguishes intended configuration from operational state and configuration actually in
use. The useful lesson is to avoid pretending that an accepted request proves the runtime's
current value. Conduit already chose last-confirmed configuration plus separate outstanding
requests, rather than NMDA's complete datastore architecture. Preserve that choice when designing
inspection/export. [RFC 8342 §5](https://www.rfc-editor.org/rfc/rfc8342.html)

NETCONF offers capabilities such as candidate configuration and confirmed commit. These work
within that protocol's defined datastore/transaction model. They do not establish rollback across
an Agent, a shared MCP provider and a remote database. A Conduit control with multiple external
effects still needs honest partial-outcome and reconciliation behavior.
[RFC 6241 §§8.3–8.4](https://www.rfc-editor.org/rfc/rfc6241.html)

TOSCA is the closest broad analogy: node/relationship types, service templates, representations,
interface operations and workflows. Its substitution mappings show how a composed implementation
can expose a smaller public surface. However, TOSCA requirements primarily express dependencies
and capabilities to fulfill, including orchestrator-selected targets. Conduit requirements also
govern continuing use, and our targets/grants cannot be inferred from a topology match.
Borrow the separation of topology and procedures; keep explicit authority and our simpler graph
subset. [TOSCA 2.0 CS01 §§3.2, 3.5, 15.1](https://docs.oasis-open.org/tosca/TOSCA/v2.0/cs01/TOSCA-v2.0-cs01.html)

## 3. Make graph portability about ports and effects

CWL gives workflows typed inputs/outputs, nested processes and output-to-input links that form a
DAG. Its execution model allows independent steps to run concurrently. Its feature requirements
and optional hints also make unsupported execution capabilities visible.
This supports our finite graph and deliberately published output choices.
[CWL execution model and WorkflowStep](https://www.commonwl.org/v1.2/Workflow.html)

There is a crucial difference: CWL permits more-specific process requirements to take precedence,
and its specification includes a user override qualification for unsupported requirements.
Conduit cannot inherit those semantics for governing restrictions. Every applicable requirement
on the selected path must hold; a nested graph cannot override an outer restriction. CWL's file
staging and terminating command-line model also do not define our live endpoint lifetimes.

Tekton demonstrates result dependencies plus explicit ordering. Its documentation also identifies
a useful failure case: a task can complete without producing a declared result, with the failure
surfacing when a consumer needs it. Our eventual conformance cases should distinguish completed
work, available outputs and successful composite results rather than rely on one green status.
[Tasks](https://tekton.dev/docs/pipelines/tasks/) · [Pipelines](https://tekton.dev/docs/pipelines/pipelines/)

Open Workflow's reusable task definitions and protocol-specific calls support code-optional
composition. Its broader loops, branching, events and expressions are outside our accepted v0
graph subset. The inspected MCP binding defaults to `2025-06-18` and requires initialization;
Conduit's selected `2026-07-28` MCP basis has different semantics. This is an example of why a
familiar protocol name is insufficient compatibility evidence.
[Pinned Open Workflow reference](https://github.com/open-workflow-specification/specification/blob/2108908485b2061ccf55a8444a8c48e203aedec4/dsl-reference.md#mcp-call)

Amazon States Language separates retry matching, catch handling and input/output transformation.
Temporal records activity progress and exposes timeouts/cancellation while effects occur in
activity code. These are useful references for our graph coordinator, but adopting either runtime
would not remove operation-specific recovery duties. A timeout after a provider committed a write
cannot honestly be treated as “nothing happened.”
[ASL error handling](https://states-language.net/spec.html#error-handling) ·
[Temporal activity execution](https://docs.temporal.io/activity-execution)

**Proposed drafting discipline:** give every public composite an understandable result boundary,
and specify what child evidence must survive domain-result construction. Recovery can reuse valid
progress without exposing every private intermediate value. Cancellation should describe requested
and confirmed shutdown/effects separately, under the existing retirement contract.

## 4. Separate names, configuration and bindings

OpenTofu distinguishes a provider's global source address, its module-local name and configured
provider instances. Child modules declare their own requirements, with provider configurations
passed according to module rules. This closely matches our unresolved package/export/caller-alias
question, even though Conduit should not inherit automatic default-provider behavior.
[Provider requirements](https://opentofu.org/docs/v1.11/language/providers/requirements/) ·
[providers within modules](https://opentofu.org/docs/v1.11/language/modules/develop/providers/)

For our package walkthrough, distinguish five things before choosing punctuation:

1. Published contract identity and version: what an operation promises.
2. Implementation artifact: the exact selected code or adapter contribution.
3. Package export: the callable contribution that binds contract to implementation.
4. Local alias: a convenient name within the importing template or graph.
5. Runtime target/participation: the instance and authorized path on which it acts.

These are not five new object families or necessarily five YAML blocks. They are meanings that a
single overloaded `name` cannot safely infer. The same MCP implementation may serve two Pools;
the same named tool may run against two accounts. Resolving code is not selecting service identity.

OpenTofu's unknown values and Pulumi's dependency-carrying outputs show why deferred values should
be explicit rather than empty strings or null. Pulumi also propagates secret marking through
derived outputs. Conduit needs similarly deliberate distinctions between ordinary data, protected
references and live instance-owned handles; it need not adopt either expression system.
[OpenTofu values](https://opentofu.org/docs/v1.11/language/expressions/references/) ·
[Pulumi inputs/outputs](https://www.pulumi.com/docs/iac/concepts/inputs-outputs/) ·
[secret handling](https://www.pulumi.com/docs/iac/concepts/secrets/)

## 5. Keep validation, defaults and enforcement distinct

JSON Schema 2020-12 is a strong candidate for portable data contracts and aligns with current
MCP schemas. It is not a complete plugin language. Defaults and read/write markings are annotations;
format assertion has separately specified support; unsupported keywords can be treated as
annotations unless a required vocabulary changes that obligation.
[Core specification](https://json-schema.org/draft/2020-12/json-schema-core) ·
[validation specification](https://json-schema.org/draft/2020-12/json-schema-validation)

The local probes confirm practical consequences in the selected libraries:

- Validating `{}` does not materialize a declared default.
- An invented enforcement keyword does not make a stock validator enforce anything.
- `allOf` cannot reopen a base closed with `additionalProperties: false`; closure after composition
  behaves differently with `unevaluatedProperties`.
- `10.0` satisfies JSON Schema's mathematical integer type; string `"10"` does not.
- A safe YAML loader may still accept aliases/merges, `.inf`, and implicit date objects.

See [probe scope and results](../../../experiments/003-language-probes/README.md). These results support a staged implementation:
parse the chosen strict surface, validate the supported schema vocabulary, resolve typed names,
materialize unambiguous defaults, then check runtime prerequisites and current authority. The
order and exact schema/default rules still need a public contract. A stock loader plus validator
is insufficient, and “schema accepted” must not mean “plugin can enforce this.”

CUE's unification is a useful model for combining constraints without last-writer precedence.
CEL separates parsing/checking from evaluation over a declared environment. Dhall demonstrates
typed configuration and import integrity with explicit limits on what its safety guarantees cover.
These are optional tooling precedents, not reasons to replace YAML or add a general expression
language now. [CUE](https://cuelang.org/docs/reference/spec/) ·
[CEL](https://github.com/cel-expr/cel-spec/blob/21ec6f4e52e8d694687ff9ed7408d3eccbb955d7/doc/langdef.md) ·
[Dhall](https://docs.dhall-lang.org/discussions/Safety-guarantees.html)

In particular, a requirement's runtime duty cannot be reduced to intersecting its parameter
schemas. Two limits might compose numerically; “filter before delivering content” also requires
an identified enforcer at the correct boundary. Preserve individually enforced requirements and
optional composition checks rather than requiring a universal solver.

## 6. MCP supplies interaction, not all of Conduit's semantics

The selected MCP `2026-07-28` revision is stateless at the protocol level: requests carry version
and capability metadata, discovery is explicit, and cross-call state uses explicit handles.
Do not equate an MCP connection with a Conduit instance or logical participation.
[MCP changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)

Tool discovery can reflect request authorization; tool output schemas use JSON Schema 2020-12.
This supports familiar provider-native tools plus filtered managed access. An author-supplied
caller ID or tool annotation is not authenticated authority. Conduit's adapter/profile must bind
verified caller, target and participation while preserving the provider operation's familiar form.
[Tools](https://modelcontextprotocol.io/specification/2026-07-28/server/tools) ·
[authorization](https://modelcontextprotocol.io/specification/2026-07-28/basic/authorization)

MCP's Tasks extension offers task handles, polling, updates and cancellation for deferred results.
It is a candidate transport mapping for long-running operations, not automatically the definition
of Conduit recovery or proof that a remote effect was cancelled. Its support must be explicit;
clients without it need a declared supported interaction path or an explicit unsupported outcome.
Choosing a mapping does not add Agent job submission to the first communication experiment.
[Tasks extension](https://modelcontextprotocol.github.io/ext-tasks/specification/2026-07-28/tasks.html)

OpenAPI and AsyncAPI help separate an operation/message description from its protocol binding.
CloudEvents supplies event identity and contextual metadata, not retained exchange history or
an execution ledger. These are possible adapter/export surfaces; they do not justify replacing
MCP or requiring every author to write three API descriptions.
[OpenAPI 3.1.1](https://spec.openapis.org/oas/v3.1.1.html) ·
[AsyncAPI 3.0.0](https://www.asyncapi.com/docs/reference/specification/v3.0.0) ·
[CloudEvents 1.0.2](https://github.com/cloudevents/spec/blob/v1.0.2/cloudevents/spec.md)

## 7. Papers: useful principles, bounded claims

| Primary work | Useful implication | What it does not establish |
| --- | --- | --- |
| Crusoe et al., *Methods Included*, CACM 2022, [paper](https://ir.cwi.nl/pub/33643/33643.pdf), [DOI](https://doi.org/10.1145/3486897) | Portable tool/workflow descriptions and public conformance examples can serve multiple implementations. Make authoring fixtures part of the protocol discussion, not an afterthought. | Scientific batch portability does not prove interactive Agent lifecycle, credentials or access enforcement. We still need the independent client/plugin proof already accepted. |
| Honda, Yoshida and Carbone, *Multiparty Asynchronous Session Types*, POPL 2008, [author manuscript](https://www.doc.ic.ac.uk/~yoshida/multiparty/multiparty.pdf), [DOI](https://doi.org/10.1145/1328438.1328472) | A relationship can describe global interaction obligations that are checked against participant behavior. This supports thinking carefully about endpoint duties. | Arbitrary scripts and MCP services do not become verified session-typed programs. The paper's guarantees rely on its calculus and typing assumptions. No new session-type language is proposed for v0. |
| Gupta et al., *P: Safe Asynchronous Event-Driven Programming*, MSR-TR-2012-116, [primary report page](https://www.microsoft.com/en-us/research/publication/p-safe-asynchronous-event-driven-programming/) | Explicit event handling and modeled environments make adverse interleavings testable. Use failure traces for controls, retirement and reconnects. | This is not evidence that Conduit needs P or a universal state enum. The inspected report summary does not substitute for implementing/model-checking our own semantics. |

The earlier [configuration-paper review](../CONFIGURATION.md#research-papers-and-their-limits)
also remains relevant: avoid exposing implementation knobs merely because they exist, and make
diagnostics explain consequences rather than only rejected syntax.

## 8. What to refine before spelling the language

These are recommendations within the existing review batches, not accepted rulings or a new
backlog. The walkthrough makes each question concrete.

| Existing batch | Refinement this research exposes | Recommended starting position |
| --- | --- | --- |
| Identity/publication | Package-local names versus caller aliases; contract identity versus implementation and service target. | Lexical scope and explicit imports/exports. Resolve conflicts rather than choosing by install order. Preserve exact selected artifacts without a backward-compatibility promise. |
| Identity/publication | How much of JSON Schema is portable; how defaults, formats and protected values work. | Specify a supported dialect/profile, offline reference resolution and default rules. Do not invent runtime semantics through unknown schema keywords. |
| Templates/composition | Capability satisfaction and safe operation substitution. | Match declared contract identity/compatibility and validate bindings; distinguish type checks from runtime readiness and enforcement. No required primary implementation. |
| Templates/composition | Public composite results, output lifetime and validity after a partial failure. | Deliberate public ports/results plus retained child effect evidence. Stable references, ordinary values and live handles must be distinguishable. |
| Invocation/observation | Minimal MCP profile and deferred interaction mapping. | Keep MCP version/features explicit. Correlate the managed operation independently of a transport request or task handle. |
| Conformance | Which small examples actually test the extension boundary. | Add the failure traces in the walkthrough to Experiment 002's existing scope; keep one-machine proof and avoid expanding the initial runtime into a general workflow platform. |

No finding requires another object family, a replacement for MCP/YAML, a primary plugin, a central
universal runtime process, or a compatibility maintenance obligation. The remaining difficulty is
expressing the already-selected semantics clearly enough that an independent author can implement
them without reading the reference runtime.

## Suggested next discussion

Start with the shared-history package used in the existing
[package walkthrough](../../proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md), now viewed through
the [three kinds of connection](WALKTHROUGHS.md#three-kinds-of-connection) and
[package scope example](WALKTHROUGHS.md#case-1-one-resource-several-contributions).
Agree what is imported, exported, configured and authorized before choosing field names.
Then run the same model through a database failure and shared MCP access. That will test whether
the notation represents our semantics rather than merely looking familiar.
