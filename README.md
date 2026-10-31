# Conduit

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
| [SPEC.md](SPEC.md) | Working vocabulary, interactions and intended outcomes. |
| [DESIGN.md](DESIGN.md) | Accepted contracts, including configuration, accounts, memory, access and recovery; proposed additions are labeled. |
| [REVIEW.md](REVIEW.md) | Current stopping point, grouped unresolved choices, walkthroughs and initial experiment. |

**Current review:** templates carry complete establishment intent in typed YAML; users do not
assemble a second operation-request recipe. The unresolved boundary is how plugins and the
engine implement ordering, typed outputs, authentication waits and recovery, and whether authors
need optional graphs of typed plugin actions. Start at [Next discussion](REVIEW.md#next-discussion).
No lifecycle API, graph syntax or general workflow language has been selected.

## Research

Research retains evidence, worked examples and proposals; it does not override accepted contracts.

| Research | Purpose |
| --- | --- |
| [Establishment and recovery](research/ESTABLISHMENT.md) | Template/plugin/engine boundary; filesystem examples, login, indexing, password-manager access and optional typed composition. |
| [Resource typing and groups](research/RESOURCES.md) | Core groups, candidate types, capabilities and plugin boundaries. |
| [Conduit connections and state](research/CONDUITS.md) | Repository, communication, credential, AWS and Linear scenarios; shared state and participant lifetimes. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, configuration-language papers and candidate expression of the four families. |
| [Template/instance scenario](research/templates-instances.scenario.yaml) | Synthetic ownership fixture; not an authored operation language or a test of lifecycle/authentication behavior. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Detailed execution, recovery, frontend, storage and ecosystem evidence. |

Update these documents in place; Git history preserves earlier designs. Settle the configuration
contracts and bounded experiment before implementation or live account testing.
