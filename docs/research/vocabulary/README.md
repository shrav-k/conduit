# Vocabulary research

> **Superseded for decisions, kept as evidence.** The vocabulary was ruled on 2026-11-05; the
> accepted words are in the [vocabulary record](../../proposals/vocabulary/README.md) and the
> [specification's definitions](../../../spec/definitions.md). What remains here is the evidence
> those rulings cite: the [precedents](PRECEDENTS.md), their [sources](SOURCES.md) with retrieval
> data, and the reviewer's [simplification pass](SIMPLIFICATION.md). The candidate tables and
> the walkthroughs proposed names that the rulings have since replaced, so they were removed
> from the tree; links to them point at the last commit that held them.

Status: research and recommendations for discussion, 2026-11-04. No replacement names or field
spellings are adopted here. The five settled families remain **Interface, Agent, Resource, Pool
and Conduit**. The [vocabulary map](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/VOCABULARY.md) links the accepted meanings; its `Vxx`
identifiers remain stable documentation hooks. The [current review](https://github.com/shrav-k/conduit/blob/e271d1c711c9baa799c8d3e55708aedbd9be7fd1/docs/REVIEW.md#future-vocabulary-pr)
is still the only decision queue.

The behavior/structure and documentation work is preserved on main through
[PR #4](https://github.com/shrav-k/conduit/pull/4), merge `66b82b6`. This packet begins the separate
naming inquiry. It changes no protocol behavior, fixtures, implementation selections or support claims.

## Recommendation

Use conventional technical names for authoring and execution, with a light connection/flow vocabulary
around Conduits. The project already has two memorable organizing names. We do not need a water
metaphor for every item an author must learn.

My initial preference is to retain **template, instance, package, contract, capability, operation,
input, output, grant, endpoint and route**, subject to reviewing their definitions together.
The largest unresolved naming choices are **plugin/implementation**, **graph/procedure/workflow**,
**requirement/constraint**, **instance runtime**, and **logical participation**. Those deserve complete
examples before choosing words. This is a proposed shortlist, not a ruling.

The research supports that direction for several different reasons:

- Kubernetes connects `spec` to desired state and controllers to continuous reconciliation. Reusing
  its envelope does not make those execution semantics ours. Conduit's last-confirmed configuration
  and explicit controls need equally clear names. [Kubernetes objects](https://kubernetes.io/docs/concepts/overview/working-with-objects/),
  [controllers](https://kubernetes.io/docs/concepts/architecture/controller/).
- WIT separates packages, imports/exports and behavioral interfaces. That is useful precedent for
  our authoring boundaries, although **Interface** already names a Conduit family and WIT's
  **resource** means something different. [WIT](https://component-model.bytecodealliance.org/design/wit.html).
- TOSCA distinguishes topology relationships from workflows. Its requirements primarily connect
  needed features to offered capabilities; our requirements also impose guarantees at declared
  points during use. The resemblance is valuable, but not an equivalence. [TOSCA 2.0 CS01](https://docs.oasis-open.org/tosca/TOSCA/v2.0/cs01/TOSCA-v2.0-cs01.md).
- CWL describes graph steps and explicit input/output links; OpenTelemetry names components by
  their role in a data pipeline. Both suggest naming the job something does rather than inventing
  a universal noun for everything executable. [CWL](https://www.commonwl.org/v1.2/Workflow.html),
  [Collector components](https://opentelemetry.io/docs/collector/components/).
- TCP, QUIC, message brokers and workflow engines attach different lifetime, ordering and delivery
  guarantees to superficially similar words. A Conduit is not automatically a transport connection,
  ordered stream, message queue or durable workflow. [TCP](https://www.rfc-editor.org/rfc/rfc9293.html),
  [QUIC](https://www.rfc-editor.org/rfc/rfc9000.html),
  [AMQP 0-9-1 model](https://www.rabbitmq.com/tutorials/amqp-concepts).

These are naming inferences from the cited systems, not claims that those projects endorse our design.

## Read this packet

| Document | Purpose |
| --- | --- |
| [Precedents](PRECEDENTS.md) | What source terms actually mean, which distinctions help, and what behavior not to import. Covers orchestration, contract languages, workflows, messaging, authorization, observability and agent/runtime ecosystems. |
| [Candidates](https://github.com/shrav-k/conduit/blob/2890ef78c902f2e550fe193396ab464f2db3debc/docs/research/vocabulary/CANDIDATES.md) | Alternatives, tradeoffs and a provisional recommendation for every V01–V37 topic; light-theme comparison. |
| [Simplification](SIMPLIFICATION.md) | Reviewer feedback: which candidate terms collapse into one, which words carry too many meanings, and which distinctions must stay. Diagram included. |
| [Walkthroughs](https://github.com/shrav-k/conduit/blob/2890ef78c902f2e550fe193396ab464f2db3debc/docs/research/vocabulary/WALKTHROUGHS.md) | Test candidate language against package authoring, shared service access, setup/recovery, communication and observation. No new YAML dialect. |
| [Sources](SOURCES.md) | Evidence scope, direct primary links, pinned local sources, retrieval metadata and limitations. |

## The distinction a name must explain

An author should be able to tell which of these questions a word answers:

| Question | Existing working vocabulary |
| --- | --- |
| What can I publish and reuse? | Package, export, template, contract, graph-backed operation. |
| What meaning must an implementation satisfy? | Type/capability/operation/requirement contract and its data schemas. |
| What performs the behavior? | Selected implementation, possibly code, an existing MCP service or a composition. |
| What exists independently after creation? | An instance in one of the five families. |
| What happens once, and what remains managed? | Invocation/attempt/setup progress versus instance-owned runtime behavior. |
| How do instances interact? | Conduit, endpoint roles, participation and selected access path. |
| Why is an action permitted, and what limits it? | Grant plus current governing requirements and restrictions along the path. |
| What do we know happened? | Confirmed outcomes, observations, evidence, coverage and uncertainty. |

The table is a reading aid, not a new ontology. In particular, packages, graphs, plugins and grants
do not become additional object families.

## How to evaluate a candidate

Prefer a name that survives these tests; do not reduce the decision to a numeric popularity score.

1. **Meaning:** it describes the accepted responsibility without implying extra scheduling, replay,
   security, storage or network guarantees.
2. **Composition:** it still makes sense when implemented by several plugins, backed by an external
   MCP service, or nested inside a reusable operation.
3. **Lifecycle:** definition, instance, invocation, attempt, live connection and retained data remain
   distinguishable. A crash or detach should not make the terminology contradictory.
4. **Actor neutrality:** it works for Interfaces, Agents and Resources, not only for a human CLI or
   model tool call. Physical placement remains a separate concern.
5. **Readability:** it produces useful sentences, error messages and inspection screens without a
   paragraph of qualifications every time.
6. **Collision cost:** it does not overload a settled family, external schema keyword or another
   frequently adjacent concept. Qualified terms are preferable to silently assigning two meanings.
7. **Authoring cost:** it keeps the number of concepts small without erasing distinctions. A familiar
   synonym is not automatically worth another formal noun.

Searchability, pronunciation and the ease of explaining the system to a new author matter too.
We have not run user studies, trademark searches or name-availability checks. Framework documentation
can expose semantic risks; it cannot establish which vocabulary users will find easiest.

## Proposed discussion order

Start with the words needed to explain an original extension author, not with command-line spelling.
Use the [worked examples](https://github.com/shrav-k/conduit/blob/2890ef78c902f2e550fe193396ab464f2db3debc/docs/research/vocabulary/WALKTHROUGHS.md) as the common test for each cluster.

1. **Authoring:** package, contract, capability, implementation/plugin, export and dependency
   (V02–V08). Decide whether “plugin” remains a precise primitive or an explanatory umbrella.
2. **Execution:** operation, graph, step, invocation, attempt, input/output and instance runtime
   (V09–V14, V26–V28). Keep the finite setup procedure distinct from the live relationship.
3. **Relationships and authority:** requirement, filter, policy, grant, participation, endpoint and
   route (V17–V23). Test shared authentication and result restrictions, not only connection setup.
4. **Lifecycle and observation:** template/instance, configuration/state, setup/retirement and
   observations (V01, V15–V16, V24–V25, V29–V32).
5. **Domain language:** Pool entries/ownership, account custody, activation and memory/history
   (V33–V37). Retain provider-native terms where they describe provider-native things.

This is one suggested ordering of the existing review, not a new list of required design rulings.
Once words are accepted, concrete field spellings and examples can follow. If a candidate needs a
semantic change to work, describe that separately before accepting the name.
