# Conduit

**Active branch direction:** [Conduits as composable graphs](research/CONDUIT_GRAPHS.md).
Session, Resource and Pool templates select graphs of plugin operations to establish their
instances and continuing interactions. Conduits retain template/instance forms: reusable graph
definitions and their particular executions. Several templates can reuse one graph. Setup and
continuing activity may coexist, while the boundary between a setup invocation and separately
attached access invocations is under review. The model below is the comparison baseline, saved
on `main` at `c013f0a`.

Conduit connects models, agents and capabilities through four families, each with a reusable
**template** and a concrete **instance** form. Templates describe how to establish instances;
instances retain their own configuration and observed state.

| Family | Template | Instance |
| --- | --- | --- |
| Session | Model/agent/account defaults, activation, limits and connection choices. | A durable participant with private context and executions. |
| Resource | Typed configuration for creating or attaching data or a capability. | A concrete capability owned by exactly one Pool instance. |
| Pool | Resource provisioning selections, defaults and policies. | The ownership and creation boundary for its Resource instances. |
| Conduit | Interaction, endpoint and filtering configuration for a flow. | A live filtered flow with logical consumer/peer attachments and connection state. |

Resources have one responsibility with related operations. Pools create and own them; Conduits
connect them and narrow access; Sessions activate permitted capabilities for their runtime.
Session templates replace the earlier Profile concept. Connections use Pool-owned Resources
for any durable backing data, without becoming Resource owners themselves.

Conduit owns its runtime in Rust, with a custom Pi-inspired terminal and replaceable frontends
and integrations. Provider-native tools, MCP, native skills, trackers and memory backends retain
their useful contracts. Implementation has not started.

## Read the design

| Document | Responsibility |
| --- | --- |
| [SPEC.md](SPEC.md) | Baseline vocabulary, interactions and intended outcomes for comparison with the graph proposal. |
| [DESIGN.md](DESIGN.md) | Baseline contracts, including configuration, accounts, memory, access and recovery; proposed additions are labeled. |
| [REVIEW.md](REVIEW.md) | Active branch agenda and the preserved review checkpoint, walkthroughs and initial experiment. |

**Current review:** use the [graph proposal](research/CONDUIT_GRAPHS.md#next-rulings-and-evidence)
to settle template bindings, plugin-operation composition and remaining lifetime details. Templates
still supply complete setup intent without a second operation-request recipe. The earlier
[review checkpoint](REVIEW.md#next-discussion) remains available if this direction is set aside.
Detailed lifecycle APIs and graph syntax have not been selected.

## Research

Research retains evidence, worked examples and proposals. Only explicitly accepted rulings
change the branch's working model; other recommendations remain open.

| Research | Purpose |
| --- | --- |
| [Conduits as graphs](research/CONDUIT_GRAPHS.md) | Active branch proposal: template-selected operational graphs, Docker/BuildKit lessons, plugin composition, filtering and finite/live activity. |
| [Establishment and recovery](research/ESTABLISHMENT.md) | Template/plugin/engine boundary; filesystem examples, login, indexing, password-manager access and optional typed composition. |
| [Resource typing and groups](research/RESOURCES.md) | Core groups, candidate types, capabilities and plugin boundaries. |
| [Conduit connections and state](research/CONDUITS.md) | Repository, communication, credential, AWS and Linear scenarios; shared state and participant lifetimes. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, configuration-language papers and candidate expression of the four families. |
| [Template/instance scenario](research/templates-instances.scenario.yaml) | Synthetic ownership fixture; not an authored operation language or a test of lifecycle/authentication behavior. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Detailed execution, recovery, frontend, storage and ecosystem evidence. |

Update these documents in place; Git history preserves earlier designs. Settle the configuration
contracts and bounded experiment before implementation or live account testing.
