# Vocabulary review map

**Direction note, 2026-11-05.** Superseded in part on the same day: the vocabulary is now decided alongside the rework; see the
[vocabulary proposal](proposals/vocabulary/README.md). Earlier: by the owner's rulings in the [plan audit](AUDIT.md) the naming pass
comes last, after the language is concrete and proven, and it may merge concepts as well as rename
them. Links below into Experiments 001 to 003 will be replaced when those are removed.

Status: preparation for the naming review, 2026-11-04. **Only Interface, Agent, Resource, Pool and
Conduit are settled family names.** The accepted [vocabulary boundary](protocol/MODEL.md#vocabulary-review-boundary)
keeps other Conduit-owned names and field spellings provisional. This map proposes no replacements
and adds no behavior. The [current review](REVIEW.md#future-vocabulary-pr) remains the work queue.

Each `Vxx` is a stable **documentation topic ID** linking to the accepted definition. It is not an
object ID, contract version, language keyword or public protocol namespace. Keep the corresponding
`vxx` anchor when a heading or term changes. The short descriptions below distinguish nearby concepts;
the linked protocol text remains authoritative. An example's spelling is still illustrative.

## Definitions and extension authoring

| Topic / accepted definition | Working terms and distinction to preserve | Worked example / affected surface |
| --- | --- | --- |
| [V01](protocol/MODEL.md#v01) | **Template, instance, form.** Reusable establishment definition versus an independently identified realization; neither means a dedicated process. | [All five families](../experiments/001-shared-access/templates.yaml); object envelopes and creation/export descriptions. |
| [V02](protocol/MODEL.md#v02) | **Contract, schema.** Behavioral meaning and guarantees versus valid data shapes; schema validity alone is not conformance. | [Contract inventory](proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md#3-what-the-published-contracts-must-say); published schemas and prose guarantees. |
| [V03](protocol/MODEL.md#v03) | **Type contract.** Specializes one of the five families while preserving its invariants; distinct from implementation/package selection. | [Fixture types](../experiments/001-shared-access/CONTRACTS.md#type-and-capability-boundaries); `type`, group/name/version fields. |
| [V04](protocol/MODEL.md#v04) | **Capability.** What an endpoint offers; distinct from a Resource's type and from authority to use it. | [Shared service access](../experiments/001-shared-access/README.md#3-instantiate-independent-agents-and-attach-access); dependency and endpoint contracts. |
| [V05](protocol/MODEL.md#v05) | **Plugin, implementation binding.** Published contributions backed by code, MCP services, graphs or a combination; not necessarily an executable or a package. | [Package walkthrough](proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md); implementation selections and manifests. |
| [V06](protocol/MODEL.md#v06) | **Package, contribution, bundle.** Distribution envelope versus the definitions it publishes; a template-only package need not contain plugin code. | [Package publication](proposals/authoring-syntax/CONTRACTS.md#5-plugin-package-and-runtime-contracts); installation and release language. |
| [V07](protocol/MODEL.md#v07) | **Entry document, export, re-export.** Explicit public surface versus files merely included in a package; aliases preserve the original scope and contract. | [Re-export example](proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md#re-exporting-through-a-starter-package); package/export names. |
| [V08](protocol/MODEL.md#v08) | **Dependency, import, alias, selection.** Definition/implementation resolution; not a live Resource connection or grant. | [Package scope](protocol/MODEL.md#package-scope-and-public-bindings); `plugins`, `{plugin, name}` and replaceable dependency bindings. |

## Procedures, runtime behavior and results

| Topic / accepted definition | Working terms and distinction to preserve | Worked example / affected surface |
| --- | --- | --- |
| [V09](protocol/MODEL.md#v09) | **Operation, invocation.** Callable contract versus one request/execution of that behavior; a tool call need not be an authored graph. | [Plugin operations](../experiments/001-shared-access/CONTRACTS.md#plugin-operations-and-runtimes); MCP-facing operations and outcome records. |
| [V10](protocol/MODEL.md#v10) | **Graph, node, step, prerequisite.** A finite dependency procedure versus one operation binding within it; map order is not execution order. | [Graph language](proposals/authoring-syntax/CONTRACTS.md#4-graph-language); `nodes`, step names and success prerequisites. |
| [V11](protocol/LIFECYCLE.md#v11) | **Binding, literal, reference, object/list assembly.** Explicit value construction versus the data it produces; ordinary data is not recursively interpreted. | [Binding walkthrough](../experiments/001-shared-access/CONTRACTS.md#graph-binding-notation); all four binding forms. |
| [V12](protocol/LIFECYCLE.md#v12) | **Input, output, port, public result.** Contract-local names and deliberately exposed values; identical names do not connect anything by themselves. | [Graph outputs](proposals/authoring-syntax/CONTRACTS.md#scheduling-outputs-and-lifetime); input/output maps, publication gates and composite results. |
| [V13](protocol/LIFECYCLE.md#v13) | **Runtime, driver, runtime target.** Declared instance-owned behavior, its implementation and a reference selecting it; distinct from a usable live result. | [Explicit startup](proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md#5-the-template-an-author-or-user-works-with); named runtime maps and startup inputs. |
| [V14](protocol/LIFECYCLE.md#v14) | **Control, handler.** A supported typed configuration change versus the operation/graph that applies it; not unrestricted editing of the definition. | [Change one setting](../experiments/001-shared-access/README.md#5-change-one-setting-through-a-control); discovery, bounds, revision checks and confirmed outcomes. |
| [V24](protocol/LIFECYCLE.md#v24) | **Setup, establish, prepare, provision, attach, start.** These describe distinct effects and readiness obligations, not one interchangeable verb. | [Pool provision/use choices](../experiments/001-shared-access/README.md#2-offer-resources-then-explicitly-establish-selected-ones), [startup](../experiments/001-shared-access/CONTRACTS.md#graph-binding-notation). |
| [V25](protocol/LIFECYCLE.md#v25) | **Cancel, stop, retire, unregister/delete, purge.** Requested cancellation, confirmed shutdown, registry removal and backing/evidence deletion remain separate. | [Lifecycle duties](protocol/LIFECYCLE.md#retirement-and-unregistration), [Conduit ending](protocol/CONDUITS.md#conduits); lifecycle API verbs and evidence. |
| [V26](protocol/LIFECYCLE.md#v26) | **Execution, attempt, retry, recovery.** A producing attempt fixes its published values; valid-progress reuse does not imply replaying uncertain effects. | [Composite recovery](proposals/authoring-syntax/CONTRACTS.md#composite-outputs-and-outcomes--proposal); correlation, child progress and retained results. |
| [V27](protocol/LIFECYCLE.md#v27) | **Invocation context, metadata.** Declared graph-visible facts versus trusted context established by the responsible service; caller-supplied IDs cannot grant authority. | [MCP mapping](proposals/authoring-syntax/CONTRACTS.md#mcp-mapping); request metadata and the graph metadata source. |
| [V28](protocol/LIFECYCLE.md#v28) | **Managed reference, handle, descriptor.** Typed operation values with ownership/validity rules; distinct from the graph expression selecting such a value. | [Content and custody](proposals/authoring-syntax/CONTRACTS.md#8-content-handles-and-secret-custody); live targets, protected bindings and stored evidence. |

## Configuration, identity and publication

| Topic / accepted definition | Working terms and distinction to preserve | Worked example / affected surface |
| --- | --- | --- |
| [V15](protocol/CONFIGURATION.md#v15) | **Spec, configuration, status.** Authored definition, confirmed supported settings and reported observations have different ownership. | [Resolved instance](../experiments/001-shared-access/README.md#4-inspect-a-resolved-instance); `spec.config`, inspection and export. |
| [V16](protocol/CONFIGURATION.md#v16) | **Name, ID, configuration revision, binding.** Readable lookup, stable identity and concurrency preconditions are different; name reuse does not retarget an established binding. | [Checked updates](protocol/CONFIGURATION.md#configuration-updates); registry references and rename/delete/export behavior. |
| [V32](protocol/CONFIGURATION.md#v32) | **Contract version, package release, artifact digest, capture.** Distinct interpretation/content identities; edit-mode retention does not publish a reusable release. | [Package lifecycle](proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md); release fields, local development and retained implementation selections. |

## Relationships and authority

| Topic / accepted definition | Working terms and distinction to preserve | Worked example / affected surface |
| --- | --- | --- |
| [V17](protocol/CONDUITS.md#v17) | **Requirement, constituent, obligation.** Composable guarantees with applicability and responsible implementations; composition is not an executable graph. | [Relationship requirements](proposals/authoring-syntax/AUTHORING.md#composable-conduit-contracts); typed parameters, support declarations and enforcement evidence. |
| [V18](protocol/CONDUITS.md#v18) | **Filter.** Narrowing conditions on permitted flow; not authority creation, activation or a universal core query language. | [Issue-access example](../experiments/001-shared-access/README.md#3-instantiate-independent-agents-and-attach-access); operations, paths and provider targets. |
| [V19](protocol/MODEL.md#v19) | **Policy, governing requirement binding.** Authoritative applicability of reusable requirements; not another family or separate policy language. | [Authorization proposal](proposals/authoring-syntax/AUTHORIZATION.md); explicit target scope and live governing matches. |
| [V20](protocol/MODEL.md#v20) | **Grant, issuance, delegation, revocation.** Recorded participation authority, bounded at issuance and governed by current restrictions; no live dependency on the issuer. | [Authority rules](protocol/CONDUITS.md#authority-and-current-access); managed access and administration. |
| [V21](protocol/CONDUITS.md#v21) | **Participation, attachment, join, open.** Logical membership/access versus a socket; joining an existing flow and opening a new one remain explicit intents. | [Shared communication](../experiments/001-shared-access/README.md#6-communicate-and-observe-the-same-activity); per-consumer state and last-detach behavior. |
| [V22](protocol/CONDUITS.md#v22) | **Route, access path.** One complete authorized path's constraints stay together; alternatives do not combine permissions piecemeal. | [Typed filters](protocol/CONDUITS.md#typed-filters); routing, attribution and no silent account failover. |
| [V23](protocol/CONDUITS.md#v23) | **Endpoint, consumer, provider, peer.** Roles within a relationship; not new object families, necessarily processes, or fixed global actor types. | [Endpoint setup](../experiments/001-shared-access/CONTRACTS.md#plugin-operations-and-runtimes); connection requirements and local implementation duties. |
| [V34](protocol/POOLS.md#v34) | **Owner, backing data, Resource entry.** Pool ownership, underlying data and a reusable provisioning option have separate identities and lifecycles. | [Pool composition](../experiments/001-shared-access/README.md#2-offer-resources-then-explicitly-establish-selected-ones); template entries and explicit provision/use intent. |

## Execution, observation and domain data

| Topic / accepted definition | Working terms and distinction to preserve | Worked example / affected surface |
| --- | --- | --- |
| [V29](protocol/MODEL.md#v29) | **Observation, evidence, source, snapshot, feed, cursor.** Current/retained views with coverage and source-local continuity; not a global order or authority store. | [Observation walkthrough](../experiments/001-shared-access/README.md#6-communicate-and-observe-the-same-activity); Interface views and export contracts. |
| [V30](protocol/MODEL.md#v30) | **Management service, executor, host, worker/helper.** Physical execution and authority responsibilities; none is a sixth object family or one process per instance. | [Runtime architecture](implementation/ARCHITECTURE.md); transport, supervision and placement diagrams. |
| [V31](protocol/MODEL.md#v31) | **Frontend, Interface attachment, root/local owner.** A presentation client, its attachment and administrative identity; Interface remains a settled family name. | [Interface entry](../experiments/001-shared-access/README.md#1-start-from-the-interface); bootstrap/recovery and multi-client control. |
| [V33](protocol/RESOURCES.md#v33) | **Memory, context, history, record, collection.** Domain-specific Resource capabilities/data; no universal Context layer or mandatory room object. | [Shared history](proposals/authoring-syntax/PACKAGE_WALKTHROUGH.md); memory/history operations and provider-native context. |
| [V35](protocol/POOLS.md#v35) | **Authentication, account identity, credentials, custody.** Service login and secret ownership differ from the caller's managed authority; shared access does not merge Agent identities. | [Shared authentication](../experiments/001-shared-access/CONTRACTS.md#type-and-capability-boundaries); authentication Resources, credential adapters and Conduit grants. |
| [V36](protocol/AGENT.md#v36) | **Activation, preparation, exposure.** Loading permitted skills/configuration/tools into an Agent differs from granting access or selecting a Pool Resource. | [Agent setup](../experiments/001-shared-access/README.md#3-instantiate-independent-agents-and-attach-access); template activation rules and frontend/native adapters. |
| [V37](protocol/AGENT.md#v37) | **Model, provider domain, subscription/account.** A supported model selection within fixed boundaries differs from replacing the Agent's account/runtime definition. | [Agent instance behavior](protocol/AGENT.md#instances); provider-specific controls and retained native state. |

## Field and terminology collisions

Review these in their full example, not through a global replacement:

| Current spelling | Different meanings that must remain distinguishable | Topics |
| --- | --- | --- |
| `type` | Family-specializing contract selection; JSON Schema's standard data-type keyword; colloquial implementation type. | V02–V03, V15 |
| `reference`, `binding`, `path` | Graph expression/JSON Pointer; registry kind/form/name lookup; package export selection; managed live value; filesystem location; authorized route. | V08, V11, V16, V22, V28 |
| `node`, `port`, `source` | Graph step/output; a machine/network endpoint; observation source; package source directory. | V10–V12, V29–V30, V32 |
| `runtime`, `instance`, `execution` | Fixed managed behavior, durable object identity, an operation attempt or a native Agent process. | V01, V13, V26, V30 |
| `metadata`, `config`, `status` | Object envelope fields; invocation metadata; captured execution input; last confirmed settings; runtime observations. | V15–V16, V27, V29 |
| `version`, `revision`, `release` | Common/type/capability contract interpretation; concurrency version; selected package publication. | V03, V16, V32 |
| `profile`, `session`, `resource`, `tool` | Historical Conduit concepts versus current MCP/native-provider terminology. Conduit's Resource family is not MCP's resource surface. | V04–V05, V27, V33 |
| `scope`, `policy`, `filter`, `grant` | Historical access selections versus the current distinct authority and restriction mechanisms. There is no safe one-to-one rename for every old use. | V17–V22 |
| `identity`, `account`, `access`, `activation` | Registry identity; caller authority; service/model authentication; loading an already-permitted capability into the Agent. | V16, V20, V35–V37 |

Historical **Profile/Session**, **project/scope** and **room** terminology stays in attributed research
and archives where it explains the earlier design. Source-system names, JSON/JSON Schema keywords,
JSON Pointer syntax and MCP terminology keep their external meanings. Conduit-owned wrappers and
field names may change; an external wire contract must not be renamed accidentally.

## Review and migration procedure

1. Pick a topic ID; read its accepted definition and one complete example. State the distinction
   being named before comparing candidate words. Review related IDs together where names collide.
2. Record the owner's chosen spelling and decision link here when accepted. Until then, the table's
   names remain working vocabulary. Separate a semantic proposal from a rename if both are desired.
3. Update the canonical topic, overview/authoring guidance, active proposals, diagrams and experiment
   fixtures coherently. Follow the source responsibilities in the [documentation map](README.md).
   Exact schema/code identifiers may require a separate implementation change; do not invent support.
4. Preserve the `vxx` anchor at its definition. If the definition moves, update this map and incoming
   links together. Add IDs for new topics rather than reassigning an existing ID to a different meaning.
5. Search active docs and fixtures for the old spelling and inspect each occurrence. Preserve legitimate
   external uses and historical records. Leave research conclusions/source pins intact; update an
   active index's current-status note when necessary.
6. Run `python3 tools/check_docs.py` and `git diff --check`. Parse changed fixtures and check their
   references/bindings as appropriate; passing these checks is not runtime or behavioral conformance.

No replacement vocabulary has been accepted in this map yet. The
[audit record](archive/migrations/2026-11-vocabulary-audit.md) records its initial coverage and preservation checks.

The [naming research packet](research/vocabulary/README.md) supplies sourced precedents,
[candidates for every topic](research/vocabulary/CANDIDATES.md) and
[complete examples](research/vocabulary/WALKTHROUGHS.md). Its recommendations remain proposals;
record accepted choices here through the procedure above, not in a parallel research decision ledger.
