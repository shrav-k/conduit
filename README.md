# Conduit

Conduit connects human users, autonomous agents and capabilities through five working families:
**Interface, Agent, Resource, Pool and Conduit**. Each has reusable template and concrete instance
forms. Implementation has not started.

| Family | Responsibility |
| --- | --- |
| Interface | The human-facing entry point for discovering and controlling the system, usable through different frontends. |
| Agent | An autonomous participant that acts through Conduits and can expose operations to other participants. |
| Resource | A unitary capability, data source or service owned by exactly one Pool instance. |
| Pool | The ownership and creation boundary for Resource instances. |
| Conduit | A connection carrying permitted interactions and data between endpoints, with filtering. |

The first **management Interface** has system-wide visibility and management scope in the local
installation: inspect templates and instances, instantiate Pools, Resources, Agents and Conduits,
and invoke their supported lifecycle operations. Resource creation still goes through an owning
Pool. CLI, terminal UI and test clients should exercise the same interaction contracts; choosing
a frontend must not select a different management implementation.

Templates contain typed YAML configuration and an inline or referenced setup graph of focused
plugins. One template can create many instances. Graphs establish instances; instances manage
ongoing behavior. Each instance retains an immutable resolved definition. Its declared operations,
runtime state and Conduit connections can evolve within that definition. Provider-specific
settings, switching and recovery details belong to type/integration contracts.

**Active proposal:** [Construction, interfaces and runtime interaction](research/CONSTRUCTION.md).
Agent replaces Session in this proposal. Conduit owns its runtime in Rust, with a Pi-inspired
terminal and replaceable frontends and integrations. Provider-native tools, MCP, native skills,
trackers and memory backends retain their useful contracts.

## Read the design

The pre-inquiry design remains saved on `main` at `c013f0a`. This research branch has not been
merged into it. Use the active proposal and review agenda for current decisions; the baseline
documents preserve contracts and comparison material, not competing current guidance.

| Document | Responsibility |
| --- | --- |
| [Active proposal](research/CONSTRUCTION.md) | Five families, management Interface, typed operations, construction and runtime boundaries. |
| [REVIEW.md](REVIEW.md) | Grouped unresolved contracts and the next end-to-end walkthrough. |
| [SPEC.md](SPEC.md) | Pre-inquiry vocabulary, interactions and intended outcomes for comparison. |
| [DESIGN.md](DESIGN.md) | Baseline configuration, account, memory, access and recovery contracts. |

## Research

| Research | Purpose |
| --- | --- |
| [Establishment and recovery](research/ESTABLISHMENT.md) | Ordering, outputs, authentication waits and failure cases. |
| [Resource typing and groups](research/RESOURCES.md) | Core groups, candidate types, capabilities and plugin boundaries. |
| [Conduit connections and state](research/CONDUITS.md) | Repository, communication, credential, AWS and Linear scenarios. |
| [Configuration languages](research/CONFIGURATION.md) | Kubernetes/Docker evidence, language papers and schema proposals. |
| [Template/instance scenario](research/templates-instances.scenario.yaml) | Earlier ownership fixture; not validation of the active proposal. |
| [Pi connectors and context](research/PI_CONNECTORS.md) | Provider/authentication boundaries, context portability, activation, MCP and Chord. |
| [Pi runtime and Durable](research/PI_RUNTIME.md) | Execution, recovery, frontend, storage and ecosystem evidence. |

Research recommendations remain proposals unless explicitly accepted. Update current documents
in place; Git preserves superseded explorations. Settle contracts and a bounded experiment
before implementation or live account testing.
