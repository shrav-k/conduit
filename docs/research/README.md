# Research

Current sequence: the migration and post-migration rulings are complete. Before selecting syntax,
read the new [language/protocol foundations](language-protocol-foundations/README.md) and
[worked comparisons](language-protocol-foundations/WALKTHROUGHS.md). They compare primary sources
and record actual library probes without changing accepted semantics. The [current review queue](../REVIEW.md)
retains the connected authoring/conformance batches; [syntax examples](../proposals/authoring-syntax/README.md)
remain proposals. The [collaborator comparison](../archive/collaborator-proposals/REVIEW.md)
records adopted refinements and unadopted arguments from the two merged PRs, without treating
their external platform claims as newly verified research.

The [graph authoring decision audit](language-protocol-foundations/GRAPH_AUTHORING_REVIEW.md)
is the 2026-11-03 follow-up requested before continuing individual syntax rulings. It compares
recent choices with Kubernetes, CWL, Tekton, Argo, Compose and related contracts, including costs
and reasons to revisit choices if worked examples expose problems. The reference layout stays open.

The subsequent [runtime establishment comparison](RUNTIME_ESTABLISHMENT.md) examines how setup
addresses an instance's declared runtime using Compose, Kubernetes, CloudNativePG, AWS, Google
Cloud SQL, Azure, Nomad and Flyway. It compares explicit lifecycle nodes, typed operations with
runtime bindings and input-driven startup. The owner accepted typed operations explicitly bound
to declared instance runtimes, with targets passed as declared typed inputs. Reference encoding
and other authoring details remain open.

Research supports the [specification](../OVERVIEW.md) and [design contracts](../protocol/README.md).
It is not an additional set of requirements. The current concrete configuration belongs in
[experiments](../../experiments/README.md), where proposed syntax and validation evidence are
kept together. Completed ruling queues and duplicate configuration walkthroughs have been removed.

The [v0 implementation proposal](../implementation/README.md) applies this evidence
to concrete runtime and repository boundaries. Its pending decisions are separate from research
findings and from the accepted protocol.
The current design accepts code-optional MCP-backed plugins and independently accessible
capabilities. Executable-worker examples in earlier research are implementation options for
authored code, not a requirement for every contribution. I1 now accepts separate management and
execution services, shared policy with instance-owned state and no offline grants in v0.
The accepted first proof uses management and two executor processes on one machine, with
controlled connection failures; two-machine validation precedes remote-support claims. Consult the
[design-fit audit](../archive/reviews/CONCEPTUAL.md#design-fit-audit-code-optional-plugins-and-independent-mcp-access).

| Report | Read it for |
| --- | --- |
| [LANGUAGE / PROTOCOL FOUNDATIONS](language-protocol-foundations/README.md) | WIT, Smithy, YANG, TOSCA, CWL and other language/protocol comparisons; package scoping, typed bindings, effects and MCP compatibility. Includes primary-source inventory and links to 20 offline schema/YAML probes. |
| [ECOSYSTEM_ORGANIZATION](ECOSYSTEM_ORGANIZATION.md) | Source/release/website comparison across Pi, MCP, OpenTelemetry, Kubernetes, Compose, containerd, OCI, OpenTofu, Wasmtime and uv; evidence behind the repository/documentation migration and still-open distribution proposals. |
| [OBSERVABILITY](OBSERVABILITY.md) | Cloud/platform comparison, OpenTelemetry pipelines and correlation, Rust/GenAI compatibility, and the lightweight debugging package and capture policy accepted in I6. Backend/library/wire recommendations remain proposals. |
| [CLOUD_AUTHORITY](CLOUD_AUTHORITY.md) | AWS, Google Cloud and Azure authority patterns; accepted shared policy/instance-owned state and independent policy. Later I4 rulings define access management and a public persistence plugin loaded as a service dependency, initially using SQLite. |
| [AUTHORIZATION](AUTHORIZATION.md) | Cedar, Kubernetes RBAC, OPA, Zanzibar, Macaroons/Biscuit and MCP/OAuth: evidence for common Agent/Interface Policy and Grant contracts, delegation and enforcement. |
| [DISTRIBUTED_RUNTIME](DISTRIBUTED_RUNTIME.md) | I1 evidence from Tilt, Docker/containerd, Kubernetes, Rust systems and papers: execution/authority splits and no offline grants accepted; one-machine process proof accepted, not yet executed. |
| [CONFIGURATION](CONFIGURATION.md) | Kubernetes, Docker/Compose, typed configuration, schema semantics, papers and recorded offline probes. |
| [CONSTRUCTION](CONSTRUCTION.md) | The concise rationale for template/plugin/graph/runtime boundaries and their source evidence. |
| [ESTABLISHMENT](ESTABLISHMENT.md) | Detailed account, database, credential and filesystem setup/recovery cases. |
| [RUNTIME_ESTABLISHMENT](RUNTIME_ESTABLISHMENT.md) | Fresh database/cloud comparison of graph-to-runtime binding, setup readiness, operation progress and recovery; rationale for accepted typed-operation bindings and remaining representation choices. |
| [RESOURCES](RESOURCES.md) | Unitary responsibilities, type groups, capabilities and an open candidate catalog. |
| [CONDUITS](CONDUITS.md) | Access scenarios, shared authentication, filtering and messaging/network protocol comparisons. |
| [PI_CONNECTORS](PI_CONNECTORS.md) | Provider inventory, account/context boundaries, activation, MCP, Chord and telemetry. |
| [PI_RUNTIME](PI_RUNTIME.md) | Durable execution, replay, storage, frontend coupling and Rust-runtime implications. |
| [RUST_RUNTIME](RUST_RUNTIME.md) | Xerj/Pi source comparison, Rust executors, process/MCP/Wasm plugin boundaries, implementation pinning and a proposed runtime spike. |

## Evidence boundaries

The original Pi reports are pinned to `0441483926f283e42ca44eb7b4a46552e9469d6f`; source links
preserve that identity. RUST_RUNTIME records a separately pinned Pi follow-up and Xerj inspection.
Reported offline tests are historical evidence, not new tests performed by later research or
documentation cleanup. Source-system terminology remains where it describes the source.
Conduit owns its runtime; investigating Pi does not commit to running its runtime.
The distributed-runtime follow-up records newer source pins separately. Its historical recommendations are evidence; the accepted I1 process/authority choices are
recorded in the active protocol and implementation boundaries, not inferred from this report.

Protocol/source dates and primary links stay beside their claims. These reports were not all
re-fetched during the documentation reorganization. A real implementation must verify current
provider support and negotiate actual versions/features. The MCP reference used by the design
is 2026-07-28; the inspected Pi client has a different compatibility boundary, documented in
PI_CONNECTORS.

Examples and candidate types can be broader than the selected v0 slice. Use them as failure cases
and extension possibilities, not another review agenda. General authored graph loops, structured
work submission, remote communication and stronger security tiers remain deferred as specified
in [REVIEW](../archive/reviews/CONCEPTUAL.md#drafting-and-deferred-work).
