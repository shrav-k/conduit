# Research

Research supports the [specification](../SPEC.md) and [design contracts](../DESIGN.md).
It is not an additional set of requirements. The current concrete configuration belongs in
[experiments](../../experiments/README.md), where proposed syntax and validation evidence are
kept together. Completed ruling queues and duplicate configuration walkthroughs have been removed.

The [v0 implementation proposal](../proposals/implementation-v0/README.md) applies this evidence
to concrete runtime and repository boundaries. Its pending decisions are separate from research
findings and from the accepted protocol.
The current design accepts code-optional MCP-backed plugins and independently accessible
capabilities. Executable-worker examples in earlier research are implementation options for
authored code, not a requirement for every contribution. I1 now accepts separate management and
execution services; authority and outage behavior remain open. Consult the
[design-fit audit](../REVIEW.md#design-fit-audit-code-optional-plugins-and-independent-mcp-access).

| Report | Read it for |
| --- | --- |
| [CLOUD_AUTHORITY](CLOUD_AUTHORITY.md) | AWS, Google Cloud and Azure authority patterns; concrete change/recovery comparisons and the pending option of shared policy with instance-owned state. |
| [DISTRIBUTED_RUNTIME](DISTRIBUTED_RUNTIME.md) | I1 evidence from Tilt, Docker/containerd, Kubernetes, Rust systems and papers: execution separation accepted; authority and partition tradeoffs still open. |
| [CONFIGURATION](CONFIGURATION.md) | Kubernetes, Docker/Compose, typed configuration, schema semantics, papers and recorded offline probes. |
| [CONSTRUCTION](CONSTRUCTION.md) | The concise rationale for template/plugin/graph/runtime boundaries and their source evidence. |
| [ESTABLISHMENT](ESTABLISHMENT.md) | Detailed account, database, credential and filesystem setup/recovery cases. |
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
The distributed-runtime follow-up records newer source pins separately. It revises the
single-host recommendation without treating the replacement proposal as an accepted decision.

Protocol/source dates and primary links stay beside their claims. These reports were not all
re-fetched during the documentation reorganization. A real implementation must verify current
provider support and negotiate actual versions/features. The MCP reference used by the design
is 2026-07-28; the inspected Pi client has a different compatibility boundary, documented in
PI_CONNECTORS.

Examples and candidate types can be broader than the selected v0 slice. Use them as failure cases
and extension possibilities, not another review agenda. General authored graph loops, structured
work submission, remote communication and stronger security tiers remain deferred as specified
in [REVIEW](../REVIEW.md#drafting-and-deferred-work).
